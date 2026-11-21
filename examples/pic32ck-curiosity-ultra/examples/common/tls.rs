// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! TLS server connections over embassy-net with rustls's unbuffered API, as
//! a picoserve `Socket` (spike quality).
//!
//! [`accept`] runs the handshake on an accepted [`TcpSocket`]; the
//! [`TlsSocket`] it returns splits into a reader and a writer that share the
//! rustls state through a `RefCell`, borrowed only in synchronous code, never
//! across an await (picoserve reads and writes at once only on upgraded
//! connections, and may drop a read on a timeout, so a read that's dropped
//! mid-way loses nothing).
//!
//! Buffers, all the caller's:
//! - `rx`: TLS records from the peer, at least one whole record (up to
//!   16,384 + 256 bytes: a peer may send full-size records);
//! - `plain`: decrypted data not yet read, at least one record's payload;
//! - `tx`: records on their way out, at least `OVERHEAD` more than a
//!   record's plaintext;
//! - `out`: plaintext gathered into the next record: writes collect here
//!   and go out as one record when it fills or on a flush (picoserve
//!   flushes after each response), so a response's small writes (status
//!   line, each header) don't each become a record and a TCP segment.
//!
//! Records the reader must send (a key update's answer, an alert) wait in a
//! small buffer for the writer's next write or flush: the reader has no
//! writer. The server sends no TLS 1.3 session tickets (`send_tls13_tickets
//! = 0`), which would otherwise come out after the handshake.

use alloc::sync::Arc;
use core::cell::RefCell;

use embassy_net::tcp::{self, TcpReader, TcpSocket, TcpWriter};
use embedded_io_async::Write as _;
use picoserve::io::{ErrorType, Read, Write};
use rustls::server::{ServerConfig, UnbufferedServerConnection};
use rustls::unbuffered::{ConnectionState, EncodeError, EncryptError, InsufficientSizeError, UnbufferedStatus};

/// Bytes a TLS 1.3 record adds to its payload (header 5, content type 1, tag
/// 16), with room to spare.
pub const OVERHEAD: usize = 64;
const PENDING: usize = 256;

/// Why a connection failed.
#[derive(Debug)]
pub enum Error {
    Tcp(tcp::Error),
    Tls(rustls::Error),
    /// The peer closed the TCP connection mid-handshake or mid-record.
    Eof,
    /// A buffer couldn't hold what it had to: a record bigger than `rx`, or
    /// more decrypted data than `plain` holds while writing.
    BufferFull,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Tcp(e) => write!(f, "TCP: {e:?}"),
            Self::Tls(e) => write!(f, "TLS: {e}"),
            Self::Eof => f.write_str("the peer closed the connection"),
            Self::BufferFull => f.write_str("a TLS buffer is too small"),
        }
    }
}

impl core::error::Error for Error {}

impl embedded_io_async::Error for Error {
    fn kind(&self) -> embedded_io_async::ErrorKind {
        embedded_io_async::ErrorKind::Other
    }
}

impl From<tcp::Error> for Error {
    fn from(e: tcp::Error) -> Self {
        Self::Tcp(e)
    }
}

impl From<rustls::Error> for Error {
    fn from(e: rustls::Error) -> Self {
        Self::Tls(e)
    }
}

/// What the state machine needs next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    /// Decrypted data waits in `plain` (or the peer has closed).
    Plain,
    /// More bytes from the peer.
    Recv,
    /// Send what's been encoded (the output buffer is full or a flight ends).
    Flush,
    /// The handshake is done and nothing waits to be read.
    Ready,
    /// Application data was encrypted into the output.
    Wrote,
    /// Closed both ways.
    Closed,
}

/// Where encoded records go.
enum Out<'o> {
    /// The core's own small buffer (the reader's).
    Pending,
    /// A caller's buffer and how much of it is filled.
    Buf(&'o mut [u8], &'o mut usize),
}

/// The state both halves share.
struct Core<'b> {
    conn: UnbufferedServerConnection,
    rx: &'b mut [u8],
    rx_len: usize,
    plain: &'b mut [u8],
    plain_start: usize,
    plain_end: usize,
    pending: [u8; PENDING],
    pending_len: usize,
    peer_closed: bool,
    closed: bool,
}

impl Core<'_> {
    fn plain_len(&self) -> usize {
        self.plain_end - self.plain_start
    }

    /// Runs the state machine until it needs something: `app` is plaintext
    /// to encrypt (returned: the bytes taken).
    fn poll(&mut self, mut out: Out<'_>, app: Option<&[u8]>) -> Result<(Step, usize), Error> {
        loop {
            if self.plain_start == self.plain_end {
                self.plain_start = 0;
                self.plain_end = 0;
            }
            let UnbufferedStatus { mut discard, state } = self.conn.process_tls_records(&mut self.rx[..self.rx_len]);
            let mut taken = 0;
            let step = match state? {
                ConnectionState::ReadTraffic(mut traffic) => {
                    let mut copied = false;
                    while let Some(len) = traffic.peek_len() {
                        if self.plain.len() - self.plain_end < len.get() {
                            break;
                        }
                        let Some(record) = traffic.next_record() else { break };
                        let record = record?;
                        discard += record.discard;
                        self.plain[self.plain_end..self.plain_end + record.payload.len()].copy_from_slice(record.payload);
                        self.plain_end += record.payload.len();
                        copied = true;
                    }
                    if app.is_some() && copied {
                        None
                    } else if app.is_some() {
                        return Err(Error::BufferFull);
                    } else {
                        Some(Step::Plain)
                    }
                }
                ConnectionState::EncodeTlsData(mut encode) => {
                    let (buf, filled) = match &mut out {
                        Out::Pending => (&mut self.pending[..], &mut self.pending_len),
                        Out::Buf(buf, filled) => (&mut buf[..], &mut **filled),
                    };
                    match encode.encode(&mut buf[*filled..]) {
                        Ok(n) => {
                            *filled += n;
                            None
                        }
                        Err(EncodeError::InsufficientSize(InsufficientSizeError { .. })) => Some(Step::Flush),
                        Err(EncodeError::AlreadyEncoded) => None,
                    }
                }
                ConnectionState::TransmitTlsData(transmit) => {
                    transmit.done();
                    Some(Step::Flush)
                }
                ConnectionState::BlockedHandshake => Some(Step::Recv),
                ConnectionState::WriteTraffic(mut traffic) => match (app, &mut out) {
                    (Some(app), Out::Buf(buf, filled)) => {
                        let room = buf.len().saturating_sub(**filled + OVERHEAD);
                        let n = app.len().min(room).min(16_384);
                        if n == 0 {
                            Some(Step::Flush)
                        } else {
                            match traffic.encrypt(&app[..n], &mut buf[**filled..]) {
                                Ok(len) => {
                                    **filled += len;
                                    taken = n;
                                    Some(Step::Wrote)
                                }
                                Err(EncryptError::InsufficientSize(_)) => Some(Step::Flush),
                                Err(EncryptError::EncryptExhausted) => return Err(Error::Tls(rustls::Error::EncryptError)),
                            }
                        }
                    }
                    _ if self.plain_len() > 0 || self.peer_closed => Some(Step::Plain),
                    _ => Some(Step::Ready),
                },
                ConnectionState::PeerClosed => {
                    self.peer_closed = true;
                    None
                }
                ConnectionState::Closed => {
                    self.peer_closed = true;
                    self.closed = true;
                    Some(Step::Closed)
                }
                // Early data isn't enabled; anything newer is unexpected.
                _ => return Err(Error::Tls(rustls::Error::General(alloc::string::String::from("unexpected TLS state")))),
            };
            if discard > 0 {
                self.rx.copy_within(discard..self.rx_len, 0);
                self.rx_len -= discard;
            }
            if let Some(step) = step {
                return Ok((step, taken));
            }
        }
    }

    /// Appends bytes from the peer; how many fit.
    fn append(&mut self, data: &[u8]) -> usize {
        let n = data.len().min(self.rx.len() - self.rx_len);
        self.rx[self.rx_len..self.rx_len + n].copy_from_slice(&data[..n]);
        self.rx_len += n;
        n
    }

    fn take_plain(&mut self, buf: &mut [u8]) -> usize {
        let n = buf.len().min(self.plain_len());
        buf[..n].copy_from_slice(&self.plain[self.plain_start..self.plain_start + n]);
        self.plain_start += n;
        n
    }
}

/// An established TLS connection.
pub struct TlsSocket<'s, 'b> {
    socket: TcpSocket<'s>,
    core: RefCell<Core<'b>>,
    tx: &'b mut [u8],
    out: &'b mut [u8],
    out_len: usize,
}

/// Runs the server handshake on `socket` (already accepted).
///
/// # Errors
/// The handshake's failure, or the TCP connection's.
pub async fn accept<'s, 'b>(
    mut socket: TcpSocket<'s>,
    config: Arc<ServerConfig>,
    rx: &'b mut [u8],
    plain: &'b mut [u8],
    tx: &'b mut [u8],
    out: &'b mut [u8],
) -> Result<TlsSocket<'s, 'b>, Error> {
    let mut core = Core {
        conn: UnbufferedServerConnection::new(config)?,
        rx,
        rx_len: 0,
        plain,
        plain_start: 0,
        plain_end: 0,
        pending: [0; PENDING],
        pending_len: 0,
        peer_closed: false,
        closed: false,
    };
    let mut filled = 0;
    loop {
        let (step, _) = core.poll(Out::Buf(tx, &mut filled), None)?;
        match step {
            Step::Flush => {
                socket.write_all(&tx[..filled]).await?;
                filled = 0;
                socket.flush().await?;
            }
            Step::Recv => {
                if core.rx_len == core.rx.len() {
                    return Err(Error::BufferFull);
                }
                let n = socket.read(&mut core.rx[core.rx_len..]).await?;
                if n == 0 {
                    return Err(Error::Eof);
                }
                core.rx_len += n;
            }
            Step::Ready | Step::Plain => {
                if filled > 0 {
                    socket.write_all(&tx[..filled]).await?;
                    socket.flush().await?;
                }
                return Ok(TlsSocket { socket, core: RefCell::new(core), tx, out, out_len: 0 });
            }
            Step::Closed => return Err(Error::Eof),
            Step::Wrote => {}
        }
    }
}

impl ErrorType for TlsSocket<'_, '_> {
    type Error = Error;
}

/// The reading half.
pub struct TlsReader<'a, 'b> {
    reader: TcpReader<'a>,
    core: &'a RefCell<Core<'b>>,
}

/// The writing half.
pub struct TlsWriter<'a, 'b> {
    writer: TcpWriter<'a>,
    core: &'a RefCell<Core<'b>>,
    tx: &'a mut [u8],
    /// Plaintext gathered into the next record, up to one record's worth.
    out: &'a mut [u8],
    out_len: &'a mut usize,
}

impl ErrorType for TlsReader<'_, '_> {
    type Error = Error;
}

impl ErrorType for TlsWriter<'_, '_> {
    type Error = Error;
}

impl Read for TlsReader<'_, '_> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            let step = {
                let mut core = self.core.borrow_mut();
                if core.plain_len() > 0 {
                    return Ok(core.take_plain(buf));
                }
                if core.peer_closed {
                    return Ok(0);
                }
                let pending = core.pending_len;
                let (step, _) = core.poll(Out::Pending, None)?;
                if step == Step::Flush && core.pending_len == pending && core.pending_len == PENDING {
                    return Err(Error::BufferFull);
                }
                step
            };
            match step {
                Step::Plain | Step::Flush | Step::Wrote => {}
                Step::Closed => return Ok(0),
                Step::Recv | Step::Ready => {
                    let core = self.core;
                    let got = self
                        .reader
                        .read_with(|data| {
                            let n = core.borrow_mut().append(data);
                            (n, data.is_empty())
                        })
                        .await?;
                    if got {
                        // The TCP stream ended without a close_notify.
                        return Ok(0);
                    }
                    if self.core.borrow().rx_len == self.core.borrow().rx.len() {
                        // A full buffer with no whole record in it.
                        let (step, _) = self.core.borrow_mut().poll(Out::Pending, None)?;
                        if matches!(step, Step::Recv | Step::Ready) {
                            return Err(Error::BufferFull);
                        }
                    }
                }
            }
        }
    }
}

/// Sends what the reader left waiting.
async fn send_pending(writer: &mut TcpWriter<'_>, core: &RefCell<Core<'_>>) -> Result<(), Error> {
    let mut pending = [0u8; PENDING];
    let n = {
        let mut core = core.borrow_mut();
        let n = core.pending_len;
        pending[..n].copy_from_slice(&core.pending[..n]);
        core.pending_len = 0;
        n
    };
    if n > 0 {
        writer.write_all(&pending[..n]).await?;
    }
    Ok(())
}

/// Encrypts all of `data` into records (as few as `tx` allows) and sends
/// them.
async fn seal(writer: &mut TcpWriter<'_>, core: &RefCell<Core<'_>>, tx: &mut [u8], mut data: &[u8]) -> Result<(), Error> {
    send_pending(writer, core).await?;
    while !data.is_empty() {
        let mut filled = 0;
        let (step, taken) = core.borrow_mut().poll(Out::Buf(tx, &mut filled), Some(data))?;
        if filled > 0 {
            writer.write_all(&tx[..filled]).await?;
        }
        data = &data[taken..];
        match step {
            Step::Wrote | Step::Flush => {}
            Step::Closed => return Err(Error::Eof),
            Step::Plain | Step::Recv | Step::Ready => return Err(Error::BufferFull),
        }
    }
    Ok(())
}

impl TlsWriter<'_, '_> {
    /// The plaintext one record carries here.
    fn capacity(&self) -> usize {
        self.out.len().min(self.tx.len() - OVERHEAD).min(16_384)
    }

    /// Seals what's gathered into a record and sends it.
    async fn seal_out(&mut self) -> Result<(), Error> {
        let n = core::mem::take(self.out_len);
        seal(&mut self.writer, self.core, self.tx, &self.out[..n]).await
    }
}

impl embedded_io_async::Write for TlsWriter<'_, '_> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let capacity = self.capacity();
        if *self.out_len == 0 && buf.len() >= capacity {
            // A whole record's worth: straight out, no copy.
            seal(&mut self.writer, self.core, self.tx, &buf[..capacity]).await?;
            return Ok(capacity);
        }
        if *self.out_len == capacity {
            self.seal_out().await?;
        }
        let n = buf.len().min(capacity - *self.out_len);
        self.out[*self.out_len..*self.out_len + n].copy_from_slice(&buf[..n]);
        *self.out_len += n;
        Ok(n)
    }

    async fn flush(&mut self) -> Result<(), Error> {
        if *self.out_len > 0 {
            self.seal_out().await?;
        }
        send_pending(&mut self.writer, self.core).await?;
        self.writer.flush().await?;
        Ok(())
    }
}

impl Write for TlsWriter<'_, '_> {
    async fn write_with<F: FnOnce(picoserve::mem::BorrowedCursor<'_>) -> R, R>(&mut self, f: F) -> Result<R, Error> {
        let capacity = self.capacity();
        if *self.out_len == capacity {
            self.seal_out().await?;
        }
        let mut buffer = picoserve::mem::BorrowedBuffer::new(&mut self.out[*self.out_len..capacity]);
        let output = f(buffer.unfilled());
        *self.out_len += buffer.len();
        Ok(output)
    }
}

impl<'b> picoserve::io::Socket<picoserve::EmbassyRuntime> for TlsSocket<'_, 'b> {
    type Error = Error;
    type ReadHalf<'a>
        = TlsReader<'a, 'b>
    where
        Self: 'a;
    type WriteHalf<'a>
        = TlsWriter<'a, 'b>
    where
        Self: 'a;

    fn split(&mut self) -> (Self::ReadHalf<'_>, Self::WriteHalf<'_>) {
        let (reader, writer) = self.socket.split();
        (
            TlsReader { reader, core: &self.core },
            TlsWriter { writer, core: &self.core, tx: self.tx, out: self.out, out_len: &mut self.out_len },
        )
    }

    async fn abort<T: picoserve::Timer<picoserve::EmbassyRuntime>>(
        mut self,
        timeouts: &picoserve::Timeouts,
        timer: &T,
    ) -> Result<(), picoserve::Error<Error>> {
        TcpSocket::abort(&mut self.socket);
        timer
            .run_with_timeout(timeouts.write, self.socket.flush())
            .await
            .map_err(picoserve::Error::WriteTimeout)?
            .map_err(|e| picoserve::Error::Write(Error::Tcp(e)))
    }

    async fn shutdown<T: picoserve::Timer<picoserve::EmbassyRuntime>>(
        mut self,
        timeouts: &picoserve::Timeouts,
        timer: &T,
    ) -> Result<(), picoserve::Error<Error>> {
        // What's still gathered, a close_notify, then close the TCP side and
        // let it drain.
        if self.out_len > 0 {
            let n = core::mem::take(&mut self.out_len);
            let (_, mut writer) = self.socket.split();
            timer
                .run_with_timeout(timeouts.write, seal(&mut writer, &self.core, self.tx, &self.out[..n]))
                .await
                .map_err(picoserve::Error::WriteTimeout)?
                .map_err(picoserve::Error::Write)?;
        }
        let mut filled = 0;
        {
            let mut core = self.core.borrow_mut();
            if let Ok(ConnectionState::WriteTraffic(mut traffic)) = core.conn.process_tls_records(&mut []).state
                && let Ok(n) = traffic.queue_close_notify(self.tx)
            {
                filled = n;
            }
        }
        let result = async {
            if filled > 0 {
                self.socket.write_all(&self.tx[..filled]).await?;
            }
            self.socket.close();
            self.socket.flush().await
        };
        timer
            .run_with_timeout(timeouts.write, result)
            .await
            .map_err(picoserve::Error::WriteTimeout)?
            .map_err(|e| picoserve::Error::Write(Error::Tcp(e)))
    }
}

/// A crypto provider with only what a TLS 1.3 server with a P-256
/// certificate needs, using the fastest code there is for this core:
///
/// - suites `TLS13_CHACHA20_POLY1305_SHA256` (first: the fastest in software
///   here) and `TLS13_AES_128_GCM_SHA256` (mandatory in RFC 8446), from
///   rustls-rustcrypto;
/// - key exchange X25519 with salty (Haase's assembly, ~22 ms for both
///   halves) and secp256r1 with P256-Cortex-M4's (~10 ms); rustls takes the
///   client's preferred group of the two, which is almost always X25519;
/// - signing with an ECDSA P-256 key, with P256-Cortex-M4 (~3 ms);
/// - no signature verification: the server asks for no client certificate.
///
/// `rustls_rustcrypto::provider()` would keep every algorithm it has (RSA,
/// P-384, Ed25519, SHA-512...) alive through its tables, and its own key
/// exchanges are private.
#[must_use]
pub fn provider() -> rustls::crypto::CryptoProvider {
    rustls::crypto::CryptoProvider {
        cipher_suites: alloc::vec![rustls_rustcrypto::TLS13_CHACHA20_POLY1305_SHA256, rustls_rustcrypto::TLS13_AES_128_GCM_SHA256],
        kx_groups: alloc::vec![&kx::X25519 as &dyn rustls::crypto::SupportedKxGroup, &kx::SecP256R1],
        signature_verification_algorithms: rustls::crypto::WebPkiSupportedAlgorithms { all: &[], mapping: &[] },
        secure_random: &rustls_rustcrypto::Provider,
        key_provider: &P256Keys,
    }
}

/// `rand_core` 0.6 over getrandom (P256-Cortex-M4's bindings take 0.6).
struct Getrandom06;

impl rand_core_06::RngCore for Getrandom06 {
    fn next_u32(&mut self) -> u32 {
        let mut b = [0; 4];
        self.fill_bytes(&mut b);
        u32::from_le_bytes(b)
    }

    fn next_u64(&mut self) -> u64 {
        let mut b = [0; 8];
        self.fill_bytes(&mut b);
        u64::from_le_bytes(b)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        // The firmware's backend never fails once its source is set up.
        if getrandom::fill(dest).is_err() {
            dest.fill(0);
            panic!("no randomness");
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core_06::Error> {
        getrandom::fill(dest).map_err(|_| rand_core_06::Error::from(core::num::NonZeroU32::new(1).unwrap_or(core::num::NonZeroU32::MIN)))
    }
}

impl rand_core_06::CryptoRng for Getrandom06 {}

/// Loads ECDSA P-256 private keys (PKCS #8 holding a SEC1 key), and nothing
/// else. Parsing only: no curve arithmetic until the first signature.
#[derive(Debug)]
struct P256Keys;

impl rustls::crypto::KeyProvider for P256Keys {
    fn load_private_key(
        &self,
        key_der: rustls::pki_types::PrivateKeyDer<'static>,
    ) -> Result<Arc<dyn rustls::sign::SigningKey>, rustls::Error> {
        let bad = || rustls::Error::General(alloc::string::String::from("not an ECDSA P-256 PKCS #8 key"));
        let rustls::pki_types::PrivateKeyDer::Pkcs8(der) = key_der else { return Err(bad()) };
        let info = pkcs8::PrivateKeyInfoRef::try_from(der.secret_pkcs8_der()).map_err(|_| bad())?;
        info.algorithm
            .assert_oids(
                pkcs8::ObjectIdentifier::new_unwrap("1.2.840.10045.2.1"),
                pkcs8::ObjectIdentifier::new_unwrap("1.2.840.10045.3.1.7"),
            )
            .map_err(|_| bad())?;
        let key = sec1::EcPrivateKey::try_from(info.private_key.as_bytes()).map_err(|_| bad())?;
        let secret = p256_cortex_m4::SecretKey::from_bytes(key.private_key).map_err(|_| bad())?;
        Ok(Arc::new(P256Key(Arc::new(secret))))
    }
}

/// An ECDSA P-256 key, signing with P256-Cortex-M4.
struct P256Key(Arc<p256_cortex_m4::SecretKey>);

impl core::fmt::Debug for P256Key {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("P256Key")
    }
}

impl rustls::sign::SigningKey for P256Key {
    fn choose_scheme(&self, offered: &[rustls::SignatureScheme]) -> Option<alloc::boxed::Box<dyn rustls::sign::Signer>> {
        offered
            .contains(&rustls::SignatureScheme::ECDSA_NISTP256_SHA256)
            .then(|| alloc::boxed::Box::new(P256Signer(self.0.clone())) as alloc::boxed::Box<dyn rustls::sign::Signer>)
    }

    fn algorithm(&self) -> rustls::SignatureAlgorithm {
        rustls::SignatureAlgorithm::ECDSA
    }
}

struct P256Signer(Arc<p256_cortex_m4::SecretKey>);

impl core::fmt::Debug for P256Signer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("P256Signer")
    }
}

impl rustls::sign::Signer for P256Signer {
    fn sign(&self, message: &[u8]) -> Result<alloc::vec::Vec<u8>, rustls::Error> {
        use sha2::Digest as _;
        let digest = sha2::Sha256::digest(message);
        let signature = self.0.sign_prehashed(&digest, Getrandom06).to_untagged_bytes();
        Ok(der_signature(&signature[..32], &signature[32..]))
    }

    fn scheme(&self) -> rustls::SignatureScheme {
        rustls::SignatureScheme::ECDSA_NISTP256_SHA256
    }
}

/// An ECDSA signature as TLS carries it: DER `SEQUENCE { INTEGER r, INTEGER
/// s }`, each integer minimal and positive.
fn der_signature(r: &[u8], s: &[u8]) -> alloc::vec::Vec<u8> {
    fn integer(out: &mut alloc::vec::Vec<u8>, value: &[u8]) {
        let first = value.iter().position(|&b| b != 0).unwrap_or(value.len() - 1);
        let value = &value[first..];
        let pad = value[0] & 0x80 != 0;
        out.push(0x02);
        #[expect(clippy::cast_possible_truncation, reason = "at most 33 bytes")]
        out.push((value.len() + usize::from(pad)) as u8);
        if pad {
            out.push(0);
        }
        out.extend_from_slice(value);
    }
    let mut body = alloc::vec::Vec::with_capacity(70);
    integer(&mut body, r);
    integer(&mut body, s);
    let mut out = alloc::vec::Vec::with_capacity(72);
    out.push(0x30);
    #[expect(clippy::cast_possible_truncation, reason = "at most 70 bytes")]
    out.push(body.len() as u8);
    out.extend_from_slice(&body);
    out
}

/// The two key exchanges.
mod kx {
    use alloc::boxed::Box;

    use rustls::crypto::{ActiveKeyExchange, SharedSecret, SupportedKxGroup};
    use rustls::{NamedGroup, PeerMisbehaved};

    use super::Getrandom06;

    fn invalid() -> rustls::Error {
        rustls::Error::from(PeerMisbehaved::InvalidKeyShare)
    }

    /// X25519 with salty.
    #[derive(Debug)]
    pub struct X25519;

    impl SupportedKxGroup for X25519 {
        fn name(&self) -> NamedGroup {
            NamedGroup::X25519
        }

        fn start(&self) -> Result<Box<dyn ActiveKeyExchange>, rustls::Error> {
            let mut secret = [0u8; 32];
            getrandom::fill(&mut secret).map_err(|_| rustls::Error::General(alloc::string::String::from("no randomness")))?;
            let mut base = [0u8; 32];
            base[0] = 9;
            let public = salty::agreement::x25519(secret, base);
            Ok(Box::new(X25519Exchange { secret, public }))
        }
    }

    struct X25519Exchange {
        secret: [u8; 32],
        public: [u8; 32],
    }

    impl Drop for X25519Exchange {
        fn drop(&mut self) {
            // SAFETY: a plain write to our own field, volatile so it stays.
            unsafe { core::ptr::write_volatile(&raw mut self.secret, [0; 32]) };
        }
    }

    impl ActiveKeyExchange for X25519Exchange {
        fn complete(self: Box<Self>, peer: &[u8]) -> Result<SharedSecret, rustls::Error> {
            let peer: [u8; 32] = peer.try_into().map_err(|_| invalid())?;
            let shared = salty::agreement::x25519(self.secret, peer);
            // RFC 7748 section 6.1: an all-zero result means a small-order
            // point from the peer.
            if shared == [0; 32] {
                return Err(invalid());
            }
            Ok(shared.as_slice().into())
        }

        fn pub_key(&self) -> &[u8] {
            &self.public
        }

        fn group(&self) -> NamedGroup {
            NamedGroup::X25519
        }
    }

    /// secp256r1 with P256-Cortex-M4.
    #[derive(Debug)]
    pub struct SecP256R1;

    impl SupportedKxGroup for SecP256R1 {
        fn name(&self) -> NamedGroup {
            NamedGroup::secp256r1
        }

        fn start(&self) -> Result<Box<dyn ActiveKeyExchange>, rustls::Error> {
            let secret = p256_cortex_m4::SecretKey::random(Getrandom06);
            let public = secret.public_key().to_uncompressed_sec1_bytes();
            Ok(Box::new(SecP256R1Exchange { secret, public }))
        }
    }

    struct SecP256R1Exchange {
        secret: p256_cortex_m4::SecretKey,
        public: [u8; 65],
    }

    impl ActiveKeyExchange for SecP256R1Exchange {
        fn complete(self: Box<Self>, peer: &[u8]) -> Result<SharedSecret, rustls::Error> {
            // Checks that the point is on the curve.
            let peer = p256_cortex_m4::PublicKey::from_sec1_bytes(peer).map_err(|_| invalid())?;
            Ok(self.secret.agree(&peer).as_bytes().as_slice().into())
        }

        fn pub_key(&self) -> &[u8] {
            &self.public
        }

        fn group(&self) -> NamedGroup {
            NamedGroup::secp256r1
        }
    }
}

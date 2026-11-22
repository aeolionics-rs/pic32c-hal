// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A UART with ring buffers, as Embassy's `BufferedUart`: the SERCOM's
//! interrupts move bytes between its data register and buffers the caller
//! gives, so reception goes on while nothing reads, up to the receive
//! buffer's size, and writes return once their bytes are in the transmit
//! buffer.
//!
//! When the receive buffer is full the handler stops taking bytes (it masks
//! RXC), leaving them in the SERCOM: with flow control ([`BufferedUart::new_with_rtscts`])
//! the SERCOM's RTS then holds the far end until a read makes room, and
//! nothing is lost; without, the SERCOM overruns and the next read reports
//! [`Error::Overrun`](super::Error::Overrun). The FIFO ([`Config::fifo`](super::Config::fifo))
//! may be on: there is no DMA here.

use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;
use portable_atomic::{AtomicBool, AtomicU8, Ordering};

use embassy_hal_internal::atomic_ring_buffer::RingBuffer;

use super::{BUFOVF, Config, ConfigError, DRE, ERROR, Error, FERR, INTENCLR, INTENSET, INTFLAG, InterruptHandler, PERR, RXC, STATUS, TXC, Uart, enable_interrupts, read8, read16, write8, write16};
use crate::Peri;
use crate::interrupt::typelevel::{Binding, Handler};
use crate::mode::Async;
use crate::sercom::{Instance, Pad};

/// A SERCOM's buffered state: the rings, the receive errors seen since the
/// last read, and whether a [`BufferedUart`] owns the interrupts.
pub(crate) struct State {
    active: AtomicBool,
    rx: RingBuffer,
    tx: RingBuffer,
    /// STATUS's PERR, FERR and BUFOVF bits, gathered.
    errors: AtomicU8,
    /// Halves alive: the rings come down when it reaches 0.
    halves: AtomicU8,
}

impl State {
    pub(crate) const fn new() -> Self {
        Self {
            active: AtomicBool::new(false),
            rx: RingBuffer::new(),
            tx: RingBuffer::new(),
            errors: AtomicU8::new(0),
            halves: AtomicU8::new(0),
        }
    }

    pub(crate) fn active(&self) -> bool {
        self.active.load(Ordering::Acquire)
    }
}

/// The SERCOM's interrupts while a [`BufferedUart`] has it: the receiver
/// into the receive ring (until it's full), the transmit ring into the
/// transmitter, errors noted, waiters woken.
pub(crate) fn on_interrupt(base: usize, state: &super::State) {
    let b = &state.buffered;
    // Receive.
    let mut received = false;
    while read8(base, INTFLAG) & RXC != 0 {
        // SAFETY: the handler is the receive ring's only writer.
        let mut writer = unsafe { b.rx.writer() };
        if writer.push_slice().is_empty() {
            // Full: leave the byte in the SERCOM (RTS holds the far end, with
            // flow control) until a read makes room and unmasks RXC.
            write8(base, INTENCLR, RXC);
            break;
        }
        let status = read16(base, STATUS) & (PERR | FERR | BUFOVF);
        #[expect(clippy::cast_possible_truncation, reason = "8 data bits")]
        let byte = super::read_data(base) as u8;
        if status != 0 {
            write16(base, STATUS, status);
            b.errors.fetch_or(status as u8, Ordering::AcqRel);
        }
        writer.push_one(byte);
        received = true;
    }
    // An error with no byte (`sercom_v0` has no ERROR flag: its errors
    // come with RXC, above).
    #[cfg(not(sercom_v0))]
    if read8(base, INTFLAG) & ERROR != 0 {
        let status = read16(base, STATUS) & (PERR | FERR | BUFOVF);
        write16(base, STATUS, status);
        write8(base, INTFLAG, ERROR);
        b.errors.fetch_or(status as u8, Ordering::AcqRel);
        received = true;
    }
    if received {
        state.rx_waker.wake();
    }
    // Transmit.
    if read8(base, INTENSET) & DRE != 0 {
        // SAFETY: the handler is the transmit ring's only reader.
        let mut reader = unsafe { b.tx.reader() };
        while read8(base, INTFLAG) & DRE != 0 {
            if let Some(byte) = reader.pop_one() {
                super::write_data(base, u32::from(byte));
            } else {
                write8(base, INTENCLR, DRE);
                break;
            }
        }
        state.tx_waker.wake();
    }
    if read8(base, INTENSET) & TXC != 0 && read8(base, INTFLAG) & TXC != 0 {
        write8(base, INTENCLR, TXC);
        state.tx_waker.wake();
    }
}

/// A UART with ring buffers.
pub struct BufferedUart<'d> {
    tx: BufferedUartTx<'d>,
    rx: BufferedUartRx<'d>,
}

/// A [`BufferedUart`]'s transmitter.
pub struct BufferedUartTx<'d> {
    base: usize,
    state: &'static super::State,
    _lifetime: PhantomData<&'d mut [u8]>,
}

/// A [`BufferedUart`]'s receiver.
pub struct BufferedUartRx<'d> {
    base: usize,
    state: &'static super::State,
    _lifetime: PhantomData<&'d mut [u8]>,
}

impl<'d> BufferedUart<'d> {
    /// A buffered UART on `sercom`, TX on `tx` and RX on `rx`, with
    /// `tx_buffer` and `rx_buffer` as its rings, and `irqs` binding its four
    /// vectors to [`InterruptHandler`].
    ///
    /// # Errors
    ///
    /// If [`Config`]'s generator can't give its baud rate.
    ///
    /// # Panics
    ///
    /// If a buffer is empty.
    #[allow(clippy::needless_pass_by_value, reason = "the SERCOM has the pins for 'd")]
    pub fn new<T: Instance, const RX: u8, const IOSET: u8>(
        sercom: Peri<'d, T>,
        tx: Peri<'d, impl Pad<T, 0, IOSET>>,
        rx: Peri<'d, impl Pad<T, RX, IOSET>>,
        _irqs: impl Binding<T::Dre, InterruptHandler<T>> + Binding<T::Txc, InterruptHandler<T>> + Binding<T::Rxc, InterruptHandler<T>> + Binding<T::Error, InterruptHandler<T>> + 'd,
        tx_buffer: &'d mut [u8],
        rx_buffer: &'d mut [u8],
        config: Config,
    ) -> Result<Self, ConfigError>
    where
        InterruptHandler<T>: Handler<T::Dre> + Handler<T::Txc> + Handler<T::Rxc> + Handler<T::Error>,
    {
        const { super::check_rx_pad(RX) };
        let _ = sercom;
        let uart = Uart::<Async>::start::<T>(&*tx, tx.function(), &*rx, rx.function(), RX, false, None, config)?;
        Ok(Self::start::<T>(&uart, tx_buffer, rx_buffer))
    }

    /// As [`new`](Self::new), with hardware flow control: RX on pad 1, RTS on
    /// `rts` (pad 2), CTS on `cts` (pad 3).
    ///
    /// # Errors
    ///
    /// As [`new`](Self::new).
    ///
    /// # Panics
    ///
    /// As [`new`](Self::new).
    #[cfg(not(sercom_v0))]
    #[allow(clippy::needless_pass_by_value, reason = "the SERCOM has the pins for 'd")]
    #[allow(clippy::too_many_arguments, reason = "the SERCOM, its pins, the binding, two buffers and the configuration")]
    pub fn new_with_rtscts<T: Instance, const IOSET: u8>(
        sercom: Peri<'d, T>,
        tx: Peri<'d, impl Pad<T, 0, IOSET>>,
        rx: Peri<'d, impl Pad<T, 1, IOSET>>,
        rts: Peri<'d, impl Pad<T, 2, IOSET>>,
        cts: Peri<'d, impl Pad<T, 3, IOSET>>,
        _irqs: impl Binding<T::Dre, InterruptHandler<T>> + Binding<T::Txc, InterruptHandler<T>> + Binding<T::Rxc, InterruptHandler<T>> + Binding<T::Error, InterruptHandler<T>> + 'd,
        tx_buffer: &'d mut [u8],
        rx_buffer: &'d mut [u8],
        config: Config,
    ) -> Result<Self, ConfigError>
    where
        InterruptHandler<T>: Handler<T::Dre> + Handler<T::Txc> + Handler<T::Rxc> + Handler<T::Error>,
    {
        let _ = sercom;
        let uart = Uart::<Async>::start::<T>(&*tx, tx.function(), &*rx, rx.function(), 1, true, None, config)?;
        crate::gpio::set_function(&*rts, rts.function());
        crate::gpio::set_function(&*cts, cts.function());
        Ok(Self::start::<T>(&uart, tx_buffer, rx_buffer))
    }

    fn start<T: Instance>(uart: &Uart<'_, Async>, tx_buffer: &'d mut [u8], rx_buffer: &'d mut [u8]) -> Self {
        assert!(!tx_buffer.is_empty() && !rx_buffer.is_empty(), "the rings need room");
        let state = T::state();
        let base = uart.tx.base;
        // SAFETY: the buffers outlive the rings' use ('d, and `Drop` takes the
        // rings down), and only this UART and its handler touch them.
        unsafe {
            state.buffered.rx.init(rx_buffer.as_mut_ptr(), rx_buffer.len());
            state.buffered.tx.init(tx_buffer.as_mut_ptr(), tx_buffer.len());
        }
        state.buffered.errors.store(0, Ordering::Release);
        state.buffered.halves.store(2, Ordering::Release);
        state.buffered.active.store(true, Ordering::Release);
        enable_interrupts::<T>();
        write8(base, INTENSET, RXC | ERROR);
        Self {
            tx: BufferedUartTx { base, state, _lifetime: PhantomData },
            rx: BufferedUartRx { base, state, _lifetime: PhantomData },
        }
    }

    /// The transmitter and the receiver, apart.
    #[must_use]
    pub fn split(self) -> (BufferedUartTx<'d>, BufferedUartRx<'d>) {
        (self.tx, self.rx)
    }

    /// The baud rate the BAUD value gives, which may differ a little from
    /// the one asked for.
    #[must_use]
    pub fn baudrate(&self) -> u32 {
        self.tx.state.baudrate(self.tx.base)
    }

    /// Changes the baud rate, for both directions. BAUD can only be written
    /// with the SERCOM disabled, so a byte on the wire at that moment is
    /// lost; the rings keep what they hold. Flush first to keep what's
    /// being sent.
    ///
    /// # Errors
    ///
    /// If the SERCOM's clock can't give `baudrate`; the rate is then as it
    /// was.
    pub fn set_baudrate(&mut self, baudrate: u32) -> Result<(), ConfigError> {
        self.tx.set_baudrate(baudrate)
    }

    /// Changes the configuration, for both directions: as
    /// [`Uart::set_config`](super::Uart::set_config). The rings keep what
    /// they hold; flush first to keep what's being sent.
    ///
    /// # Errors
    ///
    /// As [`Uart::set_config`](super::Uart::set_config).
    pub fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        self.tx.set_config(config)
    }

    /// As [`BufferedUartRx::fill_buf`].
    ///
    /// # Errors
    ///
    /// As [`BufferedUartRx::fill_buf`].
    pub async fn fill_buf(&mut self) -> Result<&[u8], Error> {
        self.rx.fill_buf().await
    }

    /// As [`BufferedUartRx::consume`].
    pub fn consume(&mut self, amount: usize) {
        self.rx.consume(amount);
    }

    /// Writes as much of `bytes` as fits in the transmit ring, waiting until
    /// some does: how many.
    ///
    /// # Errors
    ///
    /// None so far; the result is for embedded-io.
    pub async fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        self.tx.write(bytes).await
    }

    /// Waits until everything written has gone out.
    ///
    /// # Errors
    ///
    /// As [`write`](Self::write).
    pub async fn flush(&mut self) -> Result<(), Error> {
        self.tx.flush().await
    }

    /// Reads what has come, waiting until something has: how many bytes.
    ///
    /// # Errors
    ///
    /// A receive error seen since the last read (overrun before framing
    /// before parity), reported once; the bytes stay for the next read.
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        self.rx.read(buffer).await
    }
}

impl BufferedUartTx<'_> {
    /// Writes as much of `bytes` as fits in the transmit ring, waiting until
    /// some does: how many.
    ///
    /// # Errors
    ///
    /// None so far; the result is for embedded-io.
    pub async fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        if bytes.is_empty() {
            return Ok(0);
        }
        poll_fn(|cx| {
            self.state.tx_waker.register(cx.waker());
            // SAFETY: this half is the transmit ring's only writer.
            let mut writer = unsafe { self.state.buffered.tx.writer() };
            let room = writer.push_slice();
            let n = room.len().min(bytes.len());
            room[..n].copy_from_slice(&bytes[..n]);
            writer.push_done(n);
            write8(self.base, INTENSET, DRE);
            if n == 0 { Poll::Pending } else { Poll::Ready(Ok(n)) }
        })
        .await
    }

    /// Writes all of `bytes`.
    ///
    /// # Errors
    ///
    /// As [`write`](Self::write).
    pub async fn write_all(&mut self, mut bytes: &[u8]) -> Result<(), Error> {
        while !bytes.is_empty() {
            let n = self.write(bytes).await?;
            bytes = &bytes[n..];
        }
        Ok(())
    }

    /// Writes as much of `bytes` as fits in the transmit ring, spinning
    /// until some does, for code that isn't async: how many.
    ///
    /// # Errors
    ///
    /// As [`write`](Self::write).
    pub fn blocking_write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        if bytes.is_empty() {
            return Ok(0);
        }
        loop {
            // SAFETY: this half is the transmit ring's only writer.
            let mut writer = unsafe { self.state.buffered.tx.writer() };
            let room = writer.push_slice();
            let n = room.len().min(bytes.len());
            room[..n].copy_from_slice(&bytes[..n]);
            writer.push_done(n);
            write8(self.base, INTENSET, DRE);
            if n > 0 {
                return Ok(n);
            }
        }
    }

    /// Bytes written and not yet handed to the transmitter.
    #[must_use]
    pub fn pending(&self) -> usize {
        self.state.buffered.tx.available()
    }

    /// Spins until everything written has gone out.
    ///
    /// # Errors
    ///
    /// As [`write`](Self::write).
    pub fn blocking_flush(&mut self) -> Result<(), Error> {
        while !(self.state.buffered.tx.is_empty() && read8(self.base, INTFLAG) & (DRE | TXC) == DRE | TXC) {}
        Ok(())
    }

    /// The baud rate, which the receiver shares.
    #[must_use]
    pub fn baudrate(&self) -> u32 {
        self.state.baudrate(self.base)
    }

    /// Changes the baud rate, for the receiver too: as
    /// [`BufferedUart::set_baudrate`].
    ///
    /// # Errors
    ///
    /// As [`BufferedUart::set_baudrate`].
    pub fn set_baudrate(&mut self, baudrate: u32) -> Result<(), ConfigError> {
        self.state.set_baudrate(self.base, baudrate)
    }

    /// Changes the configuration, for the receiver too: as
    /// [`BufferedUart::set_config`].
    ///
    /// # Errors
    ///
    /// As [`BufferedUart::set_config`].
    pub fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        self.state.configure(self.base, config)
    }

    /// Waits until everything written has gone out: the ring empty and the
    /// last byte's stop bit sent.
    ///
    /// # Errors
    ///
    /// As [`write`](Self::write).
    pub async fn flush(&mut self) -> Result<(), Error> {
        poll_fn(|cx| {
            self.state.tx_waker.register(cx.waker());
            if self.state.buffered.tx.is_empty() && read8(self.base, INTFLAG) & (DRE | TXC) == DRE | TXC {
                Poll::Ready(Ok(()))
            } else {
                write8(self.base, INTENSET, TXC | DRE);
                Poll::Pending
            }
        })
        .await
    }
}

impl BufferedUartRx<'_> {
    /// The receive error noted since the last read, if any, cleared.
    fn take_error(&self) -> Option<Error> {
        let bits = u16::from(self.state.buffered.errors.swap(0, Ordering::AcqRel));
        if bits & BUFOVF != 0 {
            Some(Error::Overrun)
        } else if bits & FERR != 0 {
            Some(Error::Framing)
        } else if bits & PERR != 0 {
            Some(Error::Parity)
        } else {
            None
        }
    }

    /// Copies what the receive ring holds into `buffer`, making room and
    /// letting the handler take bytes again: how many.
    fn take(&mut self, buffer: &mut [u8]) -> usize {
        // SAFETY: this half is the receive ring's only reader.
        let mut reader = unsafe { self.state.buffered.rx.reader() };
        let mut n = 0;
        while n < buffer.len() {
            let data = reader.pop_slice();
            if data.is_empty() {
                break;
            }
            let k = data.len().min(buffer.len() - n);
            buffer[n..n + k].copy_from_slice(&data[..k]);
            reader.pop_done(k);
            n += k;
        }
        if n > 0 {
            // Room again: let the handler take what waits in the SERCOM.
            write8(self.base, INTENSET, RXC);
        }
        n
    }

    /// Reads what has come, waiting until something has: how many bytes.
    ///
    /// # Errors
    ///
    /// A receive error seen since the last read (overrun before framing
    /// before parity), reported once; the bytes stay for the next read.
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if buffer.is_empty() {
            return Ok(0);
        }
        poll_fn(|cx| {
            self.state.rx_waker.register(cx.waker());
            if let Some(error) = self.take_error() {
                return Poll::Ready(Err(error));
            }
            let n = self.take(buffer);
            if n == 0 {
                write8(self.base, INTENSET, RXC | ERROR);
                Poll::Pending
            } else {
                Poll::Ready(Ok(n))
            }
        })
        .await
    }

    /// Fills `buffer` exactly.
    ///
    /// # Errors
    ///
    /// As [`read`](Self::read); what came before the error is in `buffer`.
    pub async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        let mut at = 0;
        while at < buffer.len() {
            at += self.read(&mut buffer[at..]).await?;
        }
        Ok(())
    }

    /// Bytes waiting in the receive ring.
    #[must_use]
    pub fn available(&self) -> usize {
        self.state.buffered.rx.available()
    }

    /// What the receive ring holds, up to where it wraps, waiting until it
    /// holds something; [`consume`](Self::consume) then says how much of it
    /// was used. The bytes stay put until then, so a line can be parsed
    /// where it lies.
    ///
    /// # Errors
    ///
    /// As [`read`](Self::read): an error seen since the last read, reported
    /// once, with the bytes kept.
    pub async fn fill_buf(&mut self) -> Result<&[u8], Error> {
        let (data, n) = poll_fn(|cx| {
            self.state.rx_waker.register(cx.waker());
            if let Some(error) = self.take_error() {
                return Poll::Ready(Err(error));
            }
            // SAFETY: this half is the receive ring's only reader.
            let mut reader = unsafe { self.state.buffered.rx.reader() };
            let (data, n) = reader.pop_buf();
            if n == 0 {
                write8(self.base, INTENSET, RXC | ERROR);
                Poll::Pending
            } else {
                Poll::Ready(Ok((data, n)))
            }
        })
        .await?;
        // SAFETY: the handler writes only the ring's free part, and these
        // bytes stay unfree until `consume`, which needs `&mut self` and so
        // ends this borrow first.
        Ok(unsafe { core::slice::from_raw_parts(data, n) })
    }

    /// Frees the first `amount` bytes [`fill_buf`](Self::fill_buf) gave,
    /// letting the handler take bytes again if the ring was full.
    pub fn consume(&mut self, amount: usize) {
        if amount == 0 {
            return;
        }
        // SAFETY: this half is the receive ring's only reader.
        let mut reader = unsafe { self.state.buffered.rx.reader() };
        reader.pop_done(amount);
        write8(self.base, INTENSET, RXC);
    }

    /// Reads what has come, spinning until something has, for code that
    /// isn't async.
    ///
    /// # Errors
    ///
    /// As [`read`](Self::read).
    pub fn blocking_read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if buffer.is_empty() {
            return Ok(0);
        }
        write8(self.base, INTENSET, RXC | ERROR);
        loop {
            if let Some(error) = self.take_error() {
                return Err(error);
            }
            let n = self.take(buffer);
            if n > 0 {
                return Ok(n);
            }
        }
    }

    /// The baud rate, which the transmitter shares.
    #[must_use]
    pub fn baudrate(&self) -> u32 {
        self.state.baudrate(self.base)
    }

    /// Changes the baud rate, for the transmitter too: as
    /// [`BufferedUart::set_baudrate`].
    ///
    /// # Errors
    ///
    /// As [`BufferedUart::set_baudrate`].
    pub fn set_baudrate(&mut self, baudrate: u32) -> Result<(), ConfigError> {
        self.state.set_baudrate(self.base, baudrate)
    }

    /// Changes the configuration, for the transmitter too: as
    /// [`BufferedUart::set_config`].
    ///
    /// # Errors
    ///
    /// As [`BufferedUart::set_config`].
    pub fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        self.state.configure(self.base, config)
    }
}

impl Drop for BufferedUartTx<'_> {
    fn drop(&mut self) {
        write8(self.base, INTENCLR, DRE | TXC);
        take_down(self.state, self.base);
    }
}

impl Drop for BufferedUartRx<'_> {
    fn drop(&mut self) {
        write8(self.base, INTENCLR, RXC | ERROR);
        take_down(self.state, self.base);
    }
}

/// When the second half goes, the rings come down and the SERCOM's handler
/// goes back to the plain UART's.
fn take_down(state: &super::State, _base: usize) {
    if state.buffered.halves.fetch_sub(1, Ordering::AcqRel) == 1 {
        state.buffered.active.store(false, Ordering::Release);
        // SAFETY: neither half uses the rings any more, and the handler sees
        // `active` false first.
        unsafe {
            state.buffered.rx.deinit();
            state.buffered.tx.deinit();
        }
    }
}

macro_rules! io {
    ($($t:ident),*) => {
        $(
            impl embedded_io::ErrorType for $t<'_> {
                type Error = Error;
            }
        )*
    };
}

io!(BufferedUart, BufferedUartTx, BufferedUartRx);

impl embedded_io_async::Write for BufferedUartTx<'_> {
    async fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        BufferedUartTx::write(self, bytes).await
    }

    async fn flush(&mut self) -> Result<(), Error> {
        BufferedUartTx::flush(self).await
    }
}

impl embedded_io_async::Write for BufferedUart<'_> {
    async fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        self.tx.write(bytes).await
    }

    async fn flush(&mut self) -> Result<(), Error> {
        self.tx.flush().await
    }
}

impl embedded_io_async::Read for BufferedUartRx<'_> {
    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        BufferedUartRx::read(self, buffer).await
    }
}

impl embedded_io_async::Read for BufferedUart<'_> {
    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        self.rx.read(buffer).await
    }
}

impl embedded_io_async::ReadReady for BufferedUartRx<'_> {
    fn read_ready(&mut self) -> Result<bool, Error> {
        Ok(!self.state.buffered.rx.is_empty())
    }
}

impl embedded_io_async::ReadReady for BufferedUart<'_> {
    fn read_ready(&mut self) -> Result<bool, Error> {
        embedded_io_async::ReadReady::read_ready(&mut self.rx)
    }
}

impl embedded_io_async::BufRead for BufferedUartRx<'_> {
    async fn fill_buf(&mut self) -> Result<&[u8], Error> {
        BufferedUartRx::fill_buf(self).await
    }

    fn consume(&mut self, amount: usize) {
        BufferedUartRx::consume(self, amount);
    }
}

impl embedded_io_async::BufRead for BufferedUart<'_> {
    async fn fill_buf(&mut self) -> Result<&[u8], Error> {
        self.rx.fill_buf().await
    }

    fn consume(&mut self, amount: usize) {
        self.rx.consume(amount);
    }
}

impl embedded_io_async::WriteReady for BufferedUartTx<'_> {
    fn write_ready(&mut self) -> Result<bool, Error> {
        Ok(!self.state.buffered.tx.is_full())
    }
}

impl embedded_io_async::WriteReady for BufferedUart<'_> {
    fn write_ready(&mut self) -> Result<bool, Error> {
        embedded_io_async::WriteReady::write_ready(&mut self.tx)
    }
}

impl embedded_io::Read for BufferedUartRx<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        self.blocking_read(buffer)
    }
}

impl embedded_io::Read for BufferedUart<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        self.rx.blocking_read(buffer)
    }
}

impl embedded_io::Write for BufferedUartTx<'_> {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        self.blocking_write(bytes)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.blocking_flush()
    }
}

impl embedded_io::Write for BufferedUart<'_> {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        self.tx.blocking_write(bytes)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.tx.blocking_flush()
    }
}

impl embassy_embedded_hal::SetConfig for BufferedUart<'_> {
    type Config = Config;
    type ConfigError = ConfigError;

    fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        BufferedUart::set_config(self, config)
    }
}

impl embassy_embedded_hal::SetConfig for BufferedUartTx<'_> {
    type Config = Config;
    type ConfigError = ConfigError;

    fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        BufferedUartTx::set_config(self, config)
    }
}

impl embassy_embedded_hal::SetConfig for BufferedUartRx<'_> {
    type Config = Config;
    type ConfigError = ConfigError;

    fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        BufferedUartRx::set_config(self, config)
    }
}

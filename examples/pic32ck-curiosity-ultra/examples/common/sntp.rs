// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Wall-clock time by SNTP (spike quality), with `sntpc` over embassy-net.
//!
//! [`run`] keeps the clock set: it asks the DHCP gateway first (the bench's
//! router serves NTP), then each address `pool.ntp.org` resolves to, three
//! seconds each; on success it syncs again in 15 minutes, on failure in 30
//! seconds. If no server ever answers the board runs clockless: [`now`] is
//! `None` until the first sync, and after one it keeps counting from the
//! last good offset.
//!
//! The gateway stands in for the NTP servers DHCP option 42 would name:
//! embassy-net 0.9.1 doesn't expose them (its `dhcpv4-ntp` feature, merged
//! upstream, isn't released yet).
//!
//! `SNTP_TEST=pool` or `SNTP_TEST=none` at build time skips the gateway, or
//! every source, to test the fallbacks.
//!
//! The clock is the Unix time of uptime zero plus embassy-time's uptime, so
//! it's as steady as the uptime clock and steps only when a sync corrects
//! it.

use core::cell::Cell;
use core::fmt::Write as _;
use core::net::{IpAddr, SocketAddr};

use critical_section::Mutex;
use embassy_net::dns::DnsQueryType;
use embassy_net::iface::Iface;
use embassy_net::udp::UdpSocket;
use embassy_net::wire::ListenSocketAddr;
use embassy_time::{Duration, Instant, Timer, with_timeout};

use pic32ck_curiosity_ultra::uart::Uart;

/// The pool, for when the gateway doesn't answer.
const POOL: &str = "pool.ntp.org";
const TIMEOUT: Duration = Duration::from_secs(3);
const RESYNC: Duration = Duration::from_secs(15 * 60);
const RETRY: Duration = Duration::from_secs(30);

/// Where the time last came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// The DHCP gateway.
    Gateway(IpAddr),
    /// One of `pool.ntp.org`'s addresses.
    Pool(IpAddr),
}

/// The last sync.
#[derive(Debug, Clone, Copy)]
pub struct Sync {
    pub source: Source,
    /// Uptime at the sync, in seconds.
    pub at: u64,
    pub stratum: u8,
    /// The correction it made, in µs (the first sets the clock from zero).
    pub offset_us: i64,
    pub roundtrip_us: u64,
}

/// Unix time at uptime zero, in µs, and the last sync; `None` before one.
static STATE: Mutex<Cell<Option<(i64, Sync)>>> = Mutex::new(Cell::new(None));

/// Microseconds since the Unix epoch, or `None` before the first sync.
#[must_use]
pub fn now_us() -> Option<u64> {
    let (base, _) = critical_section::with(|cs| STATE.borrow(cs).get())?;
    u64::try_from(base + i64::try_from(Instant::now().as_micros()).ok()?).ok()
}

/// The last sync, if any.
#[must_use]
pub fn last_sync() -> Option<Sync> {
    critical_section::with(|cs| STATE.borrow(cs).get()).map(|(_, sync)| sync)
}

/// The timestamps `sntpc` stamps its requests with: our clock, or Unix
/// zero plus uptime before the first sync (the reply's offset corrects
/// either).
#[derive(Clone, Copy, Default)]
struct Clock {
    us: u64,
}

impl sntpc::NtpTimestampGenerator for Clock {
    fn init(&mut self) {
        self.us = now_us().unwrap_or_else(|| Instant::now().as_micros());
    }

    fn timestamp_sec(&self) -> u64 {
        self.us / 1_000_000
    }

    fn timestamp_subsec_micros(&self) -> u32 {
        (self.us % 1_000_000) as u32
    }
}

/// embassy-net's UDP socket as `sntpc` wants it.
struct Socket<'d>(UdpSocket<'d>);

impl sntpc::NtpUdpSocket for Socket<'_> {
    async fn send_to(&self, buf: &[u8], addr: SocketAddr) -> sntpc::Result<usize> {
        self.0.send_to(buf, addr).await.map_err(|_| sntpc::Error::Network)?;
        Ok(buf.len())
    }

    async fn recv_from(&self, buf: &mut [u8]) -> sntpc::Result<(usize, SocketAddr)> {
        let (n, meta) = self.0.recv_from(buf).await.map_err(|_| sntpc::Error::Network)?;
        Ok((n, SocketAddr::from(meta.remote_addr)))
    }
}

/// One exchange with `server`.
async fn ask(socket: &Socket<'_>, server: IpAddr) -> Option<sntpc::NtpResult> {
    let context = sntpc::NtpContext::new(Clock::default());
    match with_timeout(TIMEOUT, sntpc::get_time(SocketAddr::new(server, 123), socket, context)).await {
        Ok(Ok(result)) => Some(result),
        Ok(Err(e)) => {
            let _ = writeln!(Uart, "sntp: {server}: {e:?}");
            None
        }
        Err(_) => {
            let _ = writeln!(Uart, "sntp: {server}: no answer in {} s", TIMEOUT.as_secs());
            None
        }
    }
}

/// Applies a result: moves the clock by its offset.
fn apply(source: Source, result: &sntpc::NtpResult) -> Sync {
    let sync = Sync {
        source,
        at: Instant::now().as_secs(),
        stratum: result.stratum(),
        offset_us: result.offset(),
        roundtrip_us: result.roundtrip(),
    };
    critical_section::with(|cs| {
        let state = STATE.borrow(cs);
        let base = state.get().map_or(0, |(base, _)| base) + result.offset();
        state.set(Some((base, sync)));
    });
    sync
}

/// Tries the gateway, then the pool; the sync, or `None` if nothing answered.
async fn sync_once(iface: Iface<'_>, socket: &Socket<'_>) -> Option<Sync> {
    // `SNTP_TEST` at build time skips sources, to test the fallbacks:
    // `pool` skips the gateway, `none` skips everything (clockless).
    let test = option_env!("SNTP_TEST");
    if test == Some("none") {
        return None;
    }
    if test != Some("pool")
        && let Some(gateway) = crate::net::gateway_v4(iface)
    {
        let server = IpAddr::V4(gateway);
        if let Some(result) = ask(socket, server).await {
            return Some(apply(Source::Gateway(server), &result));
        }
    }
    match iface.stack().dns_query(POOL, DnsQueryType::A).await {
        Ok(addresses) => {
            for address in addresses {
                let server = IpAddr::from(address);
                if let Some(result) = ask(socket, server).await {
                    return Some(apply(Source::Pool(server), &result));
                }
            }
        }
        Err(e) => {
            let _ = writeln!(Uart, "sntp: {POOL}: {e:?}");
        }
    }
    None
}

/// Keeps the clock set, forever. Run it beside the servers.
pub async fn run(iface: Iface<'_>) -> ! {
    // Datagrams wait in the stack's pool: the socket holds no buffers.
    let mut udp = UdpSocket::new(iface.stack()).expect("a UDP socket (xarxa's udp-socket-count-N)");
    // An ephemeral local port.
    let _ = udp.bind(0, ListenSocketAddr::UNSPECIFIED);
    let socket = Socket(udp);
    // IPv4 from DHCP (the gateway, and the DNS server for the pool): a
    // SLAAC address alone makes `wait_config_up` return first.
    let _ = with_timeout(Duration::from_secs(20), async {
        while crate::net::v4(iface).is_none() {
            Timer::after(Duration::from_millis(250)).await;
        }
    })
    .await;
    loop {
        let wait = if let Some(sync) = sync_once(iface, &socket).await {
            let _ = writeln!(
                Uart,
                "sntp: {:?}, stratum {}, offset {} us, roundtrip {} us; Unix time {} s",
                sync.source,
                sync.stratum,
                sync.offset_us,
                sync.roundtrip_us,
                now_us().unwrap_or(0) / 1_000_000
            );
            RESYNC
        } else {
            let _ = writeln!(Uart, "sntp: no server answered; {}", if now_us().is_some() { "keeping the last offset" } else { "clockless" });
            RETRY
        };
        Timer::after(wait).await;
    }
}

// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! An mDNS responder (spike quality), with `edge-mdns` over embassy-net:
//! `<name>.local` answers with the board's IPv4 address and its global
//! IPv6 one (the link-local one when there's no global), and DNS-SD
//! advertises the services [`run`] is given (`_https._tcp` on 443 for
//! `eth_https`).
//!
//! [`host_name`] makes the name from the MAC, `pic32ck-` and its last three
//! bytes, so each board's is its own. When the addresses change (a new DHCP
//! lease, SLAAC), the responder starts over with the new ones and announces
//! them.

use core::fmt::Write as _;
use core::net::{Ipv4Addr, Ipv6Addr};

use edge_mdns::buf::VecBufAccess;
use edge_mdns::domain::base::Ttl;
use edge_mdns::host::{Host, Service, ServiceAnswers};
use edge_mdns::io::{self, DEFAULT_SOCKET};
use edge_mdns::{HostAnswersMdnsHandler, NoHostAnswers};
use edge_nal::UdpSplit as _;
use edge_nal_embassy::Udp;
use embassy_futures::select::{Either, select};
use embassy_net::iface::Iface;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer, with_timeout};

use pic32ck_curiosity_ultra::uart::Uart;

/// `pic32ck-` and the MAC's last three bytes, in hex.
#[must_use]
pub fn host_name(iface: Iface<'_>) -> heapless::String<16> {
    let mac = crate::net::mac(iface);
    let mut name = heapless::String::new();
    let _ = write!(name, "pic32ck-{:02x}{:02x}{:02x}", mac[3], mac[4], mac[5]);
    name
}

/// The addresses to answer with: IPv4 from DHCP (unspecified if none yet),
/// and the global IPv6 one, else the link-local one (xarxa reports it now).
fn addresses(iface: Iface<'_>) -> (Ipv4Addr, Ipv6Addr) {
    let v4 = crate::net::v4(iface).map_or(Ipv4Addr::UNSPECIFIED, |c| c.address());
    let v6 = crate::net::v6(iface).map(|c| c.address()).or_else(|| crate::net::link_local(iface)).unwrap_or(Ipv6Addr::UNSPECIFIED);
    (v4, v6)
}

/// A small PRNG for the responder's random delays (RFC 6762's 20-120 ms
/// jitter): nothing secret, so no TRNG.
struct Jitter(u64);

impl rand_core_010::TryRng for Jitter {
    type Error = core::convert::Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok((self.try_next_u64()? >> 32) as u32)
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        Ok(self.0.wrapping_mul(0x2545_F491_4F6C_DD1D))
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        for chunk in dest.chunks_mut(8) {
            let word = self.try_next_u64()?.to_le_bytes();
            chunk.copy_from_slice(&word[..chunk.len()]);
        }
        Ok(())
    }
}

/// A service to advertise: DNS-SD `_<service>._<protocol>` on `port`.
#[derive(Debug, Clone, Copy)]
pub struct Advert {
    pub service: &'static str,
    pub protocol: &'static str,
    pub port: u16,
}

/// Answers for `name`, with `advert`, forever; starts over when the
/// addresses change. `seed` seeds the jitter (the chip's unique ID will do).
pub async fn run(iface: Iface<'_>, name: &str, advert: Advert, seed: u64) -> ! {
    // The socket's datagrams wait in the stack's pool.
    let udp = Udp::new(iface.stack());
    let (recv_buf, send_buf) = (VecBufAccess::<NoopRawMutex, 1500>::new(), VecBufAccess::<NoopRawMutex, 1500>::new());
    // Wait for IPv4 (or give up after 20 s and answer with what there is).
    let _ = with_timeout(Duration::from_secs(20), async {
        while crate::net::v4(iface).is_none() {
            Timer::after(Duration::from_millis(250)).await;
        }
    })
    .await;
    let mut socket = loop {
        match io::bind(&udp, DEFAULT_SOCKET, Some(Ipv4Addr::UNSPECIFIED), Some(0)).await {
            Ok(socket) => break socket,
            Err(e) => {
                let _ = writeln!(Uart, "mdns: bind: {e:?}");
                Timer::after(Duration::from_secs(5)).await;
            }
        }
    };
    let (rx, tx) = socket.split();
    let signal = Signal::<NoopRawMutex, ()>::new();
    let mdns = io::Mdns::<_, _, _, _, _, NoopRawMutex>::new(Some(Ipv4Addr::UNSPECIFIED), Some(0), rx, tx, &recv_buf, &send_buf, Jitter(seed | 1), &signal);
    loop {
        let (ipv4, ipv6) = addresses(iface);
        let host = Host {
            hostname: name,
            ipv4,
            ipv6,
            ttl: Ttl::from_secs(120),
        };
        let service = Service {
            name,
            priority: 0,
            weight: 0,
            service: advert.service,
            protocol: advert.protocol,
            port: advert.port,
            service_subtypes: &[],
            txt_kvs: &[],
        };
        let service_answers = ServiceAnswers::new(&host, &service);
        let _ = writeln!(Uart, "mdns: {name}.local -> {ipv4}, {ipv6}; {}.{} on {}", advert.service, advert.protocol, advert.port);
        let handler = HostAnswersMdnsHandler::new(NoHostAnswers.chain(&host).chain(&service_answers));
        let changed = async {
            loop {
                Timer::after(Duration::from_secs(5)).await;
                if addresses(iface) != (ipv4, ipv6) {
                    break;
                }
            }
        };
        match select(mdns.run(handler), changed).await {
            Either::First(Err(e)) => {
                let _ = writeln!(Uart, "mdns: {e:?}");
                Timer::after(Duration::from_secs(1)).await;
            }
            Either::First(Ok(())) | Either::Second(()) => {}
        }
    }
}

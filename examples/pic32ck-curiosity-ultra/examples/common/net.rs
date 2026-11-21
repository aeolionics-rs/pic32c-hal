// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! What the network examples share on embassy-net's xarxa stack: an
//! interface's addresses, and a listener several tasks accept from.
//!
//! On xarxa a port has one [`TcpListener`], its queue of connection
//! attempts, and taking an attempt needs the listener mutably: tasks that
//! serve the same port share it behind a mutex, each holding it only while
//! it waits for the next attempt. Attempts beyond those being served wait in
//! the listener's backlog (xarxa's `tcp-listener-backlog-N`, 4) instead of
//! being refused.

#![allow(dead_code, reason = "each example uses part of it")]

use core::net::{Ipv4Addr, Ipv6Addr};

use embassy_net::iface::{AddrOrigin, Iface};
use embassy_net::tcp::{AcceptError, TcpListener, TcpSocket};
use embassy_net::wire::{HardwareAddress, IpCidr, Ipv4Cidr, Ipv6Cidr};
use embassy_net::Stack;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;

/// A port's listener, shared by the tasks that serve it.
pub type SharedListener<'d> = Mutex<NoopRawMutex, TcpListener<'d>>;

/// A listener on `port`.
///
/// # Panics
///
/// If the stack has no listener left (xarxa's `tcp-listener-count-N`), or
/// another listens on the port.
pub fn listen(stack: Stack<'_>, port: u16) -> SharedListener<'_> {
    let mut listener = TcpListener::new(stack).expect("a TCP listener (xarxa's tcp-listener-count-N)");
    listener.listen(port).expect("the port free");
    Mutex::new(listener)
}

/// The next connection on `listener`, its handshake done, in a socket on
/// `rx` and `tx`.
///
/// # Errors
///
/// If the stack has no TCP socket left (xarxa's `tcp-socket-count-N`), or
/// the client reset the connection during the handshake.
pub async fn accept<'a, 'd>(stack: Stack<'d>, listener: &SharedListener<'d>, rx: &'a mut [u8], tx: &'a mut [u8]) -> Result<TcpSocket<'a, 'd>, AcceptError> {
    let token = listener.lock().await.accept().await?;
    let mut socket = TcpSocket::new(stack, rx, tx).map_err(|_| AcceptError::InvalidState)?;
    socket.accept(token).await?;
    Ok(socket)
}

/// The interface's MAC.
pub fn mac(iface: Iface<'_>) -> [u8; 6] {
    match iface.hardware_addr() {
        HardwareAddress::Ethernet(mac) => mac.0,
        #[allow(unreachable_patterns, reason = "with medium-ip the stack has other kinds")]
        _ => [0; 6],
    }
}

/// The interface's IPv4 address and prefix, from DHCP or set by hand.
pub fn v4(iface: Iface<'_>) -> Option<Ipv4Cidr> {
    iface.ip_addrs().iter().find_map(|a| match a.cidr {
        IpCidr::V4(cidr) => Some(cidr),
        IpCidr::V6(_) => None,
    })
}

/// The interface's first IPv6 address other than the link-local one
/// (SLAAC's, or one set by hand).
pub fn v6(iface: Iface<'_>) -> Option<Ipv6Cidr> {
    iface.ip_addrs().iter().find_map(|a| match a.cidr {
        IpCidr::V6(cidr) if a.origin != AddrOrigin::LinkLocal => Some(cidr),
        _ => None,
    })
}

/// The interface's IPv6 link-local address, which xarxa forms itself.
pub fn link_local(iface: Iface<'_>) -> Option<Ipv6Addr> {
    iface.ip_addrs().iter().find_map(|a| match a.cidr {
        IpCidr::V6(cidr) if a.origin == AddrOrigin::LinkLocal => Some(cidr.address()),
        _ => None,
    })
}

/// The IPv4 router the DHCP server named.
pub fn gateway_v4(iface: Iface<'_>) -> Option<Ipv4Addr> {
    iface.dhcpv4_lease().and_then(|lease| lease.router)
}

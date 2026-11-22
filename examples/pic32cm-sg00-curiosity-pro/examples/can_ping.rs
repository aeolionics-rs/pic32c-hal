// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! CAN0 (J502) on a real bus at 250 kbit/s, or, built with `CAN_FD=1`, CAN1
//! (J503) as FD at 500 kbit/s and 2 Mbit/s, against a node that sends each
//! frame back with its identifier's top bit set (bit 10 of a standard ID,
//! bit 28 of an extended one), such as the CK board's `can_echo` or
//! `tools/host/can_echo.py` on a Linux host.
//!
//! In each of ten rounds:
//! - every kind of classic frame (standard and extended IDs, data and
//!   remote, every length from 0 to 8) goes out one at a time, and each must
//!   be reported sent and its echo come back with the same data and DLC;
//! - FD built, every kind of FD frame too (every length, with and without
//!   bit-rate switching);
//! - eight frames (64-byte FD ones, FD built) go out at once, and all eight
//!   must be reported sent, in order, and all eight echoes come back.
//!
//! DLCs over 8 aren't sent: a Linux peer sees them as 8 (`can_loopback`
//! covers them). Frames from the bus's other nodes are counted and otherwise
//! ignored. The bus state and error counters must stay clean. Not for a bus
//! with NMEA 2000 devices on it: they'd take the extended frames as
//! messages. Results go out over the virtual COM port; LED0 blinks slowly
//! when all passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::sync::atomic::{AtomicU32, Ordering};
use core::time::Duration;

use can::Channel;
use can_hal::{BusState, BusStatus, CanFdFrame, CanFrame, CanId, ConfirmedTransmit, Event, Frame, Outcome, ReceiveEvents, Statistics, TransmitId};
use cortex_m_rt::entry;
use embassy_time::block_for;
use panic_halt as _;
use pic32c_hal::can::Mode;
use pic32c_hal::gpio::{Level, Output};
use pic32cm_sg00_curiosity_pro::{Log, micros};

#[path = "common/can.rs"]
#[allow(dead_code, reason = "shared by several examples; each uses some of it")]
mod can;

pic32cm_sg00_curiosity_pro::boot!();

const ROUNDS: u32 = 10;
const TIMEOUT: Duration = Duration::from_millis(100);
/// The pings' IDs: one of each kind, and the eight sent at once.
const STANDARD: u16 = 0x123;
const EXTENDED: u32 = 0x0234_5678;
const EIGHT: u16 = 0x100;
/// Whether this is the FD build, on CAN1.
const FD: bool = option_env!("CAN_FD").is_some();

/// Frames from the bus's other nodes received meanwhile.
static OTHERS: AtomicU32 = AtomicU32::new(0);

fn echo_id(id: CanId) -> CanId {
    match id {
        CanId::Standard(id) => CanId::new_standard(id | 0x400).unwrap(),
        CanId::Extended(id) => CanId::new_extended(id | 0x1000_0000).unwrap(),
    }
}

fn echo_of(frame: &Frame) -> Frame {
    let id = echo_id(frame.id());
    match frame {
        Frame::Can(can) if can.is_remote_frame() => Frame::Can(CanFrame::new_remote(id, can.dlc()).unwrap()),
        Frame::Can(can) => Frame::Can(CanFrame::new_with_dlc(id, can.data(), can.dlc()).unwrap()),
        Frame::Fd(fd) => Frame::Fd(CanFdFrame::new(id, fd.data(), fd.brs(), false).unwrap()),
    }
}

/// Whether `id` is one of this test's echoes.
fn ours(id: CanId) -> bool {
    match id {
        CanId::Standard(id) => id == STANDARD | 0x400 || ((EIGHT | 0x400)..((EIGHT + 8) | 0x400)).contains(&id),
        CanId::Extended(id) => id == EXTENDED | 0x1000_0000,
    }
}

#[entry]
fn main() -> ! {
    let peripherals = pic32cm_sg00_curiosity_pro::init();
    let led = Output::new(peripherals.PA20, Level::High);
    let mut log = Log::init(peripherals.SERCOM4, peripherals.PD19, peripherals.PD20);
    let opened = if FD {
        let _ = writeln!(log, "\npic32cm-sg00-curiosity-pro ping: CAN1, FD, {} and {} bit/s", can::FD_RATES.0, can::FD_RATES.1);
        can::open_fd(peripherals.CAN1, Mode::Normal, peripherals.PD13, peripherals.PD12)
    } else {
        let _ = writeln!(log, "\npic32cm-sg00-curiosity-pro ping: CAN0, classic, {} bit/s", can::CLASSIC_RATE);
        can::open(peripherals.CAN0, can::CLASSIC_RATE, Mode::Normal, peripherals.PA11, peripherals.PA10)
    };
    let mut can = match opened {
        Ok(can) => can,
        Err(e) => {
            let _ = writeln!(log, "open: {e:?}");
            halt(&mut log, 1, led);
        }
    };
    let start = micros();
    while !can.is_synchronized() && micros() - start < 100_000 {}
    let _ = writeln!(log, "synchronised {}", can.is_synchronized());

    let mut failures = 0;
    let started = micros();
    for round in 0..ROUNDS {
        let before = failures;
        failures += every_classic_frame(&mut can, &mut log);
        if FD {
            failures += every_fd_frame(&mut can, &mut log);
        }
        failures += eight_at_once(&mut can, &mut log, round);
        if failures != before {
            let _ = writeln!(log, "round {round}: {} failures", failures - before);
            if failures > 20 {
                break;
            }
        }
    }
    let _ = writeln!(log, "{ROUNDS} rounds in {} ms", (micros() - started) / 1000);
    let _ = writeln!(log, "other nodes' frames meanwhile: {}", OTHERS.load(Ordering::Relaxed));
    let _ = writeln!(log, "bus state {:?}, counters {:?}", can.bus_state(), can.error_counters());
    let _ = writeln!(log, "statistics {:?}", can.statistics());
    if can.bus_state() != Ok(BusState::ErrorActive) {
        failures += 1;
    }
    if failures == 0 {
        let _ = writeln!(log, "ping: all passed");
    } else {
        let _ = writeln!(log, "ping: {failures} failures");
    }
    halt(&mut log, failures, led);
}

fn halt(log: &mut Log, failures: u32, mut led: Output<'static>) -> ! {
    log.flush();
    let period = if failures == 0 { 500 } else { 100 };
    loop {
        led.toggle();
        block_for(embassy_time::Duration::from_millis(period));
    }
}

/// The next event that isn't another node's frame.
fn next(can: &mut Channel) -> Option<Event<Frame, embassy_time::Instant>> {
    loop {
        let event = can.receive_event_timeout(TIMEOUT).ok().flatten();
        if let Some(Event::Received(r)) = &event
            && !ours(r.frame().id())
        {
            OTHERS.fetch_add(1, Ordering::Relaxed);
            continue;
        }
        return event;
    }
}

fn ping(can: &mut Channel, log: &mut Log, frame: &Frame) -> u32 {
    let tx = match ConfirmedTransmit::<Frame>::try_transmit(can, frame) {
        Ok(tx) => tx,
        Err(e) => {
            let _ = writeln!(log, "{frame:?}: transmit {e:?}");
            return 1;
        }
    };
    let expected = echo_of(frame);
    let (mut echoed, mut sent) = (false, false);
    while !(echoed && sent) {
        match next(can) {
            Some(Event::Received(r)) if *r.frame() == expected => echoed = true,
            Some(Event::Transmitted { id, outcome: Outcome::Sent(_) }) if id == tx => sent = true,
            other => {
                let _ = writeln!(log, "{frame:?}: sent {sent}, echoed {echoed}, then {other:?}");
                return 1;
            }
        }
    }
    0
}

fn every_classic_frame(can: &mut Channel, log: &mut Log) -> u32 {
    let data = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let ids = [CanId::new_standard(STANDARD).unwrap(), CanId::new_extended(EXTENDED).unwrap()];
    let mut failures = 0;
    for id in ids {
        for len in 0..=8usize {
            for frame in [CanFrame::new(id, &data[..len]).unwrap(), CanFrame::new_remote(id, len).unwrap()] {
                failures += ping(can, log, &Frame::Can(frame));
            }
        }
    }
    failures
}

fn every_fd_frame(can: &mut Channel, log: &mut Log) -> u32 {
    let mut data = [0u8; 64];
    for (i, byte) in (0u8..).zip(data.iter_mut()) {
        *byte = i.wrapping_mul(37) ^ 0x5A;
    }
    let ids = [CanId::new_standard(STANDARD).unwrap(), CanId::new_extended(EXTENDED).unwrap()];
    let mut failures = 0;
    for id in ids {
        for len in [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 16, 20, 24, 32, 48, 64] {
            for brs in [false, true] {
                failures += ping(can, log, &Frame::Fd(CanFdFrame::new(id, &data[..len], brs, false).unwrap()));
            }
        }
    }
    failures
}

fn eight_at_once(can: &mut Channel, log: &mut Log, round: u32) -> u32 {
    #[expect(clippy::cast_possible_truncation, reason = "a byte of the round")]
    let r = round as u8;
    let frame = |n: u8| {
        let id = CanId::new_standard(EIGHT + u16::from(n)).unwrap();
        if FD {
            let mut data = [0u8; 64];
            for (i, byte) in (0u8..).zip(data.iter_mut()) {
                *byte = n ^ r ^ i;
            }
            Frame::Fd(CanFdFrame::new(id, &data, true, false).unwrap())
        } else {
            Frame::Can(CanFrame::new(id, &[n, r, 0x55, 0xAA, n, r, 0xFF, 0x00]).unwrap())
        }
    };
    let mut ids: [Option<TransmitId>; 8] = [None; 8];
    for n in 0..8u8 {
        match ConfirmedTransmit::<Frame>::try_transmit(can, &frame(n)) {
            Ok(tx) => ids[usize::from(n)] = Some(tx),
            Err(e) => {
                let _ = writeln!(log, "eight at once: frame {n}: transmit {e:?}");
                return 1;
            }
        }
    }
    let (mut sent, mut echoed) = (0usize, [false; 8]);
    while sent < 8 || echoed.iter().any(|e| !e) {
        match next(can) {
            Some(Event::Transmitted { id, outcome: Outcome::Sent(_) }) if Some(id) == ids[sent] => sent += 1,
            Some(Event::Received(r)) => match r.frame() {
                got if (0..8u8).any(|n| *got == echo_of(&frame(n))) => {
                    let CanId::Standard(id) = got.id() else { unreachable!() };
                    echoed[usize::from(id & 0x7)] = true;
                }
                other => {
                    let _ = writeln!(log, "eight at once: sent {sent}, echoes {echoed:?}, then received {other:?}");
                    return 1;
                }
            },
            other => {
                let _ = writeln!(log, "eight at once: sent {sent}, echoes {echoed:?}, then {other:?}");
                return 1;
            }
        }
    }
    0
}

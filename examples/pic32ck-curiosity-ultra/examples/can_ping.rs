// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! CAN0 on a real bus at 250 kbit/s (FD at 500 kbit/s and 2 Mbit/s when
//! built with `CAN_FD=1`; `can_echo` must be built the same way), against a node
//! running `can_echo` (which sends each frame back with its identifier's top bit
//! set).
//!
//! In each of ten rounds:
//! - every kind of classic frame (standard and extended IDs, data and
//!   remote, every DLC from 0 to 15) goes out one at a time, and each must be
//!   reported sent and its echo come back with the same data and DLC;
//! - FD built, every kind of FD frame too (every length, with and without
//!   bit-rate switching);
//! - eight frames (64-byte FD ones, FD built) go out at once, and all eight must be reported sent, in
//!   order, and all eight echoes come back.
//!
//! The bus state and error counters must stay clean. Results go out over the
//! virtual COM port; LED0 blinks slowly when all passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::time::Duration;

use can_hal::{BusState, BusStatus, CanFdFrame, CanFrame, CanId, ConfirmedTransmit, Event, Frame, Outcome, ReceiveEvents, Statistics, TransmitId};
use can0::Channel;
use cortex_m_rt::entry;
use panic_halt as _;
use pic32ck_curiosity_ultra::pic32c_hal::can::Mode;
use pic32ck_curiosity_ultra::pic32c_hal::gpio::{Level, Output};
use pic32ck_curiosity_ultra::time;
use pic32ck_curiosity_ultra::uart::Uart;
#[path = "common/can0.rs"]
#[allow(dead_code, reason = "shared by several examples; each uses some of it")]
mod can0;

pic32ck_curiosity_ultra::boot!();

const ROUNDS: u32 = 10;
const TIMEOUT: Duration = Duration::from_millis(100);

/// The echo `frame` should get back.
fn echo_of(frame: &Frame) -> Frame {
    let id = match frame.id() {
        CanId::Standard(id) => CanId::new_standard(id | 0x400).unwrap(),
        CanId::Extended(id) => CanId::new_extended(id | 0x1000_0000).unwrap(),
    };
    match frame {
        Frame::Can(can) if can.is_remote_frame() => Frame::Can(CanFrame::new_remote(id, can.dlc()).unwrap()),
        Frame::Can(can) => Frame::Can(CanFrame::new_with_dlc(id, can.data(), can.dlc()).unwrap()),
        Frame::Fd(fd) => Frame::Fd(CanFdFrame::new(id, fd.data(), fd.brs(), false).unwrap()),
    }
}

#[entry]
fn main() -> ! {
    let peripherals = pic32ck_curiosity_ultra::init();
    let led = Output::new(peripherals.PD20, Level::High);
    time::init(cortex_m::Peripherals::take().unwrap());
    let mut log = Uart::init(peripherals.SERCOM5, peripherals.PA24, peripherals.PA25);
    let _ = writeln!(log, "\npic32ck-curiosity-ultra ping: CPU {} Hz", pic32ck_curiosity_ultra::pic32c_hal::clocks::generator_hz(0).unwrap_or(0));
    if can0::FD {
        let _ = writeln!(log, "FD, {} and {} bit/s", can0::FD_RATES.0, can0::FD_RATES.1);
    } else {
        let _ = writeln!(log, "classic, {} bit/s", can0::CLASSIC_RATE);
    }
    let mut can = match can0::open_bus(peripherals.CAN0, Mode::Normal, peripherals.PB16, peripherals.PB17) {
        Ok(can) => can,
        Err(e) => {
            let _ = writeln!(log, "CAN0: {e:?}");
            halt(&mut log, 1, led);
        }
    };
    let start = time::micros();
    while !can.is_synchronized() && time::micros() - start < 100_000 {}
    let _ = writeln!(log, "synchronised {}", can.is_synchronized());

    let mut failures = 0;
    let started = time::micros();
    for round in 0..ROUNDS {
        let before = failures;
        failures += every_classic_frame(&mut can, &mut log);
        if can0::FD {
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
    let _ = writeln!(log, "{ROUNDS} rounds in {} ms", (time::micros() - started) / 1000);
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

fn halt(log: &mut Uart, failures: u32, mut led: Output<'static>) -> ! {
    log.flush();
    let period = if failures == 0 { 500 } else { 100 };
    loop {
        led.toggle();
        time::delay(Duration::from_millis(period));
    }
}

fn next(can: &mut Channel) -> Option<Event<Frame, u64>> {
    can.receive_event_timeout(TIMEOUT).ok().flatten()
}

/// Sends `frame`: it must be reported sent and its echo come back.
fn ping(can: &mut Channel, log: &mut Uart, frame: &Frame) -> u32 {
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

fn every_classic_frame(can: &mut Channel, log: &mut Uart) -> u32 {
    let data = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let ids = [CanId::new_standard(0x123).unwrap(), CanId::new_extended(0x0234_5678).unwrap()];
    let mut failures = 0;
    for id in ids {
        for dlc in 0..=15 {
            for frame in [CanFrame::new_with_dlc(id, &data[..dlc.min(8)], dlc).unwrap(), CanFrame::new_remote(id, dlc).unwrap()] {
                failures += ping(can, log, &Frame::Can(frame));
            }
        }
    }
    failures
}

fn every_fd_frame(can: &mut Channel, log: &mut Uart) -> u32 {
    let mut data = [0u8; 64];
    for (i, byte) in (0u8..).zip(data.iter_mut()) {
        *byte = i.wrapping_mul(37) ^ 0x5A;
    }
    let ids = [CanId::new_standard(0x123).unwrap(), CanId::new_extended(0x0234_5678).unwrap()];
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

/// Eight frames at once: all reported sent in order, all eight echoed.
fn eight_at_once(can: &mut Channel, log: &mut Uart, round: u32) -> u32 {
    let frame = |n: u8| {
        #[expect(clippy::cast_possible_truncation, reason = "a byte of the round")]
        let r = round as u8;
        let id = CanId::new_standard(0x100 + u16::from(n)).unwrap();
        let mut data = [0u8; 64];
        for (i, byte) in (0u8..).zip(data.iter_mut()) {
            *byte = n ^ r ^ i;
        }
        if can0::FD {
            Frame::Fd(CanFdFrame::new(id, &data, true, false).unwrap())
        } else {
            Frame::Can(CanFrame::new(id, &[n, r, 0x55, 0xAA, n, r, 0xFF, 0x00]).unwrap())
        }
    };
    let mut ids = [TransmitId(0); 8];
    for (n, id) in (0u8..).zip(ids.iter_mut()) {
        match ConfirmedTransmit::<Frame>::try_transmit(can, &frame(n)) {
            Ok(tx) => *id = tx,
            Err(e) => {
                let _ = writeln!(log, "frame {n} of 8: {e:?}");
                return 1;
            }
        }
    }
    let (mut reported, mut echoes) = (0usize, 0u8);
    let mut failures = 0;
    while reported < 8 || echoes != 0xFF {
        match next(can) {
            Some(Event::Transmitted { id, outcome }) if reported < 8 => {
                if id != ids[reported] || !matches!(outcome, Outcome::Sent(_)) {
                    let _ = writeln!(log, "report {reported}: {id:?} {outcome:?}, expected {:?} sent", ids[reported]);
                    failures += 1;
                }
                reported += 1;
            }
            Some(Event::Received(r)) => {
                if let Some(n) = (0u8..8).find(|&n| *r.frame() == echo_of(&frame(n))) {
                    echoes |= 1 << n;
                } else {
                    let _ = writeln!(log, "unexpected echo {:?}", r.frame());
                    failures += 1;
                }
            }
            other => {
                let _ = writeln!(log, "eight at once: {reported} reported, echoes {echoes:#04x}, then {other:?}");
                return failures + 1;
            }
        }
    }
    failures
}

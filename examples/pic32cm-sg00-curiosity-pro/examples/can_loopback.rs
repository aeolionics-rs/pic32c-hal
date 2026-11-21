// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! pic32c-hal's `can` on the SG00's CAN0, in internal loopback (`M_CAN`
//! TEST.LBCK with CCCR.MON): the TX pin (PA11) stays recessive, so nothing
//! reaches the board's ATA6561 or a bus on it. An FD channel at 500 kbit/s
//! with a 2 Mbit/s data phase from the 40 MHz kernel clock (generator 2):
//! every kind of classic frame (standard and extended IDs, data and remote,
//! DLC 0 to 15) and every FD frame length, with and without bit-rate
//! switching, must come back both received unchanged and reported sent.
//!
//! The CK board's `can_loopback`, cut down. Results go out over the virtual
//! COM port; LED0 blinks slowly when all passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::time::Duration;

use can_hal::{BusState, BusStatus, CanFdFrame, CanFrame, CanId, ConfirmedTransmit, Event, Frame, Outcome, ReceiveEvents, SamplePoint};
use cortex_m_rt::entry;
use panic_halt as _;
use pic32c_hal::can::{BitTiming, Can, Config, FdMessageRam, InterruptHandler, Mode, State};
use pic32c_hal::gpio::{Level, Output};
use pic32c_hal::uart::{self, Uart};
use pic32c_hal::{bind_interrupts, peripherals};
use pic32cm_sg00_curiosity_pro::GEN_CAN;
use static_cell::ConstStaticCell;

pic32cm_sg00_curiosity_pro::boot!();

bind_interrupts!(struct Irqs {
    CAN0_INT0 => InterruptHandler<peripherals::CAN0>;
});

/// embassy-time as the driver's clock.
struct TimeClock;

impl pic32c_hal::can::Clock for TimeClock {
    type Instant = embassy_time::Instant;

    fn now() -> embassy_time::Instant {
        embassy_time::Instant::now()
    }

    fn deadline(timeout: Duration) -> embassy_time::Instant {
        embassy_time::Instant::now() + embassy_time::Duration::from_micros(u64::try_from(timeout.as_micros()).unwrap_or(u64::MAX))
    }

    fn has_passed(deadline: &embassy_time::Instant) -> bool {
        embassy_time::Instant::now() >= *deadline
    }
}

const EVENTS: usize = 64;
type Channel = Can<TimeClock, EVENTS>;

static STATE: State<TimeClock, EVENTS> = State::new();
static RAM: ConstStaticCell<FdMessageRam> = ConstStaticCell::new(FdMessageRam::new());

const TIMEOUT: Duration = Duration::from_millis(100);

struct Log<'d>(Uart<'d, pic32c_hal::mode::Blocking>);

impl Write for Log<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for line in s.split_inclusive('\n') {
            self.0.blocking_write(line.trim_end_matches('\n').as_bytes());
            if line.ends_with('\n') {
                self.0.blocking_write(b"\r\n");
            }
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let p = pic32cm_sg00_curiosity_pro::init();
    let led = Output::new(p.PA20, Level::High);
    let mut log = Log(Uart::new_blocking(p.SERCOM4, p.PD19, p.PD20, uart::Config::default()).unwrap());
    let kernel_hz = pic32c_hal::clocks::generator_hz(GEN_CAN).unwrap_or(0);
    let _ = writeln!(log, "\npic32cm-sg00-curiosity-pro can_loopback: CPU {} Hz, CAN kernel {kernel_hz} Hz", pic32c_hal::clocks::generator_hz(0).unwrap_or(0));
    let nominal = BitTiming::solve_nominal(kernel_hz, 500_000, SamplePoint::from_per_mille(875)).unwrap();
    let data = BitTiming::solve_data(kernel_hz, 2_000_000, SamplePoint::from_per_mille(800)).unwrap();
    let _ = writeln!(log, "nominal {nominal:?}: {} bit/s; data {data:?}: {} bit/s", nominal.bitrate(kernel_hz), data.bitrate(kernel_hz));
    let config = Config::new(nominal).with_data(data).with_mode(Mode::SilentLoopback).with_generator(GEN_CAN);
    let mut can = match Can::new(p.CAN0, Irqs, p.PA11, p.PA10, RAM.take(), &STATE, config).and_then(|mut can| can.start().map(|()| can)) {
        Ok(can) => can,
        Err(e) => {
            let _ = writeln!(log, "Can::new: {e:?}");
            halt(&mut log, 1, led);
        }
    };
    let _ = writeln!(log, "CREL {:#010x}", can.core_release());
    let start = embassy_time::Instant::now();
    while !can.is_synchronized() && start.elapsed() < embassy_time::Duration::from_millis(100) {}
    let _ = writeln!(log, "synchronised {}", can.is_synchronized());

    let mut failures = every_classic_frame(&mut can, &mut log);
    failures += every_fd_frame(&mut can, &mut log);
    let _ = writeln!(log, "bus state {:?}, counters {:?}", can.bus_state(), can.error_counters());
    if can.bus_state() != Ok(BusState::ErrorActive) {
        let _ = writeln!(log, "not error active");
        failures += 1;
    }
    if failures == 0 {
        let _ = writeln!(log, "can_loopback: all passed");
    } else {
        let _ = writeln!(log, "can_loopback: {failures} failures");
    }
    halt(&mut log, failures, led);
}

/// Blinks LED0 for ever: slowly if `failures` is 0, fast otherwise.
fn halt(log: &mut Log, failures: u32, mut led: Output<'static>) -> ! {
    log.0.blocking_flush();
    let period = if failures == 0 { 500 } else { 100 };
    loop {
        led.toggle();
        embassy_time::block_for(embassy_time::Duration::from_millis(period));
    }
}

fn ids() -> [CanId; 2] {
    [CanId::new_standard(0x123).unwrap(), CanId::new_extended(0x1234_5678).unwrap()]
}

/// Sends `frame`, which must come back received unchanged and reported sent.
fn round_trip(can: &mut Channel, log: &mut Log, frame: &Frame) -> u32 {
    let tx = match ConfirmedTransmit::<Frame>::try_transmit(can, frame) {
        Ok(tx) => tx,
        Err(e) => {
            let _ = writeln!(log, "{frame:?}: transmit {e:?}");
            return 1;
        }
    };
    let (mut received, mut sent) = (false, false);
    while !(received && sent) {
        match can.receive_event_timeout(TIMEOUT).ok().flatten() {
            Some(Event::Received(r)) if r.frame() == frame => received = true,
            Some(Event::Transmitted { id, outcome: Outcome::Sent(_) }) if id == tx => sent = true,
            other => {
                let _ = writeln!(log, "{frame:?}: unexpected {other:?}");
                return 1;
            }
        }
    }
    0
}

fn every_classic_frame(can: &mut Channel, log: &mut Log) -> u32 {
    let data = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let (mut frames, mut failures) = (0, 0);
    for id in ids() {
        for dlc in 0..=15 {
            for frame in [CanFrame::new_with_dlc(id, &data[..dlc.min(8)], dlc).unwrap(), CanFrame::new_remote(id, dlc).unwrap()] {
                frames += 1;
                failures += round_trip(can, log, &Frame::Can(frame));
            }
        }
    }
    let _ = writeln!(log, "every classic frame: {frames} frames, {failures} failures");
    failures
}

fn every_fd_frame(can: &mut Channel, log: &mut Log) -> u32 {
    let mut data = [0u8; 64];
    for (i, byte) in (0u8..).zip(data.iter_mut()) {
        *byte = i.wrapping_mul(37) ^ 0x5A;
    }
    let (mut frames, mut failures) = (0, 0);
    for id in ids() {
        for len in [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 16, 20, 24, 32, 48, 64] {
            for (brs, esi) in [(false, false), (true, false), (false, true), (true, true)] {
                frames += 1;
                failures += round_trip(can, log, &Frame::Fd(CanFdFrame::new(id, &data[..len], brs, esi).unwrap()));
            }
        }
    }
    let _ = writeln!(log, "every FD frame: {frames} frames, {failures} failures");
    failures
}

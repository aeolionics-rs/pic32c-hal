// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! pic32c-hal's `can` on CAN0 in loopback, at 250 kbit/s from the 80 MHz kernel
//! clock: first internal loopback (no pins), then external loopback, which
//! drives PB16 (CAN0 TX) and so the board's ATA6561 and J704 too. The
//! `M_CAN` feeds its TX back to its receiver internally in both, ignoring
//! the RX pin, and ignores the missing acknowledgement, so nothing need be
//! on the bus; the receive path from the pin is tested on a bus.
//!
//! By default an FD channel (in `FdMessageRam`, 500 kbit/s with a 2 Mbit/s
//! data phase); built with `LOOPBACK_CLASSIC=1`, a classic one (in
//! `ClassicMessageRam`, 250 kbit/s). In each mode:
//! - every kind of classic frame (standard and extended IDs, data and
//!   remote, every DLC from 0 to 15), and on the FD channel every kind of FD
//!   frame (every length, with and without bit-rate switching and the error
//!   state indicator), must come back both as a received frame and as a
//!   transmission reported sent;
//! - eight frames (64-byte FD ones on the FD channel) are handed over at
//!   once, a ninth must find no room, and
//!   an abort of the last is reported in its place in order.
//!
//! Then acceptance filters: one standard identifier, a masked range of
//! extended ones (FD frames too, on the FD channel), too many filters
//! refused, cleared, shorter lists from the configuration (none for
//! standard identifiers) and lists longer than the RAM refused. Then the FD channel, reconfigured as classic, must refuse an FD frame;
//! the classic one must refuse a data phase (`FdNeedsFdRam`).
//!
//! Results go out over the virtual COM port; LED0 blinks slowly when all
//! passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::time::Duration;

use can_hal::{BusState, BusStatus, CanFdFrame, CanFrame, CanId, ConfirmedTransmit, Event, Filter, Filterable, Frame, Outcome, ReceiveEvents, SamplePoint, Statistics, TransmitId, TryTransmitError};
use cortex_m_rt::entry;
use panic_halt as _;
use can0::Channel;
use pic32ck_curiosity_ultra::pic32c_hal::can::{BitTiming, Config, Error, Mode, STANDARD_FILTERS};
use pic32ck_curiosity_ultra::pic32c_hal::gpio::{Level, Output};
use pic32ck_curiosity_ultra::time;
use pic32ck_curiosity_ultra::uart::Uart;
use pic32ck_curiosity_ultra::{clocks, crumb};
#[path = "common/can0.rs"]
#[allow(dead_code, reason = "shared by several examples; each uses some of it")]
mod can0;

pic32ck_curiosity_ultra::boot!();

/// Whether this build tests a classic channel (`LOOPBACK_CLASSIC=1`).
const CLASSIC: bool = match option_env!("LOOPBACK_CLASSIC") {
    Some(value) => matches!(value.as_bytes(), b"1"),
    None => false,
};
const BITRATE: u32 = if CLASSIC { 250_000 } else { 500_000 };
const DATA_BITRATE: u32 = 2_000_000;
const TIMEOUT: Duration = Duration::from_millis(100);

#[entry]
fn main() -> ! {
    crumb(0, 0x10DB_0001);
    let peripherals = pic32ck_curiosity_ultra::init();
    let led = Output::new(peripherals.PD20, Level::High);
    time::init(cortex_m::Peripherals::take().unwrap());
    let mut log = Uart::init(peripherals.SERCOM5, peripherals.PA24, peripherals.PA25);
    let _ = writeln!(log, "\npic32ck-curiosity-ultra loopback: CPU {} Hz", pic32ck_curiosity_ultra::pic32c_hal::clocks::generator_hz(0).unwrap_or(0));

    let nominal = BitTiming::solve_nominal(clocks::CAN_HZ, BITRATE, SamplePoint::from_per_mille(875)).unwrap();
    let data = BitTiming::solve_data(clocks::CAN_HZ, DATA_BITRATE, SamplePoint::from_per_mille(800)).unwrap();
    let _ = writeln!(log, "{} channel, nominal {nominal:?}: {} bit/s", if CLASSIC { "classic" } else { "FD" }, nominal.bitrate(clocks::CAN_HZ));
    if !CLASSIC {
        let _ = writeln!(log, "data {data:?}: {} bit/s", data.bitrate(clocks::CAN_HZ));
    }
    let opened = if CLASSIC {
        can0::open(peripherals.CAN0, BITRATE, Mode::SilentLoopback, peripherals.PB16, peripherals.PB17)
    } else {
        can0::open_fd(peripherals.CAN0, BITRATE, DATA_BITRATE, Mode::SilentLoopback, peripherals.PB16, peripherals.PB17)
    };
    let mut can = match opened {
        Ok(can) => can,
        Err(e) => {
            let _ = writeln!(log, "Can::new: {e:?}");
            halt(&mut log, 1, led);
        }
    };
    let _ = writeln!(log, "CREL {:#010x}", can.core_release());

    let mut failures = 0;
    for mode in [Mode::SilentLoopback, Mode::Loopback] {
        let config = if CLASSIC { Config::new(nominal) } else { Config::new(nominal).with_data(data) };
        if let Err(e) = can.reconfigure(config.with_mode(mode)).and_then(|()| can.start()) {
            let _ = writeln!(log, "{mode:?}: start {e:?}");
            failures += 1;
            continue;
        }
        let start = time::micros();
        while !can.is_synchronized() && time::micros() - start < 100_000 {}
        let _ = writeln!(log, "{mode:?}: synchronised {}", can.is_synchronized());
        failures += every_classic_frame(&mut can, &mut log);
        if !CLASSIC {
            failures += every_fd_frame(&mut can, &mut log);
        }
        failures += all_at_once(&mut can, &mut log);
        let _ = writeln!(log, "{mode:?}: bus state {:?}, counters {:?}", can.bus_state(), can.error_counters());
        if can.bus_state() != Ok(BusState::ErrorActive) {
            let _ = writeln!(log, "not error active");
            failures += 1;
        }
    }
    failures += filters(&mut can, &mut log);
    failures += if CLASSIC { refuses_data_phase(&mut can, &mut log, nominal, data) } else { classic_only(&mut can, &mut log, nominal) };
    let _ = writeln!(log, "statistics {:?}", can.statistics());
    if failures == 0 {
        let _ = writeln!(log, "loopback: all passed");
    } else {
        let _ = writeln!(log, "loopback: {failures} failures");
    }
    halt(&mut log, failures, led);
}

/// Blinks LED0 for ever: slowly if `failures` is 0, fast otherwise.
fn halt(log: &mut Uart, failures: u32, mut led: Output<'static>) -> ! {
    log.flush();
    crumb(1, failures);
    let period = if failures == 0 { 500 } else { 100 };
    loop {
        led.toggle();
        time::delay(Duration::from_millis(period));
    }
}

fn ids() -> [CanId; 2] {
    [CanId::new_standard(0x123).unwrap(), CanId::new_extended(0x1234_5678).unwrap()]
}

fn next(can: &mut Channel) -> Option<Event<Frame, u64>> {
    can.receive_event_timeout(TIMEOUT).ok().flatten()
}

/// Sends `frame`, which must come back received unchanged and reported sent.
fn round_trip(can: &mut Channel, log: &mut Uart, frame: &Frame) -> u32 {
    let tx = match ConfirmedTransmit::<Frame>::try_transmit(can, frame) {
        Ok(tx) => tx,
        Err(e) => {
            let _ = writeln!(log, "{frame:?}: transmit {e:?}");
            return 1;
        }
    };
    let (mut received, mut sent) = (false, false);
    while !(received && sent) {
        match next(can) {
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

fn every_classic_frame(can: &mut Channel, log: &mut Uart) -> u32 {
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

fn every_fd_frame(can: &mut Channel, log: &mut Uart) -> u32 {
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

/// Sends `frame`: it must be reported sent, and come back received if and
/// only if `passes`.
fn filtered(can: &mut Channel, log: &mut Uart, frame: &Frame, passes: bool) -> u32 {
    let tx = match ConfirmedTransmit::<Frame>::try_transmit(can, frame) {
        Ok(tx) => tx,
        Err(e) => {
            let _ = writeln!(log, "{frame:?}: transmit {e:?}");
            return 1;
        }
    };
    let (mut received, mut sent) = (false, false);
    while !sent || (passes && !received) {
        match next(can) {
            Some(Event::Received(r)) if r.frame() == frame && passes => received = true,
            Some(Event::Transmitted { id, outcome: Outcome::Sent(_) }) if id == tx => sent = true,
            other => {
                let _ = writeln!(log, "{frame:?} (should pass: {passes}): sent {sent}, received {received}, then {other:?}");
                return 1;
            }
        }
    }
    if !passes && let Ok(Some(other)) = can.receive_event_timeout(Duration::from_millis(5)) {
        let _ = writeln!(log, "{frame:?} should be filtered out, then {other:?}");
        return 1;
    }
    0
}

/// Acceptance filters in the channel's current mode.
fn filters(can: &mut Channel, log: &mut Uart) -> u32 {
    let standard = |id: u16| Frame::Can(CanFrame::new(CanId::new_standard(id).unwrap(), &[1, 2, 3]).unwrap());
    let extended = |id: u32| Frame::Can(CanFrame::new(CanId::new_extended(id).unwrap(), &[4, 5, 6]).unwrap());
    let fd = |id: u32| Frame::Fd(CanFdFrame::new(CanId::new_extended(id).unwrap(), &[7; 24], true, false).unwrap());
    let mut failures = 0;
    let nominal = BitTiming::solve_nominal(clocks::CAN_HZ, BITRATE, SamplePoint::from_per_mille(875)).unwrap();
    let mut config = Config::new(nominal).with_mode(Mode::SilentLoopback);
    if !CLASSIC {
        config = config.with_data(BitTiming::solve_data(clocks::CAN_HZ, DATA_BITRATE, SamplePoint::from_per_mille(800)).unwrap());
    }
    if let Err(e) = can.reconfigure(config).and_then(|()| can.start()) {
        let _ = writeln!(log, "filters: start {e:?}");
        return 1;
    }

    // One standard identifier: it passes, its neighbour and extended frames
    // don't.
    let _ = can.set_filters(&[Filter::new(CanId::new_standard(0x123).unwrap(), 0x7FF)]);
    failures += filtered(can, log, &standard(0x123), true);
    failures += filtered(can, log, &standard(0x124), false);
    failures += filtered(can, log, &extended(0x1234_5678), false);

    // A masked range of extended identifiers (the low byte free), and one
    // standard identifier beside it.
    let _ = can.set_filters(&[Filter::new(CanId::new_extended(0x1234_5600).unwrap(), 0x1FFF_FF00), Filter::new(CanId::new_standard(0x700).unwrap(), 0x7FF)]);
    failures += filtered(can, log, &extended(0x1234_5678), true);
    failures += filtered(can, log, &extended(0x1234_56FF), true);
    failures += filtered(can, log, &extended(0x1234_5778), false);
    failures += filtered(can, log, &standard(0x700), true);
    failures += filtered(can, log, &standard(0x123), false);
    if !CLASSIC {
        failures += filtered(can, log, &fd(0x1234_5601), true);
        failures += filtered(can, log, &fd(0x1234_5701), false);
    }

    // Too many standard filters: refused, and everything accepted.
    let too_many = [const { Filter::new(CanId::Standard(0x100), 0x7FF) }; STANDARD_FILTERS + 1];
    let refused = can.set_filters(&too_many);
    if refused != Err(Error::TooManyFilters) {
        let _ = writeln!(log, "{} standard filters: {refused:?}", STANDARD_FILTERS + 1);
        failures += 1;
    }
    failures += filtered(can, log, &standard(0x124), true);
    failures += filtered(can, log, &extended(0x1234_5778), true);

    // Cleared: everything passes.
    let _ = can.set_filters(&[Filter::new(CanId::new_standard(0x123).unwrap(), 0x7FF)]);
    let _ = can.clear_filters();
    failures += filtered(can, log, &standard(0x124), true);
    failures += filtered(can, log, &extended(0x0BAD_CAFE), true);

    // Shorter lists from the configuration: no standard list (standard
    // frames all pass, a standard filter is refused), two extended.
    if let Err(e) = can.reconfigure(config.with_filter_lists(0, 2)).and_then(|()| can.start()) {
        let _ = writeln!(log, "filters: lists (0, 2): {e:?}");
        return failures + 1;
    }
    let _ = can.set_filters(&[Filter::new(CanId::new_extended(0x1234_5600).unwrap(), 0x1FFF_FF00)]);
    failures += filtered(can, log, &extended(0x1234_5678), true);
    failures += filtered(can, log, &extended(0x0BAD_CAFE), false);
    failures += filtered(can, log, &standard(0x124), true);
    let three = [Filter::new(CanId::Extended(1), 0x1FFF_FFFF), Filter::new(CanId::Extended(2), 0x1FFF_FFFF), Filter::new(CanId::Extended(3), 0x1FFF_FFFF)];
    for (name, filters) in [("three extended", &three[..]), ("a standard", &[Filter::new(CanId::Standard(5), 0x7FF)][..])] {
        let refused = can.set_filters(filters);
        if refused != Err(Error::TooManyFilters) {
            let _ = writeln!(log, "{name} filter(s) with lists (0, 2): {refused:?}");
            failures += 1;
        }
    }
    // Longer than the RAM holds: refused, the channel as it was.
    let refused = can.reconfigure(config.with_filter_lists(STANDARD_FILTERS + 1, 0));
    if refused != Err(Error::FilterListsTooLong) {
        let _ = writeln!(log, "lists longer than the RAM: {refused:?}");
        failures += 1;
    }
    let _ = writeln!(log, "filters: {failures} failures");
    failures
}

/// The FD channel reconfigured without a data phase refuses FD frames and
/// still carries classic ones.
fn classic_only(can: &mut Channel, log: &mut Uart, nominal: BitTiming) -> u32 {
    let mut failures = 0;
    if let Err(e) = can.reconfigure(Config::new(nominal).with_mode(Mode::SilentLoopback)).and_then(|()| can.start()) {
        let _ = writeln!(log, "classic only: start {e:?}");
        return 1;
    }
    let fd = Frame::Fd(CanFdFrame::new(ids()[0], &[1; 12], false, false).unwrap());
    let refused = ConfirmedTransmit::<Frame>::try_transmit(can, &fd);
    if refused != Err(TryTransmitError::Backend(Error::FdDisabled)) {
        let _ = writeln!(log, "an FD frame on a classic channel: {refused:?}");
        failures += 1;
    }
    failures += round_trip(can, log, &Frame::Can(CanFrame::new(ids()[1], &[9, 8, 7]).unwrap()));
    let _ = writeln!(log, "classic only: {failures} failures");
    failures
}

/// The classic channel (classic message RAM) refuses a data phase, and
/// carries on as it was.
fn refuses_data_phase(can: &mut Channel, log: &mut Uart, nominal: BitTiming, data: BitTiming) -> u32 {
    let mut failures = 0;
    let refused = can.reconfigure(Config::new(nominal).with_data(data).with_mode(Mode::SilentLoopback));
    if refused != Err(Error::FdNeedsFdRam) {
        let _ = writeln!(log, "a data phase with classic RAM: {refused:?}");
        failures += 1;
    }
    failures += round_trip(can, log, &Frame::Can(CanFrame::new(ids()[1], &[9, 8, 7]).unwrap()));
    let _ = writeln!(log, "refuses a data phase: {failures} failures");
    failures
}

/// As many frames as there are buffers handed over at once, one more
/// finding no room, and the last aborted: the reports come in the order
/// handed over.
fn all_at_once(can: &mut Channel, log: &mut Uart) -> u32 {
    const MAX: usize = 32;
    let capacity = ConfirmedTransmit::<Frame>::capacity(can);
    let frame = |n: usize| {
        #[expect(clippy::cast_possible_truncation, reason = "fewer than 32 frames")]
        let n = n as u8;
        let id = CanId::new_standard(0x100 + u16::from(n)).unwrap();
        if CLASSIC {
            Frame::Can(CanFrame::new(id, &[n; 8]).unwrap())
        } else {
            Frame::Fd(CanFdFrame::new(id, &[n; 64], true, false).unwrap())
        }
    };
    let mut ids = [TransmitId(0); MAX];
    let mut failures = 0;
    for (n, id) in ids.iter_mut().enumerate().take(capacity) {
        match ConfirmedTransmit::<Frame>::try_transmit(can, &frame(n)) {
            Ok(tx) => *id = tx,
            Err(e) => {
                let _ = writeln!(log, "frame {n} of {capacity}: {e:?}");
                return 1;
            }
        }
    }
    if !matches!(ConfirmedTransmit::<Frame>::try_transmit(can, &frame(capacity)), Err(TryTransmitError::Full)) {
        let _ = writeln!(log, "frame {} found room", capacity + 1);
        failures += 1;
    }
    ConfirmedTransmit::<Frame>::abort(can, ids[capacity - 1]).unwrap();

    let mut last = None;
    let (mut received, mut reported) = (0, 0);
    while reported < capacity {
        match next(can) {
            Some(Event::Transmitted { id, outcome }) => {
                if id != ids[reported] {
                    let _ = writeln!(log, "report {reported} for {id:?}, expected {:?}", ids[reported]);
                    failures += 1;
                }
                if reported < capacity - 1 && !matches!(outcome, Outcome::Sent(_)) {
                    let _ = writeln!(log, "frame {reported}: {outcome:?}");
                    failures += 1;
                }
                last = Some(outcome);
                reported += 1;
            }
            Some(Event::Received(_)) => received += 1,
            other => {
                let _ = writeln!(log, "unexpected {other:?}");
                failures += 1;
                break;
            }
        }
    }
    // The last was aborted in time, or was already on its way and sent.
    let aborted = matches!(last, Some(Outcome::Aborted(_)));
    let expected = if aborted { capacity - 1 } else { capacity };
    while received < expected {
        if let Some(Event::Received(_)) = next(can) {
            received += 1;
        } else {
            let _ = writeln!(log, "waiting for frames: {received} of {expected}");
            failures += 1;
            break;
        }
    }
    if let Some(other) = next(can) {
        let _ = writeln!(log, "after all at once: {other:?}");
        failures += 1;
    }
    let _ = writeln!(log, "{capacity} at once: last {last:?}, {received} received, {failures} failures");
    failures
}

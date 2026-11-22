// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pic32c-hal`'s NMI on the PL10, on PA01 (one of its three NMI pins; the
//! board's I2C SCL), driven by PA00 through a jumper (the two are next to
//! each other on the edge connector). The PL10's pull is off while the EIC
//! has a pin, so the NMI pin needs something driving it.
//!
//! - levels: armed for high with PA00 high, or low with it low, it fires at
//!   once, and only once (the handler disarms it, so the level doesn't
//!   keep firing); armed for the other level, it doesn't fire;
//! - edges: armed for a rising, falling or either edge, it doesn't fire
//!   before PA00 moves, fires once soon after the edge that meets it, and
//!   not for one that doesn't;
//! - disarmed, an edge doesn't fire it.
//!
//! Results go out over the virtual COM port; LED0 blinks slowly when all
//! passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::{entry, exception};
use embassy_time::{Duration, block_for};
use panic_halt as _;
use pic32c_hal::exti::{self, Nmi, NmiSense};
use pic32c_hal::gpio::{Input, Level, Output, Pull};
use pic32cm_pl10_curiosity_nano::{Log, micros};

/// PORT's OUTCLR and OUTSET (its one group), and PA00's bit.
const OUTCLR: usize = 0x4100_0014;
const OUTSET: usize = 0x4100_0018;
const PA00: u32 = 1 << 0;

// SAFETY: the handler only runs `on_nmi`, which takes no lock and touches
// only the EIC's NMI registers and an atomic: safe wherever an NMI lands.
#[exception]
unsafe fn NonMaskableInt() {
    exti::on_nmi();
}

/// PA00 high or low, behind its `Output`'s back, for exact timing.
fn drive(high: bool) {
    // SAFETY: PORT's OUTSET or OUTCLR: only PA00 changes.
    unsafe { core::ptr::write_volatile((if high { OUTSET } else { OUTCLR }) as *mut u32, PA00) }
}

/// Waits up to `us` for the NMI count to pass `count`; when it did.
fn fired_within(nmi: &Nmi<'_>, count: u32, us: u64) -> Option<u64> {
    let start = micros();
    while micros() - start < us {
        if nmi.count() != count {
            return Some(micros() - start);
        }
    }
    None
}

struct Tests<'a> {
    log: &'a mut Log,
    failures: u32,
}

impl Tests<'_> {
    fn check(&mut self, name: &str, good: bool, detail: core::fmt::Arguments<'_>) {
        let _ = writeln!(self.log, "{name}: {} {detail}", if good { "ok" } else { "FAILED" });
        self.failures += u32::from(!good);
    }
}

#[entry]
fn main() -> ! {
    let mut p = pic32cm_pl10_curiosity_nano::init();
    let mut log = Log::init(p.SERCOM1, p.PB00, p.PB01);
    let led0 = Output::new(p.PB02, Level::High);
    let _ = writeln!(log, "\npic32cm-pl10-curiosity-nano nmi: PA00 into PA01");

    // The jumper, first: PA01 follows PA00.
    let jumpered = {
        let mut out = Output::new(p.PA00.reborrow(), Level::Low);
        let input = Input::new(p.PA01.reborrow(), Pull::None);
        block_for(Duration::from_micros(200));
        let low = input.is_low();
        out.set_high();
        block_for(Duration::from_micros(200));
        low && input.is_high()
    };
    let mut t = Tests { log: &mut log, failures: 0 };
    t.check("PA00 to PA01 jumper", jumpered, format_args!(""));
    if !jumpered {
        halt(t.log, 1, led0);
    }

    let _out = Output::new(p.PA00, Level::Low);
    let mut nmi = Nmi::new(p.PA01, Pull::None);
    block_for(Duration::from_micros(200));

    // Levels: the one that holds fires once at once; the other never.
    for (sense, high) in [(NmiSense::High, true), (NmiSense::Low, false)] {
        drive(high);
        block_for(Duration::from_micros(200));
        let before = nmi.count();
        nmi.arm(sense);
        let at_once = fired_within(&nmi, before, 1000);
        block_for(Duration::from_millis(50));
        let taken = nmi.count() - before;
        t.check(
            "level held",
            at_once.is_some() && taken == 1 && !nmi.is_armed(),
            format_args!("{sense:?}: fired after {at_once:?} µs, {taken} NMI(s) in 50 ms, armed after {}", nmi.is_armed()),
        );

        drive(!high);
        block_for(Duration::from_micros(200));
        let before = nmi.count();
        nmi.arm(sense);
        let fired = fired_within(&nmi, before, 50_000);
        nmi.disarm();
        t.check("level not held", fired.is_none(), format_args!("{sense:?} with PA00 the other way: {}", if fired.is_none() { "no NMI in 50 ms" } else { "FIRED" }));
    }

    // Edges: none before the move, one soon after the edge that meets it.
    for (sense, from, name) in [(NmiSense::Rise, false, "rise"), (NmiSense::Fall, true, "fall"), (NmiSense::Both, false, "either, rising"), (NmiSense::Both, true, "either, falling")] {
        drive(from);
        block_for(Duration::from_micros(200));
        let before = nmi.count();
        nmi.arm(sense);
        let early = fired_within(&nmi, before, 20_000);
        drive(!from);
        let after = fired_within(&nmi, before, 10_000);
        block_for(Duration::from_millis(20));
        let taken = nmi.count() - before;
        t.check("edge", early.is_none() && after.is_some() && taken == 1, format_args!("{name}: none in 20 ms before, then after {after:?} µs, {taken} NMI(s)"));
    }
    // The wrong edge: armed for rising, PA00 falls.
    drive(true);
    block_for(Duration::from_micros(200));
    let before = nmi.count();
    nmi.arm(NmiSense::Rise);
    drive(false);
    let fired = fired_within(&nmi, before, 20_000);
    nmi.disarm();
    t.check("wrong edge", fired.is_none(), format_args!("armed for rise, PA00 fell: {}", if fired.is_none() { "no NMI" } else { "FIRED" }));

    // Disarmed: an edge does nothing.
    let before = nmi.count();
    drive(true);
    drive(false);
    let fired = fired_within(&nmi, before, 20_000);
    t.check("disarmed", fired.is_none(), format_args!("two edges: {}", if fired.is_none() { "no NMI" } else { "FIRED" }));

    let failures = t.failures;
    if failures == 0 {
        let _ = writeln!(log, "nmi: all passed");
    } else {
        let _ = writeln!(log, "nmi: {failures} failures");
    }
    halt(&mut log, failures, led0);
}

fn halt(log: &mut Log, failures: u32, mut led: Output<'static>) -> ! {
    log.flush();
    let period = if failures == 0 { 500 } else { 100 };
    loop {
        led.toggle();
        block_for(Duration::from_millis(period));
    }
}

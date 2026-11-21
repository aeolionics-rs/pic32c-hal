// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pic32c-hal`'s NMI under test, on SW1 (PD02, the NMI pin; its board
//! pull-up holds it high while released):
//! - armed for high, it fires at once, and only once (the handler disarms
//!   it, so the level doesn't keep firing);
//! - armed for low, for a rising edge, or for a falling edge, it doesn't
//!   fire within 200 ms (don't press SW1 meanwhile);
//! - then armed for a falling edge for 10 s: press SW1, if anyone's there
//!   (not a failure).
//!
//! Results go out over the virtual COM port; LED0 blinks slowly when all
//! passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::time::Duration;

use cortex_m_rt::{entry, exception};
use pic32ck_curiosity_ultra::time;
use pic32ck_curiosity_ultra::uart::Uart;
use panic_halt as _;
use pic32c_hal::exti::{self, Nmi, NmiSense};
use pic32c_hal::gpio::{Level, Output, Pull};

pic32ck_curiosity_ultra::boot!();

// SAFETY: the handler only runs `on_nmi`, which takes no lock and touches
// only the EIC's NMI registers and an atomic: safe wherever an NMI lands.
#[exception]
unsafe fn NonMaskableInt() {
    exti::on_nmi();
}

/// Waits up to `micros` for the NMI count to pass `count`; when it did.
fn fired_within(nmi: &Nmi<'_>, count: u32, micros: u64) -> Option<u64> {
    let start = time::micros();
    while time::micros() - start < micros {
        if nmi.count() != count {
            return Some(time::micros() - start);
        }
    }
    None
}

#[entry]
fn main() -> ! {
    let p = pic32ck_curiosity_ultra::init();
    time::init(cortex_m::Peripherals::take().unwrap());
    let mut log = Uart::init(p.SERCOM5, p.PA24, p.PA25);
    let led0 = Output::new(p.PD20, Level::High);
    let _ = writeln!(log, "\npic32ck-curiosity-ultra nmi");
    let mut failures = 0;
    let mut nmi = Nmi::new(p.PD02, Pull::None);

    let before = nmi.count();
    nmi.arm(NmiSense::High);
    let at_once = fired_within(&nmi, before, 1000);
    time::delay(Duration::from_millis(100));
    let taken = nmi.count() - before;
    let good = at_once.is_some() && taken == 1 && !nmi.is_armed();
    let _ = writeln!(log, "armed for high, SW1 released: fired after {at_once:?} µs, {taken} NMI(s) in 100 ms, armed after: {}{}", nmi.is_armed(), if good { "" } else { ", WRONG" });
    failures += u32::from(!good);

    for sense in [NmiSense::Low, NmiSense::Rise, NmiSense::Fall] {
        let before = nmi.count();
        nmi.arm(sense);
        let fired = fired_within(&nmi, before, 200_000);
        nmi.disarm();
        let _ = writeln!(log, "armed for {sense:?}, SW1 released: {}", if fired.is_none() { "no NMI in 200 ms" } else { "FIRED" });
        failures += u32::from(fired.is_some());
    }

    let before = nmi.count();
    nmi.arm(NmiSense::Fall);
    let _ = writeln!(log, "press SW1 within 10 s");
    match fired_within(&nmi, before, 10_000_000) {
        Some(micros) => {
            let _ = writeln!(log, "SW1 pressed: NMI after {} ms", micros / 1000);
        }
        None => {
            let _ = writeln!(log, "SW1 not pressed (not a failure)");
        }
    }
    nmi.disarm();

    if failures == 0 {
        let _ = writeln!(log, "nmi: all passed");
    } else {
        let _ = writeln!(log, "nmi: {failures} failures");
    }
    halt(&mut log, failures, led0);
}

fn halt(log: &mut Uart, failures: u32, mut led: Output<'static>) -> ! {
    log.flush();
    let period = if failures == 0 { 500 } else { 100 };
    loop {
        led.toggle();
        time::delay(Duration::from_millis(period));
    }
}

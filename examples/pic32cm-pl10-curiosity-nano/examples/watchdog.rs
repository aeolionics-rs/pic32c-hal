// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pic32c-hal`'s `wdt` on the PL10, with the reset cause: the SG00
//! board's `watchdog`, timed by embassy-time, less the unique ID.
//!
//! First boot (any reset but the two below):
//! - start and stop: a 1 s watchdog started, then stopped, mustn't reset the
//!   chip in the next 2 s; starting a running one, or with the early
//!   warning at or past the period, is refused;
//! - fed every 100 ms for 3 s (1 s period, early warning at 0.5 s): no
//!   early warning, no reset;
//! - fed every 750 ms for 3 s: an early warning each period, no reset;
//! - then not fed: an early warning about 0.5 s in, and a reset about 1 s
//!   in.
//!
//! Second boot, after the watchdog's reset: the reset cause must say so;
//! then a system reset request (`SYSRESETREQ`), marked in SRAM that a reset
//! keeps. Third boot: the reset cause must say system, and not watchdog
//! (which shows which of RCAUSE's bits the request sets). LED0 (PB02)
//! blinks slowly when all passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use embassy_time::{Duration, Instant, block_for};
use panic_halt as _;
use pic32c_hal::bind_interrupts;
use pic32c_hal::gpio::{Level, Output};
use pic32c_hal::wdt::{self, Error, Period, ResetCause, Watchdog};
use pic32cm_pl10_curiosity_nano::Log;

bind_interrupts!(struct Irqs {
    WDT => wdt::InterruptHandler;
});

const PERIOD: Period = Period::Cycles1024;
const EARLY: Period = Period::Cycles512;
/// At [`STAGE`] across the system reset the second boot asks for.
const SYSTEM_RESET_ASKED: u32 = 0x5953_5452;

/// A word of SRAM a system reset keeps: 1 KiB below the top, clear of
/// the boot ROM (whose stack, run at every reset, is at the bottom of SRAM,
/// where `.uninit` would be), of `.bss` and of the stack at start-up.
const STAGE: *mut u32 = 0x2000_1C00 as *mut u32;

fn stage() -> u32 {
    // SAFETY: a word of SRAM nothing else uses this early; any bit pattern
    // is a u32.
    unsafe { STAGE.read_volatile() }
}

fn set_stage(value: u32) {
    // SAFETY: as above; written just before a reset or at start-up.
    unsafe { STAGE.write_volatile(value) }
}

#[entry]
fn main() -> ! {
    let cause = ResetCause::read();
    let asked = stage() == SYSTEM_RESET_ASKED;
    set_stage(0);
    let p = pic32cm_pl10_curiosity_nano::init();
    let led = Output::new(p.PB02, Level::High);
    let mut log = Log::init(p.SERCOM1, p.PB00, p.PB01);
    let _ = writeln!(log, "\npic32cm-pl10-curiosity-nano watchdog: CPU {} Hz, reset cause {cause:?}", pic32c_hal::clocks::generator_hz(0).unwrap_or(0));
    let mut watchdog = Watchdog::new(p.WDT, Irqs);
    let _ = writeln!(log, "running at boot: {}", watchdog.is_running());

    if asked {
        let good = cause.system() && !cause.watchdog();
        let _ = writeln!(log, "after the system reset request: system {}, watchdog {}, lockup bit {}", cause.system(), cause.watchdog(), cause.lockup());
        report(&mut log, u32::from(!good), led);
    }
    if cause.watchdog() {
        let _ = writeln!(log, "reset by the watchdog; now a system reset request");
        log.flush();
        set_stage(SYSTEM_RESET_ASKED);
        cortex_m::peripheral::SCB::sys_reset();
    }
    let mut failures = start_and_stop(&mut watchdog, &mut log);
    failures += fed(&mut watchdog, &mut log, 100, 0);
    failures += fed(&mut watchdog, &mut log, 750, 4);
    if failures != 0 {
        report(&mut log, failures, led);
    }

    let _ = watchdog.start(PERIOD, Some(EARLY));
    let warnings = watchdog.early_warnings();
    let start = Instant::now();
    let _ = writeln!(log, "starving");
    log.flush();
    let mut reported = false;
    while start.elapsed() < Duration::from_secs(3) {
        if !reported && watchdog.early_warnings() != warnings {
            let _ = writeln!(log, "early warning after {} ms", start.elapsed().as_millis());
            reported = true;
        }
    }
    let _ = writeln!(log, "no reset after 3 s");
    report(&mut log, 1, led);
}

fn start_and_stop(watchdog: &mut Watchdog<'_>, log: &mut Log) -> u32 {
    let too_late = watchdog.start(PERIOD, Some(PERIOD));
    let started = watchdog.start(PERIOD, Some(EARLY));
    let again = watchdog.start(PERIOD, None);
    let stopped = watchdog.stop();
    block_for(Duration::from_secs(2));
    let _ = writeln!(log, "start with the warning at the period: {too_late:?}; start: {started:?}; again: {again:?}; stop: {stopped:?}; still here 2 s later");
    u32::from(too_late != Err(Error::EarlyWarningTooLate) || started.is_err() || again != Err(Error::Running) || stopped.is_err() || watchdog.is_running())
}

/// Fed every `every_ms` for 3 s: `warnings` early warnings expected (±1),
/// and no reset.
fn fed(watchdog: &mut Watchdog<'_>, log: &mut Log, every_ms: u64, warnings: u32) -> u32 {
    let before = watchdog.early_warnings();
    let started = watchdog.start(PERIOD, Some(EARLY));
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(3) {
        block_for(Duration::from_millis(every_ms));
        watchdog.feed();
    }
    let stopped = watchdog.stop();
    let count = watchdog.early_warnings() - before;
    let _ = writeln!(log, "fed every {every_ms} ms for 3 s: {started:?}, {count} early warnings, stopped {stopped:?}");
    u32::from(started.is_err() || stopped.is_err() || count.abs_diff(warnings) > 1)
}

fn report(log: &mut Log, failures: u32, mut led: Output<'static>) -> ! {
    if failures == 0 {
        let _ = writeln!(log, "watchdog: all passed");
    } else {
        let _ = writeln!(log, "watchdog: {failures} failures");
    }
    log.flush();
    let period = if failures == 0 { 500 } else { 100 };
    loop {
        led.toggle();
        block_for(Duration::from_millis(period));
    }
}

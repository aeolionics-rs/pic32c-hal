// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pic32c-hal`'s `wdt` on the SG00, with the reset cause and the chip's
//! unique ID: the PIC32CK board's `watchdog`, timed by embassy-time.
//!
//! First boot (any reset but the watchdog's):
//! - the unique ID, which must be a version 4 UUID;
//! - start and stop: a 1 s watchdog started, then stopped, mustn't reset the
//!   chip in the next 2 s; starting a running one, or with the early
//!   warning at or past the period, is refused;
//! - fed every 100 ms for 3 s (1 s period, early warning at 0.5 s): no
//!   early warning, no reset;
//! - fed every 750 ms for 3 s: an early warning each period, no reset;
//! - then not fed: an early warning about 0.5 s in, and a reset about 1 s
//!   in.
//!
//! Second boot, after the watchdog's reset: the reset cause must say so.
//! LED0 (PA20) blinks slowly when all passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use embassy_time::{Duration, Instant, block_for};
use panic_halt as _;
use pic32c_hal::bind_interrupts;
use pic32c_hal::gpio::{Level, Output};
use pic32c_hal::uart::{self, Uart};
use pic32c_hal::wdt::{self, Error, Period, ResetCause, Watchdog};

pic32cm_sg00_curiosity_pro::boot!();

bind_interrupts!(struct Irqs {
    WDT => wdt::InterruptHandler;
});

const PERIOD: Period = Period::Cycles1024;
const EARLY: Period = Period::Cycles512;

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
    let cause = ResetCause::read();
    let p = pic32cm_sg00_curiosity_pro::init();
    let led = Output::new(p.PA20, Level::High);
    let mut log = Log(Uart::new_blocking(p.SERCOM4, p.PD19, p.PD20, uart::Config::default()).unwrap());
    let _ = writeln!(log, "\npic32cm-sg00-curiosity-pro watchdog: CPU {} Hz, reset cause {cause:?}", pic32c_hal::clocks::generator_hz(0).unwrap_or(0));
    let mut watchdog = Watchdog::new(p.WDT, Irqs);
    let _ = writeln!(log, "running at boot: {}", watchdog.is_running());

    let mut failures = unique_id(&mut log);
    if cause.watchdog() {
        let _ = writeln!(log, "reset by the watchdog");
        report(&mut log, failures, led);
    }
    failures += start_and_stop(&mut watchdog, &mut log);
    failures += fed(&mut watchdog, &mut log, 100, 0);
    failures += fed(&mut watchdog, &mut log, 750, 4);
    if failures != 0 {
        report(&mut log, failures, led);
    }

    let _ = watchdog.start(PERIOD, Some(EARLY));
    let warnings = watchdog.early_warnings();
    let start = Instant::now();
    let _ = writeln!(log, "starving");
    log.0.blocking_flush();
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

/// The unique ID, and whether it's a version 4 (random) UUID.
fn unique_id(log: &mut Log) -> u32 {
    let id = pic32c_hal::uid::unique_id();
    let _ = write!(log, "unique ID ");
    for (i, byte) in id.iter().enumerate() {
        if matches!(i, 4 | 6 | 8 | 10) {
            let _ = write!(log, "-");
        }
        let _ = write!(log, "{byte:02x}");
    }
    let version_4 = id[6] >> 4 == 4 && id[8] >> 6 == 0b10;
    let _ = writeln!(log, "{}", if version_4 { ", a version 4 UUID" } else { ", NOT a version 4 UUID" });
    u32::from(!version_4)
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
    log.0.blocking_flush();
    let period = if failures == 0 { 500 } else { 100 };
    loop {
        led.toggle();
        block_for(Duration::from_millis(period));
    }
}

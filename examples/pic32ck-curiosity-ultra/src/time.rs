// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Time from the DWT cycle counter, extended to 64 bits.
//!
//! The counter wraps every 35.8 s at 120 MHz; each read carries the wraps
//! over, so it must be read at least that often. [`init`] starts `SysTick` to
//! read it every 10 ms, which keeps that so whatever the program does.

use core::cell::Cell;
use core::sync::atomic::{AtomicU32, Ordering};
use core::time::Duration;

use cortex_m::peripheral::SYST;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use critical_section::Mutex;

use crate::clocks::CPU_HZ;

/// `SysTick` interrupts taken.
static SYSTICKS: AtomicU32 = AtomicU32::new(0);

/// The last count read and the wraps before it.
static EXTENDED: Mutex<Cell<(u32, u32)>> = Mutex::new(Cell::new((0, 0)));

/// Starts the cycle counter and the `SysTick` that keeps its wraps; call once,
/// after [`crate::init`].
pub fn init(mut core: cortex_m::Peripherals) {
    core.DCB.enable_trace();
    core.DWT.enable_cycle_counter();
    let syst: &mut SYST = &mut core.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(CPU_HZ / 100 - 1);
    syst.clear_current();
    syst.enable_interrupt();
    syst.enable_counter();
}

#[exception]
fn SysTick() {
    let _ = cycles();
    SYSTICKS.fetch_add(1, Ordering::Relaxed);
}

/// `SysTick` interrupts taken (one every 10 ms), to show whether the CPU ran
/// through something, such as a flash erase.
#[must_use]
pub fn systicks() -> u32 {
    SYSTICKS.load(Ordering::Relaxed)
}

/// CPU cycles since the counter started.
#[must_use]
pub fn cycles() -> u64 {
    critical_section::with(|cs| {
        let cell = EXTENDED.borrow(cs);
        let (last, mut wraps) = cell.get();
        let now = cortex_m::peripheral::DWT::cycle_count();
        if now < last {
            wraps += 1;
        }
        cell.set((now, wraps));
        (u64::from(wraps) << 32) | u64::from(now)
    })
}

/// Microseconds since the counter started.
#[must_use]
pub fn micros() -> u64 {
    cycles() / u64::from(CPU_HZ / 1_000_000)
}

/// The cycle count `timeout` from now.
#[must_use]
pub fn deadline(timeout: Duration) -> u64 {
    let cycles_per_us = u128::from(CPU_HZ / 1_000_000);
    cycles().saturating_add(u64::try_from(timeout.as_micros() * cycles_per_us).unwrap_or(u64::MAX))
}

/// Waits `duration`.
pub fn delay(duration: Duration) {
    let deadline = deadline(duration);
    while cycles() < deadline {}
}

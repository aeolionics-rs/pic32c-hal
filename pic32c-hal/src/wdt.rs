// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The PIC32CK's watchdog (WDT, DS60001795 chapter 23) in normal mode, with
//! its early warning, and the reset controller's record of why the chip last
//! reset (RSTC.RCAUSE, chapter 29).
//!
//! The watchdog counts the 1.024 kHz output of the ultra-low-power 32 kHz
//! oscillator, which isn't trimmed to the crystal: allow for its tolerance
//! (the electrical characteristics' OSCULP32K figures) when picking a
//! [`Period`]. Its reset values come from the USER CFG page (FUCFG0): if a
//! fuse turns it on, or on always, [`Watchdog::start`] reports that instead
//! of changing it.
//!
//! [`Watchdog::new`] takes the `WDT` peripheral and a
//! [`bind_interrupts!`](crate::bind_interrupts) type binding its vector
//! (`WDT`, the early warning) to [`InterruptHandler`].

use core::future::poll_fn;
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicU32, Ordering};
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;

use crate::interrupt::typelevel::{self, Binding, Handler};
use crate::interrupt::{Interrupt, InterruptExt};
use crate::{Peri, peripherals};

#[cfg(pic32ck)]
const WDT: usize = 0x4401_6000;
/// The SG00's (DS60001921 chapter 25: the same IP; RSTC chapter 31).
#[cfg(pic32cm_sg)]
const WDT: usize = 0x4400_E000;
const CTRLA: usize = WDT;
const CONFIG: usize = WDT + 0x01;
const EWCTRL: usize = WDT + 0x02;
const INTENCLR: usize = WDT + 0x04;
const INTENSET: usize = WDT + 0x05;
const INTFLAG: usize = WDT + 0x06;
const SYNCBUSY: usize = WDT + 0x08;
const CLEAR: usize = WDT + 0x0C;

const ENABLE: u8 = 1 << 1;
const ALWAYSON: u8 = 1 << 7;
const EW: u8 = 1 << 0;
const SYNC_ENABLE: u32 = 1 << 1;
const SYNC_CLEAR: u32 = 1 << 5;
/// The one value CLEAR takes that doesn't reset the chip.
const CLEAR_KEY: u8 = 0xA5;

#[cfg(pic32ck)]
const RCAUSE: usize = 0x4400_A000;
#[cfg(pic32cm_sg)]
const RCAUSE: usize = 0x4500_4000;

/// A watchdog period: a number of cycles of its 1.024 kHz clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Period {
    Cycles8,
    Cycles16,
    Cycles32,
    Cycles64,
    Cycles128,
    Cycles256,
    Cycles512,
    Cycles1024,
    Cycles2048,
    Cycles4096,
    Cycles8192,
    Cycles16384,
}

impl Period {
    /// The cycles.
    #[must_use]
    pub const fn cycles(self) -> u32 {
        8 << self as u32
    }

    /// The nominal length in ms (1,024 cycles a second).
    #[must_use]
    pub const fn millis(self) -> u32 {
        self.cycles() * 1000 / 1024
    }
}

/// Why [`Watchdog::start`] or [`Watchdog::stop`] refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Already running, as the fuses or an earlier start left it; its
    /// settings can't change while it runs.
    Running,
    /// Always on (a fuse, or set since power-on): it can't be stopped.
    AlwaysOn,
    /// The early warning would come at or after the reset.
    EarlyWarningTooLate,
}

/// Early warnings taken.
static WARNINGS: AtomicU32 = AtomicU32::new(0);
static WARNING_WAKER: AtomicWaker = AtomicWaker::new();

/// The early warning's handler: bind `WDT` to it with
/// [`bind_interrupts!`](crate::bind_interrupts).
pub struct InterruptHandler {
    _private: (),
}

impl Handler<typelevel::WDT> for InterruptHandler {
    unsafe fn on_interrupt() {
        if read8(INTFLAG) & EW != 0 {
            write8(INTFLAG, EW);
            WARNINGS.fetch_add(1, Ordering::Relaxed);
            WARNING_WAKER.wake();
        }
    }
}

/// The watchdog.
pub struct Watchdog<'d> {
    _wdt: PhantomData<&'d mut peripherals::WDT>,
}

fn read8(address: usize) -> u8 {
    // SAFETY: a WDT register, read whole.
    unsafe { read_volatile(address as *const u8) }
}

fn write8(address: usize, value: u8) {
    // SAFETY: a WDT register; only this driver writes them.
    unsafe { write_volatile(address as *mut u8, value) }
}

fn syncbusy() -> u32 {
    // SAFETY: WDT's SYNCBUSY.
    unsafe { read_volatile(SYNCBUSY as *const u32) }
}

impl<'d> Watchdog<'d> {
    /// The watchdog, its early-warning interrupt unmasked (it fires only
    /// once [`start`](Self::start) asks for a warning).
    #[must_use]
    pub fn new(_wdt: Peri<'d, peripherals::WDT>, _irq: impl Binding<typelevel::WDT, InterruptHandler> + 'd) -> Self {
        Interrupt::WDT.unpend();
        // SAFETY: the binding makes the vector call the handler.
        unsafe { Interrupt::WDT.enable() };
        Self { _wdt: PhantomData }
    }

    /// Whether it's running.
    #[must_use]
    pub fn is_running(&self) -> bool {
        read8(CTRLA) & (ENABLE | ALWAYSON) != 0
    }

    /// Starts it: a reset unless fed within `period`, and an early warning
    /// `early_warning` into each period if given.
    ///
    /// # Errors
    ///
    /// [`Error::Running`] if it's already running, and
    /// [`Error::EarlyWarningTooLate`] if `early_warning` isn't shorter than
    /// `period`.
    pub fn start(&mut self, period: Period, early_warning: Option<Period>) -> Result<(), Error> {
        if self.is_running() {
            return Err(Error::Running);
        }
        if early_warning.is_some_and(|warning| warning >= period) {
            return Err(Error::EarlyWarningTooLate);
        }
        write8(CONFIG, period as u8);
        write8(INTFLAG, EW);
        match early_warning {
            Some(warning) => {
                write8(EWCTRL, warning as u8);
                write8(INTENSET, EW);
            }
            None => write8(INTENCLR, EW),
        }
        write8(CTRLA, ENABLE);
        while syncbusy() & SYNC_ENABLE != 0 {}
        Ok(())
    }

    /// Restarts the period. A clear takes a few of the watchdog's cycles
    /// to reach its clock; feeding again meanwhile does nothing, since the
    /// one under way restarts the period anyway.
    pub fn feed(&mut self) {
        if syncbusy() & SYNC_CLEAR == 0 {
            write8(CLEAR, CLEAR_KEY);
        }
    }

    /// Stops it.
    ///
    /// # Errors
    ///
    /// [`Error::AlwaysOn`] if it can't be stopped.
    pub fn stop(&mut self) -> Result<(), Error> {
        if read8(CTRLA) & ALWAYSON != 0 {
            return Err(Error::AlwaysOn);
        }
        write8(CTRLA, 0);
        while syncbusy() & SYNC_ENABLE != 0 {}
        write8(INTENCLR, EW);
        Ok(())
    }

    /// Early warnings taken since power-on.
    #[must_use]
    pub fn early_warnings(&self) -> u32 {
        WARNINGS.load(Ordering::Relaxed)
    }

    /// Waits for the next early warning.
    pub async fn wait_early_warning(&mut self) {
        let before = WARNINGS.load(Ordering::Relaxed);
        poll_fn(|cx| {
            WARNING_WAKER.register(cx.waker());
            if WARNINGS.load(Ordering::Relaxed) == before { Poll::Pending } else { Poll::Ready(()) }
        })
        .await;
    }
}

/// Why the chip last reset: RSTC.RCAUSE, one bit per cause.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ResetCause(pub u16);

impl ResetCause {
    /// Read from the reset controller; it holds until the next reset.
    #[must_use]
    pub fn read() -> Self {
        // SAFETY: RSTC's RCAUSE, read only.
        Self(unsafe { read_volatile(RCAUSE as *const u16) })
    }

    /// Power-on (POR or PORCORE).
    #[must_use]
    pub const fn power_on(self) -> bool {
        self.0 & 0b11 != 0
    }

    /// A brown-out on VDDREG, VDDA or VDDIO.
    #[must_use]
    pub const fn brown_out(self) -> bool {
        self.0 & 0b1_1100 != 0
    }

    /// The external reset pin.
    #[must_use]
    pub const fn external(self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// The watchdog.
    #[must_use]
    pub const fn watchdog(self) -> bool {
        self.0 & (1 << 6) != 0
    }

    /// A system reset request (`SYSRESETREQ`, from software or a debugger).
    #[must_use]
    pub const fn system(self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Waking from backup sleep.
    #[must_use]
    pub const fn backup(self) -> bool {
        self.0 & (1 << 8) != 0
    }

    /// A CPU lockup.
    #[must_use]
    pub const fn lockup(self) -> bool {
        self.0 & (1 << 9) != 0
    }
}

impl core::fmt::Debug for ResetCause {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const NAMES: [&str; 10] = ["POR", "PORCORE", "BOR VDDREG", "BOR VDDA", "BOR VDDIO", "external", "watchdog", "system", "backup", "lockup"];
        write!(f, "ResetCause({:#06x}", self.0)?;
        for (bit, name) in NAMES.iter().enumerate() {
            if self.0 & (1 << bit) != 0 {
                write!(f, " {name}")?;
            }
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn periods() {
        assert_eq!(Period::Cycles8.cycles(), 8);
        assert_eq!(Period::Cycles1024.cycles(), 1024);
        assert_eq!(Period::Cycles16384.cycles(), 16384);
        assert_eq!(Period::Cycles1024.millis(), 1000);
        assert_eq!(Period::Cycles16384.millis(), 16000);
        assert_eq!(Period::Cycles16384 as u8, 0xB);
    }

    #[test]
    fn causes() {
        assert!(ResetCause(0x40).watchdog());
        assert!(!ResetCause(0x40).power_on());
        assert!(ResetCause(0x02).power_on());
        assert!(ResetCause(0x80).system());
        assert_eq!(format!("{:?}", ResetCause(0x41)), "ResetCause(0x0041 POR watchdog)");
    }
}

// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The real-time counter (RTC, DS60001795 chapter 25) as a clock and
//! calendar, from the 32.768 kHz crystal (XOSC32K on PB02/PB03; OSC32KCTRL,
//! chapter 22) or the internal ultra-low-power oscillator, and an alarm.
//!
//! The RTC and the crystal are in the backup domain: they keep running
//! through every reset but power-on. So [`Rtc::new`] keeps the time if it
//! finds the RTC already running as this driver sets it up
//! ([`Rtc::was_running`] says so), and a reboot doesn't lose the time set
//! from SNTP, say. In clock mode the counter needs 1 Hz: the oscillator's
//! 1.024 kHz output, divided by 1,024.
//!
//! The calendar's year field holds 0 to 63, years since 2000: [`DateTime`]s
//! run from 2000-01-01 to 2063-12-31, 24-hour.

use core::future::poll_fn;
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;

use crate::interrupt::typelevel::{Binding, Handler, Interrupt as _};
use crate::{Peri, interrupt, peripherals};

const RTC: usize = 0x4401_8000;
const CTRLA: usize = RTC;
const INTENCLR: usize = RTC + 0x08;
const INTENSET: usize = RTC + 0x0A;
const INTFLAG: usize = RTC + 0x0C;
const SYNCBUSY: usize = RTC + 0x10;
const CLOCK: usize = RTC + 0x18;
const ALARM0: usize = RTC + 0x20;
const MASK0: usize = RTC + 0x24;

const SWRST: u16 = 1 << 0;
const ENABLE: u16 = 1 << 1;
/// MODE 2: clock/calendar.
const MODE_CLOCK: u16 = 0x2 << 2;
const MODE_MASK: u16 = 0x3 << 2;
/// PRESCALER 11: `CLK_RTC_OSC` / 1024.
const PRESCALER_1024: u16 = 11 << 8;
const PRESCALER_MASK: u16 = 0xF << 8;
const CLOCKSYNC: u16 = 1 << 15;
const ALARM0_FLAG: u16 = 1 << 8;
/// MASK.SEL 6: the alarm matches year to second.
const MATCH_ALL: u8 = 6;
const SYNC_ALL: u32 = 0xFFFF_FFFF;

const OSC32KCTRL: usize = 0x4400_E000;
const OSC_STATUS: usize = OSC32KCTRL + 0x0C;
const CLKSELCTRL: usize = OSC32KCTRL + 0x10;
const XOSC32K: usize = OSC32KCTRL + 0x1C;
const XOSC32KRDY: u32 = 1 << 0;
const XOSC_ENABLE: u32 = 1 << 1;
const XOSC_XTALEN: u32 = 1 << 2;
/// Set at reset: the crystal would run only while something asks for it,
/// and STATUS.XOSC32KRDY wouldn't come before then.
const XOSC_ONDEMAND: u32 = 1 << 7;
const XOSC_STARTUP_SHIFT: u32 = 8;
const XOSC_STARTUP_MASK: u32 = 0xF << XOSC_STARTUP_SHIFT;
const XOSC_CGM_SHIFT: u32 = 18;
const XOSC_CGM_MASK: u32 = 0xF << XOSC_CGM_SHIFT;
/// RTCSEL: the 1.024 kHz outputs.
const RTCSEL_MASK: u32 = 0b11;
const RTCSEL_ULP1K: u32 = 1;
const RTCSEL_XOSC1K: u32 = 3;

/// MCLK's APB mask for the RTC: CLKMSK1 bit 12 (index 44).
const MCLK_CLKMSK1: usize = 0x4401_2040;
const MCLK_RTC: u32 = 1 << 12;

/// Polls of XOSC32KRDY: a few seconds, longer than any crystal's start-up.
const XOSC_POLLS: u32 = 200_000_000;

static WAKER: AtomicWaker = AtomicWaker::new();

/// Where the RTC's 1.024 kHz clock comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Source {
    /// The 32.768 kHz crystal on XIN32/XOUT32 (PB02/PB03): tens of ppm.
    Crystal,
    /// The internal ultra-low-power oscillator: percent, but no crystal.
    UltraLowPower,
}

/// How to set the RTC up.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub source: Source,
    /// The crystal's start-up time, XOSC32K.STARTUP (0 to 10: 1 to 262,144
    /// cycles of the ULP oscillator; 5, 8,192 cycles, 250 ms, by default).
    pub crystal_startup: u8,
    /// The crystal amplifier's gain, XOSC32K.CGM (0 to 15). The data sheet
    /// gives the crystal's figures at 15, the default; it resets to 8.
    pub crystal_gain: u8,
}

impl Default for Config {
    /// The crystal, 250 ms to start, at the highest gain.
    fn default() -> Self {
        Self { source: Source::Crystal, crystal_startup: 5, crystal_gain: 15 }
    }
}

/// Why the RTC can't be set up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// The crystal didn't start: none fitted, or a fault.
    NoCrystal,
}

/// A date and time the calendar can hold: 2000-01-01 00:00:00 to
/// 2063-12-31 23:59:59, 24-hour, no time zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

/// A date or time the calendar can't hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutOfRange;

const fn is_leap(year: u16) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

const fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        2 if is_leap(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Days from 1970-01-01 to `year`-`month`-`day` (Howard Hinnant's
/// `days_from_civil`).
const fn days_from_civil(year: u16, month: u8, day: u8) -> i64 {
    let y = if month <= 2 { year as i64 - 1 } else { year as i64 };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let m = month as i64;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

impl DateTime {
    /// The first the calendar holds.
    pub const MIN: Self = Self { year: 2000, month: 1, day: 1, hour: 0, minute: 0, second: 0 };
    /// The last.
    pub const MAX: Self = Self { year: 2063, month: 12, day: 31, hour: 23, minute: 59, second: 59 };

    /// A date and time.
    ///
    /// # Errors
    ///
    /// [`OutOfRange`] if it isn't a real one, or is outside 2000 to 2063.
    pub const fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Result<Self, OutOfRange> {
        if year < 2000 || year > 2063 || month < 1 || month > 12 || day < 1 || day > days_in_month(year, month) || hour > 23 || minute > 59 || second > 59 {
            return Err(OutOfRange);
        }
        Ok(Self { year, month, day, hour, minute, second })
    }

    /// The date and time `seconds` after 1970-01-01 00:00:00 UTC.
    ///
    /// # Errors
    ///
    /// [`OutOfRange`] outside 2000 to 2063.
    pub fn from_unix(seconds: u64) -> Result<Self, OutOfRange> {
        let days = i64::try_from(seconds / 86_400).map_err(|_| OutOfRange)?;
        let rest = seconds % 86_400;
        // Howard Hinnant's `civil_from_days`.
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = doy - (153 * mp + 2) / 5 + 1;
        let month = if mp < 10 { mp + 3 } else { mp - 9 };
        let year = yoe + era * 400 + i64::from(month <= 2);
        let (Ok(year), Ok(month), Ok(day)) = (u16::try_from(year), u8::try_from(month), u8::try_from(day)) else {
            return Err(OutOfRange);
        };
        let (hour, minute, second) = ((rest / 3600) as u8, (rest / 60 % 60) as u8, (rest % 60) as u8);
        Self::new(year, month, day, hour, minute, second)
    }

    /// Seconds since 1970-01-01 00:00:00 UTC.
    #[must_use]
    pub fn to_unix(self) -> u64 {
        let days = days_from_civil(self.year, self.month, self.day);
        u64::try_from(days).unwrap_or(0) * 86_400 + u64::from(self.hour) * 3600 + u64::from(self.minute) * 60 + u64::from(self.second)
    }

    #[must_use]
    pub const fn year(self) -> u16 {
        self.year
    }
    #[must_use]
    pub const fn month(self) -> u8 {
        self.month
    }
    #[must_use]
    pub const fn day(self) -> u8 {
        self.day
    }
    #[must_use]
    pub const fn hour(self) -> u8 {
        self.hour
    }
    #[must_use]
    pub const fn minute(self) -> u8 {
        self.minute
    }
    #[must_use]
    pub const fn second(self) -> u8 {
        self.second
    }

    /// The CLOCK and ALARM registers' layout.
    const fn to_register(self) -> u32 {
        (self.second as u32) | ((self.minute as u32) << 6) | ((self.hour as u32) << 12) | ((self.day as u32) << 17) | ((self.month as u32) << 22) | (((self.year - 2000) as u32) << 26)
    }

    /// From that layout, if it holds a valid date and time.
    fn from_register(value: u32) -> Option<Self> {
        Self::new(2000 + (value >> 26) as u16, ((value >> 22) & 0xF) as u8, ((value >> 17) & 0x1F) as u8, ((value >> 12) & 0x1F) as u8, ((value >> 6) & 0x3F) as u8, (value & 0x3F) as u8).ok()
    }
}

impl core::fmt::Display for DateTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}

/// The RTC's interrupt handler: bind it with
/// [`bind_interrupts!`](crate::bind_interrupts) to `RTC`.
pub struct InterruptHandler {
    _private: (),
}

impl Handler<interrupt::typelevel::RTC> for InterruptHandler {
    unsafe fn on_interrupt() {
        let fired = read16(INTFLAG) & read16(INTENSET);
        if fired & ALARM0_FLAG != 0 {
            write16(INTENCLR, ALARM0_FLAG);
            WAKER.wake();
        }
    }
}

/// The RTC as a clock and calendar.
pub struct Rtc<'d> {
    was_running: bool,
    _rtc: PhantomData<&'d ()>,
}

impl<'d> Rtc<'d> {
    /// The RTC, from `config`'s source: kept as it is, time and all, if it
    /// is already running that way (after a reset that wasn't power-on);
    /// otherwise set up afresh, the crystal started and the time
    /// [`DateTime::MIN`].
    ///
    /// # Errors
    ///
    /// [`ConfigError::NoCrystal`] if the crystal doesn't start.
    #[allow(clippy::needless_pass_by_value, reason = "taking the RTC is what makes it this one's")]
    pub fn new(rtc: Peri<'d, peripherals::RTC>, _irqs: impl Binding<interrupt::typelevel::RTC, InterruptHandler> + 'd, config: Config) -> Result<Self, ConfigError> {
        let _ = rtc;
        critical_section::with(|_| write32(MCLK_CLKMSK1, read32(MCLK_CLKMSK1) | MCLK_RTC));
        let rtcsel = match config.source {
            Source::Crystal => RTCSEL_XOSC1K,
            Source::UltraLowPower => RTCSEL_ULP1K,
        };
        let ctrla = MODE_CLOCK | PRESCALER_1024 | CLOCKSYNC;
        let running = read16(CTRLA) & (ENABLE | MODE_MASK | PRESCALER_MASK | CLOCKSYNC) == ctrla | ENABLE
            && read32(CLKSELCTRL) & RTCSEL_MASK == rtcsel
            && (config.source == Source::UltraLowPower || read32(OSC_STATUS) & XOSC32KRDY != 0);
        if !running {
            if config.source == Source::Crystal {
                start_crystal(config.crystal_startup, config.crystal_gain)?;
            }
            // Disabled, then the clock chosen, then reset (all but DEBUG).
            write16(CTRLA, read16(CTRLA) & !ENABLE);
            sync();
            write32(CLKSELCTRL, (read32(CLKSELCTRL) & !RTCSEL_MASK) | rtcsel);
            write16(CTRLA, SWRST);
            while read16(CTRLA) & SWRST != 0 {}
            sync();
            write16(CTRLA, ctrla);
            sync();
            write32(CLOCK, DateTime::MIN.to_register());
            sync();
            write16(CTRLA, ctrla | ENABLE);
            sync();
        }
        interrupt::typelevel::RTC::unpend();
        // SAFETY: the binding makes the vector call the handler.
        unsafe { interrupt::typelevel::RTC::enable() };
        Ok(Self { was_running: running, _rtc: PhantomData })
    }

    /// Whether the RTC was already running, its time kept, rather than set
    /// up afresh.
    #[must_use]
    pub const fn was_running(&self) -> bool {
        self.was_running
    }

    /// The date and time now.
    #[must_use]
    pub fn now(&self) -> DateTime {
        // With CLOCKSYNC on, CLOCK reads a synchronized value; read it twice
        // so a carry in mid-read can't give a torn one.
        loop {
            sync();
            let a = read32(CLOCK);
            sync();
            let b = read32(CLOCK);
            if a == b {
                return DateTime::from_register(a).unwrap_or(DateTime::MIN);
            }
        }
    }

    /// Sets the date and time.
    pub fn set(&mut self, at: DateTime) {
        sync();
        write32(CLOCK, at.to_register());
        sync();
    }

    /// Waits until `at`: at once if it has passed.
    pub async fn wait_until(&mut self, at: DateTime) {
        if self.now() >= at {
            return;
        }
        // The flag comes on the counter's tick after the match, a second
        // late at 1 Hz, so the match is the second before `at` (no earlier
        // than now, since `at` is ahead).
        let before = DateTime::from_unix(at.to_unix() - 1).unwrap_or(at);
        write16(INTENCLR, ALARM0_FLAG);
        sync();
        write32(ALARM0, before.to_register());
        write8(MASK0, MATCH_ALL);
        sync();
        write16(INTFLAG, ALARM0_FLAG);
        // The flag rises a little before a read of CLOCK shows `at` (the
        // read is synchronised), so after it the wait yields until it does.
        let mut matched = false;
        poll_fn(|cx| {
            WAKER.register(cx.waker());
            if read16(INTFLAG) & ALARM0_FLAG != 0 {
                write16(INTFLAG, ALARM0_FLAG);
                matched = true;
            }
            if self.now() >= at {
                Poll::Ready(())
            } else if matched {
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                write16(INTENSET, ALARM0_FLAG);
                Poll::Pending
            }
        })
        .await;
    }
}

/// Starts the crystal if it isn't running, and waits for it. It runs
/// always, not on demand.
fn start_crystal(startup: u8, gain: u8) -> Result<(), ConfigError> {
    let xosc = read32(XOSC32K);
    if xosc & (XOSC_ENABLE | XOSC_XTALEN | XOSC_ONDEMAND) == XOSC_ENABLE | XOSC_XTALEN && read32(OSC_STATUS) & XOSC32KRDY != 0 {
        return Ok(());
    }
    // STARTUP, CGM and XTALEN take a write only while it's off.
    let xosc = xosc & !(XOSC_STARTUP_MASK | XOSC_CGM_MASK | XOSC_ONDEMAND | XOSC_ENABLE);
    write32(XOSC32K, xosc);
    while read32(OSC_STATUS) & XOSC32KRDY != 0 {}
    let fields = (u32::from(startup.min(10)) << XOSC_STARTUP_SHIFT) | (u32::from(gain.min(15)) << XOSC_CGM_SHIFT);
    write32(XOSC32K, xosc | XOSC_XTALEN | fields);
    write32(XOSC32K, read32(XOSC32K) | XOSC_ENABLE);
    for _ in 0..XOSC_POLLS {
        if read32(OSC_STATUS) & XOSC32KRDY != 0 {
            return Ok(());
        }
    }
    Err(ConfigError::NoCrystal)
}

fn sync() {
    while read32(SYNCBUSY) & SYNC_ALL != 0 {}
}

fn read16(address: usize) -> u16 {
    // SAFETY: an RTC register, read at its width.
    unsafe { read_volatile(address as *const u16) }
}

fn write8(address: usize, value: u8) {
    // SAFETY: an RTC register, written at its width.
    unsafe { write_volatile(address as *mut u8, value) }
}

fn write16(address: usize, value: u16) {
    // SAFETY: as for `write8`.
    unsafe { write_volatile(address as *mut u16, value) }
}

fn read32(address: usize) -> u32 {
    // SAFETY: an RTC, OSC32KCTRL or MCLK register, read at its width.
    unsafe { read_volatile(address as *const u32) }
}

fn write32(address: usize, value: u32) {
    // SAFETY: as for `read32`.
    unsafe { write_volatile(address as *mut u32, value) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_round_trip() {
        // 2026-11-15 12:34:56 UTC.
        let t = DateTime::new(2026, 11, 15, 12, 34, 56).unwrap();
        assert_eq!(t.to_unix(), 1_794_746_096);
        assert_eq!(DateTime::from_unix(1_794_746_096), Ok(t));
        assert_eq!(DateTime::MIN.to_unix(), 946_684_800);
        assert_eq!(DateTime::from_unix(946_684_800), Ok(DateTime::MIN));
        assert_eq!(DateTime::from_unix(DateTime::MAX.to_unix()), Ok(DateTime::MAX));
        assert_eq!(DateTime::from_unix(946_684_799), Err(OutOfRange));
        assert_eq!(DateTime::from_unix(DateTime::MAX.to_unix() + 1), Err(OutOfRange));
    }

    #[test]
    fn dates_checked() {
        assert!(DateTime::new(2024, 2, 29, 0, 0, 0).is_ok());
        assert_eq!(DateTime::new(2025, 2, 29, 0, 0, 0), Err(OutOfRange));
        assert_eq!(DateTime::new(2026, 4, 31, 0, 0, 0), Err(OutOfRange));
        assert_eq!(DateTime::new(1999, 12, 31, 23, 59, 59), Err(OutOfRange));
        assert_eq!(DateTime::new(2026, 1, 1, 24, 0, 0), Err(OutOfRange));
    }

    #[test]
    fn register_layout() {
        let t = DateTime::new(2063, 12, 31, 23, 59, 59).unwrap();
        assert_eq!(DateTime::from_register(t.to_register()), Some(t));
        assert_eq!(DateTime::MIN.to_register(), (1 << 22) | (1 << 17));
    }
}

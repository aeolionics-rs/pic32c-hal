// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! An [embassy-time](https://docs.rs/embassy-time) driver on a free-running
//! 32- or 16-bit counter, chosen by feature as embassy-stm32 chooses its
//! timer:
//!
//! | Feature | Chip | Counter |
//! |---|---|---|
//! | `time-driver-tcc0` (`time-driver`'s choice) to `-tcc3` | PIC32CK | TCC0 to TCC3, 32 bits |
//! | `time-driver-tcc4` to `-tcc7` | PIC32CK | TCC4 to TCC7, 16 bits |
//! | `time-driver-tc0-tc1` (`time-driver`'s choice) | PIC32CM PL10 | TC0 and TC1 paired, 32 bits |
//! | `time-driver-tc0`, `-tc1`, `-tc2` | PIC32CM PL10 | one TC, 16 bits |
//! | `time-driver-tcc0` (`time-driver`'s choice) to `-tcc6` | PIC32CM SG00 | TCC0 to TCC6, 16 bits |
//!
//! The PIC32CK has no basic TC, and the PL10's TCC0 is only 16 bits. The
//! counter the driver takes is missing from the
//! [`Peripherals`](crate::Peripherals) (where the chip lists it), and with
//! feature `rt` the HAL defines its vectors (a TCC's `OTHER`, `MC0` and `MC1`;
//! a TC's one). Its generic clock channel may be shared: TCC0 with TCC1, TCC2
//! with TCC3, TC0 with TC1, so the other of the pair runs from the time
//! driver's generator.
//!
//! The counter runs free from its generic clock through the prescaler at
//! [`TICK_HZ`] (1 MHz unless the application
//! picks another `tick-hz-*` feature of embassy-time). The driver extends it
//! to 64 bits by counting half-wraps ("periods"): the overflow at 0 and a
//! match on compare channel 0 at half the range each start one. Compare
//! channel 1 is the alarm. A 32-bit counter wraps every 2^32 ticks, 71.6
//! minutes at 1 MHz; a 16-bit one every 65.5 ms, so it interrupts about 30
//! times a second whatever is scheduled, and its interrupt must run within
//! half a wrap (32.8 ms) of each half-wrap, or time goes back.
//!
//! [`crate::init`] starts it from the generator its [`Config`] names, one
//! the [clocks](crate::clocks) configuration sets to `TICK_HZ` times 1, 2,
//! 4, 8, 16, 64, 256 or 1024 (the higher, the quicker a read of the count
//! synchronises: at 16 MHz it takes well under a microsecond).
//!
//! The timer queue is embassy-time-queue-utils' default: embassy-executor's
//! integrated queue, or a generic one if the application turns on one of
//! embassy-time's `generic-queue-*` features.

use core::cell::{Cell, RefCell};
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::Waker;

use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::{Driver, TICK_HZ};
use embassy_time_queue_utils::Queue;

#[cfg(any(pic32ck, pic32cm_sg))]
mod hw {
    //! A TCC on the PIC32CK (DS60001795 chapter 48): TCC0 to TCC3 count 32
    //! bits, TCC4 to TCC7 16. Their registers are laid out alike. The
    //! PIC32CM SG00's seven TCCs are the same IP, all 16-bit with two
    //! compare channels; their overflow vector is `TCCn_DFS`, not
    //! `TCCn_OTHER`.
    use core::ptr::{read_volatile, write_volatile};

    use critical_section::CriticalSection;

    use crate::interrupt::{Interrupt, InterruptExt};

    /// The chosen TCC: its base address, GCLK peripheral channel (table
    /// 20-2; the TCC chapter's setup steps give wrong ones), counter width
    /// and vectors.
    macro_rules! timer {
        ($base:literal, $channel:literal, $bits:literal, $other:ident, $mc0:ident, $mc1:ident) => {
            const BASE: usize = $base;
            const GCLK_CHANNEL: usize = $channel;
            pub(super) const BITS: u32 = $bits;

            /// Unmasks the timer's vectors in the NVIC.
            pub(super) fn unmask() {
                for irq in [Interrupt::$other, Interrupt::$mc0, Interrupt::$mc1] {
                    irq.unpend();
                    // SAFETY: the vectors below only run the driver's handler.
                    unsafe { irq.enable() };
                }
            }

            #[cfg(feature = "rt")]
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn $other() {
                super::on_interrupt();
            }

            #[cfg(feature = "rt")]
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn $mc0() {
                super::on_interrupt();
            }

            #[cfg(feature = "rt")]
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn $mc1() {
                super::on_interrupt();
            }
        };
    }

    #[cfg(all(pic32ck, time_driver = "tcc0"))]
    timer!(0x4481_8000, 23, 32, TCC0_OTHER, TCC0_MC0, TCC0_MC1);
    #[cfg(all(pic32ck, time_driver = "tcc1"))]
    timer!(0x4481_A000, 23, 32, TCC1_OTHER, TCC1_MC0, TCC1_MC1);
    #[cfg(all(pic32ck, time_driver = "tcc2"))]
    timer!(0x4481_C000, 24, 32, TCC2_OTHER, TCC2_MC0, TCC2_MC1);
    #[cfg(all(pic32ck, time_driver = "tcc3"))]
    timer!(0x4481_E000, 24, 32, TCC3_OTHER, TCC3_MC0, TCC3_MC1);
    #[cfg(all(pic32ck, time_driver = "tcc4"))]
    timer!(0x4500_8000, 29, 16, TCC4_OTHER, TCC4_MC0, TCC4_MC1);
    #[cfg(all(pic32ck, time_driver = "tcc5"))]
    timer!(0x4500_A000, 30, 16, TCC5_OTHER, TCC5_MC0, TCC5_MC1);
    #[cfg(all(pic32ck, time_driver = "tcc6"))]
    timer!(0x4500_C000, 31, 16, TCC6_OTHER, TCC6_MC0, TCC6_MC1);
    #[cfg(all(pic32ck, time_driver = "tcc7"))]
    timer!(0x4500_E000, 32, 16, TCC7_OTHER, TCC7_MC0, TCC7_MC1);
    // The SG00's, with their GCLK channels from the ATDF (TCC0 and TCC1
    // share one).
    #[cfg(all(pic32cm_sg, time_driver = "tcc0"))]
    timer!(0x4481_0000, 25, 16, TCC0_DFS, TCC0_MC0, TCC0_MC1);
    #[cfg(all(pic32cm_sg, time_driver = "tcc1"))]
    timer!(0x4481_2000, 25, 16, TCC1_DFS, TCC1_MC0, TCC1_MC1);
    #[cfg(all(pic32cm_sg, time_driver = "tcc2"))]
    timer!(0x4481_4000, 26, 16, TCC2_DFS, TCC2_MC0, TCC2_MC1);
    #[cfg(all(pic32cm_sg, time_driver = "tcc3"))]
    timer!(0x4481_6000, 27, 16, TCC3_DFS, TCC3_MC0, TCC3_MC1);
    #[cfg(all(pic32cm_sg, time_driver = "tcc4"))]
    timer!(0x4482_4000, 30, 16, TCC4_DFS, TCC4_MC0, TCC4_MC1);
    #[cfg(all(pic32cm_sg, time_driver = "tcc5"))]
    timer!(0x4482_6000, 31, 16, TCC5_DFS, TCC5_MC0, TCC5_MC1);
    #[cfg(all(pic32cm_sg, time_driver = "tcc6"))]
    timer!(0x4482_8000, 32, 16, TCC6_DFS, TCC6_MC0, TCC6_MC1);

    const CTRLA: usize = BASE;
    const CTRLBSET: usize = BASE + 0x05;
    const SYNCBUSY: usize = BASE + 0x08;
    const INTENCLR: usize = BASE + 0x24;
    const INTENSET: usize = BASE + 0x28;
    const INTFLAG: usize = BASE + 0x2C;
    const COUNT: usize = BASE + 0x34;
    const PER: usize = BASE + 0x40;
    const CC0: usize = BASE + 0x44;
    const CC1: usize = BASE + 0x48;

    const CTRLA_SWRST: u32 = 1 << 0;
    const CTRLA_ENABLE: u32 = 1 << 1;
    const CTRLA_PRESCALER_SHIFT: u32 = 8;
    /// CTRLBSET.CMD = READSYNC: copy the count to COUNT for reading.
    const CMD_READSYNC: u8 = 4 << 5;
    const CMD_MASK: u8 = 7 << 5;

    const SYNC_SWRST: u32 = 1 << 0;
    const SYNC_ENABLE: u32 = 1 << 1;
    const SYNC_CTRLB: u32 = 1 << 2;
    const SYNC_COUNT: u32 = 1 << 4;
    const SYNC_PER: u32 = 1 << 7;
    const SYNC_CC0: u32 = 1 << 8;
    const SYNC_CC1: u32 = 1 << 9;

    pub(super) const OVF: u32 = 1 << 0;
    pub(super) const MC0: u32 = 1 << 16;
    pub(super) const MC1: u32 = 1 << 17;

    fn read(address: usize) -> u32 {
        // SAFETY: a register of the driver's TCC, read whole.
        unsafe { read_volatile(address as *const u32) }
    }

    fn write(address: usize, value: u32) {
        // SAFETY: a register of the driver's TCC; only this driver uses it.
        unsafe { write_volatile(address as *mut u32, value) }
    }

    fn wait_sync(mask: u32) {
        while read(SYNCBUSY) & mask != 0 {}
    }

    /// Resets the counter, set to run up to `top` and wrap, from
    /// `generator` through `prescaler`, at `count`, CC0 at `half`; not yet
    /// enabled.
    pub(super) fn setup(generator: usize, prescaler: u32, top: u32, count: u32, half: u32) {
        crate::clocks::enable_channel(GCLK_CHANNEL, generator);
        write(CTRLA, CTRLA_SWRST);
        wait_sync(SYNC_SWRST);
        // Normal frequency waveform generation (WAVE's reset value): the
        // counter runs up to PER and wraps to 0.
        write(CTRLA, prescaler << CTRLA_PRESCALER_SHIFT);
        write(PER, top);
        wait_sync(SYNC_PER);
        write(CC0, half);
        wait_sync(SYNC_CC0);
        write(COUNT, count);
        wait_sync(SYNC_COUNT);
    }

    pub(super) fn enable(prescaler: u32) {
        write(CTRLA, (prescaler << CTRLA_PRESCALER_SHIFT) | CTRLA_ENABLE);
        wait_sync(SYNC_ENABLE);
    }

    /// The counter, synchronised for reading (`READSYNC`, then wait for the
    /// command to finish). Call inside a critical section, so one read's
    /// command doesn't overlap another's.
    pub(super) fn read_count(_cs: CriticalSection) -> u32 {
        // SAFETY: the TCC's CTRLBSET, a byte register.
        unsafe {
            write_volatile(CTRLBSET as *mut u8, CMD_READSYNC);
            wait_sync(SYNC_CTRLB);
            while read_volatile(CTRLBSET as *const u8) & CMD_MASK != 0 {}
        }
        read(COUNT)
    }

    pub(super) fn set_cc1(value: u32) {
        write(CC1, value);
        wait_sync(SYNC_CC1);
    }

    pub(super) fn flags() -> u32 {
        read(INTFLAG)
    }

    pub(super) fn clear(mask: u32) {
        write(INTFLAG, mask);
    }

    pub(super) fn enabled() -> u32 {
        read(INTENSET)
    }

    pub(super) fn enable_interrupts(mask: u32) {
        write(INTENSET, mask);
    }

    pub(super) fn disable_interrupts(mask: u32) {
        write(INTENCLR, mask);
    }
}

#[cfg(pic32cm_pl)]
mod hw {
    //! A TC on the PIC32CM PL10: TC0 and TC1 paired as one 32-bit counter
    //! (COUNT32, configured through TC0), or one TC as 16 bits (COUNT16).
    //! In both modes TOP is the counter's maximum.
    use core::ptr::{read_volatile, write_volatile};

    use critical_section::CriticalSection;

    use crate::interrupt::{Interrupt, InterruptExt};

    /// The chosen TC: its base address, GCLK peripheral channel, bus clocks
    /// (MCLK.APBCMASK), counter width and vector.
    macro_rules! timer {
        ($base:literal, $channel:literal, $apbc:expr, $bits:literal, $irq:ident) => {
            const BASE: usize = $base;
            const GCLK_CHANNEL: usize = $channel;
            const APBC: u32 = $apbc;
            pub(super) const BITS: u32 = $bits;

            /// Unmasks the timer's vector in the NVIC.
            pub(super) fn unmask() {
                Interrupt::$irq.unpend();
                // SAFETY: the vector below only runs the driver's handler.
                unsafe { Interrupt::$irq.enable() };
            }

            #[cfg(feature = "rt")]
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn $irq() {
                super::on_interrupt();
            }
        };
    }

    // TC0 and TC1 share GCLK channel 9; TC2 has channel 10.
    #[cfg(time_driver = "tc0_tc1")]
    timer!(0x4200_0C00, 9, (1 << 3) | (1 << 4), 32, TC0);
    #[cfg(time_driver = "tc0")]
    timer!(0x4200_0C00, 9, 1 << 3, 16, TC0);
    #[cfg(time_driver = "tc1")]
    timer!(0x4200_1000, 9, 1 << 4, 16, TC1);
    #[cfg(time_driver = "tc2")]
    timer!(0x4200_1400, 10, 1 << 5, 16, TC2);

    const CTRLA: usize = BASE;
    const CTRLBSET: usize = BASE + 0x05;
    const INTENCLR: usize = BASE + 0x08;
    const INTENSET: usize = BASE + 0x09;
    const INTFLAG: usize = BASE + 0x0A;
    const SYNCBUSY: usize = BASE + 0x10;
    /// COUNT, then CC0 and CC1, each as wide as the counter.
    const COUNT: usize = BASE + 0x14;
    const CC0: usize = BASE + 0x1C;
    const CC1: usize = CC0 + BITS as usize / 8;

    const CTRLA_SWRST: u32 = 1 << 0;
    const CTRLA_ENABLE: u32 = 1 << 1;
    /// CTRLA.MODE: COUNT32 (pairing TC1 with TC0) or COUNT16.
    const CTRLA_MODE: u32 = if BITS == 32 { 2 << 2 } else { 0 };
    const CTRLA_PRESCALER_SHIFT: u32 = 8;
    const CMD_READSYNC: u8 = 4 << 5;
    const CMD_MASK: u8 = 7 << 5;

    const SYNC_SWRST: u32 = 1 << 0;
    const SYNC_ENABLE: u32 = 1 << 1;
    const SYNC_CTRLB: u32 = 1 << 2;
    const SYNC_COUNT: u32 = 1 << 4;
    const SYNC_CC0: u32 = 1 << 6;
    const SYNC_CC1: u32 = 1 << 7;

    pub(super) const OVF: u32 = 1 << 0;
    pub(super) const MC0: u32 = 1 << 4;
    pub(super) const MC1: u32 = 1 << 5;

    fn read32(address: usize) -> u32 {
        // SAFETY: a register of the driver's TC, read whole.
        unsafe { read_volatile(address as *const u32) }
    }

    fn write32(address: usize, value: u32) {
        // SAFETY: a register of the driver's TC; only this driver uses it.
        unsafe { write_volatile(address as *mut u32, value) }
    }

    fn read8(address: usize) -> u8 {
        // SAFETY: a byte register of the driver's TC.
        unsafe { read_volatile(address as *const u8) }
    }

    fn write8(address: usize, value: u8) {
        // SAFETY: a byte register of the driver's TC; only this driver uses it.
        unsafe { write_volatile(address as *mut u8, value) }
    }

    /// COUNT or a CC, at the counter's width: in 16-bit mode CC0 and CC1
    /// are neighbouring halfwords, so a word write would reach both.
    fn read_counter(address: usize) -> u32 {
        if BITS == 32 {
            read32(address)
        } else {
            // SAFETY: a halfword register of the driver's TC.
            u32::from(unsafe { read_volatile(address as *const u16) })
        }
    }

    #[expect(clippy::cast_possible_truncation, reason = "a 16-bit counter's value is in the low half")]
    fn write_counter(address: usize, value: u32) {
        if BITS == 32 {
            write32(address, value);
        } else {
            // SAFETY: a halfword register of the driver's TC; only this
            // driver uses it.
            unsafe { write_volatile(address as *mut u16, value as u16) }
        }
    }

    fn wait_sync(mask: u32) {
        while read32(SYNCBUSY) & mask != 0 {}
    }

    /// Resets the counter from `generator` through `prescaler`, at `count`,
    /// CC0 at `half`; not yet enabled. TOP is fixed at the maximum, which
    /// `top` must be.
    pub(super) fn setup(generator: usize, prescaler: u32, top: u32, count: u32, half: u32) {
        debug_assert_eq!(top, super::MASK);
        crate::clocks::enable_apbc(APBC);
        crate::clocks::enable_channel(GCLK_CHANNEL, generator);
        write32(CTRLA, CTRLA_SWRST);
        wait_sync(SYNC_SWRST);
        write32(CTRLA, CTRLA_MODE | (prescaler << CTRLA_PRESCALER_SHIFT));
        write_counter(CC0, half);
        wait_sync(SYNC_CC0);
        write_counter(COUNT, count);
        wait_sync(SYNC_COUNT);
    }

    pub(super) fn enable(prescaler: u32) {
        write32(CTRLA, CTRLA_MODE | (prescaler << CTRLA_PRESCALER_SHIFT) | CTRLA_ENABLE);
        wait_sync(SYNC_ENABLE);
    }

    /// The counter, synchronised for reading, as for the TCC.
    pub(super) fn read_count(_cs: CriticalSection) -> u32 {
        write8(CTRLBSET, CMD_READSYNC);
        wait_sync(SYNC_CTRLB);
        while read8(CTRLBSET) & CMD_MASK != 0 {}
        read_counter(COUNT)
    }

    pub(super) fn set_cc1(value: u32) {
        write_counter(CC1, value);
        wait_sync(SYNC_CC1);
    }

    pub(super) fn flags() -> u32 {
        u32::from(read8(INTFLAG))
    }

    #[expect(clippy::cast_possible_truncation, reason = "the flags are in the low byte")]
    pub(super) fn clear(mask: u32) {
        write8(INTFLAG, mask as u8);
    }

    pub(super) fn enabled() -> u32 {
        u32::from(read8(INTENSET))
    }

    #[expect(clippy::cast_possible_truncation, reason = "the flags are in the low byte")]
    pub(super) fn enable_interrupts(mask: u32) {
        write8(INTENSET, mask as u8);
    }

    #[expect(clippy::cast_possible_truncation, reason = "the flags are in the low byte")]
    pub(super) fn disable_interrupts(mask: u32) {
        write8(INTENCLR, mask as u8);
    }
}

use hw::{BITS, MC0, MC1, OVF};

/// The counter's maximum.
const MASK: u32 = mask(BITS);
/// Half the counter's range: compare channel 0's match.
const HALF: u32 = 1 << (BITS - 1);
/// An alarm at most this far ahead is armed at once; one further ahead is
/// armed by the period that brings it this close. Three quarters of a wrap,
/// so an armed alarm's low bits can't match before its time.
const ARM_WITHIN: u64 = arm_within(BITS);

const fn mask(bits: u32) -> u32 {
    if bits == 32 { u32::MAX } else { (1 << bits) - 1 }
}

const fn arm_within(bits: u32) -> u64 {
    3 << (bits - 2)
}

/// The prescaler's divisions, by CTRLA.PRESCALER value.
const PRESCALERS: [u64; 8] = [1, 2, 4, 8, 16, 64, 256, 1024];

/// The time driver's configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// The generic clock generator feeding the counter.
    pub generator: usize,
    /// The time to start from, in ticks.
    pub start: u32,
}

impl Config {
    /// The counter from `generator`, counting from 0.
    #[must_use]
    pub const fn new(generator: usize) -> Self {
        Self { generator, start: 0 }
    }

    /// Starts the time at `ticks` instead of 0, so a bench can test the
    /// counter's half-wrap and wrap without waiting for them (for a 32-bit
    /// counter, 2^31 and 2^32).
    #[must_use]
    pub const fn starting_at(self, ticks: u32) -> Self {
        Self { start: ticks, ..self }
    }
}

/// The prescaler setting that divides `input_hz` to `TICK_HZ`.
fn prescaler_for(input_hz: u32) -> Option<u32> {
    let position = PRESCALERS.iter().position(|&division| u64::from(input_hz) == TICK_HZ * division)?;
    u32::try_from(position).ok()
}

/// The time from the period count and a `bits`-wide counter.
///
/// The period is even while the counter is in its lower half and odd while
/// it's in its upper half, except just after the counter crosses a half and
/// before the interrupt counts the new period; a counter in the "wrong" half
/// for the period means that, and belongs to the next period.
const fn calc_now_bits(bits: u32, period: u64, counter: u32) -> u64 {
    let flip = ((period & 1) as u32) << (bits - 1);
    (period << (bits - 1)) + (counter ^ flip) as u64
}

fn calc_now(period: u64, counter: u32) -> u64 {
    calc_now_bits(BITS, period, counter)
}

/// Sets the compare to `at`'s low bits, clears any stale match, and enables
/// the alarm's interrupt.
fn arm(at: u64) {
    #[expect(clippy::cast_possible_truncation, reason = "the compare channel takes the counter's low bits")]
    hw::set_cc1(at as u32 & MASK);
    hw::clear(MC1);
    hw::enable_interrupts(MC1);
}

struct TimerDriver {
    /// Half-wraps since the time was 0.
    period: Mutex<Cell<u64>>,
    /// The armed alarm's time, `u64::MAX` if none.
    alarm: Mutex<Cell<u64>>,
    queue: Mutex<RefCell<Queue>>,
}

static STARTED: AtomicBool = AtomicBool::new(false);

embassy_time_driver::time_driver_impl!(static DRIVER: TimerDriver = TimerDriver {
    period: Mutex::new(Cell::new(0)),
    alarm: Mutex::new(Cell::new(u64::MAX)),
    queue: Mutex::new(RefCell::new(Queue::new())),
});

impl TimerDriver {
    fn now_in(&self, cs: CriticalSection) -> u64 {
        if !STARTED.load(Ordering::Acquire) {
            return 0;
        }
        // The period first: the counter may cross a half after it, which
        // `calc_now` allows for, but not before (the interrupt that counts
        // it waits for the critical section).
        let period = self.period.borrow(cs).get();
        calc_now(period, hw::read_count(cs))
    }

    /// Counts a period, and arms the alarm if it's now close enough.
    fn next_period(&self, cs: CriticalSection) {
        let period = self.period.borrow(cs).get() + 1;
        self.period.borrow(cs).set(period);
        let start = period << (BITS - 1);
        let at = self.alarm.borrow(cs).get();
        if at < start + ARM_WITHIN && hw::enabled() & MC1 == 0 {
            // `set_alarm` left it unarmed, so it is at least a quarter wrap
            // past this period's start, and so is the compare.
            arm(at);
        }
    }

    /// Arms the alarm for `at`, given the time `now`; false if `at` has
    /// already come (the caller then runs the queue again).
    fn set_alarm(&self, cs: CriticalSection, at: u64, now: u64) -> bool {
        let alarm = self.alarm.borrow(cs);
        hw::disable_interrupts(MC1);
        if at <= now {
            alarm.set(u64::MAX);
            return false;
        }
        alarm.set(at);
        if at - now >= ARM_WITHIN {
            // None, or too far for the compare: `next_period` arms it once
            // it's near enough.
            return true;
        }
        arm(at);
        // The counter may have passed `at` while the compare was being set.
        if at <= self.now_in(cs) {
            hw::disable_interrupts(MC1);
            alarm.set(u64::MAX);
            return false;
        }
        true
    }

    /// Wakes what's due and arms the alarm for what's next.
    fn run_queue(&self, cs: CriticalSection, queue: &mut Queue) {
        loop {
            let now = self.now_in(cs);
            if self.set_alarm(cs, queue.next_expiration(now), now) {
                return;
            }
        }
    }
}

impl Driver for TimerDriver {
    fn now(&self) -> u64 {
        critical_section::with(|cs| self.now_in(cs))
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow_ref_mut(cs);
            if queue.schedule_wake(at, waker) {
                self.run_queue(cs, &mut queue);
            }
        });
    }
}

/// Starts the counter as `config` says, from its generator running at `hz`,
/// and unmasks its interrupts.
///
/// # Panics
///
/// If no prescaler divides `hz` to `TICK_HZ`.
///
/// # Safety
///
/// Call once, from [`crate::init`], after the clocks. Nothing else may use
/// the counter.
pub(crate) unsafe fn init(config: Config, hz: u32) {
    let Some(prescaler) = prescaler_for(hz) else {
        panic!("time driver: no prescaler divides its generator's rate to the tick rate");
    };
    hw::setup(config.generator, prescaler, MASK, config.start & MASK, HALF);
    critical_section::with(|cs| DRIVER.period.borrow(cs).set(u64::from(config.start >> (BITS - 1))));
    hw::clear(OVF | MC0 | MC1);
    hw::enable_interrupts(OVF | MC0);
    hw::enable(prescaler);
    STARTED.store(true, Ordering::Release);
    hw::unmask();
}

/// The timer's interrupt handler, for its vectors.
#[cfg_attr(not(feature = "rt"), allow(dead_code, reason = "only the vectors call it"))]
fn on_interrupt() {
    critical_section::with(|cs| {
        let pending = hw::flags() & hw::enabled() & (OVF | MC0 | MC1);
        hw::clear(pending);
        if pending & OVF != 0 {
            DRIVER.next_period(cs);
        }
        if pending & MC0 != 0 {
            DRIVER.next_period(cs);
        }
        if pending & MC1 != 0 {
            hw::disable_interrupts(MC1);
            DRIVER.alarm.borrow(cs).set(u64::MAX);
            DRIVER.run_queue(cs, &mut DRIVER.queue.borrow_ref_mut(cs));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIDTHS: [u32; 2] = [32, 16];

    fn half(bits: u32) -> u32 {
        1 << (bits - 1)
    }

    #[test]
    fn counts_from_the_period_and_counter() {
        for bits in WIDTHS {
            let (h, max, wrap) = (half(bits), mask(bits), 1u64 << bits);
            assert_eq!(calc_now_bits(bits, 0, 0), 0);
            assert_eq!(calc_now_bits(bits, 0, 5), 5);
            assert_eq!(calc_now_bits(bits, 1, h), u64::from(h));
            assert_eq!(calc_now_bits(bits, 1, max), u64::from(max));
            assert_eq!(calc_now_bits(bits, 2, 7), wrap + 7);
            assert_eq!(calc_now_bits(bits, 3, h + 7), wrap + u64::from(h) + 7);
        }
    }

    #[test]
    fn a_crossing_before_the_interrupt_belongs_to_the_next_period() {
        for bits in WIDTHS {
            let (h, wrap) = (half(bits), 1u64 << bits);
            // Past the half, period not yet counted.
            assert_eq!(calc_now_bits(bits, 0, h + 3), u64::from(h) + 3);
            // Past the wrap, period not yet counted.
            assert_eq!(calc_now_bits(bits, 1, 3), wrap + 3);
            assert_eq!(calc_now_bits(bits, 5, 3), 3 * wrap + 3);
        }
    }

    #[test]
    fn is_monotonic_across_every_crossing() {
        for bits in WIDTHS {
            let (h, max) = (half(bits), mask(bits));
            // Each step's counter value, with the period as the interrupt
            // would have it either before or after it runs.
            let mut last = 0;
            for period in 0..6u64 {
                for counter in [0, 1, h - 1, h, h + 1, max] {
                    let in_period = (counter >= h) == (period & 1 == 1);
                    if !in_period {
                        continue;
                    }
                    let now = calc_now_bits(bits, period, counter);
                    assert!(now >= last, "{bits} bits: period {period} counter {counter:#x}");
                    last = now;
                }
            }
        }
    }

    #[test]
    fn starting_time_reads_back() {
        for bits in WIDTHS {
            for start in [0, 1, half(bits) - 1, half(bits), mask(bits), 0x7FB3_B4C0, u32::MAX - 1_000_000, u32::MAX] {
                let period = u64::from(start >> (bits - 1));
                assert_eq!(calc_now_bits(bits, period, start & mask(bits)), u64::from(start), "{bits} bits: {start:#x}");
            }
        }
    }

    #[test]
    fn arms_within_three_quarters_of_a_wrap() {
        assert_eq!(arm_within(32), 0xC000_0000);
        assert_eq!(arm_within(16), 0xC000);
    }

    #[test]
    fn picks_the_prescaler() {
        assert_eq!(TICK_HZ, 1_000_000);
        assert_eq!(prescaler_for(1_000_000), Some(0));
        assert_eq!(prescaler_for(16_000_000), Some(4));
        assert_eq!(prescaler_for(64_000_000), Some(5));
        assert_eq!(prescaler_for(48_000_000), None);
    }
}

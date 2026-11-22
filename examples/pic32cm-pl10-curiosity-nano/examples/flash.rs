// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pic32c-hal`'s `flash` on the PL10's NVMCTRL, on two scratch areas of
//! PFM far above the program: the top 2 KiB (offset `0xF800`, 4 pages) and
//! a page at offset `0xC000`. PFM is one panel, so the CPU, running from
//! it, stalls through every erase and write: the `SysTick`s counted
//! meanwhile (at 10 kHz, the handler in flash too) show it.
//!
//! On each: erase and check it reads erased; write a page from SRAM, words
//! from a pattern held in flash, and a run starting mid-page; read each
//! back; check the bytes around a write stay erased; check misaligned and
//! out-of-range requests are refused; erase again. Timings, the `SysTick`s
//! taken meanwhile and the NVMCTRL's settings go out over the virtual COM
//! port; LED0 blinks slowly when all passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use embassy_time::{Duration, block_for};
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use panic_halt as _;
use pic32c_hal::flash::{Error, Flash, PAGE_SIZE, WORD_SIZE};
use pic32c_hal::gpio::{Level, Output};
use pic32cm_pl10_curiosity_nano::{Log, micros};

const CAPACITY: usize = pic32c_hal::FLASH_SIZE;
const TOP_AREA: u32 = 0xF800;
const MIDDLE_AREA: u32 = 0xC000;

/// `SysTick`s taken.
static SYSTICKS: AtomicU32 = AtomicU32::new(0);

#[exception]
fn SysTick() {
    // No read-modify-write atomics on the M0+; only this handler writes.
    SYSTICKS.store(SYSTICKS.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
}

/// Half a page of pattern in flash, for writes whose data isn't in SRAM.
static FLASH_PATTERN: [u8; 256] = {
    let mut row = [0; 256];
    let mut i = 0;
    while i < 256 {
        #[expect(clippy::cast_possible_truncation, reason = "a byte of pattern")]
        let byte = (i * 7 + 3) as u8 ^ 0xA5;
        row[i] = byte;
        i += 1;
    }
    row
};

/// A page of data in SRAM.
struct Page([u8; PAGE_SIZE]);

/// A length or offset as the flash API's `u32`.
fn u(n: usize) -> u32 {
    u32::try_from(n).unwrap()
}

fn pattern(seed: u32, page: &mut Page) {
    for (i, byte) in page.0.iter_mut().enumerate() {
        let value = (u32::try_from(i).unwrap().wrapping_mul(0x9E37_79B1) ^ seed) >> 13;
        #[expect(clippy::cast_possible_truncation, reason = "bytes of pattern")]
        let value = value as u8;
        *byte = value;
    }
}

#[entry]
fn main() -> ! {
    let peripherals = pic32cm_pl10_curiosity_nano::init();
    let led = Output::new(peripherals.PB02, Level::High);
    let cpu_hz = pic32c_hal::clocks::generator_hz(0).unwrap_or(24_000_000);
    let mut core = cortex_m::Peripherals::take().unwrap();
    core.SYST.set_clock_source(SystClkSource::Core);
    core.SYST.set_reload(cpu_hz / 10_000 - 1);
    core.SYST.clear_current();
    core.SYST.enable_interrupt();
    core.SYST.enable_counter();
    let mut log = Log::init(peripherals.SERCOM1, peripherals.PB00, peripherals.PB01);
    let _ = writeln!(log, "\npic32cm-pl10-curiosity-nano flash: CPU {cpu_hz} Hz");
    // SAFETY: NVMCTRL registers, read only.
    let r = |address: usize| unsafe { core::ptr::read_volatile(address as *const u32) };
    let _ = writeln!(
        log,
        "NVMCTRL PARAM {:#010x} STATUS {:#x} INTFLAG {:#x} LOCK {:#06x} WPCTRL {:#x}",
        r(0x4100_4008),
        r(0x4100_401C),
        r(0x4100_4014),
        r(0x4100_4024),
        r(0x4100_402C)
    );

    // The areas used are far above the program.
    let mut flash = Flash::new_blocking(peripherals.NVMCTRL);
    let mut failures = 0;
    failures += area(&mut flash, &mut log, "top", TOP_AREA, 4);
    failures += area(&mut flash, &mut log, "middle", MIDDLE_AREA, 1);
    failures += refusals(&mut flash, &mut log);
    if failures == 0 {
        let _ = writeln!(log, "flash: all passed");
    } else {
        let _ = writeln!(log, "flash: {failures} failures");
    }
    halt(&mut log, failures, led);
}

fn halt(log: &mut Log, failures: u32, mut led: Output<'static>) -> ! {
    log.flush();
    let period = if failures == 0 { 500 } else { 100 };
    loop {
        led.toggle();
        block_for(Duration::from_millis(period));
    }
}

/// Runs `operation`, returning its result, the µs it took and the
/// `SysTick`s taken meanwhile.
fn timed<T>(operation: impl FnOnce() -> T) -> (T, u64, u32) {
    let (start, ticks) = (micros(), SYSTICKS.load(Ordering::Relaxed));
    let result = operation();
    (result, micros() - start, SYSTICKS.load(Ordering::Relaxed) - ticks)
}

/// Whether `length` bytes at `offset` read as `expected` (or erased, if
/// `None`).
fn reads_as(flash: &mut Flash<'_>, offset: u32, length: usize, expected: Option<&[u8]>) -> bool {
    let mut buffer = [0; 128];
    let mut done = 0;
    while done < length {
        let n = buffer.len().min(length - done);
        if flash.read(offset + u(done), &mut buffer[..n]).is_err() {
            return false;
        }
        let good = match expected {
            Some(expected) => buffer[..n] == expected[done..done + n],
            None => buffer[..n].iter().all(|&b| b == 0xFF),
        };
        if !good {
            return false;
        }
        done += n;
    }
    true
}

/// The first difference between flash at `offset` and `expected` (or
/// erased), for diagnosis: offset, expected, read, and how many differ.
fn first_difference(flash: &mut Flash<'_>, offset: u32, length: usize, expected: Option<&[u8]>) -> Option<(usize, u8, u8, usize)> {
    let mut first = None;
    let mut count = 0;
    for i in 0..length {
        let mut byte = [0];
        flash.read(offset + u(i), &mut byte).ok()?;
        let want = expected.map_or(0xFF, |e| e[i]);
        if byte[0] != want {
            count += 1;
            if first.is_none() {
                first = Some((i, want, byte[0]));
            }
        }
    }
    first.map(|(i, want, got)| (i, want, got, count))
}

fn report(log: &mut Log, what: &str, result: Result<(), Error>, micros: u64, systicks: u32, good: bool) -> u32 {
    let _ = writeln!(log, "  {what}: {result:?} in {micros} µs, {systicks} SysTicks meanwhile, {}", if good { "reads back" } else { "WRONG" });
    u32::from(result.is_err() || !good)
}

fn area(flash: &mut Flash<'_>, log: &mut Log, name: &str, base: u32, pages: u32) -> u32 {
    let _ = writeln!(log, "{name}: {pages} page(s) at offset {base:#x}");
    let mut failures = 0;
    let end = base + pages * u(PAGE_SIZE);
    let length = (end - base) as usize;

    let (result, us, ticks) = timed(|| flash.erase(base, end));
    let good = reads_as(flash, base, length, None);
    failures += report(log, "erase", result, us, ticks, good);

    // Page 0: a page from SRAM.
    let mut page = Page([0; PAGE_SIZE]);
    pattern(0x1234_5678 ^ base, &mut page);
    let (result, us, ticks) = timed(|| flash.write(base, &page.0));
    let good = reads_as(flash, base, PAGE_SIZE, Some(&page.0));
    failures += report(log, "128 words (a page from SRAM)", result, us, ticks, good);
    if !good {
        let _ = writeln!(log, "  first difference: {:?}", first_difference(flash, base, PAGE_SIZE, Some(&page.0)));
    }

    if pages >= 3 {
        // Page 1: words, the data in flash.
        let target = base + u(PAGE_SIZE);
        let (result, us, ticks) = timed(|| flash.write(target, &FLASH_PATTERN));
        let good = reads_as(flash, target, FLASH_PATTERN.len(), Some(&FLASH_PATTERN));
        failures += report(log, "64 words (256 B from flash)", result, us, ticks, good);
        if !reads_as(flash, target + u(FLASH_PATTERN.len()), PAGE_SIZE - FLASH_PATTERN.len(), None) {
            let _ = writeln!(log, "  the rest of that page isn't erased");
            failures += 1;
        }

        // Page 2: a run from SRAM starting mid-page, crossing into page 3.
        let target = base + 2 * u(PAGE_SIZE);
        pattern(0xCAFE_F00D, &mut page);
        let (start, length) = (PAGE_SIZE - 3 * WORD_SIZE, 3 * WORD_SIZE + 5 * WORD_SIZE);
        let run = &page.0[..length];
        let offset = target + u(start);
        let (result, us, ticks) = timed(|| flash.write(offset, run));
        let good = reads_as(flash, offset, length, Some(run));
        failures += report(log, "8 words across a page boundary", result, us, ticks, good);
        let around = reads_as(flash, target, start, None) && reads_as(flash, offset + u(length), 2 * PAGE_SIZE - start - length, None);
        if !around {
            let _ = writeln!(log, "  bytes around that run aren't erased");
            failures += 1;
        }
    }

    let (result, us, ticks) = timed(|| flash.erase(base, end));
    let good = reads_as(flash, base, length, None);
    failures += report(log, "erase again", result, us, ticks, good);
    if !good {
        let _ = writeln!(log, "  first non-erased: {:?}", first_difference(flash, base, length, None));
    }
    failures
}

fn refusals(flash: &mut Flash<'_>, log: &mut Log) -> u32 {
    let word = [0u8; 8];
    let cases = [
        ("write at a half word", flash.write(TOP_AREA + 2, &word[..4]), Error::NotAligned),
        ("write of 2 bytes", flash.write(TOP_AREA, &word[..2]), Error::NotAligned),
        ("write past the end", flash.write(u(CAPACITY) - 4, &word), Error::OutOfBounds),
        ("erase of half a page", flash.erase(TOP_AREA, TOP_AREA + 256), Error::NotAligned),
        ("erase backwards", flash.erase(TOP_AREA + 512, TOP_AREA), Error::OutOfBounds),
    ];
    let mut failures = 0;
    for (what, result, expected) in cases {
        let good = result == Err(expected);
        let _ = writeln!(log, "{what}: {result:?}{}", if good { "" } else { ", WRONG" });
        failures += u32::from(!good);
    }
    // None of those may have touched the area.
    if !reads_as(flash, TOP_AREA, 4 * PAGE_SIZE, None) {
        let _ = writeln!(log, "a refused request changed the area");
        failures += 1;
    }
    failures
}

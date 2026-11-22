// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pic32c-hal`'s `flash` under test, on two scratch areas of PFM, empty in both
//! boards' backups and far above the program:
//! - the top 16 KiB of panel 2 (offset `0x1F_C000`, 4 pages), which this
//!   program doesn't run from, so `SysTick` should keep running through an
//!   erase or write there;
//! - the top page of panel 1 (offset `0x0F_F000`), the panel the program
//!   runs from, so the CPU should stall instead.
//!
//! On each: erase and check it reads erased; write rows from SRAM, flash
//! words from a pattern held in flash, and a run starting mid-row that mixes
//! both; read each back; check the bytes around a write stay erased; check
//! misaligned and out-of-range requests are refused; erase again. Each
//! check reads the same words just before the operation, so a stale cache
//! line would show. Timings, the `SysTick`s taken meanwhile and the FCW's
//! and FCR's settings go out over the virtual COM port; LED0 blinks slowly
//! when all passed, fast on a failure. Build with `FLASH_NO_CACHE=1` to leave
//! the CMCC off: then the CPU stalls while panel 1 is busy.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::time::Duration;

use cortex_m_rt::entry;
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use panic_halt as _;
use pic32c_hal::flash::{Error, Flash, PAGE_SIZE, ROW_SIZE, WORD_SIZE};
use pic32c_hal::gpio::{Level, Output};
use pic32ck_curiosity_ultra::time;
use pic32ck_curiosity_ultra::uart::Uart;

pic32ck_curiosity_ultra::boot!();

const CAPACITY: usize = pic32c_hal::FLASH_SIZE;
const PANEL2_AREA: u32 = 0x1F_C000;
const PANEL1_AREA: u32 = 0x0F_F000;

/// A row of pattern in flash, for writes whose data isn't in SRAM.
static FLASH_PATTERN: [u8; ROW_SIZE] = {
    let mut row = [0; ROW_SIZE];
    let mut i = 0;
    while i < ROW_SIZE {
        #[expect(clippy::cast_possible_truncation, reason = "a byte of pattern")]
        let byte = (i * 7 + 3) as u8 ^ 0xA5;
        row[i] = byte;
        i += 1;
    }
    row
};

/// A page of data in SRAM, word aligned for the FCW's row writes.
#[repr(C, align(4))]
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
    // `FLASH_NO_CACHE` (set when building) leaves the CMCC off, so code
    // runs from flash and stalls while its panel is busy.
    let cached = option_env!("FLASH_NO_CACHE").is_none();
    let mut config = pic32ck_curiosity_ultra::config(0);
    config.cache = cached;
    let peripherals = pic32ck_curiosity_ultra::init_with(config);
    let led = Output::new(peripherals.PD20, Level::High);
    time::init(cortex_m::Peripherals::take().unwrap());
    let mut log = Uart::init(peripherals.SERCOM5, peripherals.PA24, peripherals.PA25);
    let _ = writeln!(
        log,
        "\npic32ck-curiosity-ultra flash: CPU {} Hz, cache {}",
        pic32c_hal::clocks::generator_hz(0).unwrap_or(0),
        if pic32c_hal::cache::is_enabled() { "on" } else { "off" }
    );
    let r = |address: usize| unsafe { core::ptr::read_volatile(address as *const u32) };
    let _ = writeln!(
        log,
        "FCR CTRLA {:#010x} ECCCTRL {:#06x}; FCW MUTEX {:#x} STATUS {:#x} INTFLAG {:#06x} SWAP {:#x} PWP0 {:#010x} LBWP {:#x} CWP {:#x}",
        r(0x4400_2000),
        r(0x4400_201C),
        r(0x4400_4008),
        r(0x4400_4018),
        r(0x4400_4014),
        r(0x4400_4048),
        r(0x4400_404C),
        r(0x4400_406C),
        r(0x4400_4078),
    );

    // The areas used are far above the program.
    let mut flash = Flash::new_blocking(peripherals.FCW);
    let mut failures = 0;
    failures += area(&mut flash, &mut log, "panel 2", PANEL2_AREA, 4);
    failures += area(&mut flash, &mut log, "panel 1", PANEL1_AREA, 1);
    failures += refusals(&mut flash, &mut log);
    if failures == 0 {
        let _ = writeln!(log, "flash: all passed");
    } else {
        let _ = writeln!(log, "flash: {failures} failures");
    }
    halt(&mut log, failures, led);
}

fn halt(log: &mut Uart, failures: u32, mut led: Output<'static>) -> ! {
    log.flush();
    let period = if failures == 0 { 500 } else { 100 };
    loop {
        led.toggle();
        time::delay(Duration::from_millis(period));
    }
}

/// Runs `operation`, returning its result, the µs it took and the
/// `SysTick`s taken meanwhile.
fn timed<T>(operation: impl FnOnce() -> T) -> (T, u64, u32) {
    let (start, ticks) = (time::micros(), time::systicks());
    let result = operation();
    (result, time::micros() - start, time::systicks() - ticks)
}

/// Whether `length` bytes at `offset` read as `expected` (or erased, if
/// `None`).
fn reads_as(flash: &mut Flash<'_>, offset: u32, length: usize, expected: Option<&[u8]>) -> bool {
    let mut buffer = [0; 256];
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
/// erased), for diagnosis.
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

fn report(log: &mut Uart, what: &str, result: Result<(), Error>, micros: u64, systicks: u32, good: bool) -> u32 {
    let _ = writeln!(log, "  {what}: {result:?} in {micros} µs, {systicks} SysTicks meanwhile, {}", if good { "reads back" } else { "WRONG" });
    u32::from(result.is_err() || !good)
}

fn area(flash: &mut Flash<'_>, log: &mut Uart, name: &str, base: u32, pages: u32) -> u32 {
    let _ = writeln!(log, "{name}: {pages} page(s) at offset {base:#x}");
    let mut failures = 0;
    let end = base + pages * u(PAGE_SIZE);
    let length = (end - base) as usize;

    let (result, us, ticks) = timed(|| flash.erase(base, end));
    let good = reads_as(flash, base, length, None);
    failures += report(log, "erase", result, us, ticks, good);

    // Page 0: rows from SRAM.
    let mut page = Page([0; PAGE_SIZE]);
    pattern(0x1234_5678 ^ base, &mut page);
    let (result, us, ticks) = timed(|| flash.write(base, &page.0));
    let good = reads_as(flash, base, PAGE_SIZE, Some(&page.0));
    failures += report(log, "4 row writes (4 KiB from SRAM)", result, us, ticks, good);
    if !good {
        let _ = writeln!(log, "  first difference (offset, expected, read, count): {:?}", first_difference(flash, base, PAGE_SIZE, Some(&page.0)));
    }

    if pages >= 3 {
        // Page 1: flash words, the data in flash.
        let target = base + u(PAGE_SIZE);
        let (result, us, ticks) = timed(|| flash.write(target, &FLASH_PATTERN));
        let good = reads_as(flash, target, ROW_SIZE, Some(&FLASH_PATTERN));
        failures += report(log, "32 word writes (1 KiB from flash)", result, us, ticks, good);
        let rest_erased = reads_as(flash, target + u(ROW_SIZE), PAGE_SIZE - ROW_SIZE, None);
        if !rest_erased {
            let _ = writeln!(log, "  the rest of that page isn't erased");
            failures += 1;
        }

        // Page 2: a run from SRAM starting mid-row: two words, a row, a word.
        let target = base + 2 * u(PAGE_SIZE);
        pattern(0xCAFE_F00D, &mut page);
        let (start, length) = (ROW_SIZE - 2 * WORD_SIZE, 2 * WORD_SIZE + ROW_SIZE + WORD_SIZE);
        let run = &page.0[..length];
        let offset = target + u(start);
        let (result, us, ticks) = timed(|| flash.write(offset, run));
        let good = reads_as(flash, offset, length, Some(run));
        failures += report(log, "2 words, a row, a word (from SRAM)", result, us, ticks, good);
        let around = reads_as(flash, target, start, None) && reads_as(flash, offset + u(length), PAGE_SIZE - start - length, None);
        if !around {
            let _ = writeln!(log, "  bytes around that run aren't erased");
            failures += 1;
        }
    }

    let (result, us, ticks) = timed(|| flash.erase(base, end));
    let good = reads_as(flash, base, length, None);
    failures += report(log, "erase again", result, us, ticks, good);
    if !good {
        let _ = writeln!(log, "  first non-erased (offset, expected, read, count): {:?}", first_difference(flash, base, length, None));
    }
    failures
}

fn refusals(flash: &mut Flash<'_>, log: &mut Uart) -> u32 {
    let word = [0u8; WORD_SIZE];
    let cases = [
        ("write at a half word", flash.write(PANEL2_AREA + 16, &word), Error::NotAligned),
        ("write of 16 bytes", flash.write(PANEL2_AREA, &word[..16]), Error::NotAligned),
        ("write past the end", flash.write(u(CAPACITY) - 32, &[0; 64]), Error::OutOfBounds),
        ("erase of half a page", flash.erase(PANEL2_AREA, PANEL2_AREA + 2048), Error::NotAligned),
        ("erase backwards", flash.erase(PANEL2_AREA + 4096, PANEL2_AREA), Error::OutOfBounds),
    ];
    let mut failures = 0;
    for (what, result, expected) in cases {
        let good = result == Err(expected);
        let _ = writeln!(log, "{what}: {result:?}{}", if good { "" } else { ", WRONG" });
        failures += u32::from(!good);
    }
    // None of those may have touched the area.
    if !reads_as(flash, PANEL2_AREA, 4 * PAGE_SIZE, None) {
        let _ = writeln!(log, "a refused request changed the area");
        failures += 1;
    }
    failures
}

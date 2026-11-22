// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pic32c-hal`'s `uid` on the PL10: the signature row's 128-bit serial
//! number, which mustn't be all ones (unprogrammed) or all zeros, must
//! read the same twice, and must match SERNUM0 to SERNUM3 read here
//! directly. Over the virtual COM port; LED0 blinks slowly when all
//! passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use embassy_time::{Duration, block_for};
use panic_halt as _;
use pic32c_hal::gpio::{Level, Output};
use pic32cm_pl10_curiosity_nano::Log;

#[entry]
fn main() -> ! {
    let p = pic32cm_pl10_curiosity_nano::init();
    let mut led = Output::new(p.PB02, Level::High);
    let mut log = Log::init(p.SERCOM1, p.PB00, p.PB01);
    let _ = writeln!(log, "\npic32cm-pl10-curiosity-nano uid");
    let id = pic32c_hal::uid::unique_id();
    let _ = write!(log, "unique ID ");
    for byte in id {
        let _ = write!(log, "{byte:02x}");
    }
    let _ = writeln!(log);
    let mut words = [0u32; 4];
    for (n, word) in words.iter_mut().enumerate() {
        // SAFETY: the signature row's SERNUMn, read only.
        *word = unsafe { core::ptr::read_volatile((0x0D00_0220 + 8 * n) as *const u32) };
    }
    let _ = writeln!(log, "SERNUM0-3 {:08x} {:08x} {:08x} {:08x}", words[0], words[1], words[2], words[3]);
    let mut expected = [0u8; 16];
    for (bytes, word) in expected.as_chunks_mut::<4>().0.iter_mut().zip(words) {
        *bytes = word.to_le_bytes();
    }
    let good = id != [0xFF; 16] && id != [0; 16] && id == pic32c_hal::uid::unique_id() && id == expected;
    let _ = writeln!(log, "uid: {}", if good { "all passed" } else { "FAILED" });
    log.flush();
    let period = if good { 500 } else { 100 };
    loop {
        led.toggle();
        block_for(Duration::from_millis(period));
    }
}

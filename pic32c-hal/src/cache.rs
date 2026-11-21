// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The CMCC, the 4 KiB L1 cache in front of flash (DS60001795 chapter 14),
//! for instructions and data. It's off out of reset; [`crate::init`] turns
//! it on unless [`Config::cache`](crate::Config::cache) says not to. With
//! five flash wait states at 120 MHz it about halves an embassy-time wake
//! (13 µs to 7 µs on the bench).
//!
//! The flash driver invalidates it after each erase or write. Code that
//! changes flash some other way must call [`invalidate`].
//!
//! CTRL is write-only (it reads 0 whatever the state); SR.CSTS is the
//! state.

use core::ptr::{read_volatile, write_volatile};

const CMCC: usize = 0x4480_8000;
const CTRL: usize = CMCC + 0x08;
const SR: usize = CMCC + 0x0C;
const MAINT0: usize = CMCC + 0x20;
const CEN: u32 = 1 << 0;
const CSTS: u32 = 1 << 0;
const INVALL: u32 = 1 << 0;

fn read(address: usize) -> u32 {
    // SAFETY: a CMCC register, read whole.
    unsafe { read_volatile(address as *const u32) }
}

fn write(address: usize, value: u32) {
    // SAFETY: a CMCC register; this module is its only writer.
    unsafe { write_volatile(address as *mut u32, value) }
}

/// Whether the cache is on.
#[must_use]
pub fn is_enabled() -> bool {
    read(SR) & CSTS != 0
}

fn disable() {
    write(CTRL, 0);
    while is_enabled() {}
}

/// Turns the cache on (emptied first) or off.
pub(crate) fn set(enabled: bool) {
    if enabled == is_enabled() {
        return;
    }
    if enabled {
        // An invalidate needs the cache off, as it is here.
        write(MAINT0, INVALL);
        write(CTRL, CEN);
    } else {
        disable();
    }
}

/// Empties the cache, if it's on, so reads see flash as it is now: off,
/// invalidate all, on again.
pub fn invalidate() {
    if !is_enabled() {
        return;
    }
    disable();
    write(MAINT0, INVALL);
    write(CTRL, CEN);
}

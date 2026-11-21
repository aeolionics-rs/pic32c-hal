// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The chip's unique ID (DS60001795 section 11.8).

use core::ptr::read_volatile;

/// The unique ID's place in the CAL OTP page (table 11-7).
const UNIQUE_ID: usize = 0x0A00_71E0;

/// The chip's 128-bit unique ID, programmed at manufacture into the CAL OTP
/// page (which can't be written or erased): a version 4 (random) UUID, in
/// RFC 4122's byte order (DS60001795 section 11.8).
#[must_use]
pub fn unique_id() -> [u8; 16] {
    let mut id = [0; 16];
    for (i, byte) in id.iter_mut().enumerate() {
        // SAFETY: the CAL OTP page, always readable.
        *byte = unsafe { read_volatile((UNIQUE_ID + i) as *const u8) };
    }
    id
}

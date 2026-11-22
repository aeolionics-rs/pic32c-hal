// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The chip's unique ID (DS60001795 section 11.8; on the PIC32CM SG00,
//! DS60001921 section 12.5, at the same address; on the PL10, DS40002667
//! section 5.4, the signature row's serial number; on the GV00, DS60001881
//! section 7.9).

use core::ptr::read_volatile;

/// The unique ID's place in the CAL OTP page (table 11-7).
#[cfg(any(pic32ck, pic32cm_sg))]
const UNIQUE_ID: usize = 0x0A00_71E0;

/// The PL10's serial number: SERNUM0 to SERNUM3, a word each, 8 bytes
/// apart in the signature row (`0x0D00_0200`).
#[cfg(pic32cm_pl)]
const SERNUM: [usize; 4] = [0x0D00_0220, 0x0D00_0228, 0x0D00_0230, 0x0D00_0238];

/// The GV00's serial number: four words, the first apart from the others.
#[cfg(pic32cm_gv)]
const SERNUM: [usize; 4] = [0x0080_A00C, 0x0080_A040, 0x0080_A044, 0x0080_A048];

/// The chip's 128-bit unique ID, programmed at manufacture and never
/// changed.
///
/// On the PIC32CK and the SG00 it's in the CAL OTP page (which can't be
/// written or erased): a version 4 (random) UUID, in RFC 4122's byte order
/// (DS60001795 section 11.8). On the PL10 it's the signature row's serial
/// number, SERNUM0 to SERNUM3 in turn, each little-endian: unique only
/// as all 128 bits (its lot, wafer and position on the wafer), and not a
/// UUID. The GV00's serial number likewise: its words 0 to 3 in turn, each
/// little-endian, unique only as all 128 bits.
#[must_use]
pub fn unique_id() -> [u8; 16] {
    let mut id = [0; 16];
    #[cfg(any(pic32ck, pic32cm_sg))]
    for (i, byte) in id.iter_mut().enumerate() {
        // SAFETY: the CAL OTP page, always readable.
        *byte = unsafe { read_volatile((UNIQUE_ID + i) as *const u8) };
    }
    #[cfg(any(pic32cm_pl, pic32cm_gv))]
    for (word, address) in id.as_chunks_mut::<4>().0.iter_mut().zip(SERNUM) {
        // SAFETY: the signature row, always readable, read a word at a time.
        word.copy_from_slice(&unsafe { read_volatile(address as *const u32) }.to_le_bytes());
    }
    id
}

// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! An image as the bootloader finds it in a panel, and the check that it's whole.
//!
//! An image is a program linked to run from the start of program flash, its vector table first,
//! with a header after it that a host tool (`tools/host/image.py`) adds:
//!
//! - The image's length, a multiple of [`ALIGN`], goes in the vector table's word 8 (the first
//!   of the Armv8-M's reserved entries, which a program leaves zero; word 7 is `SecureFault`), so
//!   the header is found without a linker script.
//! - At that offset, [`HEADER`] bytes: [`MAGIC`], the version and the length again (each a `u32`,
//!   little-endian), zeros, then at byte 32 the SHA-256 of the image (its first `length` bytes,
//!   word 8 included).
//! - The image's [`ImageId`] is the hash's first 16 bytes.
//!
//! The hash finds an image cut short or corrupted, not one made to look whole: that takes a
//! signature, later.

use sha2::{Digest, Sha256};

use crate::decide::ImageId;

/// The header's first bytes.
pub const MAGIC: [u8; 4] = *b"P32I";
/// The header's size.
pub const HEADER: usize = 64;
/// What the length is a multiple of: the PIC32CK's flash word, so the header starts one.
pub const ALIGN: usize = 32;
/// The vector table's word that holds the length.
const LENGTH_WORD: usize = 8;

/// What a header says.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub version: u32,
    /// The image's length, the header not included.
    pub length: usize,
    pub hash: [u8; 32],
}

impl Header {
    /// The image's identity: its hash's first 16 bytes.
    #[must_use]
    pub fn id(&self) -> ImageId {
        let mut id = [0; 16];
        id.copy_from_slice(&self.hash[..16]);
        ImageId(id)
    }
}

fn word(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

/// The header of the image at the start of `panel` (the panel's bytes, its image first), if the
/// vector table points at one; not checked against the image ([`check`] does that).
#[must_use]
pub fn header(panel: &[u8]) -> Option<Header> {
    let length = usize::try_from(word(panel, LENGTH_WORD * 4)?).ok()?;
    if length == 0 || !length.is_multiple_of(ALIGN) || length.checked_add(HEADER)? > panel.len() {
        return None;
    }
    let header = &panel[length..length + HEADER];
    if header[..4] != MAGIC || usize::try_from(word(header, 8)?).ok()? != length {
        return None;
    }
    let mut hash = [0; 32];
    hash.copy_from_slice(&header[32..64]);
    Some(Header { version: word(header, 4)?, length, hash })
}

/// The header of the image at the start of `panel`, if the image is whole: its hash matches.
#[must_use]
pub fn check(panel: &[u8]) -> Option<Header> {
    let header = header(panel)?;
    (Sha256::digest(&panel[..header.length])[..] == header.hash).then_some(header)
}

/// How much [`check_yielding`] hashes between yields: some 2 ms of software SHA-256 on a
/// Cortex-M33 at 120 MHz.
pub const CHUNK: usize = 4096;

/// [`check`], for an application on an executor: the hash taken [`CHUNK`] bytes at a time with a
/// yield between, so other tasks run meanwhile (the whole image takes some 300 ms on the
/// PIC32CK, which has no hash engine outside the SG parts' HSM).
pub async fn check_yielding(panel: &[u8]) -> Option<Header> {
    let header = header(panel)?;
    let mut hasher = Sha256::new();
    for chunk in panel[..header.length].chunks(CHUNK) {
        hasher.update(chunk);
        yield_now().await;
    }
    (hasher.finalize()[..] == header.hash).then_some(header)
}

/// Let the executor run other tasks once: pending the first time it's polled, ready the next.
fn yield_now() -> impl Future<Output = ()> {
    let mut yielded = false;
    core::future::poll_fn(move |cx| {
        if yielded {
            core::task::Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            core::task::Poll::Pending
        }
    })
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use super::*;

    /// What the host tool does: `program` padded, its length in word 8, then the header.
    fn seal(program: &[u8], version: u32) -> Vec<u8> {
        let mut image = program.to_vec();
        image.resize(program.len().div_ceil(ALIGN) * ALIGN, 0);
        let length = u32::try_from(image.len()).unwrap();
        image[32..36].copy_from_slice(&length.to_le_bytes());
        let hash = Sha256::digest(&image);
        image.extend_from_slice(&MAGIC);
        image.extend_from_slice(&version.to_le_bytes());
        image.extend_from_slice(&length.to_le_bytes());
        image.extend_from_slice(&[0; 20]);
        image.extend_from_slice(&hash);
        image
    }

    /// A program of `n` bytes, a vector table's worth at least, as a panel of 4 KiB would hold
    /// it (the rest erased).
    fn panel(program: &[u8], version: u32) -> Vec<u8> {
        let mut panel = seal(program, version);
        panel.resize(4096, 0xFF);
        panel
    }

    fn program(n: usize) -> Vec<u8> {
        (0..n).map(|i| u8::try_from(i % 251).unwrap()).collect()
    }

    #[test]
    fn a_whole_image_checks() {
        let p = panel(&program(1000), 7);
        let h = check(&p).unwrap();
        assert_eq!((h.version, h.length), (7, 1024));
        assert_eq!(h.id().0, h.hash[..16]);
    }

    #[test]
    fn different_programs_have_different_ids() {
        let a = check(&panel(&program(1000), 1)).unwrap();
        let b = check(&panel(&program(1001), 1)).unwrap();
        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn a_damaged_image_doesnt() {
        let good = panel(&program(1000), 1);
        // A byte of the program, of the hash, the magic, the length in the header.
        for at in [500, 1024 + 40, 1024, 1024 + 8] {
            let mut p = good.clone();
            p[at] ^= 1;
            assert_eq!(check(&p), None, "byte {at}");
        }
        // The length word pointing elsewhere, or past the panel, or not aligned.
        for length in [992u32, 4096, 1000, 0] {
            let mut p = good.clone();
            p[32..36].copy_from_slice(&length.to_le_bytes());
            assert_eq!(check(&p), None, "length {length}");
        }
    }

    #[test]
    fn an_empty_panel_has_no_image() {
        assert_eq!(header(&vec![0xFF; 4096]), None);
        assert_eq!(header(&[0; 16]), None);
    }

    /// `future` run to its end, polled until ready: each poll counted.
    fn run<T>(future: impl Future<Output = T>) -> (T, usize) {
        let mut future = core::pin::pin!(future);
        let mut cx = core::task::Context::from_waker(core::task::Waker::noop());
        let mut polls = 0;
        loop {
            polls += 1;
            if let core::task::Poll::Ready(value) = future.as_mut().poll(&mut cx) {
                return (value, polls);
            }
        }
    }

    #[test]
    fn the_yielding_check_agrees_and_yields() {
        let mut p = seal(&program(3000), 2);
        p.resize(16384, 0xFF);
        let (header, polls) = run(check_yielding(&p));
        assert_eq!(header, check(&p));
        assert!(header.is_some());
        // 3008 bytes: one chunk, one yield.
        assert_eq!(polls, 2);
        let big = {
            let mut p = seal(&program(10_000), 2);
            p.resize(16384, 0xFF);
            p
        };
        let (header, polls) = run(check_yielding(&big));
        assert_eq!(header, check(&big));
        // 10,016 bytes: three chunks, three yields.
        assert_eq!(polls, 4);
        let mut bad = big.clone();
        bad[9000] ^= 1;
        assert_eq!(run(check_yielding(&bad)).0, None);
    }
}

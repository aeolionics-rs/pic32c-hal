// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The boot log's two pages on any [`NorFlash`]: pic32c-hal's `flash::Flash` on the part.
//!
//! [`NorPages`] puts the log's pages at two offsets of the flash, each an erase unit. A block is
//! written in as many of the flash's words as it takes (one on the PIC32CK, whose quad double
//! word is 32 bytes; two on the PIC32CM SG00's 16). The flash's own erase unit is the log's page,
//! so its block count is the erase unit over [`BLOCK`].
//!
//! On the PIC32CK, put one page at the top of each program flash panel. The bootloader maps
//! either panel first (`PFSWAP`), which swaps where the two pages appear, and the log doesn't
//! mind: which page is current comes from the records, not from where the page is. The updater
//! must then leave that page alone when it erases a panel for a new image. Writing the page in
//! the panel the code runs from stalls the CPU for the write (some 110 µs, a page erase at a flip
//! some 20 ms), unless the code is in the CMCC or runs from elsewhere.

use embedded_storage::nor_flash::NorFlash;

use crate::log::{BLOCK, Flash};

/// The log's two pages, at `pages` (offsets into the flash, each the start of an erase unit),
/// on `flash`.
pub struct NorPages<F> {
    flash: F,
    pages: [u32; 2],
}

impl<F: NorFlash> NorPages<F> {
    /// The pages at `pages` on `flash`, or `None` if they aren't each a whole erase unit inside
    /// the flash, or are the same one, or the flash's units don't suit the log.
    pub fn new(flash: F, pages: [u32; 2]) -> Option<Self> {
        let unit = F::ERASE_SIZE;
        let fits = |page: u32| (page as usize).is_multiple_of(unit) && (page as usize).checked_add(unit).is_some_and(|end| end <= flash.capacity());
        let suits = unit.is_multiple_of(BLOCK) && BLOCK.is_multiple_of(F::WRITE_SIZE) && F::READ_SIZE <= BLOCK && BLOCK.is_multiple_of(F::READ_SIZE);
        (suits && pages[0] != pages[1] && pages.iter().all(|&p| fits(p))).then_some(Self { flash, pages })
    }

    /// The flash, back.
    pub fn into_inner(self) -> F {
        self.flash
    }

    #[expect(clippy::cast_possible_truncation, reason = "a block inside an erase unit inside the flash, checked in new")]
    fn offset(&self, page: usize, block: usize) -> u32 {
        self.pages[page] + (block * BLOCK) as u32
    }
}

impl<F: NorFlash> Flash for NorPages<F> {
    type Error = F::Error;
    const BLOCKS: usize = F::ERASE_SIZE / BLOCK;

    fn read(&mut self, page: usize, block: usize) -> [u8; BLOCK] {
        let mut data = [0xFF; BLOCK];
        // Inside the flash (checked in new), so only a failing flash fails here, and a block it
        // can't read is one the log skips.
        if self.flash.read(self.offset(page, block), &mut data).is_err() {
            data = [0; BLOCK];
        }
        data
    }

    fn write(&mut self, page: usize, block: usize, data: &[u8; BLOCK]) -> Result<(), F::Error> {
        self.flash.write(self.offset(page, block), data)
    }

    fn erase(&mut self, page: usize) -> Result<(), F::Error> {
        let from = self.pages[page];
        #[expect(clippy::cast_possible_truncation, reason = "an erase unit inside the flash, checked in new")]
        let to = from + F::ERASE_SIZE as u32;
        self.flash.erase(from, to)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use embedded_storage::nor_flash::{ErrorType, NorFlashError, NorFlashErrorKind, ReadNorFlash};

    use super::*;
    use crate::decide::{ImageId, Record};
    use crate::log::Log;

    /// NOR flash in RAM with `W`-byte words and 256-byte erase units, refusing a word written
    /// twice as flash with ECC would.
    struct Ram<const W: usize> {
        bytes: Vec<u8>,
        written: Vec<bool>,
    }

    impl<const W: usize> Ram<W> {
        fn new(size: usize) -> Self {
            Self {
                bytes: vec![0xFF; size],
                written: vec![false; size / W],
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    enum RamError {
        Twice,
        Bounds,
    }

    impl NorFlashError for RamError {
        fn kind(&self) -> NorFlashErrorKind {
            match self {
                RamError::Twice => NorFlashErrorKind::Other,
                RamError::Bounds => NorFlashErrorKind::OutOfBounds,
            }
        }
    }

    impl<const W: usize> ErrorType for Ram<W> {
        type Error = RamError;
    }

    impl<const W: usize> ReadNorFlash for Ram<W> {
        const READ_SIZE: usize = 1;
        fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), RamError> {
            let at = offset as usize;
            bytes.copy_from_slice(self.bytes.get(at..at + bytes.len()).ok_or(RamError::Bounds)?);
            Ok(())
        }
        fn capacity(&self) -> usize {
            self.bytes.len()
        }
    }

    impl<const W: usize> NorFlash for Ram<W> {
        const WRITE_SIZE: usize = W;
        const ERASE_SIZE: usize = 256;
        fn erase(&mut self, from: u32, to: u32) -> Result<(), RamError> {
            let (from, to) = (from as usize, to as usize);
            self.bytes[from..to].fill(0xFF);
            self.written[from / W..to / W].fill(false);
            Ok(())
        }
        fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), RamError> {
            let at = offset as usize;
            for word in at / W..(at + bytes.len()) / W {
                if core::mem::replace(&mut self.written[word], true) {
                    return Err(RamError::Twice);
                }
            }
            self.bytes[at..at + bytes.len()].copy_from_slice(bytes);
            Ok(())
        }
    }

    fn id(n: u8) -> ImageId {
        ImageId([n; 16])
    }

    #[test]
    fn checks_the_pages() {
        assert!(NorPages::new(Ram::<32>::new(1024), [0, 768]).is_some());
        // Not at an erase unit, the same one twice, past the end.
        assert!(NorPages::new(Ram::<32>::new(1024), [16, 768]).is_none());
        assert!(NorPages::new(Ram::<32>::new(1024), [256, 256]).is_none());
        assert!(NorPages::new(Ram::<32>::new(1024), [0, 1024]).is_none());
        // Words bigger than a block don't suit the log.
        assert!(NorPages::new(Ram::<64>::new(1024), [0, 768]).is_none());
    }

    /// The log on NOR flash with 32- and 16-byte words (the PIC32CK's and the SG00's), its
    /// pages apart in the flash: three flips' worth of records, read back in order.
    #[test]
    fn a_log_on_nor_flash() {
        fn run<const W: usize>() {
            let pages = NorPages::new(Ram::<W>::new(2048), [256, 1792]).unwrap();
            assert_eq!(NorPages::<Ram<W>>::BLOCKS, 8);
            let mut log = Log::open(pages);
            let history: Vec<Record> = (0..20).map(|n| Record::Confirmed(id(n))).collect();
            for r in &history {
                log.append(*r).unwrap();
            }
            let mut log = Log::open(log.into_flash());
            let read: Vec<Record> = log.newest_first().collect();
            // The current page and the one before: 20 records, 8 a page, so the newest 12.
            let newest: Vec<Record> = history.iter().rev().take(12).copied().collect();
            assert_eq!(read, newest, "{W}-byte words");
            // Nothing outside the two pages was touched.
            let ram = log.into_flash().into_inner();
            assert!(ram.bytes[..256].iter().chain(&ram.bytes[512..1792]).all(|&b| b == 0xFF));
        }
        run::<32>();
        run::<16>();
    }
}

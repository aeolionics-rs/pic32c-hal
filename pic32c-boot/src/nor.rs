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
use embedded_storage_async::nor_flash::NorFlash as AsyncNorFlash;

use crate::log::{AsyncFlash, BLOCK, Flash};

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
        layout(F::ERASE_SIZE, F::WRITE_SIZE, F::READ_SIZE, flash.capacity(), pages).then_some(Self { flash, pages })
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

/// Whether `pages` suit the log on a flash of `capacity` bytes, erased in units of `unit`,
/// written in words of `write` and read in `read`: each page a whole unit inside the flash, the
/// two apart, and a block whole units of writing and reading.
fn layout(unit: usize, write: usize, read: usize, capacity: usize, pages: [u32; 2]) -> bool {
    let fits = |page: u32| (page as usize).is_multiple_of(unit) && (page as usize).checked_add(unit).is_some_and(|end| end <= capacity);
    let suits = unit.is_multiple_of(BLOCK) && BLOCK.is_multiple_of(write) && read <= BLOCK && BLOCK.is_multiple_of(read);
    suits && pages[0] != pages[1] && pages.iter().all(|&p| fits(p))
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

/// [`NorPages`] on an async NOR flash (`embedded-storage-async`), for [`crate::log::AsyncLog`]:
/// pic32c-hal's `flash::Flash<Async>`, which waits on the controller's interrupt.
pub struct AsyncNorPages<F> {
    flash: F,
    pages: [u32; 2],
}

impl<F: AsyncNorFlash> AsyncNorPages<F> {
    /// As [`NorPages::new`].
    pub fn new(flash: F, pages: [u32; 2]) -> Option<Self> {
        layout(F::ERASE_SIZE, F::WRITE_SIZE, F::READ_SIZE, flash.capacity(), pages).then_some(Self { flash, pages })
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

impl<F: AsyncNorFlash> AsyncFlash for AsyncNorPages<F> {
    type Error = F::Error;
    const BLOCKS: usize = F::ERASE_SIZE / BLOCK;

    async fn read(&mut self, page: usize, block: usize) -> [u8; BLOCK] {
        let mut data = [0xFF; BLOCK];
        // As NorPages: a block the flash can't read is one the log skips.
        if self.flash.read(self.offset(page, block), &mut data).await.is_err() {
            data = [0; BLOCK];
        }
        data
    }

    async fn write(&mut self, page: usize, block: usize, data: &[u8; BLOCK]) -> Result<(), F::Error> {
        self.flash.write(self.offset(page, block), data).await
    }

    async fn erase(&mut self, page: usize) -> Result<(), F::Error> {
        let from = self.pages[page];
        #[expect(clippy::cast_possible_truncation, reason = "an erase unit inside the flash, checked in new")]
        let to = from + F::ERASE_SIZE as u32;
        self.flash.erase(from, to).await
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
    use crate::log::{AsyncLog, Log};

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
            for written in &mut self.written[at / W..(at + bytes.len()) / W] {
                if core::mem::replace(written, true) {
                    return Err(RamError::Twice);
                }
            }
            self.bytes[at..at + bytes.len()].copy_from_slice(bytes);
            Ok(())
        }
    }

    #[expect(clippy::unused_async_trait_impl, reason = "the async form of a flash that never waits")]
    impl<const W: usize> embedded_storage_async::nor_flash::ReadNorFlash for Ram<W> {
        const READ_SIZE: usize = 1;
        async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), RamError> {
            ReadNorFlash::read(self, offset, bytes)
        }
        fn capacity(&self) -> usize {
            self.bytes.len()
        }
    }

    #[expect(clippy::unused_async_trait_impl, reason = "the async form of a flash that never waits")]
    impl<const W: usize> AsyncNorFlash for Ram<W> {
        const WRITE_SIZE: usize = W;
        const ERASE_SIZE: usize = 256;
        async fn erase(&mut self, from: u32, to: u32) -> Result<(), RamError> {
            NorFlash::erase(self, from, to)
        }
        async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), RamError> {
            NorFlash::write(self, offset, bytes)
        }
    }

    /// `future` run to its end: the RAM flash is never pending.
    fn block_on<T>(future: impl Future<Output = T>) -> T {
        let mut future = core::pin::pin!(future);
        let mut cx = core::task::Context::from_waker(core::task::Waker::noop());
        match future.as_mut().poll(&mut cx) {
            core::task::Poll::Ready(value) => value,
            core::task::Poll::Pending => panic!("the RAM flash never waits"),
        }
    }

    /// The log through both adapters on one NOR flash, appends taking turns over two flips: what
    /// the blocking log reads, the async one finds, in order.
    #[test]
    fn blocking_and_async_pages_are_one_log() {
        fn run<const W: usize>() {
            let mut ram = Ram::<W>::new(2048);
            let mut history: Vec<Record> = Vec::new();
            for n in 0..20u8 {
                let record = Record::Confirmed(id(n));
                if n % 2 == 0 {
                    let mut log = Log::open(NorPages::new(&mut ram, [256, 1792]).unwrap());
                    log.append(record).unwrap();
                } else {
                    block_on(async {
                        let mut log = AsyncLog::open(AsyncNorPages::new(&mut ram, [256, 1792]).unwrap()).await;
                        log.append(record).await.unwrap();
                    });
                }
                history.push(record);
            }
            let blocking: Vec<Record> = Log::open(NorPages::new(&mut ram, [256, 1792]).unwrap()).newest_first().collect();
            let mut found = Vec::new();
            block_on(async {
                let mut log = AsyncLog::open(AsyncNorPages::new(&mut ram, [256, 1792]).unwrap()).await;
                log.find(|r| {
                    found.push(r);
                    false
                })
                .await;
            });
            assert_eq!(found, blocking, "{W}-byte words");
            assert_eq!(blocking, history.iter().rev().take(12).copied().collect::<Vec<_>>(), "{W}-byte words");
        }
        run::<32>();
        run::<16>();
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

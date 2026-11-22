// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Program flash erase and write through the GV00's Non-Volatile Memory
//! Controller (NVMCTRL, the SAM D20's, DS60001881 chapter 18), as an
//! [`embedded_storage`] [`NorFlash`]: the same API as the other chips'
//! `flash`, over another IP.
//!
//! Offsets count from the start of flash (address 0, where the program
//! is). Flash is one array, so every read from it, code included, stalls
//! while an operation runs.
//!
//! - Erase is by 256-byte row ([`ROW_SIZE`], four pages).
//! - Writes are in 32-bit words ([`WORD_SIZE`]), each written once after an
//!   erase. Stores to flash fill the 64-byte page buffer ([`PAGE_SIZE`]);
//!   each page touched is then written with a Write Page command
//!   ([`crate::init`] sets CTRLB.MANW, so no page is written by a store
//!   alone).
//!   The buffer is cleared first, so the rest of the page is written as
//!   `0xFF`, which leaves what's there. A row takes at most 8 page writes
//!   between erases (the NVM characteristics' note): more than eight
//!   separate writes into one row need an erase between.
//! - Rows in a locked region (the 16 region lock bits, the boot section)
//!   aren't changed: [`Error::Failed`] with `LOCKE`.
//! - After each erase or write the NVM cache is invalidated (INVALL), so
//!   reads see what's now there.
//!
//! [`Flash::new_blocking`] waits on the NVMCTRL by polling. [`Flash::new`]
//! (async, as an [`embedded_storage_async`] `NorFlash` too) waits on its
//! READY interrupt instead (bind `NVMCTRL` to [`InterruptHandler`]); with
//! one array, code must run from SRAM for anything to run meanwhile.
//!
//! An async operation dropped part way finishes before the drop returns.

use core::future::poll_fn;
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;
use embedded_storage::nor_flash::{ErrorType, NorFlash, NorFlashError, NorFlashErrorKind, ReadNorFlash};

use crate::interrupt::InterruptExt;
use crate::interrupt::typelevel::{self, Binding, Handler};
pub use crate::mode::{Async, Blocking};
use crate::{Peri, peripherals};

/// Where flash starts in the memory map.
pub const PFM_BASE: u32 = 0;
/// The erase unit: a row, four pages.
pub const ROW_SIZE: usize = 256;
/// The write unit of the NVM, and the size of its page buffer.
pub const PAGE_SIZE: usize = 64;
/// The write unit here: one 32-bit word (byte stores to the page buffer
/// are bus errors).
pub const WORD_SIZE: usize = 4;
/// Flash's one array.
pub const PANEL_SIZE: usize = crate::FLASH_SIZE;

const NVMCTRL: usize = 0x4100_4000;
const CTRLA: usize = NVMCTRL;
const INTENCLR: usize = NVMCTRL + 0x0C;
const INTENSET: usize = NVMCTRL + 0x10;
const INTFLAG: usize = NVMCTRL + 0x14;
const STATUS: usize = NVMCTRL + 0x18;
const ADDR: usize = NVMCTRL + 0x1C;

/// CTRLA.CMDEX: the key that has CTRLA.CMD taken.
const CMDEX: u16 = 0xA5 << 8;
const READY: u8 = 1 << 0;
const ERROR: u8 = 1 << 1;
/// PROGE (a command refused), LOCKE (a locked region) and NVME (the NVM
/// reported an error).
const ERRORS: u16 = 0b1_1100;

/// CTRLA.CMD values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    /// Erase the row ADDR is in.
    EraseRow = 0x02,
    /// Write the page buffer to the page ADDR is in.
    WritePage = 0x04,
    /// Set the page buffer to all ones.
    PageBufferClear = 0x44,
    /// Invalidate the NVM cache.
    InvalidateCache = 0x46,
}

/// What went wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// An offset or length isn't a multiple of the operation's unit.
    NotAligned,
    /// The range runs past the flash.
    OutOfBounds,
    /// Never on the GV00 (the other chips' write protection, locked); kept
    /// so code can match on one `Error` across chips.
    Busy,
    /// The NVMCTRL reported errors: its STATUS bits (`PROGE` 0x4 for a
    /// refused command, `LOCKE` 0x8 for a locked region, `NVME` 0x10).
    Failed(u32),
}

impl NorFlashError for Error {
    fn kind(&self) -> NorFlashErrorKind {
        match self {
            Self::NotAligned => NorFlashErrorKind::NotAligned,
            Self::OutOfBounds => NorFlashErrorKind::OutOfBounds,
            Self::Busy | Self::Failed(_) => NorFlashErrorKind::Other,
        }
    }
}

fn check(offset: u32, length: usize, unit: usize, capacity: usize) -> Result<(), Error> {
    let offset = offset as usize;
    if !offset.is_multiple_of(unit) || !length.is_multiple_of(unit) {
        return Err(Error::NotAligned);
    }
    if offset.checked_add(length).is_none_or(|end| end > capacity) {
        return Err(Error::OutOfBounds);
    }
    Ok(())
}

fn read8(address: usize) -> u8 {
    // SAFETY: an NVMCTRL register, read at its width.
    unsafe { read_volatile(address as *const u8) }
}

fn write8(address: usize, value: u8) {
    // SAFETY: an NVMCTRL register; only this driver writes them.
    unsafe { write_volatile(address as *mut u8, value) }
}

fn read16(address: usize) -> u16 {
    // SAFETY: as for `read8`.
    unsafe { read_volatile(address as *const u16) }
}

fn write16(address: usize, value: u16) {
    // SAFETY: as for `write8`.
    unsafe { write_volatile(address as *mut u16, value) }
}

fn ready() -> bool {
    read8(INTFLAG) & READY != 0
}

static WAKER: AtomicWaker = AtomicWaker::new();

/// The NVMCTRL's handler for async operations: bind `NVMCTRL` to it with
/// [`bind_interrupts!`](crate::bind_interrupts).
pub struct InterruptHandler {
    _private: (),
}

impl Handler<typelevel::NVMCTRL> for InterruptHandler {
    unsafe fn on_interrupt() {
        // READY stays set while idle: masked until the next wait.
        write8(INTENCLR, READY);
        WAKER.wake();
    }
}

/// Flash, through the NVMCTRL: [`Blocking`] waits on each operation by
/// polling, [`Async`] on its READY interrupt.
pub struct Flash<'d, M = Blocking> {
    capacity: usize,
    _nvmctrl: Peri<'d, peripherals::NVMCTRL>,
    _mode: PhantomData<M>,
}

impl<'d> Flash<'d, Blocking> {
    /// The chip's flash ([`FLASH_SIZE`](crate::FLASH_SIZE) bytes). Erasing
    /// or writing the code that runs, or data in use, is the caller's to
    /// avoid.
    #[must_use]
    pub fn new_blocking(nvmctrl: Peri<'d, peripherals::NVMCTRL>) -> Self {
        Self {
            capacity: crate::FLASH_SIZE,
            _nvmctrl: nvmctrl,
            _mode: PhantomData,
        }
    }
}

impl<'d> Flash<'d, Async> {
    /// The chip's flash, waiting on the NVMCTRL's interrupt (`NVMCTRL`,
    /// bound to [`InterruptHandler`]). Erasing or writing the code that
    /// runs, or data in use, is the caller's to avoid.
    #[must_use]
    pub fn new(nvmctrl: Peri<'d, peripherals::NVMCTRL>, _irqs: impl Binding<typelevel::NVMCTRL, InterruptHandler>) -> Self {
        write8(INTENCLR, READY | ERROR);
        let irq = crate::interrupt::NVMCTRL;
        irq.unpend();
        // SAFETY: the handler only masks the NVMCTRL's interrupt and wakes.
        unsafe { irq.enable() };
        Self {
            capacity: crate::FLASH_SIZE,
            _nvmctrl: nvmctrl,
            _mode: PhantomData,
        }
    }
}

/// Starts `command` on the row or page at `address` (ADDR counts 16-bit
/// halfwords), after waiting out anything still running (a dropped
/// future's) and clearing the last errors.
fn start(command: Command, address: u32) {
    while !ready() {}
    write16(STATUS, ERRORS);
    write8(INTFLAG, ERROR);
    // SAFETY: ADDR, a word register.
    unsafe { write_volatile(ADDR as *mut u32, address / 2) };
    write16(CTRLA, CMDEX | command as u16);
}

/// The errors of the command that just ran.
fn errors() -> Result<(), Error> {
    let status = read16(STATUS) & ERRORS;
    if status == 0 { Ok(()) } else { Err(Error::Failed(u32::from(status))) }
}

/// Runs a command that needs no wait worth an interrupt (clearing the page
/// buffer, invalidating the cache), and waits for it.
fn quick(command: Command) {
    start(command, 0);
    while !ready() {}
}

/// Clears the page buffer and stores the words of `bytes`, all in the page
/// at `page` (an address), into it.
fn fill_page(offset: u32, bytes: &[u8]) {
    quick(Command::PageBufferClear);
    for (address, word) in words(offset, bytes) {
        // SAFETY: a word-aligned address in flash (checked by the caller):
        // the store loads the page buffer; nothing is programmed until
        // Write Page (`init` set MANW).
        unsafe { write_volatile(address as *mut u32, word) };
    }
}

/// The parts of `bytes` at `offset`, page by page: each part's offset and
/// bytes.
fn pages(offset: u32, bytes: &[u8]) -> impl Iterator<Item = (u32, &[u8])> + '_ {
    let mut at = 0usize;
    core::iter::from_fn(move || {
        if at == bytes.len() {
            return None;
        }
        #[expect(clippy::cast_possible_truncation, reason = "offsets inside 32 KB of flash")]
        let start = offset + at as u32;
        let room = PAGE_SIZE - start as usize % PAGE_SIZE;
        let n = room.min(bytes.len() - at);
        let part = (start, &bytes[at..at + n]);
        at += n;
        Some(part)
    })
}

/// Waits for READY on the interrupt.
async fn wait_ready() {
    poll_fn(|cx| {
        WAKER.register(cx.waker());
        if ready() {
            return Poll::Ready(());
        }
        write8(INTENSET, READY);
        // It may have ended between the check and the unmasking.
        if ready() { Poll::Ready(()) } else { Poll::Pending }
    })
    .await;
    write8(INTENCLR, READY);
}

fn read_into(offset: u32, bytes: &mut [u8]) {
    let start = (PFM_BASE + offset) as *const u8;
    for (i, byte) in bytes.iter_mut().enumerate() {
        // SAFETY: inside flash, checked by the caller.
        *byte = unsafe { read_volatile(start.add(i)) };
    }
}

/// The words of `bytes`, little-endian, with their addresses from `offset`.
fn words(offset: u32, bytes: &[u8]) -> impl Iterator<Item = (u32, u32)> + '_ {
    (PFM_BASE + offset..).step_by(WORD_SIZE).zip(bytes.as_chunks::<4>().0.iter().map(|w| u32::from_le_bytes(*w)))
}

impl<M> ErrorType for Flash<'_, M> {
    type Error = Error;
}

impl ReadNorFlash for Flash<'_, Blocking> {
    const READ_SIZE: usize = 1;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Error> {
        check(offset, bytes.len(), 1, self.capacity)?;
        read_into(offset, bytes);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl NorFlash for Flash<'_, Blocking> {
    const WRITE_SIZE: usize = WORD_SIZE;
    const ERASE_SIZE: usize = ROW_SIZE;

    fn erase(&mut self, from: u32, to: u32) -> Result<(), Error> {
        let length = to.checked_sub(from).ok_or(Error::OutOfBounds)? as usize;
        check(from, length, ROW_SIZE, self.capacity)?;
        let result = (from..to).step_by(ROW_SIZE).try_for_each(|row| {
            start(Command::EraseRow, PFM_BASE + row);
            while !ready() {}
            errors()
        });
        quick(Command::InvalidateCache);
        result
    }

    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Error> {
        check(offset, bytes.len(), WORD_SIZE, self.capacity)?;
        let result = pages(offset, bytes).try_for_each(|(at, part)| {
            fill_page(at, part);
            start(Command::WritePage, PFM_BASE + at);
            while !ready() {}
            errors()
        });
        quick(Command::InvalidateCache);
        result
    }
}

impl embedded_storage_async::nor_flash::ReadNorFlash for Flash<'_, Async> {
    const READ_SIZE: usize = 1;

    #[expect(clippy::unused_async_trait_impl, reason = "the trait's; reads are plain memory reads")]
    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Error> {
        check(offset, bytes.len(), 1, self.capacity)?;
        read_into(offset, bytes);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

/// Invalidates the cache when dropped: after an async operation, finished
/// or dropped part way (the drop waits for the command under way).
struct Invalidate;

impl Drop for Invalidate {
    fn drop(&mut self) {
        quick(Command::InvalidateCache);
    }
}

impl embedded_storage_async::nor_flash::NorFlash for Flash<'_, Async> {
    const WRITE_SIZE: usize = WORD_SIZE;
    const ERASE_SIZE: usize = ROW_SIZE;

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Error> {
        let length = to.checked_sub(from).ok_or(Error::OutOfBounds)? as usize;
        check(from, length, ROW_SIZE, self.capacity)?;
        let _invalidate = Invalidate;
        for row in (from..to).step_by(ROW_SIZE) {
            start(Command::EraseRow, PFM_BASE + row);
            wait_ready().await;
            errors()?;
        }
        Ok(())
    }

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Error> {
        check(offset, bytes.len(), WORD_SIZE, self.capacity)?;
        let _invalidate = Invalidate;
        for (at, part) in pages(offset, bytes) {
            fill_page(at, part);
            start(Command::WritePage, PFM_BASE + at);
            wait_ready().await;
            errors()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_alignment_and_bounds() {
        let capacity = 32 << 10;
        assert_eq!(check(0, 4, WORD_SIZE, capacity), Ok(()));
        assert_eq!(check(2, 4, WORD_SIZE, capacity), Err(Error::NotAligned));
        assert_eq!(check(256, 512, ROW_SIZE, capacity), Ok(()));
        assert_eq!(check(64, 256, ROW_SIZE, capacity), Err(Error::NotAligned));
        assert_eq!(check(31 << 10, 2048, ROW_SIZE, capacity), Err(Error::OutOfBounds));
    }

    #[test]
    fn splits_by_page() {
        let bytes = [0u8; 100];
        let parts: Vec<_> = pages(56, &bytes).map(|(at, part)| (at, part.len())).collect();
        assert_eq!(parts, [(56, 8), (64, 64), (128, 28)]);
        let parts: Vec<_> = pages(128, &bytes[..64]).map(|(at, part)| (at, part.len())).collect();
        assert_eq!(parts, [(128, 64)]);
        assert_eq!(pages(0, &[]).count(), 0);
    }

    #[test]
    fn words_are_little_endian() {
        let got: Vec<_> = words(8, &[1, 2, 3, 4, 5, 6, 7, 8]).collect();
        assert_eq!(got, [(PFM_BASE + 8, 0x0403_0201), (PFM_BASE + 12, 0x0807_0605)]);
    }
}

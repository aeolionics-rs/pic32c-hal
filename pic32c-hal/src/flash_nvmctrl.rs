// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Program flash (PFM) erase and write through the PL10's Non-Volatile
//! Memory Controller (NVMCTRL, DS40002667 chapter 26), as an
//! [`embedded_storage`] [`NorFlash`]: the same API as the FCW's driver on
//! the PIC32CK and the SG00, over another IP.
//!
//! Offsets count from the start of PFM (`0x0C00_0000`). PFM is one panel,
//! so every read from it, code included, stalls while an operation runs.
//!
//! - Erase is by 512-byte page ([`PAGE_SIZE`]).
//! - Writes are in 32-bit words ([`WORD_SIZE`]), each written once after an
//!   erase: a command (CTRLB) enables writes, then each word stored to the
//!   flash's address is programmed.
//! - If the NVMCTRL's registers are write-protected (WPCTRL.WPEN), each
//!   operation lifts the protection and puts it back; locked protection
//!   (WPLCK) can't be lifted, and the operation fails.
//! - Pages in a locked region (the 16 region lock bits, the boot section)
//!   aren't changed: [`Error::Failed`] with `LOCKE`.
//!
//! [`Flash::new_blocking`] waits on the NVMCTRL by polling. [`Flash::new`]
//! (async, as an [`embedded_storage_async`] `NorFlash` too) waits on its
//! READY interrupt instead (bind `NVMCTRL` to [`InterruptHandler`]); with
//! one panel, code must run from SRAM for anything to run meanwhile.
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

/// Where PFM starts in the memory map.
pub const PFM_BASE: u32 = 0x0C00_0000;
/// The erase unit.
pub const PAGE_SIZE: usize = 512;
/// The write unit: one 32-bit word (byte and halfword stores to flash are
/// bus errors).
pub const WORD_SIZE: usize = 4;
/// PFM's one panel.
pub const PANEL_SIZE: usize = crate::FLASH_SIZE;

const NVMCTRL: usize = 0x4100_4000;
const CTRLB: usize = NVMCTRL + 0x04;
const INTENCLR: usize = NVMCTRL + 0x0C;
const INTENSET: usize = NVMCTRL + 0x10;
const INTFLAG: usize = NVMCTRL + 0x14;
const STATUS: usize = NVMCTRL + 0x1C;
const WPCTRL: usize = NVMCTRL + 0x2C;

/// CTRLB.CMDEX: the key that has CTRLB.CMD taken.
const CMDEX: u32 = 0xA5 << 8;
/// WPCTRL.WPKEY ("NVM"), WPEN and WPLCK.
const WPKEY: u32 = 0x004E_564D << 8;
const WPEN: u32 = 1 << 0;
const WPLCK: u32 = 1 << 1;
const READY: u32 = 1 << 0;
const ERROR: u32 = 1 << 1;
/// PROGE (a command refused, or a write while busy) and LOCKE (a locked
/// region).
const ERRORS: u32 = 0b1100;

/// CTRLB.CMD values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    /// No command: leaves the write or erase mode.
    None = 0x00,
    /// Each word stored to the flash is programmed.
    Write = 0x02,
    /// A store to the flash erases its page.
    PageErase = 0x08,
}

/// What went wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// An offset or length isn't a multiple of the operation's unit.
    NotAligned,
    /// The range runs past the flash.
    OutOfBounds,
    /// The NVMCTRL's registers are write-protected and locked (WPLCK).
    Busy,
    /// The NVMCTRL reported errors: its STATUS bits (`PROGE` 0x4 for a
    /// refused command, `LOCKE` 0x8 for a locked region).
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

fn read(address: usize) -> u32 {
    // SAFETY: an NVMCTRL register, read whole.
    unsafe { read_volatile(address as *const u32) }
}

fn write(address: usize, value: u32) {
    // SAFETY: an NVMCTRL register; only this driver writes them.
    unsafe { write_volatile(address as *mut u32, value) }
}

fn ready() -> bool {
    read(INTFLAG) & READY != 0
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
        write(INTENCLR, READY);
        WAKER.wake();
    }
}

/// PFM, through the NVMCTRL: [`Blocking`] waits on each operation by
/// polling, [`Async`] on its READY interrupt.
pub struct Flash<'d, M = Blocking> {
    capacity: usize,
    _nvmctrl: Peri<'d, peripherals::NVMCTRL>,
    _mode: PhantomData<M>,
}

impl<'d> Flash<'d, Blocking> {
    /// The chip's PFM ([`FLASH_SIZE`](crate::FLASH_SIZE) bytes). Erasing or
    /// writing the code that runs, or data in use, is the caller's to avoid.
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
    /// The chip's PFM, waiting on the NVMCTRL's interrupt (`NVMCTRL`, bound
    /// to [`InterruptHandler`]). Erasing or writing the code that runs, or
    /// data in use, is the caller's to avoid.
    #[must_use]
    pub fn new(nvmctrl: Peri<'d, peripherals::NVMCTRL>, _irqs: impl Binding<typelevel::NVMCTRL, InterruptHandler>) -> Self {
        write(INTENCLR, READY | ERROR);
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

/// An erase or a run of writes under way: CTRLB holds its command, and the
/// registers' write protection, if it was on, is lifted until it ends.
struct Session {
    protected: bool,
}

impl Session {
    /// Waits out anything still running (a dropped future's), lifts the
    /// write protection and sets `command`.
    fn begin(command: Command) -> Result<Self, Error> {
        while !ready() {}
        let wpctrl = read(WPCTRL);
        if wpctrl & WPLCK != 0 && wpctrl & WPEN != 0 {
            return Err(Error::Busy);
        }
        let protected = wpctrl & WPEN != 0;
        if protected {
            write(WPCTRL, WPKEY);
        }
        write(STATUS, ERRORS);
        write(INTFLAG, ERROR);
        write(CTRLB, CMDEX | command as u32);
        Ok(Self { protected })
    }

    /// Stores `word` at `address`, starting the operation the command set.
    fn store(address: u32, word: u32) {
        // SAFETY: a word-aligned address in PFM (checked by the caller);
        // with a write or erase command set, the store programs or erases.
        unsafe { write_volatile(address as *mut u32, word) };
    }

    /// The errors of what has run so far.
    fn errors() -> Result<(), Error> {
        let status = read(STATUS) & ERRORS;
        if status == 0 { Ok(()) } else { Err(Error::Failed(status)) }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        while !ready() {}
        write(CTRLB, CMDEX | Command::None as u32);
        if self.protected {
            write(WPCTRL, WPKEY | WPEN);
        }
    }
}

/// Waits for READY on the interrupt.
async fn wait_ready() {
    poll_fn(|cx| {
        WAKER.register(cx.waker());
        if ready() {
            return Poll::Ready(());
        }
        write(INTENSET, READY);
        // It may have ended between the check and the unmasking.
        if ready() { Poll::Ready(()) } else { Poll::Pending }
    })
    .await;
    write(INTENCLR, READY);
}

fn read_into(offset: u32, bytes: &mut [u8]) {
    let start = (PFM_BASE + offset) as *const u8;
    for (i, byte) in bytes.iter_mut().enumerate() {
        // SAFETY: inside PFM, checked by the caller.
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
    const ERASE_SIZE: usize = PAGE_SIZE;

    fn erase(&mut self, from: u32, to: u32) -> Result<(), Error> {
        let length = to.checked_sub(from).ok_or(Error::OutOfBounds)? as usize;
        check(from, length, PAGE_SIZE, self.capacity)?;
        let _session = Session::begin(Command::PageErase)?;
        for page in (from..to).step_by(PAGE_SIZE) {
            Session::store(PFM_BASE + page, 0xFFFF_FFFF);
            while !ready() {}
            Session::errors()?;
        }
        Ok(())
    }

    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Error> {
        check(offset, bytes.len(), WORD_SIZE, self.capacity)?;
        let _session = Session::begin(Command::Write)?;
        for (address, word) in words(offset, bytes) {
            Session::store(address, word);
            while !ready() {}
            Session::errors()?;
        }
        Ok(())
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

impl embedded_storage_async::nor_flash::NorFlash for Flash<'_, Async> {
    const WRITE_SIZE: usize = WORD_SIZE;
    const ERASE_SIZE: usize = PAGE_SIZE;

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Error> {
        let length = to.checked_sub(from).ok_or(Error::OutOfBounds)? as usize;
        check(from, length, PAGE_SIZE, self.capacity)?;
        let _session = Session::begin(Command::PageErase)?;
        for page in (from..to).step_by(PAGE_SIZE) {
            Session::store(PFM_BASE + page, 0xFFFF_FFFF);
            wait_ready().await;
            Session::errors()?;
        }
        Ok(())
    }

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Error> {
        check(offset, bytes.len(), WORD_SIZE, self.capacity)?;
        let _session = Session::begin(Command::Write)?;
        for (address, word) in words(offset, bytes) {
            Session::store(address, word);
            wait_ready().await;
            Session::errors()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_alignment_and_bounds() {
        let capacity = 64 << 10;
        assert_eq!(check(0, 4, WORD_SIZE, capacity), Ok(()));
        assert_eq!(check(2, 4, WORD_SIZE, capacity), Err(Error::NotAligned));
        assert_eq!(check(0, 3, WORD_SIZE, capacity), Err(Error::NotAligned));
        assert_eq!(check(512, 1024, PAGE_SIZE, capacity), Ok(()));
        assert_eq!(check(256, 512, PAGE_SIZE, capacity), Err(Error::NotAligned));
        assert_eq!(check(63 << 10, 2048, PAGE_SIZE, capacity), Err(Error::OutOfBounds));
    }

    #[test]
    fn words_are_little_endian() {
        let got: Vec<_> = words(8, &[1, 2, 3, 4, 5, 6, 7, 8]).collect();
        assert_eq!(got, [(PFM_BASE + 8, 0x0403_0201), (PFM_BASE + 12, 0x0807_0605)]);
    }
}

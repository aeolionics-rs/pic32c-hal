// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Program flash (PFM) erase and write on Microchip's PIC32CK, through its
//! Flash Controller, Write (FCW; DS60001795 chapter 31.2), as an
//! [`embedded_storage`] [`NorFlash`].
//!
//! Offsets count from the start of PFM (`0x0C00_0000`). PFM is two panels of
//! half the capacity each; while one is erased or written, reads from it
//! (code included) stall until the operation ends, and reads from the other
//! carry on ("live update").
//!
//! - Erase is by 4 KiB page.
//! - Writes are in 32-byte flash words ("quad writes"), each written once
//!   after an erase: the FCW stores ECC with each, whatever the ECC mode.
//!   Single 8-byte writes, which store only parity and which the ECC-on mode
//!   refuses, aren't offered. A whole 1 KiB row whose data is in SRAM goes in
//!   one row write, which the FCW reads from SRAM itself.
//! - The FCW is shared with the HSM through a hardware mutex; an operation
//!   that finds the HSM holding it fails with [`Error::Busy`].
//! - After each operation the CMCC (the L1 cache in front of flash), if on,
//!   is invalidated, so reads see the new content.
//!
//! [`Flash::new_blocking`] waits on the FCW by polling. [`Flash::new`]
//! (async, as an [`embedded_storage_async`] `NorFlash` too) waits on its
//! interrupt instead (bind `FCW` to [`InterruptHandler`]), so other tasks
//! run meanwhile, as long as what they run isn't in the panel being changed:
//! erase or write the panel the program doesn't run from (with the program
//! in panel 1, panel 2, offsets from [`PANEL_SIZE`]), and keep the interrupt
//! handlers there too, or the CPU stalls on fetches (the CMCC hides that
//! only for code it holds). A page erase takes up to 20 ms (`NVM_11`), a
//! double word up to 20 µs (`NVM_7`).
//!
//! An async operation dropped part way finishes before the drop returns (a
//! row write reads its data from the caller's buffer while it runs).
//!
//! PFM write protection (PWP) is the application's to set; it is off out of
//! reset.

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
pub const PAGE_SIZE: usize = 4096;
/// The largest single write, from SRAM.
pub const ROW_SIZE: usize = 1024;
/// The write unit: one flash word.
pub const WORD_SIZE: usize = 32;
/// Each of PFM's two panels: offsets from here are in panel 2.
pub const PANEL_SIZE: usize = crate::FLASH_SIZE / 2;

const FCW: usize = 0x4400_4000;
const CTRLA: usize = FCW;
const MUTEX: usize = FCW + 0x08;
const INTENCLR: usize = FCW + 0x0C;
const INTENSET: usize = FCW + 0x10;
const INTFLAG: usize = FCW + 0x14;
const STATUS: usize = FCW + 0x18;
const KEY: usize = FCW + 0x1C;
const ADDR: usize = FCW + 0x20;
const SRCADDR: usize = FCW + 0x24;
const DATA: usize = FCW + 0x28;

/// KEY: the unlock code with WRKEY, which unlocks CTRLA for one operation.
const WRKEY: u32 = 0x91C3_2C01;
/// MUTEX: LOCK with OWNER 1, the CPU.
const MUTEX_CPU: u32 = 0b011;
const MUTEX_MASK: u32 = 0b111;
const STATUS_BUSY: u32 = 1;
const DONE: u32 = 1 << 0;
/// KEYERR, CFGERR, FIFOERR, BUSERR, WPERR, OPERR, SECERR, RSTERR, WRERR.
const ERRORS: u32 = 0x3FFE;


const SRAM: core::ops::Range<usize> = 0x2000_0000..0x2008_0000;

/// NVMOP values (CTRLA bits 3:0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    QuadWrite = 0b0010,
    RowWrite = 0b0011,
    PageErase = 0b0100,
}

/// What went wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// An offset or length isn't a multiple of the operation's unit.
    NotAligned,
    /// The range runs past the flash.
    OutOfBounds,
    /// The HSM holds the FCW's mutex.
    Busy,
    /// The FCW reported errors: its INTFLAG bits (`WPERR` 0x20 for a
    /// write-protected target, `SECERR` 0x80, `WRERR` 0x2000, …).
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

/// One FCW write of a run of data: a row (from SRAM) or a flash word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Step {
    operation: Operation,
    /// From the start of the run.
    start: usize,
    length: usize,
}

/// The writes for `length` bytes at `offset`: rows where a whole aligned
/// row fits and the data is in SRAM, flash words elsewhere. Both are
/// multiples of [`WORD_SIZE`].
fn steps(offset: usize, length: usize, from_sram: bool) -> impl Iterator<Item = Step> {
    let mut start = 0;
    core::iter::from_fn(move || {
        if start >= length {
            return None;
        }
        let row = from_sram && (offset + start).is_multiple_of(ROW_SIZE) && length - start >= ROW_SIZE;
        let step = if row {
            Step { operation: Operation::RowWrite, start, length: ROW_SIZE }
        } else {
            Step { operation: Operation::QuadWrite, start, length: WORD_SIZE }
        };
        start += step.length;
        Some(step)
    })
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
    // SAFETY: an FCW register, read whole.
    unsafe { read_volatile(address as *const u32) }
}

fn write(address: usize, value: u32) {
    // SAFETY: an FCW register; only this driver writes them.
    unsafe { write_volatile(address as *mut u32, value) }
}

static WAKER: AtomicWaker = AtomicWaker::new();

/// The FCW's handler for async operations: bind `FCW` to it with
/// [`bind_interrupts!`](crate::bind_interrupts).
pub struct InterruptHandler {
    _private: (),
}

impl Handler<typelevel::FCW> for InterruptHandler {
    unsafe fn on_interrupt() {
        // Masked until the next operation; the waiting side reads the flags.
        write(INTENCLR, DONE | ERRORS);
        WAKER.wake();
    }
}

/// PFM, through the FCW: [`Blocking`] waits on each operation by polling,
/// [`Async`] on the FCW's interrupt.
pub struct Flash<'d, M = Blocking> {
    capacity: usize,
    _fcw: Peri<'d, peripherals::FCW>,
    _mode: PhantomData<M>,
}

impl<'d> Flash<'d, Blocking> {
    /// The chip's PFM ([`FLASH_SIZE`](crate::FLASH_SIZE) bytes). Erasing or
    /// writing the code that runs, or data in use, is the caller's to avoid.
    #[must_use]
    pub fn new_blocking(fcw: Peri<'d, peripherals::FCW>) -> Self {
        Self { capacity: crate::FLASH_SIZE, _fcw: fcw, _mode: PhantomData }
    }
}

impl<'d> Flash<'d, Async> {
    /// The chip's PFM, waiting on the FCW's interrupt (`FCW`, bound to
    /// [`InterruptHandler`]). Erasing or writing the code that runs, or data
    /// in use, is the caller's to avoid.
    #[must_use]
    pub fn new(fcw: Peri<'d, peripherals::FCW>, _irqs: impl Binding<typelevel::FCW, InterruptHandler>) -> Self {
        write(INTENCLR, DONE | ERRORS);
        let irq = crate::interrupt::FCW;
        irq.unpend();
        // SAFETY: the handler only masks the FCW's interrupt and wakes.
        unsafe { irq.enable() };
        Self { capacity: crate::FLASH_SIZE, _fcw: fcw, _mode: PhantomData }
    }
}

/// Starts one NVMOP at `address`, after `setup` (which fills DATA or
/// SRCADDR): takes the FCW's mutex (waiting out an operation still running
/// from a dropped future) and kicks it off.
fn start(operation: Operation, address: u32, setup: impl FnOnce()) -> Result<(), Error> {
    while read(STATUS) & STATUS_BUSY != 0 {}
    write(MUTEX, MUTEX_CPU);
    if read(MUTEX) & MUTEX_MASK != MUTEX_CPU {
        return Err(Error::Busy);
    }
    write(INTFLAG, ERRORS | DONE);
    write(ADDR, address);
    setup();
    write(KEY, WRKEY);
    write(CTRLA, operation as u32);
    Ok(())
}

/// Whether the operation started has ended (DONE or an error).
fn ended() -> bool {
    read(STATUS) & STATUS_BUSY == 0 && read(INTFLAG) & (DONE | ERRORS) != 0
}

/// Ends the operation: its flags, the mutex back, the cache invalidated.
fn finish() -> Result<(), Error> {
    let flags = read(INTFLAG);
    write(INTFLAG, flags);
    write(MUTEX, 0);
    crate::cache::invalidate();
    if flags & ERRORS == 0 { Ok(()) } else { Err(Error::Failed(flags & ERRORS)) }
}

/// Runs one NVMOP and waits for it by polling.
fn run_blocking(operation: Operation, address: u32, setup: impl FnOnce()) -> Result<(), Error> {
    start(operation, address, setup)?;
    while !ended() {}
    finish()
}

/// Waits out an operation if its future is dropped first.
struct Running;

impl Drop for Running {
    fn drop(&mut self) {
        write(INTENCLR, DONE | ERRORS);
        while !ended() {}
        let _ = finish();
    }
}

/// Runs one NVMOP and waits for it on the interrupt.
async fn run_async(operation: Operation, address: u32, setup: impl FnOnce()) -> Result<(), Error> {
    start(operation, address, setup)?;
    let running = Running;
    poll_fn(|cx| {
        WAKER.register(cx.waker());
        if ended() {
            return Poll::Ready(());
        }
        write(INTENSET, DONE | ERRORS);
        // It may have ended between the check and the unmasking.
        if ended() { Poll::Ready(()) } else { Poll::Pending }
    })
    .await;
    core::mem::forget(running);
    write(INTENCLR, DONE | ERRORS);
    finish()
}

/// What `data` (all of one step) asks of the FCW before the operation.
fn load(step: Step, data: &[u8]) {
    match step.operation {
        #[allow(clippy::cast_possible_truncation, reason = "an SRAM address; addresses are 32 bits on the target")]
        Operation::RowWrite => write(SRCADDR, data.as_ptr() as u32),
        _ => {
            for (i, word) in data.as_chunks::<4>().0.iter().enumerate() {
                write(DATA + 4 * i, u32::from_le_bytes(*word));
            }
        }
    }
}

/// Whether a write of `bytes` may use row writes (the FCW reads rows from
/// SRAM, word-aligned).
fn from_sram(bytes: &[u8]) -> bool {
    SRAM.contains(&(bytes.as_ptr() as usize)) && (bytes.as_ptr() as usize).is_multiple_of(4)
}

fn read_into(offset: u32, bytes: &mut [u8]) {
    let start = (PFM_BASE + offset) as *const u8;
    for (i, byte) in bytes.iter_mut().enumerate() {
        // SAFETY: inside PFM, checked by the caller.
        *byte = unsafe { read_volatile(start.add(i)) };
    }
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
        for page in (from..to).step_by(PAGE_SIZE) {
            run_blocking(Operation::PageErase, PFM_BASE + page, || {})?;
        }
        Ok(())
    }

    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Error> {
        check(offset, bytes.len(), WORD_SIZE, self.capacity)?;
        for step in steps(offset as usize, bytes.len(), from_sram(bytes)) {
            let data = &bytes[step.start..step.start + step.length];
            #[expect(clippy::cast_possible_truncation, reason = "inside PFM, checked above")]
            let address = PFM_BASE + offset + step.start as u32;
            run_blocking(step.operation, address, || load(step, data))?;
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
        for page in (from..to).step_by(PAGE_SIZE) {
            run_async(Operation::PageErase, PFM_BASE + page, || {}).await?;
        }
        Ok(())
    }

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Error> {
        check(offset, bytes.len(), WORD_SIZE, self.capacity)?;
        for step in steps(offset as usize, bytes.len(), from_sram(bytes)) {
            let data = &bytes[step.start..step.start + step.length];
            #[expect(clippy::cast_possible_truncation, reason = "inside PFM, checked above")]
            let address = PFM_BASE + offset + step.start as u32;
            run_async(step.operation, address, || load(step, data)).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(offset: usize, length: usize, from_sram: bool) -> Vec<(Operation, usize)> {
        steps(offset, length, from_sram).map(|s| (s.operation, s.start)).collect()
    }

    #[test]
    fn words_only_from_flash() {
        let plan = plan(0, 2048, false);
        assert_eq!(plan.len(), 64);
        assert!(plan.iter().all(|(op, _)| *op == Operation::QuadWrite));
    }

    #[test]
    fn rows_where_they_fit_from_sram() {
        assert_eq!(plan(0, 2048, true), [(Operation::RowWrite, 0), (Operation::RowWrite, 1024)]);
        // Words up to the row boundary, a row, then words.
        let plan = plan(1024 - 64, 64 + 1024 + 32, true);
        assert_eq!(plan[..2], [(Operation::QuadWrite, 0), (Operation::QuadWrite, 32)]);
        assert_eq!(plan[2], (Operation::RowWrite, 64));
        assert_eq!(plan[3..], [(Operation::QuadWrite, 64 + 1024)]);
    }

    #[test]
    fn checks_alignment_and_bounds() {
        let capacity = 2 << 20;
        assert_eq!(check(0, 32, WORD_SIZE, capacity), Ok(()));
        assert_eq!(check(16, 32, WORD_SIZE, capacity), Err(Error::NotAligned));
        assert_eq!(check(0, 31, WORD_SIZE, capacity), Err(Error::NotAligned));
        assert_eq!(check(0, 4096, PAGE_SIZE, capacity), Ok(()));
        assert_eq!(check(4096 * 511, 8192, PAGE_SIZE, capacity), Err(Error::OutOfBounds));
        assert_eq!(check(0xFFFF_FFE0, 64, WORD_SIZE, capacity), Err(Error::OutOfBounds));
    }
}

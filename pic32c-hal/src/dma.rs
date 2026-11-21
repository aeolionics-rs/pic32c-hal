// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The DMA controllers (DS60001795 chapter 26): DMA0 with 8 channels and
//! DMA1 with 4, each channel a singleton (`DMA0_CH0` to `DMA0_CH7`,
//! `DMA1_CH0` to `DMA1_CH3`), on the SG01 and the GC01 alike.
//!
//! This is the PIC32CK's own DMA IP (`dma_axi_apb_v1`), not the SAM D5x
//! DMAC: each channel moves a *block* of up to 64 KiB as *cells*, one cell
//! per start trigger. A [`Transfer`] is a future for one block:
//!
//! - [`Transfer::new_copy`]: memory to memory, the whole block on one
//!   software trigger;
//! - [`Transfer::new_read`] and [`Transfer::new_write`]: a peripheral's data
//!   register to or from memory, one word per DMA request from the
//!   peripheral (the [`Request`]s are table 26-1's).
//!
//! It completes when the controller reports the block complete, or with
//! [`Error::Bus`] if a read or write hit a bus error. Dropping it before
//! then stops the channel, which discards any data in flight.
//!
//! Peripherals with bus masters of their own (CAN, Ethernet, USB) don't use
//! these channels. With feature `rt` the HAL defines the controllers'
//! priority vectors (`DMA0_PRI0` to `_PRI2`, `DMA1_PRI0` and `_PRI1`); every
//! channel runs at priority 1, so only `DMA0_PRI0` and `DMA1_PRI0` fire.

use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering, compiler_fence};
use core::task::{Context, Poll};

use embassy_sync::waitqueue::AtomicWaker;

use crate::interrupt::{Interrupt, InterruptExt};
use crate::{Peri, PeripheralType, peripherals};

/// The controllers' register blocks.
const CONTROLLERS: [usize; 2] = [0x4480_2000, 0x4480_4000];
/// Channels on each controller.
const CHANNELS: [usize; 2] = [8, 4];
/// All channels.
const ALL_CHANNELS: usize = 12;

const CTRLA: usize = 0x00;
const CTRLA_ENABLE: u32 = 1 << 1;

/// Channel `k`'s registers start at `0x50 + 0x50 * k`.
const CHANNEL_BASE: usize = 0x50;
const CHANNEL_STRIDE: usize = 0x50;
const CHCTRLA: usize = 0x00;
const CHCTRLB: usize = 0x04;
const CHINTENCLR: usize = 0x0C;
const CHINTENSET: usize = 0x10;
const CHINTF: usize = 0x14;
const CHSSA: usize = 0x18;
const CHDSA: usize = 0x1C;
const CHSSTRD: usize = 0x20;
const CHDSTRD: usize = 0x24;
const CHXSIZ: usize = 0x28;
const CHNXT: usize = 0x38;
const CHSTATBC: usize = 0x40;
const CHSTAT: usize = 0x48;

const CHCTRLA_ENABLE: u32 = 1 << 0;
const CHCTRLA_SWFRC: u32 = 1 << 16;
const CHCTRLA_LLEN: u32 = 1 << 8;
const CHCTRLB_RAS_SHIFT: u32 = 4;
const CHCTRLB_TRIG_SHIFT: u32 = 16;
const CHCTRLB_CASTEN: u32 = 1 << 29;
/// Transfer aborted, block complete.
const INT_TA: u32 = 1 << 1;
const INT_BC: u32 = 1 << 3;
/// Block half complete.
const INT_BH: u32 = 1 << 4;
/// Bus errors.
const INT_WRE: u32 = 1 << 17;
const INT_RDE: u32 = 1 << 18;
/// Every flag CHINTF has.
const INT_ALL: u32 = 0b11_1111 | INT_WRE | INT_RDE;
const CHSTAT_BLKBUSY: u32 = 1 << 0;

/// MCLK's clock masks: AHB in CLKMSK0 (bits 8 and 9), APB in CLKMSK2 (bits
/// 1 and 2), for DMA0 and DMA1.
const MCLK_CLKMSK0: usize = 0x4401_203C;
const MCLK_CLKMSK2: usize = 0x4401_2044;

/// Largest block.
pub const MAX_BLOCK: usize = 65_536;
/// Largest cell.
const MAX_CELL: usize = 1024;

/// Address sequences (CHCTRLB.RAS and WAS).
mod sequence {
    pub const INCREMENT_BYTE: u32 = 0b000;
    pub const INCREMENT_HALFWORD: u32 = 0b001;
    pub const INCREMENT_AUTO: u32 = 0b010;
    pub const FIXED_BYTE: u32 = 0b011;
    pub const FIXED_HALFWORD: u32 = 0b100;
    pub const FIXED_WORD: u32 = 0b101;
}

/// Where [`Transfer::new_read_discard`] throws its words: only the DMA
/// writes it, and nothing reads it.
struct Sink(core::cell::UnsafeCell<u32>);
// SAFETY: never read; the DMA's writes to it race only each other.
unsafe impl Sync for Sink {}
static SINK: Sink = Sink(core::cell::UnsafeCell::new(0));

static WAKERS: [AtomicWaker; ALL_CHANNELS] = [const { AtomicWaker::new() }; ALL_CHANNELS];
static STARTED: [AtomicBool; 2] = [const { AtomicBool::new(false) }; 2];
/// Which channels run a [`ReadableRing`], and its laps so far.
static CIRCULAR: [AtomicBool; ALL_CHANNELS] = [const { AtomicBool::new(false) }; ALL_CHANNELS];
static LAPS: [AtomicU32; ALL_CHANNELS] = [const { AtomicU32::new(0) }; ALL_CHANNELS];

/// A linked-list block descriptor (table 26-4), the ten words, aligned so
/// that it sits well inside a 1 KiB boundary as the DMA requires.
#[repr(C, align(64))]
struct Descriptor(core::cell::UnsafeCell<[u32; 10]>);
// SAFETY: written only while its channel is stopped, read by the DMA.
unsafe impl Sync for Descriptor {}
static DESCRIPTORS: [Descriptor; ALL_CHANNELS] = [const { Descriptor(core::cell::UnsafeCell::new([0; 10])) }; ALL_CHANNELS];

/// BDCFG: reload DSA and XSIZ; ENABLE and LLEN set again.
const BDCFG_DSA: u32 = 1 << 3;
const BDCFG_XSIZ: u32 = 1 << 6;
const BDCFG_ENABLE: u32 = 1 << 24;
const BDCFG_LLEN: u32 = 1 << 25;

/// A word the DMA moves: `u8`, `u16` or `u32`.
#[allow(private_bounds, reason = "sealed")]
pub trait Word: SealedWord + Copy + 'static {}

trait SealedWord {
    /// The address sequences that step through memory and stay on a
    /// register, one word per beat.
    const INCREMENT: u32;
    const FIXED: u32;
}

impl SealedWord for u8 {
    const INCREMENT: u32 = sequence::INCREMENT_BYTE;
    const FIXED: u32 = sequence::FIXED_BYTE;
}
impl SealedWord for u16 {
    const INCREMENT: u32 = sequence::INCREMENT_HALFWORD;
    const FIXED: u32 = sequence::FIXED_HALFWORD;
}
impl SealedWord for u32 {
    // There is no increment-by-word sequence: auto picks word beats for
    // word-aligned addresses and sizes.
    const INCREMENT: u32 = sequence::INCREMENT_AUTO;
    const FIXED: u32 = sequence::FIXED_WORD;
}
impl Word for u8 {}
impl Word for u16 {}
impl Word for u32 {}

/// A peripheral's DMA request, the start trigger of each cell (table 26-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Request(u8);

impl Request {
    /// SERCOM `n`'s receive request (data ready to read), `n` 0 to 7.
    ///
    /// # Panics
    ///
    /// If `n` is over 7: at compile time in a `const`.
    #[must_use]
    pub const fn sercom_rx(n: u8) -> Self {
        assert!(n < 8, "SERCOM0 to SERCOM7");
        Self(if n < 4 { 5 + 2 * n } else { 41 + 2 * (n - 4) })
    }

    /// SERCOM `n`'s transmit request (room to write), `n` 0 to 7.
    ///
    /// # Panics
    ///
    /// As [`sercom_rx`](Self::sercom_rx).
    #[must_use]
    pub const fn sercom_tx(n: u8) -> Self {
        Self(Self::sercom_rx(n).0 + 1)
    }

    /// The trigger index itself, for a request this type doesn't name.
    ///
    /// # Safety
    ///
    /// `index` must be table 26-1's request of the peripheral the transfer
    /// reads or writes, or the channel may run at the wrong pace.
    #[must_use]
    pub const unsafe fn from_index(index: u8) -> Self {
        Self(index)
    }

    /// The trigger index.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }
}

/// Why a transfer ended early.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// A read or write got a bus error response.
    Bus,
    /// The controller aborted the transfer.
    Aborted,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Bus => "DMA bus error",
            Self::Aborted => "DMA transfer aborted",
        })
    }
}

impl core::error::Error for Error {}

/// Seals [`Channel`].
trait SealedChannel {}

/// A DMA channel.
#[allow(private_bounds, reason = "sealed: only this crate's channels")]
pub trait Channel: PeripheralType + SealedChannel + Into<AnyChannel> + 'static {}

/// Any DMA channel.
#[derive(Clone, Copy)]
pub struct AnyChannel {
    controller: u8,
    index: u8,
}

impl PeripheralType for AnyChannel {}
impl SealedChannel for AnyChannel {}
impl Channel for AnyChannel {}

impl AnyChannel {
    fn base(self) -> usize {
        CONTROLLERS[self.controller as usize] + CHANNEL_BASE + CHANNEL_STRIDE * self.index as usize
    }

    /// Its number among all channels, DMA0's first.
    fn number(self) -> usize {
        self.controller as usize * CHANNELS[0] + self.index as usize
    }
}

macro_rules! channels {
    ($($name:ident = $controller:literal, $index:literal;)*) => {
        $(
            impl SealedChannel for peripherals::$name {}
            impl Channel for peripherals::$name {}
            impl From<peripherals::$name> for AnyChannel {
                fn from(_: peripherals::$name) -> Self {
                    Self { controller: $controller, index: $index }
                }
            }
        )*
    };
}

channels! {
    DMA0_CH0 = 0, 0; DMA0_CH1 = 0, 1; DMA0_CH2 = 0, 2; DMA0_CH3 = 0, 3;
    DMA0_CH4 = 0, 4; DMA0_CH5 = 0, 5; DMA0_CH6 = 0, 6; DMA0_CH7 = 0, 7;
    DMA1_CH0 = 1, 0; DMA1_CH1 = 1, 1; DMA1_CH2 = 1, 2; DMA1_CH3 = 1, 3;
}

fn read(address: usize) -> u32 {
    // SAFETY: a DMA or MCLK register, read as a 32-bit word.
    unsafe { read_volatile(address as *const u32) }
}

fn write(address: usize, value: u32) {
    // SAFETY: a DMA or MCLK register, written as a 32-bit word.
    unsafe { write_volatile(address as *mut u32, value) }
}

/// Clocks the controller, enables it and unmasks its vectors, once.
fn start_controller(controller: usize) {
    if STARTED[controller].swap(true, Ordering::AcqRel) {
        return;
    }
    critical_section::with(|_| {
        write(MCLK_CLKMSK0, read(MCLK_CLKMSK0) | 1 << (8 + controller));
        write(MCLK_CLKMSK2, read(MCLK_CLKMSK2) | 1 << (1 + controller));
    });
    let base = CONTROLLERS[controller];
    write(base + CTRLA, read(base + CTRLA) | CTRLA_ENABLE);
    let lines: &[Interrupt] = if controller == 0 {
        &[Interrupt::DMA0_PRI0, Interrupt::DMA0_PRI1, Interrupt::DMA0_PRI2]
    } else {
        &[Interrupt::DMA1_PRI0, Interrupt::DMA1_PRI1]
    };
    for line in lines {
        line.unpend();
        // SAFETY: the handlers touch only the channels' interrupt enables
        // and wakers.
        unsafe { line.enable() };
    }
}

/// Masks the channels' interrupts that are pending and wakes their
/// transfers, which read the flags themselves.
#[cfg_attr(not(feature = "rt"), expect(dead_code, reason = "called from the vectors `rt` defines"))]
fn on_interrupt(controller: usize) {
    for index in 0..CHANNELS[controller] {
        let base = CONTROLLERS[controller] + CHANNEL_BASE + CHANNEL_STRIDE * index;
        let number = controller * CHANNELS[0] + index;
        let flags = read(base + CHINTF) & read(base + CHINTENSET);
        if flags == 0 {
            continue;
        }
        if CIRCULAR[number].load(Ordering::Acquire) {
            // A ring: count the laps, leave the interrupts on.
            count_lap(base, number);
            write(base + CHINTF, INT_BH);
        } else {
            write(base + CHINTENCLR, INT_ALL);
        }
        WAKERS[number].wake();
    }
}

/// Counts a ring's lap if its block-complete flag is set, clearing it.
fn count_lap(base: usize, number: usize) {
    if read(base + CHINTF) & INT_BC != 0 {
        write(base + CHINTF, INT_BC);
        LAPS[number].fetch_add(1, Ordering::AcqRel);
    }
}

macro_rules! vectors {
    ($($vector:ident = $controller:literal),*) => {
        $(
            #[cfg(feature = "rt")]
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn $vector() {
                on_interrupt($controller);
            }
        )*
    };
}

vectors!(DMA0_PRI0 = 0, DMA0_PRI1 = 0, DMA0_PRI2 = 0, DMA1_PRI0 = 1, DMA1_PRI1 = 1);

/// One block transfer on a channel, running until it completes or is
/// dropped.
#[must_use = "dropping a transfer stops it"]
pub struct Transfer<'a> {
    channel: Peri<'a, AnyChannel>,
    _buffers: PhantomData<&'a mut [u8]>,
}

/// How to set a channel up.
struct Setup {
    source: usize,
    destination: usize,
    read_sequence: u32,
    write_sequence: u32,
    bytes: usize,
    cell: usize,
    /// The start trigger: 0 for software.
    trigger: u8,
}

impl<'a> Transfer<'a> {
    /// Copies `source` into `destination`, which must be as long, on
    /// `channel`; the block moves on one software trigger, in the widest
    /// beats the two addresses' alignment allows.
    ///
    /// # Panics
    ///
    /// If the slices' lengths differ, or the block is over [`MAX_BLOCK`]
    /// bytes or empty.
    pub fn new_copy<W: Word>(channel: Peri<'a, impl Channel>, source: &'a [W], destination: &'a mut [W]) -> Self {
        assert_eq!(source.len(), destination.len(), "source and destination differ in length");
        let bytes = core::mem::size_of_val(source);
        Self::start(
            channel.into(),
            &Setup {
                source: source.as_ptr() as usize,
                destination: destination.as_mut_ptr() as usize,
                // Memory both sides: the controller picks the widest beats the
                // alignment allows, whatever `W` is.
                read_sequence: sequence::INCREMENT_AUTO,
                write_sequence: sequence::INCREMENT_AUTO,
                bytes,
                cell: bytes.min(MAX_CELL),
                trigger: 0,
            },
        )
    }

    /// Reads `buffer.len()` words from the peripheral register at `register`
    /// into `buffer`, one each time the peripheral makes `request`.
    ///
    /// # Safety
    ///
    /// `register` must be the data register of the peripheral `request`
    /// belongs to, readable as a `W`, and that peripheral set up to make
    /// the request.
    ///
    /// # Panics
    ///
    /// If the block is over [`MAX_BLOCK`] bytes or empty.
    pub unsafe fn new_read<W: Word>(channel: Peri<'a, impl Channel>, request: Request, register: *const W, buffer: &'a mut [W]) -> Self {
        Self::start(
            channel.into(),
            &Setup {
                source: register as usize,
                destination: buffer.as_mut_ptr() as usize,
                read_sequence: W::FIXED,
                write_sequence: W::INCREMENT,
                bytes: core::mem::size_of_val(buffer),
                cell: size_of::<W>(),
                trigger: request.0,
            },
        )
    }

    /// Writes `buffer` to the peripheral register at `register`, a word each
    /// time the peripheral makes `request`.
    ///
    /// # Safety
    ///
    /// As [`new_read`](Self::new_read), for a register writable as a `W`.
    ///
    /// # Panics
    ///
    /// If the block is over [`MAX_BLOCK`] bytes or empty.
    pub unsafe fn new_write<W: Word>(channel: Peri<'a, impl Channel>, request: Request, buffer: &'a [W], register: *mut W) -> Self {
        Self::start(
            channel.into(),
            &Setup {
                source: buffer.as_ptr() as usize,
                destination: register as usize,
                read_sequence: W::INCREMENT,
                write_sequence: W::FIXED,
                bytes: core::mem::size_of_val(buffer),
                cell: size_of::<W>(),
                trigger: request.0,
            },
        )
    }

    /// Writes `word` to the peripheral register at `register` `count`
    /// times, once each time the peripheral makes `request`: to clock a
    /// peripheral (an SPI host's dummy bytes, say) without a buffer.
    ///
    /// # Safety
    ///
    /// As [`new_write`](Self::new_write).
    ///
    /// # Panics
    ///
    /// If `count` words are over [`MAX_BLOCK`] bytes, or `count` is 0.
    pub unsafe fn new_write_repeated<W: Word>(channel: Peri<'a, impl Channel>, request: Request, word: &'a W, count: usize, register: *mut W) -> Self {
        Self::start(
            channel.into(),
            &Setup {
                source: core::ptr::from_ref(word) as usize,
                destination: register as usize,
                read_sequence: W::FIXED,
                write_sequence: W::FIXED,
                bytes: count * size_of::<W>(),
                cell: size_of::<W>(),
                trigger: request.0,
            },
        )
    }

    /// Reads `count` words from the peripheral register at `register`, once
    /// each time the peripheral makes `request`, and throws them away: to
    /// keep a receiver from overflowing while only sending.
    ///
    /// # Safety
    ///
    /// As [`new_read`](Self::new_read).
    ///
    /// # Panics
    ///
    /// If `count` words are over [`MAX_BLOCK`] bytes, or `count` is 0.
    pub unsafe fn new_read_discard<W: Word>(channel: Peri<'a, impl Channel>, request: Request, register: *const W, count: usize) -> Self {
        Self::start(
            channel.into(),
            &Setup {
                source: register as usize,
                destination: SINK.0.get() as usize,
                read_sequence: W::FIXED,
                write_sequence: W::FIXED,
                bytes: count * size_of::<W>(),
                cell: size_of::<W>(),
                trigger: request.0,
            },
        )
    }

    fn start(channel: Peri<'a, AnyChannel>, setup: &Setup) -> Self {
        assert!(setup.bytes > 0 && setup.bytes <= MAX_BLOCK, "a block is 1 to 65,536 bytes");
        start_controller(channel.controller as usize);
        let base = channel.base();
        stop(base);
        write(base + CHINTF, INT_ALL);
        write(base + CHSSA, u32::try_from(setup.source).unwrap_or(0));
        write(base + CHDSA, u32::try_from(setup.destination).unwrap_or(0));
        write(base + CHSSTRD, 0);
        write(base + CHDSTRD, 0);
        // A size of 0 means the largest: 65,536 bytes, 1,024 bytes.
        #[expect(clippy::cast_possible_truncation, reason = "the field's width, its largest value 0")]
        write(base + CHXSIZ, ((setup.bytes as u32 & 0xFFFF) << 16) | (setup.cell as u32 & 0x3FF));
        let software = setup.trigger == 0;
        write(
            base + CHCTRLB,
            setup.write_sequence | (setup.read_sequence << CHCTRLB_RAS_SHIFT) | (u32::from(setup.trigger) << CHCTRLB_TRIG_SHIFT) | if software { CHCTRLB_CASTEN } else { 0 },
        );
        write(base + CHINTENSET, INT_BC | INT_TA);
        // The buffers' contents must be in memory before the channel reads
        // them.
        compiler_fence(Ordering::SeqCst);
        write(base + CHCTRLA, CHCTRLA_ENABLE);
        if software {
            write(base + CHCTRLA, CHCTRLA_ENABLE | CHCTRLA_SWFRC);
        }
        Self { channel, _buffers: PhantomData }
    }

    /// Whether the block is still moving.
    #[must_use]
    pub fn is_running(&self) -> bool {
        running(self.channel.base())
    }

    /// Bytes still to move.
    #[must_use]
    pub fn remaining(&self) -> usize {
        let base = self.channel.base();
        let total = (read(base + CHXSIZ) >> 16) as usize;
        let total = if total == 0 { MAX_BLOCK } else { total };
        // CHSTATBC counts the bytes of the block transferred so far.
        total.saturating_sub(read(base + 0x40) as usize & 0x1_FFFF)
    }

    /// Spins until the block is done.
    ///
    /// # Errors
    ///
    /// As the future's output.
    pub fn blocking_wait(self) -> Result<(), Error> {
        while self.is_running() {}
        self.outcome()
    }

    fn outcome(&self) -> Result<(), Error> {
        compiler_fence(Ordering::SeqCst);
        let flags = read(self.channel.base() + CHINTF);
        if flags & (INT_RDE | INT_WRE) != 0 {
            Err(Error::Bus)
        } else if flags & INT_TA != 0 {
            Err(Error::Aborted)
        } else {
            Ok(())
        }
    }
}

impl Future for Transfer<'_> {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let base = self.channel.base();
        WAKERS[self.channel.number()].register(cx.waker());
        // Unmasked again before the check, so an end after it still wakes.
        write(base + CHINTENSET, INT_BC | INT_TA);
        if running(base) { Poll::Pending } else { Poll::Ready(self.outcome()) }
    }
}

impl Drop for Transfer<'_> {
    fn drop(&mut self) {
        let base = self.channel.base();
        write(base + CHINTENCLR, INT_ALL);
        stop(base);
    }
}

fn running(base: usize) -> bool {
    read(base + CHCTRLA) & CHCTRLA_ENABLE != 0 || read(base + CHSTAT) & CHSTAT_BLKBUSY != 0
}

/// Stops the channel and resets it: clearing ENABLE suspends a block in
/// progress, and a write to a control register then resets the channel,
/// flushing its FIFO, BLKBUSY clearing when that's done.
fn stop(base: usize) {
    write(base + CHCTRLA, 0);
    if read(base + CHSTAT) & CHSTAT_BLKBUSY != 0 {
        write(base + CHXSIZ, read(base + CHXSIZ));
        while read(base + CHSTAT) & CHSTAT_BLKBUSY != 0 {}
    }
    compiler_fence(Ordering::SeqCst);
}

/// A ring buffer a channel fills continuously from a peripheral's register,
/// as Embassy's `ReadableRingBuffer`: one self-linked descriptor brings the
/// channel back to the ring's start at the end of each block, with no CPU in
/// between. Reads take what has come since the last; a reader a whole ring
/// behind has lost bytes, [`Overrun`].
pub struct ReadableRing<'a> {
    channel: Peri<'a, AnyChannel>,
    request: Request,
    register: usize,
    ring: *mut u8,
    len: usize,
    /// Bytes read so far, all laps.
    read: u64,
    running: bool,
    _ring: PhantomData<&'a mut [u8]>,
}

/// The reader fell a whole ring behind: bytes were lost. Reading carries on
/// from the newest data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Overrun;

impl<'a> ReadableRing<'a> {
    /// A ring on `channel`, filling `ring` a byte each time the peripheral
    /// makes `request`, from `register`. Nothing moves until
    /// [`start`](Self::start).
    ///
    /// # Safety
    ///
    /// `register` must be the data register of the peripheral `request`
    /// belongs to, readable as a byte.
    ///
    /// # Panics
    ///
    /// If the ring is empty or over [`MAX_BLOCK`] bytes.
    pub unsafe fn new(channel: Peri<'a, impl Channel>, request: Request, register: *const u8, ring: &'a mut [u8]) -> Self {
        assert!(!ring.is_empty() && ring.len() <= MAX_BLOCK, "a ring is 1 to 65,536 bytes");
        Self {
            channel: channel.into(),
            request,
            register: register as usize,
            ring: ring.as_mut_ptr(),
            len: ring.len(),
            read: 0,
            running: false,
            _ring: PhantomData,
        }
    }

    /// Starts filling the ring from its start.
    pub fn start(&mut self) {
        start_controller(self.channel.controller as usize);
        let base = self.channel.base();
        let number = self.channel.number();
        stop(base);
        #[expect(clippy::cast_possible_truncation, reason = "the field's width, its largest value 0")]
        let xsiz = ((self.len as u32 & 0xFFFF) << 16) | 1;
        let ring = u32::try_from(self.ring as usize).unwrap_or(0);
        let descriptor = DESCRIPTORS[number].0.get();
        let next = u32::try_from(descriptor as usize).unwrap_or(0);
        // SAFETY: the channel is stopped, so the DMA isn't reading it.
        unsafe {
            (*descriptor)[0] = next;
            (*descriptor)[1] = BDCFG_DSA | BDCFG_XSIZ | BDCFG_ENABLE | BDCFG_LLEN;
            (*descriptor)[5] = ring;
            (*descriptor)[7] = xsiz;
        }
        LAPS[number].store(0, Ordering::Release);
        CIRCULAR[number].store(true, Ordering::Release);
        self.read = 0;
        write(base + CHINTF, INT_ALL);
        write(base + CHSSA, u32::try_from(self.register).unwrap_or(0));
        write(base + CHDSA, ring);
        write(base + CHSSTRD, 0);
        write(base + CHDSTRD, 0);
        write(base + CHXSIZ, xsiz);
        write(base + CHCTRLB, u8::INCREMENT | (u8::FIXED << CHCTRLB_RAS_SHIFT) | (u32::from(self.request.0) << CHCTRLB_TRIG_SHIFT));
        write(base + CHNXT, next);
        write(base + CHINTENSET, INT_BC | INT_BH);
        compiler_fence(Ordering::SeqCst);
        write(base + CHCTRLA, CHCTRLA_ENABLE | CHCTRLA_LLEN);
        self.running = true;
    }

    /// Stops filling the ring.
    pub fn stop(&mut self) {
        let base = self.channel.base();
        write(base + CHINTENCLR, INT_ALL);
        CIRCULAR[self.channel.number()].store(false, Ordering::Release);
        stop(base);
        self.running = false;
    }

    /// Whether it is running.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Bytes written into the ring so far, all laps: laps and the count in
    /// the current block read together, any lap not yet counted counted.
    fn written(&self) -> u64 {
        let base = self.channel.base();
        let number = self.channel.number();
        critical_section::with(|_| {
            count_lap(base, number);
            let laps = LAPS[number].load(Ordering::Acquire);
            let mut count = read(base + CHSTATBC) & 0x1_FFFF;
            // A block may have ended since: its count is the new block's.
            count_lap(base, number);
            let now = LAPS[number].load(Ordering::Acquire);
            if now != laps {
                count = read(base + CHSTATBC) & 0x1_FFFF;
            }
            u64::from(now) * self.len as u64 + u64::from(count).min(self.len as u64)
        })
    }

    /// Bytes waiting to be read.
    ///
    /// # Errors
    ///
    /// [`Overrun`] if more than a ring's worth have come.
    pub fn len(&self) -> Result<usize, Overrun> {
        let waiting = self.written().saturating_sub(self.read);
        if waiting > self.len as u64 { Err(Overrun) } else { Ok(usize::try_from(waiting).unwrap_or(usize::MAX)) }
    }

    /// Whether nothing waits.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self.len(), Ok(0))
    }

    /// Copies what has come into `buffer`, without waiting: how many bytes.
    ///
    /// # Errors
    ///
    /// [`Overrun`] if the reader fell a ring behind; it then skips to the
    /// newest data.
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Overrun> {
        let written = self.written();
        let waiting = written.saturating_sub(self.read);
        if waiting > self.len as u64 {
            self.read = written;
            return Err(Overrun);
        }
        compiler_fence(Ordering::SeqCst);
        let n = usize::try_from(waiting).unwrap_or(usize::MAX).min(buffer.len());
        let start = usize::try_from(self.read % self.len as u64).unwrap_or(0);
        let first = n.min(self.len - start);
        // SAFETY: the ring outlives this ('a), and these bytes are behind
        // the DMA's write position, which the check below confirms.
        unsafe {
            core::ptr::copy_nonoverlapping(self.ring.add(start), buffer.as_mut_ptr(), first);
            core::ptr::copy_nonoverlapping(self.ring, buffer.as_mut_ptr().add(first), n - first);
        }
        // Still within a lap of the writer after the copy, or the copy may
        // have been overwritten part way.
        if self.written().saturating_sub(self.read) > self.len as u64 {
            self.read = self.written();
            return Err(Overrun);
        }
        self.read += n as u64;
        Ok(n)
    }

    /// Registers `cx`'s waker for the channel's half and full laps.
    pub fn register_waker(&self, waker: &core::task::Waker) {
        WAKERS[self.channel.number()].register(waker);
    }
}

impl Drop for ReadableRing<'_> {
    fn drop(&mut self) {
        if self.running {
            self.stop();
        }
    }
}

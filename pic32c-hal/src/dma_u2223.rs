// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The DMA controller of the PIC32CM SG00 (DS60001921 chapter 28) and the
//! PL10 (DS40002667): one DMAC, with 8 channels on the SG00 (`DMA_CH0` to
//! `DMA_CH7`) and 2 on the PL10 (`DMA_CH0`, `DMA_CH1`), each a singleton.
//!
//! This is the SAM D21/C21 kind of DMAC (`dma_u2223_v2`), not the PIC32CK's
//! own DMA IP, behind the same API as the CK's [`dma`](self): each channel
//! moves a *block* of up to 65,535 *beats*, described by a descriptor in
//! SRAM, which the controller copies into a write-back section of its own as
//! the block goes. A [`Transfer`] is a future for one block:
//!
//! - [`Transfer::new_copy`]: memory to memory, the whole block on one
//!   software trigger;
//! - [`Transfer::new_read`] and [`Transfer::new_write`]: a peripheral's data
//!   register to or from memory, one beat per DMA request from the
//!   peripheral (the [`Request`]s are the trigger sources of the
//!   peripherals' chapters).
//!
//! It completes when the controller reports the block complete, or with
//! [`Error::Bus`] if a read or write hit a bus error. Dropping it before
//! then stops the channel, which finishes the beat in flight first.
//!
//! The channels' registers are reached through one window, CHID choosing
//! the channel, so every access to them is in a critical section. With
//! feature `rt` the HAL defines the controller's vectors: on the SG00
//! `DMAC_TCMPL0` to `_TCMPL3`, one each for channels 0 to 3, and
//! `DMAC_TCMPL4` for 4 to 7; on the PL10 `DMAC`, for both.

use core::cell::UnsafeCell;
use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::ptr::{addr_of, read_volatile, write_volatile};
use core::sync::atomic::{compiler_fence, fence};

use core::task::{Context, Poll};
use portable_atomic::{AtomicBool, AtomicU32, Ordering};

use embassy_sync::waitqueue::AtomicWaker;

use crate::interrupt::{Interrupt, InterruptExt};
use crate::{Peri, PeripheralType, peripherals};

/// The controller's register block, and its channels.
#[cfg(pic32cm_sg)]
const DMAC_BASE: usize = 0x4480_2000;
#[cfg(pic32cm_sg)]
const CHANNELS: usize = 8;
#[cfg(pic32cm_pl)]
const DMAC_BASE: usize = 0x4100_6000;
#[cfg(pic32cm_pl)]
const CHANNELS: usize = 2;

const CTRL: usize = 0x00;
const CTRL_SWRST: u16 = 1 << 0;
const CTRL_DMAENABLE: u16 = 1 << 1;
/// All four priority levels on.
const CTRL_LVLEN_ALL: u16 = 0xF << 8;
const SWTRIGCTRL: usize = 0x10;
const INTSTATUS: usize = 0x24;
const ACTIVE: usize = 0x30;
const ACTIVE_ABUSY: u32 = 1 << 15;
const ACTIVE_ID_SHIFT: u32 = 8;
const ACTIVE_ID_MASK: u32 = 0x1F;
const ACTIVE_BTCNT_SHIFT: u32 = 16;
const BASEADDR: usize = 0x34;
const WRBADDR: usize = 0x38;
const CHID: usize = 0x3F;
const CHCTRLA: usize = 0x40;
const CHCTRLA_SWRST: u8 = 1 << 0;
const CHCTRLA_ENABLE: u8 = 1 << 1;
const CHCTRLB: usize = 0x44;
const CHCTRLB_TRIGSRC_SHIFT: u32 = 8;
const CHCTRLB_TRIGACT_SHIFT: u32 = 22;
/// TRIGACT: a block per trigger, or a beat.
const TRIGACT_BLOCK: u32 = 0;
const TRIGACT_BEAT: u32 = 2;
const CHINTENCLR: usize = 0x4C;
const CHINTENSET: usize = 0x4D;
const CHINTFLAG: usize = 0x4E;
/// Transfer error, transfer (block) complete, channel suspended.
const INT_TERR: u8 = 1 << 0;
const INT_TCMPL: u8 = 1 << 1;
const INT_SUSP: u8 = 1 << 2;
const INT_ALL: u8 = INT_TERR | INT_TCMPL | INT_SUSP;

/// The DMAC's bus clocks, as MCLK registers and their bits. The SG00's
/// are CLKMSK0's bits 28 (AHB) and 29 (APB) (table 28-2), on at reset; the
/// PL10's AHBMASK bit 7, on at reset, and APBBMASK bit 3, off.
#[cfg(pic32cm_sg)]
const MCLK_DMAC: [(usize, u32); 1] = [(0x4400_A000 + 0x3C, 0b11 << 28)];
#[cfg(pic32cm_pl)]
const MCLK_DMAC: [(usize, u32); 2] = [(0x4000_0800 + 0x10, 1 << 7), (0x4000_0800 + 0x18, 1 << 3)];

/// BTCTRL: the descriptor is valid; at the block's end, disable the channel
/// if it was the last and set TCMPL; the beat size; the addresses that step.
const BTCTRL_VALID: u16 = 1 << 0;
const BTCTRL_BLOCKACT_INT: u16 = 1 << 3;
const BTCTRL_BEATSIZE_SHIFT: u16 = 8;
const BTCTRL_SRCINC: u16 = 1 << 10;
const BTCTRL_DSTINC: u16 = 1 << 11;

/// Largest block, in beats, and so in bytes for byte transfers.
pub const MAX_BLOCK: usize = 65_535;

/// A transfer descriptor (section 28.8): four words, 128-bit aligned.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
struct Descriptor {
    btctrl: u16,
    btcnt: u16,
    /// The *end* of the block, when the address steps: the controller works
    /// back from it.
    srcaddr: u32,
    dstaddr: u32,
    descaddr: u32,
}

impl Descriptor {
    const EMPTY: Self = Self {
        btctrl: 0,
        btcnt: 0,
        srcaddr: 0,
        dstaddr: 0,
        descaddr: 0,
    };
}

/// One descriptor per channel, in channel order, as BASEADDR and WRBADDR
/// want them.
#[repr(C)]
struct Section(UnsafeCell<[Descriptor; CHANNELS]>);
// SAFETY: a channel's entry is written by the CPU only while the channel is
// stopped, by the controller only while it runs.
unsafe impl Sync for Section {}

impl Section {
    const fn new() -> Self {
        Self(UnsafeCell::new([Descriptor::EMPTY; CHANNELS]))
    }

    fn entry(&self, channel: usize) -> *mut Descriptor {
        // SAFETY: in bounds; a pointer, no reference made.
        unsafe { self.0.get().cast::<Descriptor>().add(channel) }
    }

    fn address(&self, channel: usize) -> u32 {
        u32::try_from(self.entry(channel) as usize).unwrap_or(0)
    }

    fn set(&self, channel: usize, descriptor: Descriptor) {
        // SAFETY: the channel is stopped; volatile, as the controller reads it.
        unsafe { write_volatile(self.entry(channel), descriptor) }
    }
}

/// The channels' first descriptors (BASEADDR), what the controller writes
/// back as they go (WRBADDR), and each ring's second descriptor.
static FIRST: Section = Section::new();
static WRITE_BACK: Section = Section::new();
static SECOND: Section = Section::new();

/// Where [`Transfer::new_read_discard`] throws its words: only the DMA
/// writes it, and nothing reads it.
struct Sink(UnsafeCell<u32>);
// SAFETY: never read; the DMA's writes to it race only each other.
unsafe impl Sync for Sink {}
static SINK: Sink = Sink(UnsafeCell::new(0));

static WAKERS: [AtomicWaker; CHANNELS] = [const { AtomicWaker::new() }; CHANNELS];
static STARTED: AtomicBool = AtomicBool::new(false);
/// Which channels run a [`ReadableRing`], and the blocks it has finished.
static CIRCULAR: [AtomicBool; CHANNELS] = [const { AtomicBool::new(false) }; CHANNELS];
static BLOCKS: [AtomicU32; CHANNELS] = [const { AtomicU32::new(0) }; CHANNELS];

/// A word the DMA moves: `u8`, `u16` or `u32`.
#[allow(private_bounds, reason = "sealed")]
pub trait Word: SealedWord + Copy + 'static {}

trait SealedWord {
    /// BTCTRL.BEATSIZE.
    const BEATSIZE: u16;
}

impl SealedWord for u8 {
    const BEATSIZE: u16 = 0;
}
impl SealedWord for u16 {
    const BEATSIZE: u16 = 1;
}
impl SealedWord for u32 {
    const BEATSIZE: u16 = 2;
}
impl Word for u8 {}
impl Word for u16 {}
impl Word for u32 {}

/// A peripheral's DMA request, the trigger of each beat (CHCTRLB.TRIGSRC).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Request(u8);

impl Request {
    /// SERCOM `n`'s receive request (data ready to read), `n` 0 to 5 on the
    /// SG00, 0 or 1 on the PL10.
    ///
    /// # Panics
    ///
    /// If there is no SERCOM `n`: at compile time in a `const`.
    #[must_use]
    pub const fn sercom_rx(n: u8) -> Self {
        #[cfg(pic32cm_sg)]
        {
            assert!(n < 6, "SERCOM0 to SERCOM5");
            // SERCOM0-3 on 4-11, SERCOM4 and 5 on 24-27 (table 7-1).
            Self(if n < 4 { 4 + 2 * n } else { 24 + 2 * (n - 4) })
        }
        #[cfg(pic32cm_pl)]
        {
            assert!(n < 2, "SERCOM0 or SERCOM1");
            // SERCOM0 on 1-2, SERCOM1 on 3-4.
            Self(1 + 2 * n)
        }
    }

    /// SERCOM `n`'s transmit request (room to write), as
    /// [`sercom_rx`](Self::sercom_rx).
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
    /// `index` must be the DMA trigger of the peripheral the transfer reads
    /// or writes, or the channel may run at the wrong pace.
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
    /// The controller suspended the channel: it fetched a descriptor that
    /// wasn't valid.
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
    index: u8,
}

impl PeripheralType for AnyChannel {}
impl SealedChannel for AnyChannel {}
impl Channel for AnyChannel {}

impl AnyChannel {
    fn number(self) -> usize {
        self.index as usize
    }
}

macro_rules! channels {
    ($($name:ident = $index:literal;)*) => {
        $(
            impl SealedChannel for peripherals::$name {}
            impl Channel for peripherals::$name {}
            impl From<peripherals::$name> for AnyChannel {
                fn from(_: peripherals::$name) -> Self {
                    Self { index: $index }
                }
            }
        )*
    };
}

channels! {
    DMA_CH0 = 0; DMA_CH1 = 1;
}
#[cfg(pic32cm_sg)]
channels! {
    DMA_CH2 = 2; DMA_CH3 = 3; DMA_CH4 = 4; DMA_CH5 = 5; DMA_CH6 = 6; DMA_CH7 = 7;
}

fn read8(offset: usize) -> u8 {
    // SAFETY: a DMAC register, read as a byte.
    unsafe { read_volatile((DMAC_BASE + offset) as *const u8) }
}

fn write8(offset: usize, value: u8) {
    // SAFETY: a DMAC register, written as a byte.
    unsafe { write_volatile((DMAC_BASE + offset) as *mut u8, value) }
}

fn read16(offset: usize) -> u16 {
    // SAFETY: a DMAC register, read as a halfword.
    unsafe { read_volatile((DMAC_BASE + offset) as *const u16) }
}

fn write16(offset: usize, value: u16) {
    // SAFETY: a DMAC register, written as a halfword.
    unsafe { write_volatile((DMAC_BASE + offset) as *mut u16, value) }
}

fn read32(address: usize) -> u32 {
    // SAFETY: a DMAC or MCLK register, read as a word.
    unsafe { read_volatile(address as *const u32) }
}

fn write32(address: usize, value: u32) {
    // SAFETY: a DMAC or MCLK register, written as a word.
    unsafe { write_volatile(address as *mut u32, value) }
}

/// Runs `f` with channel `number`'s registers in the window, in a critical
/// section: an interrupt handler moving CHID meanwhile would send `f`'s
/// accesses to another channel.
fn with_channel<R>(number: usize, f: impl FnOnce() -> R) -> R {
    critical_section::with(|_| {
        #[expect(clippy::cast_possible_truncation, reason = "a channel number, under 8")]
        write8(CHID, number as u8);
        f()
    })
}

/// Clocks the controller, points it at the descriptor sections, enables it
/// and unmasks its vectors, once.
fn start_controller() {
    if STARTED.swap(true, Ordering::AcqRel) {
        return;
    }
    critical_section::with(|_| {
        for (register, bits) in MCLK_DMAC {
            write32(register, read32(register) | bits);
        }
    });
    // From reset, whatever ran before left it: BASEADDR and WRBADDR take
    // writes only while it is disabled.
    write16(CTRL, 0);
    while read16(CTRL) & CTRL_DMAENABLE != 0 {}
    write16(CTRL, CTRL_SWRST);
    while read16(CTRL) & CTRL_SWRST != 0 {}
    write32(DMAC_BASE + BASEADDR, FIRST.address(0));
    write32(DMAC_BASE + WRBADDR, WRITE_BACK.address(0));
    write16(CTRL, CTRL_DMAENABLE | CTRL_LVLEN_ALL);
    #[cfg(pic32cm_sg)]
    let lines = [Interrupt::DMAC_TCMPL0, Interrupt::DMAC_TCMPL1, Interrupt::DMAC_TCMPL2, Interrupt::DMAC_TCMPL3, Interrupt::DMAC_TCMPL4];
    #[cfg(pic32cm_pl)]
    let lines = [Interrupt::DMAC];
    for line in lines {
        line.unpend();
        // SAFETY: the handlers touch only the channels' interrupt enables,
        // flags and wakers.
        unsafe { line.enable() };
    }
}

/// Masks the interrupts of the channels in `channels` that are pending and
/// wakes their transfers, which read the flags themselves; a ring's block
/// complete is counted and cleared instead, its interrupts left on.
#[cfg_attr(not(feature = "rt"), expect(dead_code, reason = "called from the vectors `rt` defines"))]
fn on_interrupt(channels: core::ops::Range<usize>) {
    let pending = read32(DMAC_BASE + INTSTATUS);
    for number in channels {
        if pending & (1 << number) == 0 {
            continue;
        }
        let woke = with_channel(number, || {
            let flags = read8(CHINTFLAG) & read8(CHINTENSET);
            if flags == 0 {
                return false;
            }
            if CIRCULAR[number].load(Ordering::Acquire) {
                count_block(number);
                // A bus error stops the ring: don't let it fire again.
                write8(CHINTENCLR, flags & (INT_TERR | INT_SUSP));
            } else {
                write8(CHINTENCLR, INT_ALL);
            }
            true
        });
        if woke {
            WAKERS[number].wake();
        }
    }
}

/// Counts a ring's block if its transfer-complete flag is set, clearing it.
/// With the channel in the window.
fn count_block(number: usize) {
    if read8(CHINTFLAG) & INT_TCMPL != 0 {
        write8(CHINTFLAG, INT_TCMPL);
        BLOCKS[number].fetch_add(1, Ordering::AcqRel);
    }
}

macro_rules! vectors {
    ($($vector:ident = $channels:expr),*) => {
        $(
            #[cfg(feature = "rt")]
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn $vector() {
                on_interrupt($channels);
            }
        )*
    };
}

#[cfg(pic32cm_sg)]
vectors!(DMAC_TCMPL0 = 0..1, DMAC_TCMPL1 = 1..2, DMAC_TCMPL2 = 2..3, DMAC_TCMPL3 = 3..4, DMAC_TCMPL4 = 4..CHANNELS);
#[cfg(pic32cm_pl)]
vectors!(DMAC = 0..CHANNELS);

/// Stops channel `number` and resets it: a channel being disabled finishes
/// its beat in flight and writes its descriptor back first, ENABLE reading 1
/// until then. With the channel in the window.
fn stop_in_window() {
    write8(CHCTRLA, 0);
    while read8(CHCTRLA) & CHCTRLA_ENABLE != 0 {}
    write8(CHCTRLA, CHCTRLA_SWRST);
    while read8(CHCTRLA) & CHCTRLA_SWRST != 0 {}
    write8(CHINTENCLR, INT_ALL);
    write8(CHINTFLAG, INT_ALL);
}

fn stop(number: usize) {
    with_channel(number, stop_in_window);
    compiler_fence(Ordering::SeqCst);
}

/// Whether channel `number` is still moving: enabled, and not suspended.
fn running(number: usize) -> bool {
    with_channel(number, || read8(CHCTRLA) & CHCTRLA_ENABLE != 0 && read8(CHINTFLAG) & INT_SUSP == 0)
}

/// Where channel `number` is: the block in its write-back entry (named by
/// that entry's next-descriptor address) and that block's beats still to
/// move, from ACTIVE while the channel is the controller's active one, else
/// from the write-back entry.
fn position(number: usize) -> (u32, u16) {
    let entry = WRITE_BACK.entry(number);
    loop {
        // SAFETY: the controller's write-back entry for the channel, read
        // as it may be changing.
        let next = unsafe { read_volatile(addr_of!((*entry).descaddr)) };
        let active = read32(DMAC_BASE + ACTIVE);
        // SAFETY: as above.
        let written_back = unsafe { read_volatile(addr_of!((*entry).btcnt)) };
        // SAFETY: as above.
        if unsafe { read_volatile(addr_of!((*entry).descaddr)) } != next {
            // A new block came meanwhile: again.
            continue;
        }
        let ours = active & ACTIVE_ABUSY != 0 && ((active >> ACTIVE_ID_SHIFT) & ACTIVE_ID_MASK) as usize == number;
        let count = if ours { u16::try_from(active >> ACTIVE_BTCNT_SHIFT).unwrap_or(0) } else { written_back };
        return (next, count);
    }
}

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
    source_steps: bool,
    destination_steps: bool,
    /// BTCTRL.BEATSIZE: bytes a beat, `1 << beat_size`.
    beat_size: u16,
    beats: usize,
    /// The trigger: 0 for software, then the whole block on one.
    trigger: u8,
}

impl<'a> Transfer<'a> {
    /// Copies `source` into `destination`, which must be as long, on
    /// `channel`; the block moves on one software trigger, in the widest
    /// beats the two addresses' alignment and the length allow.
    ///
    /// # Panics
    ///
    /// If the slices' lengths differ, or the block is over [`MAX_BLOCK`]
    /// bytes or empty.
    pub fn new_copy<W: Word>(channel: Peri<'a, impl Channel>, source: &'a [W], destination: &'a mut [W]) -> Self {
        assert_eq!(source.len(), destination.len(), "source and destination differ in length");
        let bytes = core::mem::size_of_val(source);
        assert!(bytes > 0 && bytes <= MAX_BLOCK, "a block is 1 to 65,535 bytes");
        let (from, to) = (source.as_ptr() as usize, destination.as_mut_ptr() as usize);
        let beat_size = match (from | to | bytes) & 3 {
            0 => 2,
            2 => 1,
            _ => 0,
        };
        Self::start(
            channel.into(),
            &Setup {
                source: from,
                destination: to,
                source_steps: true,
                destination_steps: true,
                beat_size,
                beats: bytes >> beat_size,
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
    /// If the block is over [`MAX_BLOCK`] words or empty.
    pub unsafe fn new_read<W: Word>(channel: Peri<'a, impl Channel>, request: Request, register: *const W, buffer: &'a mut [W]) -> Self {
        Self::start(
            channel.into(),
            &Setup {
                source: register as usize,
                destination: buffer.as_mut_ptr() as usize,
                source_steps: false,
                destination_steps: true,
                beat_size: W::BEATSIZE,
                beats: buffer.len(),
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
    /// If the block is over [`MAX_BLOCK`] words or empty.
    pub unsafe fn new_write<W: Word>(channel: Peri<'a, impl Channel>, request: Request, buffer: &'a [W], register: *mut W) -> Self {
        Self::start(
            channel.into(),
            &Setup {
                source: buffer.as_ptr() as usize,
                destination: register as usize,
                source_steps: true,
                destination_steps: false,
                beat_size: W::BEATSIZE,
                beats: buffer.len(),
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
    /// If `count` is over [`MAX_BLOCK`] or 0.
    pub unsafe fn new_write_repeated<W: Word>(channel: Peri<'a, impl Channel>, request: Request, word: &'a W, count: usize, register: *mut W) -> Self {
        Self::start(
            channel.into(),
            &Setup {
                source: core::ptr::from_ref(word) as usize,
                destination: register as usize,
                source_steps: false,
                destination_steps: false,
                beat_size: W::BEATSIZE,
                beats: count,
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
    /// If `count` is over [`MAX_BLOCK`] or 0.
    pub unsafe fn new_read_discard<W: Word>(channel: Peri<'a, impl Channel>, request: Request, register: *const W, count: usize) -> Self {
        Self::start(
            channel.into(),
            &Setup {
                source: register as usize,
                destination: SINK.0.get() as usize,
                source_steps: false,
                destination_steps: false,
                beat_size: W::BEATSIZE,
                beats: count,
                trigger: request.0,
            },
        )
    }

    fn start(channel: Peri<'a, AnyChannel>, setup: &Setup) -> Self {
        assert!(setup.beats > 0 && setup.beats <= MAX_BLOCK, "a block is 1 to 65,535 beats");
        start_controller();
        let number = channel.number();
        let bytes = setup.beats << setup.beat_size;
        let end = |address: usize, steps: bool| u32::try_from(if steps { address + bytes } else { address }).unwrap_or(0);
        let mut btctrl = BTCTRL_VALID | BTCTRL_BLOCKACT_INT | (setup.beat_size << BTCTRL_BEATSIZE_SHIFT);
        if setup.source_steps {
            btctrl |= BTCTRL_SRCINC;
        }
        if setup.destination_steps {
            btctrl |= BTCTRL_DSTINC;
        }
        let descriptor = Descriptor {
            btctrl,
            btcnt: u16::try_from(setup.beats).unwrap_or(0),
            srcaddr: end(setup.source, setup.source_steps),
            dstaddr: end(setup.destination, setup.destination_steps),
            descaddr: 0,
        };
        let software = setup.trigger == 0;
        let trigger_action = if software { TRIGACT_BLOCK } else { TRIGACT_BEAT };
        stop(number);
        FIRST.set(number, descriptor);
        // The write-back entry as the controller will make it, so that
        // `remaining` reads right before the first beat.
        WRITE_BACK.set(number, descriptor);
        // The descriptor and the buffers' contents must be in memory before
        // the channel reads them.
        fence(Ordering::SeqCst);
        with_channel(number, || {
            write32(DMAC_BASE + CHCTRLB, (u32::from(setup.trigger) << CHCTRLB_TRIGSRC_SHIFT) | (trigger_action << CHCTRLB_TRIGACT_SHIFT));
            write8(CHINTENSET, INT_ALL);
            write8(CHCTRLA, CHCTRLA_ENABLE);
            if software {
                write32(DMAC_BASE + SWTRIGCTRL, read32(DMAC_BASE + SWTRIGCTRL) | 1 << number);
            }
        });
        Self { channel, _buffers: PhantomData }
    }

    /// Whether the block is still moving.
    #[must_use]
    pub fn is_running(&self) -> bool {
        running(self.channel.number())
    }

    /// Bytes still to move.
    #[must_use]
    pub fn remaining(&self) -> usize {
        let number = self.channel.number();
        let (_, beats) = position(number);
        // SAFETY: the channel's first descriptor, which only `start` writes.
        let btctrl = unsafe { read_volatile(addr_of!((*FIRST.entry(number)).btctrl)) };
        usize::from(beats) << ((btctrl >> BTCTRL_BEATSIZE_SHIFT) & 3)
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
        let flags = with_channel(self.channel.number(), || read8(CHINTFLAG));
        if flags & INT_TERR != 0 {
            Err(Error::Bus)
        } else if flags & INT_SUSP != 0 {
            Err(Error::Aborted)
        } else {
            Ok(())
        }
    }
}

impl Future for Transfer<'_> {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let number = self.channel.number();
        WAKERS[number].register(cx.waker());
        // Unmasked again before the check, so an end after it still wakes.
        with_channel(number, || write8(CHINTENSET, INT_ALL));
        if running(number) { Poll::Pending } else { Poll::Ready(self.outcome()) }
    }
}

impl Drop for Transfer<'_> {
    fn drop(&mut self) {
        stop(self.channel.number());
    }
}

/// A ring buffer a channel fills continuously from a peripheral's register,
/// as Embassy's `ReadableRingBuffer`: two descriptors, each half the ring,
/// linked to each other, bring the channel back to the ring's start with no
/// CPU in between, and each half's end wakes the reader. Reads take what has
/// come since the last; a reader a whole ring behind has lost bytes,
/// [`Overrun`].
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
        assert!(!ring.is_empty() && ring.len() <= MAX_BLOCK, "a ring is 1 to 65,535 bytes");
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

    /// The first half's length: all of a one-byte ring, which then has one
    /// descriptor linked to itself.
    fn first_len(&self) -> usize {
        if self.len > 1 { self.len / 2 } else { self.len }
    }

    /// Starts filling the ring from its start.
    pub fn start(&mut self) {
        start_controller();
        let number = self.channel.number();
        stop(number);
        let ring = self.ring as usize;
        let first_len = self.first_len();
        let block = |at: usize, len: usize, next: u32| Descriptor {
            btctrl: BTCTRL_VALID | BTCTRL_BLOCKACT_INT | BTCTRL_DSTINC,
            btcnt: u16::try_from(len).unwrap_or(0),
            srcaddr: u32::try_from(self.register).unwrap_or(0),
            dstaddr: u32::try_from(ring + at + len).unwrap_or(0),
            descaddr: next,
        };
        let first = if first_len == self.len {
            block(0, self.len, FIRST.address(number))
        } else {
            SECOND.set(number, block(first_len, self.len - first_len, FIRST.address(number)));
            block(0, first_len, SECOND.address(number))
        };
        FIRST.set(number, first);
        WRITE_BACK.set(number, first);
        BLOCKS[number].store(0, Ordering::Release);
        CIRCULAR[number].store(true, Ordering::Release);
        self.read = 0;
        fence(Ordering::SeqCst);
        with_channel(number, || {
            write32(DMAC_BASE + CHCTRLB, (u32::from(self.request.0) << CHCTRLB_TRIGSRC_SHIFT) | (TRIGACT_BEAT << CHCTRLB_TRIGACT_SHIFT));
            write8(CHINTENSET, INT_ALL);
            write8(CHCTRLA, CHCTRLA_ENABLE);
        });
        self.running = true;
    }

    /// Stops filling the ring.
    pub fn stop(&mut self) {
        let number = self.channel.number();
        CIRCULAR[number].store(false, Ordering::Release);
        stop(number);
        self.running = false;
    }

    /// Whether it is running.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Bytes written into the ring so far, all laps.
    ///
    /// The blocks counted give a floor, never more than the truth; the
    /// write-back entry says where in the ring the channel is, which fixes
    /// the bytes past the floor, less than a ring's worth.
    fn written(&self) -> u64 {
        let number = self.channel.number();
        let len = self.len as u64;
        let first_len = self.first_len() as u64;
        critical_section::with(|_| {
            with_channel(number, || count_block(number));
            let blocks = u64::from(BLOCKS[number].load(Ordering::Acquire));
            let (next, count) = position(number);
            let count = u64::from(count);
            let at = if first_len == len {
                len - count
            } else if next == SECOND.address(number) {
                // In the first half, the second next.
                first_len - count
            } else {
                len - count
            };
            let floor = if first_len == len { blocks * len } else { blocks / 2 * len + blocks % 2 * first_len };
            let lap = floor - floor % len;
            let mut written = lap + at % len;
            if written < floor {
                written += len;
            }
            written
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

    /// Registers `waker` for the ends of the ring's halves.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(pic32cm_sg)]
    fn sercom_requests() {
        // Table 7-1 / the ATDF's DMAC_ID_RX and _TX.
        assert_eq!(Request::sercom_rx(0).index(), 4);
        assert_eq!(Request::sercom_tx(3).index(), 11);
        assert_eq!(Request::sercom_rx(4).index(), 24);
        assert_eq!(Request::sercom_tx(5).index(), 27);
    }

    #[test]
    #[cfg(pic32cm_pl)]
    fn sercom_requests() {
        // The ATDF's DMAC_ID_RX and _TX.
        assert_eq!(Request::sercom_rx(0).index(), 1);
        assert_eq!(Request::sercom_tx(0).index(), 2);
        assert_eq!(Request::sercom_rx(1).index(), 3);
        assert_eq!(Request::sercom_tx(1).index(), 4);
    }

    #[test]
    fn descriptor_layout() {
        assert_eq!(size_of::<Descriptor>(), 16);
        assert_eq!(core::mem::align_of::<Descriptor>(), 16);
        assert_eq!(size_of::<Section>(), 16 * CHANNELS);
    }
}

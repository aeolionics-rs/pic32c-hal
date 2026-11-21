// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The full Bosch `M_CAN` (3.2.x): its registers, as Bosch lays them out
//! from offset 0 (checked against Microchip's `can.h` for the PIC32CK SG01,
//! DS60001795 chapter 39), and the message RAM this driver lays out for it.
//!
//! Microchip's integration puts `MRCFG` at 0x08, where Bosch has `CUST`: its
//! `OFFSET` field supplies the upper 16 bits of every message RAM address,
//! and the start-address fields (`F0SA`, `EFSA`, `TBSA`, …) the lower 16.

use core::cell::UnsafeCell;
use core::ptr::NonNull;

/// One 32-bit memory-mapped register, or message RAM word.
#[repr(transparent)]
pub(crate) struct Reg(UnsafeCell<u32>);

impl Reg {
    pub(crate) const fn new() -> Self {
        Self(UnsafeCell::new(0))
    }

    pub(crate) fn read(&self) -> u32 {
        // SAFETY: memory-mapped I/O or message RAM, valid for as long as its
        // block is.
        unsafe { self.0.get().read_volatile() }
    }

    pub(crate) fn write(&self, value: u32) {
        // SAFETY: as for `read`.
        unsafe { self.0.get().write_volatile(value) }
    }

    pub(crate) fn modify(&self, f: impl FnOnce(u32) -> u32) {
        self.write(f(self.read()));
    }
}

/// The `M_CAN` register block.
#[repr(C)]
pub(crate) struct RegisterBlock {
    pub(crate) crel: Reg,
    pub(crate) endn: Reg,
    /// Microchip's message RAM configuration (Bosch's `CUST`).
    pub(crate) mrcfg: Reg,
    pub(crate) dbtp: Reg,
    pub(crate) test: Reg,
    pub(crate) rwd: Reg,
    pub(crate) cccr: Reg,
    pub(crate) nbtp: Reg,
    pub(crate) tscc: Reg,
    pub(crate) tscv: Reg,
    pub(crate) tocc: Reg,
    pub(crate) tocv: Reg,
    _reserved0: [Reg; 4],
    pub(crate) ecr: Reg,
    pub(crate) psr: Reg,
    pub(crate) tdcr: Reg,
    _reserved1: Reg,
    pub(crate) ir: Reg,
    pub(crate) ie: Reg,
    pub(crate) ils: Reg,
    pub(crate) ile: Reg,
    _reserved2: [Reg; 8],
    pub(crate) gfc: Reg,
    pub(crate) sidfc: Reg,
    pub(crate) xidfc: Reg,
    _reserved3: Reg,
    pub(crate) xidam: Reg,
    pub(crate) hpms: Reg,
    pub(crate) ndat1: Reg,
    pub(crate) ndat2: Reg,
    pub(crate) rxf0c: Reg,
    pub(crate) rxf0s: Reg,
    pub(crate) rxf0a: Reg,
    pub(crate) rxbc: Reg,
    pub(crate) rxf1c: Reg,
    pub(crate) rxf1s: Reg,
    pub(crate) rxf1a: Reg,
    pub(crate) rxesc: Reg,
    pub(crate) txbc: Reg,
    pub(crate) txfqs: Reg,
    pub(crate) txesc: Reg,
    pub(crate) txbrp: Reg,
    pub(crate) txbar: Reg,
    pub(crate) txbcr: Reg,
    pub(crate) txbto: Reg,
    pub(crate) txbcf: Reg,
    pub(crate) txbtie: Reg,
    pub(crate) txbcie: Reg,
    _reserved4: [Reg; 2],
    pub(crate) txefc: Reg,
    pub(crate) txefs: Reg,
    pub(crate) txefa: Reg,
}

const _: () = {
    assert!(core::mem::offset_of!(RegisterBlock, mrcfg) == 0x08);
    assert!(core::mem::offset_of!(RegisterBlock, cccr) == 0x18);
    assert!(core::mem::offset_of!(RegisterBlock, tocv) == 0x2C);
    assert!(core::mem::offset_of!(RegisterBlock, ecr) == 0x40);
    assert!(core::mem::offset_of!(RegisterBlock, ir) == 0x50);
    assert!(core::mem::offset_of!(RegisterBlock, ile) == 0x5C);
    assert!(core::mem::offset_of!(RegisterBlock, gfc) == 0x80);
    assert!(core::mem::offset_of!(RegisterBlock, xidam) == 0x90);
    assert!(core::mem::offset_of!(RegisterBlock, rxf0c) == 0xA0);
    assert!(core::mem::offset_of!(RegisterBlock, rxesc) == 0xBC);
    assert!(core::mem::offset_of!(RegisterBlock, txbc) == 0xC0);
    assert!(core::mem::offset_of!(RegisterBlock, txbcie) == 0xE4);
    assert!(core::mem::offset_of!(RegisterBlock, txefc) == 0xF0);
    assert!(core::mem::offset_of!(RegisterBlock, txefa) == 0xF8);
};

/// Transmit buffers, all in the transmit FIFO: the transmissions
/// outstanding at once. Up to 32; eight is room enough for a fast-packet
/// burst's frames to queue while one is on the bus.
pub(crate) const TX_BUFFERS: usize = 8;
/// Tx event FIFO elements, one per transmit buffer.
pub(crate) const TX_EVENTS: usize = TX_BUFFERS;
/// Receive FIFO 0's elements: 16 frames, 1.9 ms of a 250 kbit/s bus at its
/// densest (classic frames of no data, 47 bits plus 3 of intermission).
pub(crate) const RX_FIFO: usize = 16;
/// The message RAM types' default room for standard (11-bit) acceptance
/// filters, one word each.
pub const STANDARD_FILTERS: usize = 8;
/// Their default room for extended (29-bit) acceptance filters, two words
/// each: more of them, as J1939 and protocols on it use extended identifiers
/// only.
pub const EXTENDED_FILTERS: usize = 16;

/// The same counts as the configuration registers take them.
#[expect(clippy::cast_possible_truncation, reason = "each is at most 64")]
pub(crate) const COUNTS: (u32, u32, u32) = (TX_BUFFERS as u32, TX_EVENTS as u32, RX_FIFO as u32);
const _: () = assert!(TX_BUFFERS <= 32 && TX_EVENTS <= 32 && RX_FIFO <= 64);

/// A receive or transmit element: two header words and `W` data words, 8
/// bytes (`W` 2, `RXESC`/`TXESC` code 0) for classic channels or 64 (`W` 16,
/// code [`ESC_64`]) for FD ones.
#[repr(C)]
pub(crate) struct Element<const W: usize> {
    pub(crate) header: [Reg; 2],
    pub(crate) data: [Reg; W],
}

impl<const W: usize> Element<W> {
    const fn new() -> Self {
        Self {
            header: [Reg::new(), Reg::new()],
            data: [const { Reg::new() }; W],
        }
    }
}

/// The sections of a message RAM with `W` data words per element: the
/// standard and extended filter lists, receive FIFO 0, the Tx event FIFO and
/// the transmit buffers.
#[repr(C)]
pub(crate) struct Sections<const W: usize, const S: usize, const E: usize> {
    standard_filters: [Reg; S],
    extended_filters: [[Reg; 2]; E],
    rx_fifo: [Element<W>; RX_FIFO],
    tx_events: [[Reg; 2]; TX_EVENTS],
    tx_buffers: [Element<W>; TX_BUFFERS],
}

impl<const W: usize, const S: usize, const E: usize> Sections<W, S, E> {
    const fn new() -> Self {
        // The controller's limits: 128 standard filter elements, 64 extended.
        const { assert!(S <= 128 && E <= 64, "at most 128 standard and 64 extended filters") };
        Self {
            standard_filters: [const { Reg::new() }; S],
            extended_filters: [const { [Reg::new(), Reg::new()] }; E],
            rx_fifo: [const { Element::new() }; RX_FIFO],
            tx_events: [const { [Reg::new(), Reg::new()] }; TX_EVENTS],
            tx_buffers: [const { Element::new() }; TX_BUFFERS],
        }
    }

    fn layout(&'static self, esc: u32) -> Ram {
        Ram {
            base: core::ptr::from_ref(self) as usize,
            size: core::mem::size_of::<Self>(),
            standard_filters: core::ptr::from_ref(&self.standard_filters) as usize,
            extended_filters: core::ptr::from_ref(&self.extended_filters) as usize,
            standard_capacity: S,
            extended_capacity: E,
            rx_fifo: core::ptr::from_ref(&self.rx_fifo) as usize,
            tx_events: core::ptr::from_ref(&self.tx_events) as usize,
            tx_buffers: core::ptr::from_ref(&self.tx_buffers) as usize,
            data_words: W,
            esc,
        }
    }
}

/// A controller's message RAM for a classic channel: eight-byte data fields,
/// with room for `STANDARD` standard and `EXTENDED` extended acceptance
/// filters (by default 8 and 16: 608 bytes in all), so the smallest parts (a
/// SAM C21's 32 KiB of SRAM, say) lose little to it; a build that filters
/// extended identifiers only might take `ClassicMessageRam<0, 16>`. The application declares one per
/// controller in SRAM (a `static_cell::ConstStaticCell`, say), and gives it
/// to [`Can::new`](crate::can::Can::new).
///
/// It must sit inside one 64 KiB block, as every address the controller
/// forms shares `MRCFG.OFFSET`'s upper 16 bits: [`Can::new`](crate::can::Can::new)
/// checks, rather than an alignment that would waste RAM to guarantee it.
#[repr(C)]
pub struct ClassicMessageRam<const STANDARD: usize = STANDARD_FILTERS, const EXTENDED: usize = EXTENDED_FILTERS>(Sections<2, STANDARD, EXTENDED>);

/// A controller's message RAM for an FD channel (or a classic one): 64-byte
/// data fields, room for any frame (by default 1,952 bytes with the filter
/// lists). As [`ClassicMessageRam`] otherwise.
#[repr(C)]
pub struct FdMessageRam<const STANDARD: usize = STANDARD_FILTERS, const EXTENDED: usize = EXTENDED_FILTERS>(Sections<16, STANDARD, EXTENDED>);

const _: () = assert!(core::mem::size_of::<ClassicMessageRam>() == 608);
const _: () = assert!(core::mem::size_of::<ClassicMessageRam<0, 16>>() == 576);
const _: () = assert!(core::mem::size_of::<FdMessageRam>() == 1952);

// SAFETY: the controller and the one channel that owns the RAM are its only
// users, the channel touching it with the critical section held.
unsafe impl<const S: usize, const E: usize> Sync for ClassicMessageRam<S, E> {}
// SAFETY: as above.
unsafe impl<const S: usize, const E: usize> Sync for FdMessageRam<S, E> {}

impl<const S: usize, const E: usize> ClassicMessageRam<S, E> {
    /// Message RAM for one controller, zeroed.
    #[must_use]
    pub const fn new() -> Self {
        Self(Sections::new())
    }
}

impl<const S: usize, const E: usize> FdMessageRam<S, E> {
    /// Message RAM for one controller, zeroed.
    #[must_use]
    pub const fn new() -> Self {
        Self(Sections::new())
    }
}

impl<const S: usize, const E: usize> Default for ClassicMessageRam<S, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const S: usize, const E: usize> Default for FdMessageRam<S, E> {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<const S: usize, const E: usize> Sealed for super::ClassicMessageRam<S, E> {}
    impl<const S: usize, const E: usize> Sealed for super::FdMessageRam<S, E> {}
}

/// Message RAM for [`Can::new`](crate::can::Can::new):
/// [`ClassicMessageRam`] or [`FdMessageRam`].
pub trait MessageRam: sealed::Sealed + Sync + 'static {
    #[doc(hidden)]
    fn layout(&'static self) -> Ram;
}

impl<const S: usize, const E: usize> MessageRam for ClassicMessageRam<S, E> {
    fn layout(&'static self) -> Ram {
        self.0.layout(0)
    }
}

impl<const S: usize, const E: usize> MessageRam for FdMessageRam<S, E> {
    fn layout(&'static self) -> Ram {
        self.0.layout(ESC_64)
    }
}

/// Where a message RAM's sections are, and its element size: what the
/// driver needs of either kind.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Ram {
    base: usize,
    size: usize,
    standard_filters: usize,
    extended_filters: usize,
    /// Filter elements each list has room for.
    standard_capacity: usize,
    extended_capacity: usize,
    rx_fifo: usize,
    tx_events: usize,
    tx_buffers: usize,
    /// Data words per element: 2 or 16.
    data_words: usize,
    /// `RXESC`/`TXESC`'s data field size code.
    esc: u32,
}

impl Ram {
    /// The address's lower 16 bits, as the start-address fields take it.
    #[expect(clippy::cast_possible_truncation, reason = "sixteen bits")]
    const fn low(address: usize) -> u32 {
        (address & 0xFFFC) as u32
    }

    /// `MRCFG.OFFSET`: the message RAM's 64 KiB block of SRAM, which the
    /// controller addresses as `0x2000_0000 + (OFFSET << 16)` (DS60001795
    /// 39.8.3), so bits 23:16 of its address.
    #[expect(clippy::cast_possible_truncation, reason = "eight bits")]
    pub(crate) const fn offset(&self) -> u32 {
        ((self.base >> 16) & 0xFF) as u32
    }

    /// Whether the controller can reach it: in system SRAM, at `0x2000_0000`
    /// and the 16 MiB above, the reach of `MRCFG`'s eight bits, and inside
    /// one 64 KiB block.
    pub(crate) const fn is_reachable(&self) -> bool {
        self.base >> 24 == 0x20 && self.base >> 16 == (self.base + self.size - 1) >> 16
    }

    /// Filter elements the lists have room for: standard, extended.
    pub(crate) const fn filter_capacity(&self) -> (usize, usize) {
        (self.standard_capacity, self.extended_capacity)
    }

    pub(crate) const fn standard_filters_start(&self) -> u32 {
        Self::low(self.standard_filters)
    }

    pub(crate) const fn extended_filters_start(&self) -> u32 {
        Self::low(self.extended_filters)
    }

    /// Standard filter `index`.
    pub(crate) fn standard_filter(&self, index: usize) -> &'static Reg {
        debug_assert!(index < self.standard_capacity);
        Self::word(self.standard_filters + 4 * index, 0)
    }

    /// Word `word` (0 or 1) of extended filter `index`.
    pub(crate) fn extended_filter(&self, index: usize, word: usize) -> &'static Reg {
        debug_assert!(index < self.extended_capacity && word < 2);
        Self::word(self.extended_filters + 8 * index, word)
    }

    /// Whether its elements hold FD frames' 64 bytes.
    pub(crate) const fn holds_fd(&self) -> bool {
        self.data_words == 16
    }

    pub(crate) const fn esc(&self) -> u32 {
        self.esc
    }

    pub(crate) const fn data_words(&self) -> usize {
        self.data_words
    }

    pub(crate) const fn rx_fifo_start(&self) -> u32 {
        Self::low(self.rx_fifo)
    }

    pub(crate) const fn tx_events_start(&self) -> u32 {
        Self::low(self.tx_events)
    }

    pub(crate) const fn tx_buffers_start(&self) -> u32 {
        Self::low(self.tx_buffers)
    }

    const fn element_bytes(&self) -> usize {
        4 * (2 + self.data_words)
    }

    /// Word `word` of an element at `address`.
    fn word(address: usize, word: usize) -> &'static Reg {
        // SAFETY: inside a message RAM that lives as long as the program
        // (`Sections`), at a word of one of its elements.
        unsafe { &*((address + 4 * word) as *const Reg) }
    }

    /// Word `word` (headers first, then data) of receive FIFO 0's element
    /// `index`.
    pub(crate) fn rx_word(&self, index: usize, word: usize) -> &'static Reg {
        debug_assert!(index < RX_FIFO && word < 2 + self.data_words);
        Self::word(self.rx_fifo + index * self.element_bytes(), word)
    }

    /// The same, of transmit buffer `index`.
    pub(crate) fn tx_word(&self, index: usize, word: usize) -> &'static Reg {
        debug_assert!(index < TX_BUFFERS && word < 2 + self.data_words);
        Self::word(self.tx_buffers + index * self.element_bytes(), word)
    }

    /// Word `word` (0 or 1) of Tx event `index`.
    pub(crate) fn tx_event_word(&self, index: usize, word: usize) -> &'static Reg {
        debug_assert!(index < TX_EVENTS && word < 2);
        Self::word(self.tx_events + 8 * index, word)
    }
}

/// Pointers to a controller's registers and message RAM.
#[derive(Clone, Copy)]
pub(crate) struct Regs {
    regs: NonNull<RegisterBlock>,
    ram: Ram,
}

// SAFETY: the registers are only touched with the critical section held, or
// by the one channel that owns them.
unsafe impl Send for Regs {}

impl Regs {
    /// # Safety
    ///
    /// `regs` must be an `M_CAN`'s register block.
    pub(crate) unsafe fn new(regs: usize, ram: Ram) -> Self {
        // SAFETY: the caller gives its address, which is never null.
        Self {
            regs: unsafe { NonNull::new_unchecked(regs as *mut RegisterBlock) },
            ram,
        }
    }

    pub(crate) fn get(&self) -> &RegisterBlock {
        // SAFETY: a register block (see `new`), which lives for as long as the
        // program.
        unsafe { self.regs.as_ref() }
    }

    pub(crate) const fn ram(&self) -> Ram {
        self.ram
    }
}

pub(crate) const MRCFG_OFFSET_SHIFT: u32 = 16;

// CCCR: control.
pub(crate) const CCCR_INIT: u32 = 1 << 0;
pub(crate) const CCCR_CCE: u32 = 1 << 1;
pub(crate) const CCCR_MON: u32 = 1 << 5;
pub(crate) const CCCR_DAR: u32 = 1 << 6;
pub(crate) const CCCR_TEST: u32 = 1 << 7;
pub(crate) const CCCR_FDOE: u32 = 1 << 8;
pub(crate) const CCCR_BRSE: u32 = 1 << 9;

// DBTP: data bit timing.
pub(crate) const DBTP_TDC: u32 = 1 << 23;

// RXESC (F0DS, F1DS, RBDS) and TXESC (TBDS): a 64-byte data field.
pub(crate) const ESC_64: u32 = 0b111;

// TEST.
pub(crate) const TEST_LBCK: u32 = 1 << 4;

// ECR: error counters. REC counts to 127; RP says it has reached 128.
pub(crate) const ECR_RP: u32 = 1 << 15;

// PSR: protocol status.
pub(crate) const PSR_EP: u32 = 1 << 5;
/// Either error counter has reached the warning limit, 96.
pub(crate) const PSR_EW: u32 = 1 << 6;
pub(crate) const PSR_BO: u32 = 1 << 7;
/// ACT: 0 while synchronising, then idle, receiving or transmitting.
pub(crate) const PSR_ACT_MASK: u32 = 0b11 << 3;
pub(crate) const PSR_LEC_SHIFT: u32 = 0;
/// PSR.DLEC: the last error in an FD frame's data phase.
pub(crate) const PSR_DLEC_SHIFT: u32 = 8;

// TSCC: timestamp counter, counting bit times.
pub(crate) const TSCC_TSS_INTERNAL: u32 = 0b01;

// TOCC: timeout counter, on the timestamp counter's prescaler (bit times),
// in continuous mode (TOS 0).
pub(crate) const TOCC_ETOC: u32 = 1 << 0;
pub(crate) const TOCC_TOP_SHIFT: u32 = 16;

// IR / IE: interrupts (Bosch's numbering).
pub(crate) const IR_RF0N: u32 = 1 << 0;
pub(crate) const IR_RF0L: u32 = 1 << 3;
pub(crate) const IR_TCF: u32 = 1 << 10;
pub(crate) const IR_TEFN: u32 = 1 << 12;
pub(crate) const IR_TEFL: u32 = 1 << 15;
pub(crate) const IR_MRAF: u32 = 1 << 17;
pub(crate) const IR_TOO: u32 = 1 << 18;
pub(crate) const IR_EP: u32 = 1 << 23;
pub(crate) const IR_EW: u32 = 1 << 24;
pub(crate) const IR_BO: u32 = 1 << 25;
pub(crate) const IR_PEA: u32 = 1 << 27;
pub(crate) const IR_PED: u32 = 1 << 28;

// ILE: interrupt line enable.
pub(crate) const ILE_EINT0: u32 = 1 << 0;

// RXF0C, TXEFC, TXBC: sizes of the FIFOs whose start addresses they hold.
pub(crate) const RXF0C_F0S_SHIFT: u32 = 16;
pub(crate) const TXEFC_EFS_SHIFT: u32 = 16;
pub(crate) const TXBC_TFQS_SHIFT: u32 = 24;

// RXF0S: receive FIFO 0 status.
pub(crate) const RXFS_FFL_MASK: u32 = 0x7F;
pub(crate) const RXFS_FGI_SHIFT: u32 = 8;
pub(crate) const RXFS_FGI_MASK: u32 = 0x3F;

// TXFQS: transmit FIFO status.
pub(crate) const TXFQS_TFQPI_SHIFT: u32 = 16;
pub(crate) const TXFQS_TFQPI_MASK: u32 = 0x1F;
pub(crate) const TXFQS_TFQF: u32 = 1 << 21;

// TXEFS: Tx event FIFO status.
pub(crate) const TXEFS_EFFL_MASK: u32 = 0x3F;
pub(crate) const TXEFS_EFGI_SHIFT: u32 = 8;
pub(crate) const TXEFS_EFGI_MASK: u32 = 0x1F;

// SIDFC/XIDFC: the list's start (FLSSA/FLESA, bits 15:2) and size (LSS
// bits 23:16, LSE bits 22:16).
pub(crate) const FILTERS_SIZE_SHIFT: u32 = 16;
// GFC: standard (ANFS) and extended (ANFE) frames that match no filter
// rejected (2; 0 accepts them into FIFO 0); remote frames filtered like data
// ones (RRFS, RRFE 0).
pub(crate) const GFC_ANFS_REJECT: u32 = 0b10 << 4;
pub(crate) const GFC_ANFE_REJECT: u32 = 0b10 << 2;
// A standard filter element: SFT classic (2, filter and mask), SFEC (0
// disabled, 1 store in FIFO 0), SFID1 the identifier, SFID2 the mask.
pub(crate) const SFT_CLASSIC: u32 = 0b10 << 30;
pub(crate) const SFEC_FIFO0: u32 = 1 << 27;
pub(crate) const SFID1_SHIFT: u32 = 16;
// An extended filter element: word 0 EFEC (0 disabled, 1 store in FIFO 0)
// and EFID1 the identifier; word 1 EFT classic (2) and EFID2 the mask.
pub(crate) const EFEC_FIFO0: u32 = 1 << 29;
pub(crate) const EFT_CLASSIC: u32 = 0b10 << 30;

// Element header bits: word 0 (T0, R0, E0).
pub(crate) const H0_RTR: u32 = 1 << 29;
pub(crate) const H0_XTD: u32 = 1 << 30;
pub(crate) const H0_ESI: u32 = 1 << 31;
// Word 1 (T1, R1, E1).
pub(crate) const H1_DLC_SHIFT: u32 = 16;
/// BRS: the data phase at the data bit rate.
pub(crate) const H1_BRS: u32 = 1 << 20;
/// FDF: an FD frame.
pub(crate) const H1_FDF: u32 = 1 << 21;
pub(crate) const T1_EFC: u32 = 1 << 23;
pub(crate) const H1_MM_SHIFT: u32 = 24;

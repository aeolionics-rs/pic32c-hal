// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The Ethernet MAC (ETH, DS60001795 chapter 32: Cadence's GEM, as on the
//! SAM E5x/E7x GMAC), as a [`xarxa_driver::Driver`] (embassy-net's), in RMII
//! to an external PHY managed over MDIO.
//!
//! [`Ethernet::new`] takes the `ETH` peripheral, a
//! [`bind_interrupts!`](crate::bind_interrupts) type binding its vector
//! (`ETH`) to [`InterruptHandler`], the RMII pins ([`Pins`]: any the chip
//! offers for each signal, checked at compile time), the PHY's reset pin and
//! a [`PacketQueue`] for the DMA's descriptors. The PHY's 50 MHz reference
//! clock comes in on `REF_CLK` (PC00); the MAC also needs its two generic
//! clock channels fed, from the generator `new` is given. It finds the PHY's
//! address by scanning MDIO, restarts autonegotiation, and polls the link
//! about twice a second; the MAC's speed and duplex follow what was
//! negotiated.
//!
//! The frames live in the stack's packet pool: the DMA receives straight
//! into the buffers the stack gives it and sends straight from the ones it
//! hands over, so the pool's buffers must hold [`BUFFER_SIZE`] bytes, be
//! aligned to 8 (the DMA's bus width) and sit in SRAM, which the cache
//! doesn't cover (`StaticPool<1536, N, 8>` in a `static` does). One DMA
//! queue, whole frames per buffer, the FCS stripped, multicast through the
//! hash filter the stack sets, no checksum offload yet. Unlike the GEM's
//! usual ISR, this chip's is write-to-clear. The DMA goes round both rings
//! strictly in order, so the driver fills and empties them in order too.

use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{Ordering, compiler_fence};
use core::task::{Context, Waker};

use embassy_sync::waitqueue::AtomicWaker;
use embassy_time::{Duration, Instant, Timer};
use xarxa_driver::{Capabilities, DriverBuf, HardwareAddress, LinkState, NotSupported, RxWanted};

use crate::gpio::{Function, Level, Output, Pin as GpioPin, set_function};
use crate::interrupt::typelevel::{self, Binding, Handler};
use crate::interrupt::{Interrupt, InterruptExt};
use crate::{Peri, peripherals};

const ETH: usize = 0x4502_2000;
/// The generic clock channels (table 20-2), and their highest rate
/// (`fGCLK_ETH`, electrical characteristics).
const GCLK_CHANNEL_TX: usize = 41;
const GCLK_CHANNEL_TSU: usize = 42;
const MAX_GCLK_HZ: u32 = 25_000_000;
const CTRLA: usize = ETH;
const SYNCB: usize = ETH + 0x20;
const NCR: usize = ETH + 0x1000;
const NCFGR: usize = ETH + 0x1004;
const NSR: usize = ETH + 0x1008;
const UR: usize = ETH + 0x100C;
const DCFGR: usize = ETH + 0x1010;
const TSR: usize = ETH + 0x1014;
const RBQB: usize = ETH + 0x1018;
const TBQB: usize = ETH + 0x101C;
const RSR: usize = ETH + 0x1020;
const ISR: usize = ETH + 0x1024;
const IER: usize = ETH + 0x1028;
const IDR: usize = ETH + 0x102C;
const MAN: usize = ETH + 0x1034;
const HRB: usize = ETH + 0x1080;
const HRT: usize = ETH + 0x1084;
const SAB1: usize = ETH + 0x1088;
const SAT1: usize = ETH + 0x108C;

const CTRLA_ENABLE: u32 = 1 << 1;
const SYNCB_ENABLE: u32 = 1 << 1;
const NCR_RXEN: u32 = 1 << 2;
const NCR_TXEN: u32 = 1 << 3;
const NCR_MPE: u32 = 1 << 4;
const NCR_CLRSTAT: u32 = 1 << 5;
const NCR_TSTART: u32 = 1 << 9;
const NCFGR_SPD: u32 = 1 << 0;
const NCFGR_FD: u32 = 1 << 1;
const NCFGR_MTIHEN: u32 = 1 << 6;
const NCFGR_MAXFS: u32 = 1 << 8;
const NCFGR_RFCS: u32 = 1 << 17;
/// MDC = MCK / 64: 1.9 MHz from 120 MHz (802.3 allows 2.5).
const NCFGR_CLK_DIV64: u32 = 4 << 18;
/// The data bus width, "always 1" (64 bits).
const NCFGR_DBW64: u32 = 1 << 21;
const NSR_IDLE: u32 = 1 << 2;
/// TSR: transmit active.
const TSR_TXGO: u32 = 1 << 3;
/// DCFGR: INCR4 bursts, full RX and TX packet buffers, 1,536-byte receive
/// buffers (24 x 64).
const DCFGR_VALUE: u32 = 0x04 | (3 << 8) | (1 << 10) | (24 << 16);
/// ISR/IER: receive complete, RX used bit read, TX used bit read, transmit
/// underrun, retry limit, transmit complete, RX overrun, HRESP not OK.
const IRQ_RCOMP: u32 = 1 << 1;
const IRQ_RXUBR: u32 = 1 << 2;
const IRQ_TXUBR: u32 = 1 << 3;
const IRQ_TUR: u32 = 1 << 4;
const IRQ_RLEX: u32 = 1 << 5;
const IRQ_TCOMP: u32 = 1 << 7;
const IRQ_ROVR: u32 = 1 << 10;
const IRQ_HRESP: u32 = 1 << 11;
const IRQ_RX: u32 = IRQ_RCOMP | IRQ_RXUBR | IRQ_ROVR;
const IRQ_TX: u32 = IRQ_TCOMP | IRQ_TXUBR | IRQ_TUR | IRQ_RLEX;
const IRQS: u32 = IRQ_RX | IRQ_TX | IRQ_HRESP;
/// Frames `receive` hands out before it yields (returns `None` and wakes
/// itself): xarxa takes frames until `receive` says there are none, so a
/// flood would otherwise keep its poll, and with it the executor, busy for
/// as long as frames keep coming.
const RX_BUDGET: u8 = 8;

/// The size a pool buffer must have: the DMA's receive buffer size
/// (DCFGR.DRBS), a whole frame with room to spare.
pub const BUFFER_SIZE: usize = 1536;
/// The alignment a pool buffer must have: the DMA's bus width.
pub const BUFFER_ALIGN: usize = 8;
/// The largest frame the stack may hand over (no FCS).
const MTU: usize = 1514;

const RX_OWNED: u32 = 1 << 0;
const RX_WRAP: u32 = 1 << 1;
const RX_SOF: u32 = 1 << 14;
const RX_EOF: u32 = 1 << 15;
const RX_LENGTH: u32 = 0x1FFF;
const TX_USED: u32 = 1 << 31;
const TX_WRAP: u32 = 1 << 30;
const TX_LAST: u32 = 1 << 15;
const TX_LENGTH: u32 = 0x3FFF;

/// A DMA descriptor: an address word and a control/status word.
#[repr(C, align(8))]
struct Descriptor {
    address: u32,
    status: u32,
}

impl Descriptor {
    const fn new() -> Self {
        Self { address: 0, status: 0 }
    }

    fn address(&self) -> u32 {
        // SAFETY: a descriptor in the DMA's memory, read whole.
        unsafe { read_volatile(&raw const self.address) }
    }

    fn set_address(&mut self, value: u32) {
        // SAFETY: as for `address`.
        unsafe { write_volatile(&raw mut self.address, value) }
    }

    fn status(&self) -> u32 {
        // SAFETY: as for `address`.
        unsafe { read_volatile(&raw const self.status) }
    }

    fn set_status(&mut self, value: u32) {
        // SAFETY: as for `address`.
        unsafe { write_volatile(&raw mut self.status, value) }
    }
}

/// The DMA's descriptors, `TX` transmit and `RX` receive, and the pool
/// buffers they point at while the DMA has them. Give [`Ethernet::new`] a
/// `&'static mut` one (a `static` through `StaticCell`, say).
pub struct PacketQueue<const TX: usize, const RX: usize> {
    tx_descriptors: [Descriptor; TX],
    rx_descriptors: [Descriptor; RX],
    tx_buffers: [Option<DriverBuf>; TX],
    rx_buffers: [Option<DriverBuf>; RX],
}

impl<const TX: usize, const RX: usize> PacketQueue<TX, RX> {
    /// An empty queue.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            tx_descriptors: [const { Descriptor::new() }; TX],
            rx_descriptors: [const { Descriptor::new() }; RX],
            tx_buffers: [const { None }; TX],
            rx_buffers: [const { None }; RX],
        }
    }
}

impl<const TX: usize, const RX: usize> Default for PacketQueue<TX, RX> {
    fn default() -> Self {
        Self::new()
    }
}

/// The stack's waker: xarxa keeps one per driver, for receive, transmit and
/// the link alike.
static WAKER: AtomicWaker = AtomicWaker::new();

/// The ETH vector's handler: wakes the stack.
pub struct InterruptHandler {
    _private: (),
}

impl Handler<typelevel::ETH> for InterruptHandler {
    unsafe fn on_interrupt() {
        // ISR is write-to-clear on this chip (the GEM's usual ISR clears on
        // read); clear, then read back, as the datasheet asks.
        let fired = read(ISR);
        write(ISR, fired);
        let _ = read(ISR);
        if fired & (IRQ_RX | IRQ_HRESP) != 0 {
            write(RSR, 0xF);
            // Masked until the driver finds the ring empty: while frames
            // wait it's polling anyway, and a flood would otherwise raise
            // an interrupt per frame.
            write(IDR, IRQ_RX);
        }
        if fired & (IRQ_TX | IRQ_HRESP) != 0 {
            write(TSR, 0x17F);
        }
        if fired & IRQS != 0 {
            WAKER.wake();
        }
    }
}

fn read(address: usize) -> u32 {
    // SAFETY: an ETH register, read whole.
    unsafe { read_volatile(address as *const u32) }
}

fn write(address: usize, value: u32) {
    // SAFETY: an ETH register; this driver is its only writer.
    unsafe { write_volatile(address as *mut u32, value) }
}

/// A buffer's address as the DMA takes it.
fn dma_address(data: *const u8) -> u32 {
    #[expect(clippy::cast_possible_truncation, reason = "an SRAM address; addresses are 32 bits on the target")]
    let address = data as usize as u32;
    address
}

/// MDIO, clause 22, through ETH's `MAN` register (the management port must be
/// on: NCR.MPE).
fn mdio_read(phy: u8, register: u8) -> u16 {
    write(MAN, (1 << 30) | (2 << 28) | (u32::from(phy) << 23) | (u32::from(register) << 18) | (2 << 16));
    while read(NSR) & NSR_IDLE == 0 {}
    #[expect(clippy::cast_possible_truncation, reason = "the data field is 16 bits")]
    let data = read(MAN) as u16;
    data
}

fn mdio_write(phy: u8, register: u8, value: u16) {
    write(MAN, (1 << 30) | (1 << 28) | (u32::from(phy) << 23) | (u32::from(register) << 18) | (2 << 16) | u32::from(value));
    while read(NSR) & NSR_IDLE == 0 {}
}

/// The PHY's standard registers (802.3 clause 22).
const BMCR: u8 = 0;
const BMSR: u8 = 1;
const PHYID1: u8 = 2;
const PHYID2: u8 = 3;
const ANAR: u8 = 4;
const ANLPAR: u8 = 5;
const BMCR_ANENABLE: u16 = 1 << 12;
const BMCR_ANRESTART: u16 = 1 << 9;
const BMSR_LINK: u16 = 1 << 2;
const BMSR_ANCOMPLETE: u16 = 1 << 5;

/// The pin traits, one per RMII signal: each is implemented for the pins the
/// chip's pin multiplexing table offers for its signal, with the pin function
/// that selects it.
macro_rules! pin_traits {
    ($($(#[$doc:meta])* $name:ident;)*) => {
        $(
            $(#[$doc])*
            pub trait $name: GpioPin {
                #[doc(hidden)]
                fn function(&self) -> Function;
            }
        )*
    };
}

pin_traits! {
    /// A pin that can be `REF_CLK`, the PHY's 50 MHz reference clock (the
    /// datasheet's `ETH_TX_CLK`, an input).
    RefClkPin;
    /// A pin that can be `MDC`, the management clock.
    MdcPin;
    /// A pin that can be `MDIO`, the management data.
    MdioPin;
    /// A pin that can be `CRS_DV`, carrier sense and receive data valid (the
    /// datasheet's `ETH_RXDV`).
    CrsDvPin;
    /// A pin that can be `RX_ER`, the receive error.
    RxErPin;
    /// A pin that can be `RXD0`.
    Rxd0Pin;
    /// A pin that can be `RXD1`.
    Rxd1Pin;
    /// A pin that can be `TX_EN`, the transmit enable.
    TxEnPin;
    /// A pin that can be `TXD0`.
    Txd0Pin;
    /// A pin that can be `TXD1`.
    Txd1Pin;
}

/// Each RMII signal's pins, by the signal's pin trait: pin, function.
macro_rules! impl_rmii_pins {
    ($($signal:ident: $($pin:ident $function:ident),*;)*) => {
        $(
            $(
                impl $crate::eth::$signal for $crate::peripherals::$pin {
                    fn function(&self) -> $crate::gpio::Function {
                        $crate::gpio::Function::$function
                    }
                }
            )*
        )*
    };
}

pub(crate) use impl_rmii_pins;

/// The RMII pins: the PHY's reference clock, the management pair, receive
/// and transmit. Each field takes any pin the chip offers for its signal
/// (its pin trait, from each chip's pin multiplexing table); the signals
/// have no I/O sets, so any mix will do.
///
/// The management pair on PA11 and PA12, the rest where the chip has them:
///
/// ```no_run
/// # use pic32c_hal::eth::Pins;
/// let p = pic32c_hal::init(pic32c_hal::Config::default());
/// let pins = Pins {
///     ref_clk: p.PC00,
///     mdc: p.PA11,
///     mdio: p.PA12,
///     crs_dv: p.PC10,
///     rx_er: p.PC09,
///     rxd0: p.PC07,
///     rxd1: p.PC06,
///     tx_en: p.PC11,
///     txd0: p.PC12,
///     txd1: p.PC13,
/// };
/// ```
///
/// MDC on PA12, which can only be MDIO, doesn't compile:
///
/// ```compile_fail
/// # use pic32c_hal::eth::Pins;
/// let p = pic32c_hal::init(pic32c_hal::Config::default());
/// let pins = Pins {
///     ref_clk: p.PC00,
///     mdc: p.PA12,
///     mdio: p.PC04,
///     crs_dv: p.PC10,
///     rx_er: p.PC09,
///     rxd0: p.PC07,
///     rxd1: p.PC06,
///     tx_en: p.PC11,
///     txd0: p.PC12,
///     txd1: p.PC13,
/// };
/// ```
pub struct Pins<'d, RefClk: RefClkPin, Mdc: MdcPin, Mdio: MdioPin, CrsDv: CrsDvPin, RxEr: RxErPin, Rxd0: Rxd0Pin, Rxd1: Rxd1Pin, TxEn: TxEnPin, Txd0: Txd0Pin, Txd1: Txd1Pin> {
    pub ref_clk: Peri<'d, RefClk>,
    pub mdc: Peri<'d, Mdc>,
    pub mdio: Peri<'d, Mdio>,
    pub crs_dv: Peri<'d, CrsDv>,
    pub rx_er: Peri<'d, RxEr>,
    pub rxd0: Peri<'d, Rxd0>,
    pub rxd1: Peri<'d, Rxd1>,
    pub tx_en: Peri<'d, TxEn>,
    pub txd0: Peri<'d, Txd0>,
    pub txd1: Peri<'d, Txd1>,
}

impl<RefClk: RefClkPin, Mdc: MdcPin, Mdio: MdioPin, CrsDv: CrsDvPin, RxEr: RxErPin, Rxd0: Rxd0Pin, Rxd1: Rxd1Pin, TxEn: TxEnPin, Txd0: Txd0Pin, Txd1: Txd1Pin> Pins<'_, RefClk, Mdc, Mdio, CrsDv, RxEr, Rxd0, Rxd1, TxEn, Txd0, Txd1> {
    /// Gives each pin to the MAC, on the pin function its signal is on.
    fn connect(&self) {
        set_function(&*self.ref_clk, self.ref_clk.function());
        set_function(&*self.mdc, self.mdc.function());
        set_function(&*self.mdio, self.mdio.function());
        set_function(&*self.crs_dv, self.crs_dv.function());
        set_function(&*self.rx_er, self.rx_er.function());
        set_function(&*self.rxd0, self.rxd0.function());
        set_function(&*self.rxd1, self.rxd1.function());
        set_function(&*self.tx_en, self.tx_en.function());
        set_function(&*self.txd0, self.txd0.function());
        set_function(&*self.txd1, self.txd1.function());
    }
}

/// Why [`Ethernet::new`] failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// No PHY answered on MDIO.
    NoPhy,
}

/// The receive ring. The DMA fills the descriptors in order and stops at
/// the first one software owns, which is also how an empty slot (no pool
/// buffer yet) holds it up.
struct RxRing<'d> {
    descriptors: &'d mut [Descriptor],
    buffers: &'d mut [Option<DriverBuf>],
    /// The next descriptor to receive from.
    next: usize,
    /// Where to start looking for an empty slot.
    fill: usize,
    /// Slots without a buffer.
    empty: usize,
}

impl RxRing<'_> {
    fn wrap(&self, index: usize) -> u32 {
        if index == self.descriptors.len() - 1 { RX_WRAP } else { 0 }
    }

    fn give(&mut self, mut buf: DriverBuf) {
        // The buffer as the stack gives it: no headroom, empty, so the
        // frame starts at the storage's start.
        let data = buf.storage_mut().as_mut_ptr();
        assert!(
            buf.capacity() >= BUFFER_SIZE && (data as usize).is_multiple_of(BUFFER_ALIGN),
            "eth: pool buffers must hold {BUFFER_SIZE} bytes, aligned to {BUFFER_ALIGN}"
        );
        let len = self.descriptors.len();
        let Some(index) = (0..len).map(|i| (self.fill + i) % len).find(|&i| self.buffers[i].is_none()) else {
            // More than `rx_wanted` asked for; back to the pool.
            return;
        };
        let address = dma_address(data);
        self.buffers[index] = Some(buf);
        compiler_fence(Ordering::SeqCst);
        self.descriptors[index].set_status(0);
        // Ownership last: the DMA may take it at once.
        self.descriptors[index].set_address(address | self.wrap(index));
        self.fill = (index + 1) % len;
        self.empty -= 1;
    }

    /// Whether the next descriptor holds a frame.
    fn ready(&self) -> bool {
        self.buffers[self.next].is_some() && self.descriptors[self.next].address() & RX_OWNED != 0
    }

    /// The next whole received frame, giving back to the DMA any buffers
    /// that hold part of a frame (a frame larger than a buffer, which the
    /// MTU rules out, or the tail of one after an overrun).
    fn receive(&mut self) -> Option<DriverBuf> {
        loop {
            let index = self.next;
            self.buffers[index].as_ref()?;
            let len = self.descriptors.len();
            let descriptor = &mut self.descriptors[index];
            let address = descriptor.address();
            if address & RX_OWNED == 0 {
                return None;
            }
            let status = descriptor.status();
            self.next = (index + 1) % len;
            if status & (RX_SOF | RX_EOF) == RX_SOF | RX_EOF {
                compiler_fence(Ordering::SeqCst);
                let mut buf = self.buffers[index].take()?;
                self.empty += 1;
                buf.set_len(((status & RX_LENGTH) as usize).min(BUFFER_SIZE));
                return Some(buf);
            }
            descriptor.set_address(address & !RX_OWNED);
        }
    }
}

/// The transmit ring. Each frame is one descriptor pointing into its pool
/// buffer, held until the DMA marks the descriptor used.
struct TxRing<'d> {
    descriptors: &'d mut [Descriptor],
    buffers: &'d mut [Option<DriverBuf>],
    /// The next descriptor to fill.
    next: usize,
    /// The oldest frame the DMA may still be sending.
    reclaim: usize,
    in_flight: usize,
}

impl TxRing<'_> {
    /// Gives the sent frames' buffers back to the pool.
    fn take_sent(&mut self) {
        while self.in_flight > 0 && self.descriptors[self.reclaim].status() & TX_USED != 0 {
            self.buffers[self.reclaim] = None;
            self.reclaim = (self.reclaim + 1) % self.descriptors.len();
            self.in_flight -= 1;
        }
    }

    /// Gives the sent frames' buffers back to the pool, and restarts the
    /// DMA if it stopped with frames still queued.
    fn reclaim(&mut self) {
        self.take_sent();
        // `transmit` starts the DMA for each frame, but a start written
        // while it is still finishing the last frame does nothing. Having
        // read the next descriptor as used, it stops, resting on the last
        // descriptor it sent, so a later start alone reads that one again
        // and stops at once: the frames queued after it wait until the
        // stack queues one more into that slot, which may be a TCP
        // retransmission 200 ms later, and then go out behind it. So with
        // the DMA idle and frames queued, point it at the oldest of them
        // (TBQB may be written while it's idle) and start it. Anything it
        // finished before going idle is marked used by then, so look once
        // more first: a frame it sent must not go out twice.
        if self.in_flight > 0 && read(TSR) & TSR_TXGO == 0 {
            cortex_m::asm::dsb();
            self.take_sent();
            if self.in_flight > 0 {
                write(TBQB, dma_address((&raw const self.descriptors[self.reclaim]).cast()));
                write(NCR, read(NCR) | NCR_TSTART);
            }
        }
    }

    fn can_transmit(&mut self) -> bool {
        self.reclaim();
        self.in_flight < self.descriptors.len()
    }

    fn transmit(&mut self, buf: DriverBuf) {
        let index = self.next;
        let length = buf.len().min(MTU);
        let descriptor = &mut self.descriptors[index];
        let wrap = descriptor.status() & TX_WRAP;
        descriptor.set_address(dma_address(buf.as_ptr()));
        self.buffers[index] = Some(buf);
        compiler_fence(Ordering::SeqCst);
        #[expect(clippy::cast_possible_truncation, reason = "at most the MTU")]
        descriptor.set_status(wrap | TX_LAST | (length as u32 & TX_LENGTH));
        compiler_fence(Ordering::SeqCst);
        write(NCR, read(NCR) | NCR_TSTART);
        self.next = (index + 1) % self.descriptors.len();
        self.in_flight += 1;
    }
}

/// The MAC, as a [`xarxa_driver::Driver`].
pub struct Ethernet<'d> {
    rx: RxRing<'d>,
    tx: TxRing<'d>,
    mac: [u8; 6],
    phy: u8,
    phy_id: u32,
    link: Option<(bool, bool)>,
    /// Frames handed out since the last yield (see [`RX_BUDGET`]).
    rx_burst: u8,
    /// The multicast hash last set (`HRT << 32 | HRB`).
    hash: u64,
    next_poll: Instant,
    poll: Option<Timer>,
    /// While frames wait to be sent: the next look at the transmit DMA (see
    /// `register_waker`).
    tx_watch: Option<Timer>,
    _reset: Output<'d>,
    _lifetime: PhantomData<&'d ()>,
}

impl<'d> Ethernet<'d> {
    /// The MAC on `pins`, its PHY out of reset (`phy_reset`, active low),
    /// with `queue`'s descriptors and the hardware address `mac`; its two
    /// generic clock channels (`GCLK_ETH_TX`, `GCLK_ETH_TSU`) from
    /// `generator`.
    ///
    /// # Errors
    ///
    /// [`Error::NoPhy`] if no PHY answers on MDIO.
    ///
    /// # Panics
    ///
    /// If `generator` doesn't run, or runs over 25 MHz.
    #[allow(clippy::needless_pass_by_value, reason = "the MAC has the pins and the peripheral for 'd")]
    pub async fn new<const TX: usize, const RX: usize>(
        eth: Peri<'d, peripherals::ETH>,
        _irqs: impl Binding<typelevel::ETH, InterruptHandler> + 'd,
        pins: Pins<'d, impl RefClkPin, impl MdcPin, impl MdioPin, impl CrsDvPin, impl RxErPin, impl Rxd0Pin, impl Rxd1Pin, impl TxEnPin, impl Txd0Pin, impl Txd1Pin>,
        phy_reset: Peri<'d, impl GpioPin>,
        queue: &'d mut PacketQueue<TX, RX>,
        mac: [u8; 6],
        generator: usize,
    ) -> Result<Self, Error> {
        let _ = eth;
        // The PHY reset, then out: its 50 MHz reference clock (REF_CLK),
        // which the MAC's enable synchronises with, runs only out of reset.
        let mut reset = Output::new(phy_reset, Level::Low);
        Timer::after(Duration::from_millis(10)).await;
        reset.set_high();
        Timer::after(Duration::from_millis(100)).await;
        pins.connect();

        // GCLK_ETH_TX and GCLK_ETH_TSU: the MAC's enable doesn't
        // synchronise without them, even in RMII with REF_CLK from the PHY.
        let hz = crate::clocks::generator_hz(generator);
        assert!(hz.is_some_and(|hz| hz <= MAX_GCLK_HZ), "ETH: generator {generator} must run, at 25 MHz or less; it's {hz:?}");
        crate::clocks::enable_channel(GCLK_CHANNEL_TX, generator);
        crate::clocks::enable_channel(GCLK_CHANNEL_TSU, generator);
        write(CTRLA, CTRLA_ENABLE);
        while read(SYNCB) & SYNCB_ENABLE != 0 {}
        write(NCR, 0);
        write(NCR, NCR_CLRSTAT);
        write(IDR, u32::MAX);
        write(ISR, u32::MAX);
        write(RSR, 0xF);
        write(TSR, 0x17F);
        // RMII.
        write(UR, 0);
        // Multicast through the hash, no bucket on until the stack sets
        // its groups (`set_multicast_filter`).
        write(HRB, 0);
        write(HRT, 0);
        write(NCFGR, NCFGR_CLK_DIV64 | NCFGR_DBW64 | NCFGR_MTIHEN | NCFGR_MAXFS | NCFGR_RFCS | NCFGR_SPD | NCFGR_FD);
        write(DCFGR, DCFGR_VALUE);

        let PacketQueue {
            tx_descriptors,
            rx_descriptors,
            tx_buffers,
            rx_buffers,
        } = queue;
        // Every receive slot starts empty and software's, so the DMA waits
        // for the stack's first buffers.
        for (i, descriptor) in rx_descriptors.iter_mut().enumerate() {
            let wrap = if i == RX - 1 { RX_WRAP } else { 0 };
            descriptor.set_address(RX_OWNED | wrap);
            descriptor.set_status(0);
        }
        for (i, descriptor) in tx_descriptors.iter_mut().enumerate() {
            let wrap = if i == TX - 1 { TX_WRAP } else { 0 };
            descriptor.set_address(0);
            descriptor.set_status(TX_USED | wrap);
        }
        rx_buffers.fill_with(|| None);
        tx_buffers.fill_with(|| None);
        compiler_fence(Ordering::SeqCst);
        #[allow(clippy::cast_possible_truncation, reason = "SRAM addresses; addresses are 32 bits on the target")]
        {
            write(RBQB, rx_descriptors.as_ptr() as u32);
            write(TBQB, tx_descriptors.as_ptr() as u32);
        }
        write(SAB1, u32::from_le_bytes([mac[0], mac[1], mac[2], mac[3]]));
        write(SAT1, u32::from(u16::from_le_bytes([mac[4], mac[5]])));

        write(NCR, NCR_MPE);
        // The PHY's MDIO address: the first that answers with an ID.
        let (phy, phy_id) = (0..32u8)
            .find_map(|phy| {
                let id = (u32::from(mdio_read(phy, PHYID1)) << 16) | u32::from(mdio_read(phy, PHYID2));
                (id != 0 && id != 0xFFFF_FFFF).then_some((phy, id))
            })
            .ok_or(Error::NoPhy)?;
        mdio_write(phy, ANAR, 0x01E1);
        mdio_write(phy, BMCR, BMCR_ANENABLE | BMCR_ANRESTART);

        write(IER, IRQS);
        Interrupt::ETH.unpend();
        // SAFETY: the binding makes the vector call the handler.
        unsafe { Interrupt::ETH.enable() };
        write(NCR, NCR_MPE | NCR_RXEN | NCR_TXEN);

        Ok(Self {
            rx: RxRing {
                descriptors: rx_descriptors,
                buffers: rx_buffers,
                next: 0,
                fill: 0,
                empty: RX,
            },
            tx: TxRing {
                descriptors: tx_descriptors,
                buffers: tx_buffers,
                next: 0,
                reclaim: 0,
                in_flight: 0,
            },
            mac,
            phy,
            phy_id,
            link: None,
            rx_burst: 0,
            hash: 0,
            next_poll: Instant::now(),
            poll: None,
            tx_watch: None,
            _reset: reset,
            _lifetime: PhantomData,
        })
    }

    /// The PHY's MDIO address and its ID (`PHYID1 << 16 | PHYID2`).
    #[must_use]
    pub fn phy(&self) -> (u8, u32) {
        (self.phy, self.phy_id)
    }

    /// The link: `Some((100 Mbit/s, full duplex))`, or `None` while down.
    #[must_use]
    pub fn link(&self) -> Option<(bool, bool)> {
        self.link
    }

    /// Reads the link from the PHY, and sets the MAC's speed and duplex to
    /// what was negotiated.
    fn poll_link(&mut self) {
        // The link bit latches low: read twice for the present state.
        let _ = mdio_read(self.phy, BMSR);
        let bmsr = mdio_read(self.phy, BMSR);
        let link = if bmsr & BMSR_LINK != 0 && bmsr & BMSR_ANCOMPLETE != 0 {
            let common = mdio_read(self.phy, ANAR) & mdio_read(self.phy, ANLPAR);
            let (fast, full) = if common & (1 << 8) != 0 {
                (true, true)
            } else if common & (1 << 7) != 0 {
                (true, false)
            } else {
                (false, common & (1 << 6) != 0)
            };
            Some((fast, full))
        } else {
            None
        };
        if link != self.link {
            if let Some((fast, full)) = link {
                let ncfgr = read(NCFGR) & !(NCFGR_SPD | NCFGR_FD);
                write(NCFGR, ncfgr | if fast { NCFGR_SPD } else { 0 } | if full { NCFGR_FD } else { 0 });
            }
            self.link = link;
        }
    }
}

/// The hash filter's bucket for a destination address (DS60001795 32.6.9):
/// bit `i` of the index is the XOR of the address's bits `i`, `i + 6`, ...,
/// `i + 42`, counting from the first byte's least significant bit, the first
/// on the wire.
fn hash_index(addr: &[u8; 6]) -> u32 {
    let mut bits = 0u64;
    for (i, byte) in addr.iter().enumerate() {
        bits |= u64::from(*byte) << (8 * i);
    }
    let mut index = 0u64;
    for chunk in 0..8 {
        index ^= (bits >> (6 * chunk)) & 0x3F;
    }
    #[expect(clippy::cast_possible_truncation, reason = "six bits")]
    let index = index as u32;
    index
}

impl xarxa_driver::Driver for Ethernet<'_> {
    fn capabilities(&self) -> Capabilities {
        let mut capabilities = Capabilities::default();
        capabilities.max_transmission_unit = MTU;
        capabilities
    }

    fn hardware_address(&self) -> HardwareAddress {
        HardwareAddress::Ethernet(self.mac)
    }

    fn link_state(&mut self) -> LinkState {
        if self.link.is_some() { LinkState::Up } else { LinkState::Down }
    }

    fn register_waker(&mut self, waker: &Waker) -> Result<(), NotSupported> {
        WAKER.register(waker);
        // The PHY, twice a second: xarxa registers once per poll, which is
        // where the timer for the next one is armed.
        if Instant::now() >= self.next_poll {
            self.poll_link();
            self.next_poll = Instant::now() + Duration::from_millis(500);
            self.poll = Some(Timer::at(self.next_poll));
        }
        let mut cx = Context::from_waker(waker);
        if let Some(timer) = self.poll.as_mut()
            && Pin::new(timer).poll(&mut cx).is_ready()
        {
            self.poll = None;
            waker.wake_by_ref();
        }
        // The transmit DMA, every millisecond while frames wait, so that
        // `reclaim` sees it stopped even if no interrupt comes after: the
        // one that wakes the stack once the DMA has finished can arrive
        // while TSR still shows it running.
        self.tx.reclaim();
        if self.tx.in_flight == 0 {
            self.tx_watch = None;
        } else {
            let watch = self.tx_watch.get_or_insert_with(|| Timer::after(Duration::from_millis(1)));
            if Pin::new(watch).poll(&mut cx).is_ready() {
                self.tx_watch = None;
                waker.wake_by_ref();
            }
        }
        Ok(())
    }

    fn rx_wanted(&mut self) -> RxWanted {
        RxWanted {
            count: self.rx.empty,
            starved: self.rx.empty == self.rx.descriptors.len(),
        }
    }

    fn rx_give(&mut self, buf: DriverBuf) {
        self.rx.give(buf);
    }

    fn receive(&mut self) -> Option<DriverBuf> {
        self.tx.reclaim();
        if self.rx_burst >= RX_BUDGET {
            // Let the other tasks run; come straight back after them.
            self.rx_burst = 0;
            WAKER.wake();
            return None;
        }
        let Some(buf) = self.rx.receive() else {
            // Empty: the interrupt wakes us for the next frame. It doesn't
            // for one the MAC finished while it was masked, and that
            // frame's descriptor can reach memory after the interrupt that
            // woke us, so a frame can be there unseen (Linux's macb driver,
            // for the same Cadence MAC, notes this too). Look again once the
            // interrupt is on: one there now is ours to take.
            self.rx_burst = 0;
            write(IER, IRQ_RX);
            cortex_m::asm::dsb();
            if !self.rx.ready() {
                return None;
            }
            write(IDR, IRQ_RX);
            let Some(buf) = self.rx.receive() else {
                // Only part of a frame, given back: masked again, so come
                // back for the rest ourselves.
                WAKER.wake();
                return None;
            };
            self.rx_burst += 1;
            return Some(buf);
        };
        self.rx_burst += 1;
        Some(buf)
    }

    fn can_transmit(&mut self) -> bool {
        // Nothing goes out while the link is down: a frame sent then is
        // lost (the router solicitations at boot would be spent before
        // autonegotiation ends, leaving SLAAC waiting for the router's next
        // periodic advertisement). The PHY poll wakes the stack when the
        // link comes up.
        self.link.is_some() && self.tx.can_transmit()
    }

    fn transmit(&mut self, buf: DriverBuf) -> Result<(), DriverBuf> {
        if !self.can_transmit() {
            return Err(buf);
        }
        self.tx.transmit(buf);
        Ok(())
    }

    fn set_multicast_filter(&mut self, addrs: &[[u8; 6]]) {
        let hash = addrs.iter().fold(0u64, |hash, addr| hash | 1 << hash_index(addr));
        if hash != self.hash {
            self.hash = hash;
            #[expect(clippy::cast_possible_truncation, reason = "the two halves")]
            {
                write(HRB, hash as u32);
                write(HRT, (hash >> 32) as u32);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::hash_index;

    #[test]
    fn hash_buckets() {
        // All zeros hash to bucket 0; one bit set lands in its own bucket.
        assert_eq!(hash_index(&[0; 6]), 0);
        assert_eq!(hash_index(&[1, 0, 0, 0, 0, 0]), 1);
        assert_eq!(hash_index(&[0x40, 0, 0, 0, 0, 0]), 1);
        // IPv6 all-nodes, 33:33:00:00:00:01.
        let bits = 0x33u64 | 0x33 << 8 | 1 << 40;
        let expected = (0..8).fold(0, |i, c| i ^ ((bits >> (6 * c)) & 0x3F));
        assert_eq!(u64::from(hash_index(&[0x33, 0x33, 0, 0, 0, 1])), expected);
    }
}

// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The Ethernet MAC (ETH, DS60001795 chapter 32: Cadence's GEM, as on the
//! SAM E5x/E7x GMAC), as an [`embassy_net_driver::Driver`], in RMII to an
//! external PHY managed over MDIO.
//!
//! [`Ethernet::new`] takes the `ETH` peripheral, a
//! [`bind_interrupts!`](crate::bind_interrupts) type binding its vector
//! (`ETH`) to [`InterruptHandler`], the RMII pins, the PHY's reset pin and a
//! [`PacketQueue`] for the DMA's descriptors and buffers (in SRAM, which the
//! cache doesn't cover, so no cache upkeep). The PHY's 50 MHz reference
//! clock comes in on `REF_CLK` (PC00); the MAC also needs its two generic
//! clock channels fed, from the generator `new` is given. It finds the PHY's address by
//! scanning MDIO, restarts autonegotiation, and polls the link about twice a
//! second; the MAC's speed and duplex follow what was negotiated.
//!
//! One DMA queue, whole frames per buffer (1,536 bytes), the FCS stripped,
//! all multicast let through (for IPv6), no checksum offload yet (smoltcp
//! computes checksums). Unlike the GEM's usual ISR, this chip's is
//! write-to-clear. The DMA sends its ring strictly in order, so a transmit
//! descriptor is taken only when its token is consumed.

use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{Ordering, compiler_fence};
use core::task::Context;

use embassy_net_driver::{Capabilities, HardwareAddress, LinkState};
use embassy_sync::waitqueue::AtomicWaker;
use embassy_time::{Duration, Instant, Timer};

use crate::gpio::{Function, Level, Output, set_function};
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
/// itself), so a flood can't keep smoltcp's poll loop, and with it the
/// executor, busy for as long as frames keep coming.
const RX_BUDGET: u8 = 8;

/// The DMA buffer size: a whole frame with room to spare (DCFGR.DRBS).
pub const BUFFER_SIZE: usize = 1536;
/// The largest frame smoltcp may hand over (no FCS).
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

#[repr(C, align(32))]
struct Buffer([u8; BUFFER_SIZE]);

/// The DMA's descriptors and buffers: `TX` transmit and `RX` receive
/// buffers of [`BUFFER_SIZE`]. Give [`Ethernet::new`] a `&'static mut` one
/// (a `static` through `StaticCell`, say).
pub struct PacketQueue<const TX: usize, const RX: usize> {
    tx_descriptors: [Descriptor; TX],
    rx_descriptors: [Descriptor; RX],
    tx_buffers: [Buffer; TX],
    rx_buffers: [Buffer; RX],
}

impl<const TX: usize, const RX: usize> PacketQueue<TX, RX> {
    /// An empty queue.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            tx_descriptors: [const { Descriptor::new() }; TX],
            rx_descriptors: [const { Descriptor::new() }; RX],
            tx_buffers: [const { Buffer([0; BUFFER_SIZE]) }; TX],
            rx_buffers: [const { Buffer([0; BUFFER_SIZE]) }; RX],
        }
    }
}

impl<const TX: usize, const RX: usize> Default for PacketQueue<TX, RX> {
    fn default() -> Self {
        Self::new()
    }
}

static RX_WAKER: AtomicWaker = AtomicWaker::new();
static TX_WAKER: AtomicWaker = AtomicWaker::new();

/// The ETH vector's handler: wakes the receive and transmit sides.
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
            RX_WAKER.wake();
        }
        if fired & (IRQ_TX | IRQ_HRESP) != 0 {
            write(TSR, 0x17F);
            TX_WAKER.wake();
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

/// The RMII pins: the PHY's reference clock, the management pair, receive
/// and transmit.
pub struct Pins<'d> {
    pub ref_clk: Peri<'d, peripherals::PC00>,
    pub mdc: Peri<'d, peripherals::PC03>,
    pub mdio: Peri<'d, peripherals::PC04>,
    pub crs_dv: Peri<'d, peripherals::PC10>,
    pub rx_er: Peri<'d, peripherals::PC09>,
    pub rxd0: Peri<'d, peripherals::PC07>,
    pub rxd1: Peri<'d, peripherals::PC06>,
    pub tx_en: Peri<'d, peripherals::PC11>,
    pub txd0: Peri<'d, peripherals::PC12>,
    pub txd1: Peri<'d, peripherals::PC13>,
}

/// Why [`Ethernet::new`] failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// No PHY answered on MDIO.
    NoPhy,
}

/// A ring of descriptors and their buffers, and the next one to use. The
/// DMA goes round it in order, stopping at the first one software owns, so
/// software must take them in order too.
struct Ring<'d> {
    descriptors: &'d mut [Descriptor],
    buffers: &'d mut [Buffer],
    next: usize,
}

impl Ring<'_> {
    fn advance(&mut self) {
        self.next = (self.next + 1) % self.descriptors.len();
    }

    /// Whether the next transmit descriptor is software's to fill.
    fn tx_ready(&self) -> bool {
        self.descriptors[self.next].status() & TX_USED != 0
    }

    /// The next whole received frame's length, giving back any buffers
    /// that hold part of a frame (a frame larger than a buffer, which the
    /// MTU rules out, or the tail of one after an overrun).
    fn rx_ready(&mut self) -> Option<usize> {
        loop {
            let descriptor = &mut self.descriptors[self.next];
            let address = descriptor.address();
            if address & RX_OWNED == 0 {
                return None;
            }
            let status = descriptor.status();
            if status & (RX_SOF | RX_EOF) == RX_SOF | RX_EOF {
                return Some(((status & RX_LENGTH) as usize).min(BUFFER_SIZE));
            }
            descriptor.set_address(address & !RX_OWNED);
            self.advance();
        }
    }
}

/// The MAC, as an [`embassy_net_driver::Driver`].
pub struct Ethernet<'d> {
    rx: Ring<'d>,
    tx: Ring<'d>,
    mac: [u8; 6],
    phy: u8,
    phy_id: u32,
    link: Option<(bool, bool)>,
    /// Frames handed out since the last yield (see [`RX_BUDGET`]).
    rx_burst: u8,
    next_poll: Instant,
    poll: Option<Timer>,
    _reset: Output<'d>,
    _lifetime: PhantomData<&'d ()>,
}

impl<'d> Ethernet<'d> {
    /// The MAC on `pins`, its PHY out of reset (`phy_reset`, active low),
    /// with `queue`'s buffers and the hardware address `mac`; its two
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
        pins: Pins<'d>,
        phy_reset: Peri<'d, peripherals::PD21>,
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
        set_function(&*pins.ref_clk, Function::L);
        set_function(&*pins.mdc, Function::L);
        set_function(&*pins.mdio, Function::L);
        set_function(&*pins.crs_dv, Function::L);
        set_function(&*pins.rx_er, Function::L);
        set_function(&*pins.rxd0, Function::L);
        set_function(&*pins.rxd1, Function::L);
        set_function(&*pins.tx_en, Function::L);
        set_function(&*pins.txd0, Function::L);
        set_function(&*pins.txd1, Function::L);

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
        // Multicast through the hash, every bucket on: IPv6's neighbour
        // discovery and router advertisements need it; smoltcp filters.
        write(HRB, u32::MAX);
        write(HRT, u32::MAX);
        write(NCFGR, NCFGR_CLK_DIV64 | NCFGR_DBW64 | NCFGR_MTIHEN | NCFGR_MAXFS | NCFGR_RFCS | NCFGR_SPD | NCFGR_FD);
        write(DCFGR, DCFGR_VALUE);

        let PacketQueue { tx_descriptors, rx_descriptors, tx_buffers, rx_buffers } = queue;
        for (i, (descriptor, buffer)) in rx_descriptors.iter_mut().zip(rx_buffers.iter()).enumerate() {
            let wrap = if i == RX - 1 { RX_WRAP } else { 0 };
            #[allow(clippy::cast_possible_truncation, reason = "an SRAM address; addresses are 32 bits on the target")]
            descriptor.set_address(buffer.0.as_ptr() as u32 | wrap);
            descriptor.set_status(0);
        }
        for (i, (descriptor, buffer)) in tx_descriptors.iter_mut().zip(tx_buffers.iter()).enumerate() {
            let wrap = if i == TX - 1 { TX_WRAP } else { 0 };
            #[allow(clippy::cast_possible_truncation, reason = "an SRAM address; addresses are 32 bits on the target")]
            descriptor.set_address(buffer.0.as_ptr() as u32);
            descriptor.set_status(TX_USED | wrap);
        }
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
            rx: Ring { descriptors: rx_descriptors, buffers: rx_buffers, next: 0 },
            tx: Ring { descriptors: tx_descriptors, buffers: tx_buffers, next: 0 },
            mac,
            phy,
            phy_id,
            link: None,
            rx_burst: 0,
            next_poll: Instant::now(),
            poll: None,
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

/// A received frame, in its DMA buffer until consumed.
pub struct RxToken<'a> {
    ring: &'a mut Ring<'static>,
    length: usize,
}

impl embassy_net_driver::RxToken for RxToken<'_> {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        compiler_fence(Ordering::SeqCst);
        let index = self.ring.next;
        let result = f(&mut self.ring.buffers[index].0[..self.length]);
        // Back to the DMA.
        compiler_fence(Ordering::SeqCst);
        let address = self.ring.descriptors[index].address() & !RX_OWNED;
        self.ring.descriptors[index].set_address(address);
        self.ring.advance();
        result
    }
}

/// A transmit buffer, taken (in ring order) and sent when consumed.
pub struct TxToken<'a> {
    ring: &'a mut Ring<'static>,
}

impl embassy_net_driver::TxToken for TxToken<'_> {
    fn consume<R, F>(self, length: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let index = self.ring.next;
        let length = length.min(MTU);
        let result = f(&mut self.ring.buffers[index].0[..length]);
        compiler_fence(Ordering::SeqCst);
        let descriptor = &mut self.ring.descriptors[index];
        let wrap = descriptor.status() & TX_WRAP;
        #[expect(clippy::cast_possible_truncation, reason = "at most the MTU")]
        descriptor.set_status(wrap | TX_LAST | (length as u32 & TX_LENGTH));
        compiler_fence(Ordering::SeqCst);
        write(NCR, read(NCR) | NCR_TSTART);
        self.ring.advance();
        result
    }
}

impl embassy_net_driver::Driver for Ethernet<'static> {
    type RxToken<'a>
        = RxToken<'a>
    where
        Self: 'a;
    type TxToken<'a>
        = TxToken<'a>
    where
        Self: 'a;

    fn receive(&mut self, cx: &mut Context) -> Option<(RxToken<'_>, TxToken<'_>)> {
        RX_WAKER.register(cx.waker());
        TX_WAKER.register(cx.waker());
        if self.rx_burst >= RX_BUDGET {
            // Let the other tasks run; come straight back after them.
            self.rx_burst = 0;
            cx.waker().wake_by_ref();
            return None;
        }
        let Some(length) = self.rx.rx_ready() else {
            // Empty: the interrupt wakes us for the next frame (one that
            // came in before this unmasking still raises it).
            self.rx_burst = 0;
            write(IER, IRQ_RX);
            return None;
        };
        if !self.tx.tx_ready() {
            return None;
        }
        self.rx_burst += 1;
        Some((RxToken { ring: &mut self.rx, length }, TxToken { ring: &mut self.tx }))
    }

    fn transmit(&mut self, cx: &mut Context) -> Option<TxToken<'_>> {
        TX_WAKER.register(cx.waker());
        // Nothing goes out while the link is down: a frame sent then is lost,
        // and smoltcp counts it as sent (its three router solicitations at
        // boot are spent before autonegotiation ends, leaving SLAAC waiting
        // for the router's next periodic advertisement).
        (self.link.is_some() && self.tx.tx_ready()).then_some(TxToken { ring: &mut self.tx })
    }

    fn link_state(&mut self, cx: &mut Context) -> LinkState {
        // Poll the PHY twice a second, waking for the next poll.
        if Instant::now() >= self.next_poll {
            self.poll_link();
            self.next_poll = Instant::now() + Duration::from_millis(500);
            self.poll = Some(Timer::at(self.next_poll));
        }
        if let Some(timer) = self.poll.as_mut()
            && Pin::new(timer).poll(cx).is_ready()
        {
            self.poll = None;
            cx.waker().wake_by_ref();
        }
        if self.link.is_some() { LinkState::Up } else { LinkState::Down }
    }

    fn capabilities(&self) -> Capabilities {
        let mut capabilities = Capabilities::default();
        capabilities.max_transmission_unit = MTU;
        capabilities.max_burst_size = Some(self.tx.descriptors.len().min(self.rx.descriptors.len()));
        capabilities
    }

    fn hardware_address(&self) -> HardwareAddress {
        HardwareAddress::Ethernet(self.mac)
    }
}

// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! An [`embassy_usb_driver`] for the high-speed USB controller of
//! Microchip's PIC32CK (USBHS, DS60001795 chapter 37): a Mentor MUSB
//! (MUSBMHDRC) core at `0x4600_1000`, behind Microchip's wrapper (CTRLA,
//! INTFLAG, STATUS at `0x4600_0000`) and with an on-chip HS PHY (PHY00 to
//! PHY50 at `0x4600_1500`).
//!
//! High speed (480 Mbit/s), or full speed if [`Config::high_speed`] says
//! so; device mode, endpoint 0 and seven endpoint numbers each with an IN
//! and an OUT endpoint: control (64 bytes), bulk (up to 512 bytes,
//! double-buffered) and interrupt (up to 1,024 bytes); no isochronous. The
//! classes choose the packet sizes: 512-byte bulk endpoints are only legal
//! at high speed, so a device that may meet a full-speed port has to
//! describe 64-byte ones. [`is_high_speed`] says what the bus chose.
//!
//! The packets go through the core's own 8 KiB of FIFO RAM, which the CPU
//! loads and unloads 32 bits at a time (13.8-14.3 MB/s each way through a
//! 512-byte CDC-ACM echo; 0.53 MB/s at full speed), or the core's DMA if
//! [`Config::dma`] says so;
//! endpoint 0 has the first 64 bytes and each other endpoint a slice from
//! [`alloc_endpoint_in`](embassy_usb_driver::Driver::alloc_endpoint_in) and
//! `_out`, twice its packet size for bulk. A write returns once the packet
//! is in the FIFO, so a bulk endpoint can have one packet on the wire and
//! the next loaded.
//!
//! The core's endpoint registers are indexed (INDEX.SELEP), so each access
//! to them selects the endpoint in a critical section. The FIFOs aren't
//! indexed and are copied outside it.
//!
//! [`Driver::new`] takes the `USBHS` peripheral, a
//! [`bind_interrupts!`](crate::bind_interrupts) type binding its vector
//! (`USBHS`) to [`InterruptHandler`], and a [`Config`]. It turns on the USB
//! regulator (SUPC VREGCTRL.AVREGEN), loads the PHY's trim from CAL OTP
//! (FCCFG67), and starts the PHY, whose PLL runs from the XOSC at 12 or
//! 24 MHz: [`crate::init`]'s clocks must have the XOSC at one of those (see
//! [`clocks::usbhs_reference_hz`](crate::clocks::usbhs_reference_hz)). The
//! data lines, VBUS (which starts the session) and the bias resistor are
//! dedicated pins, so the driver takes no pin singletons; the ID pin is
//! overridden to a B device. Microchip's erratum 2.21.1: toggling PD02-PD04
//! or PD13-PD16 as GPIO disturbs high-speed traffic.
//!
//! The core attaches (its pull-up on D+) only during a session, which
//! starts when VBUS reaches the pin: the driver turns on the PHY's VBUS
//! comparators (PHY24 bit 1, off out of reset). No interrupt says VBUS
//! came or went, so the bus looks at DEVCTL.VBUS every 32 ms while it
//! waits for power and while suspended (a host keeps VBUS up through a
//! suspend; a pulled cable shows first as a suspend), and reports
//! `PowerDetected` and `PowerRemoved` from it. That and remote wakeup's
//! 10 ms of resume signalling use embassy-time, so the application needs
//! a time driver. (The wrapper's own 1 ms tick hangs in SYNCBUSY once the
//! PHY stops in suspend.)
//!
//! Tested on the SG01 Curiosity Ultra against macOS and Linux:
//! enumeration at either speed, CDC-ACM and CDC-NCM (with embassy-net,
//! whose notifications run the interrupt endpoint and whose data interface
//! switches alternate settings), VBUS removal and return, suspend, resume
//! and remote wakeup (Linux: 64 MiB echoed at 15.9 MB/s each way, 20
//! autosuspend cycles clean, through a hub too). Known issue: on a marginal
//! link (another cable and port, while a back-powered device upset the
//! hub), a corrupted packet left the core deaf: FRAME stops counting, the PHY still reports
//! ready, no suspend is seen, and the host's transfers all fail (-71) until
//! the board is reset; a suspend there also dropped off the bus about one
//! time in five. Pulsing SOFTRST.NRSTX doesn't bring it back (it stops
//! enumeration altogether). On a good link neither happens.

use core::future::poll_fn;
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU16, Ordering};
use core::task::Poll;

use embassy_futures::select::{Either, select};
use embassy_sync::waitqueue::AtomicWaker;
use embassy_time::{Duration, Timer};
use embassy_usb_driver::{Direction, EndpointAddress, EndpointAllocError, EndpointError, EndpointInfo, EndpointType, Event, Unsupported};

use crate::interrupt::typelevel::{self, Binding, Handler};
use crate::interrupt::{Interrupt, InterruptExt};
use crate::{Peri, peripherals};

/// The controller: Microchip's wrapper at 0x000, the MUSB core from 0x1000.
const BASE: usize = 0x4600_0000;
const CTRLA: usize = 0x00;
const INTENSET: usize = 0x10;
const INTFLAG: usize = 0x14;
const STATUS: usize = 0x18;
const SYNCBUSY: usize = 0x1C;
const FADDR: usize = 0x1000;
const POWR: usize = 0x1001;
const INTRTX: usize = 0x1002;
const INTRRX: usize = 0x1004;
const INTRTXE: usize = 0x1006;
const INTRRXE: usize = 0x1008;
const INTRUSB: usize = 0x100A;
const INTRUSBE: usize = 0x100B;
const INDEX: usize = 0x100E;
/// Indexed: the selected endpoint's registers. CSR0 (endpoint 0) shares
/// TXCSR's place and COUNT0 RXCOUNT's.
const TXMAXP: usize = 0x1010;
const CSR0L: usize = 0x1012;
const TXCSRL: usize = 0x1012;
const TXCSRH: usize = 0x1013;
const RXMAXP: usize = 0x1014;
const RXCSRL: usize = 0x1016;
const RXCSRH: usize = 0x1017;
const RXCOUNT: usize = 0x1018;
const TXFIFOSZ: usize = 0x1062;
const RXFIFOSZ: usize = 0x1063;
const TXFIFOADD: usize = 0x1064;
const RXFIFOADD: usize = 0x1066;
const DEVCTL: usize = 0x1060;
/// Endpoint `n`'s FIFO, not indexed.
const FIFO: usize = 0x1020;
/// The DMA's interrupt flags (cleared by reading), and channel `c`'s
/// control, memory address and count at 0x1204 + 16 c (the data sheet's
/// offsets overlap; the PAC's and Mentor's layout).
const DMAINTR: usize = 0x1200;
const fn dma(channel: usize, register: usize) -> usize {
    0x1204 + 16 * channel + register
}
const DMA_CNTL: usize = 0x0;
const DMA_ADDR: usize = 0x4;
const DMA_COUNT: usize = 0x8;
const DMA_EN: u32 = 1 << 0;
/// DMADIR: memory to the FIFO (an IN endpoint).
const DMA_TO_FIFO: u32 = 1 << 1;
const DMA_IE: u32 = 1 << 3;
const DMA_EP_SHIFT: u32 = 4;
const DMA_ERR: u32 = 1 << 8;
/// DMABRSTM 3: INCR16, INCR8, INCR4 or unspecified bursts.
const DMA_BURST: u32 = 3 << 9;
const DMA_CHANNELS: usize = 8;
/// SRAM, the one memory the DMA may use: flash reads by bus masters other
/// than the CPU fail in each panel's first 96 KiB with the address wait
/// state on (erratum 2.6.3).
const SRAM: core::ops::Range<usize> = 0x2000_0000..0x2008_0000;
/// The PHY's trim registers.
const PHY04: usize = 0x1504;
const PHY08: usize = 0x1508;
const PHY0C: usize = 0x150C;
const PHY10: usize = 0x1510;
const PHY14: usize = 0x1514;
const PHY18: usize = 0x1518;
const PHY20: usize = 0x1520;
const PHY24: usize = 0x1524;
const PHY28: usize = 0x1528;
/// PHY24.OTGPDN: powers the PHY's VBUS and session comparators, which are
/// off out of reset (DEVCTL then reads VBUS below session end whatever the
/// pin sees, and the core never attaches). The data sheet calls the bit
/// "ODT power down"; found on the bench.
const PHY24_OTGPDN: u32 = 1 << 1;

const CTRLA_SWRST: u32 = 1 << 0;
const CTRLA_ENABLE: u32 = 1 << 1;
const CTRLA_IDVAL: u32 = 1 << 8;
const CTRLA_IDOVEN: u32 = 1 << 9;
const CTRLA_REFCLKSEL_24MHZ: u32 = 1 << 10;
/// INTFLAG/INTENSET.USB: the core's interrupts, and .DMA: its DMA's.
const INT_USB: u32 = 1 << 2;
const INT_DMA: u32 = 1 << 3;
const STATUS_PHYRDY: u32 = 1 << 0;

const POWR_RESUME: u8 = 1 << 2;
const POWR_HSMODE: u8 = 1 << 4;
const POWR_HSEN: u8 = 1 << 5;
const POWR_SOFTCONN: u8 = 1 << 6;
const USB_SUSPEND: u8 = 1 << 0;
const USB_RESUME: u8 = 1 << 1;
const USB_RESET: u8 = 1 << 2;
/// In device mode: the session ended (VBUS gone).
const USB_DISCON: u8 = 1 << 5;
/// DEVCTL.VBUS: 0b11 above VBUS valid.
const DEVCTL_VBUS: u8 = 0b11 << 3;

const CSR0_RXPKTRDY: u8 = 1 << 0;
const CSR0_TXPKTRDY: u8 = 1 << 1;
const CSR0_SENTSTALL: u8 = 1 << 2;
const CSR0_DATAEND: u8 = 1 << 3;
const CSR0_SETUPEND: u8 = 1 << 4;
const CSR0_SENDSTALL: u8 = 1 << 5;
const CSR0_SERVICEDRXPKTRDY: u8 = 1 << 6;
const CSR0_SERVICEDSETUPEND: u8 = 1 << 7;

const TX_TXPKTRDY: u8 = 1 << 0;
const TX_FIFONE: u8 = 1 << 1;
const TX_FLUSHFIFO: u8 = 1 << 3;
const TX_SENDSTALL: u8 = 1 << 4;
const TX_SENTSTALL: u8 = 1 << 5;
const TX_CLRDT: u8 = 1 << 6;
const TX_INCOMPTX: u8 = 1 << 7;
/// TXCSRL's bits that writing 0 clears: written as 1 to leave them.
const TX_KEEP: u8 = TX_SENTSTALL | TX_INCOMPTX | TX_FIFONE;
const TXCSRH_MODE_TX: u8 = 1 << 5;

const RX_RXPKTRDY: u8 = 1 << 0;
const RX_OVERRUN: u8 = 1 << 2;
const RX_FLUSHFIFO: u8 = 1 << 4;
const RX_SENDSTALL: u8 = 1 << 5;
const RX_SENTSTALL: u8 = 1 << 6;
const RX_CLRDATATOG: u8 = 1 << 7;
/// RXCSRL's bits that writing 0 clears, RXPKTRDY among them.
const RX_KEEP: u8 = RX_SENTSTALL | RX_OVERRUN | RX_RXPKTRDY;
const RXCSRH_DISNYET: u8 = 1 << 4;
/// TXFIFOSZ/RXFIFOSZ.DPB: two packets.
const FIFOSZ_DPB: u8 = 1 << 4;

/// SUPC VREGCTRL.AVREGEN: the USB regulator, which must be up 55 µs before
/// the controller's registers are touched (DS60001795 37.6).
const SUPC_VREGCTRL: usize = 0x4400_801C;
const VREGCTRL_AVREGEN: u32 = 1 << 16;
/// CAL OTP FCCFG67: the PHY's trim (squelch, HS amplitude, termination, HS
/// slew, disconnect detection, HS driver compensation).
const PHY_TRIM_OTP: usize = 0x0A00_718C;

/// Endpoint numbers, endpoint 0 included.
const ENDPOINTS: usize = 8;
/// The FIFO RAM given to endpoints: 8 KiB of the data sheet's 9 KB
/// (RAMINFO.RAMBITS reads 12: 4 Ki words of address); endpoint 0 has the
/// first 64 bytes.
const FIFO_BYTES: usize = 8192;
const EP0_FIFO_BYTES: usize = 64;

static BUS_WAKER: AtomicWaker = AtomicWaker::new();
static IN_WAKERS: [AtomicWaker; ENDPOINTS] = [const { AtomicWaker::new() }; ENDPOINTS];
static OUT_WAKERS: [AtomicWaker; ENDPOINTS] = [const { AtomicWaker::new() }; ENDPOINTS];
/// The core's bus interrupts (INTRUSB), which reading clears, taken by the
/// handler for [`Bus::poll`].
static BUS_EVENTS: AtomicU8 = AtomicU8::new(0);
/// Set by the handler on each endpoint 0 interrupt.
static EP0_EVENT: AtomicBool = AtomicBool::new(false);
/// Which endpoints are enabled: bit n OUT n, bit 8 + n IN n.
static ENABLED: AtomicU16 = AtomicU16::new(0);
/// DMA channels finished, from DMAINTR, and their waiters.
static DMA_DONE: AtomicU8 = AtomicU8::new(0);
static DMA_WAKERS: [AtomicWaker; DMA_CHANNELS] = [const { AtomicWaker::new() }; DMA_CHANNELS];

fn read8(offset: usize) -> u8 {
    // SAFETY: a USBHS register, read whole.
    unsafe { read_volatile((BASE + offset) as *const u8) }
}

fn write8(offset: usize, value: u8) {
    // SAFETY: a USBHS register; the indexed ones under `indexed`'s critical
    // section, each endpoint's by its one owner.
    unsafe { write_volatile((BASE + offset) as *mut u8, value) }
}

fn read16(offset: usize) -> u16 {
    // SAFETY: as for `read8`.
    unsafe { read_volatile((BASE + offset) as *const u16) }
}

fn write16(offset: usize, value: u16) {
    // SAFETY: as for `write8`.
    unsafe { write_volatile((BASE + offset) as *mut u16, value) }
}

fn read32(address: usize) -> u32 {
    // SAFETY: a USBHS, SUPC or CAL OTP word.
    unsafe { read_volatile(address as *const u32) }
}

fn write32(address: usize, value: u32) {
    // SAFETY: a USBHS or SUPC register.
    unsafe { write_volatile(address as *mut u32, value) }
}

/// Runs `f` with endpoint `n`'s registers selected, where nothing else
/// selects another's.
fn indexed<R>(n: usize, f: impl FnOnce() -> R) -> R {
    critical_section::with(|_| {
        #[expect(clippy::cast_possible_truncation, reason = "an endpoint number, under 8")]
        write8(INDEX, n as u8);
        f()
    })
}

/// Loads `data` into endpoint `n`'s FIFO: words, then a half word and a
/// byte for the tail. A byte after the half word goes on its own lane
/// (FIFO address + 2): at the FIFO's address it was lost (found on the
/// bench, with packets of 4n + 3 bytes); a lone byte or half word at the
/// end works at the FIFO's address.
fn fifo_write(n: usize, data: &[u8]) {
    let fifo = BASE + FIFO + 4 * n;
    let (words, tail) = data.as_chunks::<4>();
    for word in words {
        // SAFETY: the endpoint's FIFO register, which only its owner loads.
        unsafe { write_volatile(fifo as *mut u32, u32::from_le_bytes(*word)) };
    }
    if tail.len() >= 2 {
        // SAFETY: as above.
        unsafe { write_volatile(fifo as *mut u16, u16::from_le_bytes([tail[0], tail[1]])) };
    }
    if tail.len() % 2 == 1 {
        // SAFETY: as above, on the byte's lane.
        unsafe { write_volatile((fifo + tail.len() - 1) as *mut u8, tail[tail.len() - 1]) };
    }
}

/// Unloads `buf.len()` bytes from endpoint `n`'s FIFO, as `fifo_write`
/// loads them.
fn fifo_read(n: usize, buf: &mut [u8]) {
    let fifo = BASE + FIFO + 4 * n;
    let (words, tail) = buf.as_chunks_mut::<4>();
    for word in words {
        // SAFETY: the endpoint's FIFO register, which only its owner unloads.
        *word = unsafe { read_volatile(fifo as *const u32) }.to_le_bytes();
    }
    let len = tail.len();
    if len >= 2 {
        // SAFETY: as above.
        tail[..2].copy_from_slice(&unsafe { read_volatile(fifo as *const u16) }.to_le_bytes());
    }
    if len % 2 == 1 {
        // SAFETY: as above, on the byte's lane.
        tail[len - 1] = unsafe { read_volatile((fifo + len - 1) as *const u8) };
    }
}

fn enabled_bit(addr: EndpointAddress) -> u16 {
    let shift = if addr.is_in() { 8 } else { 0 };
    1 << (addr.index() + shift)
}

fn is_enabled(addr: EndpointAddress) -> bool {
    ENABLED.load(Ordering::Acquire) & enabled_bit(addr) != 0
}

/// Whether the bus negotiated high speed at its last reset (POWR.HSMODE).
#[must_use]
pub fn is_high_speed() -> bool {
    read8(POWR) & POWR_HSMODE != 0
}

/// The controller's interrupt handler: bind `USBHS` to it with
/// [`bind_interrupts!`](crate::bind_interrupts).
pub struct InterruptHandler {
    _private: (),
}

impl Handler<typelevel::USBHS> for InterruptHandler {
    unsafe fn on_interrupt() {
        on_interrupt();
    }
}

/// Takes the core's interrupts, which reading clears: the bus events for
/// [`Bus::poll`], and a wake for each endpoint's waiter, which then reads
/// the endpoint's own status.
fn on_interrupt() {
    // The wrapper's flag first, so an interrupt the core raises after the
    // reads below raises it again.
    write32(BASE + INTFLAG, INT_USB | INT_DMA);
    let _ = read32(BASE + INTFLAG);
    let dma = read8(DMAINTR);
    if dma != 0 {
        DMA_DONE.fetch_or(dma, Ordering::AcqRel);
        for (channel, waker) in DMA_WAKERS.iter().enumerate() {
            if dma & (1 << channel) != 0 {
                waker.wake();
            }
        }
    }
    let bus = read8(INTRUSB) & read8(INTRUSBE);
    let tx = read16(INTRTX);
    let rx = read16(INTRRX);
    if bus != 0 {
        BUS_EVENTS.fetch_or(bus, Ordering::AcqRel);
        BUS_WAKER.wake();
    }
    if tx & 1 != 0 {
        EP0_EVENT.store(true, Ordering::Release);
        IN_WAKERS[0].wake();
        OUT_WAKERS[0].wake();
    }
    for n in 1..ENDPOINTS {
        if tx & (1 << n) != 0 {
            IN_WAKERS[n].wake();
        }
        if rx & (1 << n) != 0 {
            OUT_WAKERS[n].wake();
        }
    }
}

/// The driver's settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Config {
    /// Negotiate high speed at each bus reset (POWR.HSEN); `false` keeps
    /// the controller at full speed, as on a full-speed port. On by
    /// default.
    pub high_speed: bool,
    /// Move bulk packets between the FIFOs and memory by the controller's
    /// DMA (one of its eight channels for each of the first eight bulk
    /// endpoints), for buffers in SRAM at a 4-byte boundary, the CPU free
    /// to run other tasks meanwhile; the CPU copies the rest. A packet's
    /// DMA is no faster than the CPU's copy (a 512-byte CDC-ACM echo:
    /// 13.6 MB/s each way with it, 13.8 without), so it pays where the CPU
    /// has other work. Off by default, as the newer path.
    pub dma: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { high_speed: true, dma: false }
    }
}

/// An endpoint's settings from its allocation.
#[derive(Clone, Copy)]
struct EndpointConfig {
    ep_type: EndpointType,
    max_packet_size: u16,
    /// Its FIFO's start in the FIFO RAM, in bytes, and size code (8 << code
    /// bytes a packet).
    fifo_start: u16,
    fifo_size: u8,
    double: bool,
}

/// The driver, before [`start`](embassy_usb_driver::Driver::start).
pub struct Driver<'d> {
    /// POWR's speed setting: HSEN or nothing.
    speed: u8,
    out_eps: [Option<EndpointConfig>; ENDPOINTS],
    in_eps: [Option<EndpointConfig>; ENDPOINTS],
    /// The first free byte of FIFO RAM.
    fifo_free: usize,
    /// DMA channels left to give bulk endpoints (none with DMA off).
    dma_free: usize,
    _lifetime: PhantomData<&'d ()>,
}

impl<'d> Driver<'d> {
    /// The driver for the controller, set up from `config`, its PHY
    /// powered and running, still detached from the bus, its interrupt
    /// unmasked.
    ///
    /// # Panics
    ///
    /// If the XOSC doesn't run at 12 or 24 MHz, or the PHY doesn't come up.
    #[must_use]
    #[allow(clippy::needless_pass_by_value, reason = "the controller is the driver's for 'd")]
    pub fn new(_usbhs: Peri<'d, peripherals::USBHS>, _irqs: impl Binding<typelevel::USBHS, InterruptHandler> + 'd, config: Config) -> Self {
        let reference = crate::clocks::usbhs_reference_hz();
        let Some(reference) = reference else {
            panic!("USBHS needs the XOSC at 12 or 24 MHz; it runs at {:?} Hz", crate::clocks::frequencies().and_then(|f| f.xosc));
        };
        let speed = if config.high_speed { POWR_HSEN } else { 0 };
        power_up(reference, speed);
        Interrupt::USBHS.unpend();
        // SAFETY: the binding makes the vector call the handler.
        unsafe { Interrupt::USBHS.enable() };
        let dma_free = if config.dma { DMA_CHANNELS } else { 0 };
        Self { speed, out_eps: [None; ENDPOINTS], in_eps: [None; ENDPOINTS], fifo_free: EP0_FIFO_BYTES, dma_free, _lifetime: PhantomData }
    }

    /// A DMA channel for a new endpoint of `ep_type`: bulk endpoints get
    /// them while they last.
    fn dma_channel(&mut self, ep_type: EndpointType) -> Option<u8> {
        if ep_type != EndpointType::Bulk || self.dma_free == 0 {
            return None;
        }
        self.dma_free -= 1;
        #[expect(clippy::cast_possible_truncation, reason = "under 8")]
        Some((DMA_CHANNELS - 1 - self.dma_free) as u8)
    }

    fn alloc(&mut self, direction: Direction, ep_type: EndpointType, ep_addr: Option<EndpointAddress>, max_packet_size: u16) -> Result<usize, EndpointAllocError> {
        let largest = match ep_type {
            EndpointType::Bulk => 512,
            EndpointType::Interrupt => 1024,
            EndpointType::Control | EndpointType::Isochronous => return Err(EndpointAllocError),
        };
        if max_packet_size == 0 || max_packet_size > largest {
            return Err(EndpointAllocError);
        }
        let eps = if direction == Direction::In { &self.in_eps } else { &self.out_eps };
        let n = match ep_addr {
            Some(addr) if addr.index() > 0 && addr.index() < ENDPOINTS && eps[addr.index()].is_none() => addr.index(),
            Some(_) => return Err(EndpointAllocError),
            None => (1..ENDPOINTS).find(|&n| eps[n].is_none()).ok_or(EndpointAllocError)?,
        };
        // FIFOs of 8 << code bytes a packet, twice that for bulk.
        let packet = usize::from(max_packet_size).next_power_of_two().max(8);
        #[expect(clippy::cast_possible_truncation, reason = "at most 7, for 1024 bytes")]
        let fifo_size = (packet.trailing_zeros() - 3) as u8;
        let double = ep_type == EndpointType::Bulk;
        let bytes = if double { 2 * packet } else { packet };
        if self.fifo_free + bytes > FIFO_BYTES {
            return Err(EndpointAllocError);
        }
        #[expect(clippy::cast_possible_truncation, reason = "under 8192")]
        let fifo_start = self.fifo_free as u16;
        self.fifo_free += bytes;
        let config = Some(EndpointConfig { ep_type, max_packet_size, fifo_start, fifo_size, double });
        if direction == Direction::In {
            self.in_eps[n] = config;
        } else {
            self.out_eps[n] = config;
        }
        Ok(n)
    }
}

/// Brings the controller up as a B device, detached: the USB regulator,
/// a reset, the PHY's trim and VBUS comparators, the reference clock, and
/// the PHY's PLL locked.
fn power_up(reference_hz: u32, speed: u8) {
    let vregctrl = read32(SUPC_VREGCTRL);
    if vregctrl & VREGCTRL_AVREGEN == 0 {
        write32(SUPC_VREGCTRL, vregctrl | VREGCTRL_AVREGEN);
        // At least 55 µs before the first access (here 100 µs at the
        // fastest CPU clock; slower ones wait longer).
        cortex_m::asm::delay(crate::clocks::CPU_MAX_HZ / 10_000);
    }
    // Erratum 2.10.2: a read of STATUS ends the bus clock request that
    // touching the PHY's registers leaves on; once at start-up, then after
    // each PHY access.
    let _ = read32(BASE + STATUS);
    write32(BASE + CTRLA, CTRLA_SWRST);
    while read32(BASE + SYNCBUSY) & CTRLA_SWRST != 0 {}
    load_phy_trim();
    write32(BASE + PHY24, read32(BASE + PHY24) | PHY24_OTGPDN);
    let _ = read32(BASE + STATUS);
    // B device whatever the ID pin says, then enabled (the other CTRLA bits
    // are fixed while it is).
    let refclk = if reference_hz == 24_000_000 { CTRLA_REFCLKSEL_24MHZ } else { 0 };
    let control = refclk | CTRLA_IDOVEN | CTRLA_IDVAL;
    write32(BASE + CTRLA, control);
    write32(BASE + CTRLA, control | CTRLA_ENABLE);
    while read32(BASE + SYNCBUSY) & CTRLA_ENABLE != 0 {}
    let mut spins = 0u32;
    while read32(BASE + STATUS) & STATUS_PHYRDY == 0 {
        spins += 1;
        assert!(spins < 10_000_000, "the USBHS PHY didn't become ready (STATUS {:#x})", read32(BASE + STATUS));
    }
    write8(POWR, speed);
    write8(INTRUSBE, 0);
    write16(INTRTXE, 0);
    write16(INTRRXE, 0);
    write32(BASE + INTENSET, INT_USB | INT_DMA);
}

/// Loads the PHY's trim from CAL OTP FCCFG67 into PHY04-PHY28 (each field
/// split across two registers, low bits first), unless the word is blank.
fn load_phy_trim() {
    let otp = read32(PHY_TRIM_OTP);
    if otp == 0 || otp == u32::MAX {
        return;
    }
    let field = |shift: u32, width: u32| (otp >> shift) & ((1 << width) - 1);
    let squelch = field(0, 4);
    let tune = field(4, 8);
    let odt = field(12, 3);
    let hsslew = field(15, 3);
    let discondet = field(18, 4);
    let hsdrvcomp = field(22, 3);
    let update = |offset: usize, mask: u32, value: u32| {
        let old = read32(BASE + offset);
        write32(BASE + offset, (old & !mask) | (value & mask));
        let _ = read32(BASE + STATUS);
    };
    update(PHY04, 0xE0, (squelch & 0x7) << 5);
    update(PHY08, 0x01, squelch >> 3);
    update(PHY0C, 0xE0, (tune & 0x7) << 5);
    update(PHY10, 0x1F, tune >> 3);
    update(PHY14, 0x80, (odt & 0x1) << 7);
    update(PHY18, 0x03, odt >> 1);
    update(PHY20, 0xC0, (hsslew & 0x3) << 6);
    update(PHY24, 0x01, hsslew >> 2);
    update(PHY28, 0xFE, (hsdrvcomp << 5) | (discondet << 1));
}

impl<'d> embassy_usb_driver::Driver<'d> for Driver<'d> {
    type EndpointOut = Endpoint<'d, Out>;
    type EndpointIn = Endpoint<'d, In>;
    type ControlPipe = ControlPipe<'d>;
    type Bus = Bus<'d>;

    fn alloc_endpoint_out(&mut self, ep_type: EndpointType, ep_addr: Option<EndpointAddress>, max_packet_size: u16, interval_ms: u8) -> Result<Self::EndpointOut, EndpointAllocError> {
        let n = self.alloc(Direction::Out, ep_type, ep_addr, max_packet_size)?;
        Ok(Endpoint::new(EndpointAddress::from_parts(n, Direction::Out), ep_type, max_packet_size, interval_ms, self.dma_channel(ep_type)))
    }

    fn alloc_endpoint_in(&mut self, ep_type: EndpointType, ep_addr: Option<EndpointAddress>, max_packet_size: u16, interval_ms: u8) -> Result<Self::EndpointIn, EndpointAllocError> {
        let n = self.alloc(Direction::In, ep_type, ep_addr, max_packet_size)?;
        Ok(Endpoint::new(EndpointAddress::from_parts(n, Direction::In), ep_type, max_packet_size, interval_ms, self.dma_channel(ep_type)))
    }

    fn start(self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        // High speed takes 64 bytes on endpoint 0, full speed up to 64.
        let control = control_max_packet_size.min(64);
        (
            Bus { speed: self.speed, out_eps: self.out_eps, in_eps: self.in_eps, powered: false, suspended: false, _lifetime: PhantomData },
            ControlPipe { max_packet_size: control, setup_pending: false, out_pending: false, _lifetime: PhantomData },
        )
    }
}

/// Bus control.
pub struct Bus<'d> {
    speed: u8,
    out_eps: [Option<EndpointConfig>; ENDPOINTS],
    in_eps: [Option<EndpointConfig>; ENDPOINTS],
    /// Whether the last [`Event`] was `PowerDetected` rather than
    /// `PowerRemoved`, and whether the bus is suspended since.
    powered: bool,
    suspended: bool,
    _lifetime: PhantomData<&'d ()>,
}

/// Endpoint 0's interrupt only, and the bus's: what a reset leaves.
fn reset_interrupts() {
    write16(INTRTXE, 1);
    write16(INTRRXE, 0);
    write8(INTRUSBE, USB_RESET | USB_SUSPEND | USB_RESUME | USB_DISCON);
}

/// Whether VBUS is above VBUS valid (DEVCTL.VBUS 0b11).
fn vbus_valid() -> bool {
    read8(DEVCTL) & DEVCTL_VBUS == DEVCTL_VBUS
}

/// How often the bus looks at VBUS while it waits for power, or is
/// suspended: no interrupt says VBUS came or went. (The wrapper's 1 ms tick
/// would serve, but turning it on hangs in SYNCBUSY once VBUS is gone.)
const VBUS_WATCH: Duration = Duration::from_millis(32);

impl Bus<'_> {
    /// The next bus event, from the handler's flags and VBUS's level.
    fn event(&mut self) -> Option<Event> {
        if !self.powered {
            if vbus_valid() {
                self.powered = true;
                BUS_EVENTS.store(0, Ordering::Release);
                return Some(Event::PowerDetected);
            }
            return None;
        }
        let events = BUS_EVENTS.load(Ordering::Acquire);
        // The cable gone: the bus idles (a suspend), then VBUS sags below
        // valid, slowly with no load on the connector's capacitance, so the
        // session lasts seconds and its end (DISCON) comes late if at all.
        // A host keeps VBUS up through a real suspend, so while suspended
        // the bus watches it.
        let suspended = self.suspended || events & USB_SUSPEND != 0;
        if events & USB_DISCON != 0 || (suspended && !vbus_valid()) {
            BUS_EVENTS.store(0, Ordering::Release);
            self.powered = false;
            self.suspended = false;
            ENABLED.store(0, Ordering::Release);
            wake_endpoints();
            return Some(Event::PowerRemoved);
        }
        if events & USB_RESET != 0 {
            BUS_EVENTS.fetch_and(!(USB_RESET | USB_SUSPEND | USB_RESUME), Ordering::AcqRel);
            self.suspended = false;
            // The reset cleared the address and every endpoint but 0;
            // embassy-usb enables them again on SET_CONFIGURATION.
            ENABLED.store(0, Ordering::Release);
            reset_interrupts();
            wake_endpoints();
            return Some(Event::Reset);
        }
        if events & USB_RESUME != 0 {
            BUS_EVENTS.fetch_and(!USB_RESUME, Ordering::AcqRel);
            self.suspended = false;
            return Some(Event::Resume);
        }
        if events & USB_SUSPEND != 0 {
            BUS_EVENTS.fetch_and(!USB_SUSPEND, Ordering::AcqRel);
            self.suspended = true;
            return Some(Event::Suspend);
        }
        None
    }
}

/// Wakes every endpoint's waiters, to see they're disabled.
fn wake_endpoints() {
    for n in 0..ENDPOINTS {
        IN_WAKERS[n].wake();
        OUT_WAKERS[n].wake();
    }
}

impl embassy_usb_driver::Bus for Bus<'_> {
    async fn enable(&mut self) {
        // Stale interrupts, which reading clears.
        let _ = (read8(INTRUSB), read16(INTRTX), read16(INTRRX));
        BUS_EVENTS.store(0, Ordering::Release);
        ENABLED.store(0, Ordering::Release);
        write8(FADDR, 0);
        reset_interrupts();
        // Attach: the PHY's pull-up on D+, then the chirp at reset.
        write8(POWR, self.speed | POWR_SOFTCONN);
    }

    async fn disable(&mut self) {
        write8(POWR, self.speed);
        write8(INTRUSBE, 0);
        ENABLED.store(0, Ordering::Release);
    }

    async fn poll(&mut self) -> Event {
        loop {
            // Unpowered or suspended, VBUS is watched as well.
            let watch = !self.powered || self.suspended;
            let event = poll_fn(|cx| {
                BUS_WAKER.register(cx.waker());
                self.event().map_or(Poll::Pending, Poll::Ready)
            });
            if !watch {
                return event.await;
            }
            if let Either::First(event) = select(event, Timer::after(VBUS_WATCH)).await {
                return event;
            }
        }
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        let n = ep_addr.index();
        if n == 0 || n >= ENDPOINTS {
            return;
        }
        let config = if ep_addr.is_in() { self.in_eps[n] } else { self.out_eps[n] };
        let Some(config) = config else { return };
        indexed(n, || {
            if enabled {
                set_up_endpoint(ep_addr.direction(), config);
            }
            let register = if ep_addr.is_in() { INTRTXE } else { INTRRXE };
            let mask = read16(register);
            write16(register, if enabled { mask | (1 << n) } else { mask & !(1 << n) });
        });
        if enabled {
            ENABLED.fetch_or(enabled_bit(ep_addr), Ordering::AcqRel);
        } else {
            ENABLED.fetch_and(!enabled_bit(ep_addr), Ordering::AcqRel);
        }
        if ep_addr.is_in() { IN_WAKERS[n].wake() } else { OUT_WAKERS[n].wake() }
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        let n = ep_addr.index();
        if n >= ENDPOINTS {
            return;
        }
        indexed(n, || {
            if n == 0 {
                write8(CSR0L, if stalled { CSR0_SENDSTALL } else { 0 });
            } else if ep_addr.is_in() {
                let flush = if read8(TXCSRL) & TX_FIFONE != 0 { TX_FLUSHFIFO } else { 0 };
                // Stalled: SENDSTALL, the FIFO dropped. Cleared: SENDSTALL
                // and SENTSTALL off, the toggle back to DATA0.
                write8(TXCSRL, if stalled { TX_SENDSTALL | TX_KEEP | flush } else { TX_CLRDT | TX_INCOMPTX | TX_FIFONE });
            } else {
                write8(RXCSRL, if stalled { RX_SENDSTALL | RX_KEEP } else { RX_CLRDATATOG | RX_OVERRUN | RX_RXPKTRDY });
            }
        });
        if ep_addr.is_in() { IN_WAKERS[n].wake() } else { OUT_WAKERS[n].wake() }
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        let n = ep_addr.index();
        if n >= ENDPOINTS {
            return false;
        }
        indexed(n, || match (n, ep_addr.direction()) {
            (0, _) => read8(CSR0L) & CSR0_SENDSTALL != 0,
            (_, Direction::In) => read8(TXCSRL) & TX_SENDSTALL != 0,
            (_, Direction::Out) => read8(RXCSRL) & RX_SENDSTALL != 0,
        })
    }

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        // Resume signalling for 10 ms (the data sheet's 10 to 15).
        write8(POWR, read8(POWR) | POWR_RESUME);
        Timer::after_millis(10).await;
        write8(POWR, read8(POWR) & !POWR_RESUME);
        self.suspended = false;
        Ok(())
    }
}

/// Sets the selected endpoint up from its allocation, empty, unstalled and
/// at DATA0.
fn set_up_endpoint(direction: Direction, config: EndpointConfig) {
    let size = config.fifo_size | if config.double { FIFOSZ_DPB } else { 0 };
    let start = config.fifo_start / 8;
    if direction == Direction::In {
        write16(TXMAXP, config.max_packet_size);
        write8(TXFIFOSZ, size);
        write16(TXFIFOADD, start);
        write8(TXCSRH, TXCSRH_MODE_TX);
        // Each flush drops one packet; a double-buffered FIFO may hold two.
        for _ in 0..2 {
            if read8(TXCSRL) & TX_FIFONE != 0 {
                write8(TXCSRL, TX_FLUSHFIFO | TX_FIFONE);
            }
        }
        write8(TXCSRL, TX_CLRDT);
    } else {
        write16(RXMAXP, config.max_packet_size);
        write8(RXFIFOSZ, size);
        write16(RXFIFOADD, start);
        // At high speed an interrupt OUT endpoint ACKs rather than NYETs.
        write8(RXCSRH, if config.ep_type == EndpointType::Interrupt { RXCSRH_DISNYET } else { 0 });
        for _ in 0..2 {
            if read8(RXCSRL) & RX_RXPKTRDY != 0 {
                write8(RXCSRL, RX_FLUSHFIFO | RX_RXPKTRDY);
            }
        }
        write8(RXCSRL, RX_CLRDATATOG);
    }
}

/// Moves `len` bytes between `address` and endpoint `n`'s FIFO on DMA
/// `channel` (mode 0, the one packet in or for the FIFO), into the FIFO if
/// `to_fifo`. `Err` on a bus error.
///
/// # Safety
///
/// `address..address + len` is SRAM the DMA may write (or read, if
/// `to_fifo`) until this future completes or is dropped: if dropped, it
/// waits for the transfer to finish.
async unsafe fn dma_transfer(channel: usize, n: usize, address: usize, len: usize, to_fifo: bool) -> Result<(), ()> {
    let bit = 1 << channel;
    DMA_DONE.fetch_and(!bit, Ordering::AcqRel);
    #[expect(clippy::cast_possible_truncation, reason = "SRAM addresses and packet lengths fit 32 bits")]
    {
        write32(BASE + dma(channel, DMA_ADDR), address as u32);
        write32(BASE + dma(channel, DMA_COUNT), len as u32);
        let direction = if to_fifo { DMA_TO_FIFO } else { 0 };
        write32(BASE + dma(channel, DMA_CNTL), DMA_EN | DMA_IE | DMA_BURST | direction | ((n as u32) << DMA_EP_SHIFT));
    }
    // Dropped part way: wait out the transfer (a packet is microseconds)
    // before the buffer can go, then drop the half-handled packet, whose
    // FIFO pointer has moved: the OUT packet is lost, the IN one unsent.
    let guard = OnDrop::new(|| {
        while DMA_DONE.load(Ordering::Acquire) & bit == 0 && read32(BASE + dma(channel, DMA_COUNT)) != 0 {
            core::hint::spin_loop();
        }
        write32(BASE + dma(channel, DMA_CNTL), 0);
        indexed(n, || {
            if to_fifo {
                write8(TXCSRL, TX_FLUSHFIFO | TX_FIFONE);
            } else {
                write8(RXCSRL, (read8(RXCSRL) & RX_SENDSTALL) | RX_SENTSTALL | RX_OVERRUN);
            }
        });
    });
    poll_fn(|cx| {
        DMA_WAKERS[channel].register(cx.waker());
        if DMA_DONE.load(Ordering::Acquire) & bit != 0 { Poll::Ready(()) } else { Poll::Pending }
    })
    .await;
    guard.defuse();
    let control = read32(BASE + dma(channel, DMA_CNTL));
    write32(BASE + dma(channel, DMA_CNTL), 0);
    if control & DMA_ERR != 0 { Err(()) } else { Ok(()) }
}

/// Runs a closure when dropped, unless defused.
struct OnDrop<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> OnDrop<F> {
    const fn new(f: F) -> Self {
        Self(Some(f))
    }

    fn defuse(mut self) {
        self.0 = None;
    }
}

impl<F: FnOnce()> Drop for OnDrop<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

/// Endpoint direction markers.
pub struct In;
/// Endpoint direction markers.
pub struct Out;

/// An IN or OUT endpoint other than 0.
pub struct Endpoint<'d, D> {
    info: EndpointInfo,
    /// Its DMA channel, if it has one.
    dma: Option<u8>,
    _marker: PhantomData<(&'d (), D)>,
}

impl<D> Endpoint<'_, D> {
    const fn new(addr: EndpointAddress, ep_type: EndpointType, max_packet_size: u16, interval_ms: u8, dma: Option<u8>) -> Self {
        Self { info: EndpointInfo { addr, ep_type, max_packet_size, interval_ms }, dma, _marker: PhantomData }
    }

    /// The DMA channel to move `len` bytes at `address` with, if this
    /// endpoint has one and the DMA can reach the buffer.
    fn dma_for(&self, address: usize, len: usize) -> Option<usize> {
        let channel = self.dma?;
        (len > 0 && address.is_multiple_of(4) && SRAM.contains(&address) && SRAM.contains(&(address + len - 1))).then_some(usize::from(channel))
    }

    fn waker(&self) -> &'static AtomicWaker {
        let n = self.info.addr.index();
        if self.info.addr.is_in() { &IN_WAKERS[n] } else { &OUT_WAKERS[n] }
    }
}

impl<D> embassy_usb_driver::Endpoint for Endpoint<'_, D> {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        let addr = self.info.addr;
        let waker = self.waker();
        poll_fn(|cx| {
            waker.register(cx.waker());
            if is_enabled(addr) { Poll::Ready(()) } else { Poll::Pending }
        })
        .await;
    }
}

impl embassy_usb_driver::EndpointOut for Endpoint<'_, Out> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        let addr = self.info.addr;
        let n = addr.index();
        let waker = self.waker();
        let len = poll_fn(|cx| {
            waker.register(cx.waker());
            if !is_enabled(addr) {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            match indexed(n, || (read8(RXCSRL) & RX_RXPKTRDY != 0).then(|| usize::from(read16(RXCOUNT)))) {
                Some(len) => Poll::Ready(Ok(len)),
                None => Poll::Pending,
            }
        })
        .await?;
        let result = if len > buf.len() {
            Err(EndpointError::BufferOverflow)
        } else if let Some(channel) = self.dma_for(buf.as_ptr() as usize, len) {
            // SAFETY: `buf` stays borrowed until the transfer is done, even
            // if this future is dropped first (the transfer's guard waits).
            match unsafe { dma_transfer(channel, n, buf.as_mut_ptr() as usize, len, false) }.await {
                Ok(()) => Ok(len),
                Err(()) => Err(EndpointError::Disabled),
            }
        } else {
            fifo_read(n, &mut buf[..len]);
            Ok(len)
        };
        // The packet's done with (or dropped): RXPKTRDY off, the other bits
        // that writing 0 clears left as they are.
        indexed(n, || write8(RXCSRL, (read8(RXCSRL) & RX_SENDSTALL) | RX_SENTSTALL | RX_OVERRUN));
        result
    }
}

impl embassy_usb_driver::EndpointIn for Endpoint<'_, In> {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        let addr = self.info.addr;
        let n = addr.index();
        if buf.len() > usize::from(self.info.max_packet_size) {
            return Err(EndpointError::BufferOverflow);
        }
        let waker = self.waker();
        // Room for a packet: TXPKTRDY clear (a double-buffered FIFO clears
        // it again while its first packet waits for the host).
        poll_fn(|cx| {
            waker.register(cx.waker());
            if !is_enabled(addr) {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            if indexed(n, || read8(TXCSRL) & TX_TXPKTRDY == 0) { Poll::Ready(Ok(())) } else { Poll::Pending }
        })
        .await?;
        if let Some(channel) = self.dma_for(buf.as_ptr() as usize, buf.len()) {
            // SAFETY: as in `read`; the DMA only reads `buf`.
            if unsafe { dma_transfer(channel, n, buf.as_ptr() as usize, buf.len(), true) }.await.is_err() {
                return Err(EndpointError::Disabled);
            }
        } else {
            fifo_write(n, buf);
        }
        indexed(n, || write8(TXCSRL, (read8(TXCSRL) & TX_SENDSTALL) | TX_KEEP | TX_TXPKTRDY));
        Ok(())
    }
}

/// Endpoint 0, through the core's control handshake: a SETUP stays in the
/// FIFO (RXPKTRDY) until the request's next step services it, so `accept`
/// and `reject` can end a request without data in one write (DATAEND, or
/// SENDSTALL), and the last OUT data packet likewise waits for the answer.
pub struct ControlPipe<'d> {
    max_packet_size: u16,
    /// The SETUP's RXPKTRDY, not yet serviced.
    setup_pending: bool,
    /// The last OUT data packet's RXPKTRDY, not yet serviced.
    out_pending: bool,
    _lifetime: PhantomData<&'d ()>,
}

/// Why endpoint 0 stopped waiting.
enum Ep0 {
    Ready,
    /// The host ended the request early (SETUPEND), most likely with a new
    /// SETUP.
    Ended,
}

impl ControlPipe<'_> {
    /// Waits for `ready(CSR0L)`, or for the host to end the request.
    async fn wait(ready: impl Fn(u8) -> bool, waker: &AtomicWaker) -> Ep0 {
        poll_fn(|cx| {
            waker.register(cx.waker());
            let csr = indexed(0, || read8(CSR0L));
            if csr & CSR0_SETUPEND != 0 {
                Poll::Ready(Ep0::Ended)
            } else if ready(csr) {
                Poll::Ready(Ep0::Ready)
            } else {
                Poll::Pending
            }
        })
        .await
    }

    /// Services whatever packet the request left in the FIFO, with `extra`
    /// (DATAEND, SENDSTALL) in the same write.
    fn service(&mut self, extra: u8) {
        let serviced = if self.setup_pending || self.out_pending { CSR0_SERVICEDRXPKTRDY } else { 0 };
        self.setup_pending = false;
        self.out_pending = false;
        if serviced | extra != 0 {
            indexed(0, || write8(CSR0L, serviced | extra));
        }
    }
}

impl embassy_usb_driver::ControlPipe for ControlPipe<'_> {
    fn max_packet_size(&self) -> usize {
        usize::from(self.max_packet_size)
    }

    async fn setup(&mut self) -> [u8; 8] {
        loop {
            let len = poll_fn(|cx| {
                OUT_WAKERS[0].register(cx.waker());
                indexed(0, || {
                    let csr = read8(CSR0L);
                    // A stall sent, or a request the host ended early: both
                    // done with.
                    if csr & CSR0_SENTSTALL != 0 {
                        write8(CSR0L, 0);
                    }
                    if csr & CSR0_SETUPEND != 0 {
                        write8(CSR0L, CSR0_SERVICEDSETUPEND);
                    }
                    if csr & CSR0_RXPKTRDY != 0 { Poll::Ready(usize::from(read16(RXCOUNT))) } else { Poll::Pending }
                })
            })
            .await;
            if len != 8 {
                // Not a SETUP: an OUT packet nothing waits for.
                indexed(0, || write8(CSR0L, CSR0_SERVICEDRXPKTRDY));
                continue;
            }
            let mut packet = [0; 8];
            fifo_read(0, &mut packet);
            self.setup_pending = true;
            self.out_pending = false;
            return packet;
        }
    }

    async fn data_out(&mut self, buf: &mut [u8], _first: bool, last: bool) -> Result<usize, EndpointError> {
        // The SETUP, or the packet before, serviced: the host may send.
        self.service(0);
        let Ep0::Ready = Self::wait(|csr| csr & CSR0_RXPKTRDY != 0, &OUT_WAKERS[0]).await else {
            return Err(EndpointError::Disabled);
        };
        let len = indexed(0, || usize::from(read16(RXCOUNT)));
        if len > buf.len() {
            indexed(0, || write8(CSR0L, CSR0_SERVICEDRXPKTRDY | CSR0_SENDSTALL));
            return Err(EndpointError::BufferOverflow);
        }
        fifo_read(0, &mut buf[..len]);
        // The last packet waits for `accept` (DATAEND) or `reject`.
        self.out_pending = true;
        if !last {
            self.service(0);
        }
        Ok(len)
    }

    async fn data_in(&mut self, data: &[u8], _first: bool, last: bool) -> Result<(), EndpointError> {
        if data.len() > usize::from(self.max_packet_size) {
            return Err(EndpointError::BufferOverflow);
        }
        self.service(0);
        if let Ep0::Ended = Self::wait(|csr| csr & CSR0_TXPKTRDY == 0, &IN_WAKERS[0]).await {
            return Err(EndpointError::Disabled);
        }
        fifo_write(0, data);
        // DATAEND with the last packet: the core then runs the status stage.
        indexed(0, || write8(CSR0L, CSR0_TXPKTRDY | if last { CSR0_DATAEND } else { 0 }));
        match Self::wait(|csr| csr & CSR0_TXPKTRDY == 0, &IN_WAKERS[0]).await {
            Ep0::Ready => Ok(()),
            Ep0::Ended => Err(EndpointError::Disabled),
        }
    }

    async fn accept(&mut self) {
        // DATAEND: the core answers the status stage itself.
        self.service(CSR0_DATAEND);
    }

    async fn reject(&mut self) {
        self.service(CSR0_SENDSTALL);
    }

    async fn accept_set_address(&mut self, addr: u8) {
        // The new address takes effect after the status stage, which ends
        // with an endpoint 0 interrupt (or a new SETUP's).
        EP0_EVENT.store(false, Ordering::Release);
        self.accept().await;
        poll_fn(|cx| {
            IN_WAKERS[0].register(cx.waker());
            if EP0_EVENT.load(Ordering::Acquire) { Poll::Ready(()) } else { Poll::Pending }
        })
        .await;
        write8(FADDR, addr & 0x7F);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn driver() -> Driver<'static> {
        Driver { speed: POWR_HSEN, out_eps: [None; ENDPOINTS], in_eps: [None; ENDPOINTS], fifo_free: EP0_FIFO_BYTES, dma_free: DMA_CHANNELS, _lifetime: PhantomData }
    }

    #[test]
    fn fifos_follow_endpoint_zero_and_double_for_bulk() {
        let mut d = driver();
        assert_eq!(d.alloc(Direction::In, EndpointType::Interrupt, None, 10), Ok(1));
        assert_eq!(d.alloc(Direction::Out, EndpointType::Bulk, None, 512), Ok(1));
        assert_eq!(d.alloc(Direction::In, EndpointType::Bulk, None, 512), Ok(2));
        let interrupt = d.in_eps[1].unwrap();
        assert_eq!((interrupt.fifo_start, interrupt.fifo_size, interrupt.double), (64, 1, false));
        let out = d.out_eps[1].unwrap();
        assert_eq!((out.fifo_start, out.fifo_size, out.double), (80, 6, true));
        let bulk_in = d.in_eps[2].unwrap();
        assert_eq!((bulk_in.fifo_start, bulk_in.fifo_size, bulk_in.double), (80 + 1024, 6, true));
        assert_eq!(d.fifo_free, 80 + 2048);
    }

    #[test]
    fn dma_channels_go_to_the_first_eight_bulk_endpoints() {
        let mut d = driver();
        assert_eq!(d.dma_channel(EndpointType::Interrupt), None);
        for channel in 0..8 {
            assert_eq!(d.dma_channel(EndpointType::Bulk), Some(channel));
        }
        assert_eq!(d.dma_channel(EndpointType::Bulk), None);
        let mut d = driver();
        d.dma_free = 0;
        assert_eq!(d.dma_channel(EndpointType::Bulk), None);
    }

    #[test]
    fn allocation_stops_at_the_fifo_ram_and_the_limits() {
        let mut d = driver();
        // Seven double 512-byte bulk FIFOs fit beside endpoint 0's; the
        // eighth doesn't.
        for n in 1..ENDPOINTS {
            assert_eq!(d.alloc(Direction::Out, EndpointType::Bulk, None, 512), Ok(n));
        }
        assert_eq!(d.alloc(Direction::In, EndpointType::Bulk, None, 512), Err(EndpointAllocError));
        let mut d = driver();
        assert_eq!(d.alloc(Direction::In, EndpointType::Bulk, None, 1024), Err(EndpointAllocError));
        assert_eq!(d.alloc(Direction::In, EndpointType::Isochronous, None, 64), Err(EndpointAllocError));
        assert_eq!(d.alloc(Direction::In, EndpointType::Interrupt, Some(EndpointAddress::from_parts(0, Direction::In)), 8), Err(EndpointAllocError));
        assert_eq!(d.alloc(Direction::In, EndpointType::Interrupt, Some(EndpointAddress::from_parts(3, Direction::In)), 8), Ok(3));
        assert_eq!(d.alloc(Direction::In, EndpointType::Interrupt, Some(EndpointAddress::from_parts(3, Direction::In)), 8), Err(EndpointAllocError));
    }
}

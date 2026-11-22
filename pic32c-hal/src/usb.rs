// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! An [`embassy_usb_driver`] for the full-speed USB device controller of
//! Microchip's PIC32CK (USBFS, DS60001795 chapter 38): the same IP as the
//! SAM D5x/E5x `USB`, whose endpoint descriptors and buffers live in SRAM.
//!
//! Full speed, device mode, eight endpoint numbers each with an IN and an
//! OUT endpoint of up to 64 bytes, one bank each (no ping-pong), control,
//! bulk and interrupt (no isochronous). No VBUS detection: the controller
//! has none, so the bus reports power at once.
//!
//! The module drives the registers itself. [`Driver::new`] takes the `USB`
//! peripheral, a [`bind_interrupts!`](crate::bind_interrupts) type binding
//! all four of its vectors (`USB_OTHER`, `USB_SOF`, `USB_TRCPT0`,
//! `USB_TRCPT1`) to [`InterruptHandler`], and a generic clock generator at
//! 48 MHz (crystal-derived, for USB's ±0.25 %), which it feeds the
//! controller from (GCLK channel 46). The bus clocks are on at reset. The driver takes its
//! pins, D+ (PD04) and D- (PD03), and gives them to the controller
//! (function H): without them it attaches to nothing and sees the bus as
//! SE0.
//!
//! The descriptors and buffers are this module's statics. The controller's
//! DMA reaches only SRAM, so data is copied to and from them; with the
//! SG01's default fuses (everything Secure) any SRAM serves.

use core::cell::UnsafeCell;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::ptr::{addr_of_mut, read_volatile, write_volatile};
use core::sync::atomic::{AtomicU8, AtomicU16, Ordering, compiler_fence};
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver::{Direction, EndpointAddress, EndpointAllocError, EndpointConfig, EndpointDisabledError, EndpointError, EndpointInfo, EndpointType, Event, Speed, Unsupported};

use crate::gpio::Function;
use crate::interrupt::typelevel::{self, Binding, Handler};
use crate::interrupt::{Interrupt, InterruptExt};

/// USB's clock, and its GCLK channel.
const USB_HZ: u32 = 48_000_000;
const GCLK_CHANNEL: usize = 46;
use crate::{Peri, peripherals};

/// The controller's registers.
const BASE: usize = 0x4502_A000;
const CTRLA: usize = 0x00;
const SYNCBUSY: usize = 0x02;
const CTRLB: usize = 0x08;
const DADD: usize = 0x0A;
const INTENCLR: usize = 0x14;
const INTENSET: usize = 0x18;
const INTFLAG: usize = 0x1C;
const EPINTSMRY: usize = 0x20;
const DESCADD: usize = 0x24;
const PADCAL: usize = 0x28;
/// Endpoint `n`'s registers are at 0x100 + 0x20 n.
const EP_BASE: usize = 0x100;
const EPCFG: usize = 0x0;
const EPSTATUSCLR: usize = 0x4;
const EPSTATUSSET: usize = 0x5;
const EPSTATUS: usize = 0x6;
const EPINTFLAG: usize = 0x7;
const EPINTENCLR: usize = 0x8;
const EPINTENSET: usize = 0x9;

const CTRLA_SWRST: u8 = 1 << 0;
const CTRLA_ENABLE: u8 = 1 << 1;
const CTRLB_DETACH: u16 = 1 << 0;
const CTRLB_UPRSM: u16 = 1 << 1;
const DADD_ADDEN: u8 = 1 << 7;
const INT_SUSPEND: u16 = 1 << 0;
const INT_EORST: u16 = 1 << 3;
const INT_WAKEUP: u16 = 1 << 4;
const INT_EORSM: u16 = 1 << 5;

const DTGLOUT: u8 = 1 << 0;
const DTGLIN: u8 = 1 << 1;
const STALLRQ0: u8 = 1 << 4;
const STALLRQ1: u8 = 1 << 5;
const BK0RDY: u8 = 1 << 6;
const BK1RDY: u8 = 1 << 7;
const TRCPT0: u8 = 1 << 0;
const TRCPT1: u8 = 1 << 1;
const TRFAIL0: u8 = 1 << 2;
const TRFAIL1: u8 = 1 << 3;
const RXSTP: u8 = 1 << 4;
const STALL0: u8 = 1 << 5;
const STALL1: u8 = 1 << 6;
/// The flags of bank 0 (OUT and SETUP) and bank 1 (IN).
const OUT_FLAGS: u8 = TRCPT0 | TRFAIL0 | RXSTP | STALL0;
const IN_FLAGS: u8 = TRCPT1 | TRFAIL1 | STALL1;

/// EPCFG endpoint types.
const EPTYPE_CONTROL: u8 = 1;
const EPTYPE_BULK: u8 = 3;
const EPTYPE_INTERRUPT: u8 = 4;

/// The USB pads' calibration, in CAL OTP (FCCFG68), laid out as PADCAL.
const PADCAL_OTP: usize = 0x0A00_7190;
const PADCAL_MASK: u16 = 0x77DF;

/// Endpoint numbers, and the largest packet.
const ENDPOINTS: usize = 8;
const MAX_PACKET: usize = 64;
/// Each bank's buffer: a packet and room for the CRC the controller may
/// write after a short one.
const BUFFER_WORDS: usize = MAX_PACKET / 4 + 1;

/// One bank of an endpoint descriptor (16 bytes).
#[repr(C)]
struct Bank {
    addr: u32,
    pcksize: u32,
    extreg: u16,
    status_bk: u8,
    _reserved: [u8; 5],
}

const PCKSIZE_COUNT_MASK: u32 = 0x3FFF;
const PCKSIZE_MULTI_SHIFT: u32 = 14;
const PCKSIZE_SIZE_SHIFT: u32 = 28;

#[repr(C, align(4))]
struct Ram {
    descriptors: [[Bank; 2]; ENDPOINTS],
    buffers: [[[u32; BUFFER_WORDS]; 2]; ENDPOINTS],
}

/// The descriptors and buffers, shared with the controller's DMA.
struct Shared(UnsafeCell<Ram>);

// SAFETY: each bank belongs to one endpoint object, which alone touches it,
// handing it to the controller by BKxRDY and taking it back on TRCPTx.
unsafe impl Sync for Shared {}

static RAM: Shared = Shared(UnsafeCell::new(Ram {
    descriptors: [const {
        [const {
            Bank {
                addr: 0,
                pcksize: 0,
                extreg: 0,
                status_bk: 0,
                _reserved: [0; 5],
            }
        }; 2]
    }; ENDPOINTS],
    buffers: [[[0; BUFFER_WORDS]; 2]; ENDPOINTS],
}));

static BUS_WAKER: AtomicWaker = AtomicWaker::new();
static IN_WAKERS: [AtomicWaker; ENDPOINTS] = [const { AtomicWaker::new() }; ENDPOINTS];
static OUT_WAKERS: [AtomicWaker; ENDPOINTS] = [const { AtomicWaker::new() }; ENDPOINTS];
/// Device interrupt flags taken by the handler, for [`Bus::poll`].
static BUS_EVENTS: AtomicU16 = AtomicU16::new(0);
/// A `SET_ADDRESS`'s address with ADDEN, for the handler to write to DADD
/// when the status stage's IN completes; 0 for none.
static PENDING_ADDRESS: AtomicU8 = AtomicU8::new(0);
/// Which endpoints are enabled: bit n OUT n, bit 8 + n IN n.
static ENABLED: AtomicU16 = AtomicU16::new(0);

fn read8(offset: usize) -> u8 {
    // SAFETY: a USB register, read whole.
    unsafe { read_volatile((BASE + offset) as *const u8) }
}

fn write8(offset: usize, value: u8) {
    // SAFETY: a USB register; the endpoint registers are each one
    // endpoint's, and the device registers the bus's or the handler's.
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

const fn ep(n: usize, register: usize) -> usize {
    EP_BASE + 0x20 * n + register
}

fn bank(n: usize, bank: usize) -> *mut Bank {
    // SAFETY: a place in RAM, not a reference: the controller writes it too.
    unsafe { addr_of_mut!((*RAM.0.get()).descriptors[n][bank]) }
}

fn buffer(n: usize, bank: usize) -> *mut u8 {
    // SAFETY: as for `bank`.
    unsafe { addr_of_mut!((*RAM.0.get()).buffers[n][bank]).cast() }
}

fn set_pcksize(n: usize, b: usize, value: u32) {
    // SAFETY: the bank's descriptor word, which only this endpoint writes
    // while the controller doesn't own the bank.
    unsafe { write_volatile(addr_of_mut!((*bank(n, b)).pcksize), value) }
}

fn byte_count(n: usize, b: usize) -> usize {
    // SAFETY: as for `set_pcksize`, after the controller has finished.
    (unsafe { read_volatile(addr_of_mut!((*bank(n, b)).pcksize)) } & PCKSIZE_COUNT_MASK) as usize
}

/// PCKSIZE.SIZE for a packet size of up to 64.
const fn size_code(max_packet_size: u16) -> u32 {
    match max_packet_size {
        0..=8 => 0,
        9..=16 => 1,
        17..=32 => 2,
        _ => 3,
    }
}

/// Points endpoint `n`'s banks at their buffers, with packets of
/// `max_packet_size`.
fn set_up_banks(n: usize, max_packet_size: u16) {
    for b in 0..2 {
        // SAFETY: the descriptor's address word, before the endpoint runs.
        unsafe { write_volatile(addr_of_mut!((*bank(n, b)).addr), buffer(n, b) as u32) };
        set_pcksize(n, b, size_code(max_packet_size) << PCKSIZE_SIZE_SHIFT);
    }
}

/// Bank 0 ready to take one OUT packet of up to `max_packet_size`.
fn arm_out(n: usize, max_packet_size: u16) {
    set_pcksize(n, 0, (size_code(max_packet_size) << PCKSIZE_SIZE_SHIFT) | (u32::from(max_packet_size) << PCKSIZE_MULTI_SHIFT));
    write8(ep(n, EPINTFLAG), TRCPT0 | TRFAIL0);
    compiler_fence(Ordering::SeqCst);
    write8(ep(n, EPSTATUSCLR), BK0RDY);
}

/// Bank 1 loaded with `data` and handed to the controller.
fn send_in(n: usize, max_packet_size: u16, data: &[u8]) {
    // SAFETY: the bank's buffer, which the controller doesn't own while
    // BK1RDY is clear; `data` is at most a packet.
    unsafe { core::ptr::copy_nonoverlapping(data.as_ptr(), buffer(n, 1), data.len()) };
    #[expect(clippy::cast_possible_truncation, reason = "at most a packet")]
    let len = data.len() as u32;
    set_pcksize(n, 1, (size_code(max_packet_size) << PCKSIZE_SIZE_SHIFT) | len);
    write8(ep(n, EPINTFLAG), TRCPT1 | TRFAIL1);
    compiler_fence(Ordering::SeqCst);
    write8(ep(n, EPSTATUSSET), BK1RDY);
}

/// Waits for any of `mask` among endpoint `n`'s flags, enabling their
/// interrupts while it waits; returns those set, without clearing them.
/// Stops early, with `None`, once `stop` says so.
async fn wait_flags(n: usize, mask: u8, waker: &AtomicWaker, stop: impl Fn() -> bool) -> Option<u8> {
    poll_fn(|cx| {
        waker.register(cx.waker());
        let flags = read8(ep(n, EPINTFLAG)) & mask;
        if flags != 0 {
            Poll::Ready(Some(flags))
        } else if stop() {
            Poll::Ready(None)
        } else {
            write8(ep(n, EPINTENSET), mask);
            Poll::Pending
        }
    })
    .await
}

fn enabled_bit(addr: EndpointAddress) -> u16 {
    let shift = if addr.is_in() { 8 } else { 0 };
    1 << (addr.index() + shift)
}

fn is_enabled(addr: EndpointAddress) -> bool {
    ENABLED.load(Ordering::Acquire) & enabled_bit(addr) != 0
}

/// The controller's interrupt handler, for all four of its vectors: bind
/// them with [`bind_interrupts!`](crate::bind_interrupts).
pub struct InterruptHandler {
    _private: (),
}

macro_rules! handler {
    ($($irq:ident),*) => {
        $(
            impl Handler<typelevel::$irq> for InterruptHandler {
                unsafe fn on_interrupt() {
                    on_interrupt();
                }
            }
        )*
    };
}

handler!(USB_OTHER, USB_SOF, USB_TRCPT0, USB_TRCPT1);

/// Takes the device events for [`Bus::poll`](embassy_usb_driver::Bus::poll)
/// and masks each endpoint interrupt that fired, waking its waiter, which
/// clears the flag.
fn on_interrupt() {
    let device = read16(INTFLAG) & read16(INTENSET) & (INT_SUSPEND | INT_EORST | INT_WAKEUP | INT_EORSM);
    if device != 0 {
        if device & INT_EORST != 0 {
            // A reset abandons a SET_ADDRESS in progress.
            PENDING_ADDRESS.store(0, Ordering::Relaxed);
        }
        write16(INTFLAG, device);
        BUS_EVENTS.fetch_or(device, Ordering::AcqRel);
        BUS_WAKER.wake();
        // A control stage in progress ends on a reset or a suspend.
        OUT_WAKERS[0].wake();
        IN_WAKERS[0].wake();
    }
    let summary = read16(EPINTSMRY);
    for n in 0..ENDPOINTS {
        if summary & (1 << n) == 0 {
            continue;
        }
        let flags = read8(ep(n, EPINTFLAG)) & read8(ep(n, EPINTENSET));
        if flags != 0 {
            write8(ep(n, EPINTENCLR), flags);
        }
        // `SET_ADDRESS`'s status stage is done: the address applies from the
        // host's next transaction, which may follow within 2 ms, sooner
        // than the waiting task may run.
        if n == 0 && flags & TRCPT1 != 0 {
            let address = PENDING_ADDRESS.swap(0, Ordering::Relaxed);
            if address != 0 {
                write8(DADD, address);
            }
        }
        // On endpoint 0 a SETUP ends a wait for either bank, so any flag
        // wakes both.
        if flags & OUT_FLAGS != 0 || (n == 0 && flags != 0) {
            OUT_WAKERS[n].wake();
        }
        if flags & IN_FLAGS != 0 || (n == 0 && flags != 0) {
            IN_WAKERS[n].wake();
        }
    }
}

/// The driver, before [`start`](embassy_usb_driver::Driver::start).
pub struct Driver<'d> {
    out_types: [Option<(EndpointType, u16)>; ENDPOINTS],
    in_types: [Option<(EndpointType, u16)>; ENDPOINTS],
    _lifetime: PhantomData<&'d ()>,
}

impl<'d> Driver<'d> {
    /// The driver for the controller, on its pins, clocked from generic
    /// clock generator `generator`, its four interrupts unmasked.
    ///
    /// # Panics
    ///
    /// If `generator` doesn't run at 48 MHz.
    #[must_use]
    #[allow(clippy::needless_pass_by_value, reason = "the controller has the pins for 'd")]
    pub fn new(
        _usb: Peri<'d, peripherals::USB>,
        _irqs: impl Binding<typelevel::USB_OTHER, InterruptHandler> + Binding<typelevel::USB_SOF, InterruptHandler> + Binding<typelevel::USB_TRCPT0, InterruptHandler> + Binding<typelevel::USB_TRCPT1, InterruptHandler> + 'd,
        dp: Peri<'d, peripherals::PD04>,
        dm: Peri<'d, peripherals::PD03>,
        generator: usize,
    ) -> Self {
        let hz = crate::clocks::generator_hz(generator);
        assert!(hz == Some(USB_HZ), "USB needs 48 MHz; generator {generator} runs at {hz:?} Hz");
        crate::clocks::enable_channel(GCLK_CHANNEL, generator);
        crate::gpio::set_function(&*dp, Function::H);
        crate::gpio::set_function(&*dm, Function::H);
        for irq in [Interrupt::USB_OTHER, Interrupt::USB_SOF, Interrupt::USB_TRCPT0, Interrupt::USB_TRCPT1] {
            irq.unpend();
            // SAFETY: the binding makes each vector call the handler.
            unsafe { irq.enable() };
        }
        Self {
            out_types: [None; ENDPOINTS],
            in_types: [None; ENDPOINTS],
            _lifetime: PhantomData,
        }
    }

    fn alloc(types: &mut [Option<(EndpointType, u16)>; ENDPOINTS], ep_type: EndpointType, ep_addr: Option<EndpointAddress>, max_packet_size: u16) -> Result<usize, EndpointAllocError> {
        if usize::from(max_packet_size) > MAX_PACKET || ep_type == EndpointType::Isochronous || ep_type == EndpointType::Control {
            return Err(EndpointAllocError);
        }
        let n = match ep_addr {
            Some(addr) if addr.index() > 0 && addr.index() < ENDPOINTS && types[addr.index()].is_none() => addr.index(),
            Some(_) => return Err(EndpointAllocError),
            None => (1..ENDPOINTS).find(|&n| types[n].is_none()).ok_or(EndpointAllocError)?,
        };
        types[n] = Some((ep_type, max_packet_size));
        Ok(n)
    }
}

impl<'d> embassy_usb_driver::Driver<'d> for Driver<'d> {
    type EndpointOut = Endpoint<'d, Out>;
    type EndpointIn = Endpoint<'d, In>;
    type Bus = Bus<'d>;

    fn alloc_endpoint_out(&mut self, config: &EndpointConfig) -> Result<Self::EndpointOut, EndpointAllocError> {
        let n = Self::alloc(&mut self.out_types, config.ep_type, config.addr, config.max_packet_size)?;
        Ok(Endpoint::new(EndpointInfo::new(EndpointAddress::from_parts(n, Direction::Out), config.ep_type, config.max_packet_size, config.interval)))
    }

    fn alloc_endpoint_in(&mut self, config: &EndpointConfig) -> Result<Self::EndpointIn, EndpointAllocError> {
        let n = Self::alloc(&mut self.in_types, config.ep_type, config.addr, config.max_packet_size)?;
        Ok(Endpoint::new(EndpointInfo::new(EndpointAddress::from_parts(n, Direction::In), config.ep_type, config.max_packet_size, config.interval)))
    }

    fn start(self, control_max_packet_size: u16) -> Self::Bus {
        let control = control_max_packet_size.min(64);
        Bus {
            out_types: self.out_types,
            in_types: self.in_types,
            control,
            powered: false,
            _lifetime: PhantomData,
        }
    }
}

/// Sets endpoint 0 up as the control endpoint, waiting for a SETUP.
fn set_up_control(max_packet_size: u16) {
    set_up_banks(0, max_packet_size);
    write8(ep(0, EPCFG), EPTYPE_CONTROL | (EPTYPE_CONTROL << 4));
    write8(ep(0, EPINTFLAG), 0xFF);
    write8(ep(0, EPSTATUSCLR), BK1RDY | STALLRQ0 | STALLRQ1);
    // Bank 0 empty, ready for a SETUP or OUT.
    arm_out(0, max_packet_size);
}

/// Whether a SETUP has come in that [`Bus::poll`](embassy_usb_driver::Bus::poll)
/// hasn't taken yet.
fn setup_waiting() -> bool {
    read8(ep(0, EPINTFLAG)) & RXSTP != 0
}

/// Bank 0 ready for the status stage's OUT, or for the next SETUP, unless
/// that SETUP has already come in. The host may send the next request as
/// soon as a status IN completes, before the task that waited for it runs
/// again; arming the bank then would set its byte count to 0, so `poll`
/// would drop the SETUP as short, and an OUT could overwrite it. The check
/// and the arming are in one critical section, so only the controller can
/// come between them, for a few cycles.
fn arm_control_out(max_packet_size: u16) {
    critical_section::with(|_| {
        if !setup_waiting() {
            arm_out(0, max_packet_size);
        }
    });
}

/// Whether a control stage can no longer complete: a reset or a suspend
/// waits for [`Bus::poll`](embassy_usb_driver::Bus::poll), which takes it.
fn control_aborted() -> bool {
    BUS_EVENTS.load(Ordering::Acquire) & (INT_EORST | INT_SUSPEND) != 0
}

/// Bus control, and endpoint 0.
pub struct Bus<'d> {
    out_types: [Option<(EndpointType, u16)>; ENDPOINTS],
    in_types: [Option<(EndpointType, u16)>; ENDPOINTS],
    control: u16,
    powered: bool,
    _lifetime: PhantomData<&'d ()>,
}

impl Bus<'_> {
    /// The bus events, in the order embassy-usb wants them: a reset first,
    /// which makes anything before it stale.
    fn poll_bus(&mut self) -> Poll<Event> {
        if !self.powered {
            self.powered = true;
            return Poll::Ready(Event::PowerDetected);
        }
        let events = BUS_EVENTS.load(Ordering::Acquire);
        if events & INT_EORST != 0 {
            BUS_EVENTS.fetch_and(!(INT_EORST | INT_SUSPEND | INT_WAKEUP | INT_EORSM), Ordering::AcqRel);
            // A reset leaves endpoint 0's configuration and clears the
            // others' and the address; set endpoint 0 up afresh, which
            // drops a SETUP from before it.
            set_up_control(self.control);
            ENABLED.store(0, Ordering::Release);
            for n in 0..ENDPOINTS {
                IN_WAKERS[n].wake();
                OUT_WAKERS[n].wake();
            }
            write16(INTENCLR, INT_WAKEUP);
            write16(INTENSET, INT_SUSPEND | INT_EORSM);
            return Poll::Ready(Event::Reset(Speed::Full));
        }
        if events & (INT_WAKEUP | INT_EORSM) != 0 {
            BUS_EVENTS.fetch_and(!(INT_WAKEUP | INT_EORSM), Ordering::AcqRel);
            write16(INTENCLR, INT_WAKEUP);
            write16(INTENSET, INT_SUSPEND);
            return Poll::Ready(Event::Resume);
        }
        if events & INT_SUSPEND != 0 {
            BUS_EVENTS.fetch_and(!INT_SUSPEND, Ordering::AcqRel);
            // Suspended, watch for the wake-up instead.
            write16(INTENCLR, INT_SUSPEND);
            write16(INTFLAG, INT_WAKEUP);
            write16(INTENSET, INT_WAKEUP);
            return Poll::Ready(Event::Suspend);
        }
        Poll::Pending
    }

    /// A SETUP, if one has come in: its eight bytes, the request it ends
    /// cleaned up after.
    fn poll_setup(&mut self) -> Poll<[u8; 8]> {
        if read8(ep(0, EPINTFLAG)) & RXSTP == 0 {
            write8(ep(0, EPINTENSET), RXSTP);
            return Poll::Pending;
        }
        compiler_fence(Ordering::SeqCst);
        let len = byte_count(0, 0);
        let mut packet = [0u8; 8];
        // SAFETY: bank 0's buffer, which a SETUP fills and leaves the CPU's
        // (BK0RDY set).
        unsafe { core::ptr::copy_nonoverlapping(buffer(0, 0), packet.as_mut_ptr(), 8) };
        // Start the request clean: the SETUP cancelled whatever was in
        // progress (the controller clears BK1RDY and the stalls).
        write8(ep(0, EPINTFLAG), RXSTP | TRCPT0 | TRCPT1 | TRFAIL0 | TRFAIL1 | STALL0 | STALL1);
        if len == 8 {
            return Poll::Ready(packet);
        }
        arm_out(0, self.control);
        write8(ep(0, EPINTENSET), RXSTP);
        Poll::Pending
    }

    /// Waits for `flag` on endpoint 0; `Err` if the request in progress
    /// ended first (a new SETUP, a reset or a suspend), leaving that for
    /// `poll`.
    async fn wait(&self, flag: u8, waker: &AtomicWaker) -> Result<(), EndpointDisabledError> {
        let flags = wait_flags(0, flag | RXSTP, waker, control_aborted).await.unwrap_or(RXSTP);
        if flags & RXSTP != 0 || control_aborted() { Err(EndpointDisabledError) } else { Ok(()) }
    }

    /// Sends one packet of `data` from bank 1 and waits until the host has
    /// it.
    async fn send(&self, data: &[u8]) -> Result<(), EndpointDisabledError> {
        send_in(0, self.control, data);
        self.wait(TRCPT1, &IN_WAKERS[0]).await?;
        write8(ep(0, EPINTFLAG), TRCPT1 | TRFAIL1);
        Ok(())
    }
}

impl embassy_usb_driver::Bus for Bus<'_> {
    async fn enable(&mut self) {
        write8(CTRLA, CTRLA_SWRST);
        while read8(SYNCBUSY) & CTRLA_SWRST != 0 {}
        // SAFETY: the descriptor table's address, which DESCADD takes.
        write_volatile_u32(BASE + DESCADD, unsafe { addr_of_mut!((*RAM.0.get()).descriptors) } as u32);
        // Device mode (MODE 0), then the pads' calibration, which the data
        // sheet loads once the controller is enabled.
        write8(CTRLA, CTRLA_ENABLE);
        while read8(SYNCBUSY) & CTRLA_ENABLE != 0 {}
        write16(PADCAL, padcal());
        // Full speed (SPDCONF 0), still detached.
        write16(CTRLB, CTRLB_DETACH);
        set_up_control(self.control);
        write16(INTFLAG, 0xFFFF);
        write16(INTENSET, INT_EORST | INT_SUSPEND | INT_EORSM);
        write16(CTRLB, 0);
    }

    async fn disable(&mut self) {
        write16(CTRLB, CTRLB_DETACH);
        write8(CTRLA, 0);
        while read8(SYNCBUSY) & CTRLA_ENABLE != 0 {}
        ENABLED.store(0, Ordering::Release);
    }

    async fn poll(&mut self) -> Event {
        poll_fn(|cx| {
            BUS_WAKER.register(cx.waker());
            OUT_WAKERS[0].register(cx.waker());
            match self.poll_bus() {
                Poll::Ready(event) => Poll::Ready(event),
                Poll::Pending => self.poll_setup().map(Event::Setup),
            }
        })
        .await
    }

    fn control_max_packet_size(&self) -> usize {
        usize::from(self.control)
    }

    async fn control_data_out(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        let mut total = 0;
        loop {
            arm_control_out(self.control);
            self.wait(TRCPT0, &OUT_WAKERS[0]).await?;
            compiler_fence(Ordering::SeqCst);
            let len = byte_count(0, 0);
            write8(ep(0, EPINTFLAG), TRCPT0 | TRFAIL0);
            let rest = &mut buf[total..];
            if len > rest.len() {
                return Err(EndpointError::BufferOverflow);
            }
            // SAFETY: bank 0's buffer, handed back with `len` bytes.
            unsafe { core::ptr::copy_nonoverlapping(buffer(0, 0), rest.as_mut_ptr(), len) };
            total += len;
            if len < usize::from(self.control) || total == buf.len() {
                return Ok(total);
            }
        }
    }

    async fn control_data_in(&mut self, data: &[u8], needs_zlp: bool) -> Result<(), EndpointDisabledError> {
        let packet = usize::from(self.control);
        if data.is_empty() {
            self.send(&[]).await?;
        }
        for chunk in data.chunks(packet) {
            self.send(chunk).await?;
        }
        if needs_zlp && !data.is_empty() && data.len().is_multiple_of(packet) {
            self.send(&[]).await?;
        }
        // The status stage: bank 0 empty, to take the host's zero-length
        // OUT.
        arm_control_out(self.control);
        Ok(())
    }

    async fn control_accept(&mut self) {
        // The status stage: a zero-length IN. A new SETUP ends it too.
        let _ = self.send(&[]).await;
        arm_control_out(self.control);
    }

    async fn control_reject(&mut self) {
        // A SETUP already in has ended the request, and the stall would
        // answer the new one.
        critical_section::with(|_| {
            if !setup_waiting() {
                write8(ep(0, EPSTATUSSET), STALLRQ0 | STALLRQ1);
                arm_out(0, self.control);
            }
        });
    }

    async fn control_accept_set_address(&mut self, addr: u8) {
        // The address takes effect after the status stage: the interrupt
        // handler writes it when the zero-length IN completes, or, if the
        // IN completed before the wait enabled that interrupt, this does.
        // One the host abandoned (a reset, a new SETUP) doesn't count.
        PENDING_ADDRESS.store((addr & 0x7F) | DADD_ADDEN, Ordering::Relaxed);
        let done = self.send(&[]).await.is_ok();
        let address = PENDING_ADDRESS.swap(0, Ordering::Relaxed);
        if done && address != 0 {
            write8(DADD, address);
        }
        arm_control_out(self.control);
    }

    #[expect(clippy::unused_async_trait_impl, reason = "the trait's signature; nothing to wait for")]
    async fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        let n = ep_addr.index();
        if n == 0 || n >= ENDPOINTS {
            return;
        }
        let (types, shift) = if ep_addr.is_in() { (&self.in_types, 4) } else { (&self.out_types, 0) };
        let Some((ep_type, max_packet_size)) = types[n] else { return };
        let config = read8(ep(n, EPCFG)) & !(0x7 << shift);
        if enabled {
            let code = if ep_type == EndpointType::Interrupt { EPTYPE_INTERRUPT } else { EPTYPE_BULK };
            if ep_addr.is_in() {
                // SAFETY: bank 1's descriptor, before the endpoint runs.
                unsafe { write_volatile(addr_of_mut!((*bank(n, 1)).addr), buffer(n, 1) as u32) };
                set_pcksize(n, 1, size_code(max_packet_size) << PCKSIZE_SIZE_SHIFT);
                write8(ep(n, EPSTATUSCLR), BK1RDY | DTGLIN | STALLRQ1);
            } else {
                // SAFETY: bank 0's descriptor, before the endpoint runs.
                unsafe { write_volatile(addr_of_mut!((*bank(n, 0)).addr), buffer(n, 0) as u32) };
                set_pcksize(n, 0, size_code(max_packet_size) << PCKSIZE_SIZE_SHIFT);
                // Bank 0 full until a read arms it: OUT packets wait (NAK).
                write8(ep(n, EPSTATUSSET), BK0RDY);
                write8(ep(n, EPSTATUSCLR), DTGLOUT | STALLRQ0);
            }
            write8(ep(n, EPCFG), config | (code << shift));
            ENABLED.fetch_or(enabled_bit(ep_addr), Ordering::AcqRel);
        } else {
            write8(ep(n, EPCFG), config);
            ENABLED.fetch_and(!enabled_bit(ep_addr), Ordering::AcqRel);
        }
        if ep_addr.is_in() { IN_WAKERS[n].wake() } else { OUT_WAKERS[n].wake() }
    }

    fn endpoint_is_enabled(&mut self, ep_addr: EndpointAddress) -> bool {
        ep_addr.index() == 0 || is_enabled(ep_addr)
    }

    async fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        set_stalled(ep_addr, stalled);
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        is_stalled(ep_addr)
    }

    #[expect(clippy::unused_async_trait_impl, reason = "the trait's signature; nothing to wait for")]
    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        write16(CTRLB, read16(CTRLB) | CTRLB_UPRSM);
        Ok(())
    }
}

fn set_stalled(ep_addr: EndpointAddress, stalled: bool) {
    let n = ep_addr.index();
    let (stall, toggle) = if ep_addr.is_in() { (STALLRQ1, DTGLIN) } else { (STALLRQ0, DTGLOUT) };
    if stalled {
        write8(ep(n, EPSTATUSSET), stall);
    } else {
        write8(ep(n, EPSTATUSCLR), stall | toggle);
    }
    if ep_addr.is_in() { IN_WAKERS[n].wake() } else { OUT_WAKERS[n].wake() }
}

fn is_stalled(ep_addr: EndpointAddress) -> bool {
    let stall = if ep_addr.is_in() { STALLRQ1 } else { STALLRQ0 };
    read8(ep(ep_addr.index(), EPSTATUS)) & stall != 0
}

fn write_volatile_u32(address: usize, value: u32) {
    // SAFETY: a USB register.
    unsafe { write_volatile(address as *mut u32, value) }
}

/// The pads' calibration from CAL OTP, or Microchip's usual values (TRANSP
/// 29, TRANSN 5, TRIM 3) if that is blank.
fn padcal() -> u16 {
    // SAFETY: a word of the calibration area, always readable.
    let otp = unsafe { read_volatile(PADCAL_OTP as *const u32) };
    #[expect(clippy::cast_possible_truncation, reason = "PADCAL's sixteen bits")]
    let value = otp as u16 & PADCAL_MASK;
    if value == PADCAL_MASK || value == 0 { 0x1D | (5 << 6) | (3 << 12) } else { value }
}

/// Endpoint direction markers.
pub struct In;
/// Endpoint direction markers.
pub struct Out;

/// An IN or OUT endpoint other than 0.
pub struct Endpoint<'d, D> {
    info: EndpointInfo,
    _marker: PhantomData<(&'d (), D)>,
}

impl<D> Endpoint<'_, D> {
    const fn new(info: EndpointInfo) -> Self {
        Self { info, _marker: PhantomData }
    }

    fn waker(&self) -> &'static AtomicWaker {
        let n = self.info.addr.index();
        if self.info.addr.is_in() { &IN_WAKERS[n] } else { &OUT_WAKERS[n] }
    }
}

impl<D> embassy_usb_driver::Endpoint for Endpoint<'_, D> {
    fn info(&self) -> EndpointInfo {
        self.info
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

    fn is_enabled(&self) -> bool {
        is_enabled(self.info.addr)
    }

    async fn set_stalled(&mut self, stalled: bool) {
        set_stalled(self.info.addr, stalled);
    }

    fn is_stalled(&self) -> bool {
        is_stalled(self.info.addr)
    }
}

impl Endpoint<'_, Out> {
    /// Takes one packet into `buf`.
    async fn read_packet(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        let addr = self.info.addr;
        let n = addr.index();
        if !is_enabled(addr) {
            return Err(EndpointError::Disabled);
        }
        arm_out(n, self.info.max_packet_size);
        if wait_flags(n, TRCPT0, self.waker(), || !is_enabled(addr)).await.is_none() {
            return Err(EndpointError::Disabled);
        }
        compiler_fence(Ordering::SeqCst);
        let len = byte_count(n, 0);
        write8(ep(n, EPINTFLAG), TRCPT0 | TRFAIL0);
        if len > buf.len() {
            return Err(EndpointError::BufferOverflow);
        }
        // SAFETY: the bank's buffer, which the controller has handed back
        // (BK0RDY is set again) and filled with `len` bytes.
        unsafe { core::ptr::copy_nonoverlapping(buffer(n, 0), buf.as_mut_ptr(), len) };
        Ok(len)
    }
}

impl embassy_usb_driver::EndpointOut for Endpoint<'_, Out> {
    async fn read_transfer(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        let packet = usize::from(self.info.max_packet_size);
        let mut total = 0;
        loop {
            let len = self.read_packet(&mut buf[total..]).await?;
            total += len;
            if len < packet || total == buf.len() {
                return Ok(total);
            }
        }
    }
}

impl Endpoint<'_, In> {
    /// Sends one packet, `data`, and waits until the host has it.
    async fn write_packet(&mut self, data: &[u8]) -> Result<(), EndpointDisabledError> {
        let addr = self.info.addr;
        let n = addr.index();
        if !is_enabled(addr) {
            return Err(EndpointDisabledError);
        }
        send_in(n, self.info.max_packet_size, data);
        if wait_flags(n, TRCPT1, self.waker(), || !is_enabled(addr)).await.is_none() {
            return Err(EndpointDisabledError);
        }
        write8(ep(n, EPINTFLAG), TRCPT1 | TRFAIL1);
        Ok(())
    }
}

impl embassy_usb_driver::EndpointIn for Endpoint<'_, In> {
    async fn write_transfer(&mut self, buf: &[u8], needs_zlp: bool) -> Result<(), EndpointDisabledError> {
        let packet = usize::from(self.info.max_packet_size);
        if buf.is_empty() {
            return self.write_packet(&[]).await;
        }
        for chunk in buf.chunks(packet) {
            self.write_packet(chunk).await?;
        }
        if needs_zlp && buf.len().is_multiple_of(packet) {
            self.write_packet(&[]).await?;
        }
        Ok(())
    }
}

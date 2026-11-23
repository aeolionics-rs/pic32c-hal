// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Waiting on pin levels and edges through the External Interrupt
//! Controller (EIC; DS60001795 chapter 30 on the PIC32CK, the same IP on the
//! PIC32CM SG00 and the PL10): [`ExtiInput`].
//!
//! The EIC has 16 lines, `EXTINT0` to `EXTINT15`, each a singleton; each pin
//! that has a line has exactly one ([`ExtiPin::Channel`], from the chip's
//! pin multiplexing table: mostly the pin's number modulo 16, but not
//! always), and [`ExtiInput::new`] takes the pin and its line. [`Nmi`]
//! drives the NMI, on its pin ([`NmiPin`]): PD02 on the PIC32CK, PD00 on the
//! SG00, any of PA01, PA07 and PA30 on the PL10, PA08 on the GV00.
//!
//! Edges are detected asynchronously and levels without the filter, so the
//! EIC needs no clock. Its sense settings can only change while it's
//! disabled, so each wait disables it for a moment to set its line, which
//! can drop another line's edge in that moment.
//!
//! With feature `rt` the HAL defines the EIC's vectors: 16 (`EIC_EXTINT_0`
//! to `_15` on the PIC32CK, `EIC_EXTINT0` to `15` on the SG00), or on the
//! PL10 its one, `EIC`, for all the lines. On the PIC32CK SG the EIC, like
//! PORT, has a Secure view, which this module uses (`EIC_SEC`,
//! `0x4401_B000`); the GC has only `0x4401_A000`, the SG00 only
//! `0x4402_0000` and the PL10 `0x4000_2800`.
//!
//! The GV00's EIC (`eic_v0`, the SAM D20's, DS60001881 chapter 16, at
//! `0x4000_1800`) has the same lines, sense settings and NMI at other
//! offsets, but no asynchronous edge detection: its edges are sampled by
//! its generic clock, `GCLK_EIC`, which the first [`ExtiInput`] or [`Nmi`]
//! feeds from generator 0 (the CPU's). It has one vector, `EIC`, as the
//! PL10 has, and no PINSTATE.

use core::future::poll_fn;
use core::ptr::{read_volatile, write_volatile};
use core::task::Poll;
use portable_atomic::{AtomicU16, AtomicU32, Ordering};

use embassy_sync::waitqueue::AtomicWaker;

use crate::gpio::{Flex, Function, Level, Pin, Pull};
use crate::interrupt::{Interrupt, InterruptExt};
use crate::{Peri, PeripheralType};

const CTRLA: usize = 0x00;
#[cfg(not(eic_v0))]
const NMICTRL: usize = 0x01;
#[cfg(not(eic_v0))]
const NMIFLAG: usize = 0x02;
#[cfg(not(eic_v0))]
const SYNCBUSY: usize = 0x04;
#[cfg(not(eic_v0))]
const INTENCLR: usize = 0x0C;
#[cfg(not(eic_v0))]
const INTENSET: usize = 0x10;
#[cfg(not(eic_v0))]
const INTFLAG: usize = 0x14;
#[cfg(not(eic_v0))]
const ASYNCH: usize = 0x18;
#[cfg(not(eic_v0))]
const CONFIG: usize = 0x1C;
/// None on the PL10 or the GV00.
#[cfg(not(any(pic32cm_pl, eic_v0)))]
const PINSTATE: usize = 0x38;
/// `eic_v0`: STATUS, whose SYNCBUSY (bit 7) covers every write.
#[cfg(eic_v0)]
const STATUS: usize = 0x01;
#[cfg(eic_v0)]
const NMICTRL: usize = 0x02;
#[cfg(eic_v0)]
const NMIFLAG: usize = 0x03;
#[cfg(eic_v0)]
const INTENCLR: usize = 0x08;
#[cfg(eic_v0)]
const INTENSET: usize = 0x0C;
#[cfg(eic_v0)]
const INTFLAG: usize = 0x10;
#[cfg(eic_v0)]
const CONFIG: usize = 0x18;
/// `GCLK_EIC`'s channel (`eic_v0`), and the generator it's fed from.
#[cfg(eic_v0)]
const GCLK_EIC: usize = 0x03;
#[cfg(eic_v0)]
const EIC_GENERATOR: usize = 0;

const ENABLE: u8 = 1 << 1;
/// CKSEL: `CLK_ULP32K` (`CLK_OSC32K` on the PL10), for whatever would need
/// a clock (nothing this module sets). `eic_v0` has none: its clock is
/// `GCLK_EIC`.
#[cfg(not(eic_v0))]
const CKSEL_ULP32K: u8 = 1 << 4;
#[cfg(eic_v0)]
const CKSEL_ULP32K: u8 = 0;
#[cfg(not(eic_v0))]
const SYNC_ENABLE: u32 = 1 << 1;

const LINES: usize = 16;

/// What a line senses (CONFIG.SENSEx).
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum Sense {
    None = 0,
    Rise = 1,
    Fall = 2,
    Both = 3,
    High = 4,
    Low = 5,
}

static WAKERS: [AtomicWaker; LINES] = [const { AtomicWaker::new() }; LINES];
/// Lines whose condition has come since their wait set them up.
static FIRED: AtomicU16 = AtomicU16::new(0);

fn read32(offset: usize) -> u32 {
    // SAFETY: an EIC register, read whole.
    unsafe { read_volatile((crate::chip::EIC + offset) as *const u32) }
}

fn write32(offset: usize, value: u32) {
    // SAFETY: an EIC register; this module is its only writer.
    unsafe { write_volatile((crate::chip::EIC + offset) as *mut u32, value) }
}

fn write8(offset: usize, value: u8) {
    // SAFETY: as for `write32`.
    unsafe { write_volatile((crate::chip::EIC + offset) as *mut u8, value) }
}

/// Waits for the EIC's enable to synchronise.
fn sync_enable() {
    #[cfg(not(eic_v0))]
    while read32(SYNCBUSY) & SYNC_ENABLE != 0 {}
    // SAFETY: STATUS, a byte register; SYNCBUSY is bit 7.
    #[cfg(eic_v0)]
    while unsafe { read_volatile((crate::chip::EIC + STATUS) as *const u8) } & 0x80 != 0 {}
}

/// Feeds `GCLK_EIC` (`eic_v0`'s edge detection needs it); nothing
/// elsewhere.
fn clock_eic() {
    #[cfg(eic_v0)]
    crate::clocks::enable_channel(GCLK_EIC, EIC_GENERATOR);
}

/// Sets `line`'s sense (asynchronous for edges, where the EIC can), with
/// the EIC disabled for the moment, and clears its flag.
fn configure(line: usize, sense: Sense) {
    critical_section::with(|_| {
        write8(CTRLA, CKSEL_ULP32K);
        sync_enable();
        let config = CONFIG + 4 * (line / 8);
        let shift = 4 * (line % 8);
        write32(config, (read32(config) & !(0xF << shift)) | ((sense as u32) << shift));
        #[cfg(not(eic_v0))]
        {
            let edge = matches!(sense, Sense::Rise | Sense::Fall | Sense::Both);
            let asynch = read32(ASYNCH) & !(1 << line);
            write32(ASYNCH, asynch | (u32::from(edge) << line));
        }
        write32(INTFLAG, 1 << line);
        FIRED.fetch_and(!(1 << line), Ordering::AcqRel);
        write8(CTRLA, CKSEL_ULP32K | ENABLE);
        sync_enable();
    });
}

/// One line's interrupt: masks it, notes it fired, wakes its waiter.
#[cfg_attr(any(pic32cm_pl, pic32cm_gv), allow(dead_code, reason = "the PL10's and GV00's one vector takes every line at once"))]
fn on_interrupt(line: usize) {
    let bit = 1 << line;
    if read32(INTFLAG) & bit != 0 {
        write32(INTENCLR, bit);
        write32(INTFLAG, bit);
        #[expect(clippy::cast_possible_truncation, reason = "16 lines")]
        FIRED.fetch_or(bit as u16, Ordering::AcqRel);
        WAKERS[line].wake();
    }
}

#[cfg(not(any(pic32cm_pl, pic32cm_gv)))]
macro_rules! vectors {
    ($($vector:ident = $line:literal),*) => {
        $(
            #[cfg(feature = "rt")]
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn $vector() {
                on_interrupt($line);
            }
        )*

        const INTERRUPTS: [Interrupt; LINES] = [$(Interrupt::$vector),*];
    };
}

#[cfg(pic32ck)]
vectors!(
    EIC_EXTINT_0 = 0,
    EIC_EXTINT_1 = 1,
    EIC_EXTINT_2 = 2,
    EIC_EXTINT_3 = 3,
    EIC_EXTINT_4 = 4,
    EIC_EXTINT_5 = 5,
    EIC_EXTINT_6 = 6,
    EIC_EXTINT_7 = 7,
    EIC_EXTINT_8 = 8,
    EIC_EXTINT_9 = 9,
    EIC_EXTINT_10 = 10,
    EIC_EXTINT_11 = 11,
    EIC_EXTINT_12 = 12,
    EIC_EXTINT_13 = 13,
    EIC_EXTINT_14 = 14,
    EIC_EXTINT_15 = 15
);
/// The PL10's (and the GV00's) one vector, for every line: each enabled
/// line that fired is masked, noted and woken.
#[cfg(all(any(pic32cm_pl, pic32cm_gv), feature = "rt"))]
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
unsafe extern "C" fn EIC() {
    let fired = read32(INTFLAG) & read32(INTENSET) & 0xFFFF;
    if fired != 0 {
        write32(INTENCLR, fired);
        write32(INTFLAG, fired);
        // 16 lines: the mask above keeps it to them.
        FIRED.fetch_or(fired as u16, Ordering::AcqRel);
        for (line, waker) in WAKERS.iter().enumerate() {
            if fired & (1 << line) != 0 {
                waker.wake();
            }
        }
    }
}

/// Each line's vector: the one on the PL10 and the GV00.
#[cfg(any(pic32cm_pl, pic32cm_gv))]
const INTERRUPTS: [Interrupt; LINES] = [Interrupt::EIC; LINES];

#[cfg(pic32cm_sg)]
vectors!(
    EIC_EXTINT0 = 0,
    EIC_EXTINT1 = 1,
    EIC_EXTINT2 = 2,
    EIC_EXTINT3 = 3,
    EIC_EXTINT4 = 4,
    EIC_EXTINT5 = 5,
    EIC_EXTINT6 = 6,
    EIC_EXTINT7 = 7,
    EIC_EXTINT8 = 8,
    EIC_EXTINT9 = 9,
    EIC_EXTINT10 = 10,
    EIC_EXTINT11 = 11,
    EIC_EXTINT12 = 12,
    EIC_EXTINT13 = 13,
    EIC_EXTINT14 = 14,
    EIC_EXTINT15 = 15
);

/// An EIC line.
pub(crate) trait SealedChannel {
    fn line(&self) -> usize;
}

/// An EIC line, `EXTINT0` to `EXTINT15`.
#[allow(private_bounds, reason = "sealed")]
pub trait Channel: PeripheralType + SealedChannel + 'static {}

/// A pin with an EIC line.
pub trait ExtiPin: Pin {
    /// Its line.
    type Channel: Channel;
}

macro_rules! impl_channels {
    ($($channel:ident = $line:literal),*) => {
        $(
            impl $crate::exti::SealedChannel for $crate::peripherals::$channel {
                fn line(&self) -> usize {
                    $line
                }
            }
            impl $crate::exti::Channel for $crate::peripherals::$channel {}
        )*
    };
}

macro_rules! impl_exti_pins {
    ($($pin:ident = $channel:ident),* $(,)?) => {
        $(
            impl $crate::exti::ExtiPin for $crate::peripherals::$pin {
                type Channel = $crate::peripherals::$channel;
            }
        )*
    };
}

pub(crate) use {impl_channels, impl_exti_pins};

/// A pin whose levels and edges can be waited for.
pub struct ExtiInput<'d> {
    pin: Flex<'d>,
    line: usize,
}

impl<'d> ExtiInput<'d> {
    /// `pin`, on its EIC line `channel`, with `pull`, which holds while the
    /// EIC has the pin (the EIC only reads it) on the PIC32CK and the SG00.
    /// On the PL10 it doesn't: a peripheral function takes the pin from
    /// PORT, pull and all (seen on the bench), so the pin needs an external
    /// pull, or something driving it.
    #[allow(clippy::needless_pass_by_value, reason = "the line is the input's for 'd")]
    pub fn new<P: ExtiPin>(pin: Peri<'d, P>, channel: Peri<'d, P::Channel>, pull: Pull) -> Self {
        let line = channel.line();
        clock_eic();
        let mut pin = Flex::new(pin);
        // The pull's direction and enable, and the input buffer, so PORT's
        // IN still reads the pin; then the EIC's function.
        pin.set_as_input(pull);
        pin.set_as_function_keeping_input(Function::A);
        configure(line, Sense::None);
        let irq = INTERRUPTS[line];
        irq.unpend();
        // SAFETY: the vectors above only run `on_interrupt`.
        unsafe { irq.enable() };
        Self { pin, line }
    }

    /// The pin's level, as PORT reads it.
    #[must_use]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    /// Not [`Self::is_high`].
    #[must_use]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    /// The pin's level.
    #[must_use]
    pub fn get_level(&self) -> Level {
        self.is_high().into()
    }

    /// The EIC's PINSTATE for the line. It follows the pin only while the
    /// line samples with a clock (synchronous edges, the filter or the
    /// debouncer), none of which this module uses, so it reads low here.
    /// The PL10 and the GV00 have no PINSTATE.
    #[cfg(not(any(pic32cm_pl, eic_v0)))]
    #[must_use]
    pub fn eic_level(&self) -> Level {
        (read32(PINSTATE) & (1 << self.line) != 0).into()
    }

    /// Waits until the pin is high (at once if it is).
    pub async fn wait_for_high(&mut self) {
        self.wait(Sense::High).await;
    }

    /// Waits until the pin is low (at once if it is).
    pub async fn wait_for_low(&mut self) {
        self.wait(Sense::Low).await;
    }

    /// Waits for a rising edge.
    pub async fn wait_for_rising_edge(&mut self) {
        self.wait(Sense::Rise).await;
    }

    /// Waits for a falling edge.
    pub async fn wait_for_falling_edge(&mut self) {
        self.wait(Sense::Fall).await;
    }

    /// Waits for either edge.
    pub async fn wait_for_any_edge(&mut self) {
        self.wait(Sense::Both).await;
    }

    async fn wait(&mut self, sense: Sense) {
        let line = self.line;
        let bit = 1u16 << line;
        configure(line, sense);
        write32(INTENSET, 1 << line);
        poll_fn(|cx| {
            WAKERS[line].register(cx.waker());
            if FIRED.load(Ordering::Acquire) & bit != 0 { Poll::Ready(()) } else { Poll::Pending }
        })
        .await;
        configure(line, Sense::None);
    }
}

impl Drop for ExtiInput<'_> {
    fn drop(&mut self) {
        write32(INTENCLR, 1 << self.line);
        configure(self.line, Sense::None);
        // On the PL10 and the GV00 the vector is every line's: masking the
        // line is enough.
        #[cfg(not(any(pic32cm_pl, pic32cm_gv)))]
        INTERRUPTS[self.line].disable();
    }
}

mod embedded_hal_traits {
    use core::convert::Infallible;

    use super::ExtiInput;

    impl embedded_hal::digital::ErrorType for ExtiInput<'_> {
        type Error = Infallible;
    }

    impl embedded_hal::digital::InputPin for ExtiInput<'_> {
        fn is_high(&mut self) -> Result<bool, Infallible> {
            Ok(ExtiInput::is_high(self))
        }

        fn is_low(&mut self) -> Result<bool, Infallible> {
            Ok(ExtiInput::is_low(self))
        }
    }

    impl embedded_hal_async::digital::Wait for ExtiInput<'_> {
        async fn wait_for_high(&mut self) -> Result<(), Infallible> {
            ExtiInput::wait_for_high(self).await;
            Ok(())
        }

        async fn wait_for_low(&mut self) -> Result<(), Infallible> {
            ExtiInput::wait_for_low(self).await;
            Ok(())
        }

        async fn wait_for_rising_edge(&mut self) -> Result<(), Infallible> {
            ExtiInput::wait_for_rising_edge(self).await;
            Ok(())
        }

        async fn wait_for_falling_edge(&mut self) -> Result<(), Infallible> {
            ExtiInput::wait_for_falling_edge(self).await;
            Ok(())
        }

        async fn wait_for_any_edge(&mut self) -> Result<(), Infallible> {
            ExtiInput::wait_for_any_edge(self).await;
            Ok(())
        }
    }
}

/// A pin the NMI can be on.
pub trait NmiPin: Pin {}

macro_rules! impl_nmi_pins {
    ($($pin:ident),* $(,)?) => {
        $(
            impl $crate::exti::NmiPin for $crate::peripherals::$pin {}
        )*
    };
}

pub(crate) use impl_nmi_pins;

/// What arms the NMI (NMICTRL.NMISENSE).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NmiSense {
    Rise = 1,
    Fall = 2,
    Both = 3,
    High = 4,
    Low = 5,
}

/// NMIASYNCH: edges detected without the EIC clock (`eic_v0` has none: its
/// NMI's edges are sampled by `GCLK_EIC`).
#[cfg(not(eic_v0))]
const NMI_ASYNCH: u8 = 1 << 4;
#[cfg(eic_v0)]
const NMI_ASYNCH: u8 = 0;
const NMI_FLAG: u16 = 1 << 0;

/// NMIs taken (by [`on_nmi`]).
static NMIS: AtomicU32 = AtomicU32::new(0);

/// NMIFLAG: 16 bits, or 8 on `eic_v0`.
fn read_nmiflag() -> u16 {
    // SAFETY: an EIC register, read whole.
    #[cfg(not(eic_v0))]
    return unsafe { read_volatile((crate::chip::EIC + NMIFLAG) as *const u16) };
    // SAFETY: as above.
    #[cfg(eic_v0)]
    return u16::from(unsafe { read_volatile((crate::chip::EIC + NMIFLAG) as *const u8) });
}

#[cfg_attr(eic_v0, expect(clippy::cast_possible_truncation, reason = "the flag is in the low byte"))]
fn clear_nmiflag() {
    // SAFETY: as for `write32`.
    #[cfg(not(eic_v0))]
    unsafe {
        write_volatile((crate::chip::EIC + NMIFLAG) as *mut u16, NMI_FLAG);
    }
    #[cfg(eic_v0)]
    write8(NMIFLAG, NMI_FLAG as u8);
}

/// The NMI handler's part: call it from the application's
/// `NonMaskableInt` exception handler (cortex-m-rt's
/// `#[exception] unsafe fn NonMaskableInt()`, unsafe because an NMI can
/// land inside a critical section). It disarms the NMI (each arming
/// fires once: a level would otherwise fire for as long as it lasts),
/// clears the flag and counts. It touches only the EIC's NMI registers and
/// an atomic, so it's safe however the NMI interrupts the program; critical
/// sections don't keep an NMI out, so nothing here takes one.
pub fn on_nmi() {
    if read_nmiflag() & NMI_FLAG != 0 {
        write8(NMICTRL, 0);
        clear_nmiflag();
        NMIS.fetch_add(1, Ordering::AcqRel);
    }
}

/// The non-maskable interrupt on its pin (PD02 on the PIC32CK, SW1 on the
/// Curiosity Ultra boards; PD00 on the SG00; PA01, PA07 or PA30 on the
/// PL10; PA08 on the GV00), armed for one firing at a time.
pub struct Nmi<'d> {
    _pin: Flex<'d>,
}

impl<'d> Nmi<'d> {
    /// The NMI on `pin`, with `pull`, disarmed.
    #[must_use]
    pub fn new(pin: Peri<'d, impl NmiPin>, pull: Pull) -> Self {
        clock_eic();
        let mut pin = Flex::new(pin);
        pin.set_as_input(pull);
        pin.set_as_function_keeping_input(Function::A);
        write8(NMICTRL, 0);
        clear_nmiflag();
        Self { _pin: pin }
    }

    /// Arms it for one NMI when `sense` is met (at once, for a level that
    /// already holds).
    pub fn arm(&mut self, sense: NmiSense) {
        write8(NMICTRL, 0);
        clear_nmiflag();
        let asynch = if matches!(sense, NmiSense::Rise | NmiSense::Fall | NmiSense::Both) { NMI_ASYNCH } else { 0 };
        write8(NMICTRL, asynch | sense as u8);
    }

    /// Disarms it.
    pub fn disarm(&mut self) {
        write8(NMICTRL, 0);
    }

    /// Whether it's armed (not yet fired since [`Self::arm`]).
    #[must_use]
    pub fn is_armed(&self) -> bool {
        // SAFETY: NMICTRL, read.
        let nmictrl = unsafe { read_volatile((crate::chip::EIC + NMICTRL) as *const u8) };
        nmictrl & 0x7 != 0
    }

    /// NMIs taken since power-on.
    #[must_use]
    pub fn count(&self) -> u32 {
        NMIS.load(Ordering::Acquire)
    }
}

impl Drop for Nmi<'_> {
    fn drop(&mut self) {
        write8(NMICTRL, 0);
    }
}

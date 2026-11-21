// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Waiting on pin levels and edges through the External Interrupt
//! Controller (EIC, DS60001795 chapter 30): [`ExtiInput`].
//!
//! The EIC has 16 lines, `EXTINT0` to `EXTINT15`, each a singleton; each pin
//! that has a line has exactly one ([`ExtiPin::Channel`], from the chip's
//! pin multiplexing table: mostly the pin's number modulo 16, but not
//! always), and [`ExtiInput::new`] takes the pin and its line. PD02 has only
//! the NMI, which [`Nmi`] drives.
//!
//! Edges are detected asynchronously and levels without the filter, so the
//! EIC needs no clock. Its sense settings can only change while it's
//! disabled, so each wait disables it for a moment to set its line, which
//! can drop another line's edge in that moment.
//!
//! With feature `rt` the HAL defines the 16 vectors (`EIC_EXTINT_0` to
//! `_15`). On the SG parts the EIC, like PORT, has a Secure view, which this
//! module uses (`EIC_SEC`, `0x4401_B000`); the GC has only `0x4401_A000`.

use core::future::poll_fn;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicU16, AtomicU32, Ordering};
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;

use crate::gpio::{Flex, Function, Level, Pin, Pull};
use crate::interrupt::{Interrupt, InterruptExt};
use crate::{Peri, PeripheralType, peripherals};

const CTRLA: usize = 0x00;
const NMICTRL: usize = 0x01;
const NMIFLAG: usize = 0x02;
const SYNCBUSY: usize = 0x04;
const INTENCLR: usize = 0x0C;
const INTENSET: usize = 0x10;
const INTFLAG: usize = 0x14;
const ASYNCH: usize = 0x18;
const CONFIG: usize = 0x1C;
const PINSTATE: usize = 0x38;

const ENABLE: u8 = 1 << 1;
/// CKSEL: `CLK_ULP32K`, for whatever would need a clock (nothing this module
/// sets).
const CKSEL_ULP32K: u8 = 1 << 4;
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

/// Sets `line`'s sense (asynchronous for edges), with the EIC disabled for
/// the moment, and clears its flag.
fn configure(line: usize, sense: Sense) {
    critical_section::with(|_| {
        write8(CTRLA, CKSEL_ULP32K);
        while read32(SYNCBUSY) & SYNC_ENABLE != 0 {}
        let config = CONFIG + 4 * (line / 8);
        let shift = 4 * (line % 8);
        write32(config, (read32(config) & !(0xF << shift)) | ((sense as u32) << shift));
        let edge = matches!(sense, Sense::Rise | Sense::Fall | Sense::Both);
        let asynch = read32(ASYNCH) & !(1 << line);
        write32(ASYNCH, asynch | (u32::from(edge) << line));
        write32(INTFLAG, 1 << line);
        FIRED.fetch_and(!(1 << line), Ordering::AcqRel);
        write8(CTRLA, CKSEL_ULP32K | ENABLE);
        while read32(SYNCBUSY) & SYNC_ENABLE != 0 {}
    });
}

/// One line's interrupt: masks it, notes it fired, wakes its waiter.
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

vectors!(
    EIC_EXTINT_0 = 0, EIC_EXTINT_1 = 1, EIC_EXTINT_2 = 2, EIC_EXTINT_3 = 3, EIC_EXTINT_4 = 4, EIC_EXTINT_5 = 5, EIC_EXTINT_6 = 6, EIC_EXTINT_7 = 7,
    EIC_EXTINT_8 = 8, EIC_EXTINT_9 = 9, EIC_EXTINT_10 = 10, EIC_EXTINT_11 = 11, EIC_EXTINT_12 = 12, EIC_EXTINT_13 = 13, EIC_EXTINT_14 = 14, EIC_EXTINT_15 = 15
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
    /// EIC has the pin (the EIC only reads it).
    #[allow(clippy::needless_pass_by_value, reason = "the line is the input's for 'd")]
    pub fn new<P: ExtiPin>(pin: Peri<'d, P>, channel: Peri<'d, P::Channel>, pull: Pull) -> Self {
        let line = channel.line();
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

/// NMIASYNCH: edges detected without the EIC clock.
const NMI_ASYNCH: u8 = 1 << 4;
const NMI_FLAG: u16 = 1 << 0;

/// NMIs taken (by [`on_nmi`]).
static NMIS: AtomicU32 = AtomicU32::new(0);

fn read16(offset: usize) -> u16 {
    // SAFETY: an EIC register, read whole.
    unsafe { read_volatile((crate::chip::EIC + offset) as *const u16) }
}

fn write16(offset: usize, value: u16) {
    // SAFETY: as for `write32`.
    unsafe { write_volatile((crate::chip::EIC + offset) as *mut u16, value) }
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
    if read16(NMIFLAG) & NMI_FLAG != 0 {
        write8(NMICTRL, 0);
        write16(NMIFLAG, NMI_FLAG);
        NMIS.fetch_add(1, Ordering::AcqRel);
    }
}

/// The non-maskable interrupt on PD02 (SW1 on the Curiosity Ultra boards),
/// armed for one firing at a time.
pub struct Nmi<'d> {
    _pin: Flex<'d>,
}

impl<'d> Nmi<'d> {
    /// The NMI on `pin`, with `pull`, disarmed.
    #[must_use]
    pub fn new(pin: Peri<'d, peripherals::PD02>, pull: Pull) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_as_input(pull);
        pin.set_as_function_keeping_input(Function::A);
        write8(NMICTRL, 0);
        write16(NMIFLAG, NMI_FLAG);
        Self { _pin: pin }
    }

    /// Arms it for one NMI when `sense` is met (at once, for a level that
    /// already holds).
    pub fn arm(&mut self, sense: NmiSense) {
        write8(NMICTRL, 0);
        write16(NMIFLAG, NMI_FLAG);
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


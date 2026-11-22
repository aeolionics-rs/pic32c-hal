// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The PL10's ADC (ADC0, IP 06272, at `0x4200_1C00`; DS40002667 chapter
//! 33): a 12-bit SAR core with single-ended inputs AIN2-7, AIN16-29, AIN31
//! and AIN40-43 on pins, and inside the chip VDDIO/10, VDDCORE/4, the
//! temperature sensor and ground. The same API as the PIC32CK's and the
//! SG00's ADC driver, over another IP: [`Adc::blocking_read`] and, on its
//! interrupt (bind `ADC0` to [`InterruptHandler`]), [`Adc::read`], each
//! taking a pin (`&mut p.PA04`), [`AnyAdcChannel`], [`Vddio`] or
//! [`Temperature`].
//!
//! Each read is a single conversion started by software (COMMAND.START
//! IMMEDIATE) after the input and sample length are set. Results are
//! unsigned on a 12-bit scale, `0..4096`, at every [`Resolution`] (an
//! 8- or 10-bit result is shifted up, its low bits zero), full scale being
//! the [`Reference`]: VDD, the external VREFA pin (PA23), or one of four
//! internal references (each only below VDD - 0.5 V, and within 4 %).
//!
//! The ADC is clocked from its bus clock, the CPU's, divided by 4
//! (`CLK_ADC`: 6 MHz from the boards' 24 MHz, under the 12 MHz its 83 ns
//! minimum period allows). There's no bandgap input: [`Vddio`] read
//! against an internal reference gives VDD instead ([`Vddio::millivolts`]).
//! [`Calibration`] holds the factory's temperature-sensor gain and offset
//! (SYSCTRL.TEMPSENSE), from which [`Calibration::celsius`] works.

use core::future::poll_fn;
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;

use crate::gpio::Function;
use crate::interrupt::InterruptExt;
use crate::interrupt::typelevel::{self, Binding, Handler};
use crate::mode::{Async, Blocking};
use crate::{Peri, peripherals};

const ADC: usize = 0x4200_1C00;
const CTRLA: usize = ADC;
const CTRLB: usize = ADC + 0x04;
const CTRLC: usize = ADC + 0x08;
const CTRLD: usize = ADC + 0x0C;
const CTRLE: usize = ADC + 0x10;
const INPUTCTRL: usize = ADC + 0x28;
const INTENCLR: usize = ADC + 0x30;
const INTENSET: usize = ADC + 0x34;
const INTFLAG: usize = ADC + 0x38;
const COMMAND: usize = ADC + 0x48;
const RESULT: usize = ADC + 0x50;

const SWRST: u32 = 1 << 0;
const ENABLE: u32 = 1 << 1;
/// CTRLB.PRESCALER DIV4: `CLK_ADC` a quarter of the bus clock.
const PRESCALER_DIV4: u32 = 0x02;
const PRESCALER_DIVISOR: u32 = 4;
const TIMEBASE_SHIFT: u32 = 8;
const RESSEL_SHIFT: u32 = 8;
const MUXPOS_SHIFT: u32 = 8;
/// COMMAND: a single conversion (MODE SINGLE), started now (START
/// IMMEDIATE).
const SINGLE_NOW: u32 = (0x1 << 4) | 0x1;
const RESRDY: u32 = 1 << 0;

/// MCLK.APBCMASK: ADC0's bus clock (bit 7) and SYSCTRL's (bit 11, for the
/// calibration).
const APBC_ADC0: u32 = 1 << 7;
const APBC_SYSCTRL: u32 = 1 << 11;
/// SYSCTRL.TEMPSENSE: the temperature sensor's gain (low half) and offset.
const TEMPSENSE: usize = 0x4200_2C88;

/// MUXPOS for the inputs inside the chip.
const MUX_VDDIO_DIV10: u8 = 0x64;
const MUX_TEMPSENSE: u8 = 0x62;

static WAKER: AtomicWaker = AtomicWaker::new();

/// The result's width (CTRLD.RESSEL). Results stay on a 12-bit scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resolution {
    Bits8,
    Bits10,
    #[default]
    Bits12,
}

impl Resolution {
    /// The number of bits.
    #[must_use]
    pub const fn bits(self) -> u8 {
        match self {
            Self::Bits8 => 8,
            Self::Bits10 => 10,
            Self::Bits12 => 12,
        }
    }

    const fn ressel(self) -> u32 {
        match self {
            Self::Bits12 => 0x0,
            Self::Bits10 => 0x2,
            Self::Bits8 => 0x3,
        }
    }
}

/// Full scale's voltage (CTRLC.REFSEL).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reference {
    /// VDD.
    #[default]
    Avdd,
    /// VREFA on PA23. The driver doesn't yet put the pin on its analog
    /// function (B): do that first, with
    /// [`gpio::set_as_function`](crate::gpio::set_as_function).
    External,
    /// The internal 1.024 V reference.
    Internal1V024,
    /// The internal 2.048 V reference (the temperature sensor's).
    Internal2V048,
    /// The internal 2.5 V reference.
    Internal2V500,
    /// The internal 4.096 V reference (VDD 4.6 V or more).
    Internal4V096,
}

impl Reference {
    /// An internal reference's nominal voltage in mV.
    #[must_use]
    pub const fn millivolts(self) -> Option<u32> {
        match self {
            Self::Avdd | Self::External => None,
            Self::Internal1V024 => Some(1024),
            Self::Internal2V048 => Some(2048),
            Self::Internal2V500 => Some(2500),
            Self::Internal4V096 => Some(4096),
        }
    }

    const fn refsel(self) -> u32 {
        match self {
            Self::Avdd => 0x0,
            Self::External => 0x2,
            Self::Internal1V024 => 0x4,
            Self::Internal2V048 => 0x5,
            Self::Internal4V096 => 0x6,
            Self::Internal2V500 => 0x7,
        }
    }
}

/// How long a read samples its input, in `CLK_ADC` periods (1 to 256:
/// CTRLE.SAMPLEN + 1; 6 MHz, so 167 ns each). Higher source impedances need
/// longer; the temperature sensor at least 32 µs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SampleTime(pub u16);

impl SampleTime {
    /// 16 periods, 2.7 µs: low-impedance sources.
    pub const SHORT: Self = Self(16);
    /// 256 periods, 43 µs: enough for the temperature sensor.
    pub const LONG: Self = Self(256);

    const fn samplen(self) -> u32 {
        let n = if self.0 == 0 { 1 } else { self.0 };
        (if n > 256 { 256 } else { n }) as u32 - 1
    }
}

impl Default for SampleTime {
    fn default() -> Self {
        Self::SHORT
    }
}

/// How to set the ADC up.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Config {
    pub resolution: Resolution,
    pub reference: Reference,
}

/// Why the ADC can't be set up. Nothing stops it on the PL10; this is
/// here for the API the other chips share.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ConfigError {}

pub(crate) trait SealedAdcChannel {
    fn setup(&mut self) {}
    fn channel(&self) -> u8;
}

/// Something the ADC reads: a pin, [`Vddio`] or [`Temperature`].
#[expect(private_bounds, reason = "sealed")]
pub trait AdcChannel: SealedAdcChannel + Sized {
    /// Erases the channel's type, setting a pin up as an analog input.
    fn degrade_adc<'a>(mut self) -> AnyAdcChannel<'a>
    where
        Self: 'a,
    {
        self.setup();
        AnyAdcChannel {
            channel: self.channel(),
            _lifetime: PhantomData,
        }
    }
}

/// A channel of any kind, to keep channels in an array.
pub struct AnyAdcChannel<'a> {
    channel: u8,
    _lifetime: PhantomData<&'a mut ()>,
}

impl AnyAdcChannel<'_> {
    /// The input's number (MUXPOS: `AINn` is n; the inputs inside the
    /// chip from `0x60`).
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}

impl SealedAdcChannel for AnyAdcChannel<'_> {
    fn channel(&self) -> u8 {
        self.channel
    }
}
impl AdcChannel for AnyAdcChannel<'_> {}

/// VDDIO divided by 10, from [`Adc::enable_vddio`]: VDD, measured against
/// an internal reference.
pub struct Vddio;

impl Vddio {
    /// VDDIO in mV, from a reading of it against a reference of
    /// `reference_mv` (an internal one's [`Reference::millivolts`]).
    #[must_use]
    pub const fn millivolts(reading: u16, reference_mv: u32) -> u32 {
        reading as u32 * reference_mv * 10 / 4096
    }
}

impl SealedAdcChannel for Vddio {
    fn channel(&self) -> u8 {
        MUX_VDDIO_DIV10
    }
}
impl AdcChannel for Vddio {}

/// The die's temperature sensor, from [`Adc::enable_temperature`]. Read it
/// against [`Reference::Internal2V048`], sampling for at least 32 µs
/// ([`SampleTime::LONG`]).
pub struct Temperature;

impl SealedAdcChannel for Temperature {
    fn channel(&self) -> u8 {
        MUX_TEMPSENSE
    }
}
impl AdcChannel for Temperature {}

macro_rules! impl_adc_pin {
    ($($pin:ident = $channel:literal),* $(,)?) => {$(
        impl SealedAdcChannel for Peri<'_, peripherals::$pin> {
            fn setup(&mut self) {
                crate::gpio::set_function(&**self, Function::B);
            }
            fn channel(&self) -> u8 {
                $channel
            }
        }
        impl AdcChannel for Peri<'_, peripherals::$pin> {}
    )*};
}

// The ATDF's AIN signals, all on function B.
impl_adc_pin!(
    PA02 = 2,
    PA03 = 3,
    PA04 = 4,
    PA05 = 5,
    PA06 = 6,
    PA07 = 7,
    PA16 = 16,
    PA17 = 17,
    PA18 = 18,
    PA19 = 19,
    PA20 = 20,
    PA21 = 21,
    PA22 = 22,
    PA23 = 23,
    PA24 = 24,
    PA25 = 25,
    PA26 = 26,
    PA27 = 27,
    PA28 = 28,
    PA29 = 29,
    PA31 = 31,
    PB08 = 40,
    PB09 = 41,
    PB10 = 42,
    PB11 = 43,
);

/// The factory's temperature-sensor calibration (SYSCTRL.TEMPSENSE,
/// DS40002667 chapter 34): a gain and an offset, for 12-bit readings
/// against the internal 2.048 V reference.
///
/// The chapter's equation, `T = (offset - result) × gain / 4096` in kelvin,
/// is what this uses: the sensor reads lower as it warms. Its code example
/// instead takes a 10-bit reading less the offset, which with this part's
/// offset (3,460) gives nonsense.
///
/// **Known issue:** on the bench part (gain 1006, offset 3460) the equation
/// gave 33.5 °C with an SHT40 beside the board reading 24.7 °C: about 9 °C
/// high, far more than the chip warms itself at 24 MHz. The preliminary
/// data sheet gives the sensor no accuracy and doesn't settle which form is
/// right; until it does, this reports the equation's result uncorrected,
/// and a one-point calibration against a known temperature is the
/// application's to apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Calibration {
    /// TEMPSENSE0: the gain, in kelvin per 12-bit count, times 4,096.
    pub gain: u16,
    /// TEMPSENSE1: the offset, in 12-bit counts, signed.
    pub offset: i16,
}

impl Calibration {
    /// This chip's calibration.
    #[must_use]
    pub fn read() -> Self {
        crate::clocks::enable_apbc(APBC_SYSCTRL);
        Self::from_word(read32(TEMPSENSE))
    }

    /// From TEMPSENSE.
    #[must_use]
    pub const fn from_word(word: u32) -> Self {
        #[expect(clippy::cast_possible_truncation, reason = "the word's two halves")]
        let (gain, offset) = (word as u16, (word >> 16) as u16);
        Self { gain, offset: offset.cast_signed() }
    }

    /// The die's temperature in tenths of a degree Celsius, by the data
    /// sheet's equation (the offset less the reading, times the gain, over
    /// 4,096, in kelvin), from a 12-bit reading of the sensor against a
    /// reference of `reference_mv` (scaled to the 2.048 V the calibration
    /// is for).
    #[must_use]
    pub fn celsius(&self, reading: u16, reference_mv: u32) -> i32 {
        // The reading at 2.048 V, in sixteenths of a count, for the
        // rounding.
        let reading_16ths = i64::from(reading) * i64::from(reference_mv) * 16 / 2048;
        let kelvin_tenths = (i64::from(self.offset) * 16 - reading_16ths) * i64::from(self.gain) * 10 / (4096 * 16);
        i32::try_from(kelvin_tenths - 2732).unwrap_or(i32::MIN)
    }
}

/// The ADC's interrupt handler: bind `ADC0` to it with
/// [`bind_interrupts!`](crate::bind_interrupts).
pub struct InterruptHandler {
    _private: (),
}

impl Handler<typelevel::ADC0> for InterruptHandler {
    unsafe fn on_interrupt() {
        // Masked; the waiting side reads the result, which clears it.
        write32(INTENCLR, RESRDY);
        WAKER.wake();
    }
}

/// The ADC, enabled while this lives.
pub struct Adc<'d, M> {
    resolution: Resolution,
    _peripheral: PhantomData<&'d mut peripherals::ADC>,
    _mode: PhantomData<M>,
}

impl<'d> Adc<'d, Blocking> {
    /// Sets the ADC up, waiting on its flag.
    ///
    /// # Errors
    ///
    /// None on the PL10 ([`ConfigError`] has no variants).
    #[allow(clippy::must_use_candidate, reason = "a Result, as the other chips' constructor returns")]
    pub fn new_blocking(adc: Peri<'d, peripherals::ADC>, config: Config) -> Result<Self, ConfigError> {
        Ok(Self::setup(adc, config))
    }
}

impl<'d> Adc<'d, Async> {
    /// Sets the ADC up, waiting on its interrupt in [`Adc::read`].
    ///
    /// # Errors
    ///
    /// None on the PL10 ([`ConfigError`] has no variants).
    pub fn new(adc: Peri<'d, peripherals::ADC>, _irqs: impl Binding<typelevel::ADC0, InterruptHandler> + 'd, config: Config) -> Result<Self, ConfigError> {
        let adc = Self::setup(adc, config);
        let irq = crate::interrupt::ADC0;
        irq.unpend();
        // SAFETY: the handler only masks the ADC's flag and wakes.
        unsafe { irq.enable() };
        Ok(adc)
    }

    /// Reads `channel`, sampling it for `sample_time`, and waits for the
    /// result on the interrupt.
    pub async fn read(&mut self, channel: &mut impl AdcChannel, sample_time: SampleTime) -> u16 {
        start(channel, sample_time);
        poll_fn(|cx| {
            WAKER.register(cx.waker());
            if read32(INTFLAG) & RESRDY != 0 {
                Poll::Ready(())
            } else {
                write32(INTENSET, RESRDY);
                Poll::Pending
            }
        })
        .await;
        write32(INTENCLR, RESRDY);
        self.result()
    }
}

impl<'d, M> Adc<'d, M> {
    fn setup(_adc: Peri<'d, peripherals::ADC>, config: Config) -> Self {
        crate::clocks::enable_apbc(APBC_ADC0);
        let bus_hz = crate::clocks::frequencies().map_or(24_000_000, |f| f.cpu());
        write32(CTRLA, SWRST);
        while read32(CTRLA) & SWRST != 0 {}
        write32(INTENCLR, u32::MAX);
        // TIMEBASE: bus cycles in 1 µs, rounded up, for the ADC's own
        // delays (its start-up, an internal reference's 40 µs).
        write32(CTRLB, (bus_hz.div_ceil(1_000_000).min(31) << TIMEBASE_SHIFT) | PRESCALER_DIV4);
        write32(CTRLC, config.reference.refsel());
        // One sample per result, right-aligned, unscaled.
        write32(CTRLD, config.resolution.ressel() << RESSEL_SHIFT);
        write32(CTRLA, ENABLE);
        Self {
            resolution: config.resolution,
            _peripheral: PhantomData,
            _mode: PhantomData,
        }
    }

    /// The result's width.
    #[must_use]
    pub const fn resolution(&self) -> Resolution {
        self.resolution
    }

    /// `CLK_ADC`'s rate: a quarter of the bus clock.
    #[must_use]
    pub fn clock_hz(&self) -> u32 {
        crate::clocks::frequencies().map_or(24_000_000, |f| f.cpu()) / PRESCALER_DIVISOR
    }

    /// VDDIO divided by 10. Read it against an internal reference to know
    /// VDD.
    #[must_use]
    pub fn enable_vddio(&self) -> Vddio {
        Vddio
    }

    /// The temperature sensor. Read it against
    /// [`Reference::Internal2V048`] for [`SampleTime::LONG`].
    #[must_use]
    pub fn enable_temperature(&self) -> Temperature {
        Temperature
    }

    /// Reads `channel`, sampling it for `sample_time`, spinning until the
    /// result is ready.
    pub fn blocking_read(&mut self, channel: &mut impl AdcChannel, sample_time: SampleTime) -> u16 {
        start(channel, sample_time);
        while read32(INTFLAG) & RESRDY == 0 {}
        self.result()
    }

    /// The result, on the 12-bit scale; reading RESULT clears RESRDY.
    fn result(&self) -> u16 {
        #[expect(clippy::cast_possible_truncation, reason = "a single 12-bit conversion")]
        let raw = read32(RESULT) as u16;
        raw << (12 - self.resolution.bits())
    }
}

/// Selects `channel`, sets the sample length and starts a single
/// conversion.
fn start(channel: &mut impl AdcChannel, sample_time: SampleTime) {
    channel.setup();
    write32(CTRLE, sample_time.samplen());
    write32(INPUTCTRL, u32::from(channel.channel()) << MUXPOS_SHIFT);
    write32(INTFLAG, RESRDY);
    write32(COMMAND, SINGLE_NOW);
}

impl<M> Drop for Adc<'_, M> {
    fn drop(&mut self) {
        write32(INTENCLR, u32::MAX);
        write32(CTRLA, 0);
    }
}

fn read32(address: usize) -> u32 {
    // SAFETY: an ADC or SYSCTRL register.
    unsafe { read_volatile(address as *const u32) }
}

fn write32(address: usize, value: u32) {
    // SAFETY: an ADC register.
    unsafe { write_volatile(address as *mut u32, value) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_lengths() {
        assert_eq!(SampleTime(1).samplen(), 0);
        assert_eq!(SampleTime(0).samplen(), 0);
        assert_eq!(SampleTime::SHORT.samplen(), 15);
        assert_eq!(SampleTime::LONG.samplen(), 255);
        assert_eq!(SampleTime(1000).samplen(), 255);
    }

    #[test]
    fn vddio() {
        // 3.3 V: VDDIO/10 is 0.33 V, 1,320 counts against 1.024 V.
        assert_eq!(Vddio::millivolts(1320, 1024), 3300);
    }

    #[test]
    fn temperature() {
        // The bench part's calibration (gain 1006, offset 3460) and reading
        // (2212 against 2.048 V): (3460 - 2212) × 1006 / 4096 = 306.5 K.
        let c = Calibration::from_word(1006 | (3460 << 16));
        assert_eq!((c.gain, c.offset), (1006, 3460));
        assert_eq!(c.celsius(2212, 2048), 333);
        // Warmer reads lower: 4 counts fewer, about 1 K more.
        assert_eq!(c.celsius(2208, 2048), 342);
        // The same voltage read against 1.024 V reads twice as many counts.
        assert_eq!(c.celsius(4424, 1024), 333);
    }
}

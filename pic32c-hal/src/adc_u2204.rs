// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The GV00's ADC (the SAM D20's, IP U2204, at `0x4200_4000`; DS60001881
//! chapter 27): a 12-bit SAR core with a gain stage, single-ended inputs
//! AIN0-19 on pins, and inside the chip the bandgap, VDDCORE/4, VDDIO/4,
//! the temperature sensor and the DAC's output. The same API as the other
//! chips' ADC drivers: [`Adc::blocking_read`] and, on its interrupt (bind
//! `ADC` to [`InterruptHandler`]), [`Adc::read`], each taking a pin
//! (`&mut p.PA03`), [`AnyAdcChannel`], [`VrefInt`], [`Vddio`],
//! [`Temperature`] or [`DacOutput`].
//!
//! Each read is a single conversion started by software (SWTRIG.START)
//! after the input and sample length are set. Results are unsigned on a
//! 12-bit scale, `0..4096`, at every [`Resolution`] (an 8- or 10-bit result
//! is shifted up, its low bits zero). Full scale is the [`Reference`]
//! divided by the [`Gain`]: the default, half AVDD at half gain, is AVDD
//! (the usual way to read the whole supply range, as the reference may be at
//! most AVDD - 0.6 V).
//!
//! The ADC is clocked from its generic clock (`GCLK_ADC`, from
//! [`Config::generator`]) through its prescaler, which the driver sets to
//! keep `CLK_ADC` at or under 2.1 MHz (the data sheet's 476 ns minimum
//! period): 2 MHz from 8 MHz, 1.5 MHz from 48 MHz. The factory's linearity
//! and bias calibration (the NVM software calibration area) goes into CALIB
//! at set-up. The temperature sensor has no factory calibration in the data
//! sheet: [`Temperature::typical_celsius`] works from its typical figures
//! (±10 °C).

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

const ADC: usize = 0x4200_4000;
const CTRLA: usize = ADC;
const REFCTRL: usize = ADC + 0x01;
const AVGCTRL: usize = ADC + 0x02;
const SAMPCTRL: usize = ADC + 0x03;
const CTRLB: usize = ADC + 0x04;
const SWTRIG: usize = ADC + 0x0C;
const INPUTCTRL: usize = ADC + 0x10;
const INTENCLR: usize = ADC + 0x16;
const INTENSET: usize = ADC + 0x17;
const INTFLAG: usize = ADC + 0x18;
const STATUS: usize = ADC + 0x19;
const RESULT: usize = ADC + 0x1A;
const CALIB: usize = ADC + 0x28;

const SWRST: u8 = 1 << 0;
const ENABLE: u8 = 1 << 1;
const SYNCBUSY: u8 = 1 << 7;
const START: u8 = 1 << 1;
const RESRDY: u8 = 1 << 0;
const PRESCALER_SHIFT: u16 = 8;
const RESSEL_SHIFT: u16 = 4;
/// INPUTCTRL.MUXNEG: internal ground, for single-ended reads.
const MUXNEG_GND: u32 = 0x18 << 8;
const GAIN_SHIFT: u32 = 24;
/// `CLK_ADC`'s highest rate: 1 / 476.2 ns.
const MAX_CLK_ADC_HZ: u32 = 2_100_000;
/// `GCLK_ADC`'s channel (a CLKCTRL.ID).
const GCLK_ADC: usize = 0x17;
/// PM.APBCMASK: the ADC's bus clock (on at reset).
const APBC_ADC: u32 = 1 << 16;

/// SYSCTRL.VREF, whose TSEN and BGOUTEN route the temperature sensor and
/// the bandgap to the ADC.
const SYSCTRL_VREF: usize = 0x4000_0840;
const TSEN: u32 = 1 << 1;
const BGOUTEN: u32 = 1 << 2;
/// The NVM software calibration area's first 64 bits (table 7-7).
const NVM_CALIBRATION: usize = 0x0080_6020;

/// MUXPOS for the inputs inside the chip.
const MUX_TEMP: u8 = 0x18;
const MUX_BANDGAP: u8 = 0x19;
const MUX_VDDIO_DIV4: u8 = 0x1B;
const MUX_DAC: u8 = 0x1C;

static WAKER: AtomicWaker = AtomicWaker::new();

/// The result's width (CTRLB.RESSEL). Results stay on a 12-bit scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resolution {
    Bits8,
    Bits10,
    #[default]
    Bits12,
}

impl Resolution {
    /// The bits.
    #[must_use]
    pub const fn bits(self) -> u8 {
        match self {
            Self::Bits8 => 8,
            Self::Bits10 => 10,
            Self::Bits12 => 12,
        }
    }

    const fn ressel(self) -> u16 {
        match self {
            Self::Bits12 => 0x0,
            Self::Bits10 => 0x2,
            Self::Bits8 => 0x3,
        }
    }
}

/// The reference (REFCTRL.REFSEL).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reference {
    /// The internal 1.0 V reference (from the 1.1 V bandgap).
    Internal1V0,
    /// AVDD / 1.48.
    AvddDiv1v48,
    /// AVDD / 2 (only for AVDD over 2.0 V): with [`Gain::Half`], full scale
    /// is AVDD.
    #[default]
    AvddDiv2,
    /// The external VREFA pin (PA03).
    ExternalA,
    /// The external VREFB pin (PA04).
    ExternalB,
}

impl Reference {
    /// An internal fixed reference's voltage in mV; `None` for those that
    /// follow AVDD or come from a pin.
    #[must_use]
    pub const fn millivolts(self) -> Option<u32> {
        match self {
            Self::Internal1V0 => Some(1000),
            _ => None,
        }
    }

    const fn refsel(self) -> u8 {
        match self {
            Self::Internal1V0 => 0x0,
            Self::AvddDiv1v48 => 0x1,
            Self::AvddDiv2 => 0x2,
            Self::ExternalA => 0x3,
            Self::ExternalB => 0x4,
        }
    }
}

/// The gain stage (INPUTCTRL.GAIN): full scale is the reference divided by
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gain {
    X1,
    X2,
    X4,
    X8,
    X16,
    /// One half: full scale twice the reference.
    #[default]
    Half,
}

impl Gain {
    const fn code(self) -> u32 {
        match self {
            Self::X1 => 0x0,
            Self::X2 => 0x1,
            Self::X4 => 0x2,
            Self::X8 => 0x3,
            Self::X16 => 0x4,
            Self::Half => 0xF,
        }
    }

    /// Full scale in mV for a reference of `reference_mv`.
    #[must_use]
    pub const fn full_scale_mv(self, reference_mv: u32) -> u32 {
        match self {
            Self::X1 => reference_mv,
            Self::X2 => reference_mv / 2,
            Self::X4 => reference_mv / 4,
            Self::X8 => reference_mv / 8,
            Self::X16 => reference_mv / 16,
            Self::Half => reference_mv * 2,
        }
    }
}

/// How long a read samples its input, in `CLK_ADC` periods (1 to 32:
/// SAMPCTRL.SAMPLEN counts half periods). Higher source impedances need
/// longer, and the inputs inside the chip [`SampleTime::LONG`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SampleTime(pub u16);

impl SampleTime {
    /// 1 period (the hardware's least, half a period plus the half SAMPLEN
    /// 0 adds), 0.5 µs at 2 MHz: low-impedance sources.
    pub const SHORT: Self = Self(1);
    /// 32 periods, 16 µs at 2 MHz: the bandgap, the temperature sensor.
    pub const LONG: Self = Self(32);

    #[expect(clippy::cast_possible_truncation, reason = "at most 63")]
    const fn samplen(self) -> u8 {
        let n = if self.0 == 0 { 1 } else { self.0 };
        (if n > 32 { 32 } else { n } * 2 - 1) as u8
    }
}

impl Default for SampleTime {
    fn default() -> Self {
        Self::SHORT
    }
}

/// How to set the ADC up.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub resolution: Resolution,
    pub reference: Reference,
    pub gain: Gain,
    /// The generic clock generator to feed the ADC from.
    pub generator: usize,
}

impl Default for Config {
    /// 12 bits, AVDD full scale (half AVDD at half gain), from generator 0.
    fn default() -> Self {
        Self {
            resolution: Resolution::default(),
            reference: Reference::default(),
            gain: Gain::default(),
            generator: 0,
        }
    }
}

/// Why the ADC can't be set up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ConfigError {
    /// [`Config::generator`] isn't configured.
    NoClock,
    /// The generator is too fast for the prescaler (over 512 × 2.1 MHz).
    ClockTooFast,
}

pub(crate) trait SealedAdcChannel {
    fn setup(&mut self) {}
    fn channel(&self) -> u8;
}

/// Something the ADC reads: a pin, or an input inside the chip.
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
    /// The input's number (MUXPOS: `AINn` is n; the inputs inside the chip
    /// from `0x18`).
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

/// The 1.1 V bandgap, from [`Adc::enable_vrefint`]. Read against a
/// reference that follows AVDD, it gives AVDD.
pub struct VrefInt;

impl VrefInt {
    /// The bandgap's nominal voltage (typical; the data sheet gives no
    /// factory reading of it).
    pub const NOMINAL_MV: u32 = 1100;

    /// Full scale in mV, from a reading of the bandgap: AVDD, with the
    /// default reference and gain.
    #[must_use]
    pub const fn full_scale_mv(reading: u16) -> u32 {
        if reading == 0 { 0 } else { Self::NOMINAL_MV * 4096 / reading as u32 }
    }
}

impl SealedAdcChannel for VrefInt {
    fn channel(&self) -> u8 {
        MUX_BANDGAP
    }
}
impl AdcChannel for VrefInt {}

/// VDDIO divided by 4, from [`Adc::enable_vddio`].
pub struct Vddio;

impl Vddio {
    /// VDDIO in mV, from a reading of it at a full scale of
    /// `full_scale_mv` ([`Gain::full_scale_mv`] of an internal reference).
    #[must_use]
    pub const fn millivolts(reading: u16, full_scale_mv: u32) -> u32 {
        reading as u32 * full_scale_mv * 4 / 4096
    }
}

impl SealedAdcChannel for Vddio {
    fn channel(&self) -> u8 {
        MUX_VDDIO_DIV4
    }
}
impl AdcChannel for Vddio {}

/// The die's temperature sensor, from [`Adc::enable_temperature`]. Read it
/// for [`SampleTime::LONG`].
pub struct Temperature;

impl Temperature {
    /// The die's temperature in tenths of a degree Celsius from the sensor's
    /// output in mV, by the data sheet's typical figures (0.667 V at 25 °C,
    /// 2.4 mV per °C; ±10 °C absolute).
    #[must_use]
    pub const fn typical_celsius(millivolts: u32) -> i32 {
        #[expect(clippy::cast_possible_wrap, reason = "millivolts of an ADC input")]
        let mv = millivolts as i32;
        250 + (mv - 667) * 10 * 10 / 24
    }
}

impl SealedAdcChannel for Temperature {
    fn channel(&self) -> u8 {
        MUX_TEMP
    }
}
impl AdcChannel for Temperature {}

/// The DAC's output, inside the chip, through the DAC's output buffer: a
/// [`Dac::new`](crate::dac::Dac::new), which drives PA02 too (the buffer
/// is what feeds the ADC; `Dac::new_internal` doesn't turn it on).
pub struct DacOutput;

impl SealedAdcChannel for DacOutput {
    fn channel(&self) -> u8 {
        MUX_DAC
    }
}
impl AdcChannel for DacOutput {}

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
    PA02 = 0,
    PA03 = 1,
    PB08 = 2,
    PB09 = 3,
    PA04 = 4,
    PA05 = 5,
    PA06 = 6,
    PA07 = 7,
    PB00 = 8,
    PB01 = 9,
    PB02 = 10,
    PB03 = 11,
    PB04 = 12,
    PB05 = 13,
    PB06 = 14,
    PB07 = 15,
    PA08 = 16,
    PA09 = 17,
    PA10 = 18,
    PA11 = 19,
);

/// The ADC's interrupt handler: bind `ADC` to it with
/// [`bind_interrupts!`](crate::bind_interrupts).
pub struct InterruptHandler {
    _private: (),
}

impl Handler<typelevel::ADC> for InterruptHandler {
    unsafe fn on_interrupt() {
        // Masked; the waiting side reads the result, which clears it.
        write8(INTENCLR, RESRDY);
        WAKER.wake();
    }
}

/// The ADC, enabled while this lives.
pub struct Adc<'d, M> {
    resolution: Resolution,
    clock_hz: u32,
    _peripheral: PhantomData<&'d mut peripherals::ADC>,
    _mode: PhantomData<M>,
}

impl<'d> Adc<'d, Blocking> {
    /// Sets the ADC up, waiting on its flag.
    ///
    /// # Errors
    ///
    /// If [`Config::generator`] isn't configured, or is too fast.
    pub fn new_blocking(adc: Peri<'d, peripherals::ADC>, config: Config) -> Result<Self, ConfigError> {
        Self::setup(adc, config)
    }
}

impl<'d> Adc<'d, Async> {
    /// Sets the ADC up, waiting on its interrupt in [`Adc::read`].
    ///
    /// # Errors
    ///
    /// As [`Adc::new_blocking`].
    pub fn new(adc: Peri<'d, peripherals::ADC>, _irqs: impl Binding<typelevel::ADC, InterruptHandler> + 'd, config: Config) -> Result<Self, ConfigError> {
        let adc = Self::setup(adc, config)?;
        let irq = crate::interrupt::ADC;
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
            if read8(INTFLAG) & RESRDY != 0 {
                Poll::Ready(())
            } else {
                write8(INTENSET, RESRDY);
                Poll::Pending
            }
        })
        .await;
        write8(INTENCLR, RESRDY);
        self.result()
    }
}

/// The prescaler setting (CTRLB.PRESCALER, DIV4 << n) that brings `hz` to
/// `MAX_CLK_ADC_HZ` or under, and the rate it gives.
fn prescaler_for(hz: u32) -> Option<(u16, u32)> {
    (0..8u16).map(|p| (p, hz >> (p + 2))).find(|&(_, clk)| clk <= MAX_CLK_ADC_HZ)
}

/// The linearity and bias calibration (bits 34:27 and 37:35 of the NVM
/// software calibration area), as CALIB takes them.
fn factory_calib() -> u16 {
    // SAFETY: the calibration area, always readable, a word at a time.
    let high = unsafe { read_volatile((NVM_CALIBRATION + 4) as *const u32) };
    // SAFETY: as above.
    let low = unsafe { read_volatile(NVM_CALIBRATION as *const u32) };
    let area = (u64::from(high) << 32) | u64::from(low);
    let (linearity, bias) = (((area >> 27) & 0xFF) as u16, ((area >> 35) & 0x7) as u16);
    linearity | (bias << 8)
}

impl<'d, M> Adc<'d, M> {
    fn setup(_adc: Peri<'d, peripherals::ADC>, config: Config) -> Result<Self, ConfigError> {
        let hz = crate::clocks::generator_hz(config.generator).ok_or(ConfigError::NoClock)?;
        let (prescaler, clock_hz) = prescaler_for(hz).ok_or(ConfigError::ClockTooFast)?;
        crate::clocks::enable_apbc(APBC_ADC);
        crate::clocks::enable_channel(GCLK_ADC, config.generator);
        write8(CTRLA, SWRST);
        while read8(CTRLA) & SWRST != 0 || read8(STATUS) & SYNCBUSY != 0 {}
        write8(INTENCLR, 0xFF);
        write16(CALIB, factory_calib());
        write8(REFCTRL, config.reference.refsel());
        // One sample per result.
        write8(AVGCTRL, 0);
        // Single-ended, right-aligned.
        write16(CTRLB, (prescaler << PRESCALER_SHIFT) | (config.resolution.ressel() << RESSEL_SHIFT));
        sync();
        write32(INPUTCTRL, MUXNEG_GND | (config.gain.code() << GAIN_SHIFT));
        sync();
        write8(CTRLA, ENABLE);
        sync();
        let mut adc = Self {
            resolution: config.resolution,
            clock_hz,
            _peripheral: PhantomData,
            _mode: PhantomData,
        };
        // The first conversion after the reference is set may be off: one
        // thrown away.
        let _ = adc.blocking_read(&mut DacOutput, SampleTime::LONG);
        Ok(adc)
    }

    /// The result's width.
    #[must_use]
    pub const fn resolution(&self) -> Resolution {
        self.resolution
    }

    /// `CLK_ADC`'s rate.
    #[must_use]
    pub const fn clock_hz(&self) -> u32 {
        self.clock_hz
    }

    /// The bandgap, routed to the ADC (SYSCTRL.VREF.BGOUTEN).
    #[must_use]
    pub fn enable_vrefint(&self) -> VrefInt {
        set_vref(BGOUTEN);
        VrefInt
    }

    /// VDDIO divided by 4.
    #[must_use]
    pub fn enable_vddio(&self) -> Vddio {
        Vddio
    }

    /// The temperature sensor, turned on (SYSCTRL.VREF.TSEN). Read it for
    /// [`SampleTime::LONG`].
    #[must_use]
    pub fn enable_temperature(&self) -> Temperature {
        set_vref(TSEN);
        Temperature
    }

    /// The DAC's output, inside the chip.
    #[must_use]
    pub fn enable_dac_output(&self) -> DacOutput {
        DacOutput
    }

    /// Reads `channel`, sampling it for `sample_time`, spinning until the
    /// result is ready.
    pub fn blocking_read(&mut self, channel: &mut impl AdcChannel, sample_time: SampleTime) -> u16 {
        start(channel, sample_time);
        while read8(INTFLAG) & RESRDY == 0 {}
        self.result()
    }

    /// The result, on the 12-bit scale; reading RESULT clears RESRDY.
    fn result(&self) -> u16 {
        read16(RESULT) << (12 - self.resolution.bits())
    }
}

/// Sets `bits` in SYSCTRL.VREF, keeping its calibration.
fn set_vref(bits: u32) {
    critical_section::with(|_| write32(SYSCTRL_VREF, read32(SYSCTRL_VREF) | bits));
}

/// Selects `channel`, sets the sample length and starts a single
/// conversion.
fn start(channel: &mut impl AdcChannel, sample_time: SampleTime) {
    channel.setup();
    write8(SAMPCTRL, sample_time.samplen());
    let inputctrl = read32(INPUTCTRL) & !0x1F;
    write32(INPUTCTRL, inputctrl | u32::from(channel.channel()));
    sync();
    write8(INTFLAG, RESRDY);
    write8(SWTRIG, START);
    sync();
}

fn sync() {
    while read8(STATUS) & SYNCBUSY != 0 {}
}

impl<M> Drop for Adc<'_, M> {
    fn drop(&mut self) {
        write8(INTENCLR, 0xFF);
        write8(CTRLA, 0);
        sync();
    }
}

fn read8(address: usize) -> u8 {
    // SAFETY: an ADC register, read at its width.
    unsafe { read_volatile(address as *const u8) }
}

fn write8(address: usize, value: u8) {
    // SAFETY: an ADC register; only this driver writes them.
    unsafe { write_volatile(address as *mut u8, value) }
}

fn read16(address: usize) -> u16 {
    // SAFETY: as for `read8`.
    unsafe { read_volatile(address as *const u16) }
}

fn write16(address: usize, value: u16) {
    // SAFETY: as for `write8`.
    unsafe { write_volatile(address as *mut u16, value) }
}

fn read32(address: usize) -> u32 {
    // SAFETY: an ADC or SYSCTRL register, read whole.
    unsafe { read_volatile(address as *const u32) }
}

fn write32(address: usize, value: u32) {
    // SAFETY: an ADC register, or SYSCTRL.VREF (read-modify-written under a
    // critical section).
    unsafe { write_volatile(address as *mut u32, value) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_lengths() {
        assert_eq!(SampleTime(0).samplen(), 1);
        assert_eq!(SampleTime::SHORT.samplen(), 1);
        assert_eq!(SampleTime::LONG.samplen(), 63);
        assert_eq!(SampleTime(100).samplen(), 63);
    }

    #[test]
    fn prescalers() {
        assert_eq!(prescaler_for(8_000_000), Some((0, 2_000_000)));
        assert_eq!(prescaler_for(47_972_352), Some((3, 1_499_136)));
        assert_eq!(prescaler_for(1_000_000), Some((0, 250_000)));
    }

    #[test]
    fn conversions() {
        // 3.3 V: VDDIO/4 is 825 mV, 1,024 counts at a 3.3 V full scale.
        assert_eq!(Vddio::millivolts(1024, 3300), 3300);
        // The bandgap at 1,365 counts: full scale 3.3 V.
        assert_eq!(VrefInt::full_scale_mv(1365), 3300);
        assert_eq!(Temperature::typical_celsius(667), 250);
        assert_eq!(Temperature::typical_celsius(691), 350);
        assert_eq!(Gain::Half.full_scale_mv(1650), 3300);
    }
}

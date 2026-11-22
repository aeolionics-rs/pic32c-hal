// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The ADC (at `0x4501_0000` on the PIC32CK, `0x4481_8000` on the PIC32CM
//! SG00, the same IP): one 12-bit SAR core with single-ended inputs. On the
//! PIC32CK, the pins VINP0-11 and two inside the chip, the temperature
//! sensor (AIN12) and the 1.2 V bandgap, IVREF (AIN13); DS60001795 section
//! 44. On the SG00, the pins AIN0-11 and the bandgap on 14 (DS60001921's
//! table lists no temperature sensor among the inputs, and its CAL OTP no
//! factory readings, so `Temperature` and `Calibration` are the PIC32CK's
//! only); bind `ADC_REQ1` there.
//!
//! Its channel, trigger and clock settings are fixed while it's enabled (a
//! write then is a bus error), so [`Adc`] sets them once, with no hardware
//! triggers, and converts in the core's software-controlled mode
//! (CTRLB.SWCNVEN): each read selects its channel, samples it for the read's
//! [`SampleTime`], then converts. That's embassy-stm32's shape:
//! [`Adc::blocking_read`] and, on the core's interrupt, [`Adc::read`], each
//! taking a pin (`&mut p.PA10`), [`AnyAdcChannel`], [`VrefInt`] or
//! [`Temperature`].
//!
//! Results are unsigned on a 12-bit scale, `0..4096`, full scale being the
//! reference: AVDD, or VREFP on PB01 ([`Reference`]). A lower
//! [`Resolution`] converts faster but keeps that scale, leaving the low
//! bits zero (measured: the bandgap reads 1488 at 10 and 8 bits, 1489 at
//! 12). [`Calibration`]
//! holds the factory's readings of the bandgap and the temperature sensor,
//! from which [`Calibration::millivolts`] and [`Calibration::celsius`]
//! work.
//!
//! **Erratum 2.1.2 (DS80001066E, A0 silicon):** with PA09 (VINP5) or PA10
//! (VINP6) on the ADC's function, "a potentially destructive short can
//! occur" if the analog comparator selects `AC_AIN0` (PA06) or `AC_AIN1`
//! (PA07). This HAL has no AC driver; don't enable the AC on those inputs
//! while those pins are analog.
//!
//! # Known issue: the temperature sensor
//!
//! [`Calibration::celsius`] reads about 65 °C low on the parts this HAL
//! supports, and doesn't correct for it yet.
//!
//! Microchip's erratum 2.18.1 (DS80001066E, "SUPC: ADC VREF"; revision A0,
//! 256 KB and 512 KB SRAM parts, not the 128 KB ones) says the sensor's
//! output is offset when the ADC runs in normal mode, and that the offset,
//! "Moff", is to be derived from the production calibration and subtracted.
//! It doesn't say which calibration value gives Moff, and the documented
//! CAL OTP words (FCCFG69-71) hold none that would.
//!
//! What we measured on two boards (an SG01 and a GC01), against the
//! boards' 0.1 % 2.048 V reference (an MCP1501 on VREFP) with an SHT40
//! giving the air's temperature: the bandgap reads within 3 mV of its
//! factory value, but the sensor's output is 180 to 190 mV below the
//! calibration's line, the same in the ADC's triggered and
//! software-controlled modes and against either reference. So the offset
//! is in the sensor's output voltage, consistent with the erratum, and
//! about the same on both parts.
//!
//! Until Microchip says how Moff is meant to be found, this driver reports
//! the sensor as calibrated, uncorrected. An application that needs the die
//! temperature can subtract its own offset: about 185 mV (in counts at the
//! measured reference) gives roughly ±3 °C on these parts, and a one-point
//! calibration against a known temperature about ±1 °C (the `moff` bench
//! program shows how).
//!
//! The core needs AVDD of 2.5 V or more as this driver sets it up (below
//! that the SUPC's charge pumps must be on, which it doesn't do).

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

#[cfg(pic32ck)]
const ADC: usize = 0x4501_0000;
#[cfg(pic32cm_sg)]
const ADC: usize = 0x4481_8000;

/// The core's interrupt line (channel ready).
#[cfg(pic32ck)]
type Line = typelevel::ADC_CORE1;
#[cfg(pic32cm_sg)]
type Line = typelevel::ADC_REQ1;
const CTRLA: usize = ADC;
const CTRLB: usize = ADC + 0x04;
const CTRLD: usize = ADC + 0x10;
const CORCTRL: usize = ADC + 0x20;
const CHNCFG1: usize = ADC + 0x24;
const CHNCFG2: usize = ADC + 0x28;
const CHNCFG3: usize = ADC + 0x2C;
const CHNCFG4: usize = ADC + 0x30;
const CHNCFG5: usize = ADC + 0x34;
const CALCTRL: usize = ADC + 0x38;
const EVCTRL: usize = ADC + 0x3C;
const CORCHDATAID: usize = ADC + 0xD0;
const CHRDYDAT: usize = ADC + 0xD4;
const SYNCBUSY: usize = ADC + 0xE8;
const CTLINTENCLR: usize = ADC + 0x100;
const CTLINTFLAG: usize = ADC + 0x104;
const INTENCLR: usize = ADC + 0x120;
const INTENSET: usize = ADC + 0x124;
const INTFLAG: usize = ADC + 0x128;

const SWRST: u32 = 1 << 0;
const ENABLE: u32 = 1 << 1;
const ANAEN: u32 = 1 << 2;
const SYNC_CTRLB: u32 = 1 << 2;

const SWCNVEN: u32 = 1 << 15;
const SAMP: u32 = 1 << 7;
const RQCNVRT: u32 = 1 << 6;

const CHNEN: u32 = 1 << 16;
const ANLEN: u32 = 1 << 20;
const WKUP_SHIFT: u32 = 24;
const VREFSEL_SHIFT: u32 = 28;

const ADCDIV_SHIFT: u32 = 24;
const SELRES_SHIFT: u32 = 10;

const VREFRDY: u32 = 1 << 7;
const CRRDY0: u32 = 1 << 0;
const CHRDY_SHIFT: u32 = 16;
const CHRDY_ALL: u32 = 0xFFFF << CHRDY_SHIFT;

/// GCLK peripheral channel 33.
const GCLK_CHANNEL: usize = 33;
/// The most `GCLK_ADC` may be.
const GCLK_MAX_HZ: u32 = 100_000_000;
/// The least warm-up after the analog circuits are enabled (CTRLD.WKUPEXP).
const WARM_UP_NANOS: u64 = 50_000;

/// SUPC.VREFCTRL: TSEN puts the temperature sensor on AIN12.
#[cfg(pic32ck)]
const SUPC_VREFCTRL: usize = 0x4400_8020;
#[cfg(pic32ck)]
const TSEN: u32 = 1 << 4;

/// The CAL OTP: FCCFG65 holds the ADC's CALCTRL, FCCFG69-71 the
/// temperature sensor's and the bandgap's factory readings.
const FCCFG65: usize = 0x0A00_7184;
#[cfg(pic32ck)]
const FCCFG69: usize = 0x0A00_7194;
#[cfg(pic32ck)]
const FCCFG70: usize = 0x0A00_7198;
#[cfg(pic32ck)]
const FCCFG71: usize = 0x0A00_719C;

/// Polls of a ready flag before giving up: well over the warm-up.
const POLLS: u32 = 10_000_000;

#[cfg(pic32ck)]
const TEMPERATURE_CHANNEL: u8 = 12;
#[cfg(pic32ck)]
const VREFINT_CHANNEL: u8 = 13;
#[cfg(pic32cm_sg)]
const VREFINT_CHANNEL: u8 = 14;

static WAKER: AtomicWaker = AtomicWaker::new();

/// The result's width (CORCTRL.SELRES).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resolution {
    Bits6,
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
            Self::Bits6 => 6,
            Self::Bits8 => 8,
            Self::Bits10 => 10,
            Self::Bits12 => 12,
        }
    }

    const fn selres(self) -> u32 {
        match self {
            Self::Bits6 => 0,
            Self::Bits8 => 1,
            Self::Bits10 => 2,
            Self::Bits12 => 3,
        }
    }
}

/// Full scale's voltage (CTRLD.VREFSEL).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reference {
    /// AVDD.
    #[default]
    Avdd,
    /// VREFP on PB01 on the PIC32CK, VREFH on PA09 on the SG00 (2.4 V to
    /// AVDD). The driver doesn't yet put the pin on its analog function
    /// (B): do that first, with
    /// [`gpio::set_as_function`](crate::gpio::set_as_function).
    External,
}

/// How long a read samples its input before converting, at least, in ADC
/// clock periods (TAD; `Config::generator`'s rate over two). The driver
/// times it by spinning the CPU, which runs long (measured: 1,024 TAD at
/// 24 MHz, 43 µs, took about 125 µs). Higher source impedances need
/// longer; the bandgap at least 30 µs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SampleTime(pub u16);

impl SampleTime {
    /// 16 TAD: low-impedance sources.
    pub const SHORT: Self = Self(16);
    /// 1,024 TAD: 43 µs at the 24 MHz TAD that 48 MHz gives, enough for
    /// the bandgap and the temperature sensor.
    pub const LONG: Self = Self(1024);
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
    /// The GCLK generator feeding `GCLK_ADC` (at most 100 MHz; 3 by default,
    /// 48 MHz on the boards' clocks). TAD is half of it.
    pub generator: usize,
}

impl Default for Config {
    /// 12 bits against AVDD, from generator 3.
    fn default() -> Self {
        Self {
            resolution: Resolution::Bits12,
            reference: Reference::Avdd,
            generator: 3,
        }
    }
}

/// Why the ADC can't be set up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ConfigError {
    /// The generator isn't running, or runs over 100 MHz.
    Clock,
    /// The reference never became ready (CTLINTFLAG.VREFRDY): with
    /// [`Reference::External`], nothing on VREFP.
    Reference,
    /// The core never became ready after its warm-up (CTLINTFLAG.CRRDY0).
    Core,
}

pub(crate) trait SealedAdcChannel {
    fn setup(&mut self) {}
    fn channel(&self) -> u8;
}

/// Something the ADC reads: a pin, [`VrefInt`] or [`Temperature`].
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
    /// The input's number, AIN0-13.
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

/// The 1.2 V bandgap (AIN13 on the PIC32CK, input 14 on the SG00), from
/// [`Adc::enable_vrefint`].
pub struct VrefInt;

impl SealedAdcChannel for VrefInt {
    fn channel(&self) -> u8 {
        VREFINT_CHANNEL
    }
}
impl AdcChannel for VrefInt {}

/// The die's temperature sensor, AIN12, from [`Adc::enable_temperature`].
#[cfg(pic32ck)]
pub struct Temperature;

#[cfg(pic32ck)]
impl SealedAdcChannel for Temperature {
    fn channel(&self) -> u8 {
        TEMPERATURE_CHANNEL
    }
}
#[cfg(pic32ck)]
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

// VINP0-11 (the ATDF's ADC signals, function B), the same on the SG01 and
// GC01.
#[cfg(pic32ck)]
impl_adc_pin!(PA04 = 0, PA05 = 1, PA06 = 2, PA07 = 3, PA08 = 4, PA09 = 5, PA10 = 6, PA11 = 7, PA12 = 8, PB00 = 9, PB11 = 10, PB12 = 11,);
// AIN0-11 on the SG00 (likewise, function B).
#[cfg(pic32cm_sg)]
impl_adc_pin!(PA03 = 0, PA04 = 1, PA05 = 2, PA06 = 3, PA07 = 4, PA08 = 5, PA09 = 6, PB00 = 7, PB03 = 8, PB04 = 9, PB05 = 10, PB06 = 11,);

#[cfg(pic32ck)]
/// The factory's readings, from the CAL OTP (DS60001795 section 27.6.10):
/// the temperature sensor (PTAT) and the buffered bandgap at two
/// temperatures, 12 bits against `reference_mv`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Calibration {
    pub room_ptat: u16,
    pub room_bandgap: u16,
    /// Room temperature in tenths of a degree Celsius.
    pub room_decidegrees: i32,
    pub hot_ptat: u16,
    pub hot_bandgap: u16,
    /// The hot temperature in tenths of a degree Celsius.
    pub hot_decidegrees: i32,
    /// The reference (AVDD) the readings were taken against, in mV.
    pub reference_mv: u16,
}

#[cfg(pic32ck)]
impl Calibration {
    /// This chip's calibration.
    #[must_use]
    pub fn read() -> Self {
        Self::from_words(read32(FCCFG69), read32(FCCFG70), read32(FCCFG71))
    }

    /// From FCCFG69, FCCFG70 and FCCFG71.
    #[must_use]
    pub fn from_words(fccfg69: u32, fccfg70: u32, fccfg71: u32) -> Self {
        let field = |word: u32, shift: u32, bits: u32| (word >> shift) & ((1 << bits) - 1);
        #[expect(clippy::cast_possible_truncation, reason = "masked to 12 bits")]
        let twelve = |word: u32, shift: u32| field(word, shift, 12) as u16;
        #[expect(clippy::cast_possible_wrap, reason = "masked to 8 and 4 bits")]
        let decidegrees = |integer: u32, tenths: u32| integer as i32 * 10 + tenths as i32;
        Self {
            room_ptat: twelve(fccfg69, 0),
            room_bandgap: twelve(fccfg69, 12),
            room_decidegrees: decidegrees(field(fccfg69, 24, 8), field(fccfg70, 0, 4)),
            reference_mv: twelve(fccfg70, 4),
            hot_decidegrees: decidegrees(field(fccfg71, 24, 8), field(fccfg70, 16, 4)),
            hot_ptat: twelve(fccfg71, 0),
            hot_bandgap: twelve(fccfg71, 12),
        }
    }

    /// The reference's voltage in mV, from a 12-bit reading of the
    /// bandgap against it: the factory's bandgap reading scaled by how much
    /// smaller the reference is. `None` for a zero reading.
    #[must_use]
    pub fn millivolts(&self, bandgap: u16) -> Option<u32> {
        if bandgap == 0 {
            return None;
        }
        let bandgap_cal = u32::from(self.room_bandgap) + u32::from(self.hot_bandgap);
        Some(u32::from(self.reference_mv) * bandgap_cal / (2 * u32::from(bandgap)))
    }

    /// The die's temperature in tenths of a degree Celsius, by the data
    /// sheet's method (section 27.6.10), from a 12-bit reading of the
    /// temperature sensor against a reference of `reference_mv` (the PTAT
    /// reading scaled to the factory's reference, then placed on the line
    /// between its two calibration points). `None` if the calibration's two
    /// points are the same.
    ///
    /// **Known issue:** on current silicon this reads about 65 °C low; see
    /// [the module's notes](self#known-issue-the-temperature-sensor).
    #[must_use]
    pub fn celsius(&self, ptat: u16, reference_mv: u32) -> Option<i32> {
        let span = i64::from(self.hot_ptat) - i64::from(self.room_ptat);
        if span == 0 || self.reference_mv == 0 {
            return None;
        }
        // The reading as the factory's reference would have seen it.
        let scaled = i64::from(ptat) * i64::from(reference_mv) / i64::from(self.reference_mv);
        let degrees = i64::from(self.hot_decidegrees - self.room_decidegrees);
        let t = i64::from(self.room_decidegrees) + (scaled - i64::from(self.room_ptat)) * degrees / span;
        i32::try_from(t).ok()
    }
}

/// The ADC's core interrupt handler: bind `ADC_CORE1` (PIC32CK) or
/// `ADC_REQ1` (SG00) to it with
/// [`bind_interrupts!`](crate::bind_interrupts).
pub struct InterruptHandler {
    _private: (),
}

impl Handler<Line> for InterruptHandler {
    unsafe fn on_interrupt() {
        // Mask the channel-ready flags; the waiting side clears them.
        write32(INTENCLR, CHRDY_ALL);
        WAKER.wake();
    }
}

/// The ADC, enabled while this lives.
pub struct Adc<'d, M> {
    resolution: Resolution,
    /// CPU cycles per TAD, for timing the sampling.
    cpu_per_tad: u32,
    _peripheral: PhantomData<&'d mut peripherals::ADC>,
    _mode: PhantomData<M>,
}

impl<'d> Adc<'d, Blocking> {
    /// Sets the ADC up, waiting on its flags.
    ///
    /// # Errors
    ///
    /// See [`ConfigError`].
    pub fn new_blocking(adc: Peri<'d, peripherals::ADC>, config: Config) -> Result<Self, ConfigError> {
        Self::setup(adc, config)
    }
}

impl<'d> Adc<'d, Async> {
    /// Sets the ADC up, waiting on its core interrupt in [`Adc::read`].
    ///
    /// # Errors
    ///
    /// See [`ConfigError`].
    pub fn new(adc: Peri<'d, peripherals::ADC>, _irqs: impl Binding<Line, InterruptHandler> + 'd, config: Config) -> Result<Self, ConfigError> {
        let adc = Self::setup(adc, config)?;
        #[cfg(pic32ck)]
        let irq = crate::interrupt::ADC_CORE1;
        #[cfg(pic32cm_sg)]
        let irq = crate::interrupt::ADC_REQ1;
        irq.unpend();
        // SAFETY: the handler only masks the ADC's flags and wakes.
        unsafe { irq.enable() };
        Ok(adc)
    }

    /// Reads `channel`, sampling it for `sample_time`, and waits for the
    /// result on the interrupt (sampling itself is timed by spinning).
    pub async fn read(&mut self, channel: &mut impl AdcChannel, sample_time: SampleTime) -> u16 {
        let k = self.start(channel, sample_time);
        let ready = 1 << (CHRDY_SHIFT + u32::from(k));
        poll_fn(|cx| {
            WAKER.register(cx.waker());
            if read32(INTFLAG) & ready != 0 {
                Poll::Ready(())
            } else {
                write32(INTENSET, ready);
                Poll::Pending
            }
        })
        .await;
        write32(INTENCLR, ready);
        Self::result(k)
    }
}

impl<'d, M> Adc<'d, M> {
    fn setup(_adc: Peri<'d, peripherals::ADC>, config: Config) -> Result<Self, ConfigError> {
        let gclk = crate::clocks::generator_hz(config.generator).ok_or(ConfigError::Clock)?;
        if gclk > GCLK_MAX_HZ {
            return Err(ConfigError::Clock);
        }
        let cpu = crate::clocks::frequencies().map_or(gclk, |f| f.cpu());
        // TQ = GCLK_ADC (CTLCKDIV 0); TAD = 2 TQ (ADCDIV 1): 50 MHz at most,
        // under the 75 MHz limit.
        let tad = gclk / 2;
        crate::clocks::enable_channel(GCLK_CHANNEL, config.generator);

        write32(CTRLA, SWRST);
        wait(|| read32(SYNCBUSY) & SWRST == 0);
        write32(CTLINTENCLR, u32::MAX);
        write32(INTENCLR, u32::MAX);
        // The factory's trims, before the analog circuits are enabled.
        write32(CALCTRL, read32(FCCFG65));
        write32(CTRLA, ANAEN);
        write32(CTRLD, (reference_code(config.reference) << VREFSEL_SHIFT) | (warm_up_exponent(tad) << WKUP_SHIFT) | CHNEN | ANLEN);
        write32(CORCTRL, (1 << ADCDIV_SHIFT) | (config.resolution.selres() << SELRES_SHIFT) | 1);
        // Single-ended, unsigned, right-aligned, no triggers: each read is
        // software's.
        for register in [CHNCFG1, CHNCFG2, CHNCFG3, CHNCFG4, CHNCFG5, EVCTRL] {
            write32(register, 0);
        }
        write32(CTRLA, ANAEN | ENABLE);
        wait(|| read32(SYNCBUSY) & ENABLE == 0);
        if !wait(|| read32(CTLINTFLAG) & VREFRDY != 0) {
            return Err(ConfigError::Reference);
        }
        if !wait(|| read32(CTLINTFLAG) & CRRDY0 != 0) {
            return Err(ConfigError::Core);
        }
        write_ctrlb(SWCNVEN);
        Ok(Self {
            resolution: config.resolution,
            cpu_per_tad: cpu.div_ceil(tad.max(1)),
            _peripheral: PhantomData,
            _mode: PhantomData,
        })
    }

    /// The result's width.
    #[must_use]
    pub const fn resolution(&self) -> Resolution {
        self.resolution
    }

    /// The bandgap (AIN13 on the PIC32CK, input 14 on the SG00). Sample it
    /// for [`SampleTime::LONG`] (30 µs at least).
    #[must_use]
    pub fn enable_vrefint(&self) -> VrefInt {
        VrefInt
    }

    /// The temperature sensor on AIN12 (SUPC.VREFCTRL.TSEN), on the PIC32CK.
    /// Sample it for
    /// [`SampleTime::LONG`]. It settles for about 100 ms after the first
    /// call, reading low meanwhile.
    #[cfg(pic32ck)]
    #[must_use]
    pub fn enable_temperature(&self) -> Temperature {
        write32(SUPC_VREFCTRL, read32(SUPC_VREFCTRL) | TSEN);
        Temperature
    }

    /// Reads `channel`, sampling it for `sample_time`, spinning until the
    /// result is ready.
    pub fn blocking_read(&mut self, channel: &mut impl AdcChannel, sample_time: SampleTime) -> u16 {
        let k = self.start(channel, sample_time);
        let ready = 1 << (CHRDY_SHIFT + u32::from(k));
        while read32(INTFLAG) & ready == 0 {}
        Self::result(k)
    }

    /// Samples `channel` and starts its conversion: SAMP for the sample
    /// time, then RQCNVRT with SAMP cleared.
    fn start(&mut self, channel: &mut impl AdcChannel, sample_time: SampleTime) -> u8 {
        channel.setup();
        let k = channel.channel();
        write32(INTFLAG, 1 << (CHRDY_SHIFT + u32::from(k)));
        let select = SWCNVEN | u32::from(k);
        write_ctrlb(select | SAMP);
        cortex_m::asm::delay(u32::from(sample_time.0.max(3)) * self.cpu_per_tad);
        write_ctrlb(select | RQCNVRT);
        k
    }

    fn result(k: u8) -> u16 {
        write32(INTFLAG, 1 << (CHRDY_SHIFT + u32::from(k)));
        write32(CORCHDATAID, u32::from(k));
        #[expect(clippy::cast_possible_truncation, reason = "CHRDYDAT's 16 data bits")]
        let data = read32(CHRDYDAT) as u16;
        data
    }
}

impl<M> Drop for Adc<'_, M> {
    fn drop(&mut self) {
        write32(INTENCLR, u32::MAX);
        write32(CTRLA, 0);
        wait(|| read32(SYNCBUSY) & ENABLE == 0);
    }
}

const fn reference_code(reference: Reference) -> u32 {
    match reference {
        Reference::Avdd => 0,
        Reference::External => 1,
    }
}

/// CTRLD.WKUPEXP: the least `n` (4 to 15) with 2^n TAD covering 50 µs.
fn warm_up_exponent(tad_hz: u32) -> u32 {
    let needed = WARM_UP_NANOS * u64::from(tad_hz) / 1_000_000_000;
    (4..15).find(|&n| (1u64 << n) >= needed).unwrap_or(15)
}

fn write_ctrlb(value: u32) {
    write32(CTRLB, value);
    while read32(SYNCBUSY) & SYNC_CTRLB != 0 {}
}

fn wait(mut done: impl FnMut() -> bool) -> bool {
    (0..POLLS).any(|_| done())
}

fn read32(address: usize) -> u32 {
    // SAFETY: an ADC, SUPC or CAL OTP register.
    unsafe { read_volatile(address as *const u32) }
}

fn write32(address: usize, value: u32) {
    // SAFETY: an ADC or SUPC register.
    unsafe { write_volatile(address as *mut u32, value) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warm_up() {
        // 24 MHz: 1,200 TAD, so 2^11.
        assert_eq!(warm_up_exponent(24_000_000), 11);
        assert_eq!(warm_up_exponent(50_000_000), 12);
        assert_eq!(warm_up_exponent(100_000), 4);
    }

    #[test]
    #[cfg(pic32ck)]
    #[expect(clippy::decimal_bitwise_operands, reason = "the data sheet's values, in decimal as it gives them")]
    fn calibration_defaults() {
        // The data sheet's defaults: PTAT 992 at 25.0 °C, 1325 at 125.0 °C,
        // the bandgap 1489 at both, against 3300 mV.
        let fccfg69 = 992 | (1489 << 12) | (25 << 24);
        let fccfg70 = 3300 << 4;
        let fccfg71 = 1325 | (1489 << 12) | (125 << 24);
        let c = Calibration::from_words(fccfg69, fccfg70, fccfg71);
        assert_eq!((c.room_ptat, c.room_bandgap, c.room_decidegrees), (992, 1489, 250));
        assert_eq!((c.hot_ptat, c.hot_bandgap, c.hot_decidegrees, c.reference_mv), (1325, 1489, 1250, 3300));
        assert_eq!(c.celsius(992, 3300), Some(250));
        assert_eq!(c.celsius(1325, 3300), Some(1250));
        // 3.3 V: the bandgap reads as calibrated.
        assert_eq!(c.millivolts(1489), Some(3300));
        // A smaller reference reads the bandgap higher.
        assert_eq!(c.millivolts(1638), Some(2999));
        assert_eq!(c.millivolts(0), None);
    }
}

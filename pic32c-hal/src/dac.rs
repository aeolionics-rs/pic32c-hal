// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The GV00's DAC (the SAM D20's, IP U2214, at `0x4200_4800`; DS60001881
//! chapter 29): one 10-bit output, through its output buffer to VOUT (PA02)
//! and the ADC ([`adc::DacOutput`](crate::adc::DacOutput)), and unbuffered
//! to the analog comparators.
//!
//! [`Dac::new`] turns the buffer on, so the pin and the ADC both have it;
//! [`Dac::new_internal`] only the comparators' path: the ADC then doesn't
//! read the DAC (section 29.6.7.1: it needs the output buffer; seen on the
//! bench, code 512 read 1,376 of 4,096 rather than 2,048). The output is `code / 1024` of the [`Reference`] (the internal
//! 1.0 V, AVDD, or the VREFA pin, PA03), settling within about 2.5 µs of a
//! full-scale step. The DAC is clocked from its generic clock (`GCLK_DAC`,
//! from [`Config::generator`]), which its voltage pump needs below 2.5 V.

use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

use crate::gpio::{Function, set_function};
use crate::{Peri, peripherals};

const DAC: usize = 0x4200_4800;
const CTRLA: usize = DAC;
const CTRLB: usize = DAC + 0x01;
const STATUS: usize = DAC + 0x07;
const DATA: usize = DAC + 0x08;

const SWRST: u8 = 1 << 0;
const ENABLE: u8 = 1 << 1;
/// CTRLB: the output on VOUT (EOEN), and inside the chip (IOEN).
const EOEN: u8 = 1 << 0;
const IOEN: u8 = 1 << 1;
const REFSEL_SHIFT: u8 = 6;
const SYNCBUSY: u8 = 1 << 7;
/// `GCLK_DAC`'s channel (a CLKCTRL.ID).
const GCLK_DAC: usize = 0x1A;
/// PM.APBCMASK: the DAC's bus clock.
const APBC_DAC: u32 = 1 << 18;

/// The largest code: 10 bits.
pub const MAX_CODE: u16 = 1023;

/// The reference (CTRLB.REFSEL).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reference {
    /// The internal 1.0 V reference (from the 1.1 V bandgap).
    Internal1V0,
    /// AVDD.
    #[default]
    Avdd,
    /// The external VREFA pin (PA03).
    External,
}

impl Reference {
    const fn refsel(self) -> u8 {
        match self {
            Self::Internal1V0 => 0,
            Self::Avdd => 1,
            Self::External => 2,
        }
    }
}

/// How to set the DAC up.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub reference: Reference,
    /// The generic clock generator to feed the DAC from.
    pub generator: usize,
}

impl Default for Config {
    /// AVDD, from generator 0.
    fn default() -> Self {
        Self { reference: Reference::default(), generator: 0 }
    }
}

/// Why the DAC can't be set up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ConfigError {
    /// [`Config::generator`] isn't configured.
    NoClock,
}

/// The DAC, enabled while this lives, its output at 0 to start.
pub struct Dac<'d> {
    code: u16,
    _peripheral: PhantomData<&'d mut peripherals::DAC>,
}

impl<'d> Dac<'d> {
    /// The DAC driving `vout` (PA02) through its output buffer, which also
    /// feeds the ADC's [`DacOutput`](crate::adc::DacOutput), and the
    /// comparators.
    ///
    /// # Errors
    ///
    /// If [`Config::generator`] isn't configured.
    #[allow(clippy::needless_pass_by_value, reason = "taking the pin makes it the DAC's")]
    pub fn new(dac: Peri<'d, peripherals::DAC>, vout: Peri<'d, peripherals::PA02>, config: Config) -> Result<Self, ConfigError> {
        let dac = Self::setup(dac, config, EOEN | IOEN)?;
        set_function(&*vout, Function::B);
        Ok(dac)
    }

    /// The DAC driving only the comparators, unbuffered: not the pin, nor
    /// the ADC (which needs the buffer [`Dac::new`] turns on).
    ///
    /// # Errors
    ///
    /// As [`Dac::new`].
    pub fn new_internal(dac: Peri<'d, peripherals::DAC>, config: Config) -> Result<Self, ConfigError> {
        Self::setup(dac, config, IOEN)
    }

    fn setup(_dac: Peri<'d, peripherals::DAC>, config: Config, outputs: u8) -> Result<Self, ConfigError> {
        if crate::clocks::generator_hz(config.generator).is_none() {
            return Err(ConfigError::NoClock);
        }
        crate::clocks::enable_apbc(APBC_DAC);
        crate::clocks::enable_channel(GCLK_DAC, config.generator);
        write8(CTRLA, SWRST);
        while read8(CTRLA) & SWRST != 0 || read8(STATUS) & SYNCBUSY != 0 {}
        // The voltage pump left to the DAC (on below 2.5 V), right-aligned.
        write8(CTRLB, (config.reference.refsel() << REFSEL_SHIFT) | outputs);
        write8(CTRLA, ENABLE);
        sync();
        let mut dac = Self { code: 0, _peripheral: PhantomData };
        dac.set(0);
        Ok(dac)
    }

    /// Sets the output to `code / 1024` of the reference (codes over
    /// [`MAX_CODE`] give it).
    pub fn set(&mut self, code: u16) {
        let code = code.min(MAX_CODE);
        // SAFETY: DATA, a halfword register of the DAC this driver owns.
        unsafe { write_volatile(DATA as *mut u16, code) };
        sync();
        self.code = code;
    }

    /// The code last set.
    #[must_use]
    pub const fn code(&self) -> u16 {
        self.code
    }
}

impl Drop for Dac<'_> {
    fn drop(&mut self) {
        write8(CTRLA, 0);
        sync();
    }
}

fn sync() {
    while read8(STATUS) & SYNCBUSY != 0 {}
}

fn read8(address: usize) -> u8 {
    // SAFETY: a DAC register, read at its width.
    unsafe { read_volatile(address as *const u8) }
}

fn write8(address: usize, value: u8) {
    // SAFETY: a DAC register; only this driver writes them.
    unsafe { write_volatile(address as *mut u8, value) }
}

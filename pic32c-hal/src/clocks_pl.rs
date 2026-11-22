// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The PIC32CM PL10's clock tree (DS40002667, OSCCTRL, GCLK and MCLK): the
//! internal high-frequency oscillator (OSCHF, 1 to 24 MHz) and the four
//! generic clock generators that feed the CPU (generator 0) and the
//! peripherals' channels. The same API as the PIC32CK's `clocks`, smaller.
//!
//! Out of reset OSCHF runs at 4 MHz, on demand, through generator 0;
//! generators 1 to 3 are off. NVMCTRL needs no wait states up to the
//! CPU's 24 MHz. The peripherals' bus clocks on APBC (the SERCOMs, TCs,
//! TCC0, ADC, ...) are off at reset: their drivers turn them on.

use core::cell::Cell;
use core::ptr::{read_volatile, write_volatile};

use critical_section::Mutex;

const OSCCTRL: usize = 0x4000_1000;
const OSCCTRL_STATUS: usize = OSCCTRL + 0x14;
const OSCCTRL_OSCHFCTRL: usize = OSCCTRL + 0x24;

const GCLK: usize = 0x4000_1C00;
const GCLK_SYNCBUSY: usize = GCLK + 0x04;
const GCLK_GENCTRL: usize = GCLK + 0x20;
const GCLK_PCHCTRL: usize = GCLK + 0x80;

const MCLK: usize = 0x4000_0800;
const MCLK_CPUDIV: usize = MCLK + 0x04;
const MCLK_APBCMASK: usize = MCLK + 0x1C;

const OSCHFRDY: u32 = 1 << 4;
const FRQSEL_SHIFT: u32 = 10;
const GENEN: u32 = 1 << 8;
const IDC: u32 = 1 << 9;
const DIV_SHIFT: u32 = 16;
const CHEN: u32 = 1 << 6;

/// The internal 32 kHz oscillator's rate.
pub const OSC32K_HZ: u32 = 32_768;
/// Generic clock generators.
pub const GENERATORS: usize = 4;
/// The CPU's highest rate.
pub const CPU_MAX_HZ: u32 = 24_000_000;

/// OSCHF's rate (OSCHFCTRL.FRQSEL).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Oschf {
    Mhz1,
    Mhz2,
    Mhz3,
    Mhz4,
    Mhz8,
    Mhz12,
    Mhz16,
    Mhz20,
    Mhz24,
}

impl Oschf {
    /// The rate in Hz.
    #[must_use]
    pub const fn hz(self) -> u32 {
        match self {
            Self::Mhz1 => 1_000_000,
            Self::Mhz2 => 2_000_000,
            Self::Mhz3 => 3_000_000,
            Self::Mhz4 => 4_000_000,
            Self::Mhz8 => 8_000_000,
            Self::Mhz12 => 12_000_000,
            Self::Mhz16 => 16_000_000,
            Self::Mhz20 => 20_000_000,
            Self::Mhz24 => 24_000_000,
        }
    }

    const fn frqsel(self) -> u32 {
        match self {
            Self::Mhz1 => 0,
            Self::Mhz2 => 1,
            Self::Mhz3 => 2,
            Self::Mhz4 => 3,
            Self::Mhz8 => 5,
            Self::Mhz12 => 6,
            Self::Mhz16 => 7,
            Self::Mhz20 => 8,
            Self::Mhz24 => 9,
        }
    }
}

/// What a generator divides (GENCTRL.SRC).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Oschf,
    Osc32k,
}

impl Source {
    const fn code(self) -> u32 {
        match self {
            Self::Oschf => 0,
            Self::Osc32k => 3,
        }
    }
}

/// A generic clock generator: a source and a divider (1 to 65535).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Generator {
    pub source: Source,
    pub divide: u16,
}

impl Generator {
    /// `source`, undivided.
    #[must_use]
    pub const fn new(source: Source) -> Self {
        Self { source, divide: 1 }
    }

    /// `source` divided by `divide`.
    #[must_use]
    pub const fn divided(source: Source, divide: u16) -> Self {
        Self { source, divide }
    }
}

/// The clock tree: OSCHF's rate and the generators (generator 0 runs the
/// CPU).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub oschf: Oschf,
    pub generators: [Option<Generator>; GENERATORS],
}

/// Why a [`Config`] can't be brought up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Generator 0 isn't configured: the CPU needs it.
    NoCpuClock,
    /// The CPU would run faster than [`CPU_MAX_HZ`].
    CpuTooFast(u32),
    /// A generator divides by 0.
    Divider(usize),
    /// OSCHF didn't report ready.
    OschfTimeout,
    /// Generator `n` didn't synchronise.
    GeneratorTimeout(usize),
}

/// The rates a [`Config`] gives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frequencies {
    pub oschf: u32,
    pub generators: [Option<u32>; GENERATORS],
}

impl Frequencies {
    /// The CPU's rate (generator 0's).
    #[must_use]
    pub const fn cpu(&self) -> u32 {
        match self.generators[0] {
            Some(hz) => hz,
            None => 4_000_000,
        }
    }
}

impl Config {
    /// The clocks as they come out of reset: OSCHF at 4 MHz on generator 0.
    #[must_use]
    pub const fn boot() -> Self {
        Self {
            oschf: Oschf::Mhz4,
            generators: [Some(Generator::new(Source::Oschf)), None, None, None],
        }
    }

    /// OSCHF at 24 MHz, undivided, for the CPU on generator 0.
    #[must_use]
    pub const fn oschf_24mhz() -> Self {
        Self {
            oschf: Oschf::Mhz24,
            generators: [Some(Generator::new(Source::Oschf)), None, None, None],
        }
    }

    /// The rates this configuration gives, checked.
    ///
    /// # Errors
    ///
    /// See [`Error`].
    pub fn frequencies(&self) -> Result<Frequencies, Error> {
        let mut generators = [None; GENERATORS];
        for (n, generator) in self.generators.iter().enumerate() {
            if let Some(g) = generator {
                if g.divide == 0 {
                    return Err(Error::Divider(n));
                }
                let source = match g.source {
                    Source::Oschf => self.oschf.hz(),
                    Source::Osc32k => OSC32K_HZ,
                };
                generators[n] = Some(source / u32::from(g.divide));
            }
        }
        let Some(cpu) = generators[0] else {
            return Err(Error::NoCpuClock);
        };
        if cpu > CPU_MAX_HZ {
            return Err(Error::CpuTooFast(cpu));
        }
        Ok(Frequencies { oschf: self.oschf.hz(), generators })
    }
}

impl Default for Config {
    /// [`Config::boot`].
    fn default() -> Self {
        Self::boot()
    }
}

static FREQUENCIES: Mutex<Cell<Option<Frequencies>>> = Mutex::new(Cell::new(None));

/// The rates [`crate::init`] set up, or `None` before it.
#[must_use]
pub fn frequencies() -> Option<Frequencies> {
    critical_section::with(|cs| FREQUENCIES.borrow(cs).get())
}

/// A generator's rate, once [`crate::init`] has set the clocks up.
#[must_use]
pub fn generator_hz(generator: usize) -> Option<u32> {
    frequencies().and_then(|f| f.generators.get(generator).copied().flatten())
}

fn read(address: usize) -> u32 {
    // SAFETY: a clock controller register, read whole.
    unsafe { read_volatile(address as *const u32) }
}

fn write(address: usize, value: u32) {
    // SAFETY: a clock controller register; this module and the drivers'
    // own channels are its only writers.
    unsafe { write_volatile(address as *mut u32, value) }
}

fn wait(polls: u32, mut done: impl FnMut() -> bool) -> bool {
    (0..polls).any(|_| done())
}

/// Sets generator `n` in one write and waits for it to synchronise.
fn set_generator(n: usize, generator: Generator) -> Result<(), Error> {
    let divide = if generator.divide == 1 { 0 } else { u32::from(generator.divide) };
    let idc = if divide % 2 == 1 { IDC } else { 0 };
    write(GCLK_GENCTRL + 4 * n, (divide << DIV_SHIFT) | idc | generator.source.code() | GENEN);
    if wait(1_000_000, || read(GCLK_SYNCBUSY) & (1 << (2 + n)) == 0) { Ok(()) } else { Err(Error::GeneratorTimeout(n)) }
}

/// Feeds peripheral channel `channel` from generator `generator`, and waits
/// for the channel to report enabled.
pub fn enable_channel(channel: usize, generator: usize) {
    let address = GCLK_PCHCTRL + 4 * channel;
    #[expect(clippy::cast_possible_truncation, reason = "generators number under 16")]
    write(address, generator as u32 | CHEN);
    wait(1_000_000, || read(address) & CHEN != 0);
}

/// Turns on the APBC bus clocks in `mask` (MCLK.APBCMASK), which the
/// peripherals on APBC need and which are off at reset.
pub(crate) fn enable_apbc(mask: u32) {
    critical_section::with(|_| write(MCLK_APBCMASK, read(MCLK_APBCMASK) | mask));
}

/// Brings `config` up.
///
/// # Errors
///
/// [`Config::frequencies`]'s, before anything changes; or the step that
/// didn't complete.
///
/// # Safety
///
/// Once, from [`crate::init`], before any driver uses a generator.
pub(crate) unsafe fn init(config: &Config) -> Result<Frequencies, Error> {
    let frequencies = config.frequencies()?;
    // OSCHF at its rate, always on (not on demand), no auto-tuning.
    write(OSCCTRL_OSCHFCTRL, config.oschf.frqsel() << FRQSEL_SHIFT);
    if !wait(1_000_000, || read(OSCCTRL_STATUS) & OSCHFRDY != 0) {
        return Err(Error::OschfTimeout);
    }
    for (n, generator) in config.generators.iter().enumerate().skip(1) {
        if let Some(generator) = *generator {
            set_generator(n, generator)?;
        }
    }
    if let Some(generator) = config.generators[0] {
        set_generator(0, generator)?;
    }
    // The CPU undivided from generator 0.
    // SAFETY: MCLK.CPUDIV, a byte register.
    unsafe { write_volatile(MCLK_CPUDIV as *mut u8, 1) };
    critical_section::with(|cs| FREQUENCIES.borrow(cs).set(Some(frequencies)));
    Ok(frequencies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rates() {
        let f = Config::oschf_24mhz().frequencies().unwrap();
        assert_eq!((f.cpu(), f.generators[1]), (24_000_000, None));
        let mut c = Config::oschf_24mhz();
        c.generators[1] = Some(Generator::divided(Source::Oschf, 24));
        assert_eq!(c.frequencies().unwrap().generators[1], Some(1_000_000));
        c.generators[0] = None;
        assert_eq!(c.frequencies(), Err(Error::NoCpuClock));
    }
}

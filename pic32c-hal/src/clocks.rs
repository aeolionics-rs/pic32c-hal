// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The clock tree (DS60001795 chapters 19, OSCCTRL, and 20, GCLK): the
//! crystal oscillator (XOSC), PLL0, and the twelve generic clock generators
//! that feed the CPU (generator 0) and the peripherals' channels.
//!
//! [`crate::init`] applies a [`Config`], checked first by
//! [`Config::frequencies`], then records the [`Frequencies`] for drivers
//! ([`frequencies`]). It sets the flash wait states for the CPU's rate
//! itself (one per 20 MHz, with the address wait state over 60 MHz;
//! table 52-44).
//!
//! Out of reset the CPU runs from the DFLL48M, open loop (about 48 MHz, not
//! trimmed to anything: use the crystal for anything timed), through
//! generator 0; XOSC and PLL0 are off and generators 1 to 11 disabled.
//! OSCCTRL, GCLK, MCLK and FCR have no Secure alias on the SG parts.
//!
//! The high-speed USB PHY's PLL takes the XOSC directly, not a generator:
//! with a 12 or 24 MHz XOSC, `init` also passes it on to the PHY
//! (XOSCCTRLA.USBHSDIV 1, which only changes while the XOSC is off), and
//! [`usbhs_reference_hz`] says so.

use core::cell::Cell;
use core::ptr::{read_volatile, write_volatile};

use critical_section::Mutex;

const OSCCTRL: usize = 0x4400_C000;
const OSCCTRL_INTFLAG: usize = OSCCTRL + 0x0C;
const OSCCTRL_STATUS: usize = OSCCTRL + 0x10;
const OSCCTRL_XOSCCTRLA: usize = OSCCTRL + 0x14;
const OSCCTRL_XOSCCTRLB: usize = OSCCTRL + 0x18;
const OSCCTRL_PLL0CTRL: usize = OSCCTRL + 0x40;
const OSCCTRL_PLL0FBDIV: usize = OSCCTRL + 0x44;
const OSCCTRL_PLL0REFDIV: usize = OSCCTRL + 0x48;
const OSCCTRL_PLL0POSTDIVA: usize = OSCCTRL + 0x4C;
const OSCCTRL_PLL0POSTDIVB: usize = OSCCTRL + 0x50;

const GCLK: usize = 0x4401_0000;
const GCLK_SYNCBUSY: usize = GCLK + 0x04;
const GCLK_GENCTRL: usize = GCLK + 0x20;
const GCLK_PCHCTRL: usize = GCLK + 0x80;

const FCR_CTRLA: usize = 0x4400_2000;

const XOSC_ENABLE: u32 = 1 << 1;
const XOSC_AGC: u32 = 1 << 2;
const XOSC_XTALEN: u32 = 1 << 3;
/// STARTUP 8: about 7.8 ms for the crystal to settle.
const XOSC_STARTUP: u32 = 0x8 << 8;
/// XOSCCTRLB.GRES: the automatic gain control's reserve.
const XOSC_GRES: u32 = 1 << 2;
const XOSC_READY: u32 = 1 << 0;
/// XOSCCTRLA.USBHSDIV 1: the XOSC, undivided, to the USBHS PHY's PLL.
const XOSC_USBHSDIV1: u32 = 1 << 24;
const PLL_ENABLE: u32 = 1 << 1;
const PLL_REFSEL_SHIFT: u32 = 8;
const PLL_BWSEL_SHIFT: u32 = 11;
const PLL_LOCK: u32 = 1 << 24;
const PLL_OUTEN: u32 = 1 << 7;
const GENEN: u32 = 1 << 8;
const IDC: u32 = 1 << 9;
const DIV_SHIFT: u32 = 16;
const CHEN: u32 = 1 << 6;
const FCR_ADRWS: u32 = 1 << 14;
const FCR_FWS_SHIFT: u32 = 8;

/// The DFLL48M's nominal rate, open loop.
pub const DFLL48M_HZ: u32 = 48_000_000;
/// The ultra-low-power oscillator's rate.
pub const OSCULP32K_HZ: u32 = 32_768;
/// The generic clock generators.
pub const GENERATORS: usize = 12;
/// The CPU's highest rate (at -40 to +85 °C; table 52-44).
pub const CPU_MAX_HZ: u32 = 120_000_000;

/// The crystal oscillator's input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Xosc {
    /// A crystal on XIN/XOUT (PD00/PD01), at this rate, with automatic gain
    /// control.
    Crystal(u32),
    /// An external clock on XIN, at this rate.
    External(u32),
}

impl Xosc {
    const fn hz(self) -> u32 {
        match self {
            Self::Crystal(hz) | Self::External(hz) => hz,
        }
    }
}

/// PLL0's reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PllReference {
    Xosc,
    Dfll48m,
}

/// PLL0: reference / `refdiv` to the phase detector (4 to 48 MHz), times
/// `fbdiv` to the VCO (800 to 1,600 MHz), divided by each output's divider
/// (1 to 63). `outputs[i]` is output `i + 1`'s divider, or `None` for off;
/// output 0, the fractional divider's, isn't offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pll0 {
    pub reference: PllReference,
    pub refdiv: u8,
    pub fbdiv: u16,
    pub outputs: [Option<u8>; 5],
}

/// A generator's source (table 20-8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Xosc,
    Osculp32k,
    Dfll48m,
    /// PLL0's output 1 to 5.
    Pll0(u8),
}

impl Source {
    const fn code(self) -> u32 {
        match self {
            Self::Xosc => 0x00,
            Self::Osculp32k => 0x03,
            Self::Dfll48m => 0x05,
            Self::Pll0(output) => 0x06 + output as u32,
        }
    }
}

/// A generator: `source` divided by `divide` (1 to 65,535).
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

/// The clock tree to set up. Generator 0 clocks the CPU and must be given;
/// a generator left `None` is left as it is (off, out of reset).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub xosc: Option<Xosc>,
    pub pll0: Option<Pll0>,
    pub generators: [Option<Generator>; GENERATORS],
}

/// What went wrong with a [`Config`], or bringing it up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The XOSC's rate is outside 4 to 48 MHz.
    XoscRange,
    /// PLL0's reference isn't configured, or REFDIV, FBDIV or an output's
    /// divider is outside its range.
    PllDivider,
    /// The phase detector's input is outside 4 to 48 MHz.
    PllReferenceRange,
    /// The VCO is outside 800 to 1,600 MHz.
    PllVcoRange,
    /// A generator's source isn't configured, or its divider is 0.
    GeneratorSource(usize),
    /// Generator 0 isn't given.
    NoCpuClock,
    /// The CPU would run over [`CPU_MAX_HZ`].
    CpuTooFast(u32),
    /// The crystal oscillator never reported ready.
    XoscTimeout,
    /// PLL0 never locked.
    PllTimeout,
    /// A generator never synchronised.
    GeneratorTimeout(usize),
}

/// The rates a [`Config`] gives, in Hz.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frequencies {
    pub xosc: Option<u32>,
    pub pll0_vco: Option<u32>,
    /// PLL0's outputs 1 to 5.
    pub pll0: [Option<u32>; 5],
    pub generators: [Option<u32>; GENERATORS],
}

impl Frequencies {
    /// The CPU's rate (generator 0's).
    #[must_use]
    pub const fn cpu(&self) -> u32 {
        match self.generators[0] {
            Some(hz) => hz,
            None => DFLL48M_HZ,
        }
    }
}

impl Config {
    /// The clocks as they come out of reset: generator 0 from the
    /// DFLL48M, nothing else.
    #[must_use]
    pub const fn boot() -> Self {
        let mut generators = [None; GENERATORS];
        generators[0] = Some(Generator::new(Source::Dfll48m));
        Self { xosc: None, pll0: None, generators }
    }

    /// `xosc` (an 8, 12, 16, 20, 24, 40 or 48 MHz crystal or clock: one
    /// whose multiple is 960 MHz; over 45 MHz it's halved first, to keep
    /// FBDIV at 21 or more) through PLL0 to a 960 MHz VCO, and:
    /// - generator 0, the CPU: 120 MHz (output 2, / 8);
    /// - generator 2: 80 MHz (output 3, / 12), for CAN;
    /// - generator 3: 48 MHz (output 4, / 20), for USB and the SERCOMs;
    /// - generator 4: 16 MHz (generator 3's source / 3), for the time
    ///   driver.
    ///
    /// # Panics
    ///
    /// If `xosc`'s rate doesn't divide 960 MHz, or is outside 4 to 48 MHz
    /// (at compile time, in a `const`).
    #[must_use]
    pub const fn pll_from_xosc(xosc: Xosc) -> Self {
        const VCO: u32 = 960_000_000;
        let hz = xosc.hz();
        assert!(hz >= 4_000_000 && hz <= 48_000_000 && VCO.is_multiple_of(hz), "the XOSC's rate must divide 960 MHz");
        let refdiv: u8 = if hz > 45_000_000 { 2 } else { 1 };
        #[expect(clippy::cast_possible_truncation, reason = "at most 240, from the assert")]
        let fbdiv = (VCO / hz * refdiv as u32) as u16;
        let mut generators = [None; GENERATORS];
        generators[0] = Some(Generator::new(Source::Pll0(2)));
        generators[2] = Some(Generator::new(Source::Pll0(3)));
        generators[3] = Some(Generator::new(Source::Pll0(4)));
        generators[4] = Some(Generator::divided(Source::Pll0(4), 3));
        Self {
            xosc: Some(xosc),
            pll0: Some(Pll0 { reference: PllReference::Xosc, refdiv, fbdiv, outputs: [None, Some(8), Some(12), Some(20), None] }),
            generators,
        }
    }

    /// The rates this configuration gives, after checking it.
    ///
    /// # Errors
    ///
    /// The first rule it breaks.
    pub fn frequencies(&self) -> Result<Frequencies, Error> {
        let xosc = match self.xosc {
            Some(xosc) if !(4_000_000..=48_000_000).contains(&xosc.hz()) => return Err(Error::XoscRange),
            xosc => xosc.map(Xosc::hz),
        };
        let mut pll0 = [None; 5];
        let mut pll0_vco = None;
        if let Some(pll) = self.pll0 {
            let reference = match pll.reference {
                PllReference::Xosc => xosc.ok_or(Error::PllDivider)?,
                PllReference::Dfll48m => DFLL48M_HZ,
            };
            if !(1..=63).contains(&pll.refdiv) || !(21..=1023).contains(&pll.fbdiv) {
                return Err(Error::PllDivider);
            }
            let pfd = reference / u32::from(pll.refdiv);
            if !(4_000_000..=48_000_000).contains(&pfd) {
                return Err(Error::PllReferenceRange);
            }
            let vco = u64::from(reference) * u64::from(pll.fbdiv) / u64::from(pll.refdiv);
            if !(800_000_000..=1_600_000_000).contains(&vco) {
                return Err(Error::PllVcoRange);
            }
            let vco = u32::try_from(vco).map_err(|_| Error::PllVcoRange)?;
            for (rate, divider) in pll0.iter_mut().zip(pll.outputs) {
                if let Some(divider) = divider {
                    if !(1..=63).contains(&divider) {
                        return Err(Error::PllDivider);
                    }
                    *rate = Some(vco / u32::from(divider));
                }
            }
            pll0_vco = Some(vco);
        }
        let mut generators = [None; GENERATORS];
        for (n, (rate, generator)) in generators.iter_mut().zip(self.generators).enumerate() {
            let Some(generator) = generator else { continue };
            let source = match generator.source {
                Source::Xosc => xosc,
                Source::Osculp32k => Some(OSCULP32K_HZ),
                Source::Dfll48m => Some(DFLL48M_HZ),
                Source::Pll0(output @ 1..=5) => pll0[usize::from(output) - 1],
                Source::Pll0(_) => None,
            };
            let source = source.ok_or(Error::GeneratorSource(n))?;
            if generator.divide == 0 {
                return Err(Error::GeneratorSource(n));
            }
            *rate = Some(source / u32::from(generator.divide));
        }
        let cpu = generators[0].ok_or(Error::NoCpuClock)?;
        if cpu > CPU_MAX_HZ {
            return Err(Error::CpuTooFast(cpu));
        }
        Ok(Frequencies { xosc, pll0_vco, pll0, generators })
    }
}

impl Default for Config {
    /// [`Config::boot`].
    fn default() -> Self {
        Self::boot()
    }
}

/// Flash wait states (FCR.CTRLA, AUTOWS off) for a CPU rate: one per 20 MHz
/// over the first, and the address wait state with three or more.
fn flash_wait_states(cpu_hz: u32) -> u32 {
    let fws = cpu_hz.saturating_sub(1) / 20_000_000;
    (fws << FCR_FWS_SHIFT) | if fws >= 3 { FCR_ADRWS } else { 0 }
}

static FREQUENCIES: Mutex<Cell<Option<Frequencies>>> = Mutex::new(Cell::new(None));

/// The rates [`crate::init`] set up, or `None` before it.
#[must_use]
pub fn frequencies() -> Option<Frequencies> {
    critical_section::with(|cs| FREQUENCIES.borrow(cs).get())
}

/// Whether an XOSC of `hz` can be the USBHS PHY's reference: it takes
/// 12 or 24 MHz (DS60001795 37.2, note 2; CTRLA.REFCLKSEL).
const fn usbhs_reference(hz: u32) -> bool {
    hz == 12_000_000 || hz == 24_000_000
}

/// The rate [`crate::init`] passes the USBHS PHY's PLL from the XOSC (12 or
/// 24 MHz), or `None` if the XOSC is off or at another rate.
#[must_use]
pub fn usbhs_reference_hz() -> Option<u32> {
    frequencies().and_then(|f| f.xosc).filter(|&hz| usbhs_reference(hz))
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

/// Polls `done` up to `polls` times.
fn wait(polls: u32, mut done: impl FnMut() -> bool) -> bool {
    for _ in 0..polls {
        if done() {
            return true;
        }
        core::hint::spin_loop();
    }
    false
}

/// Sets generator `n` in one write, as the datasheet requires, and waits
/// for it to synchronise.
fn set_generator(n: usize, generator: Generator) -> Result<(), Error> {
    // DIV 1 and 0 both mean undivided; IDC must be set for an odd division
    // and clear for an even one.
    let divide = if generator.divide == 1 { 0 } else { u32::from(generator.divide) };
    let idc = if divide % 2 == 1 { IDC } else { 0 };
    write(GCLK_GENCTRL + 4 * n, (divide << DIV_SHIFT) | idc | generator.source.code() | GENEN);
    if wait(1_000_000, || read(GCLK_SYNCBUSY) & (1 << (2 + n)) == 0) { Ok(()) } else { Err(Error::GeneratorTimeout(n)) }
}

/// Feeds peripheral channel `channel` (table 20-2) from generator
/// `generator`, and waits for the channel to report enabled. The HAL's
/// drivers do this for their own peripherals; this is for the others.
pub fn enable_channel(channel: usize, generator: usize) {
    let address = GCLK_PCHCTRL + 4 * channel;
    #[expect(clippy::cast_possible_truncation, reason = "generators number under 16")]
    write(address, generator as u32 | CHEN);
    wait(1_000_000, || read(address) & CHEN != 0);
}

/// Brings `config` up, from whatever the clocks are now.
///
/// # Errors
///
/// [`Config::frequencies`]'s, before anything changes; or the step that
/// didn't complete, with the CPU still on the DFLL48M.
///
/// # Safety
///
/// Once, from [`crate::init`], before any driver uses a generator.
pub(crate) unsafe fn init(config: &Config) -> Result<Frequencies, Error> {
    let frequencies = config.frequencies()?;
    let dfll = Generator::new(Source::Dfll48m);

    // Back to the DFLL48M, at its wait states, if the CPU is elsewhere:
    // the XOSC's and PLL0's settings can't change while they run it.
    if read(GCLK_GENCTRL) & 0x1F != Source::Dfll48m.code() {
        write(FCR_CTRLA, flash_wait_states(CPU_MAX_HZ));
        set_generator(0, dfll)?;
    }
    write(FCR_CTRLA, flash_wait_states(DFLL48M_HZ));

    if let Some(xosc) = config.xosc {
        let control = match xosc {
            Xosc::Crystal(_) => {
                write(OSCCTRL_XOSCCTRLB, XOSC_GRES);
                XOSC_STARTUP | XOSC_XTALEN | XOSC_AGC
            }
            Xosc::External(_) => XOSC_STARTUP,
        };
        let control = control | if usbhs_reference(xosc.hz()) { XOSC_USBHSDIV1 } else { 0 };
        write(OSCCTRL_XOSCCTRLA, 0);
        write(OSCCTRL_XOSCCTRLA, control);
        write(OSCCTRL_XOSCCTRLA, control | XOSC_ENABLE);
        if !wait(50_000_000, || read(OSCCTRL_STATUS) & XOSC_READY != 0) {
            return Err(Error::XoscTimeout);
        }
    }

    if let Some(pll) = config.pll0 {
        let refsel = match pll.reference {
            PllReference::Xosc => 1,
            PllReference::Dfll48m => 2,
        };
        // Table 19-17, by the phase detector's input.
        let reference = match pll.reference {
            PllReference::Xosc => frequencies.xosc.unwrap_or(0),
            PllReference::Dfll48m => DFLL48M_HZ,
        };
        let bwsel = match reference / u32::from(pll.refdiv) {
            0..10_000_000 => 1,
            10_000_000..20_000_000 => 2,
            20_000_000..30_000_000 => 3,
            _ => 4,
        };
        let control = (refsel << PLL_REFSEL_SHIFT) | (bwsel << PLL_BWSEL_SHIFT);
        write(OSCCTRL_PLL0CTRL, 0);
        write(OSCCTRL_PLL0CTRL, control);
        write(OSCCTRL_PLL0REFDIV, u32::from(pll.refdiv));
        write(OSCCTRL_PLL0FBDIV, u32::from(pll.fbdiv));
        // One byte per output: the divider in bits 5:0, OUTEN in bit 7.
        // Off outputs keep a legal divider.
        let byte = |output: Option<u8>| output.map_or(0x20, |divider| u32::from(divider) | PLL_OUTEN);
        let [o1, o2, o3, o4, o5] = pll.outputs;
        write(OSCCTRL_PLL0POSTDIVA, 0x20 | (byte(o1) << 8) | (byte(o2) << 16) | (byte(o3) << 24));
        write(OSCCTRL_PLL0POSTDIVB, byte(o4) | (byte(o5) << 8));
        write(OSCCTRL_INTFLAG, PLL_LOCK);
        write(OSCCTRL_PLL0CTRL, control | PLL_ENABLE);
        if !wait(10_000_000, || read(OSCCTRL_INTFLAG) & PLL_LOCK != 0) {
            return Err(Error::PllTimeout);
        }
    }

    for (n, generator) in config.generators.iter().enumerate().skip(1) {
        if let Some(generator) = *generator {
            set_generator(n, generator)?;
        }
    }
    // The CPU last, with the wait states for the faster of the two rates
    // while it changes.
    let cpu = frequencies.cpu();
    write(FCR_CTRLA, flash_wait_states(cpu.max(DFLL48M_HZ)));
    if let Some(generator) = config.generators[0] {
        set_generator(0, generator)?;
    }
    write(FCR_CTRLA, flash_wait_states(cpu));
    let _ = read(FCR_CTRLA);
    critical_section::with(|cs| FREQUENCIES.borrow(cs).set(Some(frequencies)));
    Ok(frequencies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_usbhs_phy_takes_12_or_24_mhz() {
        assert!(usbhs_reference(12_000_000));
        assert!(usbhs_reference(24_000_000));
        assert!(!usbhs_reference(16_000_000));
        assert!(!usbhs_reference(48_000_000));
    }

    #[test]
    fn the_crystal_preset() {
        let f = Config::pll_from_xosc(Xosc::Crystal(12_000_000)).frequencies().unwrap();
        assert_eq!(f.pll0_vco, Some(960_000_000));
        assert_eq!(f.generators[0], Some(120_000_000));
        assert_eq!(f.generators[2], Some(80_000_000));
        assert_eq!(f.generators[3], Some(48_000_000));
        assert_eq!(f.generators[4], Some(16_000_000));
        assert_eq!(f.generators[1], None);
        for hz in [8_000_000, 16_000_000, 24_000_000, 48_000_000] {
            assert_eq!(Config::pll_from_xosc(Xosc::External(hz)).frequencies().unwrap().pll0_vco, Some(960_000_000));
        }
    }

    #[test]
    fn boot() {
        let f = Config::boot().frequencies().unwrap();
        assert_eq!(f.cpu(), DFLL48M_HZ);
        assert_eq!(f.generators[1..], [None; GENERATORS - 1]);
    }

    #[test]
    fn checks() {
        let mut c = Config::pll_from_xosc(Xosc::Crystal(12_000_000));
        c.generators[0] = Some(Generator::new(Source::Pll0(1)));
        assert_eq!(c.frequencies(), Err(Error::GeneratorSource(0)));
        c.generators[0] = Some(Generator::divided(Source::Pll0(2), 0));
        assert_eq!(c.frequencies(), Err(Error::GeneratorSource(0)));
        c.generators[0] = None;
        assert_eq!(c.frequencies(), Err(Error::NoCpuClock));

        let mut c = Config::pll_from_xosc(Xosc::Crystal(12_000_000));
        c.pll0.as_mut().unwrap().outputs[1] = Some(7);
        assert_eq!(c.frequencies(), Err(Error::CpuTooFast(137_142_857)));
        c.pll0.as_mut().unwrap().fbdiv = 140;
        assert_eq!(c.frequencies(), Err(Error::PllVcoRange));
        c.pll0.as_mut().unwrap().refdiv = 4;
        assert_eq!(c.frequencies(), Err(Error::PllReferenceRange));

        let mut c = Config::boot();
        c.xosc = Some(Xosc::Crystal(50_000_000));
        assert_eq!(c.frequencies(), Err(Error::XoscRange));
        c.xosc = None;
        c.generators[1] = Some(Generator::new(Source::Xosc));
        assert_eq!(c.frequencies(), Err(Error::GeneratorSource(1)));
    }

    #[test]
    fn wait_states() {
        assert_eq!(flash_wait_states(20_000_000), 0);
        assert_eq!(flash_wait_states(48_000_000), 2 << FCR_FWS_SHIFT);
        assert_eq!(flash_wait_states(60_000_000), 2 << FCR_FWS_SHIFT);
        assert_eq!(flash_wait_states(80_000_000), (3 << FCR_FWS_SHIFT) | FCR_ADRWS);
        assert_eq!(flash_wait_states(120_000_000), (5 << FCR_FWS_SHIFT) | FCR_ADRWS);
    }
}

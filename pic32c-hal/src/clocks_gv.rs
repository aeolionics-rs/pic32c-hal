// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The PIC32CM GV00's clock tree (DS60001881, SYSCTRL, GCLK and PM): the
//! internal 8 MHz oscillator (OSC8M), the 32.768 kHz crystal oscillator
//! (XOSC32K), the DFLL48M, and the eight generic clock generators that feed
//! the CPU (generator 0) and the peripherals' channels. The same API as the
//! other chips' `clocks` where the IP allows; its GCLK is the older kind,
//! written through ID fields (GENDIV, GENCTRL, CLKCTRL).
//!
//! Out of reset OSC8M runs divided by 8 (1 MHz) into generator 0;
//! generator 2 runs from OSCULP32K for the watchdog; the others are off.
//! The watchdog's periods count 1.024 kHz cycles, so [`Config::osc8m`] and
//! [`Config::dfll48m_from_xosc32k`] set generator 2 to OSCULP32K / 32
//! ([`WATCHDOG_GENERATOR`]); [`Config::boot`] leaves it at OSCULP32K's
//! 32 kHz.
//! Flash needs one wait state above 24 MHz at 2.7 V to 3.63 V (three at
//! 48 MHz below 2.7 V: [`Config::low_voltage`]). [`init`](crate::init) also
//! sets NVMCTRL.CTRLB.MANW, as the data sheet asks, so only a command writes
//! a flash page (the [`flash`](crate::flash) driver's way). The peripherals' bus clocks
//! on APBC (the SERCOMs, TCs, ADC, ...) are off at reset: their drivers turn
//! them on.
//!
//! The DFLL48M runs open loop from its factory calibration (DFLLVAL.COARSE
//! from the NVM software calibration area; within a few percent), or closed
//! loop locked to a generator, normally one running from XOSC32K.

use core::cell::Cell;
use core::ptr::{read_volatile, write_volatile};

use critical_section::Mutex;

const SYSCTRL: usize = 0x4000_0800;
const SYSCTRL_PCLKSR: usize = SYSCTRL + 0x0C;
const SYSCTRL_XOSC32K: usize = SYSCTRL + 0x14;
const SYSCTRL_OSC32K: usize = SYSCTRL + 0x18;
const SYSCTRL_OSC8M: usize = SYSCTRL + 0x20;
const SYSCTRL_DFLLCTRL: usize = SYSCTRL + 0x24;
const SYSCTRL_DFLLVAL: usize = SYSCTRL + 0x28;
const SYSCTRL_DFLLMUL: usize = SYSCTRL + 0x2C;

const GCLK: usize = 0x4000_0C00;
const GCLK_STATUS: usize = GCLK + 0x01;
const GCLK_CLKCTRL: usize = GCLK + 0x02;
const GCLK_GENCTRL: usize = GCLK + 0x04;
const GCLK_GENDIV: usize = GCLK + 0x08;

const PM: usize = 0x4000_0400;
const PM_CPUSEL: usize = PM + 0x08;
const PM_APBCMASK: usize = PM + 0x20;

const NVMCTRL_CTRLB: usize = 0x4100_4004;
/// CTRLB.MANW: page writes only by command. The data sheet gives it a
/// reset value of 1 and says to set it at start-up; it reads 0 out of reset
/// on the bench board, which would have a store to the last word of a page
/// buffer write the page by itself.
const NVMCTRL_MANW: u32 = 1 << 7;
/// The NVM software calibration area's first 64 bits (table 7-7).
const NVM_CALIBRATION: usize = 0x0080_6020;

// PCLKSR.
const XOSC32KRDY: u32 = 1 << 1;
const OSC32KRDY: u32 = 1 << 2;
const DFLLRDY: u32 = 1 << 4;
const DFLLLCKF: u32 = 1 << 6;
const DFLLLCKC: u32 = 1 << 7;

// XOSC32K and OSC32K.
const OSC_ENABLE: u32 = 1 << 1;
const XOSC32K_XTALEN: u32 = 1 << 2;
const XOSC32K_EN32K: u32 = 1 << 3;
const OSC32K_EN32K: u32 = 1 << 2;
/// STARTUP 4: 16,384 OSCULP32K cycles, about 0.5 s, for the crystal.
const XOSC32K_STARTUP: u32 = 4 << 8;
/// STARTUP 2 for OSC32K: 5 cycles, about 153 µs.
const OSC32K_STARTUP: u32 = 2 << 8;
const OSC32K_CALIB_SHIFT: u32 = 16;

// OSC8M.
const OSC8M_PRESC_SHIFT: u32 = 8;
const OSC8M_PRESC_MASK: u32 = 0x3 << OSC8M_PRESC_SHIFT;

// DFLLCTRL, DFLLVAL, DFLLMUL.
const DFLL_ENABLE: u32 = 1 << 1;
const DFLL_MODE: u32 = 1 << 2;
const DFLLVAL_COARSE_SHIFT: u32 = 10;
/// DFLLMUL's maximum steps while it locks: a quarter of COARSE's range and
/// a sixteenth of FINE's, well under the 50 % the data sheet allows.
const DFLLMUL_STEPS: u32 = (15 << 26) | (63 << 16);
/// DFLLVAL.FINE's middle, for the open loop.
const DFLL_FINE_MIDDLE: u32 = 512;

// GCLK.
const SYNCBUSY: u8 = 1 << 7;
const GENEN: u32 = 1 << 16;
const IDC: u32 = 1 << 17;
/// GENCTRL.DIVSEL: divide by 2^(DIV + 1) rather than by DIV.
const DIVSEL: u32 = 1 << 20;
const CLKEN: u16 = 1 << 14;
/// CLKCTRL.ID of the DFLL48M's reference (`GCLK_DFLL48M_REF`).
const DFLL48M_REF: usize = 0x00;

/// OSC8M's rate, undivided.
pub const OSC8M_HZ: u32 = 8_000_000;
/// The 32 kHz oscillators' rate.
pub const OSC32K_HZ: u32 = 32_768;
/// The DFLL48M's open-loop rate, nominally.
pub const DFLL48M_HZ: u32 = 48_000_000;
/// The DFLL48M's highest rate.
pub const DFLL48M_MAX_HZ: u32 = 48_000_000;
/// Generic clock generators.
pub const GENERATORS: usize = 8;
/// The CPU's highest rate.
pub const CPU_MAX_HZ: u32 = 48_000_000;

/// Each generator's largest integer division (GENDIV.DIV's width, from the
/// GENDIV register's table: 8 bits, 16 on generator 1, 5 on generator 2).
/// Powers of two beyond these go by DIVSEL.
const MAX_DIVIDE: [u32; GENERATORS] = [255, 65_535, 31, 255, 255, 255, 255, 255];

/// The generator the watchdog runs from (its clock out of reset), which it
/// needs at 1.024 kHz: OSCULP32K divided by 32.
pub const WATCHDOG_GENERATOR: usize = 2;
/// The watchdog generator's rate.
pub const WATCHDOG_HZ: u32 = 1024;

/// OSC8M's prescaler (OSC8M.PRESC).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Osc8mDivide {
    Div1,
    Div2,
    Div4,
    /// The reset value: 1 MHz.
    Div8,
}

impl Osc8mDivide {
    const fn presc(self) -> u32 {
        match self {
            Self::Div1 => 0,
            Self::Div2 => 1,
            Self::Div4 => 2,
            Self::Div8 => 3,
        }
    }

    /// OSC8M's rate through this prescaler.
    #[must_use]
    pub const fn hz(self) -> u32 {
        OSC8M_HZ >> self.presc()
    }
}

/// What a generator divides (GENCTRL.SRC).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// OSC8M through its prescaler.
    Osc8m,
    Dfll48m,
    /// The 32.768 kHz crystal, if [`Config::xosc32k`].
    Xosc32k,
    /// The internal 32 kHz oscillator, if [`Config::osc32k`].
    Osc32k,
    /// The ultra-low-power 32 kHz oscillator, always on.
    OscUlp32k,
    /// Generator 1's output (not for generator 1 itself).
    Generator1,
}

impl Source {
    const fn code(self) -> u32 {
        match self {
            Self::Generator1 => 0x02,
            Self::OscUlp32k => 0x03,
            Self::Osc32k => 0x04,
            Self::Xosc32k => 0x05,
            Self::Osc8m => 0x06,
            Self::Dfll48m => 0x07,
        }
    }
}

/// A generic clock generator: a source and a divider: any from 1 to 255
/// (up to 65,535 on generator 1, 31 on generator 2), or a larger power of
/// two (GENCTRL.DIVSEL).
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

/// How the DFLL48M runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dfll {
    /// Open loop, from the factory's coarse calibration and the middle fine
    /// value: about 48 MHz, as good as the calibration.
    OpenLoop,
    /// Closed loop: `multiply` times generator `reference`'s rate (1,464
    /// times 32.768 kHz for 47.97 MHz: 1,465 would be just over 48 MHz). The
    /// reference generator mustn't run from the DFLL48M.
    ClosedLoop { reference: usize, multiply: u16 },
}

/// The clock tree: the oscillators and the generators (generator 0 runs
/// the CPU). A generator left `None` is left as it is (generator 2 runs the
/// watchdog from OSCULP32K out of reset).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub osc8m: Osc8mDivide,
    /// Start the 32.768 kHz crystal on XIN32/XOUT32 (PA00/PA01).
    pub xosc32k: bool,
    /// Start the internal 32 kHz oscillator, from its factory calibration.
    pub osc32k: bool,
    pub dfll: Option<Dfll>,
    pub generators: [Option<Generator>; GENERATORS],
    /// VDD is below 2.7 V: flash then needs a wait state per 14 MHz rather
    /// than per 24 MHz.
    pub low_voltage: bool,
}

/// Why a [`Config`] can't be brought up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Generator 0 isn't configured: the CPU needs it.
    NoCpuClock,
    /// The CPU would run faster than [`CPU_MAX_HZ`].
    CpuTooFast(u32),
    /// Generator `n` divides by 0 or more than it can.
    Divider(usize),
    /// Generator `n`'s source isn't on (XOSC32K, OSC32K or the DFLL48M not
    /// configured), or it is generator 1 taking generator 1.
    SourceOff(usize),
    /// The DFLL48M's reference generator isn't configured, runs from the
    /// DFLL48M, or would make it run faster than [`DFLL48M_MAX_HZ`].
    DfllReference,
    /// XOSC32K didn't report ready (no crystal?).
    Xosc32kTimeout,
    /// OSC32K didn't report ready.
    Osc32kTimeout,
    /// The DFLL48M didn't report ready, or (closed loop) didn't lock.
    DfllTimeout,
    /// The GCLK didn't synchronise.
    GclkTimeout,
}

/// The rates a [`Config`] gives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frequencies {
    pub osc8m: u32,
    pub dfll48m: Option<u32>,
    pub generators: [Option<u32>; GENERATORS],
}

impl Frequencies {
    /// The CPU's rate (generator 0's).
    #[must_use]
    pub const fn cpu(&self) -> u32 {
        match self.generators[0] {
            Some(hz) => hz,
            None => 1_000_000,
        }
    }
}

impl Config {
    /// The clocks as they come out of reset: OSC8M divided by 8 (1 MHz) on
    /// generator 0.
    #[must_use]
    pub const fn boot() -> Self {
        let mut generators = [None; GENERATORS];
        generators[0] = Some(Generator::new(Source::Osc8m));
        Self {
            osc8m: Osc8mDivide::Div8,
            xosc32k: false,
            osc32k: false,
            dfll: None,
            generators,
            low_voltage: false,
        }
    }

    /// OSC8M undivided: 8 MHz on generator 0 for the CPU, generator 1 at
    /// 1 MHz (OSC8M / 8), for the time driver, and generator 2 at 1.024 kHz
    /// (OSCULP32K / 32) for the watchdog.
    #[must_use]
    pub const fn osc8m() -> Self {
        let mut config = Self::boot();
        config.osc8m = Osc8mDivide::Div1;
        config.generators[1] = Some(Generator::divided(Source::Osc8m, 8));
        config.generators[WATCHDOG_GENERATOR] = Some(Generator::divided(Source::OscUlp32k, 32));
        config
    }

    /// The CPU at 47.97 MHz from the DFLL48M locked to the 32.768 kHz
    /// crystal (1,464 times): XOSC32K on generator 3 as the DFLL's reference,
    /// the DFLL48M on generator 0, and generators 1 and 2 as in
    /// [`Config::osc8m`]: the DFLL's rate divides to no whole tick rate, so
    /// time stays as good as OSC8M.
    #[must_use]
    pub const fn dfll48m_from_xosc32k() -> Self {
        let mut config = Self::osc8m();
        config.xosc32k = true;
        config.generators[3] = Some(Generator::new(Source::Xosc32k));
        config.dfll = Some(Dfll::ClosedLoop { reference: 3, multiply: 1464 });
        config.generators[0] = Some(Generator::new(Source::Dfll48m));
        config
    }

    /// The rates this configuration gives, checked.
    ///
    /// # Errors
    ///
    /// See [`Error`].
    pub fn frequencies(&self) -> Result<Frequencies, Error> {
        let osc8m = self.osc8m.hz();
        // The generators that don't take the DFLL48M or generator 1 first,
        // then the DFLL48M from its reference, then the rest.
        let mut generators = [None; GENERATORS];
        let rate = |n: usize, g: &Generator, dfll: Option<u32>, gen1: Option<u32>| -> Result<Option<u32>, Error> {
            if division(n, g.divide).is_none() {
                return Err(Error::Divider(n));
            }
            let source = match g.source {
                Source::Osc8m => Some(osc8m),
                Source::OscUlp32k => Some(OSC32K_HZ),
                Source::Xosc32k if self.xosc32k => Some(OSC32K_HZ),
                Source::Osc32k if self.osc32k => Some(OSC32K_HZ),
                Source::Xosc32k | Source::Osc32k => return Err(Error::SourceOff(n)),
                Source::Dfll48m => dfll,
                Source::Generator1 if n == 1 => return Err(Error::SourceOff(n)),
                Source::Generator1 => gen1,
            };
            Ok(source.map(|hz| hz / u32::from(g.divide)))
        };
        for (n, generator) in self.generators.iter().enumerate() {
            if let Some(g) = generator
                && !matches!(g.source, Source::Dfll48m | Source::Generator1)
            {
                generators[n] = rate(n, g, None, None)?;
            }
        }
        let dfll48m = match self.dfll {
            None => None,
            Some(Dfll::OpenLoop) => Some(DFLL48M_HZ),
            Some(Dfll::ClosedLoop { reference, multiply }) => {
                let reference_hz = generators.get(reference).copied().flatten().ok_or(Error::DfllReference)?;
                let hz = u64::from(reference_hz) * u64::from(multiply);
                Some(u32::try_from(hz).ok().filter(|&hz| hz <= DFLL48M_MAX_HZ).ok_or(Error::DfllReference)?)
            }
        };
        for pass in 0..2 {
            for (n, generator) in self.generators.iter().enumerate() {
                if let Some(g) = generator
                    && matches!(g.source, Source::Dfll48m | Source::Generator1)
                {
                    generators[n] = rate(n, g, dfll48m, generators[1])?;
                    if generators[n].is_none() && (pass == 1 || g.source == Source::Dfll48m) {
                        return Err(Error::SourceOff(n));
                    }
                }
            }
        }
        let Some(cpu) = generators[0] else {
            return Err(Error::NoCpuClock);
        };
        if cpu > CPU_MAX_HZ {
            return Err(Error::CpuTooFast(cpu));
        }
        Ok(Frequencies { osc8m, dfll48m, generators })
    }

    /// Flash wait states for `cpu_hz` (NVM characteristics: 24 MHz per wait
    /// state at 2.7 V to 3.63 V, 14 MHz below).
    const fn wait_states(&self, cpu_hz: u32) -> u32 {
        let step = if self.low_voltage { 14_000_000 } else { 24_000_000 };
        cpu_hz.saturating_sub(1) / step
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

fn read8(address: usize) -> u8 {
    // SAFETY: a byte register of the clock controllers.
    unsafe { read_volatile(address as *const u8) }
}

fn write8(address: usize, value: u8) {
    // SAFETY: as for `write`.
    unsafe { write_volatile(address as *mut u8, value) }
}

fn read16(address: usize) -> u16 {
    // SAFETY: a halfword register of the clock controllers.
    unsafe { read_volatile(address as *const u16) }
}

fn write16(address: usize, value: u16) {
    // SAFETY: as for `write`.
    unsafe { write_volatile(address as *mut u16, value) }
}

fn wait(polls: u32, mut done: impl FnMut() -> bool) -> bool {
    (0..polls).any(|_| done())
}

fn gclk_sync() -> Result<(), Error> {
    if wait(1_000_000, || read8(GCLK_STATUS) & SYNCBUSY == 0) { Ok(()) } else { Err(Error::GclkTimeout) }
}

/// Generator `n`'s GENDIV.DIV and GENCTRL's DIVSEL for `divide`: an integer
/// division within DIV's width, else a power of two by DIVSEL; `None` if
/// neither can give it.
fn division(n: usize, divide: u16) -> Option<(u32, u32)> {
    let divide = u32::from(divide);
    match divide {
        0 => None,
        1 => Some((0, 0)),
        _ if divide <= MAX_DIVIDE[n] => Some((divide, 0)),
        _ if divide.is_power_of_two() && divide.trailing_zeros() - 1 <= MAX_DIVIDE[n] => Some((divide.trailing_zeros() - 1, DIVSEL)),
        _ => None,
    }
}

/// Sets generator `n`'s divider, then its source, each written with the
/// generator's ID, and waits for each to synchronise.
#[expect(clippy::cast_possible_truncation, reason = "generators number 8")]
fn set_generator(n: usize, generator: Generator) -> Result<(), Error> {
    let id = n as u32;
    let (divide, divsel) = division(n, generator.divide).ok_or(Error::Divider(n))?;
    write(GCLK_GENDIV, (divide << 8) | id);
    gclk_sync()?;
    let idc = if divsel == 0 && divide % 2 == 1 { IDC } else { 0 };
    write(GCLK_GENCTRL, (generator.source.code() << 8) | id | idc | divsel | GENEN);
    gclk_sync()
}

/// Feeds peripheral channel `channel` (a CLKCTRL.ID) from generator
/// `generator`. A channel already on that generator is left alone; one on
/// another is turned off first (CLKEN written 0, and read back as 0), as
/// the GCLK requires before it changes generator (section 12.6.3.3).
pub fn enable_channel(channel: usize, generator: usize) {
    #[expect(clippy::cast_possible_truncation, reason = "channel IDs are under 64, generators under 8")]
    let (id, id_byte, generator) = (channel as u16, channel as u8, generator as u16);
    // CLKCTRL reads back the channel its ID byte was last written with.
    let current = || {
        write8(GCLK_CLKCTRL, id_byte);
        read16(GCLK_CLKCTRL)
    };
    critical_section::with(|_| {
        let now = current();
        if now & CLKEN != 0 {
            if (now >> 8) & 0xF == generator {
                return;
            }
            write16(GCLK_CLKCTRL, id);
            let _ = gclk_sync();
            wait(1_000_000, || current() & CLKEN == 0);
        }
        write16(GCLK_CLKCTRL, id | (generator << 8) | CLKEN);
        let _ = gclk_sync();
    });
}

/// The generator peripheral channel `channel` runs from, if it's on.
pub(crate) fn channel_generator(channel: usize) -> Option<usize> {
    #[expect(clippy::cast_possible_truncation, reason = "channel IDs are under 64")]
    let id = channel as u8;
    let now = critical_section::with(|_| {
        write8(GCLK_CLKCTRL, id);
        read16(GCLK_CLKCTRL)
    });
    (now & CLKEN != 0).then_some(usize::from((now >> 8) & 0xF))
}

/// Turns on the APBC bus clocks in `mask` (PM.APBCMASK), which the
/// peripherals on APBC need and which are off at reset (but for the ADC's).
pub(crate) fn enable_apbc(mask: u32) {
    critical_section::with(|_| write(PM_APBCMASK, read(PM_APBCMASK) | mask));
}

/// The NVM software calibration area's first 64 bits.
fn calibration() -> u64 {
    // SAFETY: the calibration area, always readable, a word at a time.
    let (low, high) = unsafe { (read_volatile(NVM_CALIBRATION as *const u32), read_volatile((NVM_CALIBRATION + 4) as *const u32)) };
    u64::from(low) | (u64::from(high) << 32)
}

/// DFLLVAL.COARSE's factory value (bits 63:58), or the middle of its range
/// on dies that have none (revision C and before read all ones).
fn dfll_coarse() -> u32 {
    let coarse = (calibration() >> 58) as u32 & 0x3F;
    if coarse == 0x3F { 0x1F } else { coarse }
}

fn osc32k_calibration() -> u32 {
    (calibration() >> 38) as u32 & 0x7F
}

fn dfll_ready() -> Result<(), Error> {
    if wait(1_000_000, || read(SYSCTRL_PCLKSR) & DFLLRDY != 0) { Ok(()) } else { Err(Error::DfllTimeout) }
}

/// Starts XOSC32K, the crystal, unless it's already running as this starts
/// it: it runs through every reset but power-on, and an RTC may be counting
/// it, so it isn't restarted.
///
/// # Errors
///
/// [`Error::Xosc32kTimeout`] if it doesn't report ready.
pub(crate) fn start_xosc32k() -> Result<(), Error> {
    const ON: u32 = XOSC32K_EN32K | XOSC32K_XTALEN | OSC_ENABLE;
    if read(SYSCTRL_XOSC32K) & ON == ON && read(SYSCTRL_PCLKSR) & XOSC32KRDY != 0 {
        return Ok(());
    }
    write(SYSCTRL_XOSC32K, XOSC32K_STARTUP | XOSC32K_EN32K | XOSC32K_XTALEN);
    write(SYSCTRL_XOSC32K, XOSC32K_STARTUP | XOSC32K_EN32K | XOSC32K_XTALEN | OSC_ENABLE);
    // The start-up counter alone is half a second; the poll allows for up
    // to about 2 s at 48 MHz.
    if wait(20_000_000, || read(SYSCTRL_PCLKSR) & XOSC32KRDY != 0) { Ok(()) } else { Err(Error::Xosc32kTimeout) }
}

/// Generator `n`'s GENCTRL and GENDIV, as they are (each read after writing
/// its ID byte).
#[expect(clippy::cast_possible_truncation, reason = "generators number 8")]
fn generator_registers(n: usize) -> (u32, u32) {
    critical_section::with(|_| {
        write8(GCLK_GENCTRL, n as u8);
        let genctrl = read(GCLK_GENCTRL);
        write8(GCLK_GENDIV, n as u8);
        (genctrl, read(GCLK_GENDIV))
    })
}

/// Whether generator `n` runs as `generator` would set it (its source, its
/// division, on).
pub(crate) fn generator_is(n: usize, generator: Generator) -> bool {
    let Some((divide, divsel)) = division(n, generator.divide) else { return false };
    let (genctrl, gendiv) = generator_registers(n);
    let mask = (0x1F << 8) | GENEN | DIVSEL;
    genctrl & mask == (generator.source.code() << 8) | GENEN | divsel && (gendiv >> 8) & 0xFFFF == divide
}

/// Sets generator `n` as `generator` says, unless `keep` (it already runs
/// so, and something counting it mustn't glitch), and records its rate for
/// [`generator_hz`]: for drivers that own a generator of their own (the
/// RTC's). `source_hz` is its source's rate.
///
/// # Errors
///
/// [`Error::Divider`] or [`Error::GclkTimeout`].
pub(crate) fn use_generator(n: usize, generator: Generator, source_hz: u32, keep: bool) -> Result<u32, Error> {
    if division(n, generator.divide).is_none() {
        return Err(Error::Divider(n));
    }
    if !keep {
        set_generator(n, generator)?;
    }
    let hz = source_hz / u32::from(generator.divide);
    critical_section::with(|cs| {
        let cell = FREQUENCIES.borrow(cs);
        if let Some(mut f) = cell.get() {
            f.generators[n] = Some(hz);
            cell.set(Some(f));
        }
    });
    Ok(hz)
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
    // Wait states first, for the fastest the CPU will run meanwhile: its
    // new rate, or OSC8M's undivided while generator 0 still runs from it.
    let fastest = frequencies.cpu().max(config.osc8m.hz());
    let ctrlb = (read(NVMCTRL_CTRLB) & !(0xF << 1)) | NVMCTRL_MANW;
    write(NVMCTRL_CTRLB, ctrlb | (config.wait_states(fastest) << 1));
    // The CPU undivided.
    write8(PM_CPUSEL, 0);
    write(SYSCTRL_OSC8M, (read(SYSCTRL_OSC8M) & !OSC8M_PRESC_MASK) | (config.osc8m.presc() << OSC8M_PRESC_SHIFT));
    if config.xosc32k {
        start_xosc32k()?;
    }
    if config.osc32k {
        let value = (osc32k_calibration() << OSC32K_CALIB_SHIFT) | OSC32K_STARTUP | OSC32K_EN32K;
        write(SYSCTRL_OSC32K, value);
        write(SYSCTRL_OSC32K, value | OSC_ENABLE);
        if !wait(1_000_000, || read(SYSCTRL_PCLKSR) & OSC32KRDY != 0) {
            return Err(Error::Osc32kTimeout);
        }
    }
    // Generators 1 to 7 that don't take the DFLL48M or generator 1, so the
    // DFLL's reference is running before it starts.
    for (n, generator) in config.generators.iter().enumerate().skip(1) {
        if let Some(generator) = *generator
            && !matches!(generator.source, Source::Dfll48m | Source::Generator1)
        {
            set_generator(n, generator)?;
        }
    }
    if let Some(dfll) = config.dfll {
        // ONDEMAND off (its reset value is on) before any other DFLL
        // register, and DFLLRDY before each next write.
        write(SYSCTRL_DFLLCTRL, 0);
        dfll_ready()?;
        write(SYSCTRL_DFLLVAL, (dfll_coarse() << DFLLVAL_COARSE_SHIFT) | DFLL_FINE_MIDDLE);
        dfll_ready()?;
        match dfll {
            Dfll::OpenLoop => {
                write(SYSCTRL_DFLLCTRL, DFLL_ENABLE);
                dfll_ready()?;
            }
            Dfll::ClosedLoop { reference, multiply } => {
                enable_channel(DFLL48M_REF, reference);
                write(SYSCTRL_DFLLMUL, DFLLMUL_STEPS | u32::from(multiply));
                dfll_ready()?;
                write(SYSCTRL_DFLLCTRL, DFLL_MODE | DFLL_ENABLE);
                dfll_ready()?;
                // Coarse and fine lock: within a few hundred reference
                // cycles; the poll allows for some tens of milliseconds.
                if !wait(2_000_000, || read(SYSCTRL_PCLKSR) & (DFLLLCKC | DFLLLCKF) == DFLLLCKC | DFLLLCKF) {
                    return Err(Error::DfllTimeout);
                }
            }
        }
    }
    // Then the generators that take the DFLL48M or generator 1, generator 0
    // last.
    for (n, generator) in config.generators.iter().enumerate().skip(1) {
        if let Some(generator) = *generator
            && matches!(generator.source, Source::Dfll48m | Source::Generator1)
        {
            set_generator(n, generator)?;
        }
    }
    if let Some(generator) = config.generators[0] {
        set_generator(0, generator)?;
    }
    // Wait states for the rate the CPU settles at.
    write(NVMCTRL_CTRLB, ctrlb | (config.wait_states(frequencies.cpu()) << 1));
    critical_section::with(|cs| FREQUENCIES.borrow(cs).set(Some(frequencies)));
    Ok(frequencies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rates() {
        let f = Config::osc8m().frequencies().unwrap();
        assert_eq!((f.cpu(), f.generators[1]), (8_000_000, Some(1_000_000)));
        assert_eq!(Config::boot().frequencies().unwrap().cpu(), 1_000_000);
        let f = Config::dfll48m_from_xosc32k().frequencies().unwrap();
        assert_eq!((f.cpu(), f.dfll48m, f.generators[3]), (47_972_352, Some(47_972_352), Some(32_768)));
        assert_eq!(f.generators[WATCHDOG_GENERATOR], Some(WATCHDOG_HZ));
        assert_eq!(Config::osc8m().frequencies().unwrap().generators[WATCHDOG_GENERATOR], Some(WATCHDOG_HZ));
        let mut c = Config::osc8m();
        c.generators[0] = None;
        assert_eq!(c.frequencies(), Err(Error::NoCpuClock));
        c = Config::osc8m();
        c.generators[2] = Some(Generator::divided(Source::Osc8m, 33));
        assert_eq!(c.frequencies(), Err(Error::Divider(2)));
        c = Config::osc8m();
        c.generators[3] = Some(Generator::new(Source::Xosc32k));
        assert_eq!(c.frequencies(), Err(Error::SourceOff(3)));
        c = Config::dfll48m_from_xosc32k();
        c.dfll = Some(Dfll::ClosedLoop { reference: 3, multiply: 1465 });
        assert_eq!(c.frequencies(), Err(Error::DfllReference));
    }

    #[test]
    fn divisions() {
        assert_eq!(division(0, 1), Some((0, 0)));
        assert_eq!(division(0, 255), Some((255, 0)));
        assert_eq!(division(2, 31), Some((31, 0)));
        assert_eq!(division(2, 32), Some((4, DIVSEL)), "2^(4+1)");
        assert_eq!(division(2, 33), None);
        assert_eq!(division(0, 1024), Some((9, DIVSEL)));
        assert_eq!(division(1, 1000), Some((1000, 0)));
        assert_eq!(division(3, 0), None);
    }

    #[test]
    fn wait_states() {
        let c = Config::boot();
        assert_eq!((c.wait_states(8_000_000), c.wait_states(24_000_000), c.wait_states(48_000_000)), (0, 0, 1));
        let low = Config { low_voltage: true, ..c };
        assert_eq!((low.wait_states(14_000_000), low.wait_states(28_000_000), low.wait_states(48_000_000)), (0, 1, 3));
    }
}

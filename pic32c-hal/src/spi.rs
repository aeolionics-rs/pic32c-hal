// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A SERCOM as an SPI host (DS60001795 section 35.7; the PL10's older
//! SERCOM, `sercom_v1`, the same less CTRLC): 8-bit words, any of the four
//! modes, MSB or LSB first, SCK from the SERCOM's core clock
//! (`fref / (2 × (BAUD + 1))`, so at most half of it).
//!
//! The pins go by SERCOM pad, [`Pad<T, N>`] saying which pins can be pad `N`
//! of SERCOM `T` (from each chip's pin multiplexing table): SCK on pad 1,
//! MOSI on pad 0 or 3, and MISO on any other pad, all in one I/O set,
//! checked at compile time. Chip select is the
//! caller's: a GPIO, or `embedded-hal-bus`'s `SpiDevice`s over this bus.
//!
//! [`Spi::new_blocking`] moves each byte with the CPU. [`Spi::new`] takes two
//! [`dma`](crate::dma) channels and moves the data by DMA, receive and
//! transmit together, so a transfer has finished when its receive side has.
//! Both implement embedded-hal's `SpiBus`; the DMA one also implements
//! embedded-hal-async's. The transmit-only constructors leave MISO out; with
//! [`Config::loopback`] the receiver reads the MOSI pad instead, through the
//! pin, so a transfer reads back what it sent (a self-test needing no
//! wiring; the pin still drives MOSI).
//!
//! `sercom_v0` (the GV00's, the SAM D20's) has its registers at other
//! offsets and synchronises through STATUS.SYNCBUSY; with no DMA controller
//! it has only [`Spi::new_blocking`] and [`Spi::new_blocking_txonly`], and
//! no slew control. Its SCK is rated to 8 MHz.

use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

#[cfg(dma)]
use embassy_futures::join::join;
use embedded_hal::spi::{Mode, Phase, Polarity};

#[cfg(dma)]
use crate::dma::{self, AnyChannel, Request, Transfer};
use crate::gpio::{AnyPin, Function, set_function};
#[cfg(not(port_v0))]
use crate::gpio::{Slew, set_slew};
#[cfg(dma)]
use crate::mode::Async;
use crate::mode::Blocking;
use crate::uart::Instance;
use crate::{Peri, gpio};

const CTRLA: usize = 0x00;
const CTRLB: usize = 0x04;
#[cfg(sercom_v6)]
const CTRLC: usize = 0x08;
#[cfg(not(sercom_v0))]
const BAUD: usize = 0x0C;
#[cfg(not(sercom_v0))]
const INTFLAG: usize = 0x18;
#[cfg(not(sercom_v0))]
const STATUS: usize = 0x1A;
#[cfg(not(sercom_v0))]
const SYNCBUSY: usize = 0x1C;
#[cfg(not(sercom_v0))]
const DATA: usize = 0x28;
#[cfg(sercom_v0)]
const BAUD: usize = 0x0A;
#[cfg(sercom_v0)]
const INTFLAG: usize = 0x0E;
#[cfg(sercom_v0)]
const STATUS: usize = 0x10;
#[cfg(sercom_v0)]
const DATA: usize = 0x18;
/// STATUS.SYNCBUSY (`sercom_v0`): any write still synchronising.
#[cfg(sercom_v0)]
const STATUS_SYNCBUSY: u16 = 1 << 15;

const ENABLE: u32 = 1 << 1;
/// MODE 3: SPI host.
const MODE_HOST: u32 = 0x3 << 2;
const DOPO_SHIFT: u32 = 16;
const DIPO_SHIFT: u32 = 20;
const CPHA: u32 = 1 << 28;
const CPOL: u32 = 1 << 29;
const DORD_LSB: u32 = 1 << 30;
const RXEN: u32 = 1 << 17;
const SYNC_ENABLE: u32 = 1 << 1;
const SYNC_CTRLB: u32 = 1 << 2;

const DRE: u8 = 1 << 0;
const TXC: u8 = 1 << 1;
const RXC: u8 = 1 << 2;
const BUFOVF: u16 = 1 << 2;

/// The SERCOM core clock's highest rate.
const MAX_CORE_HZ: u32 = 100_000_000;
/// SCK's highest rate on a normal-speed SERCOM (DS60001795 table 52-31,
/// `MSP_1`: 21.3 MHz, sending or full duplex). Only SERCOM0 on PC10-PC13 is
/// rated faster; this driver doesn't use that yet. Measured: a W5500 on
/// 15 cm leads reads back wrong at 24 MHz and right at 12.
#[cfg(not(sercom_v0))]
const MAX_SCK_HZ: u32 = 21_300_000;
/// The GV00's (DS60001881 table 31-34, `MSP_1`: 8 MHz sending; full duplex
/// depends on the other end).
#[cfg(sercom_v0)]
const MAX_SCK_HZ: u32 = 8_000_000;

pub use crate::sercom::Pad;

/// Bit order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BitOrder {
    MsbFirst,
    LsbFirst,
}

/// An SPI host's configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// SCK's rate, at most: the nearest the SERCOM clock divides to that
    /// isn't faster, and no faster than the SERCOM's rated 21.3 MHz
    /// ([`Spi::frequency`] says what it is). From a 48 MHz clock that's
    /// 12 MHz at most; 80 MHz gives 20.
    pub frequency: u32,
    /// Clock polarity and phase.
    pub mode: Mode,
    pub bit_order: BitOrder,
    /// The generic clock generator to feed the SERCOM from.
    pub generator: usize,
    /// Whether the receiver reads the MOSI pad, through the pin: a
    /// self-test, reading back what was sent.
    pub loopback: bool,
    /// SCK's and MOSI's slew limit (PINCFG.SLEWLIM; embassy-stm32's
    /// `gpio_speed`). `Fast` by default; a slower edge stops a long or
    /// unterminated SCK ringing into extra clocks (measured: a W5500 on
    /// 15 cm leads slipped a bit in about one 2 KiB read in six at 12 and
    /// 20 MHz with `Fast`, none in 120 with `Slow4`; `Slow12` is too slow
    /// for 20 MHz). Pins without slew control ignore it. Not on `port_v0`
    /// (the GV00), whose PORT has no slew control.
    #[cfg(not(port_v0))]
    pub slew: Slew,
}

impl Default for Config {
    /// 1 MHz, mode 0, MSB first, from generator 3 (48 MHz in
    /// `clocks::Config::pll_from_xosc`; on the PL10, generator 0, the
    /// CPU's 24 MHz).
    fn default() -> Self {
        Self {
            frequency: 1_000_000,
            mode: embedded_hal::spi::MODE_0,
            bit_order: BitOrder::MsbFirst,
            generator: if cfg!(any(sercom_v1, sercom_v0)) { 0 } else { 3 },
            loopback: false,
            #[cfg(not(port_v0))]
            slew: Slew::Fast,
        }
    }
}

/// Why an SPI can't be set up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// The generator isn't configured.
    NoClock,
    /// The generator runs over 100 MHz.
    ClockTooFast,
    /// SCK can't be that slow from this generator (BAUD over 255), or the
    /// frequency is 0.
    FrequencyTooLow,
}

/// An SPI error: the receiver overflowed, losing a byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    Overrun,
    /// A DMA transfer failed.
    #[cfg(dma)]
    Dma(dma::Error),
}

impl embedded_hal::spi::Error for Error {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        match self {
            Self::Overrun => embedded_hal::spi::ErrorKind::Overrun,
            #[cfg(dma)]
            Self::Dma(_) => embedded_hal::spi::ErrorKind::Other,
        }
    }
}

/// An SPI host on a SERCOM.
pub struct Spi<'d, M> {
    base: usize,
    #[cfg(dma)]
    number: u8,
    gclk_channel: usize,
    /// MOSI's pad and MISO's (none: send only), which fix DOPO and DIPO.
    mosi_pad: u8,
    miso_pad: Option<u8>,
    /// SCK and MOSI, for their slew limit.
    #[cfg_attr(port_v0, allow(dead_code, reason = "no slew control"))]
    sck: AnyPin,
    #[cfg_attr(port_v0, allow(dead_code, reason = "no slew control"))]
    mosi: AnyPin,
    /// Whether the receiver is on (MISO, or loopback).
    receives: bool,
    frequency: u32,
    #[cfg(dma)]
    dma: Option<(Peri<'d, AnyChannel>, Peri<'d, AnyChannel>)>,
    _mode: PhantomData<(&'d (), M)>,
}

/// The DMA channels a [`Spi`] takes: none without a DMA controller.
#[cfg(dma)]
type DmaChannels<'d> = (Peri<'d, AnyChannel>, Peri<'d, AnyChannel>);
#[cfg(not(dma))]
type DmaChannels<'d> = (core::convert::Infallible, PhantomData<&'d ()>);

impl<'d> Spi<'d, Blocking> {
    /// An SPI host on `sercom`: SCK on pad 1, MOSI on pad `DO` (0 or 3),
    /// MISO on pad `DI` (another), moving bytes with the CPU.
    ///
    /// # Errors
    ///
    /// If [`Config`]'s generator can't give a clock at or under its
    /// frequency.
    #[allow(clippy::needless_pass_by_value, reason = "taking them is what makes them the SPI's")]
    pub fn new_blocking<T: Instance, const DO: u8, const DI: u8, const IOSET: u8>(
        sercom: Peri<'d, T>,
        sck: Peri<'d, impl Pad<T, 1, IOSET>>,
        mosi: Peri<'d, impl Pad<T, DO, IOSET>>,
        miso: Peri<'d, impl Pad<T, DI, IOSET>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        const { check_pads(DO, Some(DI)) };
        let _ = sercom;
        set_function(&*miso, miso.function());
        Self::start::<T>(&*sck, sck.function(), &*mosi, mosi.function(), DO, Some(DI), None, config)
    }

    /// An SPI host on `sercom` with no MISO: it only sends, unless
    /// [`Config::loopback`] has it read back what it sends.
    ///
    /// # Errors
    ///
    /// As [`new_blocking`](Self::new_blocking).
    #[allow(clippy::needless_pass_by_value, reason = "taking them is what makes them the SPI's")]
    pub fn new_blocking_txonly<T: Instance, const DO: u8, const IOSET: u8>(sercom: Peri<'d, T>, sck: Peri<'d, impl Pad<T, 1, IOSET>>, mosi: Peri<'d, impl Pad<T, DO, IOSET>>, config: Config) -> Result<Self, ConfigError> {
        const { check_pads(DO, None) };
        let _ = sercom;
        Self::start::<T>(&*sck, sck.function(), &*mosi, mosi.function(), DO, None, None, config)
    }
}

#[cfg(dma)]
impl<'d> Spi<'d, Async> {
    /// An SPI host as [`Spi::new_blocking`], moving the data by DMA on
    /// `tx_dma` and `rx_dma`.
    ///
    /// # Errors
    ///
    /// As [`Spi::new_blocking`].
    #[allow(clippy::needless_pass_by_value, reason = "taking them is what makes them the SPI's")]
    #[allow(clippy::too_many_arguments, reason = "the SERCOM, three pins, two channels and the configuration")]
    pub fn new<T: Instance, const DO: u8, const DI: u8, const IOSET: u8>(
        sercom: Peri<'d, T>,
        sck: Peri<'d, impl Pad<T, 1, IOSET>>,
        mosi: Peri<'d, impl Pad<T, DO, IOSET>>,
        miso: Peri<'d, impl Pad<T, DI, IOSET>>,
        tx_dma: Peri<'d, impl dma::Channel>,
        rx_dma: Peri<'d, impl dma::Channel>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        const { check_pads(DO, Some(DI)) };
        let _ = sercom;
        set_function(&*miso, miso.function());
        Self::start::<T>(&*sck, sck.function(), &*mosi, mosi.function(), DO, Some(DI), Some((tx_dma.into(), rx_dma.into())), config)
    }

    /// An SPI host as [`Spi::new_blocking_txonly`], moving the data by DMA.
    ///
    /// # Errors
    ///
    /// As [`Spi::new_blocking`].
    #[allow(clippy::needless_pass_by_value, reason = "taking them is what makes them the SPI's")]
    pub fn new_txonly<T: Instance, const DO: u8, const IOSET: u8>(
        sercom: Peri<'d, T>,
        sck: Peri<'d, impl Pad<T, 1, IOSET>>,
        mosi: Peri<'d, impl Pad<T, DO, IOSET>>,
        tx_dma: Peri<'d, impl dma::Channel>,
        rx_dma: Peri<'d, impl dma::Channel>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        const { check_pads(DO, None) };
        let _ = sercom;
        Self::start::<T>(&*sck, sck.function(), &*mosi, mosi.function(), DO, None, Some((tx_dma.into(), rx_dma.into())), config)
    }

    /// Sends `write` and reads into `read` at once, as embedded-hal's
    /// `SpiBus::transfer`: the shorter is padded (zeros sent, bytes read
    /// thrown away).
    ///
    /// # Errors
    ///
    /// A DMA error.
    pub async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        let common = read.len().min(write.len());
        let (read_common, read_rest) = read.split_at_mut(common);
        let (write_common, write_rest) = write.split_at(common);
        self.dma_exchange(Some(read_common), Some(write_common), common).await?;
        let n = read_rest.len();
        self.dma_exchange(Some(read_rest), None, n).await?;
        self.dma_exchange(None, Some(write_rest), write_rest.len()).await
    }

    /// Sends `words` and reads into them what comes back.
    ///
    /// # Errors
    ///
    /// A DMA error.
    pub async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Error> {
        // The DMA writes each byte back only after sending it, but the two
        // channels mustn't share the buffer: send from a copy, a chunk at a
        // time.
        let mut chunk = [0u8; 64];
        for part in words.chunks_mut(chunk.len()) {
            let out = &mut chunk[..part.len()];
            out.copy_from_slice(part);
            let n = part.len();
            self.dma_exchange(Some(part), Some(out), n).await?;
        }
        Ok(())
    }

    /// Reads `words`, sending zeros.
    ///
    /// # Errors
    ///
    /// A DMA error.
    pub async fn read(&mut self, words: &mut [u8]) -> Result<(), Error> {
        let n = words.len();
        self.dma_exchange(Some(words), None, n).await
    }

    /// Sends `words`, throwing away what comes back.
    ///
    /// # Errors
    ///
    /// A DMA error.
    pub async fn write(&mut self, words: &[u8]) -> Result<(), Error> {
        self.dma_exchange(None, Some(words), words.len()).await
    }

    /// Moves `count` bytes: from `write`, or zeros; into `read`, or
    /// nowhere. Done when the last byte is in, or, with no receiver, out.
    async fn dma_exchange(&mut self, mut read: Option<&mut [u8]>, write: Option<&[u8]>, count: usize) -> Result<(), Error> {
        if count == 0 {
            return Ok(());
        }
        let data = (self.base + DATA) as *mut u8;
        let (rx_request, tx_request) = (Request::sercom_rx(self.number), Request::sercom_tx(self.number));
        let receives = self.receives;
        let base = self.base;
        let (tx_dma, rx_dma) = self.dma.as_mut().expect("an async SPI has its channels");
        for at in (0..count).step_by(dma::MAX_BLOCK) {
            // TXC stays set from the last byte until cleared.
            write8(base, INTFLAG, TXC);
            let n = (count - at).min(dma::MAX_BLOCK);
            if receives {
                // The receiver first: at the higher rates the first bytes
                // come back within a microsecond of the first going out.
                // SAFETY: as above.
                let rx = unsafe {
                    match read.as_deref_mut() {
                        Some(r) => Transfer::new_read(rx_dma.reborrow(), rx_request, data.cast_const(), &mut r[at..at + n]),
                        None => Transfer::new_read_discard(rx_dma.reborrow(), rx_request, data.cast_const(), n),
                    }
                };
                let tx = start_tx(tx_dma, tx_request, write.map(|w| &w[at..at + n]), n, data);
                let (t, r) = join(tx, rx).await;
                t.and(r).map_err(Error::Dma)?;
                if read16(base, STATUS) & BUFOVF != 0 {
                    write16(base, STATUS, BUFOVF);
                    return Err(Error::Overrun);
                }
            } else {
                start_tx(tx_dma, tx_request, write.map(|w| &w[at..at + n]), n, data).await.map_err(Error::Dma)?;
                while read8(base, INTFLAG) & TXC == 0 {}
            }
        }
        Ok(())
    }
}

/// The transmit side of an exchange: `write`, or `count` zeros.
#[cfg(dma)]
fn start_tx<'a>(channel: &'a mut Peri<'_, AnyChannel>, request: Request, write: Option<&'a [u8]>, count: usize, data: *mut u8) -> Transfer<'a> {
    static ZERO: u8 = 0;
    // SAFETY: the SERCOM's DATA register, an SPI host making `request`.
    unsafe {
        match write {
            Some(w) => Transfer::new_write(channel.reborrow(), request, w, data),
            None => Transfer::new_write_repeated(channel.reborrow(), request, &ZERO, count, data),
        }
    }
}

/// The pads' rules: MOSI on pad 0 or 3, MISO on neither SCK's pad (1) nor
/// MOSI's.
const fn check_pads(mosi: u8, miso: Option<u8>) {
    assert!(mosi == 0 || mosi == 3, "MOSI goes on pad 0 or pad 3");
    if let Some(miso) = miso {
        assert!(miso < 4 && miso != 1 && miso != mosi, "MISO goes on a pad of its own, not SCK's (1) or MOSI's");
    }
}

impl<'d, M> Spi<'d, M> {
    #[allow(clippy::too_many_arguments, reason = "internal, from the constructors")]
    fn start<T: Instance>(sck: &impl gpio::SealedPin, sck_function: Function, mosi: &impl gpio::SealedPin, mosi_function: Function, mosi_pad: u8, miso_pad: Option<u8>, dma: Option<DmaChannels<'d>>, config: Config) -> Result<Self, ConfigError> {
        // Nothing is touched if `config` can't be met.
        validate(&config)?;
        let base = T::BASE;
        #[cfg(any(sercom_v1, sercom_v0))]
        crate::clocks::enable_apbc(T::APBC);
        #[cfg(not(dma))]
        let _ = dma;
        // The reset synchronises on the core clock, so that first.
        crate::clocks::enable_channel(T::state().gclk_channel, config.generator);
        crate::sercom::reset(base);
        let mut spi = Self {
            base,
            #[cfg(dma)]
            number: T::NUMBER,
            gclk_channel: T::state().gclk_channel,
            mosi_pad,
            miso_pad,
            sck: AnyPin::from_pin_port(sck.pin_port()),
            mosi: AnyPin::from_pin_port(mosi.pin_port()),
            receives: false,
            frequency: 0,
            #[cfg(dma)]
            dma,
            _mode: PhantomData,
        };
        spi.set_config(&config)?;
        set_function(sck, sck_function);
        set_function(mosi, mosi_function);
        // After the functions, which leave PINCFG's other bits clear.
        #[cfg(not(port_v0))]
        {
            set_slew(sck, config.slew);
            set_slew(mosi, config.slew);
        }
        Ok(spi)
    }

    /// Changes the configuration (embassy-embedded-hal's `SetConfig`, for
    /// a shared bus's devices): SCK's rate, the mode, the bit order, the
    /// generator, loopback and the slew limit. The SERCOM is disabled
    /// meanwhile, so call it between transfers.
    ///
    /// # Errors
    ///
    /// If `config` can't be met; the configuration is then as it was.
    pub fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        let (clock_hz, baud) = validate(config)?;
        let base = self.base;
        crate::clocks::enable_channel(self.gclk_channel, config.generator);
        write32(base, CTRLA, read32(base, CTRLA) & !ENABLE);
        wait_sync(base, SYNC_ENABLE);
        let out_pinout = if self.mosi_pad == 3 { 2 } else { 0 };
        let (dipo, receives) = match (config.loopback, self.miso_pad) {
            (true, _) => (self.mosi_pad, true),
            (false, Some(pad)) => (pad, true),
            (false, None) => (0, false),
        };
        let mut ctrla = MODE_HOST | (out_pinout << DOPO_SHIFT) | (u32::from(dipo) << DIPO_SHIFT);
        if config.mode.polarity == Polarity::IdleHigh {
            ctrla |= CPOL;
        }
        if config.mode.phase == Phase::CaptureOnSecondTransition {
            ctrla |= CPHA;
        }
        if config.bit_order == BitOrder::LsbFirst {
            ctrla |= DORD_LSB;
        }
        write32(base, CTRLA, ctrla);
        // 8-bit characters, no hardware SS.
        write32(base, CTRLB, if receives { RXEN } else { 0 });
        wait_sync(base, SYNC_CTRLB);
        #[cfg(sercom_v6)]
        write32(base, CTRLC, 0);
        write8(base, BAUD, baud);
        write32(base, CTRLA, ctrla | ENABLE);
        wait_sync(base, SYNC_ENABLE);
        #[cfg(not(port_v0))]
        {
            set_slew(&self.sck, config.slew);
            set_slew(&self.mosi, config.slew);
        }
        self.receives = receives;
        self.frequency = clock_hz / (2 * (u32::from(baud) + 1));
        Ok(())
    }

    /// SCK's rate.
    #[must_use]
    pub const fn frequency(&self) -> u32 {
        self.frequency
    }

    /// Sends `write` and reads into `read`, with the CPU, as
    /// [`Spi::transfer`] does by DMA.
    ///
    /// # Errors
    ///
    /// [`Error::Overrun`] if the receiver lost a byte.
    pub fn blocking_transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        write8(self.base, INTFLAG, TXC);
        for i in 0..read.len().max(write.len()) {
            let byte = self.exchange(write.get(i).copied().unwrap_or(0))?;
            if let Some(slot) = read.get_mut(i) {
                *slot = byte;
            }
        }
        self.blocking_flush();
        Ok(())
    }

    /// Sends `words` and reads into them what comes back, with the CPU.
    ///
    /// # Errors
    ///
    /// As [`blocking_transfer`](Self::blocking_transfer).
    pub fn blocking_transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Error> {
        write8(self.base, INTFLAG, TXC);
        for word in words.iter_mut() {
            *word = self.exchange(*word)?;
        }
        self.blocking_flush();
        Ok(())
    }

    /// Waits until the last byte has gone out.
    pub fn blocking_flush(&mut self) {
        while read8(self.base, INTFLAG) & TXC == 0 {}
    }

    /// Sends one byte and, if the receiver is on, returns the one that came
    /// back (0 otherwise).
    fn exchange(&mut self, byte: u8) -> Result<u8, Error> {
        while read8(self.base, INTFLAG) & DRE == 0 {}
        write_data(self.base, byte);
        if !self.receives {
            return Ok(0);
        }
        while read8(self.base, INTFLAG) & RXC == 0 {}
        if read16(self.base, STATUS) & BUFOVF != 0 {
            write16(self.base, STATUS, BUFOVF);
            return Err(Error::Overrun);
        }
        Ok(read_data(self.base))
    }
}

impl<M> Drop for Spi<'_, M> {
    fn drop(&mut self) {
        // ENABLE alone: CTRLA 0 would also make it a USART on an external
        // clock, whose ENABLE then waits for that clock to synchronise
        // (never, on the SG00).
        write32(self.base, CTRLA, read32(self.base, CTRLA) & !ENABLE);
        wait_sync(self.base, SYNC_ENABLE);
    }
}

/// Waits for the writes in `mask` (SYNCBUSY's bits) to synchronise; on
/// `sercom_v0`, for any write (STATUS.SYNCBUSY).
fn wait_sync(base: usize, mask: u32) {
    #[cfg(not(sercom_v0))]
    while read32(base, SYNCBUSY) & mask != 0 {}
    #[cfg(sercom_v0)]
    {
        let _ = mask;
        while read16(base, STATUS) & STATUS_SYNCBUSY != 0 {}
    }
}

/// `config`'s core clock and BAUD value, if it can be met.
fn validate(config: &Config) -> Result<(u32, u8), ConfigError> {
    let clock_hz = crate::clocks::generator_hz(config.generator).ok_or(ConfigError::NoClock)?;
    if clock_hz > MAX_CORE_HZ {
        return Err(ConfigError::ClockTooFast);
    }
    let baud = baud_value(clock_hz, config.frequency).ok_or(ConfigError::FrequencyTooLow)?;
    Ok((clock_hz, baud))
}

/// The BAUD value for SCK at `frequency` or under (and at the SERCOM's
/// rated maximum or under), from a `clock_hz` clock:
/// `clock_hz / (2 × (BAUD + 1))`.
fn baud_value(clock_hz: u32, frequency: u32) -> Option<u8> {
    if frequency == 0 {
        return None;
    }
    let frequency = frequency.min(MAX_SCK_HZ);
    let baud = clock_hz.div_ceil(2 * frequency).saturating_sub(1);
    u8::try_from(baud).ok()
}

impl<M> embassy_embedded_hal::SetConfig for Spi<'_, M> {
    type Config = Config;
    type ConfigError = ConfigError;

    fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        Spi::set_config(self, config)
    }
}

impl embedded_hal::spi::ErrorType for Spi<'_, Blocking> {
    type Error = Error;
}

impl embedded_hal::spi::SpiBus for Spi<'_, Blocking> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Error> {
        self.blocking_transfer(words, &[])
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Error> {
        self.blocking_transfer(&mut [], words)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        self.blocking_transfer(read, write)
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Error> {
        self.blocking_transfer_in_place(words)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.blocking_flush();
        Ok(())
    }
}

#[cfg(dma)]
impl embedded_hal::spi::ErrorType for Spi<'_, Async> {
    type Error = Error;
}

#[cfg(dma)]
impl embedded_hal::spi::SpiBus for Spi<'_, Async> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Error> {
        self.blocking_transfer(words, &[])
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Error> {
        self.blocking_transfer(&mut [], words)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        self.blocking_transfer(read, write)
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Error> {
        self.blocking_transfer_in_place(words)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.blocking_flush();
        Ok(())
    }
}

#[cfg(dma)]
impl embedded_hal_async::spi::SpiBus for Spi<'_, Async> {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Error> {
        Spi::read(self, words).await
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Error> {
        Spi::write(self, words).await
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        Spi::transfer(self, read, write).await
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Error> {
        Spi::transfer_in_place(self, words).await
    }

    async fn flush(&mut self) -> Result<(), Error> {
        // Each DMA transfer ends with its last byte in, or out.
        core::future::ready(Ok(())).await
    }
}

/// DATA, written: 32 bits wide on `sercom_v6`, 16 on `sercom_v1` and
/// `sercom_v0`.
fn write_data(base: usize, byte: u8) {
    #[cfg(sercom_v6)]
    write32(base, DATA, u32::from(byte));
    #[cfg(any(sercom_v1, sercom_v0))]
    write16(base, DATA, u16::from(byte));
}

/// DATA, read, likewise: 8-bit characters.
#[expect(clippy::cast_possible_truncation, reason = "8-bit characters")]
fn read_data(base: usize) -> u8 {
    #[cfg(sercom_v6)]
    return read32(base, DATA) as u8;
    #[cfg(any(sercom_v1, sercom_v0))]
    return read16(base, DATA) as u8;
}

fn reg<T>(base: usize, offset: usize) -> *mut T {
    (base + offset) as *mut T
}

fn read8(base: usize, offset: usize) -> u8 {
    // SAFETY: a SERCOM register, read at its own width.
    unsafe { read_volatile(reg(base, offset)) }
}

fn read16(base: usize, offset: usize) -> u16 {
    // SAFETY: as for `read8`.
    unsafe { read_volatile(reg(base, offset)) }
}

fn read32(base: usize, offset: usize) -> u32 {
    // SAFETY: as for `read8`.
    unsafe { read_volatile(reg(base, offset)) }
}

fn write8(base: usize, offset: usize, value: u8) {
    // SAFETY: as for `read8`.
    unsafe { write_volatile(reg(base, offset), value) }
}

fn write16(base: usize, offset: usize, value: u16) {
    // SAFETY: as for `read8`.
    unsafe { write_volatile(reg(base, offset), value) }
}

fn write32(base: usize, offset: usize, value: u32) {
    // SAFETY: as for `read8`.
    unsafe { write_volatile(reg(base, offset), value) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(sercom_v0))]
    fn baud_values() {
        // 48 MHz: 12 is BAUD 1, 1 MHz 23; never faster than asked, nor
        // than 21.3 MHz, so 24 and 30 MHz give 12.
        assert_eq!(baud_value(48_000_000, 24_000_000), Some(1));
        assert_eq!(baud_value(48_000_000, 30_000_000), Some(1));
        assert_eq!(baud_value(48_000_000, 12_000_000), Some(1));
        // 80 MHz: 20 MHz (BAUD 1), not 40.
        assert_eq!(baud_value(80_000_000, 40_000_000), Some(1));
        assert_eq!(baud_value(48_000_000, 1_000_000), Some(23));
        assert_eq!(baud_value(48_000_000, 7_000_000), Some(3), "6 MHz, not 8");
        // The slowest from 48 MHz is 93.75 kHz (BAUD 255).
        assert_eq!(baud_value(48_000_000, 93_750), Some(255));
        assert_eq!(baud_value(48_000_000, 90_000), None);
        assert_eq!(baud_value(48_000_000, 0), None);
    }

    #[test]
    #[cfg(sercom_v0)]
    fn baud_values() {
        // 8 MHz at most: BAUD 2 from 48 MHz, whatever faster is asked.
        assert_eq!(baud_value(48_000_000, 24_000_000), Some(2));
        assert_eq!(baud_value(48_000_000, 8_000_000), Some(2));
        assert_eq!(baud_value(8_000_000, 8_000_000), Some(0), "4 MHz, half the clock");
        assert_eq!(baud_value(48_000_000, 1_000_000), Some(23));
        assert_eq!(baud_value(48_000_000, 0), None);
    }

    #[test]
    fn pad_rules() {
        check_pads(0, Some(2));
        check_pads(3, Some(0));
        check_pads(0, None);
    }

    #[test]
    #[should_panic(expected = "MISO")]
    fn miso_on_sck() {
        check_pads(0, Some(1));
    }
}

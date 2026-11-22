// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A SERCOM as a UART (DS60001795 section 35.6): asynchronous, internal
//! clock, LSB first, 16x arithmetic sampling; 8 data bits, no, even or odd
//! parity, one or two stop bits.
//!
//! [`Uart::new_blocking`] waits on the flags; [`Uart::new`] takes two DMA
//! channels for its reads and writes (as Embassy's UARTs do), and a
//! [`bind_interrupts!`](crate::bind_interrupts) type binding the SERCOM's
//! DRE (`SERCOMn_0`), TXC (`_1`), RXC (`_2`) and ERROR (`_56`) vectors to
//! [`InterruptHandler`] for the rest. [`BufferedUart`] keeps the bytes in
//! ring buffers through the interrupts, as Embassy's does. All take the TX
//! pin (the SERCOM's pad 0) and the RX pin (pad 1, 2 or 3), both from one I/O
//! set: [`Pad`](crate::sercom::Pad) says which pins.
//!
//! The driver feeds the SERCOM's core clock channel from the generator its
//! [`Config`] names, which must run at 16 times the baud rate or more (and
//! at most 100 MHz). The FIFO is on unless [`Config::fifo`] says not; DRE
//! then means the TX FIFO has room, RXC that the RX FIFO has a byte, and
//! TXC that both it and the shift register are empty. BAUD is the 16x arithmetic
//! kind, whose steps grow at low rates (9,600 from 48 MHz is 0.14 % high).

#[cfg(dma)]
use core::future::poll_fn;
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};
#[cfg(dma)]
use core::task::Poll;
use portable_atomic::{AtomicU32, Ordering};

use embassy_sync::waitqueue::AtomicWaker;

#[cfg(dma)]
use crate::dma::AnyChannel;
use crate::gpio::{Function, SealedPin, set_function};
use crate::interrupt::typelevel::Interrupt as TypelevelInterrupt;
#[cfg(dma)]
use crate::interrupt::typelevel::{Binding, Handler};
use crate::sercom::Pad;
use crate::{Peri, PeripheralType};

const CTRLA: usize = 0x00;
const CTRLB: usize = 0x04;
#[cfg(sercom_v6)]
const CTRLC: usize = 0x08;
const BAUD: usize = 0x0C;
const INTENCLR: usize = 0x14;
const INTENSET: usize = 0x16;
const INTFLAG: usize = 0x18;
const STATUS: usize = 0x1A;
const SYNCBUSY: usize = 0x1C;
const DATA: usize = 0x28;
#[cfg(all(dma, sercom_v6))]
const FIFOSPACE: usize = 0x34;
/// FIFOSPACE.RXSPACE: bytes in the receive FIFO.
#[cfg(all(dma, sercom_v6))]
const RXSPACE_SHIFT: u16 = 8;
#[cfg(all(dma, sercom_v6))]
const RXSPACE_MASK: u16 = 0x1F;

const SWRST: u32 = 1 << 0;
const ENABLE: u32 = 1 << 1;
/// MODE 1: USART with the internal clock.
const MODE_INTERNAL: u32 = 1 << 2;
const RXPO_SHIFT: u32 = 20;
/// TXPO 2: TX on pad 0, RTS on pad 2, CTS on pad 3.
const TXPO_RTS_CTS: u32 = 0x2 << 16;
/// FORM 1: a frame with parity.
const FORM_PARITY: u32 = 1 << 24;
const DORD_LSB: u32 = 1 << 30;
const SBMODE_TWO: u32 = 1 << 6;
const PMODE_ODD: u32 = 1 << 13;
const TXEN: u32 = 1 << 16;
const RXEN: u32 = 1 << 17;
/// CTRLC.FIFOEN, with both thresholds at their default: DRE while the TX
/// FIFO isn't full, RXC while the RX FIFO isn't empty.
#[cfg(sercom_v6)]
const FIFOEN: u32 = 1 << 27;
const SYNC_SWRST: u32 = 1 << 0;
const SYNC_ENABLE: u32 = 1 << 1;
const SYNC_CTRLB: u32 = 1 << 2;

const DRE: u8 = 1 << 0;
const TXC: u8 = 1 << 1;
const RXC: u8 = 1 << 2;
const ERROR: u8 = 1 << 7;
const PERR: u16 = 1 << 0;
const FERR: u16 = 1 << 1;
const BUFOVF: u16 = 1 << 2;
const CTS: u16 = 1 << 3;

/// RX goes on pad 1, 2 or 3 (TX has pad 0).
const fn check_rx_pad(pad: u8) {
    assert!(pad >= 1 && pad <= 3, "RX goes on pad 1, 2 or 3");
}

/// The SERCOM core clock's highest rate.
const MAX_CORE_HZ: u32 = 100_000_000;

/// Parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    None,
    Even,
    Odd,
}

/// Stop bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    One,
    Two,
}

/// A UART's configuration: 8 data bits, LSB first, and these.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub baudrate: u32,
    pub parity: Parity,
    pub stop_bits: StopBits,
    /// The generic clock generator to feed the SERCOM from.
    pub generator: usize,
    /// Whether to use the SERCOM's FIFO (16 bytes on SERCOM0, 8 on the
    /// others), which lets more bytes come before an overrun, and more go
    /// out without waiting. On by default. A receive DMA started on an empty
    /// FIFO would take one stale byte from the FIFO's memory first (seen on
    /// the bench), so a [`Uart`] with DMA starts each read's transfer only
    /// once a byte is in. `sercom_v6` only (the PIC32CK); off by default,
    /// and refused, on `sercom_v1`.
    pub fifo: bool,
    /// The RX pin's pull (embassy-stm32's `rx_pull`), for a line that may
    /// be left unconnected. None by default. The PIC32CK's PORT disables
    /// pulls on peripheral pins, so there it has no effect: use a resistor.
    pub rx_pull: crate::gpio::Pull,
}

impl Default for Config {
    /// 115,200 baud, no parity, one stop bit, from generator 3 (48 MHz in
    /// [`clocks::Config::pll_from_xosc`](crate::clocks::Config::pll_from_xosc)),
    /// the FIFO on.
    fn default() -> Self {
        Self {
            baudrate: 115_200,
            parity: Parity::None,
            stop_bits: StopBits::One,
            generator: if cfg!(sercom_v1) { 0 } else { 3 },
            fifo: cfg!(sercom_v6),
            rx_pull: crate::gpio::Pull::None,
        }
    }
}

/// Why a UART couldn't be set up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    /// The generator isn't configured.
    NoClock,
    /// The generator runs over 100 MHz.
    ClockTooFast,
    /// The generator is slower than 16 times the baud rate.
    BaudTooHigh,
    /// [`Config::fifo`] on a SERCOM without one (`sercom_v1`).
    NoFifo,
}

/// A receive error. The byte that came with it is lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// No valid stop bit.
    Framing,
    /// The parity didn't match.
    Parity,
    /// A byte came while the receive buffer was full; one or more were lost.
    Overrun,
    /// A DMA transfer failed.
    #[cfg(dma)]
    Dma(crate::dma::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Framing => "framing error",
            Self::Parity => "parity error",
            Self::Overrun => "overrun",
            #[cfg(dma)]
            Self::Dma(_) => "DMA error",
        })
    }
}

impl core::error::Error for Error {}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

/// The BAUD value for `baudrate` from `clock_hz`, 16x arithmetic:
/// 65536 x (1 - 16 x baud / f), rounded.
fn baud_value(clock_hz: u32, baudrate: u32) -> Result<u16, ConfigError> {
    if clock_hz > MAX_CORE_HZ {
        return Err(ConfigError::ClockTooFast);
    }
    let sixteen = 16 * u64::from(baudrate);
    if baudrate == 0 || sixteen > u64::from(clock_hz) {
        return Err(ConfigError::BaudTooHigh);
    }
    let fraction = (65_536 * sixteen + u64::from(clock_hz) / 2) / u64::from(clock_hz);
    u16::try_from(65_536 - fraction).map_err(|_| ConfigError::BaudTooHigh)
}

/// `config`'s core clock and BAUD value, if it can be met.
fn validate(config: &Config) -> Result<(u32, u16), ConfigError> {
    let clock_hz = crate::clocks::generator_hz(config.generator).ok_or(ConfigError::NoClock)?;
    let baud = baud_value(clock_hz, config.baudrate)?;
    #[cfg(sercom_v1)]
    if config.fifo {
        return Err(ConfigError::NoFifo);
    }
    Ok((clock_hz, baud))
}

/// The rate `value` gives from `clock_hz`.
fn baud_rate(clock_hz: u32, value: u16) -> u32 {
    let rate = u64::from(clock_hz) * (65_536 - u64::from(value)) / (16 * 65_536);
    u32::try_from(rate).unwrap_or(u32::MAX)
}

pub(crate) struct State {
    pub(crate) tx_waker: AtomicWaker,
    pub(crate) rx_waker: AtomicWaker,
    /// A [`BufferedUart`]'s rings and errors, when one is on the SERCOM.
    pub(crate) buffered: buffered::State,
    /// The SERCOM's core clock, for changing the baud rate from either half.
    clock_hz: AtomicU32,
    /// The SERCOM's GCLK peripheral channel, and its RX pin
    /// ([`NO_PIN`] until a UART is made), so either half can change the
    /// whole configuration.
    pub(crate) gclk_channel: usize,
    rx_pin: AtomicU32,
}

/// [`State::rx_pin`] before a UART is made.
const NO_PIN: u32 = u32::MAX;

/// CTRLA's pad fields (TXPO, RXPO), which the pins and flow control fix:
/// [`State::configure`] keeps them.
const CTRLA_PADS: u32 = (0x3 << 16) | (0x3 << RXPO_SHIFT);

impl State {
    pub(crate) const fn new(gclk_channel: usize) -> Self {
        Self {
            tx_waker: AtomicWaker::new(),
            rx_waker: AtomicWaker::new(),
            buffered: buffered::State::new(),
            clock_hz: AtomicU32::new(0),
            gclk_channel,
            rx_pin: AtomicU32::new(NO_PIN),
        }
    }

    /// Sets the SERCOM up as `config` says, keeping CTRLA's pad fields, the
    /// interrupts enabled and the pins' functions; the RX pin's pull too
    /// once a UART has recorded it. The SERCOM is disabled meanwhile, so a
    /// byte on the wire then is lost, and the FIFO's contents with it.
    ///
    /// # Errors
    ///
    /// If `config` can't be met; nothing is then changed.
    pub(crate) fn configure(&self, base: usize, config: &Config) -> Result<(), ConfigError> {
        let (clock_hz, baud) = validate(config)?;
        crate::clocks::enable_channel(self.gclk_channel, config.generator);
        let old = read32(base, CTRLA);
        write32(base, CTRLA, old & !ENABLE);
        while read32(base, SYNCBUSY) & SYNC_ENABLE != 0 {}
        let form = if config.parity == Parity::None { 0 } else { FORM_PARITY };
        let ctrla = DORD_LSB | form | (old & CTRLA_PADS) | MODE_INTERNAL;
        write32(base, CTRLA, ctrla);
        let sbmode = if config.stop_bits == StopBits::Two { SBMODE_TWO } else { 0 };
        let pmode = if config.parity == Parity::Odd { PMODE_ODD } else { 0 };
        write32(base, CTRLB, RXEN | TXEN | pmode | sbmode);
        while read32(base, SYNCBUSY) & SYNC_CTRLB != 0 {}
        #[cfg(sercom_v6)]
        write32(base, CTRLC, if config.fifo { FIFOEN } else { 0 });
        write16(base, BAUD, baud);
        write32(base, CTRLA, ctrla | ENABLE);
        while read32(base, SYNCBUSY) & SYNC_ENABLE != 0 {}
        self.clock_hz.store(clock_hz, Ordering::Relaxed);
        let rx_pin = self.rx_pin.load(Ordering::Relaxed);
        if let Ok(rx_pin) = u8::try_from(rx_pin) {
            crate::gpio::set_function_pull(&crate::gpio::AnyPin::from_pin_port(rx_pin), config.rx_pull);
        }
        Ok(())
    }

    /// The baud rate BAUD gives now.
    pub(crate) fn baudrate(&self, base: usize) -> u32 {
        baud_rate(self.clock_hz.load(Ordering::Relaxed), read16(base, BAUD))
    }

    /// Changes the baud rate. BAUD is enable-protected, so the SERCOM is
    /// disabled for the write: a byte on the wire then is lost, and the
    /// FIFO's contents with it.
    pub(crate) fn set_baudrate(&self, base: usize, baudrate: u32) -> Result<(), ConfigError> {
        let baud = baud_value(self.clock_hz.load(Ordering::Relaxed), baudrate)?;
        let ctrla = read32(base, CTRLA);
        write32(base, CTRLA, ctrla & !ENABLE);
        while read32(base, SYNCBUSY) & SYNC_ENABLE != 0 {}
        write16(base, BAUD, baud);
        write32(base, CTRLA, ctrla);
        while read32(base, SYNCBUSY) & SYNC_ENABLE != 0 {}
        Ok(())
    }
}

/// A SERCOM.
pub(crate) trait SealedInstance {
    const BASE: usize;
    /// `n`, of `SERCOMn`: which DMA requests are its.
    #[cfg_attr(not(dma), allow(dead_code, reason = "for DMA requests"))]
    const NUMBER: u8;
    /// Its bus clock's bit in MCLK.APBCMASK (`sercom_v1`: off at reset).
    #[cfg(sercom_v1)]
    const APBC: u32;
    fn state() -> &'static State;
}

/// A SERCOM that can be a UART.
#[allow(private_bounds, reason = "sealed: only this crate's SERCOMs")]
pub trait Instance: PeripheralType + SealedInstance + 'static {
    /// DRE's vector, `SERCOMn_0` (`sercom_v1`: the SERCOM's one vector,
    /// `SERCOMn`, as for all four).
    type Dre: TypelevelInterrupt;
    /// TXC's, `SERCOMn_1`.
    type Txc: TypelevelInterrupt;
    /// RXC's, `SERCOMn_2`.
    type Rxc: TypelevelInterrupt;
    /// ERROR's (and RXBRK's), `SERCOMn_56`.
    type Error: TypelevelInterrupt;
}

macro_rules! impl_sercom {
    ($($sercom:ident = $base:literal, number $number:literal, channel $channel:literal, $dre:ident, $txc:ident, $rxc:ident, $error:ident;)*) => {
        $(
            impl $crate::uart::SealedInstance for $crate::peripherals::$sercom {
                const BASE: usize = $base;
                const NUMBER: u8 = $number;
                fn state() -> &'static $crate::uart::State {
                    static STATE: $crate::uart::State = $crate::uart::State::new($channel);
                    &STATE
                }
            }

            impl $crate::uart::Instance for $crate::peripherals::$sercom {
                type Dre = $crate::interrupt::typelevel::$dre;
                type Txc = $crate::interrupt::typelevel::$txc;
                type Rxc = $crate::interrupt::typelevel::$rxc;
                type Error = $crate::interrupt::typelevel::$error;
            }

            $crate::uart::impl_sercom!(@handler $sercom, $dre);
            $crate::uart::impl_sercom!(@handler $sercom, $txc);
            $crate::uart::impl_sercom!(@handler $sercom, $rxc);
            $crate::uart::impl_sercom!(@handler $sercom, $error);
        )*
    };
    // `sercom_v1`: one vector for all four.
    ($($sercom:ident = $base:literal, number $number:literal, channel $channel:literal, apbc $apbc:literal, $line:ident;)*) => {
        $(
            impl $crate::uart::SealedInstance for $crate::peripherals::$sercom {
                const BASE: usize = $base;
                const NUMBER: u8 = $number;
                const APBC: u32 = $apbc;
                fn state() -> &'static $crate::uart::State {
                    static STATE: $crate::uart::State = $crate::uart::State::new($channel);
                    &STATE
                }
            }

            impl $crate::uart::Instance for $crate::peripherals::$sercom {
                type Dre = $crate::interrupt::typelevel::$line;
                type Txc = $crate::interrupt::typelevel::$line;
                type Rxc = $crate::interrupt::typelevel::$line;
                type Error = $crate::interrupt::typelevel::$line;
            }

            $crate::uart::impl_sercom!(@handler $sercom, $line);
        )*
    };
    (@handler $sercom:ident, $line:ident) => {
        impl $crate::interrupt::typelevel::Handler<$crate::interrupt::typelevel::$line> for $crate::uart::InterruptHandler<$crate::peripherals::$sercom> {
            unsafe fn on_interrupt() {
                $crate::uart::on_interrupt::<$crate::peripherals::$sercom>();
            }
        }
    };
}

pub(crate) use impl_sercom;

fn reg32(base: usize, offset: usize) -> *mut u32 {
    (base + offset) as *mut u32
}

fn read8(base: usize, offset: usize) -> u8 {
    // SAFETY: a register of the SERCOM this driver owns.
    unsafe { read_volatile((base + offset) as *const u8) }
}

fn write8(base: usize, offset: usize, value: u8) {
    // SAFETY: as for `read8`.
    unsafe { write_volatile((base + offset) as *mut u8, value) }
}

fn read32(base: usize, offset: usize) -> u32 {
    // SAFETY: as for `read8`.
    unsafe { read_volatile(reg32(base, offset)) }
}

fn write32(base: usize, offset: usize, value: u32) {
    // SAFETY: as for `read8`.
    unsafe { write_volatile(reg32(base, offset), value) }
}

/// DATA, read: 32 bits wide on `sercom_v6`, 16 on `sercom_v1`.
fn read_data(base: usize) -> u32 {
    #[cfg(sercom_v6)]
    return read32(base, DATA);
    #[cfg(sercom_v1)]
    return u32::from(read16(base, DATA));
}

/// DATA, written, likewise.
fn write_data(base: usize, value: u32) {
    #[cfg(sercom_v6)]
    write32(base, DATA, value);
    #[cfg(sercom_v1)]
    #[expect(clippy::cast_possible_truncation, reason = "nine data bits at most")]
    write16(base, DATA, value as u16);
}

fn read16(base: usize, offset: usize) -> u16 {
    // SAFETY: as for `read8`.
    unsafe { read_volatile((base + offset) as *const u16) }
}

fn write16(base: usize, offset: usize, value: u16) {
    // SAFETY: as for `read8`.
    unsafe { write_volatile((base + offset) as *mut u16, value) }
}

/// The interrupt handler for a SERCOM's UART vectors: masks what fired and
/// wakes the waiting side.
pub struct InterruptHandler<T: Instance> {
    _sercom: PhantomData<T>,
}

/// Masks what fired and wakes the waiting side.
pub(crate) fn on_interrupt<T: Instance>() {
    if T::state().buffered.active() {
        buffered::on_interrupt(T::BASE, T::state());
        return;
    }
    let enabled = read8(T::BASE, INTENSET);
    let mut fired = read8(T::BASE, INTFLAG) & enabled;
    // An enabled RXC counts as fired even with its flag gone: a receive DMA
    // (a ring's) reads DATA, clearing RXC, before the handler runs. The
    // reader looks for itself and arms it again.
    fired |= enabled & RXC;
    if fired & (DRE | TXC) != 0 {
        write8(T::BASE, INTENCLR, fired & (DRE | TXC));
        T::state().tx_waker.wake();
    }
    if fired & (RXC | ERROR) != 0 {
        write8(T::BASE, INTENCLR, fired & (RXC | ERROR));
        T::state().rx_waker.wake();
    }
}

pub use crate::mode::{Async, Blocking};

mod buffered;
#[cfg(dma)]
mod ring;
pub use buffered::{BufferedUart, BufferedUartRx, BufferedUartTx};
#[cfg(dma)]
pub use ring::RingBufferedUartRx;

/// A UART on a SERCOM: its transmitter and receiver, which
/// [`split`](Self::split) separates.
pub struct Uart<'d, M> {
    tx: UartTx<'d, M>,
    rx: UartRx<'d, M>,
}

/// A UART's transmitter.
pub struct UartTx<'d, M> {
    base: usize,
    /// `n`, of `SERCOMn`: its DMA requests.
    #[cfg(dma)]
    number: u8,
    state: &'static State,
    /// The transmit channel, if writes go by DMA.
    #[cfg(dma)]
    dma: Option<Peri<'d, AnyChannel>>,
    _lifetime: PhantomData<(&'d (), M)>,
}

/// A UART's receiver.
pub struct UartRx<'d, M> {
    base: usize,
    #[cfg(dma)]
    number: u8,
    state: &'static State,
    /// The receive channel, if reads go by DMA.
    #[cfg(dma)]
    dma: Option<Peri<'d, AnyChannel>>,
    _lifetime: PhantomData<(&'d (), M)>,
}

impl<M> UartTx<'_, M> {
    /// Whether the far end lets this one send: CTS low. Only a UART with
    /// flow control ([`Uart::new_with_rtscts`]) waits on it; on one without,
    /// this reads whatever pad 3's input is.
    #[must_use]
    pub fn clear_to_send(&self) -> bool {
        read16(self.base, STATUS) & CTS == 0
    }
}

impl<'d> Uart<'d, Blocking> {
    /// A UART on `sercom`, TX on `tx` and RX on `rx`, waiting on the flags.
    ///
    /// # Errors
    ///
    /// If [`Config`]'s generator can't give its baud rate.
    #[allow(clippy::needless_pass_by_value, reason = "the SERCOM has the pins for 'd")]
    pub fn new_blocking<T: Instance, const RX: u8, const IOSET: u8>(sercom: Peri<'d, T>, tx: Peri<'d, impl Pad<T, 0, IOSET>>, rx: Peri<'d, impl Pad<T, RX, IOSET>>, config: Config) -> Result<Self, ConfigError> {
        const { check_rx_pad(RX) };
        let _ = sercom;
        Self::start::<T>(&*tx, tx.function(), &*rx, rx.function(), RX, false, None, config)
    }

    /// A UART as [`new_blocking`](Self::new_blocking) with hardware flow
    /// control: RX on pad 1, RTS on `rts` (pad 2), CTS on `cts` (pad 3). The
    /// SERCOM drives RTS high while its receive buffer is full (or the
    /// receiver is off), and sends nothing new while CTS is high.
    ///
    /// # Errors
    ///
    /// As [`new_blocking`](Self::new_blocking).
    #[allow(clippy::needless_pass_by_value, reason = "the SERCOM has the pins for 'd")]
    pub fn new_blocking_with_rtscts<T: Instance, const IOSET: u8>(
        sercom: Peri<'d, T>,
        tx: Peri<'d, impl Pad<T, 0, IOSET>>,
        rx: Peri<'d, impl Pad<T, 1, IOSET>>,
        rts: Peri<'d, impl Pad<T, 2, IOSET>>,
        cts: Peri<'d, impl Pad<T, 3, IOSET>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let _ = sercom;
        let uart = Self::start::<T>(&*tx, tx.function(), &*rx, rx.function(), 1, true, None, config)?;
        set_function(&*rts, rts.function());
        set_function(&*cts, cts.function());
        Ok(uart)
    }
}

#[cfg(dma)]
impl<'d> Uart<'d, Async> {
    /// A UART on `sercom`, TX on `tx` and RX on `rx`, its reads and writes
    /// by DMA on `tx_dma` and `rx_dma` (as Embassy's UARTs take them), and
    /// `irqs` binding its vectors for the rest.
    ///
    /// # Errors
    ///
    /// If [`Config`]'s generator can't give its baud rate.
    #[allow(clippy::needless_pass_by_value, reason = "the SERCOM has the pins for 'd")]
    #[allow(clippy::too_many_arguments, reason = "the SERCOM, its pins, the binding, two channels and the configuration")]
    pub fn new<T: Instance, const RX: u8, const IOSET: u8>(
        sercom: Peri<'d, T>,
        tx: Peri<'d, impl Pad<T, 0, IOSET>>,
        rx: Peri<'d, impl Pad<T, RX, IOSET>>,
        _irqs: impl Binding<T::Dre, InterruptHandler<T>> + Binding<T::Txc, InterruptHandler<T>> + Binding<T::Rxc, InterruptHandler<T>> + Binding<T::Error, InterruptHandler<T>> + 'd,
        tx_dma: Peri<'d, impl crate::dma::Channel>,
        rx_dma: Peri<'d, impl crate::dma::Channel>,
        config: Config,
    ) -> Result<Self, ConfigError>
    where
        InterruptHandler<T>: Handler<T::Dre> + Handler<T::Txc> + Handler<T::Rxc> + Handler<T::Error>,
    {
        let _ = sercom;
        const { check_rx_pad(RX) };
        let uart = Self::start::<T>(&*tx, tx.function(), &*rx, rx.function(), RX, false, Some((tx_dma.into(), rx_dma.into())), config)?;
        enable_interrupts::<T>();
        Ok(uart)
    }

    /// A UART as [`new`](Self::new) with hardware flow control, as
    /// [`Uart::new_blocking_with_rtscts`].
    ///
    /// # Errors
    ///
    /// As [`new`](Self::new).
    #[allow(clippy::needless_pass_by_value, reason = "the SERCOM has the pins for 'd")]
    #[allow(clippy::too_many_arguments, reason = "the SERCOM, its pins, the binding, two channels and the configuration")]
    pub fn new_with_rtscts<T: Instance, const IOSET: u8>(
        sercom: Peri<'d, T>,
        tx: Peri<'d, impl Pad<T, 0, IOSET>>,
        rx: Peri<'d, impl Pad<T, 1, IOSET>>,
        rts: Peri<'d, impl Pad<T, 2, IOSET>>,
        cts: Peri<'d, impl Pad<T, 3, IOSET>>,
        _irqs: impl Binding<T::Dre, InterruptHandler<T>> + Binding<T::Txc, InterruptHandler<T>> + Binding<T::Rxc, InterruptHandler<T>> + Binding<T::Error, InterruptHandler<T>> + 'd,
        tx_dma: Peri<'d, impl crate::dma::Channel>,
        rx_dma: Peri<'d, impl crate::dma::Channel>,
        config: Config,
    ) -> Result<Self, ConfigError>
    where
        InterruptHandler<T>: Handler<T::Dre> + Handler<T::Txc> + Handler<T::Rxc> + Handler<T::Error>,
    {
        let _ = sercom;
        let uart = Self::start::<T>(&*tx, tx.function(), &*rx, rx.function(), 1, true, Some((tx_dma.into(), rx_dma.into())), config)?;
        set_function(&*rts, rts.function());
        set_function(&*cts, cts.function());
        enable_interrupts::<T>();
        Ok(uart)
    }
}

/// Unmasks the SERCOM's four vectors.
fn enable_interrupts<T: Instance>() {
    {
        T::Dre::unpend();
        T::Txc::unpend();
        T::Rxc::unpend();
        T::Error::unpend();
        // SAFETY: the binding makes each vector call the handler.
        unsafe {
            T::Dre::enable();
            T::Txc::enable();
            T::Rxc::enable();
            T::Error::enable();
        }
    }
}

#[cfg(dma)]
impl Uart<'_, Async> {
    /// Writes all of `bytes`, by DMA if the UART has it.
    ///
    /// # Errors
    ///
    /// [`Error::Dma`] if a DMA transfer failed.
    pub async fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.tx.write(bytes).await
    }

    /// Waits until everything written has gone out.
    pub async fn flush(&mut self) {
        self.tx.flush().await;
    }

    /// Reads one byte.
    ///
    /// # Errors
    ///
    /// A receive error; the byte that came with it is lost.
    pub async fn read_byte(&mut self) -> Result<u8, Error> {
        self.rx.read_byte().await
    }

    /// Fills `buffer`.
    ///
    /// # Errors
    ///
    /// The first receive error; what was read before it is in `buffer`.
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        self.rx.read(buffer).await
    }
}

/// A UART's transmit and receive DMA channels (none without a DMA driver).
#[cfg(dma)]
type DmaChannels<'d> = (Peri<'d, AnyChannel>, Peri<'d, AnyChannel>);
#[cfg(not(dma))]
type DmaChannels<'d> = (core::convert::Infallible, PhantomData<&'d ()>);

impl<'d, M> Uart<'d, M> {
    #[allow(clippy::too_many_arguments, reason = "internal, from the constructors")]
    fn start<T: Instance>(tx: &impl SealedPin, tx_function: Function, rx: &impl SealedPin, rx_function: Function, rx_pad: u8, rtscts: bool, dma: Option<DmaChannels<'d>>, config: Config) -> Result<Self, ConfigError> {
        // Nothing is touched if `config` can't be met.
        validate(&config)?;
        let base = T::BASE;
        #[cfg(sercom_v1)]
        crate::clocks::enable_apbc(T::APBC);
        let state = T::state();
        // The reset synchronises on the core clock, so that first.
        crate::clocks::enable_channel(state.gclk_channel, config.generator);
        write32(base, CTRLA, SWRST);
        while read32(base, SYNCBUSY) & SYNC_SWRST != 0 {}
        let txpo = if rtscts { TXPO_RTS_CTS } else { 0 };
        write32(base, CTRLA, txpo | (u32::from(rx_pad) << RXPO_SHIFT));
        state.rx_pin.store(NO_PIN, Ordering::Relaxed);
        state.configure(base, &config)?;
        set_function(tx, tx_function);
        set_function(rx, rx_function);
        crate::gpio::set_function_pull(rx, config.rx_pull);
        state.rx_pin.store(u32::from(rx.pin_port()), Ordering::Relaxed);
        #[cfg(dma)]
        let number = T::NUMBER;
        #[cfg(dma)]
        let (tx_dma, rx_dma) = match dma {
            Some((t, r)) => (Some(t), Some(r)),
            None => (None, None),
        };
        #[cfg(not(dma))]
        let _ = dma;
        Ok(Self {
            tx: UartTx {
                base,
                #[cfg(dma)]
                number,
                state,
                #[cfg(dma)]
                dma: tx_dma,
                _lifetime: PhantomData,
            },
            rx: UartRx {
                base,
                #[cfg(dma)]
                number,
                state,
                #[cfg(dma)]
                dma: rx_dma,
                _lifetime: PhantomData,
            },
        })
    }

    /// The baud rate the BAUD value gives, which may differ a little from
    /// the one asked for.
    #[must_use]
    pub fn baudrate(&self) -> u32 {
        self.tx.state.baudrate(self.tx.base)
    }

    /// The transmitter and the receiver, apart, so each can wait on its own.
    #[must_use]
    pub fn split(self) -> (UartTx<'d, M>, UartRx<'d, M>) {
        (self.tx, self.rx)
    }

    /// Changes the baud rate, for both directions.
    ///
    /// # Errors
    ///
    /// If the SERCOM's clock can't give `baudrate`; the rate is then as it
    /// was.
    pub fn set_baudrate(&mut self, baudrate: u32) -> Result<(), ConfigError> {
        self.tx.state.set_baudrate(self.tx.base, baudrate)
    }

    /// Changes the configuration, for both directions: the baud rate,
    /// parity, stop bits, generator, FIFO and RX pull (embassy-embedded-hal's
    /// `SetConfig`). The pins and flow control stay as the UART was made.
    /// As for [`set_baudrate`](Self::set_baudrate), the SERCOM is disabled
    /// meanwhile, so a byte on the wire then is lost.
    ///
    /// # Errors
    ///
    /// If `config` can't be met; the configuration is then as it was.
    pub fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        self.tx.state.configure(self.tx.base, config)
    }

    /// As [`split`](Self::split), borrowed.
    pub fn split_ref(&mut self) -> (&mut UartTx<'d, M>, &mut UartRx<'d, M>) {
        (&mut self.tx, &mut self.rx)
    }

    /// Sends `byte` if there's room for it; false if not.
    pub fn try_write(&mut self, byte: u8) -> bool {
        self.tx.try_write(byte)
    }

    /// A byte, or its error, if one has come.
    pub fn try_read(&mut self) -> Option<Result<u8, Error>> {
        self.rx.try_read()
    }

    /// Whether everything written has gone out.
    #[must_use]
    pub fn is_flushed(&self) -> bool {
        self.tx.is_flushed()
    }

    /// Writes all of `bytes`, waiting for room.
    pub fn blocking_write(&mut self, bytes: &[u8]) {
        self.tx.blocking_write(bytes);
    }

    /// Waits until everything written has gone out.
    pub fn blocking_flush(&mut self) {
        self.tx.blocking_flush();
    }

    /// Fills `buffer`, waiting for each byte.
    ///
    /// # Errors
    ///
    /// The first receive error.
    pub fn blocking_read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        self.rx.blocking_read(buffer)
    }
}

impl<M> UartTx<'_, M> {
    /// Sends `byte` if there's room for it; false if not.
    pub fn try_write(&mut self, byte: u8) -> bool {
        if read8(self.base, INTFLAG) & DRE == 0 {
            return false;
        }
        write_data(self.base, u32::from(byte));
        true
    }

    /// Whether everything written has gone out: nothing waiting and the
    /// last byte's stop bit sent.
    #[must_use]
    pub fn is_flushed(&self) -> bool {
        let flags = read8(self.base, INTFLAG);
        flags & DRE != 0 && flags & TXC != 0
    }

    /// Writes all of `bytes`, waiting for room.
    pub fn blocking_write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            while !self.try_write(byte) {}
        }
    }

    /// Waits until everything written has gone out.
    pub fn blocking_flush(&mut self) {
        while !self.is_flushed() {}
    }
}

#[cfg(dma)]
impl UartTx<'_, Async> {
    /// Writes all of `bytes`: by DMA if the UART has it (done when the last
    /// byte is in the SERCOM; [`flush`](Self::flush) waits for it to go out),
    /// otherwise a byte at a time on the DRE interrupt.
    ///
    /// # Errors
    ///
    /// [`Error::Dma`] if a DMA transfer failed.
    pub async fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        use crate::dma::{MAX_BLOCK, Request, Transfer};
        if let Some(channel) = self.dma.as_mut() {
            let data = (self.base + DATA) as *mut u8;
            for chunk in bytes.chunks(MAX_BLOCK) {
                // SAFETY: this SERCOM's DATA, a UART making transmit requests.
                let transfer = unsafe { Transfer::new_write(channel.reborrow(), Request::sercom_tx(self.number), chunk, data) };
                transfer.await.map_err(Error::Dma)?;
            }
            return Ok(());
        }
        for &byte in bytes {
            poll_fn(|cx| {
                self.state.tx_waker.register(cx.waker());
                if self.try_write(byte) {
                    Poll::Ready(())
                } else {
                    write8(self.base, INTENSET, DRE);
                    Poll::Pending
                }
            })
            .await;
        }
        Ok(())
    }

    /// Waits until everything written has gone out.
    pub async fn flush(&mut self) {
        poll_fn(|cx| {
            self.state.tx_waker.register(cx.waker());
            if self.is_flushed() {
                Poll::Ready(())
            } else {
                write8(self.base, INTENSET, TXC);
                Poll::Pending
            }
        })
        .await;
    }
}

impl<M> UartRx<'_, M> {
    /// A byte, or its error, if one has come.
    pub fn try_read(&mut self) -> Option<Result<u8, Error>> {
        if read8(self.base, INTFLAG) & RXC == 0 {
            return None;
        }
        let status = read16(self.base, STATUS);
        #[expect(clippy::cast_possible_truncation, reason = "8 data bits")]
        let byte = read_data(self.base) as u8;
        if status & (PERR | FERR | BUFOVF) != 0 {
            write16(self.base, STATUS, status & (PERR | FERR | BUFOVF));
            write8(self.base, INTFLAG, ERROR);
            return Some(Err(if status & BUFOVF != 0 {
                Error::Overrun
            } else if status & FERR != 0 {
                Error::Framing
            } else {
                Error::Parity
            }));
        }
        Some(Ok(byte))
    }

    /// Fills `buffer`, waiting for each byte.
    ///
    /// # Errors
    ///
    /// The first receive error.
    pub fn blocking_read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        for byte in buffer {
            *byte = loop {
                if let Some(result) = self.try_read() {
                    break result?;
                }
            };
        }
        Ok(())
    }

    /// Fills `buffer` from at least one byte, then whatever more has
    /// already come; how many.
    fn read_available(&mut self, buffer: &mut [u8], first: u8) -> Result<usize, Error> {
        buffer[0] = first;
        let mut n = 1;
        while n < buffer.len() {
            match self.try_read() {
                Some(result) => buffer[n] = result?,
                None => break,
            }
            n += 1;
        }
        Ok(n)
    }
}

#[cfg(dma)]
impl UartRx<'_, Async> {
    /// Reads one byte.
    ///
    /// # Errors
    ///
    /// A receive error; the byte that came with it is lost.
    pub async fn read_byte(&mut self) -> Result<u8, Error> {
        poll_fn(|cx| {
            self.state.rx_waker.register(cx.waker());
            if let Some(result) = self.try_read() {
                Poll::Ready(result)
            } else {
                write8(self.base, INTENSET, RXC | ERROR);
                Poll::Pending
            }
        })
        .await
    }

    /// Fills `buffer`. By DMA if the UART has it, the SERCOM's receive
    /// request pacing it: every byte is stored, bad ones included, and the
    /// receive error seen, if any, comes after (overrun before framing before
    /// parity). Otherwise a byte at a time on the RXC interrupt, stopping at
    /// the first error, what came before it in `buffer`.
    ///
    /// # Errors
    ///
    /// A receive error, or [`Error::Dma`] if a DMA transfer failed.
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        use crate::dma::{MAX_BLOCK, Request, Transfer};
        let Some(channel) = self.dma.as_mut() else {
            for byte in buffer {
                *byte = self.read_byte().await?;
            }
            return Ok(());
        };
        let data = (self.base + DATA) as *const u8;
        for chunk in buffer.chunks_mut(MAX_BLOCK) {
            fifo_byte(self.base, self.state).await;
            // SAFETY: this SERCOM's DATA, a UART making receive requests.
            let transfer = unsafe { Transfer::new_read(channel.reborrow(), Request::sercom_rx(self.number), data, chunk) };
            transfer.await.map_err(Error::Dma)?;
        }
        let status = read16(self.base, STATUS) & (PERR | FERR | BUFOVF);
        if status != 0 {
            write16(self.base, STATUS, status);
            write8(self.base, INTFLAG, ERROR);
            return Err(if status & BUFOVF != 0 {
                Error::Overrun
            } else if status & FERR != 0 {
                Error::Framing
            } else {
                Error::Parity
            });
        }
        Ok(())
    }
}

/// With the FIFO on, waits until it holds a byte: a receive DMA started on
/// an empty FIFO takes a stale byte from the FIFO's memory first (seen on
/// the bench), so it must start only once a byte is in, at once if one is,
/// otherwise on the RXC interrupt. With the FIFO off, or none (SERCOM v1),
/// it returns at once.
#[cfg(all(dma, sercom_v6))]
async fn fifo_byte(base: usize, state: &'static State) {
    if read32(base, CTRLC) & FIFOEN == 0 {
        return;
    }
    poll_fn(|cx| {
        state.rx_waker.register(cx.waker());
        if (read16(base, FIFOSPACE) >> RXSPACE_SHIFT) & RXSPACE_MASK != 0 {
            Poll::Ready(())
        } else {
            write8(base, INTENSET, RXC);
            Poll::Pending
        }
    })
    .await;
}

#[cfg(all(dma, sercom_v1))]
async fn fifo_byte(_base: usize, _state: &'static State) {}

impl<M> UartTx<'_, M> {
    /// The baud rate, which the receiver shares.
    #[must_use]
    pub fn baudrate(&self) -> u32 {
        self.state.baudrate(self.base)
    }

    /// Changes the baud rate, for the receiver too: as
    /// [`Uart::set_baudrate`].
    ///
    /// # Errors
    ///
    /// As [`Uart::set_baudrate`].
    pub fn set_baudrate(&mut self, baudrate: u32) -> Result<(), ConfigError> {
        self.state.set_baudrate(self.base, baudrate)
    }
    /// Changes the configuration, for the receiver too: as
    /// [`Uart::set_config`].
    ///
    /// # Errors
    ///
    /// As [`Uart::set_config`].
    pub fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        self.state.configure(self.base, config)
    }
}

impl<M> UartRx<'_, M> {
    /// The baud rate, which the transmitter shares.
    #[must_use]
    pub fn baudrate(&self) -> u32 {
        self.state.baudrate(self.base)
    }

    /// Changes the baud rate, for the transmitter too: as
    /// [`Uart::set_baudrate`].
    ///
    /// # Errors
    ///
    /// As [`Uart::set_baudrate`].
    pub fn set_baudrate(&mut self, baudrate: u32) -> Result<(), ConfigError> {
        self.state.set_baudrate(self.base, baudrate)
    }
    /// Changes the configuration, for the transmitter too: as
    /// [`Uart::set_config`].
    ///
    /// # Errors
    ///
    /// As [`Uart::set_config`].
    pub fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        self.state.configure(self.base, config)
    }
}

/// embassy-embedded-hal's `SetConfig`, for the shared-bus adapters and
/// drivers that reconfigure a UART: each type's `set_config`.
macro_rules! set_config_traits {
    ($($t:ident),*) => {
        $(
            impl<M> embassy_embedded_hal::SetConfig for $t<'_, M> {
                type Config = Config;
                type ConfigError = ConfigError;

                fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
                    $t::set_config(self, config)
                }
            }
        )*
    };
}

set_config_traits!(Uart, UartTx, UartRx);

macro_rules! io_traits {
    ($($t:ident),*) => {
        $(
            impl<M> embedded_io::ErrorType for $t<'_, M> {
                type Error = Error;
            }
        )*
    };
}

io_traits!(Uart, UartTx, UartRx);

impl<M> embedded_io::Write for UartTx<'_, M> {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        self.blocking_write(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.blocking_flush();
        Ok(())
    }
}

impl<M> embedded_io::Write for Uart<'_, M> {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        embedded_io::Write::write(&mut self.tx, bytes)
    }

    fn flush(&mut self) -> Result<(), Error> {
        embedded_io::Write::flush(&mut self.tx)
    }
}

impl<M> embedded_io::Read for UartRx<'_, M> {
    /// At least one byte, then whatever more has already come.
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if buffer.is_empty() {
            return Ok(0);
        }
        let mut first = [0];
        self.blocking_read(&mut first)?;
        self.read_available(buffer, first[0])
    }
}

impl<M> embedded_io::Read for Uart<'_, M> {
    /// At least one byte, then whatever more has already come.
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        embedded_io::Read::read(&mut self.rx, buffer)
    }
}

#[cfg(dma)]
impl embedded_io_async::Write for UartTx<'_, Async> {
    async fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        UartTx::write(self, bytes).await?;
        Ok(bytes.len())
    }

    async fn flush(&mut self) -> Result<(), Error> {
        UartTx::flush(self).await;
        Ok(())
    }
}

#[cfg(dma)]
impl embedded_io_async::Write for Uart<'_, Async> {
    async fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        embedded_io_async::Write::write(&mut self.tx, bytes).await
    }

    async fn flush(&mut self) -> Result<(), Error> {
        embedded_io_async::Write::flush(&mut self.tx).await
    }
}

#[cfg(dma)]
impl embedded_io_async::Read for UartRx<'_, Async> {
    /// At least one byte, then whatever more has already come: on the RXC
    /// interrupt even on a UART with DMA, as a DMA read waits for the whole
    /// buffer.
    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if buffer.is_empty() {
            return Ok(0);
        }
        let first = self.read_byte().await?;
        self.read_available(buffer, first)
    }
}

#[cfg(dma)]
impl embedded_io_async::Read for Uart<'_, Async> {
    /// At least one byte, then whatever more has already come.
    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        embedded_io_async::Read::read(&mut self.rx, buffer).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baud_values() {
        // The bench's VCOM: 115,200 from 48 MHz.
        let value = baud_value(48_000_000, 115_200).unwrap();
        assert_eq!(value, 63_019);
        assert!(baud_rate(48_000_000, value).abs_diff(115_200) < 20);
        for rate in [9_600, 460_800, 921_600, 1_000_000, 3_000_000] {
            let got = baud_rate(48_000_000, baud_value(48_000_000, rate).unwrap());
            // Within 0.5 %: arithmetic BAUD's steps are coarse at low rates.
            assert!(u64::from(got.abs_diff(rate)) * 200 < u64::from(rate), "{rate}: {got}");
        }
        assert_eq!(baud_value(48_000_000, 3_000_001), Err(ConfigError::BaudTooHigh));
        assert_eq!(baud_value(120_000_000, 115_200), Err(ConfigError::ClockTooFast));
        assert_eq!(baud_value(48_000_000, 0), Err(ConfigError::BaudTooHigh));
    }
}

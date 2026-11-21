// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A SERCOM as an I2C host (DS60001795 section 35.8): 7-bit addresses,
//! Standard mode, Fast mode and Fast-mode Plus (100 kHz, 400 kHz, 1 MHz),
//! repeated starts between a transaction's reads and writes.
//!
//! SDA goes on the SERCOM's pad 0 and SCL on pad 1, in one I/O set
//! ([`Pad`](crate::sercom::Pad)). The pins need pull-ups on the board; the
//! driver drives them open drain through the SERCOM.
//!
//! [`I2c::new_blocking`] waits on the flags. [`I2c::new`] takes a
//! [`bind_interrupts!`](crate::bind_interrupts) type binding the SERCOM's
//! `SERCOMn_0` (MB, host on bus) and `SERCOMn_1` (SB, client on bus) vectors
//! to [`InterruptHandler`], and waits on them. Both implement embedded-hal's
//! `I2c`; the interrupt one also embedded-hal-async's. SCL held low for
//! 25 to 35 ms (a stuck device, no pull-ups) gives [`Error::Timeout`] and a
//! reset of the SERCOM; the blocking calls also give up after
//! [`TIMEOUT_POLLS`] polls of a flag.

use core::future::poll_fn;
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};
use core::task::Poll;

use embedded_hal::i2c::{ErrorKind, NoAcknowledgeSource, Operation, SevenBitAddress};

use crate::Peri;
use crate::gpio::{Function, SealedPin, set_function};
use crate::interrupt::typelevel::{Binding, Handler, Interrupt as _};
use crate::mode::{Async, Blocking};
use crate::sercom::{Instance, Pad};

/// The SERCOM's interrupt handler, shared with [`uart`](crate::uart):
/// bind `SERCOMn_0` and `SERCOMn_1` to it.
pub type InterruptHandler<T> = crate::uart::InterruptHandler<T>;

const CTRLA: usize = 0x00;
const CTRLB: usize = 0x04;
const CTRLC: usize = 0x08;
const BAUD: usize = 0x0C;
const INTENSET: usize = 0x16;
const INTFLAG: usize = 0x18;
const STATUS: usize = 0x1A;
const SYNCBUSY: usize = 0x1C;
const ADDR: usize = 0x24;
const DATA: usize = 0x28;

const SWRST: u32 = 1 << 0;
const ENABLE: u32 = 1 << 1;
/// MODE 5: I2C host.
const MODE_HOST: u32 = 0x5 << 2;
/// SDAHOLD 1: 50 to 100 ns.
const SDAHOLD_75NS: u32 = 0x1 << 20;
/// SPEED 1: Fast-mode Plus.
const SPEED_FM_PLUS: u32 = 0x1 << 24;
/// SCL held low 25 to 35 ms: a bus error, with STATUS.LOWTOUT.
const LOWTOUTEN: u32 = 1 << 30;
/// CTRLB.CMD: read the next byte (after the ack action), stop.
const CMD_READ: u32 = 0x2 << 16;
const CMD_STOP: u32 = 0x3 << 16;
const ACKACT_NACK: u32 = 1 << 18;
const SYNC_SWRST: u32 = 1 << 0;
const SYNC_ENABLE: u32 = 1 << 1;
const SYNC_SYSOP: u32 = 1 << 2;

/// Host on bus (a write step done), client on bus (a byte read).
const MB: u8 = 1 << 0;
const SB: u8 = 1 << 1;
const BUSERR: u16 = 1 << 0;
const ARBLOST: u16 = 1 << 1;
const RXNACK: u16 = 1 << 2;
const BUSSTATE_SHIFT: u16 = 4;
const BUSSTATE_IDLE: u16 = 0b01;
const LOWTOUT: u16 = 1 << 6;
const LENERR: u16 = 1 << 10;

/// The SERCOM core clock's highest rate.
const MAX_CORE_HZ: u32 = 100_000_000;
/// Polls of a flag before giving up on the bus: far longer than a byte at
/// 100 kHz takes, at any CPU clock this chip runs.
pub const TIMEOUT_POLLS: u32 = 2_000_000;

/// An I2C host's configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// SCL's rate: up to 1 MHz (Fast-mode Plus).
    pub frequency: u32,
    /// The bus's rise time in nanoseconds, which the SCL period includes:
    /// 100 ns for a short bus with strong pull-ups, up to 300 ns (Fast
    /// mode) or 1,000 ns (Standard mode).
    pub rise_ns: u32,
    /// The generic clock generator to feed the SERCOM from.
    pub generator: usize,
}

impl Default for Config {
    /// 100 kHz, 100 ns rise, from generator 3 (48 MHz in
    /// `clocks::Config::pll_from_xosc`).
    fn default() -> Self {
        Self {
            frequency: 100_000,
            rise_ns: 100,
            generator: 3,
        }
    }
}

/// Why an I2C can't be set up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// The generator isn't configured.
    NoClock,
    /// The generator runs over 100 MHz.
    ClockTooFast,
    /// The frequency is over 1 MHz, or too high or too low for the
    /// generator to divide to.
    Frequency,
}

/// An I2C error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// No device acknowledged the address.
    AddressNack,
    /// The device didn't acknowledge a byte written to it.
    DataNack,
    /// Another host won the bus.
    ArbitrationLost,
    /// A misplaced start or stop on the bus.
    Bus,
    /// The bus didn't move: SCL or SDA held low, or no pull-ups.
    Timeout,
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::AddressNack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Address),
            Self::DataNack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Data),
            Self::ArbitrationLost => ErrorKind::ArbitrationLoss,
            Self::Bus => ErrorKind::Bus,
            Self::Timeout => ErrorKind::Other,
        }
    }
}

/// An I2C host on a SERCOM.
pub struct I2c<'d, M> {
    base: usize,
    state: &'static crate::uart::State,
    gclk_channel: usize,
    ctrla: u32,
    baud: u32,
    frequency: u32,
    _mode: PhantomData<(&'d (), M)>,
}

impl<'d> I2c<'d, Blocking> {
    /// An I2C host on `sercom`, SDA on `sda` (pad 0) and SCL on `scl` (pad
    /// 1), waiting on the flags.
    ///
    /// # Errors
    ///
    /// If [`Config`]'s generator can't give its frequency.
    #[allow(clippy::needless_pass_by_value, reason = "taking them is what makes them the I2C's")]
    pub fn new_blocking<T: Instance, const IOSET: u8>(sercom: Peri<'d, T>, sda: Peri<'d, impl Pad<T, 0, IOSET>>, scl: Peri<'d, impl Pad<T, 1, IOSET>>, config: Config) -> Result<Self, ConfigError> {
        let _ = sercom;
        Self::start::<T>(&*sda, sda.function(), &*scl, scl.function(), config)
    }
}

impl<'d> I2c<'d, Async> {
    /// An I2C host as [`I2c::new_blocking`], waiting on the interrupts
    /// `irqs` binds.
    ///
    /// # Errors
    ///
    /// As [`I2c::new_blocking`].
    #[allow(clippy::needless_pass_by_value, reason = "taking them is what makes them the I2C's")]
    pub fn new<T: Instance, const IOSET: u8>(
        sercom: Peri<'d, T>,
        sda: Peri<'d, impl Pad<T, 0, IOSET>>,
        scl: Peri<'d, impl Pad<T, 1, IOSET>>,
        _irqs: impl Binding<T::Dre, InterruptHandler<T>> + Binding<T::Txc, InterruptHandler<T>> + 'd,
        config: Config,
    ) -> Result<Self, ConfigError>
    where
        InterruptHandler<T>: Handler<T::Dre> + Handler<T::Txc>,
    {
        let _ = sercom;
        let i2c = Self::start::<T>(&*sda, sda.function(), &*scl, scl.function(), config)?;
        T::Dre::unpend();
        T::Txc::unpend();
        // SAFETY: the binding makes each vector call the handler.
        unsafe {
            T::Dre::enable();
            T::Txc::enable();
        }
        Ok(i2c)
    }

    /// Runs `operations` on the device at `address`, as embedded-hal's
    /// `I2c::transaction`, waiting on the interrupts.
    ///
    /// # Errors
    ///
    /// The first error; the bus is then released with a stop.
    pub async fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Error> {
        let result = self.run(address, operations, Wait::Interrupt).await;
        self.finish(result)
    }

    /// Writes `bytes` to the device at `address`.
    ///
    /// # Errors
    ///
    /// As [`transaction`](Self::transaction).
    pub async fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error> {
        self.transaction(address, &mut [Operation::Write(bytes)]).await
    }

    /// Reads `buffer` from the device at `address`.
    ///
    /// # Errors
    ///
    /// As [`transaction`](Self::transaction).
    pub async fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Error> {
        self.transaction(address, &mut [Operation::Read(buffer)]).await
    }

    /// Writes `bytes`, then reads `buffer`, with a repeated start between.
    ///
    /// # Errors
    ///
    /// As [`transaction`](Self::transaction).
    pub async fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        self.transaction(address, &mut [Operation::Write(bytes), Operation::Read(buffer)]).await
    }
}

/// How to wait for a flag.
#[derive(Clone, Copy)]
enum Wait {
    Spin,
    Interrupt,
}

impl<M> I2c<'_, M> {
    fn start<T: Instance>(sda: &impl SealedPin, sda_function: Function, scl: &impl SealedPin, scl_function: Function, config: Config) -> Result<Self, ConfigError> {
        let mut i2c = Self {
            base: T::BASE,
            state: T::state(),
            gclk_channel: T::state().gclk_channel,
            ctrla: 0,
            baud: 0,
            frequency: 0,
            _mode: PhantomData,
        };
        i2c.set_config(&config)?;
        set_function(sda, sda_function);
        set_function(scl, scl_function);
        Ok(i2c)
    }

    /// Changes the configuration (embassy-embedded-hal's `SetConfig`): SCL's
    /// rate, the rise time and the generator. The SERCOM is reset and set up
    /// afresh, the bus idle, so call it between transactions.
    ///
    /// # Errors
    ///
    /// If `config` can't be met; the configuration is then as it was.
    pub fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        let clock_hz = crate::clocks::generator_hz(config.generator).ok_or(ConfigError::NoClock)?;
        if clock_hz > MAX_CORE_HZ {
            return Err(ConfigError::ClockTooFast);
        }
        let (baud, frequency) = baud_value(clock_hz, config.frequency, config.rise_ns).ok_or(ConfigError::Frequency)?;
        self.ctrla = MODE_HOST | SDAHOLD_75NS | LOWTOUTEN | if config.frequency > 400_000 { SPEED_FM_PLUS } else { 0 };
        self.baud = baud;
        self.frequency = frequency;
        crate::clocks::enable_channel(self.gclk_channel, config.generator);
        self.reset();
        Ok(())
    }

    /// Resets and sets the SERCOM up afresh, the bus idle.
    fn reset(&self) {
        write32(self.base, CTRLA, SWRST);
        while read32(self.base, SYNCBUSY) & SYNC_SWRST != 0 {}
        write32(self.base, CTRLA, self.ctrla);
        // Smart mode off: each byte's ack is the driver's command.
        write32(self.base, CTRLB, 0);
        write32(self.base, CTRLC, 0);
        write32(self.base, BAUD, self.baud);
        write32(self.base, CTRLA, self.ctrla | ENABLE);
        while read32(self.base, SYNCBUSY) & SYNC_ENABLE != 0 {}
        write16(self.base, STATUS, BUSSTATE_IDLE << BUSSTATE_SHIFT);
        self.sync_sysop();
    }

    fn sync_sysop(&self) {
        while read32(self.base, SYNCBUSY) & SYNC_SYSOP != 0 {}
    }

    /// SCL's rate, as BAUD gives it with the configured rise time.
    #[must_use]
    pub const fn frequency(&self) -> u32 {
        self.frequency
    }

    /// Runs `operations` on the device at `address`, waiting on the flags.
    ///
    /// # Errors
    ///
    /// The first error; the bus is then released with a stop.
    pub fn blocking_transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Error> {
        let result = embassy_futures::block_on(self.run(address, operations, Wait::Spin));
        self.finish(result)
    }

    /// Writes `bytes` to the device at `address`.
    ///
    /// # Errors
    ///
    /// As [`blocking_transaction`](Self::blocking_transaction).
    pub fn blocking_write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error> {
        self.blocking_transaction(address, &mut [Operation::Write(bytes)])
    }

    /// Reads `buffer` from the device at `address`.
    ///
    /// # Errors
    ///
    /// As [`blocking_transaction`](Self::blocking_transaction).
    pub fn blocking_read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Error> {
        self.blocking_transaction(address, &mut [Operation::Read(buffer)])
    }

    /// Writes `bytes`, then reads `buffer`, with a repeated start between.
    ///
    /// # Errors
    ///
    /// As [`blocking_transaction`](Self::blocking_transaction).
    pub fn blocking_write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        self.blocking_transaction(address, &mut [Operation::Write(bytes), Operation::Read(buffer)])
    }

    /// Releases the bus after a transaction: a stop if it went wrong part
    /// way (one that ended well stopped itself), a reset if it timed out.
    fn finish(&mut self, result: Result<(), Error>) -> Result<(), Error> {
        match result {
            Ok(()) => {}
            Err(Error::Timeout) => self.reset(),
            Err(_) => self.stop(),
        }
        result
    }

    fn stop(&self) {
        write32(self.base, CTRLB, CMD_STOP);
        self.sync_sysop();
    }

    /// Waits for MB or SB, then checks the status.
    async fn wait(&self, wait: Wait) -> Result<u8, Error> {
        let flags = match wait {
            Wait::Spin => {
                let mut polls = 0;
                loop {
                    let flags = read8(self.base, INTFLAG) & (MB | SB);
                    if flags != 0 {
                        break flags;
                    }
                    polls += 1;
                    if polls == TIMEOUT_POLLS {
                        return Err(Error::Timeout);
                    }
                }
            }
            Wait::Interrupt => {
                let base = self.base;
                let state = self.state;
                let wait = poll_fn(|cx| {
                    state.tx_waker.register(cx.waker());
                    let flags = read8(base, INTFLAG) & (MB | SB);
                    if flags == 0 {
                        write8(base, INTENSET, MB | SB);
                        Poll::Pending
                    } else {
                        Poll::Ready(flags)
                    }
                });
                wait.await
            }
        };
        let status = read16(self.base, STATUS);
        if status & LOWTOUT != 0 {
            Err(Error::Timeout)
        } else if status & BUSERR != 0 {
            Err(Error::Bus)
        } else if status & ARBLOST != 0 {
            Err(Error::ArbitrationLost)
        } else if status & LENERR != 0 {
            Err(Error::Bus)
        } else {
            Ok(flags)
        }
    }

    /// Sends a (repeated) start and `address` with the direction, and waits
    /// for its acknowledge.
    async fn address(&self, address: u8, read: bool, wait: Wait) -> Result<(), Error> {
        write32(self.base, ADDR, (u32::from(address) << 1) | u32::from(read));
        self.sync_sysop();
        let flags = self.wait(wait).await?;
        // A write's address ends with MB; a read's with SB once the first
        // byte is in, or MB if no one answered.
        if read16(self.base, STATUS) & RXNACK != 0 || (read && flags & SB == 0) {
            return Err(Error::AddressNack);
        }
        Ok(())
    }

    async fn run(&mut self, address: u8, operations: &mut [Operation<'_>], wait: Wait) -> Result<(), Error> {
        if address > 0x7F {
            return Err(Error::AddressNack);
        }
        let count = operations.len();
        let mut direction: Option<bool> = None;
        for i in 0..count {
            let read = matches!(operations[i], Operation::Read(_));
            // A start (or repeated start) where the direction changes.
            let fresh = direction != Some(read);
            // The last byte of a run of reads is NACKed.
            let last_of_run = operations.get(i + 1).is_none_or(|next| matches!(next, Operation::Read(_)) != read);
            let last = i + 1 == count;
            match &mut operations[i] {
                Operation::Write(bytes) => {
                    if fresh {
                        self.address(address, false, wait).await?;
                    }
                    for &byte in &**bytes {
                        write32(self.base, DATA, u32::from(byte));
                        self.sync_sysop();
                        self.wait(wait).await?;
                        if read16(self.base, STATUS) & RXNACK != 0 {
                            return Err(Error::DataNack);
                        }
                    }
                }
                Operation::Read(buffer) => {
                    if buffer.is_empty() {
                        // A read of nothing can't be expressed on the bus
                        // past its address byte: skip it.
                        continue;
                    }
                    if fresh {
                        self.address(address, true, wait).await?;
                    }
                    let n = buffer.len();
                    for (j, slot) in buffer.iter_mut().enumerate() {
                        // The address's SB means the first byte is in; any
                        // other byte, a continuing read's first included,
                        // comes after the last one's ACK.
                        if j > 0 || !fresh {
                            self.wait(wait).await?;
                        }
                        #[expect(clippy::cast_possible_truncation, reason = "8-bit data")]
                        let byte = read32(self.base, DATA) as u8;
                        *slot = byte;
                        let final_byte = j + 1 == n && last_of_run;
                        if !final_byte {
                            // ACK, and read the next.
                            write32(self.base, CTRLB, CMD_READ);
                            self.sync_sysop();
                        } else if last {
                            // NACK and stop.
                            write32(self.base, CTRLB, ACKACT_NACK | CMD_STOP);
                            self.sync_sysop();
                        } else {
                            // NACK, then the next operation's repeated
                            // start: ACKACT takes effect with it.
                            write32(self.base, CTRLB, ACKACT_NACK);
                            self.sync_sysop();
                        }
                    }
                    direction = Some(true);
                    continue;
                }
            }
            direction = Some(read);
        }
        // A transaction ending in a write (or with nothing read) stops here.
        if direction != Some(true) || matches!(operations.last(), Some(Operation::Write(_))) {
            self.stop();
        }
        // Ready for the next: ACK by default.
        write32(self.base, CTRLB, 0);
        self.sync_sysop();
        Ok(())
    }
}

impl<M> Drop for I2c<'_, M> {
    fn drop(&mut self) {
        write32(self.base, CTRLA, 0);
        while read32(self.base, SYNCBUSY) & SYNC_ENABLE != 0 {}
    }
}

/// BAUD for SCL at `frequency` or under from a `clock_hz` clock with a
/// `rise_ns` rise time, and the rate it gives. Up to 400 kHz SCL is high and
/// low for as long (BAUD alone); above, low twice as long as high, as
/// Fast-mode Plus asks (BAUDLOW).
fn baud_value(clock_hz: u32, frequency: u32, rise_ns: u32) -> Option<(u32, u32)> {
    if frequency == 0 || frequency > 1_000_000 {
        return None;
    }
    let rise = u64::from(clock_hz) * u64::from(rise_ns) / 1_000_000_000;
    // Cycles of the clock in a period, less the fixed 10 and the rise.
    let period = u64::from(clock_hz).div_ceil(u64::from(frequency));
    let total = period.checked_sub(10 + rise)?;
    let (baud, low) = if frequency <= 400_000 {
        let baud = total.div_ceil(2);
        (baud, 0)
    } else {
        // high + low = total, low + 5 = 2 (high + 5).
        let high = total.saturating_sub(5).div_ceil(3);
        (high, total.saturating_sub(high).max(1))
    };
    let (Ok(baud8), Ok(low8)) = (u8::try_from(baud), u8::try_from(low)) else { return None };
    if baud8 == 0 && low8 == 0 {
        return None;
    }
    let cycles = 10 + if low == 0 { 2 * baud } else { baud + low } + rise;
    let actual = u32::try_from(u64::from(clock_hz) / cycles).ok()?;
    Some((u32::from(baud8) | (u32::from(low8) << 8), actual))
}

impl<M> embassy_embedded_hal::SetConfig for I2c<'_, M> {
    type Config = Config;
    type ConfigError = ConfigError;

    fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        I2c::set_config(self, config)
    }
}

impl embedded_hal::i2c::ErrorType for I2c<'_, Blocking> {
    type Error = Error;
}

impl embedded_hal::i2c::I2c<SevenBitAddress> for I2c<'_, Blocking> {
    fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Error> {
        self.blocking_transaction(address, operations)
    }
}

impl embedded_hal::i2c::ErrorType for I2c<'_, Async> {
    type Error = Error;
}

impl embedded_hal::i2c::I2c<SevenBitAddress> for I2c<'_, Async> {
    fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Error> {
        self.blocking_transaction(address, operations)
    }
}

impl embedded_hal_async::i2c::I2c<SevenBitAddress> for I2c<'_, Async> {
    async fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Error> {
        I2c::transaction(self, address, operations).await
    }
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
    fn baud_values() {
        // 48 MHz, 100 ns rise (4.8 cycles, 4): 100 kHz needs 480 cycles,
        // 10 + 4 + 2 * 233.
        assert_eq!(baud_value(48_000_000, 100_000, 100), Some((233, 100_000)));
        // 400 kHz: 120 cycles, BAUD 53.
        let (baud, rate) = baud_value(48_000_000, 400_000, 100).unwrap();
        assert_eq!(baud & 0xFF, 53);
        assert!(rate <= 400_000 && rate > 390_000, "{rate}");
        // 1 MHz: 48 cycles, low about twice high.
        let (baud, rate) = baud_value(48_000_000, 1_000_000, 100).unwrap();
        let (high, low) = (baud & 0xFF, baud >> 8);
        assert!(low + 5 >= 2 * (high + 5) - 2 && low + 5 <= 2 * (high + 5) + 2, "high {high} low {low}");
        assert!(rate <= 1_000_000 && rate > 900_000, "{rate}");
        assert_eq!(baud_value(48_000_000, 2_000_000, 100), None);
        assert_eq!(baud_value(48_000_000, 0, 100), None);
        // Too slow for BAUD's 8 bits.
        assert_eq!(baud_value(100_000_000, 10_000, 100), None);
    }
}

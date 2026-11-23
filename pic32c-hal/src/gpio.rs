// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! General-purpose I/O through PORT (DS60001795 chapter 34): [`Input`],
//! [`Output`], [`OutputOpenDrain`] (by direction where PORT has no open
//! drain) and [`Flex`], each taking a pin singleton
//! (`p.PD20`) or an [`AnyPin`]. Dropping one returns its pin to the reset
//! state: input buffer off, no pull, not driven.
//!
//! On the SG parts PORT has two views, and every pin is Secure out of reset,
//! so this module uses the Secure one (`PORT_SEC`, `0x4480_1000`): the
//! Non-secure one reads 0 and drops writes for Secure pins. The GC parts
//! have only the one view, at `0x4480_0000`. The chip feature picks.
//!
//! [`set_as_function`] hands a pin over with PINCFG = PMUXEN alone, so no
//! pull: a peripheral that drives the pad (a SERCOM, say) controls it, pull
//! included, and an internal pull can't be relied on there. The EIC, which
//! only reads the pin, keeps the pull that [`crate::exti::ExtiInput`] sets
//! (measured on the SG01 and GC01, 2026-11-14).

use core::ptr::{read_volatile, write_volatile};

use crate::{Peri, PeripheralType};

const GROUP_STRIDE: usize = 0x80;
#[cfg(not(port_v4))]
const DIR: usize = 0x00;
const DIRCLR: usize = 0x04;
const DIRSET: usize = 0x08;
const OUT: usize = 0x10;
const OUTCLR: usize = 0x14;
const OUTSET: usize = 0x18;
const OUTTGL: usize = 0x1C;
const IN: usize = 0x20;
const PMUX: usize = 0x30;
const PINCFG: usize = 0x40;

const PMUXEN: u8 = 1 << 0;
const INEN: u8 = 1 << 1;
const PULLEN: u8 = 1 << 2;
#[cfg(port_v4)]
const ODRAIN: u8 = 1 << 3;
#[cfg(not(port_v0))]
const SLEWLIM_SHIFT: u8 = 4;
/// SLEWLIM's width: two bits on `port_v4`, one (on or off) on `port_v1`;
/// `port_v0` has none, but DRVSTR.
#[cfg(port_v4)]
const SLEWLIM_MASK: u8 = 0b11;
#[cfg(port_v1)]
const SLEWLIM_MASK: u8 = 0b1;
#[cfg(port_v0)]
const DRVSTR: u8 = 1 << 6;

/// A pin's level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

impl From<bool> for Level {
    fn from(high: bool) -> Self {
        if high { Self::High } else { Self::Low }
    }
}

impl From<Level> for bool {
    fn from(level: Level) -> Self {
        level == Level::High
    }
}

/// An input's pull resistor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pull {
    None,
    Up,
    Down,
}

/// An output's slew rate (PINCFG.SLEWLIM). Not every pin has slew control
/// (table 34-23); I²C pins must stay [`Slew::Fast`]. Not on `port_v0` (the
/// GV00), which has [`DriveStrength`] instead.
#[cfg(not(port_v0))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slew {
    Fast,
    /// 4x slower.
    Slow4,
    /// 8x slower.
    Slow8,
    /// 12x slower.
    Slow12,
}

/// An output's drive strength (PINCFG.DRVSTR, `port_v0`: the GV00).
#[cfg(port_v0)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriveStrength {
    /// The reset value.
    Normal,
    /// More current, for a heavier load.
    Strong,
}

/// A peripheral function (PMUX), A to N.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Function {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
}

/// A pin: its port group (A is 0) and number.
pub(crate) trait SealedPin {
    fn pin_port(&self) -> u8;

    fn group(&self) -> usize {
        crate::chip::PORT + GROUP_STRIDE * usize::from(self.pin_port() >> 5)
    }

    fn bit(&self) -> u32 {
        1 << (self.pin_port() & 31)
    }

    fn number(&self) -> usize {
        usize::from(self.pin_port() & 31)
    }

    fn set(&self, register: usize) {
        write32(self.group() + register, self.bit());
    }

    fn pincfg(&self) -> usize {
        self.group() + PINCFG + self.number()
    }
}

/// A GPIO pin.
#[allow(private_bounds, reason = "sealed: only this crate's pins")]
pub trait Pin: PeripheralType + Into<AnyPin> + SealedPin + Sized + 'static {
    /// Its port group: 0 for A, 1 for B, …
    fn port(&self) -> u8 {
        self.pin_port() >> 5
    }

    /// Its number in the group.
    fn pin(&self) -> u8 {
        self.pin_port() & 31
    }
}

/// Any pin, its type erased.
#[derive(Clone, Copy)]
pub struct AnyPin {
    pin_port: u8,
}

impl PeripheralType for AnyPin {}

impl AnyPin {
    pub(crate) const fn from_pin_port(pin_port: u8) -> Self {
        Self { pin_port }
    }

    /// The pin `pin` of port group `port` (A is 0).
    ///
    /// # Safety
    ///
    /// Nothing else may use the pin, and it must exist on the package.
    #[must_use]
    pub unsafe fn steal(port: u8, pin: u8) -> Peri<'static, Self> {
        // SAFETY: as for this function.
        unsafe { Peri::new_unchecked(Self { pin_port: (port << 5) | (pin & 31) }) }
    }
}

impl SealedPin for AnyPin {
    fn pin_port(&self) -> u8 {
        self.pin_port
    }
}

impl Pin for AnyPin {}

macro_rules! impl_pin {
    ($($name:ident = $port:literal, $pin:literal;)*) => {
        $(
            impl $crate::gpio::SealedPin for $crate::peripherals::$name {
                fn pin_port(&self) -> u8 {
                    ($port << 5) | $pin
                }
            }

            impl $crate::gpio::Pin for $crate::peripherals::$name {}

            impl From<$crate::peripherals::$name> for $crate::gpio::AnyPin {
                fn from(_pin: $crate::peripherals::$name) -> Self {
                    Self::from_pin_port(($port << 5) | $pin)
                }
            }
        )*
    };
}

pub(crate) use impl_pin;

fn write32(address: usize, value: u32) {
    // SAFETY: a PORT register; the bit written belongs to a pin this
    // module owns through its singleton.
    unsafe { write_volatile(address as *mut u32, value) }
}

fn read32(address: usize) -> u32 {
    // SAFETY: a PORT register, read whole.
    unsafe { read_volatile(address as *const u32) }
}

fn write8(address: usize, value: u8) {
    // SAFETY: a PINCFG or PMUX byte of a pin this module owns.
    unsafe { write_volatile(address as *mut u8, value) }
}

fn read8(address: usize) -> u8 {
    // SAFETY: a PINCFG or PMUX byte.
    unsafe { read_volatile(address as *const u8) }
}

/// A pin that can be an input, an output or a peripheral's.
pub struct Flex<'d> {
    pin: Peri<'d, AnyPin>,
}

impl<'d> Flex<'d> {
    /// The pin, left as it is (in the reset state, unless something set it
    /// up before).
    #[must_use]
    pub fn new(pin: Peri<'d, impl Pin>) -> Self {
        Self { pin: pin.into() }
    }

    /// An input, with its input buffer on and `pull`.
    pub fn set_as_input(&mut self, pull: Pull) {
        let pin = &*self.pin;
        pin.set(DIRCLR);
        match pull {
            Pull::None => write8(pin.pincfg(), INEN),
            Pull::Up | Pull::Down => {
                // OUT picks the pull's direction.
                pin.set(if pull == Pull::Up { OUTSET } else { OUTCLR });
                write8(pin.pincfg(), INEN | PULLEN);
            }
        }
    }

    /// A push-pull output, its input buffer on so [`Self::is_high`] reads
    /// the pin. Set the level first to avoid a glitch.
    pub fn set_as_output(&mut self) {
        write8(self.pin.pincfg(), INEN);
        self.pin.set(DIRSET);
    }

    /// An open-drain output: low drives low, high lets go.
    #[cfg(port_v4)]
    pub fn set_as_output_open_drain(&mut self) {
        write8(self.pin.pincfg(), INEN | ODRAIN);
        self.pin.set(DIRSET);
    }

    /// The pin's slew rate, as an output.
    #[cfg(not(port_v0))]
    pub fn set_slew(&mut self, slew: Slew) {
        set_slew(&*self.pin, slew);
    }

    /// The pin's drive strength, as an output, keeping its other settings.
    #[cfg(port_v0)]
    pub fn set_drive_strength(&mut self, strength: DriveStrength) {
        let address = self.pin.pincfg();
        let pincfg = read8(address) & !DRVSTR;
        write8(address, if strength == DriveStrength::Strong { pincfg | DRVSTR } else { pincfg });
    }

    /// Handed to peripheral function `function`.
    pub fn set_as_function(&mut self, function: Function) {
        set_function(&*self.pin, function);
    }

    /// Handed to peripheral function `function`, keeping the input buffer
    /// and pull as [`Self::set_as_input`] set them.
    #[cfg_attr(not(feature = "exti"), allow(dead_code, reason = "only the EIC needs it yet"))]
    pub(crate) fn set_as_function_keeping_input(&mut self, function: Function) {
        let pin = &*self.pin;
        let address = pin.group() + PMUX + pin.number() / 2;
        let shift = if pin.number().is_multiple_of(2) { 0 } else { 4 };
        critical_section::with(|_| {
            let pmux = read8(address) & !(0xF << shift);
            write8(address, pmux | ((function as u8) << shift));
        });
        write8(pin.pincfg(), read8(pin.pincfg()) | PMUXEN);
    }

    /// Back to the reset state: input buffer off, no pull, not driven.
    pub fn set_as_disconnected(&mut self) {
        let pin = &*self.pin;
        pin.set(DIRCLR);
        write8(pin.pincfg(), 0);
        pin.set(OUTCLR);
    }

    /// The pin's level, as read through its input buffer.
    #[must_use]
    pub fn is_high(&self) -> bool {
        read32(self.pin.group() + IN) & self.pin.bit() != 0
    }

    /// Not [`Self::is_high`].
    #[must_use]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    /// The pin's level.
    #[must_use]
    pub fn get_level(&self) -> Level {
        self.is_high().into()
    }

    /// Drives it high (or, open drain, lets go).
    pub fn set_high(&mut self) {
        self.pin.set(OUTSET);
    }

    /// Drives it low.
    pub fn set_low(&mut self) {
        self.pin.set(OUTCLR);
    }

    /// Drives it to `level`.
    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::High => self.set_high(),
            Level::Low => self.set_low(),
        }
    }

    /// Toggles the output.
    pub fn toggle(&mut self) {
        self.pin.set(OUTTGL);
    }

    /// Whether the output is set high.
    #[must_use]
    pub fn is_set_high(&self) -> bool {
        read32(self.pin.group() + OUT) & self.pin.bit() != 0
    }

    /// Not [`Self::is_set_high`].
    #[must_use]
    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }
}

impl Drop for Flex<'_> {
    fn drop(&mut self) {
        self.set_as_disconnected();
    }
}

/// Gives `pin` to peripheral function `function` for good (for as long as
/// `'d`), for a peripheral this HAL doesn't drive yet. Drivers that take
/// their pins do this themselves.
#[allow(clippy::needless_pass_by_value, reason = "the peripheral has the pin from now on")]
pub fn set_as_function(pin: Peri<'_, impl Pin>, function: Function) {
    set_function(&*pin, function);
}

/// Gives `pin` to peripheral function `function`: PMUX, then PMUXEN (and
/// the input buffer off; the peripheral has its own).
pub(crate) fn set_function(pin: &impl SealedPin, function: Function) {
    let address = pin.group() + PMUX + pin.number() / 2;
    let shift = if pin.number().is_multiple_of(2) { 0 } else { 4 };
    critical_section::with(|_| {
        let pmux = read8(address) & !(0xF << shift);
        write8(address, pmux | ((function as u8) << shift));
    });
    write8(pin.pincfg(), PMUXEN);
}

/// Gives `pin`, already on a peripheral function, a pull (PINCFG.PULLEN,
/// OUT for its direction), keeping its other settings. Whether the pull
/// acts depends on the chip: on the PIC32CK a pin on a peripheral function
/// takes no pull (its PORT disables it with PMUXEN).
pub(crate) fn set_function_pull(pin: &impl SealedPin, pull: Pull) {
    let pincfg = read8(pin.pincfg()) & !PULLEN;
    match pull {
        Pull::None => write8(pin.pincfg(), pincfg),
        Pull::Up | Pull::Down => {
            pin.set(if pull == Pull::Up { OUTSET } else { OUTCLR });
            write8(pin.pincfg(), pincfg | PULLEN);
        }
    }
}

/// Sets `pin`'s slew limit (PINCFG.SLEWLIM), keeping its other settings:
/// for drivers whose pins are a peripheral's.
#[cfg(not(port_v0))]
pub(crate) fn set_slew(pin: &impl SealedPin, slew: Slew) {
    let address = pin.pincfg();
    // `port_v1` has only on or off: any slower setting is on.
    let value = (slew as u8).min(SLEWLIM_MASK);
    let pincfg = read8(address) & !(SLEWLIM_MASK << SLEWLIM_SHIFT);
    write8(address, pincfg | (value << SLEWLIM_SHIFT));
}

/// An input.
pub struct Input<'d> {
    pin: Flex<'d>,
}

impl<'d> Input<'d> {
    /// `pin` as an input with `pull`.
    #[must_use]
    pub fn new(pin: Peri<'d, impl Pin>, pull: Pull) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_as_input(pull);
        Self { pin }
    }

    #[must_use]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    #[must_use]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    #[must_use]
    pub fn get_level(&self) -> Level {
        self.pin.get_level()
    }
}

/// A push-pull output.
pub struct Output<'d> {
    pin: Flex<'d>,
}

impl<'d> Output<'d> {
    /// `pin` driven at `level`.
    #[must_use]
    pub fn new(pin: Peri<'d, impl Pin>, level: Level) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_level(level);
        pin.set_as_output();
        Self { pin }
    }

    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level);
    }

    pub fn toggle(&mut self) {
        self.pin.toggle();
    }

    #[must_use]
    pub fn is_set_high(&self) -> bool {
        self.pin.is_set_high()
    }

    #[must_use]
    pub fn is_set_low(&self) -> bool {
        self.pin.is_set_low()
    }

    /// The pin's slew rate.
    #[cfg(not(port_v0))]
    pub fn set_slew(&mut self, slew: Slew) {
        self.pin.set_slew(slew);
    }

    /// The pin's drive strength.
    #[cfg(port_v0)]
    pub fn set_drive_strength(&mut self, strength: DriveStrength) {
        self.pin.set_drive_strength(strength);
    }
}

/// An open-drain output: low drives low, high lets go, so the pin reads
/// what the line holds (another driver may hold it low).
///
/// On `port_v4` (the PIC32CK, the SG00) PORT does it (PINCFG.ODRAIN). The
/// PL10's and the GV00's PORT have no open drain, so there it's done by
/// direction: low is OUT 0 then DIR set (the pin drives the 0), high is DIR
/// clear then OUT 1 (the pin an input again, its pull, if any, up). In that
/// order the pin never drives high, even for an instant, and each step is
/// one store to a set or clear register (no read-modify-write).
pub struct OutputOpenDrain<'d> {
    pin: Flex<'d>,
}

impl<'d> OutputOpenDrain<'d> {
    /// `pin` open drain, at `level`, with no pull: the line needs its own.
    #[must_use]
    pub fn new(pin: Peri<'d, impl Pin>, level: Level) -> Self {
        #[cfg(port_v4)]
        {
            let mut pin = Flex::new(pin);
            pin.set_level(level);
            pin.set_as_output_open_drain();
            Self { pin }
        }
        #[cfg(not(port_v4))]
        {
            let pin = Flex::new(pin);
            pin.pin.set(DIRCLR);
            write8(pin.pin.pincfg(), INEN);
            let mut pin = Self { pin };
            pin.set_level(level);
            pin
        }
    }

    /// `pin` open drain, at `level`, with the PORT's pull-up while let go
    /// (weak: about 40 kΩ; I²C wants a stronger one on the line). Not on
    /// `port_v4`, whose open drain is PORT's own.
    #[cfg(not(port_v4))]
    #[must_use]
    pub fn new_with_pull_up(pin: Peri<'d, impl Pin>, level: Level) -> Self {
        let pin = Flex::new(pin);
        pin.pin.set(DIRCLR);
        // PULLEN acts only while the pin is an input, upward while OUT is 1.
        write8(pin.pin.pincfg(), INEN | PULLEN);
        let mut pin = Self { pin };
        pin.set_level(level);
        pin
    }

    /// Lets the line go.
    pub fn set_high(&mut self) {
        #[cfg(port_v4)]
        self.pin.set_high();
        #[cfg(not(port_v4))]
        {
            self.pin.pin.set(DIRCLR);
            self.pin.pin.set(OUTSET);
        }
    }

    /// Drives the line low.
    pub fn set_low(&mut self) {
        #[cfg(port_v4)]
        self.pin.set_low();
        #[cfg(not(port_v4))]
        {
            self.pin.pin.set(OUTCLR);
            self.pin.pin.set(DIRSET);
        }
    }

    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::High => self.set_high(),
            Level::Low => self.set_low(),
        }
    }

    pub fn toggle(&mut self) {
        #[cfg(port_v4)]
        self.pin.toggle();
        #[cfg(not(port_v4))]
        if self.is_set_high() {
            self.set_low();
        } else {
            self.set_high();
        }
    }

    /// Whether the line is let go (set high).
    #[must_use]
    pub fn is_set_high(&self) -> bool {
        #[cfg(port_v4)]
        {
            self.pin.is_set_high()
        }
        #[cfg(not(port_v4))]
        {
            read32(self.pin.pin.group() + DIR) & self.pin.pin.bit() == 0
        }
    }

    #[must_use]
    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    /// The line's level, which another driver may hold low.
    #[must_use]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    #[must_use]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    #[must_use]
    pub fn get_level(&self) -> Level {
        self.pin.get_level()
    }
}

mod embedded_hal_1 {
    use core::convert::Infallible;

    use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

    use super::{Flex, Input, Output, OutputOpenDrain};

    macro_rules! error_type {
        ($($t:ident),*) => {
            $(
                impl ErrorType for $t<'_> {
                    type Error = Infallible;
                }
            )*
        };
    }

    error_type!(Flex, Input, Output);
    error_type!(OutputOpenDrain);

    macro_rules! input {
        ($($t:ident),*) => {
            $(
                impl InputPin for $t<'_> {
                    fn is_high(&mut self) -> Result<bool, Infallible> {
                        Ok($t::is_high(self))
                    }

                    fn is_low(&mut self) -> Result<bool, Infallible> {
                        Ok($t::is_low(self))
                    }
                }
            )*
        };
    }

    input!(Flex, Input);
    input!(OutputOpenDrain);

    macro_rules! output {
        ($($t:ident),*) => {
            $(
                impl OutputPin for $t<'_> {
                    fn set_high(&mut self) -> Result<(), Infallible> {
                        $t::set_high(self);
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Infallible> {
                        $t::set_low(self);
                        Ok(())
                    }
                }

                impl StatefulOutputPin for $t<'_> {
                    fn is_set_high(&mut self) -> Result<bool, Infallible> {
                        Ok($t::is_set_high(self))
                    }

                    fn is_set_low(&mut self) -> Result<bool, Infallible> {
                        Ok(!$t::is_set_high(self))
                    }

                    fn toggle(&mut self) -> Result<(), Infallible> {
                        $t::toggle(self);
                        Ok(())
                    }
                }
            )*
        };
    }

    output!(Flex, Output);
    output!(OutputOpenDrain);
}

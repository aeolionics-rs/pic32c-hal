// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The chip's side of a channel: which controllers there are, where their
//! registers are, their kernel clock channels and interrupt lines, and which
//! pins can carry them.

use core::cell::Cell;
use core::marker::PhantomData;

use critical_section::Mutex;

use crate::PeripheralType;
use crate::interrupt::typelevel::Interrupt as TypelevelInterrupt;

/// What a controller's interrupt handler calls: the [`State`](super::State)
/// of the channel on it, whatever its clock and queue length.
pub(crate) trait Handle: Sync {
    fn on_interrupt(&self);
}

/// The channel's state a controller's interrupt goes to, once a channel is
/// made on it.
pub(crate) type Slot = Mutex<Cell<Option<&'static dyn Handle>>>;

/// An `M_CAN` controller.
pub(crate) trait SealedInstance {
    /// Its register block.
    const REGISTERS: usize;
    /// Its kernel clock's GCLK peripheral channel.
    const GCLK_CHANNEL: usize;
    fn slot() -> &'static Slot;
}

/// A CAN controller: CAN0 or CAN1.
#[allow(private_bounds, reason = "sealed: only this crate's controllers")]
pub trait Instance: PeripheralType + SealedInstance + 'static {
    /// Its interrupt line 0, where the channel routes all its interrupts.
    type Interrupt: TypelevelInterrupt;
}

/// A pin that can be `T`'s TX, in its I/O set `IOSET` (TX and RX must be
/// in one).
#[allow(private_bounds, reason = "sealed")]
pub trait TxPin<T: Instance, const IOSET: u8>: crate::gpio::Pin {
    #[doc(hidden)]
    fn function(&self) -> crate::gpio::Function;
}

/// A pin that can be `T`'s RX, in its I/O set `IOSET`.
#[allow(private_bounds, reason = "sealed")]
pub trait RxPin<T: Instance, const IOSET: u8>: crate::gpio::Pin {
    #[doc(hidden)]
    fn function(&self) -> crate::gpio::Function;
}

/// The interrupt handler for a controller's line 0: bind it with
/// [`bind_interrupts!`](crate::bind_interrupts).
pub struct InterruptHandler<T: Instance> {
    _can: PhantomData<T>,
}

/// Hands the interrupt to the channel on `T`, if there is one.
pub(crate) fn on_interrupt<T: Instance>() {
    if let Some(state) = critical_section::with(|cs| T::slot().borrow(cs).get()) {
        state.on_interrupt();
    }
}

macro_rules! impl_can {
    ($($can:ident = $base:literal, channel $channel:literal, $line:ident;)*) => {
        $(
            impl $crate::can::instance::SealedInstance for $crate::peripherals::$can {
                const REGISTERS: usize = $base;
                const GCLK_CHANNEL: usize = $channel;
                fn slot() -> &'static $crate::can::instance::Slot {
                    static SLOT: $crate::can::instance::Slot = ::critical_section::Mutex::new(::core::cell::Cell::new(None));
                    &SLOT
                }
            }

            impl $crate::can::Instance for $crate::peripherals::$can {
                type Interrupt = $crate::interrupt::typelevel::$line;
            }

            impl $crate::interrupt::typelevel::Handler<$crate::interrupt::typelevel::$line> for $crate::can::InterruptHandler<$crate::peripherals::$can> {
                unsafe fn on_interrupt() {
                    $crate::can::instance::on_interrupt::<$crate::peripherals::$can>();
                }
            }
        )*
    };
}

/// Each controller's TX and RX pins: pin, function, I/O set.
macro_rules! impl_can_pins {
    ($($can:ident: $($kind:ident $pin:ident $function:ident ioset $ioset:literal),*;)*) => {
        $(
            $(
                $crate::can::instance::impl_can_pins!(@pin $kind $can $pin $function $ioset);
            )*
        )*
    };
    (@pin tx $can:ident $pin:ident $function:ident $ioset:literal) => {
        impl $crate::can::TxPin<$crate::peripherals::$can, $ioset> for $crate::peripherals::$pin {
            fn function(&self) -> $crate::gpio::Function {
                $crate::gpio::Function::$function
            }
        }
    };
    (@pin rx $can:ident $pin:ident $function:ident $ioset:literal) => {
        impl $crate::can::RxPin<$crate::peripherals::$can, $ioset> for $crate::peripherals::$pin {
            fn function(&self) -> $crate::gpio::Function {
                $crate::gpio::Function::$function
            }
        }
    };
}

pub(crate) use {impl_can, impl_can_pins};

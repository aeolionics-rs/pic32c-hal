// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! What the SERCOM drivers ([`uart`](crate::uart), [`spi`](crate::spi),
//! [`i2c`](crate::i2c)) share: the SERCOM [`Instance`]s and the [`Pad`]s
//! their pins can be.

use crate::gpio::{self, Function};
pub use crate::uart::Instance;

/// A pin that can be pad `PAD` of SERCOM `T`, in its I/O set `IOSET`, from
/// each chip's pin multiplexing table. A SERCOM's signals must all come from
/// one I/O set (DS60001795 section 6: they "cannot be mixed"), so a driver
/// takes all its pins with one `IOSET`, which the compiler infers from them
/// and refuses if they differ. A pin can be two pads of one SERCOM on two
/// pin functions (PB09 is SERCOM4's pad 0 in set 1 and pad 1 in set 4), so
/// the pad is a parameter too.
///
/// SERCOM5's TX and RX on PA24 and PA25, both in set 3:
///
/// ```no_run
/// # use pic32c_hal::uart::{Config, Uart};
/// let p = pic32c_hal::init(pic32c_hal::Config::default());
/// let uart = Uart::new_blocking(p.SERCOM5, p.PA24, p.PA25, Config::default());
/// ```
///
/// TX on PA24 (set 3) with RX on PA12 (set 1) doesn't compile:
///
/// ```compile_fail
/// # use pic32c_hal::uart::{Config, Uart};
/// let p = pic32c_hal::init(pic32c_hal::Config::default());
/// let uart = Uart::new_blocking(p.SERCOM5, p.PA24, p.PA12, Config::default());
/// ```
pub trait Pad<T: Instance, const PAD: u8, const IOSET: u8>: gpio::Pin {
    #[doc(hidden)]
    fn function(&self) -> Function;
}

/// Each SERCOM's pads: pin, pad, function, I/O set.
macro_rules! impl_pads {
    ($($sercom:ident: $($pin:ident pad $pad:literal $function:ident ioset $ioset:literal),*;)*) => {
        $(
            $(
                impl $crate::sercom::Pad<$crate::peripherals::$sercom, $pad, $ioset> for $crate::peripherals::$pin {
                    fn function(&self) -> $crate::gpio::Function {
                        $crate::gpio::Function::$function
                    }
                }
            )*
        )*
    };
}

pub(crate) use impl_pads;

// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Drivers for Microchip's PIC32C microcontrollers, so far the PIC32CK SG
//! and GC (DS60001795) and, in part, the PIC32CM PL10 (DS40002667: clocks,
//! GPIO, the UART and the time driver), in Embassy's style. As in
//! embassy-stm32, `build.rs` turns the chip feature into cfgs naming its
//! peripherals' IP versions (`sercom_v6`, `sercom_v1`, `port_v4`, ...), and
//! the drivers and modules follow those, not the chip:
//! - [`init`] returns the [`Peripherals`], one singleton per peripheral,
//!   which drivers take as [`Peri`]s;
//! - drivers that need interrupts take a type made by [`bind_interrupts!`],
//!   which checks at compile time that the vectors call their handlers;
//! - [`adc`]: the 12-bit ADC, one read at a time, blocking or async, from
//!   the pins, the bandgap and the temperature sensor;
//! - [`cache`]: the L1 cache in front of flash, on by default;
//! - [`eth`] (feature `eth`): the Ethernet MAC, as an embassy-net driver;
//! - [`gpio`]: inputs and outputs, with the embedded-hal 1.0 traits;
//! - [`exti`] (feature `exti`): waiting on pin levels and edges through the
//!   EIC, with embedded-hal-async's `Wait`;
//! - [`flash`]: program flash erase and write, as an `embedded-storage`
//!   `NorFlash`, blocking or async (`embedded-storage-async`);
//! - [`mode`]: the [`Blocking`](mode::Blocking) and [`Async`](mode::Async)
//!   driver modes;
//! - [`trng`]: the true random number generator, with the `rand_core`
//!   0.6, 0.9 and 0.10 traits;
//! - [`wdt`]: the watchdog, and why the chip last reset;
//! - [`i2c`]: a SERCOM as an I2C host, blocking or on interrupts, with
//!   embedded-hal and embedded-hal-async;
//! - [`rtc`]: the real-time counter as a clock and calendar, kept through
//!   resets, and an alarm;
//! - [`spi`]: a SERCOM as an SPI host, blocking or by DMA, with
//!   embedded-hal and embedded-hal-async;
//! - [`uart`]: a SERCOM as a UART, blocking or async, with embedded-io;
//! - [`uid`]: the chip's unique ID;
//! - [`time`] (feature `time-driver`): an embassy-time driver on TCC0, or
//!   the counter a `time-driver-*` feature names, started by [`init`], which
//!   then keeps that counter out of the [`Peripherals`];
//! - [`usb`] (feature `usb`): an embassy-usb driver for the full-speed USB
//!   controller;
//! - [`usbhs`] (feature `usbhs`): an embassy-usb driver for the high-speed
//!   USB controller.
//!
//! Choose the chip with one feature: `pic32ck2051sg01144` or
//! `pic32ck2051gc01144`, each with its own PAC (the GC's lacks the Secure
//! views of PORT and the EIC, and the HSM). Feature `rt`
//! brings in the PAC's vector table and lets the HAL define the time
//! driver's handlers. [`init`] sets the [`clocks`] up from its [`Config`],
//! and the drivers' rates follow from it.
//!
//! - [`dma`]: the DMA controllers' 12 channels, memory to memory and
//!   peripheral-paced transfers as futures;
//! - [`can`] (feature `can-hal`): CAN0 and CAN1, `M_CAN` controllers, as
//!   can-hal channels with confirmed transmission, CAN FD and live filters.

#![cfg_attr(not(test), no_std)]

#[cfg(not(any(feature = "pic32ck2051sg01144", feature = "pic32ck2051gc01144", feature = "pic32cm6408pl10048", feature = "pic32cm5112sg00100")))]
compile_error!("choose the chip: feature `pic32ck2051sg01144`, `pic32ck2051gc01144`, `pic32cm6408pl10048` or `pic32cm5112sg00100`");
#[cfg(any(
    all(feature = "pic32ck2051sg01144", feature = "pic32ck2051gc01144"),
    all(feature = "pic32ck2051sg01144", feature = "pic32cm6408pl10048"),
    all(feature = "pic32ck2051gc01144", feature = "pic32cm6408pl10048"),
    all(feature = "pic32cm5112sg00100", any(feature = "pic32ck2051sg01144", feature = "pic32ck2051gc01144", feature = "pic32cm6408pl10048"))
))]
compile_error!("choose one chip");

#[cfg_attr(pic32ck, path = "chip/pic32ck2051_144.rs")]
#[cfg_attr(pic32cm_pl, path = "chip/pic32cm6408pl10048.rs")]
#[cfg_attr(pic32cm_sg, path = "chip/pic32cm5112sg00100.rs")]
#[allow(clippy::missing_safety_doc, clippy::expl_impl_clone_on_copy, reason = "in code embassy-hal-internal's macros generate")]
mod chip;

#[cfg(adc_msh)]
pub mod adc;
#[cfg(cmcc)]
pub mod cache;
#[cfg(all(feature = "can-hal", mcan))]
pub mod can;
#[cfg_attr(any(pic32ck, pic32cm_sg), path = "clocks.rs")]
#[cfg_attr(pic32cm_pl, path = "clocks_pl.rs")]
pub mod clocks;
#[cfg(dma_axi)]
pub mod dma;
#[cfg(all(feature = "eth", gmac))]
pub mod eth;
#[cfg(all(feature = "exti", eic_v1))]
pub mod exti;
#[cfg(fcw)]
pub mod flash;
pub mod gpio;
#[cfg(sercom_v6)]
pub mod i2c;
pub mod mode;
#[cfg(rtc_v3)]
pub mod rtc;
pub mod sercom;
// Its async half is DMA-driven, so it waits for the SG00's DMAC.
#[cfg(all(sercom_v6, dma_axi))]
pub mod spi;
#[cfg(feature = "time-driver")]
pub mod time;
#[cfg(trng)]
pub mod trng;
pub mod uart;
#[cfg(any(pic32ck, pic32cm_sg))]
pub mod uid;
#[cfg(all(feature = "usb", usb_fs))]
pub mod usb;
#[cfg(all(feature = "usbhs", usb_hs))]
pub mod usbhs;
#[cfg(wdt_v2)]
pub mod wdt;

/// The chip's peripheral access crate, for what the HAL doesn't cover yet.
/// Not part of the HAL's stable API.
#[cfg(feature = "unstable-pac")]
pub use chip::pac;
#[cfg(not(feature = "unstable-pac"))]
pub(crate) use chip::pac;
pub use chip::{FLASH_SIZE, Peripherals, interrupt, peripherals};
pub use embassy_hal_internal::{Peri, PeripheralType};

/// Binds interrupts to the HAL's handlers, and makes the type drivers take
/// to prove it:
///
/// ```ignore
/// pic32c_hal::bind_interrupts!(struct Irqs {
///     USB_OTHER => pic32c_hal::usb::InterruptHandler;
/// });
/// ```
///
/// Each line defines that interrupt's vector, calling each handler named.
#[macro_export]
macro_rules! bind_interrupts {
    ($(#[$outer:meta])* $vis:vis struct $name:ident {
        $(
            $(#[doc = $doc:literal])*
            $(#[cfg($cond_irq:meta)])?
            $irq:ident => $(
                $(#[cfg($cond_handler:meta)])?
                $handler:ty
            ),*;
        )*
    }) => {
        #[derive(Copy, Clone)]
        $(#[$outer])*
        $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            $(#[cfg($cond_irq)])?
            $(#[doc = $doc])*
            unsafe extern "C" fn $irq() {
                unsafe {
                    $(
                        $(#[cfg($cond_handler)])?
                        <$handler as $crate::interrupt::typelevel::Handler<$crate::interrupt::typelevel::$irq>>::on_interrupt();
                    )*
                }
            }

            $(#[cfg($cond_irq)])?
            $crate::bind_interrupts!(@inner
                $(
                    $(#[cfg($cond_handler)])?
                    unsafe impl $crate::interrupt::typelevel::Binding<$crate::interrupt::typelevel::$irq, $handler> for $name {}
                )*
            );
        )*
    };
    (@inner $($t:tt)*) => {
        $($t)*
    }
}

/// The HAL's configuration.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// The clock tree.
    pub clocks: clocks::Config,
    /// Whether to turn the cache (`cache`) on (it is by default).
    #[cfg(cmcc)]
    pub cache: bool,
    /// The time driver: which generator feeds its counter (its rate must be
    /// embassy-time's tick rate times 1, 2, 4, 8, 16, 64, 256 or 1024).
    #[cfg(feature = "time-driver")]
    pub time: time::Config,
}

impl Config {
    /// `clocks`, the cache on, and the time driver (if on) on generator 4.
    #[must_use]
    pub const fn new(clocks: clocks::Config) -> Self {
        Self {
            clocks,
            #[cfg(cmcc)]
            cache: true,
            #[cfg(feature = "time-driver")]
            time: time::Config::new(if cfg!(pic32cm_pl) { 1 } else { 4 }),
        }
    }
}

#[cfg(pic32ck)]
impl Default for Config {
    /// The clocks as they come out of reset, plus generator 4 at 16 MHz
    /// (the DFLL48M / 3) for the time driver: no crystal needed, but the
    /// DFLL48M runs open loop, so time is only as good as it.
    fn default() -> Self {
        let mut clocks = clocks::Config::boot();
        clocks.generators[4] = Some(clocks::Generator::divided(clocks::Source::Dfll48m, 3));
        Self::new(clocks)
    }
}

#[cfg(pic32cm_sg)]
impl Default for Config {
    /// The clocks as they come out of reset, plus generator 4 at 16 MHz
    /// (the DFLL48M / 3) for the time driver, as on the PIC32CK.
    fn default() -> Self {
        let mut clocks = clocks::Config::boot();
        clocks.generators[4] = Some(clocks::Generator::divided(clocks::Source::Dfll48m, 3));
        Self::new(clocks)
    }
}

#[cfg(pic32cm_pl)]
impl Default for Config {
    /// OSCHF at 24 MHz on generator 0, and generator 1 at 1 MHz (OSCHF / 24)
    /// for the time driver.
    fn default() -> Self {
        let mut clocks = clocks::Config::oschf_24mhz();
        clocks.generators[1] = Some(clocks::Generator::divided(clocks::Source::Oschf, 24));
        #[cfg_attr(not(feature = "time-driver"), allow(unused_mut))]
        let mut config = Self::new(clocks);
        #[cfg(feature = "time-driver")]
        {
            config.time = time::Config::new(1);
        }
        config
    }
}

/// Starts the HAL: the clocks, the cache, the time driver (if on), and the
/// peripheral singletons.
///
/// # Panics
///
/// On a second call; if the clock configuration is invalid or doesn't come
/// up (its [`clocks::Error`]); or if the time driver's generator isn't
/// configured, or doesn't divide to embassy-time's tick rate.
#[must_use]
pub fn init(config: Config) -> Peripherals {
    critical_section::with(|cs| {
        let peripherals = Peripherals::take_with_cs(cs);
        // SAFETY: once (`take_with_cs` panics on a second call), before any
        // driver exists.
        let frequencies = match unsafe { clocks::init(&config.clocks) } {
            Ok(frequencies) => frequencies,
            Err(e) => panic!("clocks: {e:?}"),
        };
        #[cfg(cmcc)]
        cache::set(config.cache);
        #[cfg(feature = "time-driver")]
        {
            let Some(hz) = frequencies.generators.get(config.time.generator).copied().flatten() else {
                panic!("time driver: generator {} isn't configured", config.time.generator);
            };
            // SAFETY: once, after the clocks; the driver's counter isn't
            // in the peripherals with the time driver on.
            unsafe { time::init(config.time, hz) };
        }
        #[cfg(not(feature = "time-driver"))]
        let _ = frequencies;
        peripherals
    })
}

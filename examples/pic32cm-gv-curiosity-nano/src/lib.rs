// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Board support for the PIC32CM GV-Value Line Curiosity Nano+ Touch
//! (EV80D90A, PIC32CM3204GV00064): [`init`], which starts pic32c-hal with
//! the CPU at 8 MHz and the time driver on generator 1 (1 MHz), the board's
//! pins, and [`Log`], the bench programs' log on the virtual COM port.
//!
//! - The debugger's virtual COM port: SERCOM2, TX [`VCOM_TX`] (PA12, pad 0)
//!   and RX [`VCOM_RX`] (PA13, pad 1), both on function C.
//! - The yellow user LED: PA24, lit high.
//! - The user button: PB17, low while pressed (no pull-up on the board).
//! - The 32.768 kHz crystal on XIN32/XOUT32 (PA00/PA01), connected.
//! - The touch button and its shield: PA07 and PB01 (the PTC; not driven).
//!
//! The board's target supply is set by its debugger (3.3 V on the bench
//! board; the user guide's 5.0 V factory default would be over the GV00's
//! 3.63 V). Flash with pyOCD and Microchip's PIC32CM-GV_DFP pack.

#![no_std]

pub use pic32c_hal;
pub use pic32c_hal::pac;

use pic32c_hal::mode::Blocking;
use pic32c_hal::uart::{self, Uart};
use pic32c_hal::{Peri, peripherals};

/// The virtual COM port's pins, as names to look for in the peripherals.
pub const VCOM_TX: &str = "PA12";
/// See [`VCOM_TX`].
pub const VCOM_RX: &str = "PA13";

/// Starts pic32c-hal: OSC8M undivided (8 MHz) for the CPU, generator 1 at
/// 1 MHz for the time driver. Call it first.
#[must_use]
pub fn init() -> pic32c_hal::Peripherals {
    pic32c_hal::init(pic32c_hal::Config::default())
}

/// The virtual COM port as a blocking log (115,200 baud), `\n` sent as
/// `\r\n`.
pub struct Log(Uart<'static, Blocking>);

impl Log {
    /// The log on SERCOM2, TX PA12 and RX PA13.
    ///
    /// # Panics
    ///
    /// If the SERCOM's generator (0) can't give 115,200 baud.
    #[must_use]
    pub fn init(sercom: Peri<'static, peripherals::SERCOM2>, tx: Peri<'static, peripherals::PA12>, rx: Peri<'static, peripherals::PA13>) -> Self {
        Self(Uart::new_blocking(sercom, tx, rx, uart::Config::default()).expect("115,200 baud from generator 0"))
    }

    /// Waits until everything written has gone out.
    pub fn flush(&mut self) {
        self.0.blocking_flush();
    }
}

impl core::fmt::Write for Log {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for line in s.split_inclusive('\n') {
            self.0.blocking_write(line.trim_end_matches('\n').as_bytes());
            if line.ends_with('\n') {
                self.0.blocking_write(b"\r\n");
            }
        }
        Ok(())
    }
}

/// Microseconds since the time driver started.
#[must_use]
pub fn micros() -> u64 {
    embassy_time::Instant::now().as_micros()
}

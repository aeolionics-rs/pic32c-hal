// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Board support for the PIC32CM PL10 Curiosity Nano (EV10P22A,
//! PIC32CM6408PL10048): [`init`], which starts pic32c-hal at 24 MHz with
//! the time driver on generator 1 (1 MHz), the board's pins, and [`Log`],
//! the bench programs' log on the virtual COM port.
//!
//! - The debugger's virtual COM port: SERCOM1, TX [`VCOM_TX`] (PB00, pad 0)
//!   and RX [`VCOM_RX`] (PB01, pad 1), both on function D.
//! - LED0: PB02, lit low.
//! - SERCOM0, the other SERCOM, on function C: the board's SPI pins (MOSI
//!   PA04, SCK PA05, SS PA06, MISO PA07), its I2C pins (SDA PA00, SCL
//!   PA01), or a second UART (PA04/PA05, pads 0/1).
//!
//! Flash with pyOCD and Microchip's PIC32CM-PL_DFP pack (it runs the boot
//! ROM's handshake); probe-rs can't reset this chip, and versions before
//! 0.32.1 wedge the nEDBG.

#![no_std]

pub use pic32c_hal;
pub use pic32c_hal::pac;

use pic32c_hal::mode::Blocking;
use pic32c_hal::uart::{self, Uart};
use pic32c_hal::{Peri, peripherals};

/// The virtual COM port's pins, as names to look for in the peripherals.
pub const VCOM_TX: &str = "PB00";
/// See [`VCOM_TX`].
pub const VCOM_RX: &str = "PB01";

/// Starts pic32c-hal: OSCHF at 24 MHz for the CPU, generator 1 at 1 MHz for
/// the time driver. Call it first.
#[must_use]
pub fn init() -> pic32c_hal::Peripherals {
    pic32c_hal::init(pic32c_hal::Config::default())
}

/// The virtual COM port as a blocking log (115,200 baud), `\n` sent as
/// `\r\n`.
pub struct Log(Uart<'static, Blocking>);

impl Log {
    /// The log on SERCOM1, TX PB00 and RX PB01.
    ///
    /// # Panics
    ///
    /// If the SERCOM's generator (0, 24 MHz after [`init`]) can't give
    /// 115,200 baud.
    #[must_use]
    pub fn init(sercom: Peri<'static, peripherals::SERCOM1>, tx: Peri<'static, peripherals::PB00>, rx: Peri<'static, peripherals::PB01>) -> Self {
        Self(Uart::new_blocking(sercom, tx, rx, uart::Config::default()).expect("115,200 baud from 24 MHz"))
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

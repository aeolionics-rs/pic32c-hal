// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A log over the `PKoB4`'s virtual COM port: pic32c-hal's UART on SERCOM5,
//! 115200 8N1, TX on PA24 (pad 0) and RX on PA25 (pad 1), function C
//! (board guide DS70005529, the `PKOB4_VCOM` nets), from generator 3
//! (48 MHz).
//!
//! The UART lives in a static, so any [`Uart`] writes to it, from any
//! context. Each byte waits for room outside the critical section and goes
//! in inside one, so logging never holds interrupts off for long.
//!
//! The probe resets the chip into the boot ROM while it is connected, so
//! RTT is out; this is the bench programs' way to report.

use core::cell::RefCell;
use core::fmt;

use critical_section::Mutex;
use pic32c_hal::uart::{self, Blocking};
use pic32c_hal::{Peri, peripherals};

static LOG: Mutex<RefCell<Option<uart::Uart<'static, Blocking>>>> = Mutex::new(RefCell::new(None));

/// The log. [`Uart::init`] it once; then write to it with `write!`, from
/// this or any other `Uart`.
pub struct Uart;

impl Uart {
    /// Sets up SERCOM5 on its pins. [`crate::init`] must have started the
    /// clocks.
    ///
    /// # Panics
    ///
    /// If generator 3 can't give 115,200 baud.
    #[must_use]
    pub fn init(sercom: Peri<'static, peripherals::SERCOM5>, tx: Peri<'static, peripherals::PA24>, rx: Peri<'static, peripherals::PA25>) -> Self {
        let uart = uart::Uart::new_blocking(sercom, tx, rx, uart::Config::default()).expect("the log's UART");
        critical_section::with(|cs| LOG.borrow_ref_mut(cs).replace(uart));
        Self
    }

    /// Sends one byte, waiting for room. Before [`Uart::init`], it's lost.
    pub fn write_byte(&mut self, byte: u8) {
        while !critical_section::with(|cs| LOG.borrow_ref_mut(cs).as_mut().is_none_or(|uart| uart.try_write(byte))) {}
    }

    /// Sends one byte if there is room, `false` if not. Before
    /// [`Uart::init`], it's lost (and `true`).
    pub fn try_write_byte(&mut self, byte: u8) -> bool {
        critical_section::with(|cs| LOG.borrow_ref_mut(cs).as_mut().is_none_or(|uart| uart.try_write(byte)))
    }

    /// Waits until everything written has gone out.
    pub fn flush(&mut self) {
        while !critical_section::with(|cs| LOG.borrow_ref(cs).as_ref().is_none_or(uart::Uart::is_flushed)) {}
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
        Ok(())
    }
}

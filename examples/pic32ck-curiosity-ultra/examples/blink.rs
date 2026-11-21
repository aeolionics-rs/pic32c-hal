// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The bench's first light: LED0 (PD20) blinks once a second, and a line
//! goes out over the virtual COM port with each toggle.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::time::Duration;

use cortex_m_rt::entry;
use panic_halt as _;
use pic32c_hal::gpio::{Level, Output};
use pic32ck_curiosity_ultra::time;
use pic32ck_curiosity_ultra::uart::Uart;

pic32ck_curiosity_ultra::boot!();

#[entry]
fn main() -> ! {
    let peripherals = pic32ck_curiosity_ultra::init();
    time::init(cortex_m::Peripherals::take().unwrap());
    let mut log = Uart::init(peripherals.SERCOM5, peripherals.PA24, peripherals.PA25);
    let mut led = Output::new(peripherals.PD20, Level::High);
    let _ = writeln!(log, "\npic32ck-curiosity-ultra blink");
    let mut count = 0u32;
    loop {
        led.toggle();
        count += 1;
        let _ = writeln!(log, "toggle {count}");
        time::delay(Duration::from_millis(500));
    }
}

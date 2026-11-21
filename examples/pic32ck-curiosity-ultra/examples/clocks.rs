// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! pic32c-hal's clocks up: 120 MHz CPU, 80 MHz CAN, 48 MHz USB and UART, from the
//! 12 MHz crystal through PLL0. Prints the cycle counter every 120 million
//! cycles over the virtual COM port, so a host timing the lines checks the
//! CPU clock (one line a second), and the UART working at all checks the
//! 48 MHz. LED0 toggles with each line.

#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use pic32ck_curiosity_ultra::{clocks, crumb, time, uart::Uart};
use panic_halt as _;
use pic32c_hal::gpio::{Level, Output};

pic32ck_curiosity_ultra::boot!();


#[entry]
fn main() -> ! {
    crumb(0, 0xC10C_0001);
    let peripherals = pic32ck_curiosity_ultra::init();
    crumb(1, 0);
    let core = cortex_m::Peripherals::take().unwrap();
    time::init(core);
    let mut led = Output::new(peripherals.PD20, Level::High);
    let mut uart = Uart::init(peripherals.SERCOM5, peripherals.PA24, peripherals.PA25);
    let _ = writeln!(uart, "\npic32ck-curiosity-ultra clocks: {:?}", pic32c_hal::clocks::frequencies());
    let mut next = time::cycles();
    let mut line = 0u32;
    loop {
        next += u64::from(clocks::CPU_HZ);
        while time::cycles() < next {}
        line += 1;
        crumb(2, line);
        // SAFETY: as above.
        led.toggle();
        let _ = writeln!(uart, "tick {line} cycles {}", time::cycles());
    }
}

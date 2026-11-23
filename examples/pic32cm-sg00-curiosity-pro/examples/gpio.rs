// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pic32c-hal`'s `gpio` under test on the SG00, on what the board has
//! without wiring (the user guide's tables 3-6 and figures 3-18 to 3-21):
//! - the internal pulls on two mikroBUS pins that float with nothing in
//!   the socket, AN (PA07) and RST (PB17): high with the pull-up, low with
//!   the pull-down;
//! - SW0 (PB07) and SW1 (PB11) go to ground through their buttons: released,
//!   with the pull-up, they read high, and with no pull of the chip's the
//!   level is reported (don't press them meanwhile);
//! - LED1 (PA18, to 3.3 V through the LED and 330 Ω) as a push-pull output
//!   reads back what it drives (open drain is `open_drain`'s, on a jumper);
//! - a dropped pin goes back to its reset state (no direction, PINCFG 0);
//! - then SW0: press it within 10 s, and the program says it saw it (not
//!   counted as a failure if nobody's there).
//!
//! Results go out over the virtual COM port; LED0 blinks slowly when all
//! passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use embassy_time::{Duration, block_for};
use panic_halt as _;
use pic32c_hal::Peri;
use pic32c_hal::gpio::{AnyPin, Flex, Input, Level, Output, Pull};
use pic32cm_sg00_curiosity_pro::{Log as Uart, micros};

pic32cm_sg00_curiosity_pro::boot!();

/// PORT group A's DIR and PINCFG18 (LED1).
const DIR_A: usize = 0x4480_0000;
const PINCFG18: usize = 0x4480_0040 + 18;

/// Long enough for a pull to charge the pin and the input to resynchronise.
const SETTLE: Duration = Duration::from_micros(100);

#[entry]
fn main() -> ! {
    let p = pic32cm_sg00_curiosity_pro::init();
    let mut log = Uart::init(p.SERCOM4, p.PD19, p.PD20);
    // LED0 is active low.
    let led0 = Output::new(p.PA20, Level::High);
    let _ = writeln!(log, "\npic32cm-sg00-curiosity-pro gpio");

    let mut failures = 0;
    let floating: [(&str, Peri<'static, AnyPin>); 2] = [("mikroBUS AN (PA07)", p.PA07.into()), ("mikroBUS RST (PB17)", p.PB17.into())];
    for (name, pin) in floating {
        failures += pulls(&mut log, name, pin);
    }
    let mut sw0: Peri<'static, AnyPin> = p.PB07.into();
    let buttons: [(&str, Peri<'_, AnyPin>); 2] = [("SW0 (PB07)", sw0.reborrow()), ("SW1 (PB11)", p.PB11.into())];
    for (name, mut pin) in buttons {
        let level = {
            let button = Input::new(pin.reborrow(), Pull::None);
            block_for(SETTLE);
            button.get_level()
        };
        let button = Input::new(pin, Pull::Up);
        block_for(SETTLE);
        let up = button.get_level();
        let _ = writeln!(log, "{name} released: no pull {level:?} (reported), pull-up {up:?}{}", if up == Level::High { "" } else { ", WRONG" });
        failures += u32::from(up != Level::High);
    }
    let mut led1: Peri<'static, AnyPin> = p.PA18.into();
    failures += push_pull(&mut log, led1.reborrow());
    failures += reset_on_drop(&mut log, led1.reborrow());

    let button = Input::new(sw0.reborrow(), Pull::None);
    let _ = writeln!(log, "press SW0 within 10 s");
    let start = micros();
    let mut pressed = None;
    while pressed.is_none() && micros() - start < 10_000_000 {
        if button.is_low() {
            pressed = Some((micros() - start) / 1000);
        }
    }
    match pressed {
        Some(ms) => {
            let _ = writeln!(log, "SW0 pressed after {ms} ms");
        }
        None => {
            let _ = writeln!(log, "SW0 not pressed (not a failure)");
        }
    }

    if failures == 0 {
        let _ = writeln!(log, "gpio: all passed");
    } else {
        let _ = writeln!(log, "gpio: {failures} failures");
    }
    halt(&mut log, failures, led0);
}

fn halt(log: &mut Uart, failures: u32, mut led: Output<'static>) -> ! {
    log.flush();
    let period = if failures == 0 { 500 } else { 100 };
    loop {
        led.toggle();
        block_for(Duration::from_millis(period));
    }
}

/// A floating pin reads high with the pull-up and low with the pull-down.
fn pulls(log: &mut Uart, name: &str, pin: Peri<'_, AnyPin>) -> u32 {
    let mut pin = Flex::new(pin);
    pin.set_as_input(Pull::Up);
    block_for(SETTLE);
    let up = pin.get_level();
    pin.set_as_input(Pull::Down);
    block_for(SETTLE);
    let down = pin.get_level();
    pin.set_as_input(Pull::Up);
    block_for(SETTLE);
    let up_again = pin.get_level();
    let good = up == Level::High && down == Level::Low && up_again == Level::High;
    let _ = writeln!(log, "{name}: pull-up {up:?}, pull-down {down:?}, pull-up {up_again:?}{}", if good { "" } else { ", WRONG" });
    u32::from(!good)
}

/// Push-pull: the pin reads what it drives.
fn push_pull(log: &mut Uart, pin: Peri<'_, AnyPin>) -> u32 {
    let mut pin = Flex::new(pin);
    pin.set_high();
    pin.set_as_output();
    let mut seen = [Level::Low; 4];
    for (i, level) in seen.iter_mut().enumerate() {
        if i > 0 {
            pin.toggle();
        }
        block_for(SETTLE);
        *level = pin.get_level();
    }
    let good = seen == [Level::High, Level::Low, Level::High, Level::Low] && pin.is_set_low();
    let _ = writeln!(log, "LED1 push-pull, high then toggled three times: {seen:?}{}", if good { "" } else { ", WRONG" });
    u32::from(!good)
}

/// Dropping a driver leaves the pin with no direction and PINCFG 0.
fn reset_on_drop(log: &mut Uart, pin: Peri<'_, AnyPin>) -> u32 {
    drop(Output::new(pin, Level::Low));
    // SAFETY: PORT's DIR and LED1's PINCFG, read only.
    let (dir, pincfg) = unsafe { (core::ptr::read_volatile(DIR_A as *const u32) & (1 << 18), core::ptr::read_volatile(PINCFG18 as *const u8)) };
    let good = dir == 0 && pincfg == 0;
    let _ = writeln!(log, "LED1 dropped: DIR {dir:#x}, PINCFG {pincfg:#x}{}", if good { "" } else { ", WRONG" });
    u32::from(!good)
}

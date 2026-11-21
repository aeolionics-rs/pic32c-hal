// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pic32c-hal`'s `gpio` under test, on what the board has without wiring
//! (DS70005529 table 2-11):
//! - the internal pulls on two mikroBUS pins that float with nothing in
//!   the socket, AN (PA10) and CS (PD15): high with the pull-up, low with
//!   the pull-down;
//! - SW0 (PB26) and SW1 (PD02) go to ground through their buttons and have
//!   pull-ups on the board (stronger than the internal pull-down, which they
//!   beat): released, with no pull of the chip's, they read high (don't
//!   press them meanwhile);
//! - LED1 (PB25, to 3.3 V through the LED and 330 Ω) as a push-pull output
//!   reads back what it drives; as open drain it reads low when driven low
//!   and, let go, high (the LED's path to 3.3 V holds it there);
//! - a dropped pin goes back to its reset state (no direction, PINCFG 0);
//! - then SW0: press it within 10 s, and the program says it saw it (not
//!   counted as a failure if nobody's there).
//!
//! Results go out over the virtual COM port; LED0 blinks slowly when all
//! passed, fast on a failure.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::time::Duration;

use cortex_m_rt::entry;
use panic_halt as _;
use pic32c_hal::gpio::{AnyPin, Flex, Input, Level, Output, OutputOpenDrain, Pull};
use pic32c_hal::{Peri, pac};
use pic32ck_curiosity_ultra::time;
use pic32ck_curiosity_ultra::uart::Uart;

pic32ck_curiosity_ultra::boot!();

/// Long enough for a pull to charge the pin and the input to resynchronise.
const SETTLE: Duration = Duration::from_micros(100);

#[entry]
fn main() -> ! {
    let p = pic32ck_curiosity_ultra::init();
    time::init(cortex_m::Peripherals::take().unwrap());
    let mut log = Uart::init(p.SERCOM5, p.PA24, p.PA25);
    let led0 = Output::new(p.PD20, Level::High);
    let _ = writeln!(log, "\npic32ck-curiosity-ultra gpio");

    let mut failures = 0;
    let floating: [(&str, Peri<'static, AnyPin>); 2] = [("mikroBUS AN (PA10)", p.PA10.into()), ("mikroBUS CS (PD15)", p.PD15.into())];
    for (name, pin) in floating {
        failures += pulls(&mut log, name, pin);
    }
    let mut sw0: Peri<'static, AnyPin> = p.PB26.into();
    let buttons: [(&str, Peri<'_, AnyPin>); 2] = [("SW0 (PB26)", sw0.reborrow()), ("SW1 (PD02)", p.PD02.into())];
    for (name, pin) in buttons {
        let button = Input::new(pin, Pull::None);
        time::delay(SETTLE);
        let level = button.get_level();
        let _ = writeln!(log, "{name} released, no pull: {level:?}{}", if level == Level::High { "" } else { ", WRONG" });
        failures += u32::from(level != Level::High);
    }
    let mut led1: Peri<'static, AnyPin> = p.PB25.into();
    failures += push_pull(&mut log, led1.reborrow());
    failures += open_drain(&mut log, led1.reborrow());
    failures += reset_on_drop(&mut log, led1.reborrow());

    let button = Input::new(sw0.reborrow(), Pull::None);
    let _ = writeln!(log, "press SW0 within 10 s");
    let start = time::micros();
    let mut pressed = None;
    while pressed.is_none() && time::micros() - start < 10_000_000 {
        if button.is_low() {
            pressed = Some((time::micros() - start) / 1000);
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
        time::delay(Duration::from_millis(period));
    }
}

/// A floating pin reads high with the pull-up and low with the pull-down.
fn pulls(log: &mut Uart, name: &str, pin: Peri<'_, AnyPin>) -> u32 {
    let mut pin = Flex::new(pin);
    pin.set_as_input(Pull::Up);
    time::delay(SETTLE);
    let up = pin.get_level();
    pin.set_as_input(Pull::Down);
    time::delay(SETTLE);
    let down = pin.get_level();
    pin.set_as_input(Pull::Up);
    time::delay(SETTLE);
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
        time::delay(SETTLE);
        *level = pin.get_level();
    }
    let good = seen == [Level::High, Level::Low, Level::High, Level::Low] && pin.is_set_low();
    let _ = writeln!(log, "LED1 push-pull, high then toggled three times: {seen:?}{}", if good { "" } else { ", WRONG" });
    u32::from(!good)
}

/// Open drain: low reads low; let go, the LED's path to 3.3 V reads high.
fn open_drain(log: &mut Uart, pin: Peri<'_, AnyPin>) -> u32 {
    let mut pin = OutputOpenDrain::new(pin, Level::Low);
    time::delay(SETTLE);
    let low = pin.is_high();
    pin.set_high();
    time::delay(Duration::from_millis(1));
    let released = pin.is_high();
    let good = !low && released;
    let _ = writeln!(log, "LED1 open drain: driven low reads high {low}, let go reads high {released}{}", if good { "" } else { ", WRONG" });
    u32::from(!good)
}

/// Dropping a driver leaves the pin with no direction and PINCFG 0.
fn reset_on_drop(log: &mut Uart, pin: Peri<'_, AnyPin>) -> u32 {
    drop(Output::new(pin, Level::Low));
    // LED1 is PB25: group 1, pin 25.
    let group = port().group(1);
    let dir = group.dir().read().bits() & (1 << 25);
    let pincfg = group.pincfg(25).read().bits();
    let good = dir == 0 && pincfg == 0;
    let _ = writeln!(log, "LED1 dropped: DIR {dir:#x}, PINCFG {pincfg:#x}{}", if good { "" } else { ", WRONG" });
    u32::from(!good)
}

/// PORT as Secure code reaches it, from the PAC, to check the HAL's
/// writes independently.
fn port() -> &'static pac::port::RegisterBlock {
    #[cfg(feature = "sg01")]
    let block = pac::PortSec::ptr();
    #[cfg(not(feature = "sg01"))]
    let block = pac::Port::ptr();
    // SAFETY: PORT's register block, read only here.
    unsafe { &*block }
}

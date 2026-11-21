// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Board support for the PIC32CM SG00 Curiosity Pro (EV53V42A,
//! PIC32CM5112SG00100): [`boot!`], and [`init`], which starts pic32c-hal at
//! 72 MHz from the board's 12 MHz crystal, with the time driver on
//! generator 4 (16 MHz).
//!
//! - The debugger's virtual COM port: SERCOM4, TX PD19 (pad 0) and RX PD20
//!   (pad 1), function D, I/O set 2.
//! - LED0 PA20 and LED1 PA18, lit low; SW0 PB07 and SW1 PB11, low when
//!   pressed.
//! - CAN0 (PA10 RX, PA11 TX) and CAN1 (PD12 RX, PD13 TX) through ATA6561
//!   transceivers, function H, I/O set 3.
//!
//! Flash with pyOCD and Microchip's PIC32CM-SG00_DFP pack, which runs the
//! boot ROM's handshake; the board's on-board debugger must be in CMSIS-DAP
//! mode.

#![no_std]

pub use pic32c_hal;
pub use pic32c_hal::pac;

use pic32c_hal::clocks::{self, Xosc};

/// The CPU's rate after [`init`].
pub const CPU_HZ: u32 = 72_000_000;
/// The CAN controllers' kernel clock's generator (40 MHz).
pub const GEN_CAN: usize = 2;
/// The generator the SERCOMs take by default (48 MHz).
pub const GEN_48: usize = 3;
/// The time driver's generator (16 MHz).
pub const GEN_TIMER: usize = 4;

/// pic32c-hal's configuration for the board: the 12 MHz crystal through
/// PLL0 ([`clocks::Config::pll_from_xosc`]: 72 MHz CPU, 40 MHz on
/// generator 2 for CAN, 48 MHz on 3, 16 MHz on 4), embassy-time on
/// generator 4.
#[must_use]
pub fn config() -> pic32c_hal::Config {
    let mut config = pic32c_hal::Config::new(clocks::Config::pll_from_xosc(Xosc::Crystal(12_000_000)));
    config.time = pic32c_hal::time::Config::new(GEN_TIMER);
    config
}

/// Starts pic32c-hal with [`config`] and returns the peripherals. Call it
/// first.
///
/// # Panics
///
/// If the clocks don't come up at 72 MHz.
#[must_use]
pub fn init() -> pic32c_hal::Peripherals {
    let peripherals = pic32c_hal::init(config());
    assert_eq!(clocks::generator_hz(0), Some(CPU_HZ), "the CPU's clock");
    peripherals
}

/// The two words the boot ROM starts from, and the SRAM initialisation.
/// Invoke it once, at the top level of each program.
///
/// - The boot ROM starts the first mutable executable at the start of boot
///   flash (0x08000000; DS60001921 16.5.1.6), loading the stack pointer and PC
///   from its first two words; the program and its vector table are in
///   program flash, so boot flash holds just the stack top and cortex-m-rt's
///   `Reset` (`.bfm` in `memory.x`), and `Reset` (`set-vtor`) points VTOR at
///   the vector table.
/// - SRAM has ECC, and its content starts random: a word must be written
///   whole before it is read, or written in part (DS60001921 17.4.2). The
///   `__pre_init` that cortex-m-rt calls before it sets up `.data` and `.bss`
///   writes zero to every word from the start of SRAM up to the stack
///   pointer. In assembly: no Rust may touch RAM yet.
///
/// The same as the PIC32CK board crate's.
#[macro_export]
macro_rules! boot {
    () => {
        mod boot {
            unsafe extern "C" {
                /// cortex-m-rt: the top of the stack, and the reset handler.
                static _stack_start: u32;
                fn Reset() -> !;
            }

            #[repr(C)]
            pub struct BootVector([*const (); 2]);

            // SAFETY: two addresses, read only by the boot ROM.
            unsafe impl Sync for BootVector {}

            #[unsafe(link_section = ".bfm")]
            #[used]
            pub static BOOT: BootVector = BootVector([&raw const _stack_start as *const (), Reset as *const ()]);

            core::arch::global_asm!(
                ".section .text.__pre_init, \"ax\"",
                ".global __pre_init",
                ".type __pre_init, %function",
                ".thumb_func",
                "__pre_init:",
                "    ldr r0, =0x20000000",
                "    mov r1, sp",
                "    movs r2, #0",
                "1:  cmp r0, r1",
                "    bhs 2f",
                "    str r2, [r0]",
                "    adds r0, #4",
                "    b 1b",
                "2:  bx lr",
                ".ltorg",
            );
        }
    };
}

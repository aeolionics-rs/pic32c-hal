// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Board support for the PIC32CK SG01 and GC01 Curiosity Ultra boards:
//! [`boot!`], the [`clocks`], a [`uart`] log over the debugger's virtual COM
//! port, [`time`] from the cycle counter, and [`init`], which starts
//! pic32c-hal with these clocks. Choose the board with feature `sg01` or
//! `gc01`.
//!
//! pic32c-hal's bench programs are this crate's examples; what several of
//! them share (CAN0 on the board's transceiver, mDNS, SNTP, TLS) is in
//! `examples/common/`, not here.

#![no_std]

pub mod clocks;
pub mod time;
pub mod uart;

pub use pic32c_hal;
pub use pic32c_hal::pac;

/// Starts pic32c-hal with the boards' [`clocks`], the cache on and
/// embassy-time on generator 4, and returns the peripherals. Call it first.
///
/// # Panics
///
/// If the clocks don't come up at the rates [`clocks`] gives.
#[must_use]
pub fn init() -> pic32c_hal::Peripherals {
    init_at(0)
}

/// [`init`], with embassy-time's count starting at `start` ticks (near
/// 2^31 or 2^32 to test the time driver's crossings).
///
/// # Panics
///
/// As for [`init`].
#[must_use]
pub fn init_at(start: u32) -> pic32c_hal::Peripherals {
    init_with(config(start))
}

/// pic32c-hal's configuration for the boards: their [`clocks`], the cache
/// on, embassy-time on generator 4 from `start` ticks.
#[must_use]
pub fn config(start: u32) -> pic32c_hal::Config {
    let mut config = pic32c_hal::Config::new(clocks::CONFIG);
    config.time = pic32c_hal::time::Config { generator: clocks::GEN_TIMER, start };
    config
}

/// Starts pic32c-hal with `config` (from [`config`], changed as a program
/// needs), and checks the clocks came up at the rates [`clocks`] gives.
///
/// # Panics
///
/// As for [`init`].
#[must_use]
pub fn init_with(config: pic32c_hal::Config) -> pic32c_hal::Peripherals {
    let peripherals = pic32c_hal::init(config);
    for (generator, hz) in [(clocks::GEN_CPU, clocks::CPU_HZ), (clocks::GEN_CAN, clocks::CAN_HZ), (clocks::GEN_48, clocks::MAIN_48_HZ), (clocks::GEN_TIMER, clocks::TIMER_HZ)] {
        assert_eq!(pic32c_hal::clocks::generator_hz(generator), Some(hz), "generator {generator}");
    }
    peripherals
}

#[cfg(not(any(feature = "sg01", feature = "gc01")))]
compile_error!("choose the board: feature `sg01` or `gc01`");
#[cfg(all(feature = "sg01", feature = "gc01"))]
compile_error!("features `sg01` and `gc01` are different boards; choose one");

/// Writes word `index` of the breadcrumbs at `0x2004_0000`, which survive the
/// reset a probe makes (the boot ROM clears only the first 2 KiB of SRAM):
/// power-cycle, then read them with the probe.
pub fn crumb(index: usize, value: u32) {
    // SAFETY: SRAM well above anything the bench programs use.
    unsafe { core::ptr::write_volatile((0x2004_0000 + 4 * index) as *mut u32, value) }
}

/// The two words the boot ROM starts from, and the SRAM initialisation.
/// Invoke it once, at the top level of each program.
///
/// - The boot ROM loads the stack pointer and PC from the first two words of
///   boot flash (0x08000000); the program and its vector table are in
///   program flash, so boot flash holds just the stack top and cortex-m-rt's
///   `Reset` (`.bfm` in `memory.x`), and `Reset` (`set-vtor`) points VTOR at
///   the vector table.
/// - SRAM has ECC, and its content starts random: a word must be written
///   whole before it is read, or written in part (DS60001795 13.4.2). The
///   `__pre_init` that cortex-m-rt calls before it sets up `.data` and `.bss`
///   writes zero to every word from the start of SRAM up to the stack
///   pointer. In assembly: no Rust may touch RAM yet.
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

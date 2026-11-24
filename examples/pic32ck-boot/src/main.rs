// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! pic32c-boot's reference bootloader for the PIC32CK Curiosity Ultra (spike), in boot flash.
//!
//! Out of reset the panels aren't swapped (`PFSWAP` is volatile, 0), so program flash panel 1
//! is at `0x0C00_0000` and panel 2 at `0x0C10_0000`. The bootloader:
//!
//! 1. checks the image at the start of each panel ([`pic32c_boot::image::check`]);
//! 2. opens the boot log on the top page of each panel ([`pic32c_boot::log`]);
//! 3. asks [`Policy::decide`] which image to run, from the log, the images and the reset cause,
//!    and appends what it says to;
//! 4. maps the chosen panel first (`PFSWAP`), from boot flash with no access to either panel
//!    meanwhile, as the data sheet requires (DS60001795, 31.2.17.4);
//! 5. jumps to it through its vector table at `0x0C00_0000`.
//!
//! With nothing to run it waits, for a debugger: recovery comes later. It leaves the clocks as
//! reset left them (the application sets its own) and logs nothing; what it decided is in the
//! boot log, for the application to report. No signature yet, no locks: a spike.

#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

use cortex_m_rt::entry;
use panic_halt as _;
use pic32c_boot::image;
use pic32c_boot::log::Log;
use pic32c_boot::nor::NorPages;
use pic32c_boot::{Image, Policy, ResetCause};
use pic32c_hal::flash::{Flash, PAGE_SIZE, PANEL_SIZE, PFM_BASE};
use pic32c_hal::peripherals::FCW;
use pic32c_hal::wdt;

/// A program flash panel, where it is in the part.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Panel {
    One,
    Two,
}

impl Panel {
    /// Where the panel is while the panels aren't swapped.
    const fn base(self) -> u32 {
        match self {
            Panel::One => PFM_BASE,
            #[expect(clippy::cast_possible_truncation, reason = "a panel is 1 MiB")]
            Panel::Two => PFM_BASE + PANEL_SIZE as u32,
        }
    }
}

/// The log's pages: the top page of each panel, offsets from the start of program flash. A
/// swap trades where they appear, which the log doesn't mind.
#[expect(clippy::cast_possible_truncation, reason = "inside the 2 MiB of program flash")]
const LOG_PAGES: [u32; 2] = [(PANEL_SIZE - PAGE_SIZE) as u32, (2 * PANEL_SIZE - PAGE_SIZE) as u32];

/// The bootloader's policy: no version refused yet, three starts for a trial.
const POLICY: Policy = Policy { min_version: 1, max_starts: 3 };

/// FCW's STATUS, KEY and SWAP (DS60001795, 31.2.23).
const STATUS: *const u32 = 0x4400_4018 as *const u32;
const KEY: *mut u32 = 0x4400_401C as *mut u32;
const SWAP: *mut u32 = 0x4400_4048 as *mut u32;
const STATUS_BUSY: u32 = 1;
/// The unlock code with SWAPKEY.
const SWAPKEY: u32 = 0x91C3_2C02;
const PFSWAP: u32 = 1 << 8;

// SRAM has ECC and starts random, so it's zeroed up to the stack before anything reads it, as
// the board crate's `boot!` does for its programs.
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

#[entry]
fn main() -> ! {
    // The images, panel 1's first.
    let mut images = [Image {
        id: pic32c_boot::ImageId([0; 16]),
        version: 0,
        at: Panel::One,
    }; 2];
    let mut found = 0;
    for panel in [Panel::One, Panel::Two] {
        // SAFETY: program flash, mapped and read only here; the image is no bigger than the
        // panel less its log page.
        let bytes = unsafe { core::slice::from_raw_parts(panel.base() as *const u8, PANEL_SIZE - PAGE_SIZE) };
        if let Some(header) = image::check(bytes) {
            images[found] = Image {
                id: header.id(),
                version: header.version,
                at: panel,
            };
            found += 1;
        }
    }

    // SAFETY: the only user of the FCW until the jump.
    let flash = Flash::new_blocking(unsafe { FCW::steal() });
    let pages = NorPages::new(flash, LOG_PAGES).unwrap_or_else(|| panic!("log pages"));
    let mut log = Log::open(pages);
    let decision = POLICY.decide(reset_cause(), log.newest_first(), &images[..found]);
    let boot = decision.boot.map(|image| image.at);
    if let Some(record) = decision.append {
        // A record that didn't make it is a power cut, and the next boot decides again.
        let _ = log.append(record);
    }

    match boot {
        Some(panel) => {
            swap(panel == Panel::Two);
            // SAFETY: the chosen image's vector table is at the start of program flash now, and
            // it checked whole.
            unsafe { cortex_m::asm::bootload(PFM_BASE as *const u32) }
        }
        None => loop {
            cortex_m::asm::wfi();
        },
    }
}

/// Why the part reset, as the policy takes it.
fn reset_cause() -> ResetCause {
    let cause = wdt::ResetCause::read();
    if cause.power_on() {
        ResetCause::PowerOn
    } else if cause.brown_out() {
        ResetCause::BrownOut
    } else if cause.watchdog() {
        ResetCause::Watchdog
    } else if cause.external() {
        ResetCause::Pin
    } else {
        ResetCause::Software
    }
}

/// Map panel 2 first (`upper`), or panel 1. Runs from boot flash, with the stack in SRAM, so
/// neither panel is accessed while PFSWAP changes.
#[inline(never)]
fn swap(upper: bool) {
    cortex_m::asm::dsb();
    // SAFETY: the FCW's STATUS, KEY and SWAP, as the data sheet sets them; BFSWAP (bit 0) and
    // the locks are written back as they were.
    unsafe {
        while read_volatile(STATUS) & STATUS_BUSY != 0 {}
        write_volatile(KEY, SWAPKEY);
        let swap = read_volatile(SWAP) & !PFSWAP;
        write_volatile(SWAP, if upper { swap | PFSWAP } else { swap });
    }
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
}

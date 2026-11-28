// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A test image for pic32c-boot's bootloader (`examples/pic32ck-boot`): made into an image
//! with `tools/host/image.py` and loaded into a panel, it reports what the bootloader did and
//! plays an application's part in an update.
//!
//! It reports, over the virtual COM port, its version and ID, which physical panel it runs from
//! (`PFSWAP`), the reset cause, and the newest records in the boot log (the top page of each
//! panel). Then, as built:
//!
//! - `BOOT_APP_CONFIRM=yes`: it confirms itself (appends `Confirmed`, unless that's already the
//!   newest record), then plays the updater: if the other panel holds a whole image the log
//!   doesn't mention yet, it appends `Requested` for it and resets to start its trial. LED0
//!   blinks slowly.
//! - otherwise: it doesn't confirm, and resets itself after 3 s, as an image that fails would.
//!   LED0 blinks fast meanwhile.
//!
//! `BOOT_APP_VERSION` (1 if not set) is the version given to `image.py`, reported here.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::time::Duration;

use cortex_m_rt::entry;
use panic_halt as _;
use pic32c_boot::log::Log;
use pic32c_boot::nor::NorPages;
use pic32c_boot::{ImageId, Record, image};
use pic32c_hal::flash::{Flash, PAGE_SIZE, PANEL_SIZE, PFM_BASE};
use pic32c_hal::gpio::{Level, Output};
use pic32c_hal::wdt;
use pic32ck_curiosity_ultra::time;
use pic32ck_curiosity_ultra::uart::Uart;

pic32ck_curiosity_ultra::boot!();

#[expect(clippy::cast_possible_truncation, reason = "inside the 2 MiB of program flash")]
const LOG_PAGES: [u32; 2] = [(PANEL_SIZE - PAGE_SIZE) as u32, (2 * PANEL_SIZE - PAGE_SIZE) as u32];
/// FCW's SWAP register: PFSWAP is bit 8.
const SWAP: *const u32 = 0x4400_4048 as *const u32;

/// A panel's bytes as mapped now: `0` the one this runs from, `1` the other.
fn mapped(which: usize) -> &'static [u8] {
    // SAFETY: program flash, read only; the image is no bigger than a panel less its log page.
    unsafe { core::slice::from_raw_parts((PFM_BASE as usize + which * PANEL_SIZE) as *const u8, PANEL_SIZE - PAGE_SIZE) }
}

/// An ID's first bytes, enough to tell images apart in a log.
struct Short(ImageId);

impl core::fmt::Display for Short {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for b in &self.0.0[..4] {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

fn describe(record: Record) -> (&'static str, Short) {
    match record {
        Record::Requested(id) => ("requested", Short(id)),
        Record::Started(id) => ("started", Short(id)),
        Record::Failed(id) => ("failed", Short(id)),
        Record::Confirmed(id) => ("confirmed", Short(id)),
    }
}

#[entry]
fn main() -> ! {
    let peripherals = pic32ck_curiosity_ultra::init();
    time::init(cortex_m::Peripherals::take().unwrap());
    let mut led = Output::new(peripherals.PD20, Level::High);
    let mut log = Uart::init(peripherals.SERCOM5, peripherals.PA24, peripherals.PA25);
    let confirm = option_env!("BOOT_APP_CONFIRM") == Some("yes");
    let version = option_env!("BOOT_APP_VERSION").unwrap_or("1");

    let own = image::header(mapped(0));
    // SAFETY: a read of FCW's SWAP.
    let panel = if unsafe { core::ptr::read_volatile(SWAP) } & (1 << 8) != 0 { 2 } else { 1 };
    let _ = write!(log, "\nboot_app version {version}, ");
    let _ = match own {
        Some(h) => write!(log, "id {}", Short(h.id())),
        None => write!(log, "no header"),
    };
    let _ = writeln!(log, ", {}confirming; running from panel {panel}; reset cause {:#06x}", if confirm { "" } else { "not " }, wdt::ResetCause::read().0);

    let pages = NorPages::new(Flash::new_blocking(peripherals.FCW), LOG_PAGES).expect("log pages");
    let mut boot = Log::open(pages);
    let _ = write!(log, "log, newest first:");
    for record in boot.newest_first().take(8) {
        let (kind, id) = describe(record);
        let _ = write!(log, " {kind} {id};");
    }
    let _ = writeln!(log);

    if !confirm {
        let _ = writeln!(log, "not confirming: resetting in 3 s");
        for _ in 0..30 {
            led.toggle();
            time::delay(Duration::from_millis(100));
        }
        cortex_m::peripheral::SCB::sys_reset();
    }

    if let Some(own) = own {
        let newest = boot.newest_first().next();
        if newest != Some(Record::Confirmed(own.id())) {
            let result = boot.append(Record::Confirmed(own.id()));
            let _ = writeln!(log, "confirmed {}: {result:?}", Short(own.id()));
        }
    }
    // The updater's part: a whole image in the other panel, not in the log yet, is requested.
    if let Some(other) = image::check(mapped(1)) {
        let known = boot.newest_first().any(|r| r.id() == other.id());
        if !known {
            let result = boot.append(Record::Requested(other.id()));
            let _ = writeln!(log, "requested {} (version {}): {result:?}; resetting to start it", Short(other.id()), other.version);
            time::delay(Duration::from_millis(200));
            cortex_m::peripheral::SCB::sys_reset();
        }
    }
    let _ = writeln!(log, "running");
    loop {
        led.toggle();
        time::delay(Duration::from_millis(500));
    }
}

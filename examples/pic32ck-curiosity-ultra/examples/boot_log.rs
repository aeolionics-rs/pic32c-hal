// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pic32c-boot`'s boot log on `pic32c-hal`'s flash, through its `NorPages` adapter, on two
//! scratch pages of PFM clear of the program and of what other programs on the bench boards
//! keep in flash: the top page of panel 1 (offset `0x0F_F000`) and the third page
//! from the top of panel 2 (offset `0x1F_D000`; the two above are the boot log's and a gateway's).
//!
//! Each run opens the log as the last run left it, reports how many records it holds and the
//! newest, appends 50 (a flip every 128), opens it again and checks the newest 50 read back are
//! those, newest first. Run it a few times to see it carry on across resets and flips. The
//! results go out over the virtual COM port; LED0 blinks slowly when the check passed, fast if
//! not.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::time::Duration;

use cortex_m_rt::entry;
use panic_halt as _;
use pic32c_boot::ImageId;
use pic32c_boot::Record;
use pic32c_boot::log::Log;
use pic32c_boot::nor::NorPages;
use pic32c_hal::flash::Flash;
use pic32c_hal::gpio::{Level, Output};
use pic32ck_curiosity_ultra::time;
use pic32ck_curiosity_ultra::uart::Uart;

pic32ck_curiosity_ultra::boot!();

const PAGES: [u32; 2] = [0x0F_F000, 0x1F_D000];
const APPENDS: u8 = 50;

#[entry]
fn main() -> ! {
    let peripherals = pic32ck_curiosity_ultra::init();
    time::init(cortex_m::Peripherals::take().unwrap());
    let mut led = Output::new(peripherals.PD20, Level::High);
    let mut log = Uart::init(peripherals.SERCOM5, peripherals.PA24, peripherals.PA25);
    let _ = writeln!(log, "\npic32ck-curiosity-ultra boot_log");

    let pages = NorPages::new(Flash::new_blocking(peripherals.FCW), PAGES).expect("pages inside PFM, one erase unit each");
    let mut boot = Log::open(pages);
    let held = boot.newest_first().count();
    let newest = boot.newest_first().next();
    let _ = writeln!(log, "log holds {held} records, the newest {newest:?}");

    // This run's records: a kind and an id that differ from record to record.
    let run = newest.map_or(0, |r| r.id().0[0].wrapping_add(1));
    let record = |n: u8| {
        let id = ImageId([run.wrapping_add(n); 16]);
        match n % 4 {
            0 => Record::Requested(id),
            1 => Record::Started(id),
            2 => Record::Failed(id),
            _ => Record::Confirmed(id),
        }
    };
    let mut failures = 0u32;
    for n in 0..APPENDS {
        if let Err(e) = boot.append(record(n)) {
            let _ = writeln!(log, "append {n}: {e:?}");
            failures += 1;
        }
    }

    let mut boot = Log::open(boot.into_flash());
    let total = boot.newest_first().count();
    for (i, read) in boot.newest_first().take(usize::from(APPENDS)).enumerate() {
        let want = record(APPENDS - 1 - u8::try_from(i).unwrap_or(0));
        if read != want {
            let _ = writeln!(log, "record {i} back: {read:?}, not {want:?}");
            failures += 1;
        }
    }
    let _ = writeln!(log, "appended {APPENDS}; the log holds {total}; {}", if failures == 0 { "PASS" } else { "FAIL" });

    let period = if failures == 0 { Duration::from_millis(500) } else { Duration::from_millis(100) };
    loop {
        led.toggle();
        time::delay(period);
    }
}

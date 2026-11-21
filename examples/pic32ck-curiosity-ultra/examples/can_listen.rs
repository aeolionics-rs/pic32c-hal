// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A passive listener: CAN0 at 250 kbit/s in bus monitoring mode, which never
//! transmits or acknowledges, so it is safe on a live bus. Logs the first 40
//! frames in full, then once a second the frames received, the extended
//! identifiers' low bytes seen (J1939's source addresses), and the bus's
//! state, error counters and statistics.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::time::Duration;

use can_hal::{BusStatus, CanId, Event, Frame, ReceiveEvents, Statistics};
use cortex_m_rt::entry;
use panic_halt as _;
use pic32ck_curiosity_ultra::pic32c_hal::can::Mode;
use pic32ck_curiosity_ultra::time;
use pic32ck_curiosity_ultra::uart::Uart;

#[path = "common/can0.rs"]
#[allow(dead_code, reason = "shared by several examples; each uses some of it")]
mod can0;

pic32ck_curiosity_ultra::boot!();

/// The frames logged in full.
const SHOWN: u32 = 40;

#[entry]
fn main() -> ! {
    let peripherals = pic32ck_curiosity_ultra::init();
    time::init(cortex_m::Peripherals::take().unwrap());
    let mut log = Uart::init(peripherals.SERCOM5, peripherals.PA24, peripherals.PA25);
    let _ = writeln!(log, "\npic32ck-curiosity-ultra listen: CAN0, {} bit/s, bus monitoring", can0::CLASSIC_RATE);
    let mut can = match can0::open(peripherals.CAN0, can0::CLASSIC_RATE, Mode::Silent, peripherals.PB16, peripherals.PB17) {
        Ok(can) => can,
        Err(e) => {
            let _ = writeln!(log, "CAN0: {e:?}");
            loop {
                cortex_m::asm::wfi();
            }
        }
    };
    let (mut total, mut last_total) = (0u32, 0u32);
    let mut sources = [false; 256];
    let mut next_report = time::micros() + 1_000_000;
    loop {
        if let Ok(Some(Event::Received(received))) = can.receive_event_timeout(Duration::from_millis(1)) {
            total += 1;
            if let Frame::Can(frame) = received.frame() {
                if let CanId::Extended(id) = frame.id() {
                    sources[(id & 0xFF) as usize] = true;
                }
                if total <= SHOWN {
                    let _ = write!(log, "{total:3} ");
                    match frame.id() {
                        CanId::Extended(id) => {
                            let _ = write!(log, "{id:08X} (prio {} PGN {:5} src {:02X})", id >> 26, (id >> 8) & 0x3_FFFF, id & 0xFF);
                        }
                        CanId::Standard(id) => {
                            let _ = write!(log, "     {id:03X} (standard)");
                        }
                    }
                    let _ = write!(log, " [{}]", frame.dlc());
                    for byte in frame.data() {
                        let _ = write!(log, " {byte:02X}");
                    }
                    let _ = writeln!(log);
                }
            }
        }
        if time::micros() >= next_report {
            next_report += 1_000_000;
            let _ = write!(log, "[{} s] {total} frames, {}/s; sources", time::micros() / 1_000_000, total - last_total);
            last_total = total;
            for (address, _) in sources.iter().enumerate().filter(|(_, seen)| **seen) {
                let _ = write!(log, " {address:02X}");
            }
            let _ = writeln!(log, "; {:?} {:?} {:?}", can.bus_state(), can.error_counters(), can.statistics());
        }
    }
}

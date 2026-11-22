// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The far end of `can_ping`: CAN0 at 250 kbit/s (FD at 500 kbit/s and
//! 2 Mbit/s when built with `CAN_FD=1`), normal mode, sending back
//! every frame it receives with its identifier's top bit flipped (bit 10 of
//! a standard ID, bit 28 of an extended one), unless that bit is already
//! set: those are echoes, its own or another's. Logs a line every second
//! with its counts.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::time::Duration;

use can_hal::{BusStatus, CanFdFrame, CanFrame, CanId, ConfirmedTransmit, Event, Frame, ReceiveEvents, Statistics};
use cortex_m_rt::entry;
use panic_halt as _;
use pic32ck_curiosity_ultra::pic32c_hal::can::Mode;
use pic32ck_curiosity_ultra::time;
use pic32ck_curiosity_ultra::uart::Uart;
#[path = "common/can0.rs"]
#[allow(dead_code, reason = "shared by several examples; each uses some of it")]
mod can0;

pic32ck_curiosity_ultra::boot!();

/// The echo of `frame` (of the same kind, an FD one keeping its bit-rate
/// switching), or none if it is an echo already.
fn echo(frame: &Frame) -> Option<Frame> {
    let id = match frame.id() {
        CanId::Standard(id) if id & 0x400 == 0 => CanId::new_standard(id | 0x400)?,
        CanId::Extended(id) if id & 0x1000_0000 == 0 => CanId::new_extended(id | 0x1000_0000)?,
        _ => return None,
    };
    match frame {
        Frame::Can(can) if can.is_remote_frame() => CanFrame::new_remote(id, can.dlc()).map(Frame::Can),
        Frame::Can(can) => CanFrame::new_with_dlc(id, can.data(), can.dlc()).map(Frame::Can),
        Frame::Fd(fd) => CanFdFrame::new(id, fd.data(), fd.brs(), false).map(Frame::Fd),
    }
}

#[entry]
fn main() -> ! {
    let peripherals = pic32ck_curiosity_ultra::init();
    time::init(cortex_m::Peripherals::take().unwrap());
    let mut log = Uart::init(peripherals.SERCOM5, peripherals.PA24, peripherals.PA25);
    let _ = writeln!(log, "\npic32ck-curiosity-ultra echo: CPU {} Hz", pic32ck_curiosity_ultra::pic32c_hal::clocks::generator_hz(0).unwrap_or(0));
    if can0::FD {
        let _ = writeln!(log, "FD, {} and {} bit/s", can0::FD_RATES.0, can0::FD_RATES.1);
    } else {
        let _ = writeln!(log, "classic, {} bit/s", can0::CLASSIC_RATE);
    }
    let mut can = match can0::open_bus(peripherals.CAN0, Mode::Normal, peripherals.PB16, peripherals.PB17) {
        Ok(can) => can,
        Err(e) => {
            let _ = writeln!(log, "CAN0: {e:?}");
            loop {
                cortex_m::asm::wfi();
            }
        }
    };
    // Echoes waiting for a transmit buffer.
    let mut pending = [const { None }; 64];
    let (mut head, mut len) = (0usize, 0usize);
    let (mut echoed, mut dropped) = (0u32, 0u32);
    let mut next_report = time::micros();
    loop {
        if let Ok(Some(Event::Received(frame))) = can.receive_event_timeout(Duration::from_millis(1))
            && let Some(reply) = echo(frame.frame())
        {
            if len < pending.len() {
                pending[(head + len) % pending.len()] = Some(reply);
                len += 1;
            } else {
                dropped += 1;
            }
        }
        while len > 0 {
            let Some(reply) = pending[head].as_ref() else { break };
            if ConfirmedTransmit::<Frame>::try_transmit(&mut can, reply).is_err() {
                break;
            }
            pending[head] = None;
            head = (head + 1) % pending.len();
            len -= 1;
            echoed += 1;
        }
        if time::micros() >= next_report {
            next_report += 1_000_000;
            let _ = writeln!(log, "echoed {echoed}, dropped {dropped}, {:?} {:?} {:?}", can.bus_state(), can.error_counters(), can.statistics());
        }
    }
}

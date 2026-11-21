// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! CAN0 with pic32c-hal's `can`, as the CAN bench programs use it: PB16 (TX)
//! and PB17 (RX) to the board's ATA6561 (U701) and J704 (1 CANL, 2 GND,
//! 3 CANH; SW700 switches its split termination); the 80 MHz kernel clock
//! from generator 2.

use can_hal::SamplePoint;
use pic32c_hal::can::{BitTiming, Can, ClassicMessageRam, Config, Error, FdMessageRam, InterruptHandler, Mode, State};
use pic32c_hal::{Peri, bind_interrupts, peripherals};
use static_cell::ConstStaticCell;

use pic32ck_curiosity_ultra::{clocks, time};

bind_interrupts!(struct Irqs {
    CAN0 => InterruptHandler<peripherals::CAN0>;
});

/// The cycle counter as the driver's clock, its instants CPU cycles.
pub struct CycleClock;

impl pic32c_hal::can::Clock for CycleClock {
    type Instant = u64;

    fn now() -> u64 {
        time::cycles()
    }

    fn deadline(timeout: core::time::Duration) -> u64 {
        time::deadline(timeout)
    }

    fn has_passed(deadline: &u64) -> bool {
        time::cycles() >= *deadline
    }
}

/// Events the channel queues.
pub const EVENTS: usize = 64;
/// The channel.
pub type Channel = Can<CycleClock, EVENTS>;

static STATE: State<CycleClock, EVENTS> = State::new();
/// Whether the bus programs (`can_ping`, `can_echo`) run an FD channel: built
/// with `CAN_FD=1`, at [`FD_RATES`]; otherwise classic at 250 kbit/s.
pub const FD: bool = match option_env!("CAN_FD") {
    Some(value) => matches!(value.as_bytes(), b"1"),
    None => false,
};
/// The bus programs' FD rates: 500 kbit/s nominal, and the data rate from
/// `CAN_FD_DATA` at build time (bit/s), 2 Mbit/s if unset.
pub const FD_RATES: (u32, u32) = (
    500_000,
    match option_env!("CAN_FD_DATA") {
        Some(value) => parse(value.as_bytes()),
        None => 2_000_000,
    },
);

/// A decimal number, at compile time; a build error if it isn't one.
const fn parse(digits: &[u8]) -> u32 {
    assert!(!digits.is_empty(), "CAN_FD_DATA is empty");
    let mut value = 0u32;
    let mut i = 0;
    while i < digits.len() {
        assert!(digits[i].is_ascii_digit(), "CAN_FD_DATA isn't a number");
        value = value * 10 + (digits[i] - b'0') as u32;
        i += 1;
    }
    value
}
/// The bus programs' classic rate.
pub const CLASSIC_RATE: u32 = 250_000;

/// For a classic channel.
static RAM: ConstStaticCell<ClassicMessageRam> = ConstStaticCell::new(ClassicMessageRam::new());
/// For an FD channel.
static FD_RAM: ConstStaticCell<FdMessageRam> = ConstStaticCell::new(FdMessageRam::new());

/// CAN0 at `bitrate`, sampled at 87.5 %, classic frames only (in
/// [`ClassicMessageRam`]), in `mode`, started. Call once, after
/// [`pic32ck_curiosity_ultra::init`].
///
/// # Errors
///
/// The driver's, if the timing doesn't fit or the controller doesn't start.
pub fn open(can: Peri<'static, peripherals::CAN0>, bitrate: u32, mode: Mode, tx: Peri<'static, peripherals::PB16>, rx: Peri<'static, peripherals::PB17>) -> Result<Channel, Error> {
    let (_, config) = config(bitrate, mode)?;
    start(Can::new(can, Irqs, tx, rx, RAM.take(), &STATE, config)?)
}

/// CAN0 as [`open`] but taking FD frames too (in [`FdMessageRam`]), their
/// data phase at `data_bitrate`, sampled at 80 %.
///
/// # Errors
///
/// As [`open`].
pub fn open_fd(can: Peri<'static, peripherals::CAN0>, bitrate: u32, data_bitrate: u32, mode: Mode, tx: Peri<'static, peripherals::PB16>, rx: Peri<'static, peripherals::PB17>) -> Result<Channel, Error> {
    let (kernel_hz, config) = config(bitrate, mode)?;
    let data = BitTiming::solve_data(kernel_hz, data_bitrate, SamplePoint::from_per_mille(800))?;
    start(Can::new(can, Irqs, tx, rx, FD_RAM.take(), &STATE, config.with_data(data))?)
}

/// The kernel clock's rate, and the nominal timing for it.
fn config(bitrate: u32, mode: Mode) -> Result<(u32, Config), Error> {
    let kernel_hz = pic32c_hal::clocks::generator_hz(clocks::GEN_CAN).ok_or(Error::NoClock)?;
    let nominal = BitTiming::solve_nominal(kernel_hz, bitrate, SamplePoint::from_per_mille(875))?;
    Ok((kernel_hz, Config::new(nominal).with_mode(mode).with_generator(clocks::GEN_CAN)))
}

fn start(mut can: Channel) -> Result<Channel, Error> {
    can.start()?;
    Ok(can)
}

/// CAN0 for the bus programs: FD at [`FD_RATES`] when built with
/// `CAN_FD=1`, else classic at [`CLASSIC_RATE`].
///
/// # Errors
///
/// As [`open`].
pub fn open_bus(can: Peri<'static, peripherals::CAN0>, mode: Mode, tx: Peri<'static, peripherals::PB16>, rx: Peri<'static, peripherals::PB17>) -> Result<Channel, Error> {
    if FD { open_fd(can, FD_RATES.0, FD_RATES.1, mode, tx, rx) } else { open(can, CLASSIC_RATE, mode, tx, rx) }
}

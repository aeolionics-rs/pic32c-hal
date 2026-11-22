// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The board's two CAN ports, each on an ATA6561, as the CAN examples share
//! them: CAN0 on J502 (PA11 TX, PA10 RX) as a classic channel, CAN1 on J503
//! (PD13 TX, PD12 RX) as an FD one, each with embassy-time for its clock.

use core::time::Duration;

use can_hal::SamplePoint;
use pic32c_hal::can::{BitTiming, Can, ClassicMessageRam, Config, Error, FdMessageRam, InterruptHandler, Mode, State};
use pic32c_hal::{Peri, bind_interrupts, peripherals};
use pic32cm_sg00_curiosity_pro::GEN_CAN;
use static_cell::ConstStaticCell;

bind_interrupts!(pub struct Irqs {
    CAN0_INT0 => InterruptHandler<peripherals::CAN0>;
    CAN1_INT0 => InterruptHandler<peripherals::CAN1>;
});

/// The driver's clock: embassy-time.
pub struct TimeClock;

impl pic32c_hal::can::Clock for TimeClock {
    type Instant = embassy_time::Instant;

    fn now() -> embassy_time::Instant {
        embassy_time::Instant::now()
    }

    fn deadline(timeout: Duration) -> embassy_time::Instant {
        embassy_time::Instant::now() + embassy_time::Duration::from_micros(u64::try_from(timeout.as_micros()).unwrap_or(u64::MAX))
    }

    fn has_passed(deadline: &embassy_time::Instant) -> bool {
        embassy_time::Instant::now() >= *deadline
    }
}

const EVENTS: usize = 64;
pub type Channel = Can<TimeClock, EVENTS>;
static STATE0: State<TimeClock, EVENTS> = State::new();
static STATE1: State<TimeClock, EVENTS> = State::new();
static RAM0: ConstStaticCell<ClassicMessageRam> = ConstStaticCell::new(ClassicMessageRam::new());
static RAM1: ConstStaticCell<FdMessageRam> = ConstStaticCell::new(FdMessageRam::new());

/// The bench's classic rate (T1, the NMEA 2000 backbones), and its FD
/// rates (T2): nominal and data.
pub const CLASSIC_RATE: u32 = 250_000;
pub const FD_RATES: (u32, u32) = (500_000, 2_000_000);

/// CAN0 at `rate`, classic, sampling at 87.5 %, in `mode`, started.
///
/// # Errors
///
/// If the rate can't be had from the CAN clock, or the controller won't
/// start.
pub fn open(can: Peri<'static, peripherals::CAN0>, rate: u32, mode: Mode, tx: Peri<'static, peripherals::PA11>, rx: Peri<'static, peripherals::PA10>) -> Result<Channel, Error> {
    let kernel_hz = pic32c_hal::clocks::generator_hz(GEN_CAN).unwrap_or(0);
    let nominal = BitTiming::solve_nominal(kernel_hz, rate, SamplePoint::from_per_mille(875))?;
    let config = Config::new(nominal).with_mode(mode).with_generator(GEN_CAN);
    let mut can = Can::new(can, Irqs, tx, rx, RAM0.take(), &STATE0, config)?;
    can.start()?;
    Ok(can)
}

/// CAN1 as FD at [`FD_RATES`], the nominal phase sampled at 87.5 % and the
/// data phase at 80 %, in `mode`, started.
///
/// # Errors
///
/// As [`open`].
pub fn open_fd(can: Peri<'static, peripherals::CAN1>, mode: Mode, tx: Peri<'static, peripherals::PD13>, rx: Peri<'static, peripherals::PD12>) -> Result<Channel, Error> {
    let kernel_hz = pic32c_hal::clocks::generator_hz(GEN_CAN).unwrap_or(0);
    let nominal = BitTiming::solve_nominal(kernel_hz, FD_RATES.0, SamplePoint::from_per_mille(875))?;
    let data = BitTiming::solve_data(kernel_hz, FD_RATES.1, SamplePoint::from_per_mille(800))?;
    let config = Config::new(nominal).with_data(data).with_mode(mode).with_generator(GEN_CAN);
    let mut can = Can::new(can, Irqs, tx, rx, RAM1.take(), &STATE1, config)?;
    can.start()?;
    Ok(can)
}

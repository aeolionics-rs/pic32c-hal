// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The boards' clocks, as pic32c-hal sets them up ([`CONFIG`]): the 12 MHz
//! crystal (Y300, on PD00/PD01) through PLL0 to a 960 MHz VCO, divided to
//! 120 MHz for the CPU (generator 0), 80 MHz for CAN (generator 2), 48 MHz
//! for USB and the UART (generator 3) and 16 MHz for the time driver
//! (generator 4). [`crate::init`] brings them up and checks they match the
//! constants here.

use pic32c_hal::clocks::{Config, Xosc};

/// The clock tree.
pub const CONFIG: Config = Config::pll_from_xosc(Xosc::Crystal(12_000_000));

/// The CPU's clock, generator 0.
pub const CPU_HZ: u32 = 120_000_000;
/// The CAN kernel clock, generator 2.
pub const CAN_HZ: u32 = 80_000_000;
/// USB's and the UART's clock, generator 3.
pub const MAIN_48_HZ: u32 = 48_000_000;
/// The time driver's TCC clock, generator 4.
pub const TIMER_HZ: u32 = 16_000_000;

/// The generators.
pub const GEN_CPU: usize = 0;
pub const GEN_CAN: usize = 2;
pub const GEN_48: usize = 3;
pub const GEN_TIMER: usize = 4;

/// GCLK peripheral channels.
pub const CHANNEL_SERCOM5_CORE: usize = 26;
pub const CHANNEL_CAN0: usize = 39;
pub const CHANNEL_CAN1: usize = 40;
pub const CHANNEL_USB: usize = 46;

pub use pic32c_hal::clocks::enable_channel;

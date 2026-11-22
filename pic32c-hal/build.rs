// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Turns the chip feature into cfgs naming its family and the versions of
//! the peripheral IP it has, as embassy-stm32 does: drivers branch on the IP
//! (`sercom_v6`, `port_v1`, ...), not on the chip, so a new chip with the
//! same IP gets the same code.
//!
//! - `pic32ck`: the PIC32CK SG/GC (Cortex-M33). `sercom_v6` (FIFO, 32-bit
//!   DATA, four interrupt lines per SERCOM), `port_v4` (open drain, 2-bit
//!   slew), `dma_axi` (its own DMA IP), `adc_msh`, `rtc_v3`, `wdt_v2`,
//!   `eic_v1`, `cmcc`, `fcw` and `fcw_v1` (FCW 03433), `trng`, `mcan`,
//!   `gmac`, `usb_fs`, `usb_hs` (a MUSB core).
//! - `pic32cm_pl`: the PIC32CM PL10 (Cortex-M0+). `sercom_v1` (no FIFO,
//!   16-bit DATA, one interrupt line per SERCOM, bus clocks off at reset),
//!   `port_v1` (no open drain, 1-bit slew), `dma_u2223` (the SG00's DMAC,
//!   with 2 channels), `wdt_v1` (the CK's WDT with SYNCBUSY.CLEAR at bit 4
//!   and its own reset causes), `nvmctrl` (its flash controller: 512-byte
//!   pages, 32-bit word writes), `rtc_v3` (the CK's RTC; its OSC32KCTRL
//!   differs), `eic_v1` (the CK's EIC with one vector and no PINSTATE)
//!   and `adc_06272` (its ADC, IP 06272, laid out like an AVR Dx's).
//! - `pic32cm_sg`: the PIC32CM SG00 (Cortex-M23). The PIC32CK's
//!   `sercom_v6`, `port_v4`, `wdt_v2` and `mcan` (its `M_CAN` is 3.3.0, in a
//!   wrapper), `dma_u2223` (the SAM D21/C21 kind of DMAC), and `fcw` and
//!   `fcw_v2` (FCW 04971: NVMOP in its own register, 16-byte quad writes,
//!   one panel); the other modules whose IP it shares wait to be checked
//!   on it.
//!
//! `dma` is set with either DMA IP: the drivers that take DMA channels
//! (`uart`, `spi`) need only that.
//!
//! With feature `time-driver`, it also sets `time_driver` to the counter the
//! time driver takes: the one a `time-driver-*` feature names, else the
//! chip's usual one (`tcc0` on the PIC32CK and the SG00, `tc0_tc1` on the
//! PL10).

use std::env;

const ALL: &[&str] = &[
    "pic32ck",
    "pic32cm_pl",
    "pic32cm_sg",
    "sercom_v6",
    "sercom_v1",
    "port_v4",
    "port_v1",
    "dma",
    "dma_axi",
    "dma_u2223",
    "adc_msh",
    "adc_06272",
    "rtc_v3",
    "wdt_v2",
    "wdt_v1",
    "eic_v1",
    "cmcc",
    "fcw",
    "fcw_v1",
    "fcw_v2",
    "nvmctrl",
    "trng",
    "mcan",
    "gmac",
    "usb_fs",
    "usb_hs",
];

/// The counters the time driver can take on each family, the first the
/// usual one; each is feature `time-driver-<name>` with `_` as `-`.
const PIC32CK_TIMERS: &[&str] = &["tcc0", "tcc1", "tcc2", "tcc3", "tcc4", "tcc5", "tcc6", "tcc7"];
const PIC32CM_PL_TIMERS: &[&str] = &["tc0_tc1", "tc0", "tc1", "tc2"];
const PIC32CM_SG_TIMERS: &[&str] = &["tcc0", "tcc1", "tcc2", "tcc3", "tcc4", "tcc5", "tcc6"];

const PIC32CK: &[&str] = &[
    "pic32ck",
    "sercom_v6",
    "port_v4",
    "dma",
    "dma_axi",
    "adc_msh",
    "rtc_v3",
    "wdt_v2",
    "eic_v1",
    "cmcc",
    "fcw",
    "fcw_v1",
    "trng",
    "mcan",
    "gmac",
    "usb_fs",
    "usb_hs",
];

const PIC32CM_PL: &[&str] = &["pic32cm_pl", "sercom_v1", "port_v1", "dma", "dma_u2223", "wdt_v1", "nvmctrl", "rtc_v3", "eic_v1", "adc_06272"];

const PIC32CM_SG: &[&str] = &["pic32cm_sg", "sercom_v6", "port_v4", "dma", "dma_u2223", "fcw", "fcw_v2", "rtc_v3", "eic_v1", "adc_msh", "wdt_v2", "mcan", "usb_fs"];

fn main() {
    for cfg in ALL {
        println!("cargo::rustc-check-cfg=cfg({cfg})");
    }
    let timers: Vec<String> = PIC32CK_TIMERS.iter().chain(PIC32CM_PL_TIMERS).map(|t| format!("\"{t}\"")).collect();
    println!("cargo::rustc-check-cfg=cfg(time_driver, values({}))", timers.join(", "));
    let feature = |name: &str| env::var_os(format!("CARGO_FEATURE_{}", name.to_uppercase().replace('-', "_"))).is_some();
    let (cfgs, timers): (&[&str], &[&str]) = if feature("pic32ck2051sg01144") || feature("pic32ck2051gc01144") {
        (PIC32CK, PIC32CK_TIMERS)
    } else if feature("pic32cm6408pl10048") {
        (PIC32CM_PL, PIC32CM_PL_TIMERS)
    } else if feature("pic32cm5112sg00100") {
        (PIC32CM_SG, PIC32CM_SG_TIMERS)
    } else {
        (&[], &[])
    };
    for cfg in cfgs {
        println!("cargo::rustc-cfg={cfg}");
    }
    let named: Vec<&str> = PIC32CK_TIMERS.iter().chain(PIC32CM_PL_TIMERS).copied().filter(|t| feature(&format!("time-driver-{t}"))).collect();
    let timer = match named.as_slice() {
        [] => timers.first().copied().filter(|_| feature("time-driver")),
        [one] if timers.contains(one) => Some(*one),
        [one] => panic!("feature time-driver-{}: this chip has no such counter (it has {})", one.replace('_', "-"), timers.join(", ").replace('_', "-")),
        more => panic!("time-driver-* features for more than one counter: {}", more.join(", ").replace('_', "-")),
    };
    if let Some(timer) = timer {
        println!("cargo::rustc-cfg=time_driver=\"{timer}\"");
    }
}

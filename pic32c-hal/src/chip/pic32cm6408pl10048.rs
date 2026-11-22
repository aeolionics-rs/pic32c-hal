// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The PIC32CM6408PL10048 (Cortex-M0+, 24 MHz, 64 KB flash, 8 KB SRAM): its
//! interrupts (from the ATDF, in vector order), the peripheral singletons
//! the drivers take so far, and the 48-pin package's 42 GPIO pins. Generated
//! from Microchip's PIC32CM-PL_DFP 1.6.499 ATDF.

pub use pic32cm_pl10_pac as pac;

/// Program flash: 64 KB.
pub const FLASH_SIZE: usize = 64 << 10;

/// PORT, on the IOBUS-free APB view.
pub(crate) const PORT: usize = 0x4100_0000;

/// The EIC.
#[cfg(feature = "exti")]
pub(crate) const EIC: usize = 0x4000_2800;

embassy_hal_internal::interrupt_mod!(SYSTEM, WDT, RTC, EIC, NVMCTRL, DMAC, EVSYS, SERCOM0, SERCOM1, TC0, TC1, TC2, TCC0, ADC0, AC,);

embassy_hal_internal::peripherals!(
    PA00,
    PA01,
    PA02,
    PA03,
    PA04,
    PA05,
    PA06,
    PA07,
    PA08,
    PA09,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PA16,
    PA17,
    PA18,
    PA19,
    PA20,
    PA21,
    PA22,
    PA23,
    PA24,
    PA25,
    PA26,
    PA27,
    PA28,
    PA29,
    PA30,
    PA31,
    PB00,
    PB01,
    PB02,
    PB03,
    PB04,
    PB05,
    PB08,
    PB09,
    PB10,
    PB11,
    SERCOM0,
    SERCOM1,
    NVMCTRL,
    WDT,
    RTC,
    EXTINT0,
    EXTINT1,
    EXTINT2,
    EXTINT3,
    EXTINT4,
    EXTINT5,
    EXTINT6,
    EXTINT7,
    EXTINT8,
    EXTINT9,
    EXTINT10,
    EXTINT11,
    EXTINT12,
    EXTINT13,
    EXTINT14,
    EXTINT15,
    // The DMAC's channels.
    DMA_CH0,
    DMA_CH1,
    // The time driver's counter, if on, is its own (TC0 and TC1 both when
    // it pairs them).
    #[cfg(not(any(time_driver = "tc0", time_driver = "tc0_tc1")))]
    TC0,
    #[cfg(not(any(time_driver = "tc1", time_driver = "tc0_tc1")))]
    TC1,
    #[cfg(not(time_driver = "tc2"))]
    TC2,
);

crate::gpio::impl_pin! {
    PA00 = 0, 0;
    PA01 = 0, 1;
    PA02 = 0, 2;
    PA03 = 0, 3;
    PA04 = 0, 4;
    PA05 = 0, 5;
    PA06 = 0, 6;
    PA07 = 0, 7;
    PA08 = 0, 8;
    PA09 = 0, 9;
    PA10 = 0, 10;
    PA11 = 0, 11;
    PA12 = 0, 12;
    PA13 = 0, 13;
    PA14 = 0, 14;
    PA15 = 0, 15;
    PA16 = 0, 16;
    PA17 = 0, 17;
    PA18 = 0, 18;
    PA19 = 0, 19;
    PA20 = 0, 20;
    PA21 = 0, 21;
    PA22 = 0, 22;
    PA23 = 0, 23;
    PA24 = 0, 24;
    PA25 = 0, 25;
    PA26 = 0, 26;
    PA27 = 0, 27;
    PA28 = 0, 28;
    PA29 = 0, 29;
    PA30 = 0, 30;
    PA31 = 0, 31;
    PB00 = 1, 0;
    PB01 = 1, 1;
    PB02 = 1, 2;
    PB03 = 1, 3;
    PB04 = 1, 4;
    PB05 = 1, 5;
    PB08 = 1, 8;
    PB09 = 1, 9;
    PB10 = 1, 10;
    PB11 = 1, 11;
}

crate::uart::impl_sercom! {
    SERCOM0 = 0x4200_0400, number 0, channel 6, apbc 0x2, SERCOM0;
    SERCOM1 = 0x4200_0800, number 1, channel 8, apbc 0x4, SERCOM1;
}

// SERCOM pads, from the ATDF's pin multiplexing: each pin's pad and
// function. The PL10 has no I/O sets (any pads of a SERCOM go together), so
// every pad is in set 0.
crate::sercom::impl_pads! {
    SERCOM0: PA10 pad 0 D ioset 0, PA00 pad 0 C ioset 0, PA04 pad 0 C ioset 0, PA11 pad 1 D ioset 0, PA01 pad 1 C ioset 0, PA05 pad 1 C ioset 0, PA08 pad 2 D ioset 0, PA31 pad 2 D ioset 0, PA02 pad 2 C ioset 0, PA06 pad 2 C ioset 0, PA09 pad 3 D ioset 0, PA03 pad 3 C ioset 0, PA07 pad 3 C ioset 0;
    SERCOM1: PA00 pad 0 D ioset 0, PA26 pad 0 D ioset 0, PB00 pad 0 D ioset 0, PA10 pad 0 C ioset 0, PA14 pad 0 C ioset 0, PA20 pad 0 C ioset 0, PA31 pad 0 C ioset 0, PA01 pad 1 D ioset 0, PA27 pad 1 D ioset 0, PB01 pad 1 D ioset 0, PA11 pad 1 C ioset 0, PA15 pad 1 C ioset 0, PA21 pad 1 C ioset 0, PA02 pad 2 D ioset 0, PA24 pad 2 D ioset 0, PB02 pad 2 D ioset 0, PA08 pad 2 C ioset 0, PA12 pad 2 C ioset 0, PA22 pad 2 C ioset 0, PA03 pad 3 D ioset 0, PA25 pad 3 D ioset 0, PB03 pad 3 D ioset 0, PA09 pad 3 C ioset 0, PA13 pad 3 C ioset 0, PA23 pad 3 C ioset 0;
}

// The EIC's lines, and each pin's, from the ATDF's EXTINT signals (all on
// function A).
#[cfg(feature = "exti")]
crate::exti::impl_channels!(
    EXTINT0 = 0,
    EXTINT1 = 1,
    EXTINT2 = 2,
    EXTINT3 = 3,
    EXTINT4 = 4,
    EXTINT5 = 5,
    EXTINT6 = 6,
    EXTINT7 = 7,
    EXTINT8 = 8,
    EXTINT9 = 9,
    EXTINT10 = 10,
    EXTINT11 = 11,
    EXTINT12 = 12,
    EXTINT13 = 13,
    EXTINT14 = 14,
    EXTINT15 = 15
);

#[cfg(feature = "exti")]
crate::exti::impl_exti_pins!(
    PA00 = EXTINT0,
    PA02 = EXTINT2,
    PA03 = EXTINT3,
    PA04 = EXTINT4,
    PA05 = EXTINT5,
    PA06 = EXTINT6,
    PA08 = EXTINT8,
    PA09 = EXTINT9,
    PA10 = EXTINT10,
    PA11 = EXTINT11,
    PA12 = EXTINT12,
    PA13 = EXTINT13,
    PA14 = EXTINT14,
    PA15 = EXTINT15,
    PA16 = EXTINT0,
    PA17 = EXTINT1,
    PA18 = EXTINT2,
    PA19 = EXTINT3,
    PA20 = EXTINT4,
    PA21 = EXTINT5,
    PA22 = EXTINT6,
    PA23 = EXTINT7,
    PA24 = EXTINT8,
    PA25 = EXTINT9,
    PA26 = EXTINT10,
    PA27 = EXTINT11,
    PA28 = EXTINT12,
    PA29 = EXTINT13,
    PA31 = EXTINT15,
    PB00 = EXTINT0,
    PB01 = EXTINT1,
    PB02 = EXTINT2,
    PB03 = EXTINT3,
    PB04 = EXTINT4,
    PB05 = EXTINT5,
    PB08 = EXTINT8,
    PB09 = EXTINT9,
    PB10 = EXTINT10,
    PB11 = EXTINT11,
);

// The NMI's pins (function A).
#[cfg(feature = "exti")]
crate::exti::impl_nmi_pins!(PA01, PA07, PA30);

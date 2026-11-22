// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The PIC32CM5112SG00100 (Cortex-M23, 72 MHz, 512 KB program flash,
//! 128 KB SRAM): its interrupts (from the PAC's `Interrupt`, in vector
//! order), the peripheral singletons the drivers take so far, and the
//! 100-pin package's 80 GPIO pins. Generated from Microchip's
//! PIC32CM-SG00_DFP 1.2.83 ATDF and SVD.
//!
//! Its peripherals are the PIC32CK's IP, mostly at the same versions
//! (SERCOM, PORT, OSCCTRL, TCC, WDT, RTC, EIC, ADC, the full-speed USB), at
//! other addresses; its DMAC and flash controller are other versions, and
//! its CAN a later `M_CAN` in a wrapper.
//! PORT has one view, at `0x4480_0000`, which Secure code reaches.

pub use pic32cm_sg00_pac as pac;

/// Program flash: 512 KB.
pub const FLASH_SIZE: usize = 512 << 10;

/// PORT.
pub(crate) const PORT: usize = 0x4480_0000;

/// The EIC, which has one view on the SG00.
#[cfg(feature = "exti")]
pub(crate) const EIC: usize = 0x4402_0000;

/// The pin with only the NMI on it.
#[cfg(feature = "exti")]
pub(crate) type NmiPin = peripherals::PD00;

embassy_hal_internal::interrupt_mod!(
    FCR,
    FCW,
    PM,
    SUPC,
    OSCCTRL_XOSCRDY,
    OSCCTRL_DFLLRDY,
    OSCCTRL_PLLLOCKR_0,
    OSC32KCTRL_XOSC32KRDY,
    OSC32KCTRL_XOSC32KFAIL,
    MCLK,
    FREQM,
    WDT,
    RTC_TAMPER,
    RTC_OVF,
    RTC_PER0,
    RTC_CMP0,
    EIC_EXTINT0,
    EIC_EXTINT1,
    EIC_EXTINT2,
    EIC_EXTINT3,
    EIC_EXTINT4,
    EIC_EXTINT5,
    EIC_EXTINT6,
    EIC_EXTINT7,
    EIC_EXTINT8,
    EIC_EXTINT9,
    EIC_EXTINT10,
    EIC_EXTINT11,
    EIC_EXTINT12,
    EIC_EXTINT13,
    EIC_EXTINT14,
    EIC_EXTINT15,
    EIC_NSCHK,
    TRAM,
    MCRAMC,
    CAN0_INT0,
    CAN0_INT1,
    CAN0_BERR,
    CAN1_INT0,
    CAN1_INT1,
    CAN1_BERR,
    PORT,
    DMAC_TCMPL0,
    DMAC_TCMPL1,
    DMAC_TCMPL2,
    DMAC_TCMPL3,
    DMAC_TCMPL4,
    HMATRIX2,
    EVSYS_EVD0,
    EVSYS_EVD1,
    EVSYS_EVD2,
    EVSYS_EVD3,
    EVSYS_EVD4,
    EVSYS_EVD5,
    EVSYS_EVD6,
    EVSYS_EVD7,
    EVSYS_EVD8,
    EVSYS_EVD9,
    EVSYS_EVD10,
    EVSYS_EVD11,
    SERCOM0_6,
    SERCOM0_5,
    SERCOM0_0,
    SERCOM0_1,
    SERCOM0_2,
    SERCOM0_3,
    SERCOM0_4,
    SERCOM1_6,
    SERCOM1_5,
    SERCOM1_0,
    SERCOM1_1,
    SERCOM1_2,
    SERCOM1_3,
    SERCOM1_4,
    SERCOM2_6,
    SERCOM2_5,
    SERCOM2_0,
    SERCOM2_1,
    SERCOM2_2,
    SERCOM2_3,
    SERCOM2_4,
    SERCOM3_6,
    SERCOM3_5,
    SERCOM3_0,
    SERCOM3_1,
    SERCOM3_2,
    SERCOM3_3,
    SERCOM3_4,
    TCC0_DFS,
    TCC0_CNT,
    TCC0_MC0,
    TCC0_MC1,
    TCC1_DFS,
    TCC1_CNT,
    TCC1_MC0,
    TCC1_MC1,
    TCC2_DFS,
    TCC2_CNT,
    TCC2_MC0,
    TCC2_MC1,
    TCC3_DFS,
    TCC3_CNT,
    TCC3_MC0,
    TCC3_MC1,
    SERCOM4_6,
    SERCOM4_5,
    SERCOM4_0,
    SERCOM4_1,
    SERCOM4_2,
    SERCOM4_3,
    SERCOM4_4,
    SERCOM5_6,
    SERCOM5_5,
    SERCOM5_0,
    SERCOM5_1,
    SERCOM5_2,
    SERCOM5_3,
    SERCOM5_4,
    TCC4_DFS,
    TCC4_CNT,
    TCC4_MC0,
    TCC4_MC1,
    TCC5_DFS,
    TCC5_CNT,
    TCC5_MC0,
    TCC5_MC1,
    TCC6_DFS,
    TCC6_CNT,
    TCC6_MC0,
    TCC6_MC1,
    ADC_REQ0,
    ADC_REQ1,
    AC,
    PTC,
    USB_EORSMDNRSM,
    USB_SOFHSOF,
    USB_TRCPT00,
    USB_TRCPT10,
    AT,
    HSM,
    PUF,
);

embassy_hal_internal::peripherals!(
    CAN0,
    CAN1,
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
    PB00,
    PB01,
    PB02,
    PB03,
    PB04,
    PB05,
    PB06,
    PB07,
    PB08,
    PB09,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PB16,
    PB17,
    PC00,
    PC01,
    PC02,
    PC03,
    PC04,
    PC05,
    PC06,
    PC07,
    PC08,
    PC09,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PC16,
    PC17,
    PC18,
    PC19,
    PD00,
    PD01,
    PD02,
    PD03,
    PD04,
    PD05,
    PD06,
    PD07,
    PD08,
    PD09,
    PD10,
    PD11,
    PD12,
    PD13,
    PD14,
    PD15,
    PD16,
    PD17,
    PD18,
    PD19,
    PD20,
    SERCOM0,
    SERCOM1,
    SERCOM2,
    SERCOM3,
    SERCOM4,
    SERCOM5,
    WDT,
    ADC,
    FCW,
    RTC,
    USB,
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
    DMA_CH2,
    DMA_CH3,
    DMA_CH4,
    DMA_CH5,
    DMA_CH6,
    DMA_CH7,
    // The time driver's counter, if on, is its own.
    #[cfg(not(time_driver = "tcc0"))]
    TCC0,
    #[cfg(not(time_driver = "tcc1"))]
    TCC1,
    #[cfg(not(time_driver = "tcc2"))]
    TCC2,
    #[cfg(not(time_driver = "tcc3"))]
    TCC3,
    #[cfg(not(time_driver = "tcc4"))]
    TCC4,
    #[cfg(not(time_driver = "tcc5"))]
    TCC5,
    #[cfg(not(time_driver = "tcc6"))]
    TCC6,
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
    PB00 = 1, 0;
    PB01 = 1, 1;
    PB02 = 1, 2;
    PB03 = 1, 3;
    PB04 = 1, 4;
    PB05 = 1, 5;
    PB06 = 1, 6;
    PB07 = 1, 7;
    PB08 = 1, 8;
    PB09 = 1, 9;
    PB10 = 1, 10;
    PB11 = 1, 11;
    PB12 = 1, 12;
    PB13 = 1, 13;
    PB14 = 1, 14;
    PB15 = 1, 15;
    PB16 = 1, 16;
    PB17 = 1, 17;
    PC00 = 2, 0;
    PC01 = 2, 1;
    PC02 = 2, 2;
    PC03 = 2, 3;
    PC04 = 2, 4;
    PC05 = 2, 5;
    PC06 = 2, 6;
    PC07 = 2, 7;
    PC08 = 2, 8;
    PC09 = 2, 9;
    PC10 = 2, 10;
    PC11 = 2, 11;
    PC12 = 2, 12;
    PC13 = 2, 13;
    PC14 = 2, 14;
    PC15 = 2, 15;
    PC16 = 2, 16;
    PC17 = 2, 17;
    PC18 = 2, 18;
    PC19 = 2, 19;
    PD00 = 3, 0;
    PD01 = 3, 1;
    PD02 = 3, 2;
    PD03 = 3, 3;
    PD04 = 3, 4;
    PD05 = 3, 5;
    PD06 = 3, 6;
    PD07 = 3, 7;
    PD08 = 3, 8;
    PD09 = 3, 9;
    PD10 = 3, 10;
    PD11 = 3, 11;
    PD12 = 3, 12;
    PD13 = 3, 13;
    PD14 = 3, 14;
    PD15 = 3, 15;
    PD16 = 3, 16;
    PD17 = 3, 17;
    PD18 = 3, 18;
    PD19 = 3, 19;
    PD20 = 3, 20;
}

// Each SERCOM's vectors: DRE (`_0`), TXC (`_1`), RXC (`_2`) and ERROR
// (`_6`; RXBRK has its own, `_5`). GCLK channels from the ATDF's
// `GCLK_ID_CORE`.
crate::uart::impl_sercom! {
    SERCOM0 = 0x4480_8000, number 0, channel 21, SERCOM0_0, SERCOM0_1, SERCOM0_2, SERCOM0_6;
    SERCOM1 = 0x4480_A000, number 1, channel 22, SERCOM1_0, SERCOM1_1, SERCOM1_2, SERCOM1_6;
    SERCOM2 = 0x4480_C000, number 2, channel 23, SERCOM2_0, SERCOM2_1, SERCOM2_2, SERCOM2_6;
    SERCOM3 = 0x4480_E000, number 3, channel 24, SERCOM3_0, SERCOM3_1, SERCOM3_2, SERCOM3_6;
    SERCOM4 = 0x4482_0000, number 4, channel 28, SERCOM4_0, SERCOM4_1, SERCOM4_2, SERCOM4_6;
    SERCOM5 = 0x4482_2000, number 5, channel 29, SERCOM5_0, SERCOM5_1, SERCOM5_2, SERCOM5_6;
}

// SERCOM pads, generated from the ATDF's pin multiplexing by
// tools/pin_tables.py: each pin's pad, function and IOSET. The ATDF gives
// SERCOM1's PD00 and PD01 no IOSET; they are put in set 3 here
// (`SERCOM1:PD00=3 SERCOM1:PD01=3`), with PC12 and PC13, which a UART with
// RTS and CTS used beside them on the bench. The data sheet's table 7-11
// puts them in set 1 instead.
crate::sercom::impl_pads! {
    SERCOM0: PC00 pad 0 D ioset 1, PC01 pad 1 D ioset 1, PC02 pad 2 D ioset 1, PC03 pad 3 D ioset 1, PB15 pad 0 D ioset 2, PB16 pad 1 D ioset 2, PB17 pad 2 D ioset 2, PC11 pad 3 D ioset 2;
    SERCOM1: PC06 pad 2 D ioset 1, PC05 pad 3 D ioset 1, PD10 pad 2 D ioset 2, PD11 pad 3 D ioset 2, PC15 pad 0 D ioset 3, PD01 pad 0 D ioset 3, PC14 pad 1 D ioset 3, PD00 pad 1 D ioset 3, PC13 pad 2 D ioset 3, PC12 pad 3 D ioset 3;
    SERCOM2: PC04 pad 0 D ioset 1, PD06 pad 1 D ioset 1, PD07 pad 2 D ioset 1, PD08 pad 3 D ioset 1, PC19 pad 0 D ioset 2, PC18 pad 1 D ioset 2, PC17 pad 2 D ioset 2, PC16 pad 3 D ioset 2;
    SERCOM3: PD09 pad 0 D ioset 1, PA00 pad 1 D ioset 1, PA01 pad 2 D ioset 1, PA02 pad 3 D ioset 1, PD16 pad 0 D ioset 2, PD15 pad 1 D ioset 2, PD17 pad 2 D ioset 2, PD14 pad 3 D ioset 2;
    SERCOM4: PA03 pad 0 D ioset 1, PA04 pad 1 D ioset 1, PA05 pad 2 D ioset 1, PA06 pad 3 D ioset 1, PD19 pad 0 D ioset 2, PD20 pad 1 D ioset 2, PD18 pad 2 D ioset 2, PA14 pad 3 D ioset 2;
    SERCOM5: PB03 pad 0 D ioset 1, PB04 pad 1 D ioset 1, PB05 pad 2 D ioset 1, PB06 pad 3 D ioset 1, PA16 pad 0 D ioset 2, PA17 pad 1 D ioset 2, PA18 pad 2 D ioset 2, PA15 pad 3 D ioset 2;
}

// The `M_CAN` registers, 0x100 into each controller's wrapper; their line 0
// vectors; their GCLK channels from the ATDF.
#[cfg(feature = "can-hal")]
crate::can::instance::impl_can! {
    CAN0 = 0x4402_E100, channel 6, CAN0_INT0;
    CAN1 = 0x4403_0100, channel 7, CAN1_INT0;
}

// CAN pins, generated as the SERCOM pads are.
#[cfg(feature = "can-hal")]
crate::can::instance::impl_can_pins! {
    CAN0: rx PC01 H ioset 1, tx PC02 H ioset 1, rx PD06 H ioset 2, tx PD08 H ioset 2, rx PA10 H ioset 3, tx PA11 H ioset 3;
    CAN1: rx PC05 H ioset 1, tx PC06 H ioset 1, rx PA00 H ioset 2, tx PA01 H ioset 2, rx PD12 H ioset 3, tx PD13 H ioset 3;
}

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

// The EIC lines, from the ATDF's pin multiplexing (function A). PD00 has
// only the NMI.
#[cfg(feature = "exti")]
crate::exti::impl_exti_pins!(
    PA00 = EXTINT0,
    PA01 = EXTINT1,
    PA02 = EXTINT2,
    PA03 = EXTINT3,
    PA04 = EXTINT4,
    PA05 = EXTINT5,
    PA06 = EXTINT6,
    PA07 = EXTINT7,
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
    PB00 = EXTINT0,
    PB01 = EXTINT1,
    PB02 = EXTINT2,
    PB03 = EXTINT3,
    PB04 = EXTINT4,
    PB05 = EXTINT5,
    PB06 = EXTINT6,
    PB07 = EXTINT8,
    PB08 = EXTINT9,
    PB09 = EXTINT10,
    PB10 = EXTINT11,
    PB11 = EXTINT12,
    PB12 = EXTINT2,
    PB13 = EXTINT14,
    PB14 = EXTINT15,
    PB15 = EXTINT0,
    PB16 = EXTINT1,
    PB17 = EXTINT2,
    PC00 = EXTINT0,
    PC01 = EXTINT1,
    PC02 = EXTINT2,
    PC03 = EXTINT3,
    PC04 = EXTINT4,
    PC05 = EXTINT5,
    PC06 = EXTINT6,
    PC09 = EXTINT9,
    PC10 = EXTINT10,
    PC11 = EXTINT11,
    PC12 = EXTINT12,
    PC13 = EXTINT13,
    PC14 = EXTINT14,
    PC15 = EXTINT15,
    PC16 = EXTINT0,
    PC17 = EXTINT1,
    PC18 = EXTINT2,
    PC19 = EXTINT3,
    PD01 = EXTINT1,
    PD04 = EXTINT2,
    PD05 = EXTINT3,
    PD07 = EXTINT4,
    PD08 = EXTINT5,
    PD09 = EXTINT6,
    PD10 = EXTINT8,
    PD11 = EXTINT9,
    PD12 = EXTINT10,
    PD13 = EXTINT11,
    PD14 = EXTINT12,
    PD15 = EXTINT13,
    PD16 = EXTINT14,
    PD17 = EXTINT15,
    PD18 = EXTINT0,
    PD19 = EXTINT1,
    PD20 = EXTINT2
);

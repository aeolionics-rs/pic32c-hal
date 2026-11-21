// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The PIC32CK2051SG01144 and GC01144 (the same die, the GC without
//! `TrustZone` and the HSM): their interrupts (from the SG PAC's `Interrupt`,
//! in vector order), the peripheral singletons the drivers take so far, and
//! the 144-pin package's 107 GPIO pins.

#[cfg(feature = "pic32ck2051sg01144")]
pub use pic32ck_sg01_pac as pac;
#[cfg(feature = "pic32ck2051gc01144")]
pub use pic32ck_gc01_pac as pac;

/// Program flash: two panels of 1 MiB.
pub const FLASH_SIZE: usize = 2 << 20;

/// PORT as Secure code reaches it: the Secure view on the SG, the only
/// view on the GC.
#[cfg(feature = "pic32ck2051sg01144")]
pub(crate) const PORT: usize = 0x4480_1000;
#[cfg(feature = "pic32ck2051gc01144")]
pub(crate) const PORT: usize = 0x4480_0000;

/// The EIC as Secure code reaches it, likewise.
#[cfg(all(feature = "pic32ck2051sg01144", feature = "exti"))]
pub(crate) const EIC: usize = 0x4401_B000;
#[cfg(all(feature = "pic32ck2051gc01144", feature = "exti"))]
pub(crate) const EIC: usize = 0x4401_A000;

// The vectors: the GC's are the SG's without the HSM's three.
#[cfg(feature = "pic32ck2051sg01144")]
embassy_hal_internal::interrupt_mod!(
    FCR_ERR,
    FCR_FLT,
    FCW,
    PM,
    SUPC,
    OSCCTRL_XOSC,
    OSCCTRL_DFLL,
    OSCCTRL_PLL0,
    OSC32KCTRL,
    MCLK,
    FREQM,
    WDT,
    RTC,
    EIC_EXTINT_0,
    EIC_EXTINT_1,
    EIC_EXTINT_2,
    EIC_EXTINT_3,
    EIC_EXTINT_4,
    EIC_EXTINT_5,
    EIC_EXTINT_6,
    EIC_EXTINT_7,
    EIC_EXTINT_8,
    EIC_EXTINT_9,
    EIC_EXTINT_10,
    EIC_EXTINT_11,
    EIC_EXTINT_12,
    EIC_EXTINT_13,
    EIC_EXTINT_14,
    EIC_EXTINT_15,
    EIC_NSCHK,
    PAC,
    TRAM,
    PORT,
    DMA0_PRI0,
    DMA0_PRI1,
    DMA0_PRI2,
    DMA1_PRI0,
    DMA1_PRI1,
    HMATRIX,
    EVSYS_0,
    EVSYS_1,
    EVSYS_2,
    EVSYS_3,
    EVSYS_OTHER,
    SERCOM0_56,
    SERCOM0_0,
    SERCOM0_1,
    SERCOM0_2,
    SERCOM0_3,
    SERCOM0_4,
    SERCOM1_56,
    SERCOM1_0,
    SERCOM1_1,
    SERCOM1_2,
    SERCOM1_3,
    SERCOM1_4,
    SERCOM2_56,
    SERCOM2_0,
    SERCOM2_1,
    SERCOM2_2,
    SERCOM2_3,
    SERCOM2_4,
    SERCOM3_56,
    SERCOM3_0,
    SERCOM3_1,
    SERCOM3_2,
    SERCOM3_3,
    SERCOM3_4,
    TCC0_OTHER,
    TCC0_MC0,
    TCC0_MC1,
    TCC0_MC2,
    TCC0_MC3,
    TCC0_MC4,
    TCC0_MC5,
    TCC1_OTHER,
    TCC1_MC0,
    TCC1_MC1,
    TCC1_MC2,
    TCC1_MC3,
    TCC1_MC4,
    TCC1_MC5,
    TCC2_OTHER,
    TCC2_MC0,
    TCC2_MC1,
    TCC2_MC2,
    TCC2_MC3,
    TCC2_MC4,
    TCC2_MC5,
    TCC3_OTHER,
    TCC3_MC0,
    TCC3_MC1,
    TCC3_MC2,
    TCC3_MC3,
    TCC3_MC4,
    TCC3_MC5,
    SERCOM4_56,
    SERCOM4_0,
    SERCOM4_1,
    SERCOM4_2,
    SERCOM4_3,
    SERCOM4_4,
    SERCOM5_56,
    SERCOM5_0,
    SERCOM5_1,
    SERCOM5_2,
    SERCOM5_3,
    SERCOM5_4,
    SERCOM6_56,
    SERCOM6_0,
    SERCOM6_1,
    SERCOM6_2,
    SERCOM6_3,
    SERCOM6_4,
    SERCOM7_56,
    SERCOM7_0,
    SERCOM7_1,
    SERCOM7_2,
    SERCOM7_3,
    SERCOM7_4,
    TCC4_OTHER,
    TCC4_MC0,
    TCC4_MC1,
    TCC5_OTHER,
    TCC5_MC0,
    TCC5_MC1,
    TCC6_OTHER,
    TCC6_MC0,
    TCC6_MC1,
    TCC7_OTHER,
    TCC7_MC0,
    TCC7_MC1,
    ADC_GLOBAL,
    ADC_CORE1,
    AC,
    SPI_IXS,
    PCC,
    PDEC,
    CAN0,
    CAN1,
    ETH,
    SQI,
    TRNG,
    SDMMC0,
    SDMMC1,
    USB_OTHER,
    USB_SOF,
    USB_TRCPT0,
    USB_TRCPT1,
    USBHS,
    HSM_ERROR,
    HSM_TXINT,
    HSM_RXINT,
);
#[cfg(feature = "pic32ck2051gc01144")]
embassy_hal_internal::interrupt_mod!(
    FCR_ERR,
    FCR_FLT,
    FCW,
    PM,
    SUPC,
    OSCCTRL_XOSC,
    OSCCTRL_DFLL,
    OSCCTRL_PLL0,
    OSC32KCTRL,
    MCLK,
    FREQM,
    WDT,
    RTC,
    EIC_EXTINT_0,
    EIC_EXTINT_1,
    EIC_EXTINT_2,
    EIC_EXTINT_3,
    EIC_EXTINT_4,
    EIC_EXTINT_5,
    EIC_EXTINT_6,
    EIC_EXTINT_7,
    EIC_EXTINT_8,
    EIC_EXTINT_9,
    EIC_EXTINT_10,
    EIC_EXTINT_11,
    EIC_EXTINT_12,
    EIC_EXTINT_13,
    EIC_EXTINT_14,
    EIC_EXTINT_15,
    EIC_NSCHK,
    PAC,
    TRAM,
    PORT,
    DMA0_PRI0,
    DMA0_PRI1,
    DMA0_PRI2,
    DMA1_PRI0,
    DMA1_PRI1,
    HMATRIX,
    EVSYS_0,
    EVSYS_1,
    EVSYS_2,
    EVSYS_3,
    EVSYS_OTHER,
    SERCOM0_56,
    SERCOM0_0,
    SERCOM0_1,
    SERCOM0_2,
    SERCOM0_3,
    SERCOM0_4,
    SERCOM1_56,
    SERCOM1_0,
    SERCOM1_1,
    SERCOM1_2,
    SERCOM1_3,
    SERCOM1_4,
    SERCOM2_56,
    SERCOM2_0,
    SERCOM2_1,
    SERCOM2_2,
    SERCOM2_3,
    SERCOM2_4,
    SERCOM3_56,
    SERCOM3_0,
    SERCOM3_1,
    SERCOM3_2,
    SERCOM3_3,
    SERCOM3_4,
    TCC0_OTHER,
    TCC0_MC0,
    TCC0_MC1,
    TCC0_MC2,
    TCC0_MC3,
    TCC0_MC4,
    TCC0_MC5,
    TCC1_OTHER,
    TCC1_MC0,
    TCC1_MC1,
    TCC1_MC2,
    TCC1_MC3,
    TCC1_MC4,
    TCC1_MC5,
    TCC2_OTHER,
    TCC2_MC0,
    TCC2_MC1,
    TCC2_MC2,
    TCC2_MC3,
    TCC2_MC4,
    TCC2_MC5,
    TCC3_OTHER,
    TCC3_MC0,
    TCC3_MC1,
    TCC3_MC2,
    TCC3_MC3,
    TCC3_MC4,
    TCC3_MC5,
    SERCOM4_56,
    SERCOM4_0,
    SERCOM4_1,
    SERCOM4_2,
    SERCOM4_3,
    SERCOM4_4,
    SERCOM5_56,
    SERCOM5_0,
    SERCOM5_1,
    SERCOM5_2,
    SERCOM5_3,
    SERCOM5_4,
    SERCOM6_56,
    SERCOM6_0,
    SERCOM6_1,
    SERCOM6_2,
    SERCOM6_3,
    SERCOM6_4,
    SERCOM7_56,
    SERCOM7_0,
    SERCOM7_1,
    SERCOM7_2,
    SERCOM7_3,
    SERCOM7_4,
    TCC4_OTHER,
    TCC4_MC0,
    TCC4_MC1,
    TCC5_OTHER,
    TCC5_MC0,
    TCC5_MC1,
    TCC6_OTHER,
    TCC6_MC0,
    TCC6_MC1,
    TCC7_OTHER,
    TCC7_MC0,
    TCC7_MC1,
    ADC_GLOBAL,
    ADC_CORE1,
    AC,
    SPI_IXS,
    PCC,
    PDEC,
    CAN0,
    CAN1,
    ETH,
    SQI,
    TRNG,
    SDMMC0,
    SDMMC1,
    USB_OTHER,
    USB_SOF,
    USB_TRCPT0,
    USB_TRCPT1,
    USBHS,
);

embassy_hal_internal::peripherals!(
    CAN0,
    CAN1,
    DMA0_CH0,
    DMA0_CH1,
    DMA0_CH2,
    DMA0_CH3,
    DMA0_CH4,
    DMA0_CH5,
    DMA0_CH6,
    DMA0_CH7,
    DMA1_CH0,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    ETH,
    FCW,
    SERCOM0,
    SERCOM1,
    SERCOM2,
    SERCOM3,
    SERCOM4,
    SERCOM5,
    SERCOM6,
    SERCOM7,
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
    #[cfg(not(time_driver = "tcc7"))]
    TCC7,
    RTC,
    ADC,
    TRNG,
    USB,
    USBHS,
    WDT,
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
    PB18,
    PB19,
    PB20,
    PB21,
    PB22,
    PB23,
    PB24,
    PB25,
    PB26,
    PC00,
    PC01,
    PC02,
    PC03,
    PC04,
    PC05,
    PC06,
    PC07,
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
    PC20,
    PC21,
    PC22,
    PC23,
    PC24,
    PC25,
    PC26,
    PC27,
    PC28,
    PC29,
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
    PD21,
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
    PB18 = 1, 18;
    PB19 = 1, 19;
    PB20 = 1, 20;
    PB21 = 1, 21;
    PB22 = 1, 22;
    PB23 = 1, 23;
    PB24 = 1, 24;
    PB25 = 1, 25;
    PB26 = 1, 26;
    PC00 = 2, 0;
    PC01 = 2, 1;
    PC02 = 2, 2;
    PC03 = 2, 3;
    PC04 = 2, 4;
    PC05 = 2, 5;
    PC06 = 2, 6;
    PC07 = 2, 7;
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
    PC20 = 2, 20;
    PC21 = 2, 21;
    PC22 = 2, 22;
    PC23 = 2, 23;
    PC24 = 2, 24;
    PC25 = 2, 25;
    PC26 = 2, 26;
    PC27 = 2, 27;
    PC28 = 2, 28;
    PC29 = 2, 29;
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
    PD21 = 3, 21;
}

crate::uart::impl_sercom! {
    SERCOM0 = 0x4481_0000, number 0, channel 19, SERCOM0_0, SERCOM0_1, SERCOM0_2, SERCOM0_56;
    SERCOM1 = 0x4481_2000, number 1, channel 20, SERCOM1_0, SERCOM1_1, SERCOM1_2, SERCOM1_56;
    SERCOM2 = 0x4481_4000, number 2, channel 21, SERCOM2_0, SERCOM2_1, SERCOM2_2, SERCOM2_56;
    SERCOM3 = 0x4481_6000, number 3, channel 22, SERCOM3_0, SERCOM3_1, SERCOM3_2, SERCOM3_56;
    SERCOM4 = 0x4500_0000, number 4, channel 25, SERCOM4_0, SERCOM4_1, SERCOM4_2, SERCOM4_56;
    SERCOM5 = 0x4500_2000, number 5, channel 26, SERCOM5_0, SERCOM5_1, SERCOM5_2, SERCOM5_56;
    SERCOM6 = 0x4500_4000, number 6, channel 27, SERCOM6_0, SERCOM6_1, SERCOM6_2, SERCOM6_56;
    SERCOM7 = 0x4500_6000, number 7, channel 28, SERCOM7_0, SERCOM7_1, SERCOM7_2, SERCOM7_56;
}

// SERCOM pads, generated from the ATDF's pin multiplexing by
// tools/pin_tables.py (the same for the SG01 and GC01): each pin's pad,
// function and IOSET.
crate::sercom::impl_pads! {
    SERCOM0: PC10 pad 0 C ioset 1, PC11 pad 1 C ioset 1, PC12 pad 2 C ioset 1, PC13 pad 3 C ioset 1, PD17 pad 0 C ioset 2, PD18 pad 1 C ioset 2, PD19 pad 2 C ioset 2, PD20 pad 3 C ioset 2, PD06 pad 0 C ioset 3, PD07 pad 1 C ioset 3, PD05 pad 2 C ioset 3, PD12 pad 3 C ioset 3;
    SERCOM1: PD08 pad 0 C ioset 1, PD09 pad 1 C ioset 1, PD10 pad 2 C ioset 1, PD11 pad 3 C ioset 1, PD13 pad 0 C ioset 2, PD14 pad 1 C ioset 2, PD15 pad 2 C ioset 2, PD16 pad 3 C ioset 2;
    SERCOM2: PC11 pad 0 D ioset 1, PC10 pad 1 D ioset 1, PC12 pad 2 D ioset 1, PC13 pad 3 D ioset 1, PC15 pad 0 C ioset 2, PC16 pad 1 C ioset 2, PC17 pad 2 C ioset 2, PC18 pad 3 C ioset 2, PC21 pad 0 C ioset 3, PC22 pad 1 C ioset 3, PC14 pad 2 C ioset 3, PC23 pad 3 C ioset 3, PC06 pad 0 C ioset 4, PC07 pad 1 C ioset 4, PC09 pad 2 C ioset 4, PC26 pad 3 C ioset 4;
    SERCOM3: PC02 pad 0 C ioset 1, PC03 pad 1 C ioset 1, PC04 pad 2 C ioset 1, PC05 pad 3 C ioset 1, PB16 pad 0 C ioset 2, PB17 pad 1 C ioset 2, PB07 pad 2 C ioset 2, PB08 pad 3 C ioset 2, PB21 pad 0 C ioset 3, PB22 pad 1 C ioset 3, PB23 pad 2 C ioset 3, PB24 pad 3 C ioset 3, PC07 pad 0 D ioset 4, PC06 pad 1 D ioset 4, PC09 pad 2 D ioset 4, PC15 pad 3 D ioset 4;
    SERCOM4: PB09 pad 0 C ioset 1, PB10 pad 1 C ioset 1, PC00 pad 2 C ioset 1, PC01 pad 3 C ioset 1, PB04 pad 0 C ioset 2, PB05 pad 1 C ioset 2, PB13 pad 2 C ioset 2, PB06 pad 3 C ioset 2, PB12 pad 0 C ioset 3, PB18 pad 1 C ioset 3, PB19 pad 2 C ioset 3, PB20 pad 3 C ioset 3, PB10 pad 0 D ioset 4, PB09 pad 1 D ioset 4, PB07 pad 2 D ioset 4, PB08 pad 3 D ioset 4;
    SERCOM5: PA11 pad 0 C ioset 1, PA12 pad 1 C ioset 1, PB00 pad 2 C ioset 1, PB01 pad 3 C ioset 1, PA13 pad 0 C ioset 2, PA14 pad 1 C ioset 2, PA15 pad 2 C ioset 2, PA16 pad 3 C ioset 2, PA24 pad 0 C ioset 3, PA25 pad 1 C ioset 3, PA26 pad 2 C ioset 3, PA27 pad 3 C ioset 3, PA05 pad 0 D ioset 4, PA06 pad 1 D ioset 4, PA07 pad 2 D ioset 4, PA08 pad 3 D ioset 4;
    SERCOM6: PA07 pad 0 C ioset 1, PA08 pad 1 C ioset 1, PA09 pad 2 C ioset 1, PA10 pad 3 C ioset 1, PA00 pad 0 D ioset 2, PA01 pad 1 D ioset 2, PA02 pad 2 D ioset 2, PA03 pad 3 D ioset 2, PA18 pad 0 D ioset 3, PA28 pad 1 D ioset 3, PA19 pad 2 D ioset 3, PA09 pad 3 D ioset 3;
    SERCOM7: PA01 pad 0 C ioset 1, PA04 pad 1 C ioset 1, PA05 pad 2 C ioset 1, PA06 pad 3 C ioset 1, PA21 pad 0 C ioset 2, PA20 pad 1 C ioset 2, PA22 pad 2 C ioset 2, PA23 pad 3 C ioset 2, PA12 pad 0 D ioset 3, PA11 pad 1 D ioset 3, PB00 pad 2 D ioset 3, PB01 pad 3 D ioset 3, PB18 pad 0 D ioset 4, PB12 pad 1 D ioset 4, PB19 pad 2 D ioset 4, PB20 pad 3 D ioset 4;
}

#[cfg(feature = "can-hal")]
crate::can::instance::impl_can! {
    CAN0 = 0x4501_E000, channel 39, CAN0;
    CAN1 = 0x4502_0000, channel 40, CAN1;
}

// CAN pins, generated as the SERCOM pads are.
#[cfg(feature = "can-hal")]
crate::can::instance::impl_can_pins! {
    CAN0: rx PB09 H ioset 1, tx PB08 H ioset 1, rx PC03 H ioset 2, tx PC02 H ioset 2, rx PB17 H ioset 3, tx PB16 H ioset 3, rx PB22 H ioset 4, tx PB21 H ioset 4, rx PD19 H ioset 5, tx PD18 H ioset 5;
    CAN1: rx PB01 H ioset 1, tx PB00 H ioset 1, rx PC07 H ioset 2, tx PC06 H ioset 2, rx PB24 H ioset 3, tx PB23 H ioset 3, rx PD21 H ioset 4, tx PD20 H ioset 4;
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

// The EIC lines, from the ATDF's pin multiplexing (function A). PD02 has
// only the NMI.
#[cfg(feature = "exti")]
crate::exti::impl_exti_pins!(
    PA00 = EXTINT14,
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
    PA21 = EXTINT5,
    PA22 = EXTINT6,
    PA23 = EXTINT7,
    PA24 = EXTINT8,
    PA25 = EXTINT9,
    PA26 = EXTINT10,
    PA27 = EXTINT11,
    PA28 = EXTINT12,
    PB00 = EXTINT0,
    PB01 = EXTINT1,
    PB02 = EXTINT2,
    PB03 = EXTINT3,
    PB04 = EXTINT4,
    PB05 = EXTINT5,
    PB06 = EXTINT6,
    PB07 = EXTINT7,
    PB08 = EXTINT8,
    PB09 = EXTINT9,
    PB10 = EXTINT10,
    PB11 = EXTINT11,
    PB12 = EXTINT12,
    PB13 = EXTINT13,
    PB14 = EXTINT14,
    PB15 = EXTINT15,
    PB16 = EXTINT0,
    PB17 = EXTINT1,
    PB18 = EXTINT2,
    PB19 = EXTINT3,
    PB20 = EXTINT4,
    PB21 = EXTINT5,
    PB22 = EXTINT6,
    PB23 = EXTINT7,
    PB24 = EXTINT8,
    PB25 = EXTINT9,
    PB26 = EXTINT10,
    PC00 = EXTINT0,
    PC01 = EXTINT1,
    PC02 = EXTINT2,
    PC03 = EXTINT3,
    PC04 = EXTINT4,
    PC05 = EXTINT5,
    PC06 = EXTINT6,
    PC07 = EXTINT7,
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
    PC20 = EXTINT4,
    PC21 = EXTINT5,
    PC22 = EXTINT6,
    PC23 = EXTINT7,
    PC24 = EXTINT8,
    PC25 = EXTINT9,
    PC26 = EXTINT10,
    PC27 = EXTINT11,
    PC28 = EXTINT12,
    PC29 = EXTINT13,
    PD00 = EXTINT0,
    PD01 = EXTINT1,
    PD03 = EXTINT3,
    PD04 = EXTINT4,
    PD05 = EXTINT5,
    PD06 = EXTINT6,
    PD07 = EXTINT7,
    PD08 = EXTINT8,
    PD09 = EXTINT9,
    PD10 = EXTINT10,
    PD11 = EXTINT11,
    PD12 = EXTINT12,
    PD13 = EXTINT13,
    PD14 = EXTINT14,
    PD15 = EXTINT15,
    PD16 = EXTINT0,
    PD17 = EXTINT1,
    PD18 = EXTINT2,
    PD19 = EXTINT3,
    PD20 = EXTINT4,
    PD21 = EXTINT5,
);

#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: USART with external clock"]
    UsartExt = 0,
    #[doc = "1: USART with internal clock"]
    UsartInt = 1,
    #[doc = "2: SPI Client mode"]
    SpiSlave = 2,
    #[doc = "3: SPI Host mode"]
    SpiMaster = 3,
    #[doc = "4: I2C Client mode"]
    I2cSlave = 4,
    #[doc = "5: I2C Host mode"]
    I2cMaster = 5,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - Operating Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::UsartExt),
            1 => Some(Modeselect::UsartInt),
            2 => Some(Modeselect::SpiSlave),
            3 => Some(Modeselect::SpiMaster),
            4 => Some(Modeselect::I2cSlave),
            5 => Some(Modeselect::I2cMaster),
            _ => None,
        }
    }
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn is_usart_ext(&self) -> bool {
        *self == Modeselect::UsartExt
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn is_usart_int(&self) -> bool {
        *self == Modeselect::UsartInt
    }
    #[doc = "SPI Client mode"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == Modeselect::SpiSlave
    }
    #[doc = "SPI Host mode"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == Modeselect::SpiMaster
    }
    #[doc = "I2C Client mode"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == Modeselect::I2cSlave
    }
    #[doc = "I2C Host mode"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == Modeselect::I2cMaster
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn usart_ext(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::UsartExt)
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn usart_int(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::UsartInt)
    }
    #[doc = "SPI Client mode"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::SpiSlave)
    }
    #[doc = "SPI Host mode"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::SpiMaster)
    }
    #[doc = "I2C Client mode"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::I2cSlave)
    }
    #[doc = "I2C Host mode"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::I2cMaster)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run In Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run In Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBON` reader - Immediate Buffer Overflow Notification"]
pub type IbonR = crate::BitReader;
#[doc = "Field `IBON` writer - Immediate Buffer Overflow Notification"]
pub type IbonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sample Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Samprselect {
    #[doc = "0: 16x over-sampling using arithmetic baud rate generation"]
    _16xArithmetic = 0,
    #[doc = "1: 16x over-sampling using fractional baud rate generation"]
    _16xFractional = 1,
    #[doc = "2: 8x over-sampling using arithmetic baud rate generation"]
    _8xArithmetic = 2,
    #[doc = "3: 8x over-sampling using fractional baud rate generation"]
    _8xFractional = 3,
    #[doc = "4: 3x over-sampling using arithmetic baud rate generation"]
    _3xArithmetic = 4,
}
impl From<Samprselect> for u8 {
    #[inline(always)]
    fn from(variant: Samprselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Samprselect {
    type Ux = u8;
}
impl crate::IsEnum for Samprselect {}
#[doc = "Field `SAMPR` reader - Sample Rate"]
pub type SamprR = crate::FieldReader<Samprselect>;
impl SamprR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Samprselect> {
        match self.bits {
            0 => Some(Samprselect::_16xArithmetic),
            1 => Some(Samprselect::_16xFractional),
            2 => Some(Samprselect::_8xArithmetic),
            3 => Some(Samprselect::_8xFractional),
            4 => Some(Samprselect::_3xArithmetic),
            _ => None,
        }
    }
    #[doc = "16x over-sampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn is_16x_arithmetic(&self) -> bool {
        *self == Samprselect::_16xArithmetic
    }
    #[doc = "16x over-sampling using fractional baud rate generation"]
    #[inline(always)]
    pub fn is_16x_fractional(&self) -> bool {
        *self == Samprselect::_16xFractional
    }
    #[doc = "8x over-sampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn is_8x_arithmetic(&self) -> bool {
        *self == Samprselect::_8xArithmetic
    }
    #[doc = "8x over-sampling using fractional baud rate generation"]
    #[inline(always)]
    pub fn is_8x_fractional(&self) -> bool {
        *self == Samprselect::_8xFractional
    }
    #[doc = "3x over-sampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn is_3x_arithmetic(&self) -> bool {
        *self == Samprselect::_3xArithmetic
    }
}
#[doc = "Field `SAMPR` writer - Sample Rate"]
pub type SamprW<'a, REG> = crate::FieldWriter<'a, REG, 3, Samprselect>;
impl<'a, REG> SamprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16x over-sampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn _16x_arithmetic(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::_16xArithmetic)
    }
    #[doc = "16x over-sampling using fractional baud rate generation"]
    #[inline(always)]
    pub fn _16x_fractional(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::_16xFractional)
    }
    #[doc = "8x over-sampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn _8x_arithmetic(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::_8xArithmetic)
    }
    #[doc = "8x over-sampling using fractional baud rate generation"]
    #[inline(always)]
    pub fn _8x_fractional(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::_8xFractional)
    }
    #[doc = "3x over-sampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn _3x_arithmetic(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::_3xArithmetic)
    }
}
#[doc = "Transmit Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txposelect {
    #[doc = "0: PAD\\[0\\]=TxD, PAD\\[1\\]=XCK"]
    Mux0 = 0,
    #[doc = "1: PAD\\[2\\]=TxD, PAD\\[3\\]=XCK"]
    Mux1 = 1,
    #[doc = "2: PAD\\[0\\]=TxD, PAD\\[2\\]=RTS/TE, PAD\\[3\\]=CTS"]
    Mux2 = 2,
    #[doc = "3: PAD\\[0\\]=TxD, PAD\\[1\\]=XCK, PAD\\[2\\]=RTS/TE"]
    Mux3 = 3,
}
impl From<Txposelect> for u8 {
    #[inline(always)]
    fn from(variant: Txposelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txposelect {
    type Ux = u8;
}
impl crate::IsEnum for Txposelect {}
#[doc = "Field `TXPO` reader - Transmit Data Pinout"]
pub type TxpoR = crate::FieldReader<Txposelect>;
impl TxpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txposelect {
        match self.bits {
            0 => Txposelect::Mux0,
            1 => Txposelect::Mux1,
            2 => Txposelect::Mux2,
            3 => Txposelect::Mux3,
            _ => unreachable!(),
        }
    }
    #[doc = "PAD\\[0\\]=TxD, PAD\\[1\\]=XCK"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == Txposelect::Mux0
    }
    #[doc = "PAD\\[2\\]=TxD, PAD\\[3\\]=XCK"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == Txposelect::Mux1
    }
    #[doc = "PAD\\[0\\]=TxD, PAD\\[2\\]=RTS/TE, PAD\\[3\\]=CTS"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == Txposelect::Mux2
    }
    #[doc = "PAD\\[0\\]=TxD, PAD\\[1\\]=XCK, PAD\\[2\\]=RTS/TE"]
    #[inline(always)]
    pub fn is_mux3(&self) -> bool {
        *self == Txposelect::Mux3
    }
}
#[doc = "Field `TXPO` writer - Transmit Data Pinout"]
pub type TxpoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txposelect, crate::Safe>;
impl<'a, REG> TxpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PAD\\[0\\]=TxD, PAD\\[1\\]=XCK"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(Txposelect::Mux0)
    }
    #[doc = "PAD\\[2\\]=TxD, PAD\\[3\\]=XCK"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(Txposelect::Mux1)
    }
    #[doc = "PAD\\[0\\]=TxD, PAD\\[2\\]=RTS/TE, PAD\\[3\\]=CTS"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(Txposelect::Mux2)
    }
    #[doc = "PAD\\[0\\]=TxD, PAD\\[1\\]=XCK, PAD\\[2\\]=RTS/TE"]
    #[inline(always)]
    pub fn mux3(self) -> &'a mut crate::W<REG> {
        self.variant(Txposelect::Mux3)
    }
}
#[doc = "Receive Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxposelect {
    #[doc = "0: SERCOM PAD\\[0\\] is used for data reception"]
    Mux0 = 0,
    #[doc = "1: SERCOM PAD\\[1\\] is used for data reception"]
    Mux1 = 1,
    #[doc = "2: SERCOM PAD\\[2\\] is used for data reception"]
    Mux2 = 2,
    #[doc = "3: SERCOM PAD\\[3\\] is used for data reception"]
    Mux3 = 3,
}
impl From<Rxposelect> for u8 {
    #[inline(always)]
    fn from(variant: Rxposelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxposelect {
    type Ux = u8;
}
impl crate::IsEnum for Rxposelect {}
#[doc = "Field `RXPO` reader - Receive Data Pinout"]
pub type RxpoR = crate::FieldReader<Rxposelect>;
impl RxpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxposelect {
        match self.bits {
            0 => Rxposelect::Mux0,
            1 => Rxposelect::Mux1,
            2 => Rxposelect::Mux2,
            3 => Rxposelect::Mux3,
            _ => unreachable!(),
        }
    }
    #[doc = "SERCOM PAD\\[0\\] is used for data reception"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == Rxposelect::Mux0
    }
    #[doc = "SERCOM PAD\\[1\\] is used for data reception"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == Rxposelect::Mux1
    }
    #[doc = "SERCOM PAD\\[2\\] is used for data reception"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == Rxposelect::Mux2
    }
    #[doc = "SERCOM PAD\\[3\\] is used for data reception"]
    #[inline(always)]
    pub fn is_mux3(&self) -> bool {
        *self == Rxposelect::Mux3
    }
}
#[doc = "Field `RXPO` writer - Receive Data Pinout"]
pub type RxpoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxposelect, crate::Safe>;
impl<'a, REG> RxpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SERCOM PAD\\[0\\] is used for data reception"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Mux0)
    }
    #[doc = "SERCOM PAD\\[1\\] is used for data reception"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Mux1)
    }
    #[doc = "SERCOM PAD\\[2\\] is used for data reception"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Mux2)
    }
    #[doc = "SERCOM PAD\\[3\\] is used for data reception"]
    #[inline(always)]
    pub fn mux3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Mux3)
    }
}
#[doc = "Sample Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sampaselect {
    #[doc = "0: 16x Over-sampling = 7-8-9; 8x Over-sampling = 3-4-5"]
    Adj0 = 0,
    #[doc = "1: 16x Over-sampling = 9-10-11; 8x Over-sampling = 4-5-6"]
    Adj1 = 1,
    #[doc = "2: 16x Over-sampling = 11-12-13; 8x Over-sampling = 5-6-7"]
    Adj2 = 2,
    #[doc = "3: 16x Over-sampling = 13-14-15; 8x Over-sampling = 6-7-8"]
    Adj3 = 3,
}
impl From<Sampaselect> for u8 {
    #[inline(always)]
    fn from(variant: Sampaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sampaselect {
    type Ux = u8;
}
impl crate::IsEnum for Sampaselect {}
#[doc = "Field `SAMPA` reader - Sample Adjustment"]
pub type SampaR = crate::FieldReader<Sampaselect>;
impl SampaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sampaselect {
        match self.bits {
            0 => Sampaselect::Adj0,
            1 => Sampaselect::Adj1,
            2 => Sampaselect::Adj2,
            3 => Sampaselect::Adj3,
            _ => unreachable!(),
        }
    }
    #[doc = "16x Over-sampling = 7-8-9; 8x Over-sampling = 3-4-5"]
    #[inline(always)]
    pub fn is_adj0(&self) -> bool {
        *self == Sampaselect::Adj0
    }
    #[doc = "16x Over-sampling = 9-10-11; 8x Over-sampling = 4-5-6"]
    #[inline(always)]
    pub fn is_adj1(&self) -> bool {
        *self == Sampaselect::Adj1
    }
    #[doc = "16x Over-sampling = 11-12-13; 8x Over-sampling = 5-6-7"]
    #[inline(always)]
    pub fn is_adj2(&self) -> bool {
        *self == Sampaselect::Adj2
    }
    #[doc = "16x Over-sampling = 13-14-15; 8x Over-sampling = 6-7-8"]
    #[inline(always)]
    pub fn is_adj3(&self) -> bool {
        *self == Sampaselect::Adj3
    }
}
#[doc = "Field `SAMPA` writer - Sample Adjustment"]
pub type SampaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sampaselect, crate::Safe>;
impl<'a, REG> SampaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16x Over-sampling = 7-8-9; 8x Over-sampling = 3-4-5"]
    #[inline(always)]
    pub fn adj0(self) -> &'a mut crate::W<REG> {
        self.variant(Sampaselect::Adj0)
    }
    #[doc = "16x Over-sampling = 9-10-11; 8x Over-sampling = 4-5-6"]
    #[inline(always)]
    pub fn adj1(self) -> &'a mut crate::W<REG> {
        self.variant(Sampaselect::Adj1)
    }
    #[doc = "16x Over-sampling = 11-12-13; 8x Over-sampling = 5-6-7"]
    #[inline(always)]
    pub fn adj2(self) -> &'a mut crate::W<REG> {
        self.variant(Sampaselect::Adj2)
    }
    #[doc = "16x Over-sampling = 13-14-15; 8x Over-sampling = 6-7-8"]
    #[inline(always)]
    pub fn adj3(self) -> &'a mut crate::W<REG> {
        self.variant(Sampaselect::Adj3)
    }
}
#[doc = "Frame Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Formselect {
    #[doc = "0: USART frame"]
    UsartFrameNoParity = 0,
    #[doc = "1: USART frame with parity"]
    UsartFrameWithParity = 1,
    #[doc = "2: LIN Host - Break and sync generation. See LIN Command (CTRLB.LINCMD)."]
    UsartFrameLinbrkgen = 2,
    #[doc = "4: Auto-baud (LIN Client) - break detection and auto-baud"]
    UsartFrameAutoBaudNoParity = 4,
    #[doc = "5: Auto-baud - break detection and auto-baud with parity"]
    UsartFrameAutoBaudWithParity = 5,
}
impl From<Formselect> for u8 {
    #[inline(always)]
    fn from(variant: Formselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Formselect {
    type Ux = u8;
}
impl crate::IsEnum for Formselect {}
#[doc = "Field `FORM` reader - Frame Format"]
pub type FormR = crate::FieldReader<Formselect>;
impl FormR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Formselect> {
        match self.bits {
            0 => Some(Formselect::UsartFrameNoParity),
            1 => Some(Formselect::UsartFrameWithParity),
            2 => Some(Formselect::UsartFrameLinbrkgen),
            4 => Some(Formselect::UsartFrameAutoBaudNoParity),
            5 => Some(Formselect::UsartFrameAutoBaudWithParity),
            _ => None,
        }
    }
    #[doc = "USART frame"]
    #[inline(always)]
    pub fn is_usart_frame_no_parity(&self) -> bool {
        *self == Formselect::UsartFrameNoParity
    }
    #[doc = "USART frame with parity"]
    #[inline(always)]
    pub fn is_usart_frame_with_parity(&self) -> bool {
        *self == Formselect::UsartFrameWithParity
    }
    #[doc = "LIN Host - Break and sync generation. See LIN Command (CTRLB.LINCMD)."]
    #[inline(always)]
    pub fn is_usart_frame_linbrkgen(&self) -> bool {
        *self == Formselect::UsartFrameLinbrkgen
    }
    #[doc = "Auto-baud (LIN Client) - break detection and auto-baud"]
    #[inline(always)]
    pub fn is_usart_frame_auto_baud_no_parity(&self) -> bool {
        *self == Formselect::UsartFrameAutoBaudNoParity
    }
    #[doc = "Auto-baud - break detection and auto-baud with parity"]
    #[inline(always)]
    pub fn is_usart_frame_auto_baud_with_parity(&self) -> bool {
        *self == Formselect::UsartFrameAutoBaudWithParity
    }
}
#[doc = "Field `FORM` writer - Frame Format"]
pub type FormW<'a, REG> = crate::FieldWriter<'a, REG, 4, Formselect>;
impl<'a, REG> FormW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART frame"]
    #[inline(always)]
    pub fn usart_frame_no_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::UsartFrameNoParity)
    }
    #[doc = "USART frame with parity"]
    #[inline(always)]
    pub fn usart_frame_with_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::UsartFrameWithParity)
    }
    #[doc = "LIN Host - Break and sync generation. See LIN Command (CTRLB.LINCMD)."]
    #[inline(always)]
    pub fn usart_frame_linbrkgen(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::UsartFrameLinbrkgen)
    }
    #[doc = "Auto-baud (LIN Client) - break detection and auto-baud"]
    #[inline(always)]
    pub fn usart_frame_auto_baud_no_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::UsartFrameAutoBaudNoParity)
    }
    #[doc = "Auto-baud - break detection and auto-baud with parity"]
    #[inline(always)]
    pub fn usart_frame_auto_baud_with_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::UsartFrameAutoBaudWithParity)
    }
}
#[doc = "Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmodeselect {
    #[doc = "0: Asynchronous communication"]
    Async = 0,
    #[doc = "1: Synchronous communication"]
    Sync = 1,
}
impl From<Cmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Cmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMODE` reader - Communication Mode"]
pub type CmodeR = crate::BitReader<Cmodeselect>;
impl CmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmodeselect {
        match self.bits {
            false => Cmodeselect::Async,
            true => Cmodeselect::Sync,
        }
    }
    #[doc = "Asynchronous communication"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == Cmodeselect::Async
    }
    #[doc = "Synchronous communication"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Cmodeselect::Sync
    }
}
#[doc = "Field `CMODE` writer - Communication Mode"]
pub type CmodeW<'a, REG> = crate::BitWriter<'a, REG, Cmodeselect>;
impl<'a, REG> CmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous communication"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmodeselect::Async)
    }
    #[doc = "Synchronous communication"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Cmodeselect::Sync)
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpolselect {
    #[doc = "0: TxD Change: Rising XCK edge, RxD Sample: Falling XCK edge"]
    IdleLow = 0,
    #[doc = "1: TxD Change: Falling XCK edge, RxD Sample: Rising XCK edge"]
    IdleHigh = 1,
}
impl From<Cpolselect> for bool {
    #[inline(always)]
    fn from(variant: Cpolselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CpolR = crate::BitReader<Cpolselect>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpolselect {
        match self.bits {
            false => Cpolselect::IdleLow,
            true => Cpolselect::IdleHigh,
        }
    }
    #[doc = "TxD Change: Rising XCK edge, RxD Sample: Falling XCK edge"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == Cpolselect::IdleLow
    }
    #[doc = "TxD Change: Falling XCK edge, RxD Sample: Rising XCK edge"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == Cpolselect::IdleHigh
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpolselect>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TxD Change: Rising XCK edge, RxD Sample: Falling XCK edge"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(Cpolselect::IdleLow)
    }
    #[doc = "TxD Change: Falling XCK edge, RxD Sample: Rising XCK edge"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cpolselect::IdleHigh)
    }
}
#[doc = "Data Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dordselect {
    #[doc = "0: MSb is transferred first"]
    Msb = 0,
    #[doc = "1: LSb is transferred first"]
    Lsb = 1,
}
impl From<Dordselect> for bool {
    #[inline(always)]
    fn from(variant: Dordselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DORD` reader - Data Order"]
pub type DordR = crate::BitReader<Dordselect>;
impl DordR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dordselect {
        match self.bits {
            false => Dordselect::Msb,
            true => Dordselect::Lsb,
        }
    }
    #[doc = "MSb is transferred first"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == Dordselect::Msb
    }
    #[doc = "LSb is transferred first"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == Dordselect::Lsb
    }
}
#[doc = "Field `DORD` writer - Data Order"]
pub type DordW<'a, REG> = crate::BitWriter<'a, REG, Dordselect>;
impl<'a, REG> DordW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSb is transferred first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(Dordselect::Msb)
    }
    #[doc = "LSb is transferred first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(Dordselect::Lsb)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 7 - Run In Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IbonR {
        IbonR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Sample Rate"]
    #[inline(always)]
    pub fn sampr(&self) -> SamprR {
        SamprR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    pub fn txpo(&self) -> TxpoR {
        TxpoR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    pub fn rxpo(&self) -> RxpoR {
        RxpoR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    pub fn sampa(&self) -> SampaR {
        SampaR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FormR {
        FormR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    pub fn cmode(&self) -> CmodeR {
        CmodeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DordR {
        DordR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CtrlaSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 7 - Run In Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, CtrlaSpec> {
        RunstdbyW::new(self, 7)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&mut self) -> IbonW<'_, CtrlaSpec> {
        IbonW::new(self, 8)
    }
    #[doc = "Bits 13:15 - Sample Rate"]
    #[inline(always)]
    pub fn sampr(&mut self) -> SamprW<'_, CtrlaSpec> {
        SamprW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    pub fn txpo(&mut self) -> TxpoW<'_, CtrlaSpec> {
        TxpoW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    pub fn rxpo(&mut self) -> RxpoW<'_, CtrlaSpec> {
        RxpoW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    pub fn sampa(&mut self) -> SampaW<'_, CtrlaSpec> {
        SampaW::new(self, 22)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&mut self) -> FormW<'_, CtrlaSpec> {
        FormW::new(self, 24)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    pub fn cmode(&mut self) -> CmodeW<'_, CtrlaSpec> {
        CmodeW::new(self, 28)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, CtrlaSpec> {
        CpolW::new(self, 29)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&mut self) -> DordW<'_, CtrlaSpec> {
        DordW::new(self, 30)
    }
}
#[doc = "USART Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}

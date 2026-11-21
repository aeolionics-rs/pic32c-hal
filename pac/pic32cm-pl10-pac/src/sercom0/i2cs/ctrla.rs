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
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINOUT` reader - Pin Usage"]
pub type PinoutR = crate::BitReader;
#[doc = "Field `PINOUT` writer - Pin Usage"]
pub type PinoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SDA Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdaholdselect {
    #[doc = "0: Disabled"]
    Dis = 0,
    #[doc = "1: 50-100 ns hold time"]
    _75ns = 1,
    #[doc = "2: 300-600 ns hold time"]
    _450ns = 2,
    #[doc = "3: 400-800 ns hold time"]
    _600ns = 3,
}
impl From<Sdaholdselect> for u8 {
    #[inline(always)]
    fn from(variant: Sdaholdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdaholdselect {
    type Ux = u8;
}
impl crate::IsEnum for Sdaholdselect {}
#[doc = "Field `SDAHOLD` reader - SDA Hold Time"]
pub type SdaholdR = crate::FieldReader<Sdaholdselect>;
impl SdaholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdaholdselect {
        match self.bits {
            0 => Sdaholdselect::Dis,
            1 => Sdaholdselect::_75ns,
            2 => Sdaholdselect::_450ns,
            3 => Sdaholdselect::_600ns,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Sdaholdselect::Dis
    }
    #[doc = "50-100 ns hold time"]
    #[inline(always)]
    pub fn is_75ns(&self) -> bool {
        *self == Sdaholdselect::_75ns
    }
    #[doc = "300-600 ns hold time"]
    #[inline(always)]
    pub fn is_450ns(&self) -> bool {
        *self == Sdaholdselect::_450ns
    }
    #[doc = "400-800 ns hold time"]
    #[inline(always)]
    pub fn is_600ns(&self) -> bool {
        *self == Sdaholdselect::_600ns
    }
}
#[doc = "Field `SDAHOLD` writer - SDA Hold Time"]
pub type SdaholdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdaholdselect, crate::Safe>;
impl<'a, REG> SdaholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Sdaholdselect::Dis)
    }
    #[doc = "50-100 ns hold time"]
    #[inline(always)]
    pub fn _75ns(self) -> &'a mut crate::W<REG> {
        self.variant(Sdaholdselect::_75ns)
    }
    #[doc = "300-600 ns hold time"]
    #[inline(always)]
    pub fn _450ns(self) -> &'a mut crate::W<REG> {
        self.variant(Sdaholdselect::_450ns)
    }
    #[doc = "400-800 ns hold time"]
    #[inline(always)]
    pub fn _600ns(self) -> &'a mut crate::W<REG> {
        self.variant(Sdaholdselect::_600ns)
    }
}
#[doc = "Field `SEXTTOEN` reader - Client SCL Low Extend Time-Out"]
pub type SexttoenR = crate::BitReader;
#[doc = "Field `SEXTTOEN` writer - Client SCL Low Extend Time-Out"]
pub type SexttoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transfer Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Speedselect {
    #[doc = "0: Standard-mode (Sm) and Fast-mode (Fm)"]
    SmMode = 0,
    #[doc = "1: Fast-mode Plus (Fm+)"]
    FastplusMode = 1,
}
impl From<Speedselect> for u8 {
    #[inline(always)]
    fn from(variant: Speedselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Speedselect {
    type Ux = u8;
}
impl crate::IsEnum for Speedselect {}
#[doc = "Field `SPEED` reader - Transfer Speed"]
pub type SpeedR = crate::FieldReader<Speedselect>;
impl SpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Speedselect> {
        match self.bits {
            0 => Some(Speedselect::SmMode),
            1 => Some(Speedselect::FastplusMode),
            _ => None,
        }
    }
    #[doc = "Standard-mode (Sm) and Fast-mode (Fm)"]
    #[inline(always)]
    pub fn is_sm_mode(&self) -> bool {
        *self == Speedselect::SmMode
    }
    #[doc = "Fast-mode Plus (Fm+)"]
    #[inline(always)]
    pub fn is_fastplus_mode(&self) -> bool {
        *self == Speedselect::FastplusMode
    }
}
#[doc = "Field `SPEED` writer - Transfer Speed"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, Speedselect>;
impl<'a, REG> SpeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard-mode (Sm) and Fast-mode (Fm)"]
    #[inline(always)]
    pub fn sm_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::SmMode)
    }
    #[doc = "Fast-mode Plus (Fm+)"]
    #[inline(always)]
    pub fn fastplus_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::FastplusMode)
    }
}
#[doc = "Field `SCLSM` reader - SCL Clock Stretch Mode"]
pub type SclsmR = crate::BitReader;
#[doc = "Field `SCLSM` writer - SCL Clock Stretch Mode"]
pub type SclsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOWTOUTEN` reader - SCL Low Time-Out"]
pub type LowtoutenR = crate::BitReader;
#[doc = "Field `LOWTOUTEN` writer - SCL Low Time-Out"]
pub type LowtoutenW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 7 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    pub fn pinout(&self) -> PinoutR {
        PinoutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&self) -> SdaholdR {
        SdaholdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Client SCL Low Extend Time-Out"]
    #[inline(always)]
    pub fn sexttoen(&self) -> SexttoenR {
        SexttoenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    pub fn sclsm(&self) -> SclsmR {
        SclsmR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - SCL Low Time-Out"]
    #[inline(always)]
    pub fn lowtouten(&self) -> LowtoutenR {
        LowtoutenR::new(((self.bits >> 30) & 1) != 0)
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
    #[doc = "Bit 7 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, CtrlaSpec> {
        RunstdbyW::new(self, 7)
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    pub fn pinout(&mut self) -> PinoutW<'_, CtrlaSpec> {
        PinoutW::new(self, 16)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&mut self) -> SdaholdW<'_, CtrlaSpec> {
        SdaholdW::new(self, 20)
    }
    #[doc = "Bit 23 - Client SCL Low Extend Time-Out"]
    #[inline(always)]
    pub fn sexttoen(&mut self) -> SexttoenW<'_, CtrlaSpec> {
        SexttoenW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, CtrlaSpec> {
        SpeedW::new(self, 24)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    pub fn sclsm(&mut self) -> SclsmW<'_, CtrlaSpec> {
        SclsmW::new(self, 27)
    }
    #[doc = "Bit 30 - SCL Low Time-Out"]
    #[inline(always)]
    pub fn lowtouten(&mut self) -> LowtoutenW<'_, CtrlaSpec> {
        LowtoutenW::new(self, 30)
    }
}
#[doc = "I2CS Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

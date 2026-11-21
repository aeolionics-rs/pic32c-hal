#[doc = "Register `XOSC32KCTRL` reader"]
pub type R = crate::R<Xosc32kctrlSpec>;
#[doc = "Register `XOSC32KCTRL` writer"]
pub type W = crate::W<Xosc32kctrlSpec>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "External Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtalenselect {
    #[doc = "0: External Clock on the XTAL32K1 pin. XTAL32K2 is available for other functions. The CSUT timer is disregarded, and no start-up time is used."]
    Extclk = 0,
    #[doc = "1: External Crystal connected to the XTAL32K1 and XTAL32K2 pins"]
    Xtal = 1,
}
impl From<Xtalenselect> for bool {
    #[inline(always)]
    fn from(variant: Xtalenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALEN` reader - External Source Select"]
pub type XtalenR = crate::BitReader<Xtalenselect>;
impl XtalenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtalenselect {
        match self.bits {
            false => Xtalenselect::Extclk,
            true => Xtalenselect::Xtal,
        }
    }
    #[doc = "External Clock on the XTAL32K1 pin. XTAL32K2 is available for other functions. The CSUT timer is disregarded, and no start-up time is used."]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == Xtalenselect::Extclk
    }
    #[doc = "External Crystal connected to the XTAL32K1 and XTAL32K2 pins"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Xtalenselect::Xtal
    }
}
#[doc = "Field `XTALEN` writer - External Source Select"]
pub type XtalenW<'a, REG> = crate::BitWriter<'a, REG, Xtalenselect>;
impl<'a, REG> XtalenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External Clock on the XTAL32K1 pin. XTAL32K2 is available for other functions. The CSUT timer is disregarded, and no start-up time is used."]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalenselect::Extclk)
    }
    #[doc = "External Crystal connected to the XTAL32K1 and XTAL32K2 pins"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalenselect::Xtal)
    }
}
#[doc = "Field `ONDEMAND` reader - On-Demand Operation"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On-Demand Operation"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Crystal Oscillator Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csutselect {
    #[doc = "0: 1K XOSC32K cycles"]
    _1k = 0,
    #[doc = "1: 16K XOSC32K cycles"]
    _16k = 1,
    #[doc = "2: 32K XOSC32K cycles"]
    _32k = 2,
    #[doc = "3: 64K XOSC32K cycles"]
    _64k = 3,
}
impl From<Csutselect> for u8 {
    #[inline(always)]
    fn from(variant: Csutselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csutselect {
    type Ux = u8;
}
impl crate::IsEnum for Csutselect {}
#[doc = "Field `CSUT` reader - Crystal Oscillator Start-Up Time"]
pub type CsutR = crate::FieldReader<Csutselect>;
impl CsutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csutselect {
        match self.bits {
            0 => Csutselect::_1k,
            1 => Csutselect::_16k,
            2 => Csutselect::_32k,
            3 => Csutselect::_64k,
            _ => unreachable!(),
        }
    }
    #[doc = "1K XOSC32K cycles"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == Csutselect::_1k
    }
    #[doc = "16K XOSC32K cycles"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Csutselect::_16k
    }
    #[doc = "32K XOSC32K cycles"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == Csutselect::_32k
    }
    #[doc = "64K XOSC32K cycles"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == Csutselect::_64k
    }
}
#[doc = "Field `CSUT` writer - Crystal Oscillator Start-Up Time"]
pub type CsutW<'a, REG> = crate::FieldWriter<'a, REG, 2, Csutselect, crate::Safe>;
impl<'a, REG> CsutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1K XOSC32K cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut crate::W<REG> {
        self.variant(Csutselect::_1k)
    }
    #[doc = "16K XOSC32K cycles"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut crate::W<REG> {
        self.variant(Csutselect::_16k)
    }
    #[doc = "32K XOSC32K cycles"]
    #[inline(always)]
    pub fn _32k(self) -> &'a mut crate::W<REG> {
        self.variant(Csutselect::_32k)
    }
    #[doc = "64K XOSC32K cycles"]
    #[inline(always)]
    pub fn _64k(self) -> &'a mut crate::W<REG> {
        self.variant(Csutselect::_64k)
    }
}
#[doc = "Field `LPMODE` reader - Low Power Mode"]
pub type LpmodeR = crate::BitReader;
#[doc = "Field `LPMODE` writer - Low Power Mode"]
pub type LpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Source Select"]
    #[inline(always)]
    pub fn xtalen(&self) -> XtalenR {
        XtalenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - On-Demand Operation"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Crystal Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn csut(&self) -> CsutR {
        CsutR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 13 - Low Power Mode"]
    #[inline(always)]
    pub fn lpmode(&self) -> LpmodeR {
        LpmodeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Xosc32kctrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - External Source Select"]
    #[inline(always)]
    pub fn xtalen(&mut self) -> XtalenW<'_, Xosc32kctrlSpec> {
        XtalenW::new(self, 2)
    }
    #[doc = "Bit 7 - On-Demand Operation"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<'_, Xosc32kctrlSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Crystal Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn csut(&mut self) -> CsutW<'_, Xosc32kctrlSpec> {
        CsutW::new(self, 8)
    }
    #[doc = "Bit 13 - Low Power Mode"]
    #[inline(always)]
    pub fn lpmode(&mut self) -> LpmodeW<'_, Xosc32kctrlSpec> {
        LpmodeW::new(self, 13)
    }
    #[doc = "Bit 31 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WrtlockW<'_, Xosc32kctrlSpec> {
        WrtlockW::new(self, 31)
    }
}
#[doc = "XOSC32K Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32kctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc32kctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xosc32kctrlSpec;
impl crate::RegisterSpec for Xosc32kctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xosc32kctrl::R`](R) reader structure"]
impl crate::Readable for Xosc32kctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`xosc32kctrl::W`](W) writer structure"]
impl crate::Writable for Xosc32kctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XOSC32KCTRL to value 0x80"]
impl crate::Resettable for Xosc32kctrlSpec {
    const RESET_VALUE: u32 = 0x80;
}

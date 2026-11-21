#[doc = "Register `CRCMODE` reader"]
pub type R = crate::R<CrcmodeSpec>;
#[doc = "Register `CRCMODE` writer"]
pub type W = crate::W<CrcmodeSpec>;
#[doc = "Polynomial Length Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plen32select {
    #[doc = "0: Polynomial is 16-bits"]
    Sixteen = 0,
    #[doc = "1: Polynomial is 32-bits"]
    Thirtytwo = 1,
}
impl From<Plen32select> for bool {
    #[inline(always)]
    fn from(variant: Plen32select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLEN32` reader - Polynomial Length Select"]
pub type Plen32R = crate::BitReader<Plen32select>;
impl Plen32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plen32select {
        match self.bits {
            false => Plen32select::Sixteen,
            true => Plen32select::Thirtytwo,
        }
    }
    #[doc = "Polynomial is 16-bits"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == Plen32select::Sixteen
    }
    #[doc = "Polynomial is 32-bits"]
    #[inline(always)]
    pub fn is_thirtytwo(&self) -> bool {
        *self == Plen32select::Thirtytwo
    }
}
#[doc = "Field `PLEN32` writer - Polynomial Length Select"]
pub type Plen32W<'a, REG> = crate::BitWriter<'a, REG, Plen32select>;
impl<'a, REG> Plen32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polynomial is 16-bits"]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut crate::W<REG> {
        self.variant(Plen32select::Sixteen)
    }
    #[doc = "Polynomial is 32-bits"]
    #[inline(always)]
    pub fn thirtytwo(self) -> &'a mut crate::W<REG> {
        self.variant(Plen32select::Thirtytwo)
    }
}
#[doc = "CRC Auto Repeat\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autorselect {
    #[doc = "0: Perform CRC calculation once then set DONE and if needed, CRCERR."]
    Disable = 0,
    #[doc = "1: Continually Repeat CRC calculation; stop on error, set CRCDONE and CRCERR"]
    Enable = 1,
}
impl From<Autorselect> for bool {
    #[inline(always)]
    fn from(variant: Autorselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOR` reader - CRC Auto Repeat"]
pub type AutorR = crate::BitReader<Autorselect>;
impl AutorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autorselect {
        match self.bits {
            false => Autorselect::Disable,
            true => Autorselect::Enable,
        }
    }
    #[doc = "Perform CRC calculation once then set DONE and if needed, CRCERR."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Autorselect::Disable
    }
    #[doc = "Continually Repeat CRC calculation; stop on error, set CRCDONE and CRCERR"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Autorselect::Enable
    }
}
#[doc = "Field `AUTOR` writer - CRC Auto Repeat"]
pub type AutorW<'a, REG> = crate::BitWriter<'a, REG, Autorselect>;
impl<'a, REG> AutorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Perform CRC calculation once then set DONE and if needed, CRCERR."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Autorselect::Disable)
    }
    #[doc = "Continually Repeat CRC calculation; stop on error, set CRCDONE and CRCERR"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Autorselect::Enable)
    }
}
#[doc = "CRC Reflected Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Routselect {
    #[doc = "0: The CRCACC is Not Reflected"]
    Disable = 0,
    #[doc = "1: The CRCACC is Reflected (before the Final XOR)"]
    Enable = 1,
}
impl From<Routselect> for bool {
    #[inline(always)]
    fn from(variant: Routselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROUT` reader - CRC Reflected Output"]
pub type RoutR = crate::BitReader<Routselect>;
impl RoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Routselect {
        match self.bits {
            false => Routselect::Disable,
            true => Routselect::Enable,
        }
    }
    #[doc = "The CRCACC is Not Reflected"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Routselect::Disable
    }
    #[doc = "The CRCACC is Reflected (before the Final XOR)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Routselect::Enable
    }
}
#[doc = "Field `ROUT` writer - CRC Reflected Output"]
pub type RoutW<'a, REG> = crate::BitWriter<'a, REG, Routselect>;
impl<'a, REG> RoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CRCACC is Not Reflected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Routselect::Disable)
    }
    #[doc = "The CRCACC is Reflected (before the Final XOR)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Routselect::Enable)
    }
}
#[doc = "CRC Reflected Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rinselect {
    #[doc = "0: The LFSR CRC is calculated Most Significant Bit first (Not Reflected)"]
    Disable = 0,
    #[doc = "1: The LFSR CRC is calculated Least Significant Bit first (Reflected)"]
    Enable = 1,
}
impl From<Rinselect> for bool {
    #[inline(always)]
    fn from(variant: Rinselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIN` reader - CRC Reflected Input"]
pub type RinR = crate::BitReader<Rinselect>;
impl RinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rinselect {
        match self.bits {
            false => Rinselect::Disable,
            true => Rinselect::Enable,
        }
    }
    #[doc = "The LFSR CRC is calculated Most Significant Bit first (Not Reflected)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rinselect::Disable
    }
    #[doc = "The LFSR CRC is calculated Least Significant Bit first (Reflected)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rinselect::Enable
    }
}
#[doc = "Field `RIN` writer - CRC Reflected Input"]
pub type RinW<'a, REG> = crate::BitWriter<'a, REG, Rinselect>;
impl<'a, REG> RinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The LFSR CRC is calculated Most Significant Bit first (Not Reflected)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rinselect::Disable)
    }
    #[doc = "The LFSR CRC is calculated Least Significant Bit first (Reflected)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rinselect::Enable)
    }
}
#[doc = "Field `PERIOD` reader - Read Period in PerCLK counts"]
pub type PeriodR = crate::FieldReader<u16>;
#[doc = "Field `PERIOD` writer - Read Period in PerCLK counts"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 12 - Polynomial Length Select"]
    #[inline(always)]
    pub fn plen32(&self) -> Plen32R {
        Plen32R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRC Auto Repeat"]
    #[inline(always)]
    pub fn autor(&self) -> AutorR {
        AutorR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRC Reflected Output"]
    #[inline(always)]
    pub fn rout(&self) -> RoutR {
        RoutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CRC Reflected Input"]
    #[inline(always)]
    pub fn rin(&self) -> RinR {
        RinR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Read Period in PerCLK counts"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 12 - Polynomial Length Select"]
    #[inline(always)]
    pub fn plen32(&mut self) -> Plen32W<'_, CrcmodeSpec> {
        Plen32W::new(self, 12)
    }
    #[doc = "Bit 13 - CRC Auto Repeat"]
    #[inline(always)]
    pub fn autor(&mut self) -> AutorW<'_, CrcmodeSpec> {
        AutorW::new(self, 13)
    }
    #[doc = "Bit 14 - CRC Reflected Output"]
    #[inline(always)]
    pub fn rout(&mut self) -> RoutW<'_, CrcmodeSpec> {
        RoutW::new(self, 14)
    }
    #[doc = "Bit 15 - CRC Reflected Input"]
    #[inline(always)]
    pub fn rin(&mut self) -> RinW<'_, CrcmodeSpec> {
        RinW::new(self, 15)
    }
    #[doc = "Bits 16:27 - Read Period in PerCLK counts"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, CrcmodeSpec> {
        PeriodW::new(self, 16)
    }
}
#[doc = "CRC MODE REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcmodeSpec;
impl crate::RegisterSpec for CrcmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcmode::R`](R) reader structure"]
impl crate::Readable for CrcmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`crcmode::W`](W) writer structure"]
impl crate::Writable for CrcmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCMODE to value 0"]
impl crate::Resettable for CrcmodeSpec {}

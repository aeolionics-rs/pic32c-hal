#[doc = "Register `FFLTCTRL` reader"]
pub type R = crate::R<FfltctrlSpec>;
#[doc = "Register `FFLTCTRL` writer"]
pub type W = crate::W<FfltctrlSpec>;
#[doc = "Fault Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fltrstselect {
    #[doc = "0: No Action"]
    None = 0,
    #[doc = "1: Resets all FLT SFR bits."]
    Reset = 1,
}
impl From<Fltrstselect> for bool {
    #[inline(always)]
    fn from(variant: Fltrstselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLTRST` reader - Fault Reset"]
pub type FltrstR = crate::BitReader<Fltrstselect>;
impl FltrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fltrstselect {
        match self.bits {
            false => Fltrstselect::None,
            true => Fltrstselect::Reset,
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Fltrstselect::None
    }
    #[doc = "Resets all FLT SFR bits."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Fltrstselect::Reset
    }
}
#[doc = "Field `FLTRST` writer - Fault Reset"]
pub type FltrstW<'a, REG> = crate::BitWriter<'a, REG, Fltrstselect>;
impl<'a, REG> FltrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Fltrstselect::None)
    }
    #[doc = "Resets all FLT SFR bits."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Fltrstselect::Reset)
    }
}
#[doc = "ECC Fault Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fltenselect {
    #[doc = "0: ECC Fault Injection Disabled"]
    Disable = 0,
    #[doc = "1: ECC Fault Injection Enabled (module performs operation selected by FFLTMODE.FLTMD)"]
    Enable = 1,
}
impl From<Fltenselect> for bool {
    #[inline(always)]
    fn from(variant: Fltenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLTEN` reader - ECC Fault Enable bit"]
pub type FltenR = crate::BitReader<Fltenselect>;
impl FltenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fltenselect {
        match self.bits {
            false => Fltenselect::Disable,
            true => Fltenselect::Enable,
        }
    }
    #[doc = "ECC Fault Injection Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fltenselect::Disable
    }
    #[doc = "ECC Fault Injection Enabled (module performs operation selected by FFLTMODE.FLTMD)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fltenselect::Enable
    }
}
#[doc = "Field `FLTEN` writer - ECC Fault Enable bit"]
pub type FltenW<'a, REG> = crate::BitWriter<'a, REG, Fltenselect>;
impl<'a, REG> FltenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC Fault Injection Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fltenselect::Disable)
    }
    #[doc = "ECC Fault Injection Enabled (module performs operation selected by FFLTMODE.FLTMD)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fltenselect::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Fault Reset"]
    #[inline(always)]
    pub fn fltrst(&self) -> FltrstR {
        FltrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC Fault Enable bit"]
    #[inline(always)]
    pub fn flten(&self) -> FltenR {
        FltenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Reset"]
    #[inline(always)]
    pub fn fltrst(&mut self) -> FltrstW<'_, FfltctrlSpec> {
        FltrstW::new(self, 0)
    }
    #[doc = "Bit 1 - ECC Fault Enable bit"]
    #[inline(always)]
    pub fn flten(&mut self) -> FltenW<'_, FfltctrlSpec> {
        FltenW::new(self, 1)
    }
}
#[doc = "Flash ECC Fault Control REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfltctrlSpec;
impl crate::RegisterSpec for FfltctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffltctrl::R`](R) reader structure"]
impl crate::Readable for FfltctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ffltctrl::W`](W) writer structure"]
impl crate::Writable for FfltctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FFLTCTRL to value 0"]
impl crate::Resettable for FfltctrlSpec {}

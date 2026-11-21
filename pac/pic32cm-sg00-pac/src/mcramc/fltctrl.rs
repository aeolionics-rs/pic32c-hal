#[doc = "Register `FLTCTRL` reader"]
pub type R = crate::R<FltctrlSpec>;
#[doc = "Register `FLTCTRL` writer"]
pub type W = crate::W<FltctrlSpec>;
#[doc = "Fault Injection Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fltenselect {
    #[doc = "0: Disables fault injection."]
    Flten0 = 0,
    #[doc = "1: Enables fault injection at FLTADR address offset as selected by FLTMD and FLTxPTR."]
    Flten1 = 1,
}
impl From<Fltenselect> for bool {
    #[inline(always)]
    fn from(variant: Fltenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLTEN` reader - Fault Injection Enabled"]
pub type FltenR = crate::BitReader<Fltenselect>;
impl FltenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fltenselect {
        match self.bits {
            false => Fltenselect::Flten0,
            true => Fltenselect::Flten1,
        }
    }
    #[doc = "Disables fault injection."]
    #[inline(always)]
    pub fn is_flten_0(&self) -> bool {
        *self == Fltenselect::Flten0
    }
    #[doc = "Enables fault injection at FLTADR address offset as selected by FLTMD and FLTxPTR."]
    #[inline(always)]
    pub fn is_flten_1(&self) -> bool {
        *self == Fltenselect::Flten1
    }
}
#[doc = "Field `FLTEN` writer - Fault Injection Enabled"]
pub type FltenW<'a, REG> = crate::BitWriter<'a, REG, Fltenselect>;
impl<'a, REG> FltenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables fault injection."]
    #[inline(always)]
    pub fn flten_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fltenselect::Flten0)
    }
    #[doc = "Enables fault injection at FLTADR address offset as selected by FLTMD and FLTxPTR."]
    #[inline(always)]
    pub fn flten_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fltenselect::Flten1)
    }
}
#[doc = "Field `FLTMD` reader - Fault Injection Mode"]
pub type FltmdR = crate::FieldReader;
#[doc = "Field `FLTMD` writer - Fault Injection Mode"]
pub type FltmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Fault Injection Enabled"]
    #[inline(always)]
    pub fn flten(&self) -> FltenR {
        FltenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Fault Injection Mode"]
    #[inline(always)]
    pub fn fltmd(&self) -> FltmdR {
        FltmdR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Fault Injection Enabled"]
    #[inline(always)]
    pub fn flten(&mut self) -> FltenW<'_, FltctrlSpec> {
        FltenW::new(self, 1)
    }
    #[doc = "Bits 12:13 - Fault Injection Mode"]
    #[inline(always)]
    pub fn fltmd(&mut self) -> FltmdW<'_, FltctrlSpec> {
        FltmdW::new(self, 12)
    }
}
#[doc = "Fault Injection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fltctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltctrlSpec;
impl crate::RegisterSpec for FltctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltctrl::R`](R) reader structure"]
impl crate::Readable for FltctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fltctrl::W`](W) writer structure"]
impl crate::Writable for FltctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLTCTRL to value 0"]
impl crate::Resettable for FltctrlSpec {}

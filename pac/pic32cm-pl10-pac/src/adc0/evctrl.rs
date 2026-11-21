#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `STARTEI` reader - Conversion Start Event Input Enable"]
pub type StarteiR = crate::BitReader;
#[doc = "Field `STARTEI` writer - Conversion Start Event Input Enable"]
pub type StarteiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTINV` reader - Conversion Start Event Input Invert"]
pub type StartinvR = crate::BitReader;
#[doc = "Field `STARTINV` writer - Conversion Start Event Input Invert"]
pub type StartinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRDYEO` reader - Result Ready Event Output Enable"]
pub type ResrdyeoR = crate::BitReader;
#[doc = "Field `RESRDYEO` writer - Result Ready Event Output Enable"]
pub type ResrdyeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPRDYEO` reader - Sample Ready Event Output Enable"]
pub type SamprdyeoR = crate::BitReader;
#[doc = "Field `SAMPRDYEO` writer - Sample Ready Event Output Enable"]
pub type SamprdyeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCMPEO` reader - Window Comparator Event Output Enable"]
pub type WcmpeoR = crate::BitReader;
#[doc = "Field `WCMPEO` writer - Window Comparator Event Output Enable"]
pub type WcmpeoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Conversion Start Event Input Enable"]
    #[inline(always)]
    pub fn startei(&self) -> StarteiR {
        StarteiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Conversion Start Event Input Invert"]
    #[inline(always)]
    pub fn startinv(&self) -> StartinvR {
        StartinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Result Ready Event Output Enable"]
    #[inline(always)]
    pub fn resrdyeo(&self) -> ResrdyeoR {
        ResrdyeoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sample Ready Event Output Enable"]
    #[inline(always)]
    pub fn samprdyeo(&self) -> SamprdyeoR {
        SamprdyeoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Window Comparator Event Output Enable"]
    #[inline(always)]
    pub fn wcmpeo(&self) -> WcmpeoR {
        WcmpeoR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Conversion Start Event Input Enable"]
    #[inline(always)]
    pub fn startei(&mut self) -> StarteiW<'_, EvctrlSpec> {
        StarteiW::new(self, 0)
    }
    #[doc = "Bit 4 - Conversion Start Event Input Invert"]
    #[inline(always)]
    pub fn startinv(&mut self) -> StartinvW<'_, EvctrlSpec> {
        StartinvW::new(self, 4)
    }
    #[doc = "Bit 8 - Result Ready Event Output Enable"]
    #[inline(always)]
    pub fn resrdyeo(&mut self) -> ResrdyeoW<'_, EvctrlSpec> {
        ResrdyeoW::new(self, 8)
    }
    #[doc = "Bit 9 - Sample Ready Event Output Enable"]
    #[inline(always)]
    pub fn samprdyeo(&mut self) -> SamprdyeoW<'_, EvctrlSpec> {
        SamprdyeoW::new(self, 9)
    }
    #[doc = "Bit 10 - Window Comparator Event Output Enable"]
    #[inline(always)]
    pub fn wcmpeo(&mut self) -> WcmpeoW<'_, EvctrlSpec> {
        WcmpeoW::new(self, 10)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {}

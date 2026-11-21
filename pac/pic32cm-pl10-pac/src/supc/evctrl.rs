#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `MVIOEO` reader - MVIO Event Output Enable"]
pub type MvioeoR = crate::BitReader;
#[doc = "Field `MVIOEO` writer - MVIO Event Output Enable"]
pub type MvioeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLMEO` reader - VLM Event Output Enable"]
pub type VlmeoR = crate::BitReader;
#[doc = "Field `VLMEO` writer - VLM Event Output Enable"]
pub type VlmeoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MVIO Event Output Enable"]
    #[inline(always)]
    pub fn mvioeo(&self) -> MvioeoR {
        MvioeoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VLM Event Output Enable"]
    #[inline(always)]
    pub fn vlmeo(&self) -> VlmeoR {
        VlmeoR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MVIO Event Output Enable"]
    #[inline(always)]
    pub fn mvioeo(&mut self) -> MvioeoW<'_, EvctrlSpec> {
        MvioeoW::new(self, 0)
    }
    #[doc = "Bit 1 - VLM Event Output Enable"]
    #[inline(always)]
    pub fn vlmeo(&mut self) -> VlmeoW<'_, EvctrlSpec> {
        VlmeoW::new(self, 1)
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

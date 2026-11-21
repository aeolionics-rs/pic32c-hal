#[doc = "Register `WEXCTRL` reader"]
pub type R = crate::R<WexctrlSpec>;
#[doc = "Register `WEXCTRL` writer"]
pub type W = crate::W<WexctrlSpec>;
#[doc = "Field `OTMX` reader - Output Matrix"]
pub type OtmxR = crate::FieldReader;
#[doc = "Field `OTMX` writer - Output Matrix"]
pub type OtmxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline(always)]
    pub fn otmx(&self) -> OtmxR {
        OtmxR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline(always)]
    pub fn otmx(&mut self) -> OtmxW<'_, WexctrlSpec> {
        OtmxW::new(self, 0)
    }
}
#[doc = "Waveform Extension Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wexctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wexctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WexctrlSpec;
impl crate::RegisterSpec for WexctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wexctrl::R`](R) reader structure"]
impl crate::Readable for WexctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wexctrl::W`](W) writer structure"]
impl crate::Writable for WexctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WEXCTRL to value 0"]
impl crate::Resettable for WexctrlSpec {}

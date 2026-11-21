#[doc = "Register `anti_tamper__TAMPERFLAG` reader"]
pub type R = crate::R<AntiTamper_TamperflagSpec>;
#[doc = "Register `anti_tamper__TAMPERFLAG` writer"]
pub type W = crate::W<AntiTamper_TamperflagSpec>;
#[doc = "Field `TAMPERFLAG` reader - The 1-bit TAMPERFLAG register directly controls the TAMPER_FLAG output."]
pub type TamperflagR = crate::BitReader;
#[doc = "Field `TAMPERFLAG` writer - The 1-bit TAMPERFLAG register directly controls the TAMPER_FLAG output."]
pub type TamperflagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The 1-bit TAMPERFLAG register directly controls the TAMPER_FLAG output."]
    #[inline(always)]
    pub fn tamperflag(&self) -> TamperflagR {
        TamperflagR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The 1-bit TAMPERFLAG register directly controls the TAMPER_FLAG output."]
    #[inline(always)]
    pub fn tamperflag(&mut self) -> TamperflagW<'_, AntiTamper_TamperflagSpec> {
        TamperflagW::new(self, 0)
    }
}
#[doc = "The 1-bit TAMPERFLAG register directly controls the TAMPER_FLAG output.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__tamperflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__tamperflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_TamperflagSpec;
impl crate::RegisterSpec for AntiTamper_TamperflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__tamperflag::R`](R) reader structure"]
impl crate::Readable for AntiTamper_TamperflagSpec {}
#[doc = "`write(|w| ..)` method takes [`anti_tamper__tamperflag::W`](W) writer structure"]
impl crate::Writable for AntiTamper_TamperflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets anti_tamper__TAMPERFLAG to value 0"]
impl crate::Resettable for AntiTamper_TamperflagSpec {}

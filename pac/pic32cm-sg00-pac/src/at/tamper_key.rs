#[doc = "Register `TAMPER_KEY[%s]` reader"]
pub type R = crate::R<TamperKeySpec>;
#[doc = "Register `TAMPER_KEY[%s]` writer"]
pub type W = crate::W<TamperKeySpec>;
#[doc = "Field `TAMPERKEY` reader - Tamper Key Value"]
pub type TamperkeyR = crate::FieldReader<u32>;
#[doc = "Field `TAMPERKEY` writer - Tamper Key Value"]
pub type TamperkeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tamper Key Value"]
    #[inline(always)]
    pub fn tamperkey(&self) -> TamperkeyR {
        TamperkeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Tamper Key Value"]
    #[inline(always)]
    pub fn tamperkey(&mut self) -> TamperkeyW<'_, TamperKeySpec> {
        TamperkeyW::new(self, 0)
    }
}
#[doc = "Tamper Key n Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tamper_key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamper_key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TamperKeySpec;
impl crate::RegisterSpec for TamperKeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamper_key::R`](R) reader structure"]
impl crate::Readable for TamperKeySpec {}
#[doc = "`write(|w| ..)` method takes [`tamper_key::W`](W) writer structure"]
impl crate::Writable for TamperKeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAMPER_KEY[%s] to value 0"]
impl crate::Resettable for TamperKeySpec {}

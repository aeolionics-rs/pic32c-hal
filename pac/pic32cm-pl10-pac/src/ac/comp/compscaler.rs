#[doc = "Register `COMPSCALER` reader"]
pub type R = crate::R<CompscalerSpec>;
#[doc = "Register `COMPSCALER` writer"]
pub type W = crate::W<CompscalerSpec>;
#[doc = "Field `VALUE` reader - Scaled Reference Voltage"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Scaled Reference Voltage"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Scaled Reference Voltage"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scaled Reference Voltage"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, CompscalerSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Comparator # Comparator Scaler\n\nYou can [`read`](crate::Reg::read) this register and get [`compscaler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compscaler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompscalerSpec;
impl crate::RegisterSpec for CompscalerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compscaler::R`](R) reader structure"]
impl crate::Readable for CompscalerSpec {}
#[doc = "`write(|w| ..)` method takes [`compscaler::W`](W) writer structure"]
impl crate::Writable for CompscalerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPSCALER to value 0"]
impl crate::Resettable for CompscalerSpec {}

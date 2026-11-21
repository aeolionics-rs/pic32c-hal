#[doc = "Register `CTRLE` reader"]
pub type R = crate::R<CtrleSpec>;
#[doc = "Register `CTRLE` writer"]
pub type W = crate::W<CtrleSpec>;
#[doc = "Field `SAMPLEN` reader - Sample Length"]
pub type SamplenR = crate::FieldReader;
#[doc = "Field `SAMPLEN` writer - Sample Length"]
pub type SamplenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sample Length"]
    #[inline(always)]
    pub fn samplen(&self) -> SamplenR {
        SamplenR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sample Length"]
    #[inline(always)]
    pub fn samplen(&mut self) -> SamplenW<'_, CtrleSpec> {
        SamplenW::new(self, 0)
    }
}
#[doc = "Control E\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrleSpec;
impl crate::RegisterSpec for CtrleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrle::R`](R) reader structure"]
impl crate::Readable for CtrleSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrle::W`](W) writer structure"]
impl crate::Writable for CtrleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLE to value 0"]
impl crate::Resettable for CtrleSpec {}

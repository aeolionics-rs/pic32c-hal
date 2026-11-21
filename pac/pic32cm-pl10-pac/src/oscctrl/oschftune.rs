#[doc = "Register `OSCHFTUNE` reader"]
pub type R = crate::R<OschftuneSpec>;
#[doc = "Register `OSCHFTUNE` writer"]
pub type W = crate::W<OschftuneSpec>;
#[doc = "Field `TUNE` reader - Oscillator Tune"]
pub type TuneR = crate::FieldReader;
#[doc = "Field `TUNE` writer - Oscillator Tune"]
pub type TuneW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Oscillator Tune"]
    #[inline(always)]
    pub fn tune(&self) -> TuneR {
        TuneR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Oscillator Tune"]
    #[inline(always)]
    pub fn tune(&mut self) -> TuneW<'_, OschftuneSpec> {
        TuneW::new(self, 0)
    }
}
#[doc = "OSCHF Oscillator Frequency Tune\n\nYou can [`read`](crate::Reg::read) this register and get [`oschftune::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oschftune::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OschftuneSpec;
impl crate::RegisterSpec for OschftuneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oschftune::R`](R) reader structure"]
impl crate::Readable for OschftuneSpec {}
#[doc = "`write(|w| ..)` method takes [`oschftune::W`](W) writer structure"]
impl crate::Writable for OschftuneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSCHFTUNE to value 0"]
impl crate::Resettable for OschftuneSpec {}

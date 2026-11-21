#[doc = "Register `SAMPLE` reader"]
pub type R = crate::R<SampleSpec>;
#[doc = "Register `SAMPLE` writer"]
pub type W = crate::W<SampleSpec>;
#[doc = "Field `SAMPLE` reader - ADC Sample"]
pub type SampleR = crate::FieldReader<u16>;
#[doc = "Field `SAMPLE` writer - ADC Sample"]
pub type SampleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC Sample"]
    #[inline(always)]
    pub fn sample(&self) -> SampleR {
        SampleR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC Sample"]
    #[inline(always)]
    pub fn sample(&mut self) -> SampleW<'_, SampleSpec> {
        SampleW::new(self, 0)
    }
}
#[doc = "Sample\n\nYou can [`read`](crate::Reg::read) this register and get [`sample::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SampleSpec;
impl crate::RegisterSpec for SampleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample::R`](R) reader structure"]
impl crate::Readable for SampleSpec {}
#[doc = "`write(|w| ..)` method takes [`sample::W`](W) writer structure"]
impl crate::Writable for SampleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMPLE to value 0"]
impl crate::Resettable for SampleSpec {}

#[doc = "Register `anti_tamper__FILTERCNT` reader"]
pub type R = crate::R<AntiTamper_FiltercntSpec>;
#[doc = "Field `FILTERCNT` reader - Current value of filter counter."]
pub type FiltercntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current value of filter counter."]
    #[inline(always)]
    pub fn filtercnt(&self) -> FiltercntR {
        FiltercntR::new(self.bits)
    }
}
#[doc = "Current value of filter counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__filtercnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_FiltercntSpec;
impl crate::RegisterSpec for AntiTamper_FiltercntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__filtercnt::R`](R) reader structure"]
impl crate::Readable for AntiTamper_FiltercntSpec {}
#[doc = "`reset()` method sets anti_tamper__FILTERCNT to value 0"]
impl crate::Resettable for AntiTamper_FiltercntSpec {}

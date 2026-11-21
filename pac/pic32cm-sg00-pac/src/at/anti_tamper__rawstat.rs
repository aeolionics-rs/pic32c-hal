#[doc = "Register `anti_tamper__RAWSTAT` reader"]
pub type R = crate::R<AntiTamper_RawstatSpec>;
#[doc = "Field `RAWSTAT` reader - Raw status."]
pub type RawstatR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Raw status."]
    #[inline(always)]
    pub fn rawstat(&self) -> RawstatR {
        RawstatR::new(self.bits)
    }
}
#[doc = "Raw status.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__rawstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_RawstatSpec;
impl crate::RegisterSpec for AntiTamper_RawstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__rawstat::R`](R) reader structure"]
impl crate::Readable for AntiTamper_RawstatSpec {}
#[doc = "`reset()` method sets anti_tamper__RAWSTAT to value 0"]
impl crate::Resettable for AntiTamper_RawstatSpec {}

#[doc = "Register `anti_tamper__TIMESTAMP` reader"]
pub type R = crate::R<AntiTamper_TimestampSpec>;
#[doc = "Field `TIMESTAMP` reader - Timestamp for the last tamper."]
pub type TimestampR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp for the last tamper."]
    #[inline(always)]
    pub fn timestamp(&self) -> TimestampR {
        TimestampR::new(self.bits)
    }
}
#[doc = "Timestamp for the last tamper.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__timestamp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_TimestampSpec;
impl crate::RegisterSpec for AntiTamper_TimestampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__timestamp::R`](R) reader structure"]
impl crate::Readable for AntiTamper_TimestampSpec {}
#[doc = "`reset()` method sets anti_tamper__TIMESTAMP to value 0"]
impl crate::Resettable for AntiTamper_TimestampSpec {}

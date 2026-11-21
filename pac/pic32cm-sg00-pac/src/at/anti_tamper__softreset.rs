#[doc = "Register `anti_tamper__SOFTRESET` reader"]
pub type R = crate::R<AntiTamper_SoftresetSpec>;
#[doc = "Field `SOFTRESET` reader - Reading the SOFTLVL4 register returns 1 if any active soft tamper has a level of 4 or above."]
pub type SoftresetR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reading the SOFTLVL4 register returns 1 if any active soft tamper has a level of 4 or above."]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new((self.bits & 1) != 0)
    }
}
#[doc = "Reading the SOFTLVL4 register returns 1 if any active soft tamper has a level of 4 or above.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__softreset::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_SoftresetSpec;
impl crate::RegisterSpec for AntiTamper_SoftresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__softreset::R`](R) reader structure"]
impl crate::Readable for AntiTamper_SoftresetSpec {}
#[doc = "`reset()` method sets anti_tamper__SOFTRESET to value 0"]
impl crate::Resettable for AntiTamper_SoftresetSpec {}

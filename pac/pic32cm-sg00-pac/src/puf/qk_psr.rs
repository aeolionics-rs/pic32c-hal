#[doc = "Register `QK_PSR` reader"]
pub type R = crate::R<QkPsrSpec>;
#[doc = "Field `qk_puf_score` reader - PUF score obtained during the last Test PUF, Enroll, Start or Reconstruct operation"]
pub type QkPufScoreR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - PUF score obtained during the last Test PUF, Enroll, Start or Reconstruct operation"]
    #[inline(always)]
    pub fn qk_puf_score(&self) -> QkPufScoreR {
        QkPufScoreR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "PUF Score register; The PUF Score register shows the value of the PUF Score that was obtained during the last Test PUF, Enroll, Start or Reconstruct operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_psr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkPsrSpec;
impl crate::RegisterSpec for QkPsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_psr::R`](R) reader structure"]
impl crate::Readable for QkPsrSpec {}
#[doc = "`reset()` method sets QK_PSR to value 0x0f"]
impl crate::Resettable for QkPsrSpec {
    const RESET_VALUE: u32 = 0x0f;
}

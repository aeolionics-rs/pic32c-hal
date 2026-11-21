#[doc = "Register `RESULT` reader"]
pub type R = crate::R<ResultSpec>;
#[doc = "Field `RES` reader - Result Conversion Value."]
pub type ResR = crate::FieldReader<u16>;
#[doc = "Field `ACCRES` reader - Accumulated Result Conversion Value."]
pub type AccresR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Result Conversion Value."]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - Accumulated Result Conversion Value."]
    #[inline(always)]
    pub fn accres(&self) -> AccresR {
        AccresR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultSpec;
impl crate::RegisterSpec for ResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for ResultSpec {}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for ResultSpec {}

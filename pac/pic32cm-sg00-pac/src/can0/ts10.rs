#[doc = "Register `TS10` reader"]
pub type R = crate::R<Ts10Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS10"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS10"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 10\n\nYou can [`read`](crate::Reg::read) this register and get [`ts10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts10Spec;
impl crate::RegisterSpec for Ts10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts10::R`](R) reader structure"]
impl crate::Readable for Ts10Spec {}
#[doc = "`reset()` method sets TS10 to value 0"]
impl crate::Resettable for Ts10Spec {}

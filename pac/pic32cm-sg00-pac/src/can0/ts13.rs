#[doc = "Register `TS13` reader"]
pub type R = crate::R<Ts13Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS13"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS13"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 13\n\nYou can [`read`](crate::Reg::read) this register and get [`ts13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts13Spec;
impl crate::RegisterSpec for Ts13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts13::R`](R) reader structure"]
impl crate::Readable for Ts13Spec {}
#[doc = "`reset()` method sets TS13 to value 0"]
impl crate::Resettable for Ts13Spec {}

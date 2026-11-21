#[doc = "Register `TS8` reader"]
pub type R = crate::R<Ts8Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS8"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS8"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 8\n\nYou can [`read`](crate::Reg::read) this register and get [`ts8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts8Spec;
impl crate::RegisterSpec for Ts8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts8::R`](R) reader structure"]
impl crate::Readable for Ts8Spec {}
#[doc = "`reset()` method sets TS8 to value 0"]
impl crate::Resettable for Ts8Spec {}

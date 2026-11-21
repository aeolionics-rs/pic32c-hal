#[doc = "Register `TS12` reader"]
pub type R = crate::R<Ts12Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS12"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS12"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 12\n\nYou can [`read`](crate::Reg::read) this register and get [`ts12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts12Spec;
impl crate::RegisterSpec for Ts12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts12::R`](R) reader structure"]
impl crate::Readable for Ts12Spec {}
#[doc = "`reset()` method sets TS12 to value 0"]
impl crate::Resettable for Ts12Spec {}

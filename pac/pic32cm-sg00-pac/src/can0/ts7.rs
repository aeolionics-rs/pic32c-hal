#[doc = "Register `TS7` reader"]
pub type R = crate::R<Ts7Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS7"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS7"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 7\n\nYou can [`read`](crate::Reg::read) this register and get [`ts7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts7Spec;
impl crate::RegisterSpec for Ts7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts7::R`](R) reader structure"]
impl crate::Readable for Ts7Spec {}
#[doc = "`reset()` method sets TS7 to value 0"]
impl crate::Resettable for Ts7Spec {}

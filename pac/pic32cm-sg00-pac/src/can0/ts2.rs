#[doc = "Register `TS2` reader"]
pub type R = crate::R<Ts2Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS2"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS2"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ts2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts2Spec;
impl crate::RegisterSpec for Ts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts2::R`](R) reader structure"]
impl crate::Readable for Ts2Spec {}
#[doc = "`reset()` method sets TS2 to value 0"]
impl crate::Resettable for Ts2Spec {}

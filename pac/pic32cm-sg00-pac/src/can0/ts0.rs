#[doc = "Register `TS0` reader"]
pub type R = crate::R<Ts0Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS0"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS0"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ts0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts0Spec;
impl crate::RegisterSpec for Ts0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts0::R`](R) reader structure"]
impl crate::Readable for Ts0Spec {}
#[doc = "`reset()` method sets TS0 to value 0"]
impl crate::Resettable for Ts0Spec {}

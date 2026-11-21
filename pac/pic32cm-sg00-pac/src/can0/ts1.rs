#[doc = "Register `TS1` reader"]
pub type R = crate::R<Ts1Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS1"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS1"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ts1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts1Spec;
impl crate::RegisterSpec for Ts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts1::R`](R) reader structure"]
impl crate::Readable for Ts1Spec {}
#[doc = "`reset()` method sets TS1 to value 0"]
impl crate::Resettable for Ts1Spec {}

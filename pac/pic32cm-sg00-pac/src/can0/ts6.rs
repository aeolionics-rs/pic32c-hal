#[doc = "Register `TS6` reader"]
pub type R = crate::R<Ts6Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS6"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS6"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 6\n\nYou can [`read`](crate::Reg::read) this register and get [`ts6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts6Spec;
impl crate::RegisterSpec for Ts6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts6::R`](R) reader structure"]
impl crate::Readable for Ts6Spec {}
#[doc = "`reset()` method sets TS6 to value 0"]
impl crate::Resettable for Ts6Spec {}

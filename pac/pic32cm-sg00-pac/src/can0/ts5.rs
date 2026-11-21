#[doc = "Register `TS5` reader"]
pub type R = crate::R<Ts5Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS5"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS5"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 5\n\nYou can [`read`](crate::Reg::read) this register and get [`ts5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts5Spec;
impl crate::RegisterSpec for Ts5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts5::R`](R) reader structure"]
impl crate::Readable for Ts5Spec {}
#[doc = "`reset()` method sets TS5 to value 0"]
impl crate::Resettable for Ts5Spec {}

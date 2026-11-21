#[doc = "Register `TS9` reader"]
pub type R = crate::R<Ts9Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS9"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS9"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 9\n\nYou can [`read`](crate::Reg::read) this register and get [`ts9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts9Spec;
impl crate::RegisterSpec for Ts9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts9::R`](R) reader structure"]
impl crate::Readable for Ts9Spec {}
#[doc = "`reset()` method sets TS9 to value 0"]
impl crate::Resettable for Ts9Spec {}

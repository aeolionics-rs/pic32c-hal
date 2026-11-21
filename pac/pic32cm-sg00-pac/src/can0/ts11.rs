#[doc = "Register `TS11` reader"]
pub type R = crate::R<Ts11Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS11"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS11"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 11\n\nYou can [`read`](crate::Reg::read) this register and get [`ts11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts11Spec;
impl crate::RegisterSpec for Ts11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts11::R`](R) reader structure"]
impl crate::Readable for Ts11Spec {}
#[doc = "`reset()` method sets TS11 to value 0"]
impl crate::Resettable for Ts11Spec {}

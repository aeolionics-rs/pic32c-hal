#[doc = "Register `TS3` reader"]
pub type R = crate::R<Ts3Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS0"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS0"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ts3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts3Spec;
impl crate::RegisterSpec for Ts3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts3::R`](R) reader structure"]
impl crate::Readable for Ts3Spec {}
#[doc = "`reset()` method sets TS3 to value 0"]
impl crate::Resettable for Ts3Spec {}

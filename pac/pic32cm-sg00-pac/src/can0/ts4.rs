#[doc = "Register `TS4` reader"]
pub type R = crate::R<Ts4Spec>;
#[doc = "Field `TS` reader - Timestamp Word TS4"]
pub type TsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS4"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
#[doc = "Timestamp 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ts4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts4Spec;
impl crate::RegisterSpec for Ts4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts4::R`](R) reader structure"]
impl crate::Readable for Ts4Spec {}
#[doc = "`reset()` method sets TS4 to value 0"]
impl crate::Resettable for Ts4Spec {}

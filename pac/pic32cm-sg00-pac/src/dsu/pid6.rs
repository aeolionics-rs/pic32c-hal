#[doc = "Register `PID6` reader"]
pub type R = crate::R<Pid6Spec>;
#[doc = "Field `RES0` reader - CoreSight Reserved Field"]
pub type Res0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Reserved Field"]
    #[inline(always)]
    pub fn res0(&self) -> Res0R {
        Res0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CoreSight Peripheral Identification 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid6Spec;
impl crate::RegisterSpec for Pid6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid6::R`](R) reader structure"]
impl crate::Readable for Pid6Spec {}
#[doc = "`reset()` method sets PID6 to value 0"]
impl crate::Resettable for Pid6Spec {}

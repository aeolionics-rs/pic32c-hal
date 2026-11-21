#[doc = "Register `PID5` reader"]
pub type R = crate::R<Pid5Spec>;
#[doc = "Field `RES0` reader - CoreSight Reserved Field"]
pub type Res0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Reserved Field"]
    #[inline(always)]
    pub fn res0(&self) -> Res0R {
        Res0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CoreSight Peripheral Identification 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid5Spec;
impl crate::RegisterSpec for Pid5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid5::R`](R) reader structure"]
impl crate::Readable for Pid5Spec {}
#[doc = "`reset()` method sets PID5 to value 0"]
impl crate::Resettable for Pid5Spec {}

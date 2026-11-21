#[doc = "Register `PID4` reader"]
pub type R = crate::R<Pid4Spec>;
#[doc = "Field `DES2` reader - CoreSight JEP106 Continuation Code"]
pub type Des2R = crate::FieldReader;
#[doc = "Field `SIZE` reader - CoreSight Size"]
pub type SizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - CoreSight JEP106 Continuation Code"]
    #[inline(always)]
    pub fn des2(&self) -> Des2R {
        Des2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CoreSight Size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "CoreSight Peripheral Identification 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid4Spec;
impl crate::RegisterSpec for Pid4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid4::R`](R) reader structure"]
impl crate::Readable for Pid4Spec {}
#[doc = "`reset()` method sets PID4 to value 0"]
impl crate::Resettable for Pid4Spec {}

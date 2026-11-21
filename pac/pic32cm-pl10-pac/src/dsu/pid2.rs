#[doc = "Register `PID2` reader"]
pub type R = crate::R<Pid2Spec>;
#[doc = "Field `DES1` reader - CoreSight JEP106 Identification Code Bits"]
pub type Des1R = crate::FieldReader;
#[doc = "Field `JEDEC` reader - CoreSight JEDEC Assignment"]
pub type JedecR = crate::BitReader;
#[doc = "Field `REVISION` reader - CoreSight Revision Number"]
pub type RevisionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - CoreSight JEP106 Identification Code Bits"]
    #[inline(always)]
    pub fn des1(&self) -> Des1R {
        Des1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - CoreSight JEDEC Assignment"]
    #[inline(always)]
    pub fn jedec(&self) -> JedecR {
        JedecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - CoreSight Revision Number"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "CoreSight Peripheral Identification 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid2Spec;
impl crate::RegisterSpec for Pid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid2::R`](R) reader structure"]
impl crate::Readable for Pid2Spec {}
#[doc = "`reset()` method sets PID2 to value 0"]
impl crate::Resettable for Pid2Spec {}

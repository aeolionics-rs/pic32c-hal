#[doc = "Register `CID0` reader"]
pub type R = crate::R<Cid0Spec>;
#[doc = "Field `PRMBL0` reader - CoreSight Preamble 0"]
pub type Prmbl0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble 0"]
    #[inline(always)]
    pub fn prmbl0(&self) -> Prmbl0R {
        Prmbl0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CoreSight Component Identification 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid0Spec;
impl crate::RegisterSpec for Cid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid0::R`](R) reader structure"]
impl crate::Readable for Cid0Spec {}
#[doc = "`reset()` method sets CID0 to value 0x0d"]
impl crate::Resettable for Cid0Spec {
    const RESET_VALUE: u32 = 0x0d;
}

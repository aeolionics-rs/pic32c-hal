#[doc = "Register `CID3` reader"]
pub type R = crate::R<Cid3Spec>;
#[doc = "Field `PRMBL3` reader - CoreSight Preamble 3"]
pub type Prmbl3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble 3"]
    #[inline(always)]
    pub fn prmbl3(&self) -> Prmbl3R {
        Prmbl3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CoreSight Component Identification 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid3Spec;
impl crate::RegisterSpec for Cid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid3::R`](R) reader structure"]
impl crate::Readable for Cid3Spec {}
#[doc = "`reset()` method sets CID3 to value 0xb1"]
impl crate::Resettable for Cid3Spec {
    const RESET_VALUE: u32 = 0xb1;
}

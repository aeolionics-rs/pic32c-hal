#[doc = "Register `CID1` reader"]
pub type R = crate::R<Cid1Spec>;
#[doc = "Field `PRMBL1` reader - CoreSight Preamble 1"]
pub type Prmbl1R = crate::FieldReader;
#[doc = "Field `CCLASS` reader - CoreSight Component Class"]
pub type CclassR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - CoreSight Preamble 1"]
    #[inline(always)]
    pub fn prmbl1(&self) -> Prmbl1R {
        Prmbl1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CoreSight Component Class"]
    #[inline(always)]
    pub fn cclass(&self) -> CclassR {
        CclassR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "CoreSight Component Identification 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid1Spec;
impl crate::RegisterSpec for Cid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid1::R`](R) reader structure"]
impl crate::Readable for Cid1Spec {}
#[doc = "`reset()` method sets CID1 to value 0x10"]
impl crate::Resettable for Cid1Spec {
    const RESET_VALUE: u32 = 0x10;
}

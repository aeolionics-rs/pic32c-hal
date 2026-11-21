#[doc = "Register `ENTRY1` reader"]
pub type R = crate::R<Entry1Spec>;
#[doc = "Field `EPRES` reader - CoreSight Entry Present"]
pub type EpresR = crate::BitReader;
#[doc = "Field `FMT` reader - CoreSight Rom Table Format"]
pub type FmtR = crate::BitReader;
#[doc = "Field `ADDOFF` reader - CoreSight ROM Table Address Offset"]
pub type AddoffR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - CoreSight Entry Present"]
    #[inline(always)]
    pub fn epres(&self) -> EpresR {
        EpresR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CoreSight Rom Table Format"]
    #[inline(always)]
    pub fn fmt(&self) -> FmtR {
        FmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 12:31 - CoreSight ROM Table Address Offset"]
    #[inline(always)]
    pub fn addoff(&self) -> AddoffR {
        AddoffR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[doc = "CoreSight ROM Table Entry 1\n\nYou can [`read`](crate::Reg::read) this register and get [`entry1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Entry1Spec;
impl crate::RegisterSpec for Entry1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry1::R`](R) reader structure"]
impl crate::Readable for Entry1Spec {}
#[doc = "`reset()` method sets ENTRY1 to value 0"]
impl crate::Resettable for Entry1Spec {}

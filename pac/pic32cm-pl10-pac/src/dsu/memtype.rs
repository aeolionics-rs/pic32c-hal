#[doc = "Register `MEMTYPE` reader"]
pub type R = crate::R<MemtypeSpec>;
#[doc = "Field `SYSMEM` reader - CoreSight System Memory Present"]
pub type SysmemR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CoreSight System Memory Present"]
    #[inline(always)]
    pub fn sysmem(&self) -> SysmemR {
        SysmemR::new((self.bits & 1) != 0)
    }
}
#[doc = "CoreSight ROM Table Memory Type\n\nYou can [`read`](crate::Reg::read) this register and get [`memtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemtypeSpec;
impl crate::RegisterSpec for MemtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memtype::R`](R) reader structure"]
impl crate::Readable for MemtypeSpec {}
#[doc = "`reset()` method sets MEMTYPE to value 0"]
impl crate::Resettable for MemtypeSpec {}

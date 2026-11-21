#[doc = "Register `MEMTYPE` reader"]
pub type R = crate::R<MemtypeSpec>;
#[doc = "CoreSight System Memory Present\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysmemselect {
    #[doc = "0: System memory not present on bus. This is a dedicated debug bus."]
    Sysmem0 = 0,
    #[doc = "1: System memory is also present on this bus"]
    Sysmem1 = 1,
}
impl From<Sysmemselect> for bool {
    #[inline(always)]
    fn from(variant: Sysmemselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSMEM` reader - CoreSight System Memory Present"]
pub type SysmemR = crate::BitReader<Sysmemselect>;
impl SysmemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysmemselect {
        match self.bits {
            false => Sysmemselect::Sysmem0,
            true => Sysmemselect::Sysmem1,
        }
    }
    #[doc = "System memory not present on bus. This is a dedicated debug bus."]
    #[inline(always)]
    pub fn is_sysmem_0(&self) -> bool {
        *self == Sysmemselect::Sysmem0
    }
    #[doc = "System memory is also present on this bus"]
    #[inline(always)]
    pub fn is_sysmem_1(&self) -> bool {
        *self == Sysmemselect::Sysmem1
    }
}
impl R {
    #[doc = "Bit 0 - CoreSight System Memory Present"]
    #[inline(always)]
    pub fn sysmem(&self) -> SysmemR {
        SysmemR::new((self.bits & 1) != 0)
    }
}
#[doc = "CoreSight ROM Table Memory Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`memtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemtypeSpec;
impl crate::RegisterSpec for MemtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memtype::R`](R) reader structure"]
impl crate::Readable for MemtypeSpec {}
#[doc = "`reset()` method sets MEMTYPE to value 0"]
impl crate::Resettable for MemtypeSpec {}

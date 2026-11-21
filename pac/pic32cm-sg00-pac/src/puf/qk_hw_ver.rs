#[doc = "Register `QK_HW_VER` reader"]
pub type R = crate::R<QkHwVerSpec>;
#[doc = "Field `qk_hw_rev` reader - Module version, patch part"]
pub type QkHwRevR = crate::FieldReader;
#[doc = "Field `qk_hw_version_minor` reader - Module version, minor part"]
pub type QkHwVersionMinorR = crate::FieldReader;
#[doc = "Field `qk_hw_version_major` reader - Module version, major part"]
pub type QkHwVersionMajorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Module version, patch part"]
    #[inline(always)]
    pub fn qk_hw_rev(&self) -> QkHwRevR {
        QkHwRevR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module version, minor part"]
    #[inline(always)]
    pub fn qk_hw_version_minor(&self) -> QkHwVersionMinorR {
        QkHwVersionMinorR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Module version, major part"]
    #[inline(always)]
    pub fn qk_hw_version_major(&self) -> QkHwVersionMajorR {
        QkHwVersionMajorR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Module Version register; The Module Version register provides the version of this module: major.minor.patch.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_ver::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkHwVerSpec;
impl crate::RegisterSpec for QkHwVerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_hw_ver::R`](R) reader structure"]
impl crate::Readable for QkHwVerSpec {}
#[doc = "`reset()` method sets QK_HW_VER to value 0x0003_0701"]
impl crate::Resettable for QkHwVerSpec {
    const RESET_VALUE: u32 = 0x0003_0701;
}

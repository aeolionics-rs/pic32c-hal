#[doc = "Register `QK_HW_SETTINGS` reader"]
pub type R = crate::R<QkHwSettingsSpec>;
#[doc = "Field `qk_disable_enroll` reader - 0: Enroll operations are enabled 1: Enroll operations are disabled"]
pub type QkDisableEnrollR = crate::BitReader;
#[doc = "Field `qk_disable_start` reader - 0: Start operations are enabled 1: Start operations are disabled"]
pub type QkDisableStartR = crate::BitReader;
#[doc = "Field `qk_disable_reconstruct` reader - 0: Reconstruct operations are enabled 1: Reconstruct operations are disabled"]
pub type QkDisableReconstructR = crate::BitReader;
#[doc = "Field `qk_disable_stop` reader - 0: Stop operations are enabled 1: Stop operations are disabled"]
pub type QkDisableStopR = crate::BitReader;
#[doc = "Field `qk_disable_get_key` reader - 0: Get Key operations are enabled 1: Get Key operations are disabled"]
pub type QkDisableGetKeyR = crate::BitReader;
#[doc = "Field `qk_disable_unwrap` reader - 0: Unwrap operations are enabled 1: Unwrap operations are disabled"]
pub type QkDisableUnwrapR = crate::BitReader;
#[doc = "Field `qk_disable_wrap_generated_random` reader - 0: Wrap Generated Random operations are enabled 1: Wrap Generated Random operations are disabled"]
pub type QkDisableWrapGeneratedRandomR = crate::BitReader;
#[doc = "Field `qk_disable_wrap` reader - 0: Wrap operations are enabled 1: Wrap operations are disabled"]
pub type QkDisableWrapR = crate::BitReader;
#[doc = "Field `qk_disable_generate_random` reader - 0: Generate Random operations are enabled 1: Generate Random operations are disabled"]
pub type QkDisableGenerateRandomR = crate::BitReader;
#[doc = "Field `qk_skip_memory_test` reader - 0: Includes memory test during initialization 1: Skips memory test during initialization"]
pub type QkSkipMemoryTestR = crate::BitReader;
#[doc = "Field `qk_disable_test_memory` reader - 0: Test Memory operations are enabled 1: Test Memory operations are disabled"]
pub type QkDisableTestMemoryR = crate::BitReader;
#[doc = "Field `qk_disable_test_puf` reader - 0: Test PUF operations are enabled 1: Test PUF operations are disabled"]
pub type QkDisableTestPufR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - 0: Enroll operations are enabled 1: Enroll operations are disabled"]
    #[inline(always)]
    pub fn qk_disable_enroll(&self) -> QkDisableEnrollR {
        QkDisableEnrollR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Start operations are enabled 1: Start operations are disabled"]
    #[inline(always)]
    pub fn qk_disable_start(&self) -> QkDisableStartR {
        QkDisableStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Reconstruct operations are enabled 1: Reconstruct operations are disabled"]
    #[inline(always)]
    pub fn qk_disable_reconstruct(&self) -> QkDisableReconstructR {
        QkDisableReconstructR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: Stop operations are enabled 1: Stop operations are disabled"]
    #[inline(always)]
    pub fn qk_disable_stop(&self) -> QkDisableStopR {
        QkDisableStopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: Get Key operations are enabled 1: Get Key operations are disabled"]
    #[inline(always)]
    pub fn qk_disable_get_key(&self) -> QkDisableGetKeyR {
        QkDisableGetKeyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: Unwrap operations are enabled 1: Unwrap operations are disabled"]
    #[inline(always)]
    pub fn qk_disable_unwrap(&self) -> QkDisableUnwrapR {
        QkDisableUnwrapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: Wrap Generated Random operations are enabled 1: Wrap Generated Random operations are disabled"]
    #[inline(always)]
    pub fn qk_disable_wrap_generated_random(&self) -> QkDisableWrapGeneratedRandomR {
        QkDisableWrapGeneratedRandomR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: Wrap operations are enabled 1: Wrap operations are disabled"]
    #[inline(always)]
    pub fn qk_disable_wrap(&self) -> QkDisableWrapR {
        QkDisableWrapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - 0: Generate Random operations are enabled 1: Generate Random operations are disabled"]
    #[inline(always)]
    pub fn qk_disable_generate_random(&self) -> QkDisableGenerateRandomR {
        QkDisableGenerateRandomR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - 0: Includes memory test during initialization 1: Skips memory test during initialization"]
    #[inline(always)]
    pub fn qk_skip_memory_test(&self) -> QkSkipMemoryTestR {
        QkSkipMemoryTestR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: Test Memory operations are enabled 1: Test Memory operations are disabled"]
    #[inline(always)]
    pub fn qk_disable_test_memory(&self) -> QkDisableTestMemoryR {
        QkDisableTestMemoryR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: Test PUF operations are enabled 1: Test PUF operations are disabled"]
    #[inline(always)]
    pub fn qk_disable_test_puf(&self) -> QkDisableTestPufR {
        QkDisableTestPufR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Module Settings register; The Module Settings register shows the values of the module settings on the signal interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_settings::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkHwSettingsSpec;
impl crate::RegisterSpec for QkHwSettingsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_hw_settings::R`](R) reader structure"]
impl crate::Readable for QkHwSettingsSpec {}
#[doc = "`reset()` method sets QK_HW_SETTINGS to value 0"]
impl crate::Resettable for QkHwSettingsSpec {}

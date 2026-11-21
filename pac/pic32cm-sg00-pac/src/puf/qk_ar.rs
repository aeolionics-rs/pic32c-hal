#[doc = "Register `QK_AR` reader"]
pub type R = crate::R<QkArSpec>;
#[doc = "Field `qk_allow_enroll` reader - 0: Enroll operation is not allowed 1: Enroll operation is allowed"]
pub type QkAllowEnrollR = crate::BitReader;
#[doc = "Field `qk_allow_start` reader - 0: Start operation is not allowed 1: Start operation is allowed"]
pub type QkAllowStartR = crate::BitReader;
#[doc = "Field `qk_allow_reconstruct` reader - 0: Reconstruct operation is not allowed 1: Reconstruct operation is allowed"]
pub type QkAllowReconstructR = crate::BitReader;
#[doc = "Field `qk_allow_stop` reader - 0: Stop operation is not allowed 1: Stop operation is allowed"]
pub type QkAllowStopR = crate::BitReader;
#[doc = "Field `qk_allow_get_key` reader - 0: Get Key operation is not allowed 1: Get Key operation is allowed"]
pub type QkAllowGetKeyR = crate::BitReader;
#[doc = "Field `qk_allow_unwrap` reader - 0: Unwrap operation is not allowed 1: Unwrap operation is allowed"]
pub type QkAllowUnwrapR = crate::BitReader;
#[doc = "Field `qk_allow_wrap_generated_random` reader - 0: Wrap Generated Random operation is not allowed 1: Wrap Generated Random operation is allowed"]
pub type QkAllowWrapGeneratedRandomR = crate::BitReader;
#[doc = "Field `qk_allow_wrap` reader - 0: Wrap operation is not allowed 1: Wrap operation is allowed"]
pub type QkAllowWrapR = crate::BitReader;
#[doc = "Field `qk_allow_generate_random` reader - 0: Generate Random operation is not allowed 1: Generate Random operation is allowed"]
pub type QkAllowGenerateRandomR = crate::BitReader;
#[doc = "Field `qk_allow_test_memory` reader - 0: Test Memory operation is not allowed 1: Test Memory operation is allowed"]
pub type QkAllowTestMemoryR = crate::BitReader;
#[doc = "Field `qk_allow_test_puf` reader - 0: Test PUF operation is not allowed 1: Test PUF operation is allowed"]
pub type QkAllowTestPufR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - 0: Enroll operation is not allowed 1: Enroll operation is allowed"]
    #[inline(always)]
    pub fn qk_allow_enroll(&self) -> QkAllowEnrollR {
        QkAllowEnrollR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Start operation is not allowed 1: Start operation is allowed"]
    #[inline(always)]
    pub fn qk_allow_start(&self) -> QkAllowStartR {
        QkAllowStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Reconstruct operation is not allowed 1: Reconstruct operation is allowed"]
    #[inline(always)]
    pub fn qk_allow_reconstruct(&self) -> QkAllowReconstructR {
        QkAllowReconstructR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: Stop operation is not allowed 1: Stop operation is allowed"]
    #[inline(always)]
    pub fn qk_allow_stop(&self) -> QkAllowStopR {
        QkAllowStopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: Get Key operation is not allowed 1: Get Key operation is allowed"]
    #[inline(always)]
    pub fn qk_allow_get_key(&self) -> QkAllowGetKeyR {
        QkAllowGetKeyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: Unwrap operation is not allowed 1: Unwrap operation is allowed"]
    #[inline(always)]
    pub fn qk_allow_unwrap(&self) -> QkAllowUnwrapR {
        QkAllowUnwrapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: Wrap Generated Random operation is not allowed 1: Wrap Generated Random operation is allowed"]
    #[inline(always)]
    pub fn qk_allow_wrap_generated_random(&self) -> QkAllowWrapGeneratedRandomR {
        QkAllowWrapGeneratedRandomR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: Wrap operation is not allowed 1: Wrap operation is allowed"]
    #[inline(always)]
    pub fn qk_allow_wrap(&self) -> QkAllowWrapR {
        QkAllowWrapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - 0: Generate Random operation is not allowed 1: Generate Random operation is allowed"]
    #[inline(always)]
    pub fn qk_allow_generate_random(&self) -> QkAllowGenerateRandomR {
        QkAllowGenerateRandomR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: Test Memory operation is not allowed 1: Test Memory operation is allowed"]
    #[inline(always)]
    pub fn qk_allow_test_memory(&self) -> QkAllowTestMemoryR {
        QkAllowTestMemoryR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: Test PUF operation is not allowed 1: Test PUF operation is allowed"]
    #[inline(always)]
    pub fn qk_allow_test_puf(&self) -> QkAllowTestPufR {
        QkAllowTestPufR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Allow register; The Allow register indicates which operations are currently allowed.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_ar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkArSpec;
impl crate::RegisterSpec for QkArSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_ar::R`](R) reader structure"]
impl crate::Readable for QkArSpec {}
#[doc = "`reset()` method sets QK_AR to value 0"]
impl crate::Resettable for QkArSpec {}

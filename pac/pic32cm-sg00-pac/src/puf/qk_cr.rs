#[doc = "Register `QK_CR` reader"]
pub type R = crate::R<QkCrSpec>;
#[doc = "Register `QK_CR` writer"]
pub type W = crate::W<QkCrSpec>;
#[doc = "Field `qk_zeroize` reader - Begin Zeroize operation"]
pub type QkZeroizeR = crate::BitReader;
#[doc = "Field `qk_zeroize` writer - Begin Zeroize operation"]
pub type QkZeroizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_enroll` reader - Begin Enroll operation"]
pub type QkEnrollR = crate::BitReader;
#[doc = "Field `qk_enroll` writer - Begin Enroll operation"]
pub type QkEnrollW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_start` reader - Begin Start operation"]
pub type QkStartR = crate::BitReader;
#[doc = "Field `qk_start` writer - Begin Start operation"]
pub type QkStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_reconstruct` reader - Begin Reconstruct operation"]
pub type QkReconstructR = crate::BitReader;
#[doc = "Field `qk_reconstruct` writer - Begin Reconstruct operation"]
pub type QkReconstructW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_stop` reader - Begin Stop operation"]
pub type QkStopR = crate::BitReader;
#[doc = "Field `qk_stop` writer - Begin Stop operation"]
pub type QkStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_get_key` reader - Begin Get Key operation"]
pub type QkGetKeyR = crate::BitReader;
#[doc = "Field `qk_get_key` writer - Begin Get Key operation"]
pub type QkGetKeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_unwrap` reader - Begin Unwrap operation"]
pub type QkUnwrapR = crate::BitReader;
#[doc = "Field `qk_unwrap` writer - Begin Unwrap operation"]
pub type QkUnwrapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_wrap_generated_random` reader - Begin Wrap Generated Random operation"]
pub type QkWrapGeneratedRandomR = crate::BitReader;
#[doc = "Field `qk_wrap_generated_random` writer - Begin Wrap Generated Random operation"]
pub type QkWrapGeneratedRandomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_wrap` reader - Begin Wrap operation"]
pub type QkWrapR = crate::BitReader;
#[doc = "Field `qk_wrap` writer - Begin Wrap operation"]
pub type QkWrapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_generate_random` reader - Begin Generate Random operation"]
pub type QkGenerateRandomR = crate::BitReader;
#[doc = "Field `qk_generate_random` writer - Begin Generate Random operation"]
pub type QkGenerateRandomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_test_memory` reader - Begin Test Memory operation"]
pub type QkTestMemoryR = crate::BitReader;
#[doc = "Field `qk_test_memory` writer - Begin Test Memory operation"]
pub type QkTestMemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_test_puf` reader - Begin Test PUF operation"]
pub type QkTestPufR = crate::BitReader;
#[doc = "Field `qk_test_puf` writer - Begin Test PUF operation"]
pub type QkTestPufW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Begin Zeroize operation"]
    #[inline(always)]
    pub fn qk_zeroize(&self) -> QkZeroizeR {
        QkZeroizeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Begin Enroll operation"]
    #[inline(always)]
    pub fn qk_enroll(&self) -> QkEnrollR {
        QkEnrollR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Begin Start operation"]
    #[inline(always)]
    pub fn qk_start(&self) -> QkStartR {
        QkStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Begin Reconstruct operation"]
    #[inline(always)]
    pub fn qk_reconstruct(&self) -> QkReconstructR {
        QkReconstructR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Begin Stop operation"]
    #[inline(always)]
    pub fn qk_stop(&self) -> QkStopR {
        QkStopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Begin Get Key operation"]
    #[inline(always)]
    pub fn qk_get_key(&self) -> QkGetKeyR {
        QkGetKeyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Begin Unwrap operation"]
    #[inline(always)]
    pub fn qk_unwrap(&self) -> QkUnwrapR {
        QkUnwrapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Begin Wrap Generated Random operation"]
    #[inline(always)]
    pub fn qk_wrap_generated_random(&self) -> QkWrapGeneratedRandomR {
        QkWrapGeneratedRandomR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Begin Wrap operation"]
    #[inline(always)]
    pub fn qk_wrap(&self) -> QkWrapR {
        QkWrapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Begin Generate Random operation"]
    #[inline(always)]
    pub fn qk_generate_random(&self) -> QkGenerateRandomR {
        QkGenerateRandomR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - Begin Test Memory operation"]
    #[inline(always)]
    pub fn qk_test_memory(&self) -> QkTestMemoryR {
        QkTestMemoryR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Begin Test PUF operation"]
    #[inline(always)]
    pub fn qk_test_puf(&self) -> QkTestPufR {
        QkTestPufR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Begin Zeroize operation"]
    #[inline(always)]
    pub fn qk_zeroize(&mut self) -> QkZeroizeW<'_, QkCrSpec> {
        QkZeroizeW::new(self, 0)
    }
    #[doc = "Bit 1 - Begin Enroll operation"]
    #[inline(always)]
    pub fn qk_enroll(&mut self) -> QkEnrollW<'_, QkCrSpec> {
        QkEnrollW::new(self, 1)
    }
    #[doc = "Bit 2 - Begin Start operation"]
    #[inline(always)]
    pub fn qk_start(&mut self) -> QkStartW<'_, QkCrSpec> {
        QkStartW::new(self, 2)
    }
    #[doc = "Bit 3 - Begin Reconstruct operation"]
    #[inline(always)]
    pub fn qk_reconstruct(&mut self) -> QkReconstructW<'_, QkCrSpec> {
        QkReconstructW::new(self, 3)
    }
    #[doc = "Bit 5 - Begin Stop operation"]
    #[inline(always)]
    pub fn qk_stop(&mut self) -> QkStopW<'_, QkCrSpec> {
        QkStopW::new(self, 5)
    }
    #[doc = "Bit 6 - Begin Get Key operation"]
    #[inline(always)]
    pub fn qk_get_key(&mut self) -> QkGetKeyW<'_, QkCrSpec> {
        QkGetKeyW::new(self, 6)
    }
    #[doc = "Bit 7 - Begin Unwrap operation"]
    #[inline(always)]
    pub fn qk_unwrap(&mut self) -> QkUnwrapW<'_, QkCrSpec> {
        QkUnwrapW::new(self, 7)
    }
    #[doc = "Bit 8 - Begin Wrap Generated Random operation"]
    #[inline(always)]
    pub fn qk_wrap_generated_random(&mut self) -> QkWrapGeneratedRandomW<'_, QkCrSpec> {
        QkWrapGeneratedRandomW::new(self, 8)
    }
    #[doc = "Bit 9 - Begin Wrap operation"]
    #[inline(always)]
    pub fn qk_wrap(&mut self) -> QkWrapW<'_, QkCrSpec> {
        QkWrapW::new(self, 9)
    }
    #[doc = "Bit 15 - Begin Generate Random operation"]
    #[inline(always)]
    pub fn qk_generate_random(&mut self) -> QkGenerateRandomW<'_, QkCrSpec> {
        QkGenerateRandomW::new(self, 15)
    }
    #[doc = "Bit 30 - Begin Test Memory operation"]
    #[inline(always)]
    pub fn qk_test_memory(&mut self) -> QkTestMemoryW<'_, QkCrSpec> {
        QkTestMemoryW::new(self, 30)
    }
    #[doc = "Bit 31 - Begin Test PUF operation"]
    #[inline(always)]
    pub fn qk_test_puf(&mut self) -> QkTestPufW<'_, QkCrSpec> {
        QkTestPufW::new(self, 31)
    }
}
#[doc = "Control register; The Control register defines which command must be executed next. When the command is accepted or rejected, the bits automatically revert to 0. Only one command bit may be written with 1 at a time, with the exception of qk_zeroize. Writing qk_zeroize with 1 takes precedence over all other commands.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkCrSpec;
impl crate::RegisterSpec for QkCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_cr::R`](R) reader structure"]
impl crate::Readable for QkCrSpec {}
#[doc = "`write(|w| ..)` method takes [`qk_cr::W`](W) writer structure"]
impl crate::Writable for QkCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QK_CR to value 0"]
impl crate::Resettable for QkCrSpec {}

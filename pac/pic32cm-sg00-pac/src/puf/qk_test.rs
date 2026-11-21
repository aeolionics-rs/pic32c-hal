#[doc = "Register `QK_TEST` reader"]
pub type R = crate::R<QkTestSpec>;
#[doc = "Register `QK_TEST` writer"]
pub type W = crate::W<QkTestSpec>;
#[doc = "Field `qk_bist_enable` reader - Isolates QuiddiKey and runs BIST"]
pub type QkBistEnableR = crate::BitReader;
#[doc = "Field `qk_bist_enable` writer - Isolates QuiddiKey and runs BIST"]
pub type QkBistEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_bist_running` reader - BIST is in progress or finishing up (after qk_bist_enable = 0)"]
pub type QkBistRunningR = crate::BitReader;
#[doc = "Field `qk_bist_running` writer - BIST is in progress or finishing up (after qk_bist_enable = 0)"]
pub type QkBistRunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_bist_active` reader - BIST is in progress"]
pub type QkBistActiveR = crate::BitReader;
#[doc = "Field `qk_bist_active` writer - BIST is in progress"]
pub type QkBistActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_bist_ok` reader - BIST has passed"]
pub type QkBistOkR = crate::BitReader;
#[doc = "Field `qk_bist_ok` writer - BIST has passed"]
pub type QkBistOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_bist_error` reader - BIST has failed"]
pub type QkBistErrorR = crate::BitReader;
#[doc = "Field `qk_bist_error` writer - BIST has failed"]
pub type QkBistErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_allow_bist` reader - BIST is not allowed"]
pub type QkAllowBistR = crate::BitReader;
#[doc = "Field `qk_allow_bist` writer - BIST is not allowed"]
pub type QkAllowBistW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Isolates QuiddiKey and runs BIST"]
    #[inline(always)]
    pub fn qk_bist_enable(&self) -> QkBistEnableR {
        QkBistEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - BIST is in progress or finishing up (after qk_bist_enable = 0)"]
    #[inline(always)]
    pub fn qk_bist_running(&self) -> QkBistRunningR {
        QkBistRunningR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BIST is in progress"]
    #[inline(always)]
    pub fn qk_bist_active(&self) -> QkBistActiveR {
        QkBistActiveR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BIST has passed"]
    #[inline(always)]
    pub fn qk_bist_ok(&self) -> QkBistOkR {
        QkBistOkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BIST has failed"]
    #[inline(always)]
    pub fn qk_bist_error(&self) -> QkBistErrorR {
        QkBistErrorR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - BIST is not allowed"]
    #[inline(always)]
    pub fn qk_allow_bist(&self) -> QkAllowBistR {
        QkAllowBistR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Isolates QuiddiKey and runs BIST"]
    #[inline(always)]
    pub fn qk_bist_enable(&mut self) -> QkBistEnableW<'_, QkTestSpec> {
        QkBistEnableW::new(self, 0)
    }
    #[doc = "Bit 4 - BIST is in progress or finishing up (after qk_bist_enable = 0)"]
    #[inline(always)]
    pub fn qk_bist_running(&mut self) -> QkBistRunningW<'_, QkTestSpec> {
        QkBistRunningW::new(self, 4)
    }
    #[doc = "Bit 5 - BIST is in progress"]
    #[inline(always)]
    pub fn qk_bist_active(&mut self) -> QkBistActiveW<'_, QkTestSpec> {
        QkBistActiveW::new(self, 5)
    }
    #[doc = "Bit 6 - BIST has passed"]
    #[inline(always)]
    pub fn qk_bist_ok(&mut self) -> QkBistOkW<'_, QkTestSpec> {
        QkBistOkW::new(self, 6)
    }
    #[doc = "Bit 7 - BIST has failed"]
    #[inline(always)]
    pub fn qk_bist_error(&mut self) -> QkBistErrorW<'_, QkTestSpec> {
        QkBistErrorW::new(self, 7)
    }
    #[doc = "Bit 31 - BIST is not allowed"]
    #[inline(always)]
    pub fn qk_allow_bist(&mut self) -> QkAllowBistW<'_, QkTestSpec> {
        QkAllowBistW::new(self, 31)
    }
}
#[doc = "Test register; With the Test register a BIST can be started. It also shows the result of the last BIST.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkTestSpec;
impl crate::RegisterSpec for QkTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_test::R`](R) reader structure"]
impl crate::Readable for QkTestSpec {}
#[doc = "`write(|w| ..)` method takes [`qk_test::W`](W) writer structure"]
impl crate::Writable for QkTestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QK_TEST to value 0"]
impl crate::Resettable for QkTestSpec {}

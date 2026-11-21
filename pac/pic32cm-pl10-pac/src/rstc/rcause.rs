#[doc = "Register `RCAUSE` reader"]
pub type R = crate::R<RcauseSpec>;
#[doc = "Field `POR` reader - Power-On Reset Flag"]
pub type PorR = crate::BitReader;
#[doc = "Field `BORVDD` reader - VDD Brownout Reset Flag"]
pub type BorvddR = crate::BitReader;
#[doc = "Field `EXT` reader - External Reset Flag"]
pub type ExtR = crate::BitReader;
#[doc = "Field `WDT` reader - Watchdog Reset Flag"]
pub type WdtR = crate::BitReader;
#[doc = "Field `SYST` reader - System Reset Flag"]
pub type SystR = crate::BitReader;
#[doc = "Field `LOCKUP` reader - Software Reset Flag"]
pub type LockupR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power-On Reset Flag"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VDD Brownout Reset Flag"]
    #[inline(always)]
    pub fn borvdd(&self) -> BorvddR {
        BorvddR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - External Reset Flag"]
    #[inline(always)]
    pub fn ext(&self) -> ExtR {
        ExtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog Reset Flag"]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Reset Flag"]
    #[inline(always)]
    pub fn syst(&self) -> SystR {
        SystR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software Reset Flag"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Reset Cause\n\nYou can [`read`](crate::Reg::read) this register and get [`rcause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcauseSpec;
impl crate::RegisterSpec for RcauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcause::R`](R) reader structure"]
impl crate::Readable for RcauseSpec {}
#[doc = "`reset()` method sets RCAUSE to value 0"]
impl crate::Resettable for RcauseSpec {}

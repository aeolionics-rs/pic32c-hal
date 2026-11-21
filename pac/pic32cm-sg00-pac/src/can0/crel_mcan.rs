#[doc = "Register `CREL_MCAN` reader"]
pub type R = crate::R<CrelMcanSpec>;
#[doc = "Field `DAY` reader - Timestamp day"]
pub type DayR = crate::FieldReader;
#[doc = "Field `MON` reader - Timestamp month"]
pub type MonR = crate::FieldReader;
#[doc = "Field `YEAR` reader - Timestamp year"]
pub type YearR = crate::FieldReader;
#[doc = "Field `SUBSTEP` reader - Sub-step of Core Release"]
pub type SubstepR = crate::FieldReader;
#[doc = "Field `STEP` reader - Step of Core Release"]
pub type StepR = crate::FieldReader;
#[doc = "Field `REL` reader - Core Release"]
pub type RelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Timestamp day"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timestamp month"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp year"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    pub fn substep(&self) -> SubstepR {
        SubstepR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    pub fn rel(&self) -> RelR {
        RelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Core Release\n\nYou can [`read`](crate::Reg::read) this register and get [`crel_mcan::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrelMcanSpec;
impl crate::RegisterSpec for CrelMcanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crel_mcan::R`](R) reader structure"]
impl crate::Readable for CrelMcanSpec {}
#[doc = "`reset()` method sets CREL_MCAN to value 0x3300_0000"]
impl crate::Resettable for CrelMcanSpec {
    const RESET_VALUE: u32 = 0x3300_0000;
}

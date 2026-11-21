#[doc = "Register `STATUSC` reader"]
pub type R = crate::R<StatuscSpec>;
#[doc = "Field `EVSYS_` reader - EVSYS APB Protect Enable"]
pub type Evsys_R = crate::BitReader;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Protect Enable"]
pub type Sercom0_R = crate::BitReader;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Protect Enable"]
pub type Sercom1_R = crate::BitReader;
#[doc = "Field `TC0_` reader - TC0 APB Protect Enable"]
pub type Tc0_R = crate::BitReader;
#[doc = "Field `TC1_` reader - TC1 APB Protect Enable"]
pub type Tc1_R = crate::BitReader;
#[doc = "Field `TC2_` reader - TC2 APB Protect Enable"]
pub type Tc2_R = crate::BitReader;
#[doc = "Field `TCC0_` reader - TCC0 APB Protect Enable"]
pub type Tcc0_R = crate::BitReader;
#[doc = "Field `ADC0_` reader - ADC0 APB Protect Enable"]
pub type Adc0_R = crate::BitReader;
#[doc = "Field `AC_` reader - AC APB Protect Enable"]
pub type Ac_R = crate::BitReader;
#[doc = "Field `CCL_` reader - CCL APB Protect Enable"]
pub type Ccl_R = crate::BitReader;
#[doc = "Field `PTC_` reader - PTC APB Protect Enable"]
pub type Ptc_R = crate::BitReader;
#[doc = "Field `SYSCTRL_` reader - SYSCTRL APB Protect Enable"]
pub type Sysctrl_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EVSYS APB Protect Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> Evsys_R {
        Evsys_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> Sercom0_R {
        Sercom0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> Sercom1_R {
        Sercom1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> Tc0_R {
        Tc0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> Tc1_R {
        Tc1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> Tc2_R {
        Tc2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TCC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> Tcc0_R {
        Tcc0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0 APB Protect Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> Adc0_R {
        Adc0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AC APB Protect Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> Ac_R {
        Ac_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCL APB Protect Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> Ccl_R {
        Ccl_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PTC APB Protect Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> Ptc_R {
        Ptc_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SYSCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn sysctrl_(&self) -> Sysctrl_R {
        Sysctrl_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Peripheral Write Protection Status C\n\nYou can [`read`](crate::Reg::read) this register and get [`statusc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatuscSpec;
impl crate::RegisterSpec for StatuscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusc::R`](R) reader structure"]
impl crate::Readable for StatuscSpec {}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for StatuscSpec {}

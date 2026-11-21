#[doc = "Register `APBCMASK` reader"]
pub type R = crate::R<ApbcmaskSpec>;
#[doc = "Register `APBCMASK` writer"]
pub type W = crate::W<ApbcmaskSpec>;
#[doc = "Field `EVSYS_` reader - EVSYS APB Clock Enable"]
pub type Evsys_R = crate::BitReader;
#[doc = "Field `EVSYS_` writer - EVSYS APB Clock Enable"]
pub type Evsys_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Clock Enable"]
pub type Sercom0_R = crate::BitReader;
#[doc = "Field `SERCOM0_` writer - SERCOM0 APB Clock Enable"]
pub type Sercom0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Clock Enable"]
pub type Sercom1_R = crate::BitReader;
#[doc = "Field `SERCOM1_` writer - SERCOM1 APB Clock Enable"]
pub type Sercom1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0_` reader - TC0 APB Clock Enable"]
pub type Tc0_R = crate::BitReader;
#[doc = "Field `TC0_` writer - TC0 APB Clock Enable"]
pub type Tc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1_` reader - TC1 APB Clock Enable"]
pub type Tc1_R = crate::BitReader;
#[doc = "Field `TC1_` writer - TC1 APB Clock Enable"]
pub type Tc1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2_` reader - TC2 APB Clock Enable"]
pub type Tc2_R = crate::BitReader;
#[doc = "Field `TC2_` writer - TC2 APB Clock Enable"]
pub type Tc2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC0_` reader - TCC0 APB Clock Enable"]
pub type Tcc0_R = crate::BitReader;
#[doc = "Field `TCC0_` writer - TCC0 APB Clock Enable"]
pub type Tcc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0_` reader - ADC0 APB Clock Enable"]
pub type Adc0_R = crate::BitReader;
#[doc = "Field `ADC0_` writer - ADC0 APB Clock Enable"]
pub type Adc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC_` reader - AC APB Clock Enable"]
pub type Ac_R = crate::BitReader;
#[doc = "Field `AC_` writer - AC APB Clock Enable"]
pub type Ac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCL_` reader - CCL APB Clock Enable"]
pub type Ccl_R = crate::BitReader;
#[doc = "Field `CCL_` writer - CCL APB Clock Enable"]
pub type Ccl_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTC_` reader - PTC APB Clock Enable"]
pub type Ptc_R = crate::BitReader;
#[doc = "Field `PTC_` writer - PTC APB Clock Enable"]
pub type Ptc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCTRL_` reader - SYSCTRL APB Clock Enable"]
pub type Sysctrl_R = crate::BitReader;
#[doc = "Field `SYSCTRL_` writer - SYSCTRL APB Clock Enable"]
pub type Sysctrl_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> Evsys_R {
        Evsys_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> Sercom0_R {
        Sercom0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> Sercom1_R {
        Sercom1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> Tc0_R {
        Tc0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> Tc1_R {
        Tc1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> Tc2_R {
        Tc2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TCC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> Tcc0_R {
        Tcc0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0 APB Clock Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> Adc0_R {
        Adc0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> Ac_R {
        Ac_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> Ccl_R {
        Ccl_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PTC APB Clock Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> Ptc_R {
        Ptc_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SYSCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn sysctrl_(&self) -> Sysctrl_R {
        Sysctrl_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&mut self) -> Evsys_W<'_, ApbcmaskSpec> {
        Evsys_W::new(self, 0)
    }
    #[doc = "Bit 1 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&mut self) -> Sercom0_W<'_, ApbcmaskSpec> {
        Sercom0_W::new(self, 1)
    }
    #[doc = "Bit 2 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&mut self) -> Sercom1_W<'_, ApbcmaskSpec> {
        Sercom1_W::new(self, 2)
    }
    #[doc = "Bit 3 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&mut self) -> Tc0_W<'_, ApbcmaskSpec> {
        Tc0_W::new(self, 3)
    }
    #[doc = "Bit 4 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&mut self) -> Tc1_W<'_, ApbcmaskSpec> {
        Tc1_W::new(self, 4)
    }
    #[doc = "Bit 5 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&mut self) -> Tc2_W<'_, ApbcmaskSpec> {
        Tc2_W::new(self, 5)
    }
    #[doc = "Bit 6 - TCC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc0_(&mut self) -> Tcc0_W<'_, ApbcmaskSpec> {
        Tcc0_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC0 APB Clock Enable"]
    #[inline(always)]
    pub fn adc0_(&mut self) -> Adc0_W<'_, ApbcmaskSpec> {
        Adc0_W::new(self, 7)
    }
    #[doc = "Bit 8 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&mut self) -> Ac_W<'_, ApbcmaskSpec> {
        Ac_W::new(self, 8)
    }
    #[doc = "Bit 9 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&mut self) -> Ccl_W<'_, ApbcmaskSpec> {
        Ccl_W::new(self, 9)
    }
    #[doc = "Bit 10 - PTC APB Clock Enable"]
    #[inline(always)]
    pub fn ptc_(&mut self) -> Ptc_W<'_, ApbcmaskSpec> {
        Ptc_W::new(self, 10)
    }
    #[doc = "Bit 11 - SYSCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn sysctrl_(&mut self) -> Sysctrl_W<'_, ApbcmaskSpec> {
        Sysctrl_W::new(self, 11)
    }
}
#[doc = "APB Bridge C Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbcmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbcmaskSpec;
impl crate::RegisterSpec for ApbcmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbcmask::R`](R) reader structure"]
impl crate::Readable for ApbcmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`apbcmask::W`](W) writer structure"]
impl crate::Writable for ApbcmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBCMASK to value 0x0800"]
impl crate::Resettable for ApbcmaskSpec {
    const RESET_VALUE: u32 = 0x0800;
}

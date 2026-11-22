#[doc = "Register `APBCMASK` reader"]
pub type R = crate::R<ApbcmaskSpec>;
#[doc = "Register `APBCMASK` writer"]
pub type W = crate::W<ApbcmaskSpec>;
#[doc = "Field `PAC2_` reader - PAC2 APB Clock Enable"]
pub type Pac2_R = crate::BitReader;
#[doc = "Field `PAC2_` writer - PAC2 APB Clock Enable"]
pub type Pac2_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Clock Enable"]
pub type Sercom2_R = crate::BitReader;
#[doc = "Field `SERCOM2_` writer - SERCOM2 APB Clock Enable"]
pub type Sercom2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Clock Enable"]
pub type Sercom3_R = crate::BitReader;
#[doc = "Field `SERCOM3_` writer - SERCOM3 APB Clock Enable"]
pub type Sercom3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM4_` reader - SERCOM4 APB Clock Enable"]
pub type Sercom4_R = crate::BitReader;
#[doc = "Field `SERCOM4_` writer - SERCOM4 APB Clock Enable"]
pub type Sercom4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM5_` reader - SERCOM5 APB Clock Enable"]
pub type Sercom5_R = crate::BitReader;
#[doc = "Field `SERCOM5_` writer - SERCOM5 APB Clock Enable"]
pub type Sercom5_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `TC3_` reader - TC3 APB Clock Enable"]
pub type Tc3_R = crate::BitReader;
#[doc = "Field `TC3_` writer - TC3 APB Clock Enable"]
pub type Tc3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC4_` reader - TC4 APB Clock Enable"]
pub type Tc4_R = crate::BitReader;
#[doc = "Field `TC4_` writer - TC4 APB Clock Enable"]
pub type Tc4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC5_` reader - TC5 APB Clock Enable"]
pub type Tc5_R = crate::BitReader;
#[doc = "Field `TC5_` writer - TC5 APB Clock Enable"]
pub type Tc5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC6_` reader - TC6 APB Clock Enable"]
pub type Tc6_R = crate::BitReader;
#[doc = "Field `TC6_` writer - TC6 APB Clock Enable"]
pub type Tc6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC7_` reader - TC7 APB Clock Enable"]
pub type Tc7_R = crate::BitReader;
#[doc = "Field `TC7_` writer - TC7 APB Clock Enable"]
pub type Tc7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_` reader - ADC APB Clock Enable"]
pub type Adc_R = crate::BitReader;
#[doc = "Field `ADC_` writer - ADC APB Clock Enable"]
pub type Adc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC_` reader - AC APB Clock Enable"]
pub type Ac_R = crate::BitReader;
#[doc = "Field `AC_` writer - AC APB Clock Enable"]
pub type Ac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_` reader - DAC APB Clock Enable"]
pub type Dac_R = crate::BitReader;
#[doc = "Field `DAC_` writer - DAC APB Clock Enable"]
pub type Dac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTC_` reader - PTC APB Clock Enable"]
pub type Ptc_R = crate::BitReader;
#[doc = "Field `PTC_` writer - PTC APB Clock Enable"]
pub type Ptc_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PAC2 APB Clock Enable"]
    #[inline(always)]
    pub fn pac2_(&self) -> Pac2_R {
        Pac2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> Evsys_R {
        Evsys_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> Sercom0_R {
        Sercom0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> Sercom1_R {
        Sercom1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> Sercom2_R {
        Sercom2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> Sercom3_R {
        Sercom3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SERCOM4 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> Sercom4_R {
        Sercom4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SERCOM5 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> Sercom5_R {
        Sercom5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> Tc0_R {
        Tc0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> Tc1_R {
        Tc1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> Tc2_R {
        Tc2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> Tc3_R {
        Tc3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TC4 APB Clock Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> Tc4_R {
        Tc4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TC5 APB Clock Enable"]
    #[inline(always)]
    pub fn tc5_(&self) -> Tc5_R {
        Tc5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC6 APB Clock Enable"]
    #[inline(always)]
    pub fn tc6_(&self) -> Tc6_R {
        Tc6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TC7 APB Clock Enable"]
    #[inline(always)]
    pub fn tc7_(&self) -> Tc7_R {
        Tc7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC APB Clock Enable"]
    #[inline(always)]
    pub fn adc_(&self) -> Adc_R {
        Adc_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> Ac_R {
        Ac_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> Dac_R {
        Dac_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PTC APB Clock Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> Ptc_R {
        Ptc_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC2 APB Clock Enable"]
    #[inline(always)]
    pub fn pac2_(&mut self) -> Pac2_W<'_, ApbcmaskSpec> {
        Pac2_W::new(self, 0)
    }
    #[doc = "Bit 1 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&mut self) -> Evsys_W<'_, ApbcmaskSpec> {
        Evsys_W::new(self, 1)
    }
    #[doc = "Bit 2 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&mut self) -> Sercom0_W<'_, ApbcmaskSpec> {
        Sercom0_W::new(self, 2)
    }
    #[doc = "Bit 3 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&mut self) -> Sercom1_W<'_, ApbcmaskSpec> {
        Sercom1_W::new(self, 3)
    }
    #[doc = "Bit 4 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&mut self) -> Sercom2_W<'_, ApbcmaskSpec> {
        Sercom2_W::new(self, 4)
    }
    #[doc = "Bit 5 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom3_(&mut self) -> Sercom3_W<'_, ApbcmaskSpec> {
        Sercom3_W::new(self, 5)
    }
    #[doc = "Bit 6 - SERCOM4 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom4_(&mut self) -> Sercom4_W<'_, ApbcmaskSpec> {
        Sercom4_W::new(self, 6)
    }
    #[doc = "Bit 7 - SERCOM5 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom5_(&mut self) -> Sercom5_W<'_, ApbcmaskSpec> {
        Sercom5_W::new(self, 7)
    }
    #[doc = "Bit 8 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&mut self) -> Tc0_W<'_, ApbcmaskSpec> {
        Tc0_W::new(self, 8)
    }
    #[doc = "Bit 9 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&mut self) -> Tc1_W<'_, ApbcmaskSpec> {
        Tc1_W::new(self, 9)
    }
    #[doc = "Bit 10 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&mut self) -> Tc2_W<'_, ApbcmaskSpec> {
        Tc2_W::new(self, 10)
    }
    #[doc = "Bit 11 - TC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tc3_(&mut self) -> Tc3_W<'_, ApbcmaskSpec> {
        Tc3_W::new(self, 11)
    }
    #[doc = "Bit 12 - TC4 APB Clock Enable"]
    #[inline(always)]
    pub fn tc4_(&mut self) -> Tc4_W<'_, ApbcmaskSpec> {
        Tc4_W::new(self, 12)
    }
    #[doc = "Bit 13 - TC5 APB Clock Enable"]
    #[inline(always)]
    pub fn tc5_(&mut self) -> Tc5_W<'_, ApbcmaskSpec> {
        Tc5_W::new(self, 13)
    }
    #[doc = "Bit 14 - TC6 APB Clock Enable"]
    #[inline(always)]
    pub fn tc6_(&mut self) -> Tc6_W<'_, ApbcmaskSpec> {
        Tc6_W::new(self, 14)
    }
    #[doc = "Bit 15 - TC7 APB Clock Enable"]
    #[inline(always)]
    pub fn tc7_(&mut self) -> Tc7_W<'_, ApbcmaskSpec> {
        Tc7_W::new(self, 15)
    }
    #[doc = "Bit 16 - ADC APB Clock Enable"]
    #[inline(always)]
    pub fn adc_(&mut self) -> Adc_W<'_, ApbcmaskSpec> {
        Adc_W::new(self, 16)
    }
    #[doc = "Bit 17 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&mut self) -> Ac_W<'_, ApbcmaskSpec> {
        Ac_W::new(self, 17)
    }
    #[doc = "Bit 18 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&mut self) -> Dac_W<'_, ApbcmaskSpec> {
        Dac_W::new(self, 18)
    }
    #[doc = "Bit 19 - PTC APB Clock Enable"]
    #[inline(always)]
    pub fn ptc_(&mut self) -> Ptc_W<'_, ApbcmaskSpec> {
        Ptc_W::new(self, 19)
    }
}
#[doc = "APBC Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbcmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets APBCMASK to value 0x0001_0000"]
impl crate::Resettable for ApbcmaskSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}

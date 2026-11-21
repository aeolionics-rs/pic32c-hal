#[doc = "Register `RC48MCAL0_FUSES_OSC_RC48MHZ_V1_MODE` reader"]
pub type R = crate::R<Rc48mcal0FusesOscRc48mhzV1ModeSpec>;
#[doc = "Register `RC48MCAL0_FUSES_OSC_RC48MHZ_V1_MODE` writer"]
pub type W = crate::W<Rc48mcal0FusesOscRc48mhzV1ModeSpec>;
#[doc = "Field `temp_trim_mv` reader - Temp trim value"]
pub type TempTrimMvR = crate::FieldReader;
#[doc = "Field `temp_trim_mv` writer - Temp trim value"]
pub type TempTrimMvW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `proc_trim_mv` reader - Proc trim value"]
pub type ProcTrimMvR = crate::FieldReader;
#[doc = "Field `proc_trim_mv` writer - Proc trim value"]
pub type ProcTrimMvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `iosc_boost` reader - IOSC boost"]
pub type IoscBoostR = crate::BitReader;
#[doc = "Field `iosc_boost` writer - IOSC boost"]
pub type IoscBoostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `out_buf_sel` reader - Output buffer select"]
pub type OutBufSelR = crate::BitReader;
#[doc = "Field `out_buf_sel` writer - Output buffer select"]
pub type OutBufSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo_vout_boost` reader - LDO VOUT boost"]
pub type LdoVoutBoostR = crate::BitReader;
#[doc = "Field `ldo_vout_boost` writer - LDO VOUT boost"]
pub type LdoVoutBoostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp_8m_lp` reader - Comparator 8MHz low-power"]
pub type Cmp8mLpR = crate::BitReader;
#[doc = "Field `cmp_8m_lp` writer - Comparator 8MHz low-power"]
pub type Cmp8mLpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp_pwr_ctrl` reader - Comparator power control"]
pub type CmpPwrCtrlR = crate::FieldReader;
#[doc = "Field `cmp_pwr_ctrl` writer - Comparator power control"]
pub type CmpPwrCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Temp trim value"]
    #[inline(always)]
    pub fn temp_trim_mv(&self) -> TempTrimMvR {
        TempTrimMvR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:13 - Proc trim value"]
    #[inline(always)]
    pub fn proc_trim_mv(&self) -> ProcTrimMvR {
        ProcTrimMvR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bit 16 - IOSC boost"]
    #[inline(always)]
    pub fn iosc_boost(&self) -> IoscBoostR {
        IoscBoostR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output buffer select"]
    #[inline(always)]
    pub fn out_buf_sel(&self) -> OutBufSelR {
        OutBufSelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LDO VOUT boost"]
    #[inline(always)]
    pub fn ldo_vout_boost(&self) -> LdoVoutBoostR {
        LdoVoutBoostR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Comparator 8MHz low-power"]
    #[inline(always)]
    pub fn cmp_8m_lp(&self) -> Cmp8mLpR {
        Cmp8mLpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Comparator power control"]
    #[inline(always)]
    pub fn cmp_pwr_ctrl(&self) -> CmpPwrCtrlR {
        CmpPwrCtrlR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Temp trim value"]
    #[inline(always)]
    pub fn temp_trim_mv(&mut self) -> TempTrimMvW<'_, Rc48mcal0FusesOscRc48mhzV1ModeSpec> {
        TempTrimMvW::new(self, 0)
    }
    #[doc = "Bits 6:13 - Proc trim value"]
    #[inline(always)]
    pub fn proc_trim_mv(&mut self) -> ProcTrimMvW<'_, Rc48mcal0FusesOscRc48mhzV1ModeSpec> {
        ProcTrimMvW::new(self, 6)
    }
    #[doc = "Bit 16 - IOSC boost"]
    #[inline(always)]
    pub fn iosc_boost(&mut self) -> IoscBoostW<'_, Rc48mcal0FusesOscRc48mhzV1ModeSpec> {
        IoscBoostW::new(self, 16)
    }
    #[doc = "Bit 17 - Output buffer select"]
    #[inline(always)]
    pub fn out_buf_sel(&mut self) -> OutBufSelW<'_, Rc48mcal0FusesOscRc48mhzV1ModeSpec> {
        OutBufSelW::new(self, 17)
    }
    #[doc = "Bit 18 - LDO VOUT boost"]
    #[inline(always)]
    pub fn ldo_vout_boost(&mut self) -> LdoVoutBoostW<'_, Rc48mcal0FusesOscRc48mhzV1ModeSpec> {
        LdoVoutBoostW::new(self, 18)
    }
    #[doc = "Bit 19 - Comparator 8MHz low-power"]
    #[inline(always)]
    pub fn cmp_8m_lp(&mut self) -> Cmp8mLpW<'_, Rc48mcal0FusesOscRc48mhzV1ModeSpec> {
        Cmp8mLpW::new(self, 19)
    }
    #[doc = "Bits 20:22 - Comparator power control"]
    #[inline(always)]
    pub fn cmp_pwr_ctrl(&mut self) -> CmpPwrCtrlW<'_, Rc48mcal0FusesOscRc48mhzV1ModeSpec> {
        CmpPwrCtrlW::new(self, 20)
    }
}
#[doc = "RC48M Calibration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rc48mcal0_fuses_osc_rc48mhz_v1_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc48mcal0_fuses_osc_rc48mhz_v1_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rc48mcal0FusesOscRc48mhzV1ModeSpec;
impl crate::RegisterSpec for Rc48mcal0FusesOscRc48mhzV1ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc48mcal0_fuses_osc_rc48mhz_v1_mode::R`](R) reader structure"]
impl crate::Readable for Rc48mcal0FusesOscRc48mhzV1ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`rc48mcal0_fuses_osc_rc48mhz_v1_mode::W`](W) writer structure"]
impl crate::Writable for Rc48mcal0FusesOscRc48mhzV1ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RC48MCAL0_FUSES_OSC_RC48MHZ_V1_MODE to value 0"]
impl crate::Resettable for Rc48mcal0FusesOscRc48mhzV1ModeSpec {}

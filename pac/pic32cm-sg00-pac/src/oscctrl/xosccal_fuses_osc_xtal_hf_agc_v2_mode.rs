#[doc = "Register `XOSCCAL_FUSES_OSC_XTAL_HF_AGC_V2_MODE` reader"]
pub type R = crate::R<XosccalFusesOscXtalHfAgcV2ModeSpec>;
#[doc = "Register `XOSCCAL_FUSES_OSC_XTAL_HF_AGC_V2_MODE` writer"]
pub type W = crate::W<XosccalFusesOscXtalHfAgcV2ModeSpec>;
#[doc = "Field `FTRANS` reader - Fine Transconductance programmability for Oscillator"]
pub type FtransR = crate::FieldReader;
#[doc = "Field `FTRANS` writer - Fine Transconductance programmability for Oscillator"]
pub type FtransW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KICKEREN` reader - Kicker enable"]
pub type KickerenR = crate::BitReader;
#[doc = "Field `KICKEREN` writer - Kicker enable"]
pub type KickerenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENVAMP` reader - envelope amplitude at osci"]
pub type EnvampR = crate::BitReader;
#[doc = "Field `ENVAMP` writer - envelope amplitude at osci"]
pub type EnvampW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - loop hysteresis control"]
pub type HystR = crate::BitReader;
#[doc = "Field `HYST` writer - loop hysteresis control"]
pub type HystW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMSEL` reader - clock buffer common mode selection"]
pub type CmselR = crate::BitReader;
#[doc = "Field `CMSEL` writer - clock buffer common mode selection"]
pub type CmselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLIPPOL` reader - flip output clock polarity"]
pub type FlippolR = crate::BitReader;
#[doc = "Field `FLIPPOL` writer - flip output clock polarity"]
pub type FlippolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDGAIN` reader - reduce gain control timer"]
pub type RedgainR = crate::BitReader;
#[doc = "Field `REDGAIN` writer - reduce gain control timer"]
pub type RedgainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDOSC` reader - reduce oscillation debug counter"]
pub type RedoscR = crate::BitReader;
#[doc = "Field `REDOSC` writer - reduce oscillation debug counter"]
pub type RedoscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARES` reader - Spare bits"]
pub type SparesR = crate::FieldReader;
#[doc = "Field `SPARES` writer - Spare bits"]
pub type SparesW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Fine Transconductance programmability for Oscillator"]
    #[inline(always)]
    pub fn ftrans(&self) -> FtransR {
        FtransR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Kicker enable"]
    #[inline(always)]
    pub fn kickeren(&self) -> KickerenR {
        KickerenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - envelope amplitude at osci"]
    #[inline(always)]
    pub fn envamp(&self) -> EnvampR {
        EnvampR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - loop hysteresis control"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clock buffer common mode selection"]
    #[inline(always)]
    pub fn cmsel(&self) -> CmselR {
        CmselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - flip output clock polarity"]
    #[inline(always)]
    pub fn flippol(&self) -> FlippolR {
        FlippolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - reduce gain control timer"]
    #[inline(always)]
    pub fn redgain(&self) -> RedgainR {
        RedgainR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reduce oscillation debug counter"]
    #[inline(always)]
    pub fn redosc(&self) -> RedoscR {
        RedoscR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - Spare bits"]
    #[inline(always)]
    pub fn spares(&self) -> SparesR {
        SparesR::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Fine Transconductance programmability for Oscillator"]
    #[inline(always)]
    pub fn ftrans(&mut self) -> FtransW<'_, XosccalFusesOscXtalHfAgcV2ModeSpec> {
        FtransW::new(self, 0)
    }
    #[doc = "Bit 2 - Kicker enable"]
    #[inline(always)]
    pub fn kickeren(&mut self) -> KickerenW<'_, XosccalFusesOscXtalHfAgcV2ModeSpec> {
        KickerenW::new(self, 2)
    }
    #[doc = "Bit 3 - envelope amplitude at osci"]
    #[inline(always)]
    pub fn envamp(&mut self) -> EnvampW<'_, XosccalFusesOscXtalHfAgcV2ModeSpec> {
        EnvampW::new(self, 3)
    }
    #[doc = "Bit 4 - loop hysteresis control"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, XosccalFusesOscXtalHfAgcV2ModeSpec> {
        HystW::new(self, 4)
    }
    #[doc = "Bit 5 - clock buffer common mode selection"]
    #[inline(always)]
    pub fn cmsel(&mut self) -> CmselW<'_, XosccalFusesOscXtalHfAgcV2ModeSpec> {
        CmselW::new(self, 5)
    }
    #[doc = "Bit 6 - flip output clock polarity"]
    #[inline(always)]
    pub fn flippol(&mut self) -> FlippolW<'_, XosccalFusesOscXtalHfAgcV2ModeSpec> {
        FlippolW::new(self, 6)
    }
    #[doc = "Bit 8 - reduce gain control timer"]
    #[inline(always)]
    pub fn redgain(&mut self) -> RedgainW<'_, XosccalFusesOscXtalHfAgcV2ModeSpec> {
        RedgainW::new(self, 8)
    }
    #[doc = "Bit 9 - reduce oscillation debug counter"]
    #[inline(always)]
    pub fn redosc(&mut self) -> RedoscW<'_, XosccalFusesOscXtalHfAgcV2ModeSpec> {
        RedoscW::new(self, 9)
    }
    #[doc = "Bits 10:15 - Spare bits"]
    #[inline(always)]
    pub fn spares(&mut self) -> SparesW<'_, XosccalFusesOscXtalHfAgcV2ModeSpec> {
        SparesW::new(self, 10)
    }
}
#[doc = "XOSC Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xosccal_fuses_osc_xtal_hf_agc_v2_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosccal_fuses_osc_xtal_hf_agc_v2_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XosccalFusesOscXtalHfAgcV2ModeSpec;
impl crate::RegisterSpec for XosccalFusesOscXtalHfAgcV2ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xosccal_fuses_osc_xtal_hf_agc_v2_mode::R`](R) reader structure"]
impl crate::Readable for XosccalFusesOscXtalHfAgcV2ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`xosccal_fuses_osc_xtal_hf_agc_v2_mode::W`](W) writer structure"]
impl crate::Writable for XosccalFusesOscXtalHfAgcV2ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XOSCCAL_FUSES_OSC_XTAL_HF_AGC_V2_MODE to value 0"]
impl crate::Resettable for XosccalFusesOscXtalHfAgcV2ModeSpec {}

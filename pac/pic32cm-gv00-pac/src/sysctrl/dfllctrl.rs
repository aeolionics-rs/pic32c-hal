#[doc = "Register `DFLLCTRL` reader"]
pub type R = crate::R<DfllctrlSpec>;
#[doc = "Register `DFLLCTRL` writer"]
pub type W = crate::W<DfllctrlSpec>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Mode Selection"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - Mode Selection"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STABLE` reader - Stable Frequency"]
pub type StableR = crate::BitReader;
#[doc = "Field `STABLE` writer - Stable Frequency"]
pub type StableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLAW` reader - Lose Lock After Wake"]
pub type LlawR = crate::BitReader;
#[doc = "Field `LLAW` writer - Lose Lock After Wake"]
pub type LlawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - Enable on Demand"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - Enable on Demand"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDIS` reader - Chill Cycle Disable"]
pub type CcdisR = crate::BitReader;
#[doc = "Field `CCDIS` writer - Chill Cycle Disable"]
pub type CcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QLDIS` reader - Quick Lock Disable"]
pub type QldisR = crate::BitReader;
#[doc = "Field `QLDIS` writer - Quick Lock Disable"]
pub type QldisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stable Frequency"]
    #[inline(always)]
    pub fn stable(&self) -> StableR {
        StableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&self) -> LlawR {
        LlawR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable on Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&self) -> CcdisR {
        CcdisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&self) -> QldisR {
        QldisR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, DfllctrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Mode Selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, DfllctrlSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 3 - Stable Frequency"]
    #[inline(always)]
    pub fn stable(&mut self) -> StableW<'_, DfllctrlSpec> {
        StableW::new(self, 3)
    }
    #[doc = "Bit 4 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&mut self) -> LlawW<'_, DfllctrlSpec> {
        LlawW::new(self, 4)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, DfllctrlSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable on Demand"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<'_, DfllctrlSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bit 8 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&mut self) -> CcdisW<'_, DfllctrlSpec> {
        CcdisW::new(self, 8)
    }
    #[doc = "Bit 9 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&mut self) -> QldisW<'_, DfllctrlSpec> {
        QldisW::new(self, 9)
    }
}
#[doc = "DFLL Config\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfllctrlSpec;
impl crate::RegisterSpec for DfllctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dfllctrl::R`](R) reader structure"]
impl crate::Readable for DfllctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dfllctrl::W`](W) writer structure"]
impl crate::Writable for DfllctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFLLCTRL to value 0x80"]
impl crate::Resettable for DfllctrlSpec {
    const RESET_VALUE: u16 = 0x80;
}

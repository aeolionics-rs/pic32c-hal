#[doc = "Register `anti_tamper__FILTERCFG` reader"]
pub type R = crate::R<AntiTamper_FiltercfgSpec>;
#[doc = "Register `anti_tamper__FILTERCFG` writer"]
pub type W = crate::W<AntiTamper_FiltercfgSpec>;
#[doc = "Field `THRESH` reader - Threshold."]
pub type ThreshR = crate::FieldReader;
#[doc = "Field `THRESH` writer - Threshold."]
pub type ThreshW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PERIOD` reader - Period."]
pub type PeriodR = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Period."]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PROTKEY` reader - Protection key (0x92F5)."]
pub type ProtkeyR = crate::FieldReader<u16>;
#[doc = "Field `PROTKEY` writer - Protection key (0x92F5)."]
pub type ProtkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - Threshold."]
    #[inline(always)]
    pub fn thresh(&self) -> ThreshR {
        ThreshR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - Period."]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - Protection key (0x92F5)."]
    #[inline(always)]
    pub fn protkey(&self) -> ProtkeyR {
        ProtkeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Threshold."]
    #[inline(always)]
    pub fn thresh(&mut self) -> ThreshW<'_, AntiTamper_FiltercfgSpec> {
        ThreshW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Period."]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, AntiTamper_FiltercfgSpec> {
        PeriodW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Protection key (0x92F5)."]
    #[inline(always)]
    pub fn protkey(&mut self) -> ProtkeyW<'_, AntiTamper_FiltercfgSpec> {
        ProtkeyW::new(self, 16)
    }
}
#[doc = "Filter configuration (threshold, period).\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__filtercfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__filtercfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_FiltercfgSpec;
impl crate::RegisterSpec for AntiTamper_FiltercfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__filtercfg::R`](R) reader structure"]
impl crate::Readable for AntiTamper_FiltercfgSpec {}
#[doc = "`write(|w| ..)` method takes [`anti_tamper__filtercfg::W`](W) writer structure"]
impl crate::Writable for AntiTamper_FiltercfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets anti_tamper__FILTERCFG to value 0"]
impl crate::Resettable for AntiTamper_FiltercfgSpec {}

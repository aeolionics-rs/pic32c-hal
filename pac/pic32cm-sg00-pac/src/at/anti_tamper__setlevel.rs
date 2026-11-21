#[doc = "Register `anti_tamper__SETLEVEL` writer"]
pub type W = crate::W<AntiTamper_SetlevelSpec>;
#[doc = "Field `LEVEL` writer - Tamper level."]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ID` writer - Tamper ID."]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PROTKEY` writer - Protection key (0xA84C)."]
pub type ProtkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:2 - Tamper level."]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, AntiTamper_SetlevelSpec> {
        LevelW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Tamper ID."]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, AntiTamper_SetlevelSpec> {
        IdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Protection key (0xA84C)."]
    #[inline(always)]
    pub fn protkey(&mut self) -> ProtkeyW<'_, AntiTamper_SetlevelSpec> {
        ProtkeyW::new(self, 16)
    }
}
#[doc = "Set tamper level.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__setlevel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_SetlevelSpec;
impl crate::RegisterSpec for AntiTamper_SetlevelSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`anti_tamper__setlevel::W`](W) writer structure"]
impl crate::Writable for AntiTamper_SetlevelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets anti_tamper__SETLEVEL to value 0"]
impl crate::Resettable for AntiTamper_SetlevelSpec {}

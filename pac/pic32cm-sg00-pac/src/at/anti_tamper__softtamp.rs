#[doc = "Register `anti_tamper__SOFTTAMP` reader"]
pub type R = crate::R<AntiTamper_SofttampSpec>;
#[doc = "Register `anti_tamper__SOFTTAMP` writer"]
pub type W = crate::W<AntiTamper_SofttampSpec>;
#[doc = "Field `ACTIVE` reader - Tamper active."]
pub type ActiveR = crate::BitReader;
#[doc = "Field `ACTIVE` writer - Tamper active."]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID` reader - Tamper ID."]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - Tamper ID."]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PROTKEY` reader - Protection key (0x4BEE)."]
pub type ProtkeyR = crate::FieldReader<u16>;
#[doc = "Field `PROTKEY` writer - Protection key (0x4BEE)."]
pub type ProtkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Tamper active."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:12 - Tamper ID."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - Protection key (0x4BEE)."]
    #[inline(always)]
    pub fn protkey(&self) -> ProtkeyR {
        ProtkeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper active."]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<'_, AntiTamper_SofttampSpec> {
        ActiveW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Tamper ID."]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, AntiTamper_SofttampSpec> {
        IdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Protection key (0x4BEE)."]
    #[inline(always)]
    pub fn protkey(&mut self) -> ProtkeyW<'_, AntiTamper_SofttampSpec> {
        ProtkeyW::new(self, 16)
    }
}
#[doc = "Software triggered tamper.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__softtamp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__softtamp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_SofttampSpec;
impl crate::RegisterSpec for AntiTamper_SofttampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__softtamp::R`](R) reader structure"]
impl crate::Readable for AntiTamper_SofttampSpec {}
#[doc = "`write(|w| ..)` method takes [`anti_tamper__softtamp::W`](W) writer structure"]
impl crate::Writable for AntiTamper_SofttampSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets anti_tamper__SOFTTAMP to value 0"]
impl crate::Resettable for AntiTamper_SofttampSpec {}

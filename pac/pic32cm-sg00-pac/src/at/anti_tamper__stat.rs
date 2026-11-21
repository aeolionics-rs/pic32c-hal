#[doc = "Register `anti_tamper__STAT` reader"]
pub type R = crate::R<AntiTamper_StatSpec>;
#[doc = "Register `anti_tamper__STAT` writer"]
pub type W = crate::W<AntiTamper_StatSpec>;
#[doc = "Field `STAT` reader - Status (write to clear)."]
pub type StatR = crate::FieldReader<u32>;
#[doc = "Field `STAT` writer - Status (write to clear)."]
pub type StatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Status (write to clear)."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Status (write to clear)."]
    #[inline(always)]
    pub fn stat(&mut self) -> StatW<'_, AntiTamper_StatSpec> {
        StatW::new(self, 0)
    }
}
#[doc = "Status (write to clear).\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_StatSpec;
impl crate::RegisterSpec for AntiTamper_StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__stat::R`](R) reader structure"]
impl crate::Readable for AntiTamper_StatSpec {}
#[doc = "`write(|w| ..)` method takes [`anti_tamper__stat::W`](W) writer structure"]
impl crate::Writable for AntiTamper_StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets anti_tamper__STAT to value 0x01"]
impl crate::Resettable for AntiTamper_StatSpec {
    const RESET_VALUE: u32 = 0x01;
}

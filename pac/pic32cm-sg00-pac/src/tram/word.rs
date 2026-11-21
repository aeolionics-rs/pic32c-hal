#[doc = "Register `WORD[%s]` reader"]
pub type R = crate::R<WordSpec>;
#[doc = "Register `WORD[%s]` writer"]
pub type W = crate::W<WordSpec>;
#[doc = "Field `DATA` reader - Trust RAM Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Trust RAM Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Trust RAM Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trust RAM Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, WordSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "TrustRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`word::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WordSpec;
impl crate::RegisterSpec for WordSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`word::R`](R) reader structure"]
impl crate::Readable for WordSpec {}
#[doc = "`write(|w| ..)` method takes [`word::W`](W) writer structure"]
impl crate::Writable for WordSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WORD[%s] to value 0"]
impl crate::Resettable for WordSpec {}

#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `SERREN` reader - Single Bit Error Interrupt Enable, Set"]
pub type SerrenR = crate::BitReader;
#[doc = "Field `SERREN` writer - Single Bit Error Interrupt Enable, Set"]
pub type SerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DERREN` reader - Double Bit Error Interrupt Enable, Set"]
pub type DerrenR = crate::BitReader;
#[doc = "Field `DERREN` writer - Double Bit Error Interrupt Enable, Set"]
pub type DerrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single Bit Error Interrupt Enable, Set"]
    #[inline(always)]
    pub fn serren(&self) -> SerrenR {
        SerrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Double Bit Error Interrupt Enable, Set"]
    #[inline(always)]
    pub fn derren(&self) -> DerrenR {
        DerrenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit Error Interrupt Enable, Set"]
    #[inline(always)]
    pub fn serren(&mut self) -> SerrenW<'_, IntensetSpec> {
        SerrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Double Bit Error Interrupt Enable, Set"]
    #[inline(always)]
    pub fn derren(&mut self) -> DerrenW<'_, IntensetSpec> {
        DerrenW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}

#[doc = "Register `INTSTA` reader"]
pub type R = crate::R<IntstaSpec>;
#[doc = "Register `INTSTA` writer"]
pub type W = crate::W<IntstaSpec>;
#[doc = "Field `SERR` reader - Single Bit Error"]
pub type SerrR = crate::BitReader;
#[doc = "Field `SERR` writer - Single Bit Error"]
pub type SerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DERR` reader - Double Bit Error"]
pub type DerrR = crate::BitReader;
#[doc = "Field `DERR` writer - Double Bit Error"]
pub type DerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single Bit Error"]
    #[inline(always)]
    pub fn serr(&self) -> SerrR {
        SerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Double Bit Error"]
    #[inline(always)]
    pub fn derr(&self) -> DerrR {
        DerrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit Error"]
    #[inline(always)]
    pub fn serr(&mut self) -> SerrW<'_, IntstaSpec> {
        SerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Double Bit Error"]
    #[inline(always)]
    pub fn derr(&mut self) -> DerrW<'_, IntstaSpec> {
        DerrW::new(self, 1)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstaSpec;
impl crate::RegisterSpec for IntstaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsta::R`](R) reader structure"]
impl crate::Readable for IntstaSpec {}
#[doc = "`write(|w| ..)` method takes [`intsta::W`](W) writer structure"]
impl crate::Writable for IntstaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSTA to value 0"]
impl crate::Resettable for IntstaSpec {}

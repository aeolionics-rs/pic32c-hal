#[doc = "Register `PINEN1` reader"]
pub type R = crate::R<Pinen1Spec>;
#[doc = "Register `PINEN1` writer"]
pub type W = crate::W<Pinen1Spec>;
#[doc = "Field `PINEN` reader - PTC-Line 31-0 Enable"]
pub type PinenR = crate::FieldReader<u32>;
#[doc = "Field `PINEN` writer - PTC-Line 31-0 Enable"]
pub type PinenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PTC-Line 31-0 Enable"]
    #[inline(always)]
    pub fn pinen(&self) -> PinenR {
        PinenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PTC-Line 31-0 Enable"]
    #[inline(always)]
    pub fn pinen(&mut self) -> PinenW<'_, Pinen1Spec> {
        PinenW::new(self, 0)
    }
}
#[doc = "PTC Line Enable 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pinen1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinen1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinen1Spec;
impl crate::RegisterSpec for Pinen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinen1::R`](R) reader structure"]
impl crate::Readable for Pinen1Spec {}
#[doc = "`write(|w| ..)` method takes [`pinen1::W`](W) writer structure"]
impl crate::Writable for Pinen1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINEN1 to value 0"]
impl crate::Resettable for Pinen1Spec {}

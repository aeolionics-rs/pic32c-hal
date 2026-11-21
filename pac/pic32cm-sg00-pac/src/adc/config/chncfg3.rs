#[doc = "Register `CHNCFG3` reader"]
pub type R = crate::R<Chncfg3Spec>;
#[doc = "Register `CHNCFG3` writer"]
pub type W = crate::W<Chncfg3Spec>;
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DiffR = crate::FieldReader<u16>;
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DiffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SIGN` reader - SIGN setting"]
pub type SignR = crate::FieldReader<u16>;
#[doc = "Field `SIGN` writer - SIGN setting"]
pub type SignW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SIGN setting"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DiffW<'_, Chncfg3Spec> {
        DiffW::new(self, 0)
    }
    #[doc = "Bits 16:31 - SIGN setting"]
    #[inline(always)]
    pub fn sign(&mut self) -> SignW<'_, Chncfg3Spec> {
        SignW::new(self, 16)
    }
}
#[doc = "Channel Configuration3 (SIGN/DIFF)\n\nYou can [`read`](crate::Reg::read) this register and get [`chncfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chncfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chncfg3Spec;
impl crate::RegisterSpec for Chncfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chncfg3::R`](R) reader structure"]
impl crate::Readable for Chncfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`chncfg3::W`](W) writer structure"]
impl crate::Writable for Chncfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNCFG3 to value 0"]
impl crate::Resettable for Chncfg3Spec {}

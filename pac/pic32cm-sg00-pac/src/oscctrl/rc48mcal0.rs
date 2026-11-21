#[doc = "Register `RC48MCAL0` reader"]
pub type R = crate::R<Rc48mcal0Spec>;
#[doc = "Register `RC48MCAL0` writer"]
pub type W = crate::W<Rc48mcal0Spec>;
#[doc = "Field `CAL` reader - RC48M Calibration"]
pub type CalR = crate::FieldReader<u32>;
#[doc = "Field `CAL` writer - RC48M Calibration"]
pub type CalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RC48M Calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CalR {
        CalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RC48M Calibration"]
    #[inline(always)]
    pub fn cal(&mut self) -> CalW<'_, Rc48mcal0Spec> {
        CalW::new(self, 0)
    }
}
#[doc = "RC48M Calibration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rc48mcal0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc48mcal0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rc48mcal0Spec;
impl crate::RegisterSpec for Rc48mcal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc48mcal0::R`](R) reader structure"]
impl crate::Readable for Rc48mcal0Spec {}
#[doc = "`write(|w| ..)` method takes [`rc48mcal0::W`](W) writer structure"]
impl crate::Writable for Rc48mcal0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RC48MCAL0 to value 0"]
impl crate::Resettable for Rc48mcal0Spec {}

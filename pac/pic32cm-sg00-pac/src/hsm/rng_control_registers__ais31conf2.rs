#[doc = "Register `RNG_Control_Registers__AIS31Conf2` reader"]
pub type R = crate::R<RngControlRegisters_Ais31conf2Spec>;
#[doc = "Register `RNG_Control_Registers__AIS31Conf2` writer"]
pub type W = crate::W<RngControlRegisters_Ais31conf2Spec>;
#[doc = "Field `HMin` reader - Minimum allowed history value."]
pub type HminR = crate::FieldReader<u16>;
#[doc = "Field `HMin` writer - Minimum allowed history value."]
pub type HminW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HMa` reader - Maximum allowed history value."]
pub type HmaR = crate::FieldReader<u16>;
#[doc = "Field `HMa` writer - Maximum allowed history value."]
pub type HmaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Minimum allowed history value."]
    #[inline(always)]
    pub fn hmin(&self) -> HminR {
        HminR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Maximum allowed history value."]
    #[inline(always)]
    pub fn hma(&self) -> HmaR {
        HmaR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum allowed history value."]
    #[inline(always)]
    pub fn hmin(&mut self) -> HminW<'_, RngControlRegisters_Ais31conf2Spec> {
        HminW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Maximum allowed history value."]
    #[inline(always)]
    pub fn hma(&mut self) -> HmaW<'_, RngControlRegisters_Ais31conf2Spec> {
        HmaW::new(self, 16)
    }
}
#[doc = "AIS31 configuration register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__ais31conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__ais31conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_Ais31conf2Spec;
impl crate::RegisterSpec for RngControlRegisters_Ais31conf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__ais31conf2::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_Ais31conf2Spec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__ais31conf2::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_Ais31conf2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__AIS31Conf2 to value 0"]
impl crate::Resettable for RngControlRegisters_Ais31conf2Spec {}

#[doc = "Register `RNG_Control_Registers__DisableOsc1` reader"]
pub type R = crate::R<RngControlRegisters_DisableOsc1Spec>;
#[doc = "Register `RNG_Control_Registers__DisableOsc1` writer"]
pub type W = crate::W<RngControlRegisters_DisableOsc1Spec>;
#[doc = "Field `DisableOsc1` reader - Disable oscillator rings #32 to #63."]
pub type DisableOsc1R = crate::FieldReader<u32>;
#[doc = "Field `DisableOsc1` writer - Disable oscillator rings #32 to #63."]
pub type DisableOsc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Disable oscillator rings #32 to #63."]
    #[inline(always)]
    pub fn disable_osc1(&self) -> DisableOsc1R {
        DisableOsc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Disable oscillator rings #32 to #63."]
    #[inline(always)]
    pub fn disable_osc1(&mut self) -> DisableOsc1W<'_, RngControlRegisters_DisableOsc1Spec> {
        DisableOsc1W::new(self, 0)
    }
}
#[doc = "Disable oscillator rings #32 to #63.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__disable_osc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__disable_osc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_DisableOsc1Spec;
impl crate::RegisterSpec for RngControlRegisters_DisableOsc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__disable_osc1::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_DisableOsc1Spec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__disable_osc1::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_DisableOsc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__DisableOsc1 to value 0"]
impl crate::Resettable for RngControlRegisters_DisableOsc1Spec {}

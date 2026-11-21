#[doc = "Register `RNG_Control_Registers__PropThreshold` reader"]
pub type R = crate::R<RngControlRegisters_PropThresholdSpec>;
#[doc = "Register `RNG_Control_Registers__PropThreshold` writer"]
pub type W = crate::W<RngControlRegisters_PropThresholdSpec>;
#[doc = "Field `PropThreshold` reader - Adaptive Proportion Test (1024-sample window) Cut-Off value."]
pub type PropThresholdR = crate::FieldReader<u16>;
#[doc = "Field `PropThreshold` writer - Adaptive Proportion Test (1024-sample window) Cut-Off value."]
pub type PropThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Adaptive Proportion Test (1024-sample window) Cut-Off value."]
    #[inline(always)]
    pub fn prop_threshold(&self) -> PropThresholdR {
        PropThresholdR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Adaptive Proportion Test (1024-sample window) Cut-Off value."]
    #[inline(always)]
    pub fn prop_threshold(&mut self) -> PropThresholdW<'_, RngControlRegisters_PropThresholdSpec> {
        PropThresholdW::new(self, 0)
    }
}
#[doc = "Adaptive Proportion Test (1024-sample window) Cut-Off value.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__prop_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__prop_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_PropThresholdSpec;
impl crate::RegisterSpec for RngControlRegisters_PropThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__prop_threshold::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_PropThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__prop_threshold::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_PropThresholdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__PropThreshold to value 0x0319"]
impl crate::Resettable for RngControlRegisters_PropThresholdSpec {
    const RESET_VALUE: u32 = 0x0319;
}

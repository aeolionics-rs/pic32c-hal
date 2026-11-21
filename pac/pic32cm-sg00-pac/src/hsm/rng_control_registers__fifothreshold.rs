#[doc = "Register `RNG_Control_Registers__FIFOThreshold` reader"]
pub type R = crate::R<RngControlRegisters_FifothresholdSpec>;
#[doc = "Register `RNG_Control_Registers__FIFOThreshold` writer"]
pub type W = crate::W<RngControlRegisters_FifothresholdSpec>;
#[doc = "Field `FIFOThreshold` reader - FIFO level below which the module leaves the idle state to refill the FIFO, expressed in number of 128bit blocks."]
pub type FifothresholdR = crate::FieldReader;
#[doc = "Field `FIFOThreshold` writer - FIFO level below which the module leaves the idle state to refill the FIFO, expressed in number of 128bit blocks."]
pub type FifothresholdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - FIFO level below which the module leaves the idle state to refill the FIFO, expressed in number of 128bit blocks."]
    #[inline(always)]
    pub fn fifothreshold(&self) -> FifothresholdR {
        FifothresholdR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO level below which the module leaves the idle state to refill the FIFO, expressed in number of 128bit blocks."]
    #[inline(always)]
    pub fn fifothreshold(&mut self) -> FifothresholdW<'_, RngControlRegisters_FifothresholdSpec> {
        FifothresholdW::new(self, 0)
    }
}
#[doc = "FIFO threshold register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__fifothreshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__fifothreshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_FifothresholdSpec;
impl crate::RegisterSpec for RngControlRegisters_FifothresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__fifothreshold::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_FifothresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__fifothreshold::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_FifothresholdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__FIFOThreshold to value 0x01"]
impl crate::Resettable for RngControlRegisters_FifothresholdSpec {
    const RESET_VALUE: u32 = 0x01;
}

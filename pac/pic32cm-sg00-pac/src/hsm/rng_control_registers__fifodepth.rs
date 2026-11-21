#[doc = "Register `RNG_Control_Registers__FIFODepth` reader"]
pub type R = crate::R<RngControlRegisters_FifodepthSpec>;
#[doc = "Field `FIFODepth` reader - Maximum number of 32 bits words that can be stored in the FIFO: 2**g_fifodepth."]
pub type FifodepthR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Maximum number of 32 bits words that can be stored in the FIFO: 2**g_fifodepth."]
    #[inline(always)]
    pub fn fifodepth(&self) -> FifodepthR {
        FifodepthR::new(self.bits)
    }
}
#[doc = "FIFO depth register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__fifodepth::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_FifodepthSpec;
impl crate::RegisterSpec for RngControlRegisters_FifodepthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__fifodepth::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_FifodepthSpec {}
#[doc = "`reset()` method sets RNG_Control_Registers__FIFODepth to value 0x08"]
impl crate::Resettable for RngControlRegisters_FifodepthSpec {
    const RESET_VALUE: u32 = 0x08;
}

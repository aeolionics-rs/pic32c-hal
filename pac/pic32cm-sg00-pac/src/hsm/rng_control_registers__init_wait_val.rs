#[doc = "Register `RNG_Control_Registers__InitWaitVal` reader"]
pub type R = crate::R<RngControlRegisters_InitWaitValSpec>;
#[doc = "Register `RNG_Control_Registers__InitWaitVal` writer"]
pub type W = crate::W<RngControlRegisters_InitWaitValSpec>;
#[doc = "Field `InitWaitVal` reader - Number of clock cycles to wait before sampling data from the noise source."]
pub type InitWaitValR = crate::FieldReader<u16>;
#[doc = "Field `InitWaitVal` writer - Number of clock cycles to wait before sampling data from the noise source."]
pub type InitWaitValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of clock cycles to wait before sampling data from the noise source."]
    #[inline(always)]
    pub fn init_wait_val(&self) -> InitWaitValR {
        InitWaitValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of clock cycles to wait before sampling data from the noise source."]
    #[inline(always)]
    pub fn init_wait_val(&mut self) -> InitWaitValW<'_, RngControlRegisters_InitWaitValSpec> {
        InitWaitValW::new(self, 0)
    }
}
#[doc = "Initial wait counter value.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__init_wait_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__init_wait_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_InitWaitValSpec;
impl crate::RegisterSpec for RngControlRegisters_InitWaitValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__init_wait_val::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_InitWaitValSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__init_wait_val::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_InitWaitValSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__InitWaitVal to value 0xffff"]
impl crate::Resettable for RngControlRegisters_InitWaitValSpec {
    const RESET_VALUE: u32 = 0xffff;
}

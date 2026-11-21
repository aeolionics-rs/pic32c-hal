#[doc = "Register `RNG_Control_Registers__SwOffTmrVal` reader"]
pub type R = crate::R<RngControlRegisters_SwOffTmrValSpec>;
#[doc = "Register `RNG_Control_Registers__SwOffTmrVal` writer"]
pub type W = crate::W<RngControlRegisters_SwOffTmrValSpec>;
#[doc = "Field `SwOffTmrVal` reader - Number of clk cycles to wait before stopping the rings after the FIFO is full."]
pub type SwOffTmrValR = crate::FieldReader<u16>;
#[doc = "Field `SwOffTmrVal` writer - Number of clk cycles to wait before stopping the rings after the FIFO is full."]
pub type SwOffTmrValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of clk cycles to wait before stopping the rings after the FIFO is full."]
    #[inline(always)]
    pub fn sw_off_tmr_val(&self) -> SwOffTmrValR {
        SwOffTmrValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of clk cycles to wait before stopping the rings after the FIFO is full."]
    #[inline(always)]
    pub fn sw_off_tmr_val(&mut self) -> SwOffTmrValW<'_, RngControlRegisters_SwOffTmrValSpec> {
        SwOffTmrValW::new(self, 0)
    }
}
#[doc = "Switch off timer value.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__sw_off_tmr_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__sw_off_tmr_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_SwOffTmrValSpec;
impl crate::RegisterSpec for RngControlRegisters_SwOffTmrValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__sw_off_tmr_val::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_SwOffTmrValSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__sw_off_tmr_val::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_SwOffTmrValSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__SwOffTmrVal to value 0xffff"]
impl crate::Resettable for RngControlRegisters_SwOffTmrValSpec {
    const RESET_VALUE: u32 = 0xffff;
}

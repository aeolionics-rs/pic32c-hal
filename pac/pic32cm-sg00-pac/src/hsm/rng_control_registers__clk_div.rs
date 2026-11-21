#[doc = "Register `RNG_Control_Registers__ClkDiv` reader"]
pub type R = crate::R<RngControlRegisters_ClkDivSpec>;
#[doc = "Register `RNG_Control_Registers__ClkDiv` writer"]
pub type W = crate::W<RngControlRegisters_ClkDivSpec>;
#[doc = "Field `ClkDiv` reader - Sample clock divider. The frequency at which the outputs of the rings are sampled is given by: Fs=Fpclk/(ClkDiv+1)."]
pub type ClkDivR = crate::FieldReader;
#[doc = "Field `ClkDiv` writer - Sample clock divider. The frequency at which the outputs of the rings are sampled is given by: Fs=Fpclk/(ClkDiv+1)."]
pub type ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sample clock divider. The frequency at which the outputs of the rings are sampled is given by: Fs=Fpclk/(ClkDiv+1)."]
    #[inline(always)]
    pub fn clk_div(&self) -> ClkDivR {
        ClkDivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sample clock divider. The frequency at which the outputs of the rings are sampled is given by: Fs=Fpclk/(ClkDiv+1)."]
    #[inline(always)]
    pub fn clk_div(&mut self) -> ClkDivW<'_, RngControlRegisters_ClkDivSpec> {
        ClkDivW::new(self, 0)
    }
}
#[doc = "Sample clock divider.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__clk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__clk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_ClkDivSpec;
impl crate::RegisterSpec for RngControlRegisters_ClkDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__clk_div::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_ClkDivSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__clk_div::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_ClkDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__ClkDiv to value 0"]
impl crate::Resettable for RngControlRegisters_ClkDivSpec {}

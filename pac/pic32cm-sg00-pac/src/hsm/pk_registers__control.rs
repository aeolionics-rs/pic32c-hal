#[doc = "Register `PK_Registers__Control` writer"]
pub type W = crate::W<PkRegisters_ControlSpec>;
#[doc = "Field `Start` writer - Writing a '1' starts the processing."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ClearIRQ` writer - Writing a '1' clears the IRQ output."]
pub type ClearIrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a '1' starts the processing."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, PkRegisters_ControlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a '1' clears the IRQ output."]
    #[inline(always)]
    pub fn clear_irq(&mut self) -> ClearIrqW<'_, PkRegisters_ControlSpec> {
        ClearIrqW::new(self, 1)
    }
}
#[doc = "Command register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__control::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkRegisters_ControlSpec;
impl crate::RegisterSpec for PkRegisters_ControlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pk_registers__control::W`](W) writer structure"]
impl crate::Writable for PkRegisters_ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PK_Registers__Control to value 0"]
impl crate::Resettable for PkRegisters_ControlSpec {}

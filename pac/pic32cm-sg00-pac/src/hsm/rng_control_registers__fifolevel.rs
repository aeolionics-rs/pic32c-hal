#[doc = "Register `RNG_Control_Registers__FIFOLevel` reader"]
pub type R = crate::R<RngControlRegisters_FifolevelSpec>;
#[doc = "Register `RNG_Control_Registers__FIFOLevel` writer"]
pub type W = crate::W<RngControlRegisters_FifolevelSpec>;
#[doc = "Field `FIFOLevel` reader - Number of 32 bits words of random available in the FIFO. Any write to this register clears the FullInt flag in the Status register."]
pub type FifolevelR = crate::FieldReader<u32>;
#[doc = "Field `FIFOLevel` writer - Number of 32 bits words of random available in the FIFO. Any write to this register clears the FullInt flag in the Status register."]
pub type FifolevelW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of 32 bits words of random available in the FIFO. Any write to this register clears the FullInt flag in the Status register."]
    #[inline(always)]
    pub fn fifolevel(&self) -> FifolevelR {
        FifolevelR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of 32 bits words of random available in the FIFO. Any write to this register clears the FullInt flag in the Status register."]
    #[inline(always)]
    pub fn fifolevel(&mut self) -> FifolevelW<'_, RngControlRegisters_FifolevelSpec> {
        FifolevelW::new(self, 0)
    }
}
#[doc = "FIFO level register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__fifolevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__fifolevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_FifolevelSpec;
impl crate::RegisterSpec for RngControlRegisters_FifolevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__fifolevel::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_FifolevelSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__fifolevel::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_FifolevelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__FIFOLevel to value 0"]
impl crate::Resettable for RngControlRegisters_FifolevelSpec {}

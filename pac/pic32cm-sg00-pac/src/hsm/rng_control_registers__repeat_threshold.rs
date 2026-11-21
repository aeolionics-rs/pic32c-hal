#[doc = "Register `RNG_Control_Registers__RepeatThreshold` reader"]
pub type R = crate::R<RngControlRegisters_RepeatThresholdSpec>;
#[doc = "Register `RNG_Control_Registers__RepeatThreshold` writer"]
pub type W = crate::W<RngControlRegisters_RepeatThresholdSpec>;
#[doc = "Field `RepeatThreshold` reader - Repetition Test Count Cut-Off value."]
pub type RepeatThresholdR = crate::FieldReader;
#[doc = "Field `RepeatThreshold` writer - Repetition Test Count Cut-Off value."]
pub type RepeatThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Repetition Test Count Cut-Off value."]
    #[inline(always)]
    pub fn repeat_threshold(&self) -> RepeatThresholdR {
        RepeatThresholdR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Repetition Test Count Cut-Off value."]
    #[inline(always)]
    pub fn repeat_threshold(&mut self) -> RepeatThresholdW<'_, RngControlRegisters_RepeatThresholdSpec> {
        RepeatThresholdW::new(self, 0)
    }
}
#[doc = "Repetition Test Count Cut-Off value.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__repeat_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__repeat_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_RepeatThresholdSpec;
impl crate::RegisterSpec for RngControlRegisters_RepeatThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__repeat_threshold::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_RepeatThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__repeat_threshold::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_RepeatThresholdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__RepeatThreshold to value 0x29"]
impl crate::Resettable for RngControlRegisters_RepeatThresholdSpec {
    const RESET_VALUE: u32 = 0x29;
}

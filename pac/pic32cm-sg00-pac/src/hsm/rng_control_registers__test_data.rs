#[doc = "Register `RNG_Control_Registers__TestData` writer"]
pub type W = crate::W<RngControlRegisters_TestDataSpec>;
#[doc = "Field `TestData` writer - Test data register."]
pub type TestDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Test data register."]
    #[inline(always)]
    pub fn test_data(&mut self) -> TestDataW<'_, RngControlRegisters_TestDataSpec> {
        TestDataW::new(self, 0)
    }
}
#[doc = "Test data register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__test_data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_TestDataSpec;
impl crate::RegisterSpec for RngControlRegisters_TestDataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__test_data::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_TestDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__TestData to value 0"]
impl crate::Resettable for RngControlRegisters_TestDataSpec {}

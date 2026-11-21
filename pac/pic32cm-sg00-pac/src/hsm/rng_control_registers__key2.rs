#[doc = "Register `RNG_Control_Registers__Key2` reader"]
pub type R = crate::R<RngControlRegisters_Key2Spec>;
#[doc = "Register `RNG_Control_Registers__Key2` writer"]
pub type W = crate::W<RngControlRegisters_Key2Spec>;
#[doc = "Field `Key2` reader - Key register."]
pub type Key2R = crate::FieldReader<u32>;
#[doc = "Field `Key2` writer - Key register."]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key register."]
    #[inline(always)]
    pub fn key2(&self) -> Key2R {
        Key2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register."]
    #[inline(always)]
    pub fn key2(&mut self) -> Key2W<'_, RngControlRegisters_Key2Spec> {
        Key2W::new(self, 0)
    }
}
#[doc = "Key register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__key2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__key2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_Key2Spec;
impl crate::RegisterSpec for RngControlRegisters_Key2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__key2::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_Key2Spec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__key2::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_Key2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__Key2 to value 0"]
impl crate::Resettable for RngControlRegisters_Key2Spec {}

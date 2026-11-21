#[doc = "Register `RNG_Control_Registers__AIS31Conf1` reader"]
pub type R = crate::R<RngControlRegisters_Ais31conf1Spec>;
#[doc = "Register `RNG_Control_Registers__AIS31Conf1` writer"]
pub type W = crate::W<RngControlRegisters_Ais31conf1Spec>;
#[doc = "Field `OnlineRepThreshold` reader - Online repeat threshold."]
pub type OnlineRepThresholdR = crate::FieldReader<u16>;
#[doc = "Field `OnlineRepThreshold` writer - Online repeat threshold."]
pub type OnlineRepThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HExpectedValue` reader - Expected history value."]
pub type HexpectedValueR = crate::FieldReader<u16>;
#[doc = "Field `HExpectedValue` writer - Expected history value."]
pub type HexpectedValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Online repeat threshold."]
    #[inline(always)]
    pub fn online_rep_threshold(&self) -> OnlineRepThresholdR {
        OnlineRepThresholdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Expected history value."]
    #[inline(always)]
    pub fn hexpected_value(&self) -> HexpectedValueR {
        HexpectedValueR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Online repeat threshold."]
    #[inline(always)]
    pub fn online_rep_threshold(&mut self) -> OnlineRepThresholdW<'_, RngControlRegisters_Ais31conf1Spec> {
        OnlineRepThresholdW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Expected history value."]
    #[inline(always)]
    pub fn hexpected_value(&mut self) -> HexpectedValueW<'_, RngControlRegisters_Ais31conf1Spec> {
        HexpectedValueW::new(self, 16)
    }
}
#[doc = "AIS31 configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__ais31conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__ais31conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_Ais31conf1Spec;
impl crate::RegisterSpec for RngControlRegisters_Ais31conf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__ais31conf1::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_Ais31conf1Spec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__ais31conf1::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_Ais31conf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__AIS31Conf1 to value 0"]
impl crate::Resettable for RngControlRegisters_Ais31conf1Spec {}

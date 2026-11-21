#[doc = "Register `RNG_Control_Registers__AIS31Conf0` reader"]
pub type R = crate::R<RngControlRegisters_Ais31conf0Spec>;
#[doc = "Register `RNG_Control_Registers__AIS31Conf0` writer"]
pub type W = crate::W<RngControlRegisters_Ais31conf0Spec>;
#[doc = "Field `StartUpThreshold` reader - Start-up test threshold."]
pub type StartUpThresholdR = crate::FieldReader<u16>;
#[doc = "Field `StartUpThreshold` writer - Start-up test threshold."]
pub type StartUpThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OnlineThreshold` reader - Online threshold."]
pub type OnlineThresholdR = crate::FieldReader<u16>;
#[doc = "Field `OnlineThreshold` writer - Online threshold."]
pub type OnlineThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Start-up test threshold."]
    #[inline(always)]
    pub fn start_up_threshold(&self) -> StartUpThresholdR {
        StartUpThresholdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Online threshold."]
    #[inline(always)]
    pub fn online_threshold(&self) -> OnlineThresholdR {
        OnlineThresholdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start-up test threshold."]
    #[inline(always)]
    pub fn start_up_threshold(&mut self) -> StartUpThresholdW<'_, RngControlRegisters_Ais31conf0Spec> {
        StartUpThresholdW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Online threshold."]
    #[inline(always)]
    pub fn online_threshold(&mut self) -> OnlineThresholdW<'_, RngControlRegisters_Ais31conf0Spec> {
        OnlineThresholdW::new(self, 16)
    }
}
#[doc = "AIS31 configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__ais31conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__ais31conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_Ais31conf0Spec;
impl crate::RegisterSpec for RngControlRegisters_Ais31conf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__ais31conf0::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_Ais31conf0Spec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__ais31conf0::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_Ais31conf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__AIS31Conf0 to value 0"]
impl crate::Resettable for RngControlRegisters_Ais31conf0Spec {}

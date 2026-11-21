#[doc = "Register `RNG_Control_Registers__AIS31Status` reader"]
pub type R = crate::R<RngControlRegisters_Ais31statusSpec>;
#[doc = "Register `RNG_Control_Registers__AIS31Status` writer"]
pub type W = crate::W<RngControlRegisters_Ais31statusSpec>;
#[doc = "Field `NumPrelimAlarms` reader - Number of preliminary noise alarms since counter was last cleared."]
pub type NumPrelimAlarmsR = crate::FieldReader<u16>;
#[doc = "Field `NumPrelimAlarms` writer - Number of preliminary noise alarms since counter was last cleared."]
pub type NumPrelimAlarmsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PrelimNoiseAlarmRng` reader - Last preliminary noise alarm occurred due to history value out of range."]
pub type PrelimNoiseAlarmRngR = crate::BitReader;
#[doc = "Field `PrelimNoiseAlarmRng` writer - Last preliminary noise alarm occurred due to history value out of range."]
pub type PrelimNoiseAlarmRngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PrelimNoiseAlarmRep` reader - Last preliminary noise alarm occurred due to consecutive high X**2."]
pub type PrelimNoiseAlarmRepR = crate::BitReader;
#[doc = "Field `PrelimNoiseAlarmRep` writer - Last preliminary noise alarm occurred due to consecutive high X**2."]
pub type PrelimNoiseAlarmRepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Number of preliminary noise alarms since counter was last cleared."]
    #[inline(always)]
    pub fn num_prelim_alarms(&self) -> NumPrelimAlarmsR {
        NumPrelimAlarmsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Last preliminary noise alarm occurred due to history value out of range."]
    #[inline(always)]
    pub fn prelim_noise_alarm_rng(&self) -> PrelimNoiseAlarmRngR {
        PrelimNoiseAlarmRngR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Last preliminary noise alarm occurred due to consecutive high X**2."]
    #[inline(always)]
    pub fn prelim_noise_alarm_rep(&self) -> PrelimNoiseAlarmRepR {
        PrelimNoiseAlarmRepR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of preliminary noise alarms since counter was last cleared."]
    #[inline(always)]
    pub fn num_prelim_alarms(&mut self) -> NumPrelimAlarmsW<'_, RngControlRegisters_Ais31statusSpec> {
        NumPrelimAlarmsW::new(self, 0)
    }
    #[doc = "Bit 16 - Last preliminary noise alarm occurred due to history value out of range."]
    #[inline(always)]
    pub fn prelim_noise_alarm_rng(&mut self) -> PrelimNoiseAlarmRngW<'_, RngControlRegisters_Ais31statusSpec> {
        PrelimNoiseAlarmRngW::new(self, 16)
    }
    #[doc = "Bit 17 - Last preliminary noise alarm occurred due to consecutive high X**2."]
    #[inline(always)]
    pub fn prelim_noise_alarm_rep(&mut self) -> PrelimNoiseAlarmRepW<'_, RngControlRegisters_Ais31statusSpec> {
        PrelimNoiseAlarmRepW::new(self, 17)
    }
}
#[doc = "AIS31 status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__ais31status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__ais31status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_Ais31statusSpec;
impl crate::RegisterSpec for RngControlRegisters_Ais31statusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__ais31status::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_Ais31statusSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__ais31status::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_Ais31statusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__AIS31Status to value 0"]
impl crate::Resettable for RngControlRegisters_Ais31statusSpec {}

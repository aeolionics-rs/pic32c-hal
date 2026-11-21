#[doc = "Register `PK_Registers__Timer` reader"]
pub type R = crate::R<PkRegisters_TimerSpec>;
#[doc = "Register `PK_Registers__Timer` writer"]
pub type W = crate::W<PkRegisters_TimerSpec>;
#[doc = "Field `Timer` reader - Number of core clock cycles."]
pub type TimerR = crate::FieldReader<u32>;
#[doc = "Field `Timer` writer - Number of core clock cycles."]
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 1:31 - Number of core clock cycles."]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 1:31 - Number of core clock cycles."]
    #[inline(always)]
    pub fn timer(&mut self) -> TimerW<'_, PkRegisters_TimerSpec> {
        TimerW::new(self, 1)
    }
}
#[doc = "Timer register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkRegisters_TimerSpec;
impl crate::RegisterSpec for PkRegisters_TimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pk_registers__timer::R`](R) reader structure"]
impl crate::Readable for PkRegisters_TimerSpec {}
#[doc = "`write(|w| ..)` method takes [`pk_registers__timer::W`](W) writer structure"]
impl crate::Writable for PkRegisters_TimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PK_Registers__Timer to value 0"]
impl crate::Resettable for PkRegisters_TimerSpec {}

#[doc = "Register `OSC32KCTRL` reader"]
pub type R = crate::R<Osc32kctrlSpec>;
#[doc = "Register `OSC32KCTRL` writer"]
pub type W = crate::W<Osc32kctrlSpec>;
#[doc = "Field `ONDEMAND` reader - On-Demand Operation"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On-Demand Operation"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - On-Demand Operation"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - On-Demand Operation"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<'_, Osc32kctrlSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bit 31 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WrtlockW<'_, Osc32kctrlSpec> {
        WrtlockW::new(self, 31)
    }
}
#[doc = "32.768 kHz Internal Oscillator (OSC32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`osc32kctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc32kctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc32kctrlSpec;
impl crate::RegisterSpec for Osc32kctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc32kctrl::R`](R) reader structure"]
impl crate::Readable for Osc32kctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`osc32kctrl::W`](W) writer structure"]
impl crate::Writable for Osc32kctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSC32KCTRL to value 0x80"]
impl crate::Resettable for Osc32kctrlSpec {
    const RESET_VALUE: u32 = 0x80;
}

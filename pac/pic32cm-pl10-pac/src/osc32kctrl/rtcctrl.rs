#[doc = "Register `RTCCTRL` reader"]
pub type R = crate::R<RtcctrlSpec>;
#[doc = "Register `RTCCTRL` writer"]
pub type W = crate::W<RtcctrlSpec>;
#[doc = "RTC Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcselselect {
    #[doc = "2: 1.024 kHz from the internal 32.768 kHz oscillator"]
    Osc1k = 2,
    #[doc = "3: 32.768 kHz from the internal 32.768 kHz oscillator"]
    Osc32k = 3,
    #[doc = "4: 1.024 kHz from the external 32.768 kHz crystal oscillator"]
    Xosc1k = 4,
    #[doc = "5: 32.768 kHz from the external 32.768 kHz crystal oscillator"]
    Xosc32k = 5,
}
impl From<Rtcselselect> for u8 {
    #[inline(always)]
    fn from(variant: Rtcselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcselselect {
    type Ux = u8;
}
impl crate::IsEnum for Rtcselselect {}
#[doc = "Field `RTCSEL` reader - RTC Clock Source Selection"]
pub type RtcselR = crate::FieldReader<Rtcselselect>;
impl RtcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rtcselselect> {
        match self.bits {
            2 => Some(Rtcselselect::Osc1k),
            3 => Some(Rtcselselect::Osc32k),
            4 => Some(Rtcselselect::Xosc1k),
            5 => Some(Rtcselselect::Xosc32k),
            _ => None,
        }
    }
    #[doc = "1.024 kHz from the internal 32.768 kHz oscillator"]
    #[inline(always)]
    pub fn is_osc1k(&self) -> bool {
        *self == Rtcselselect::Osc1k
    }
    #[doc = "32.768 kHz from the internal 32.768 kHz oscillator"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == Rtcselselect::Osc32k
    }
    #[doc = "1.024 kHz from the external 32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn is_xosc1k(&self) -> bool {
        *self == Rtcselselect::Xosc1k
    }
    #[doc = "32.768 kHz from the external 32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == Rtcselselect::Xosc32k
    }
}
#[doc = "Field `RTCSEL` writer - RTC Clock Source Selection"]
pub type RtcselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rtcselselect>;
impl<'a, REG> RtcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.024 kHz from the internal 32.768 kHz oscillator"]
    #[inline(always)]
    pub fn osc1k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Osc1k)
    }
    #[doc = "32.768 kHz from the internal 32.768 kHz oscillator"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Osc32k)
    }
    #[doc = "1.024 kHz from the external 32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn xosc1k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Xosc1k)
    }
    #[doc = "32.768 kHz from the external 32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Xosc32k)
    }
}
impl R {
    #[doc = "Bits 0:2 - RTC Clock Source Selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RtcselR {
        RtcselR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RTC Clock Source Selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RtcselW<'_, RtcctrlSpec> {
        RtcselW::new(self, 0)
    }
}
#[doc = "RTC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcctrlSpec;
impl crate::RegisterSpec for RtcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcctrl::R`](R) reader structure"]
impl crate::Readable for RtcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcctrl::W`](W) writer structure"]
impl crate::Writable for RtcctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCTRL to value 0"]
impl crate::Resettable for RtcctrlSpec {}

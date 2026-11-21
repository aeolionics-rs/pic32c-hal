#[doc = "Register `OSCHFCTRL` reader"]
pub type R = crate::R<OschfctrlSpec>;
#[doc = "Register `OSCHFCTRL` writer"]
pub type W = crate::W<OschfctrlSpec>;
#[doc = "Field `ONDEMAND` reader - On-Demand Operation"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On-Demand Operation"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Automatic oscillator tune\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autotuneselect {
    #[doc = "0: Automatic oscillator frequency tune disabled"]
    Off = 0,
    #[doc = "1: Automatic oscillator frequency tune enabled"]
    On = 1,
}
impl From<Autotuneselect> for bool {
    #[inline(always)]
    fn from(variant: Autotuneselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOTUNE` reader - Automatic oscillator tune"]
pub type AutotuneR = crate::BitReader<Autotuneselect>;
impl AutotuneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autotuneselect {
        match self.bits {
            false => Autotuneselect::Off,
            true => Autotuneselect::On,
        }
    }
    #[doc = "Automatic oscillator frequency tune disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Autotuneselect::Off
    }
    #[doc = "Automatic oscillator frequency tune enabled"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Autotuneselect::On
    }
}
#[doc = "Field `AUTOTUNE` writer - Automatic oscillator tune"]
pub type AutotuneW<'a, REG> = crate::BitWriter<'a, REG, Autotuneselect>;
impl<'a, REG> AutotuneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic oscillator frequency tune disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Autotuneselect::Off)
    }
    #[doc = "Automatic oscillator frequency tune enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Autotuneselect::On)
    }
}
#[doc = "Frequency Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frqselselect {
    #[doc = "0: 1 MHz output clock"]
    _1m = 0,
    #[doc = "1: 2 MHz output clock"]
    _2m = 1,
    #[doc = "2: 3 MHz output clock"]
    _3m = 2,
    #[doc = "3: 4 MHz output clock"]
    _4m = 3,
    #[doc = "5: 8 MHz output clock"]
    _8m = 5,
    #[doc = "6: 12 MHz output clock"]
    _12m = 6,
    #[doc = "7: 16 MHz output clock"]
    _16m = 7,
    #[doc = "8: 20 MHz output clock"]
    _20m = 8,
    #[doc = "9: 24 MHz output clock"]
    _24m = 9,
}
impl From<Frqselselect> for u8 {
    #[inline(always)]
    fn from(variant: Frqselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frqselselect {
    type Ux = u8;
}
impl crate::IsEnum for Frqselselect {}
#[doc = "Field `FRQSEL` reader - Frequency Select"]
pub type FrqselR = crate::FieldReader<Frqselselect>;
impl FrqselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frqselselect> {
        match self.bits {
            0 => Some(Frqselselect::_1m),
            1 => Some(Frqselselect::_2m),
            2 => Some(Frqselselect::_3m),
            3 => Some(Frqselselect::_4m),
            5 => Some(Frqselselect::_8m),
            6 => Some(Frqselselect::_12m),
            7 => Some(Frqselselect::_16m),
            8 => Some(Frqselselect::_20m),
            9 => Some(Frqselselect::_24m),
            _ => None,
        }
    }
    #[doc = "1 MHz output clock"]
    #[inline(always)]
    pub fn is_1m(&self) -> bool {
        *self == Frqselselect::_1m
    }
    #[doc = "2 MHz output clock"]
    #[inline(always)]
    pub fn is_2m(&self) -> bool {
        *self == Frqselselect::_2m
    }
    #[doc = "3 MHz output clock"]
    #[inline(always)]
    pub fn is_3m(&self) -> bool {
        *self == Frqselselect::_3m
    }
    #[doc = "4 MHz output clock"]
    #[inline(always)]
    pub fn is_4m(&self) -> bool {
        *self == Frqselselect::_4m
    }
    #[doc = "8 MHz output clock"]
    #[inline(always)]
    pub fn is_8m(&self) -> bool {
        *self == Frqselselect::_8m
    }
    #[doc = "12 MHz output clock"]
    #[inline(always)]
    pub fn is_12m(&self) -> bool {
        *self == Frqselselect::_12m
    }
    #[doc = "16 MHz output clock"]
    #[inline(always)]
    pub fn is_16m(&self) -> bool {
        *self == Frqselselect::_16m
    }
    #[doc = "20 MHz output clock"]
    #[inline(always)]
    pub fn is_20m(&self) -> bool {
        *self == Frqselselect::_20m
    }
    #[doc = "24 MHz output clock"]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        *self == Frqselselect::_24m
    }
}
#[doc = "Field `FRQSEL` writer - Frequency Select"]
pub type FrqselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Frqselselect>;
impl<'a, REG> FrqselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 MHz output clock"]
    #[inline(always)]
    pub fn _1m(self) -> &'a mut crate::W<REG> {
        self.variant(Frqselselect::_1m)
    }
    #[doc = "2 MHz output clock"]
    #[inline(always)]
    pub fn _2m(self) -> &'a mut crate::W<REG> {
        self.variant(Frqselselect::_2m)
    }
    #[doc = "3 MHz output clock"]
    #[inline(always)]
    pub fn _3m(self) -> &'a mut crate::W<REG> {
        self.variant(Frqselselect::_3m)
    }
    #[doc = "4 MHz output clock"]
    #[inline(always)]
    pub fn _4m(self) -> &'a mut crate::W<REG> {
        self.variant(Frqselselect::_4m)
    }
    #[doc = "8 MHz output clock"]
    #[inline(always)]
    pub fn _8m(self) -> &'a mut crate::W<REG> {
        self.variant(Frqselselect::_8m)
    }
    #[doc = "12 MHz output clock"]
    #[inline(always)]
    pub fn _12m(self) -> &'a mut crate::W<REG> {
        self.variant(Frqselselect::_12m)
    }
    #[doc = "16 MHz output clock"]
    #[inline(always)]
    pub fn _16m(self) -> &'a mut crate::W<REG> {
        self.variant(Frqselselect::_16m)
    }
    #[doc = "20 MHz output clock"]
    #[inline(always)]
    pub fn _20m(self) -> &'a mut crate::W<REG> {
        self.variant(Frqselselect::_20m)
    }
    #[doc = "24 MHz output clock"]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut crate::W<REG> {
        self.variant(Frqselselect::_24m)
    }
}
impl R {
    #[doc = "Bit 7 - On-Demand Operation"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Automatic oscillator tune"]
    #[inline(always)]
    pub fn autotune(&self) -> AutotuneR {
        AutotuneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Frequency Select"]
    #[inline(always)]
    pub fn frqsel(&self) -> FrqselR {
        FrqselR::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - On-Demand Operation"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<'_, OschfctrlSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bit 8 - Automatic oscillator tune"]
    #[inline(always)]
    pub fn autotune(&mut self) -> AutotuneW<'_, OschfctrlSpec> {
        AutotuneW::new(self, 8)
    }
    #[doc = "Bits 10:13 - Frequency Select"]
    #[inline(always)]
    pub fn frqsel(&mut self) -> FrqselW<'_, OschfctrlSpec> {
        FrqselW::new(self, 10)
    }
}
#[doc = "OSCHF Control\n\nYou can [`read`](crate::Reg::read) this register and get [`oschfctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oschfctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OschfctrlSpec;
impl crate::RegisterSpec for OschfctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oschfctrl::R`](R) reader structure"]
impl crate::Readable for OschfctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`oschfctrl::W`](W) writer structure"]
impl crate::Writable for OschfctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSCHFCTRL to value 0x0c80"]
impl crate::Resettable for OschfctrlSpec {
    const RESET_VALUE: u32 = 0x0c80;
}

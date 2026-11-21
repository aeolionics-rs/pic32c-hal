#[doc = "Register `ALARM0_CLK12H_MODE` reader"]
pub type R = crate::R<Alarm0Clk12hModeSpec>;
#[doc = "Register `ALARM0_CLK12H_MODE` writer"]
pub type W = crate::W<Alarm0Clk12hModeSpec>;
#[doc = "Field `HOUR` reader - Hour"]
pub type HourR = crate::FieldReader;
#[doc = "Field `HOUR` writer - Hour"]
pub type HourW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Meridiem Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Indicatorselect {
    #[doc = "0: Before midday (AM)"]
    Am = 0,
    #[doc = "1: After midday (PM)"]
    Pm = 1,
}
impl From<Indicatorselect> for bool {
    #[inline(always)]
    fn from(variant: Indicatorselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INDICATOR` reader - Meridiem Indicator"]
pub type IndicatorR = crate::BitReader<Indicatorselect>;
impl IndicatorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Indicatorselect {
        match self.bits {
            false => Indicatorselect::Am,
            true => Indicatorselect::Pm,
        }
    }
    #[doc = "Before midday (AM)"]
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        *self == Indicatorselect::Am
    }
    #[doc = "After midday (PM)"]
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == Indicatorselect::Pm
    }
}
#[doc = "Field `INDICATOR` writer - Meridiem Indicator"]
pub type IndicatorW<'a, REG> = crate::BitWriter<'a, REG, Indicatorselect>;
impl<'a, REG> IndicatorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Before midday (AM)"]
    #[inline(always)]
    pub fn am(self) -> &'a mut crate::W<REG> {
        self.variant(Indicatorselect::Am)
    }
    #[doc = "After midday (PM)"]
    #[inline(always)]
    pub fn pm(self) -> &'a mut crate::W<REG> {
        self.variant(Indicatorselect::Pm)
    }
}
impl R {
    #[doc = "Bits 12:15 - Hour"]
    #[inline(always)]
    pub fn hour(&self) -> HourR {
        HourR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Meridiem Indicator"]
    #[inline(always)]
    pub fn indicator(&self) -> IndicatorR {
        IndicatorR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 12:15 - Hour"]
    #[inline(always)]
    pub fn hour(&mut self) -> HourW<'_, Alarm0Clk12hModeSpec> {
        HourW::new(self, 12)
    }
    #[doc = "Bit 16 - Meridiem Indicator"]
    #[inline(always)]
    pub fn indicator(&mut self) -> IndicatorW<'_, Alarm0Clk12hModeSpec> {
        IndicatorW::new(self, 16)
    }
}
#[doc = "MODE2 Alarm 0 Value\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0_clk12h_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0_clk12h_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alarm0Clk12hModeSpec;
impl crate::RegisterSpec for Alarm0Clk12hModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0_clk12h_mode::R`](R) reader structure"]
impl crate::Readable for Alarm0Clk12hModeSpec {}
#[doc = "`write(|w| ..)` method takes [`alarm0_clk12h_mode::W`](W) writer structure"]
impl crate::Writable for Alarm0Clk12hModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALARM0_CLK12H_MODE to value 0"]
impl crate::Resettable for Alarm0Clk12hModeSpec {}

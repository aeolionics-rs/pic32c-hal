#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Software Reset for this macro.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstselect {
    #[doc = "0: There is no reset operation ongoing"]
    Swrst0 = 0,
    #[doc = "1: The reset operation is ongoing"]
    Swrst1 = 1,
}
impl From<Swrstselect> for bool {
    #[inline(always)]
    fn from(variant: Swrstselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` reader - Software Reset for this macro."]
pub type SwrstR = crate::BitReader<Swrstselect>;
impl SwrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrstselect {
        match self.bits {
            false => Swrstselect::Swrst0,
            true => Swrstselect::Swrst1,
        }
    }
    #[doc = "There is no reset operation ongoing"]
    #[inline(always)]
    pub fn is_swrst_0(&self) -> bool {
        *self == Swrstselect::Swrst0
    }
    #[doc = "The reset operation is ongoing"]
    #[inline(always)]
    pub fn is_swrst_1(&self) -> bool {
        *self == Swrstselect::Swrst1
    }
}
#[doc = "Field `SWRST` writer - Software Reset for this macro."]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG, Swrstselect>;
impl<'a, REG> SwrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "There is no reset operation ongoing"]
    #[inline(always)]
    pub fn swrst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstselect::Swrst0)
    }
    #[doc = "The reset operation is ongoing"]
    #[inline(always)]
    pub fn swrst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstselect::Swrst1)
    }
}
#[doc = "PTC -- Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableselect {
    #[doc = "0: Disable module. System clock is only requested for bus transactions. GCLK is never requested."]
    Enable0 = 0,
    #[doc = "1: Enable module by allowing both the generic clock and system clock requests based on the incoming clock requests from the SIB."]
    Enable1 = 1,
}
impl From<Enableselect> for bool {
    #[inline(always)]
    fn from(variant: Enableselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - PTC -- Enable"]
pub type EnableR = crate::BitReader<Enableselect>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enableselect {
        match self.bits {
            false => Enableselect::Enable0,
            true => Enableselect::Enable1,
        }
    }
    #[doc = "Disable module. System clock is only requested for bus transactions. GCLK is never requested."]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == Enableselect::Enable0
    }
    #[doc = "Enable module by allowing both the generic clock and system clock requests based on the incoming clock requests from the SIB."]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == Enableselect::Enable1
    }
}
#[doc = "Field `ENABLE` writer - PTC -- Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enableselect>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable module. System clock is only requested for bus transactions. GCLK is never requested."]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable0)
    }
    #[doc = "Enable module by allowing both the generic clock and system clock requests based on the incoming clock requests from the SIB."]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable1)
    }
}
#[doc = "Field `PRIV` reader - Privileged Access Only -- provides filtering of privilege/non-privilege bus accesses to SFR registers"]
pub type PrivR = crate::BitReader;
#[doc = "Field `PRIV` writer - Privileged Access Only -- provides filtering of privilege/non-privilege bus accesses to SFR registers"]
pub type PrivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run In Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run In Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset for this macro."]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PTC -- Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privileged Access Only -- provides filtering of privilege/non-privilege bus accesses to SFR registers"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Run In Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset for this macro."]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - PTC -- Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Privileged Access Only -- provides filtering of privilege/non-privilege bus accesses to SFR registers"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PrivW<'_, CtrlaSpec> {
        PrivW::new(self, 2)
    }
    #[doc = "Bit 6 - Run In Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, CtrlaSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<'_, CtrlaSpec> {
        OndemandW::new(self, 7)
    }
}
#[doc = "CONTROL ENABLE REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}

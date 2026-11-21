#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset1 = Reset registers and internal state0 = No effect"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset1 = Reset registers and internal state0 = No effect"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Macro/Clock Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableselect {
    #[doc = "0: HSM clock is not requested and macros is disabled."]
    Enable0 = 0,
    #[doc = "1: HSM clock is requested and macro is enabled."]
    Enable1 = 1,
}
impl From<Enableselect> for bool {
    #[inline(always)]
    fn from(variant: Enableselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Macro/Clock Enable bit"]
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
    #[doc = "HSM clock is not requested and macros is disabled."]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == Enableselect::Enable0
    }
    #[doc = "HSM clock is requested and macro is enabled."]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == Enableselect::Enable1
    }
}
#[doc = "Field `ENABLE` writer - Macro/Clock Enable bit"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enableselect>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSM clock is not requested and macros is disabled."]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable0)
    }
    #[doc = "HSM clock is requested and macro is enabled."]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable1)
    }
}
#[doc = "Privileged Access Only - This bit affects all Wrapper and EIP registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Privselect {
    #[doc = "0: HSM registers accessible in privileged and unprivileged modes. Master Requests are NOT privileged"]
    Priv0 = 0,
    #[doc = "1: HSM registers only accessible in privileged mode. Master Requests are also privileged."]
    Priv1 = 1,
}
impl From<Privselect> for bool {
    #[inline(always)]
    fn from(variant: Privselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV` reader - Privileged Access Only - This bit affects all Wrapper and EIP registers."]
pub type PrivR = crate::BitReader<Privselect>;
impl PrivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Privselect {
        match self.bits {
            false => Privselect::Priv0,
            true => Privselect::Priv1,
        }
    }
    #[doc = "HSM registers accessible in privileged and unprivileged modes. Master Requests are NOT privileged"]
    #[inline(always)]
    pub fn is_priv_0(&self) -> bool {
        *self == Privselect::Priv0
    }
    #[doc = "HSM registers only accessible in privileged mode. Master Requests are also privileged."]
    #[inline(always)]
    pub fn is_priv_1(&self) -> bool {
        *self == Privselect::Priv1
    }
}
#[doc = "Field `PRIV` writer - Privileged Access Only - This bit affects all Wrapper and EIP registers."]
pub type PrivW<'a, REG> = crate::BitWriter<'a, REG, Privselect>;
impl<'a, REG> PrivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSM registers accessible in privileged and unprivileged modes. Master Requests are NOT privileged"]
    #[inline(always)]
    pub fn priv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Priv0)
    }
    #[doc = "HSM registers only accessible in privileged mode. Master Requests are also privileged."]
    #[inline(always)]
    pub fn priv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Priv1)
    }
}
#[doc = "Run In Standby bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Runstdbyselect {
    #[doc = "0: HSM clock is disabled in Standby Sleep mode, clock requests are de-asserted after any pending bus transactions or requests are completed."]
    Runstdby0 = 0,
    #[doc = "1: If enabled, the HSM clock remains requested in Standby Sleep mode."]
    Runstdby1 = 1,
}
impl From<Runstdbyselect> for bool {
    #[inline(always)]
    fn from(variant: Runstdbyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUNSTDBY` reader - Run In Standby bit"]
pub type RunstdbyR = crate::BitReader<Runstdbyselect>;
impl RunstdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Runstdbyselect {
        match self.bits {
            false => Runstdbyselect::Runstdby0,
            true => Runstdbyselect::Runstdby1,
        }
    }
    #[doc = "HSM clock is disabled in Standby Sleep mode, clock requests are de-asserted after any pending bus transactions or requests are completed."]
    #[inline(always)]
    pub fn is_runstdby_0(&self) -> bool {
        *self == Runstdbyselect::Runstdby0
    }
    #[doc = "If enabled, the HSM clock remains requested in Standby Sleep mode."]
    #[inline(always)]
    pub fn is_runstdby_1(&self) -> bool {
        *self == Runstdbyselect::Runstdby1
    }
}
#[doc = "Field `RUNSTDBY` writer - Run In Standby bit"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG, Runstdbyselect>;
impl<'a, REG> RunstdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSM clock is disabled in Standby Sleep mode, clock requests are de-asserted after any pending bus transactions or requests are completed."]
    #[inline(always)]
    pub fn runstdby_0(self) -> &'a mut crate::W<REG> {
        self.variant(Runstdbyselect::Runstdby0)
    }
    #[doc = "If enabled, the HSM clock remains requested in Standby Sleep mode."]
    #[inline(always)]
    pub fn runstdby_1(self) -> &'a mut crate::W<REG> {
        self.variant(Runstdbyselect::Runstdby1)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset1 = Reset registers and internal state0 = No effect"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Macro/Clock Enable bit"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privileged Access Only - This bit affects all Wrapper and EIP registers."]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Run In Standby bit"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset1 = Reset registers and internal state0 = No effect"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Macro/Clock Enable bit"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Privileged Access Only - This bit affects all Wrapper and EIP registers."]
    #[inline(always)]
    pub fn priv_(&mut self) -> PrivW<'_, CtrlaSpec> {
        PrivW::new(self, 2)
    }
    #[doc = "Bit 6 - Run In Standby bit"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, CtrlaSpec> {
        RunstdbyW::new(self, 6)
    }
}
#[doc = "Control Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

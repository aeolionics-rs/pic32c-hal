#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Software ResetNOTE: Reset to PUF must be held active long enough to reset complete macro.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstselect {
    #[doc = "0: No effect"]
    Swrst0 = 0,
    #[doc = "1: Reset registers and internal state"]
    Swrst1 = 1,
}
impl From<Swrstselect> for bool {
    #[inline(always)]
    fn from(variant: Swrstselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` reader - Software ResetNOTE: Reset to PUF must be held active long enough to reset complete macro."]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_swrst_0(&self) -> bool {
        *self == Swrstselect::Swrst0
    }
    #[doc = "Reset registers and internal state"]
    #[inline(always)]
    pub fn is_swrst_1(&self) -> bool {
        *self == Swrstselect::Swrst1
    }
}
#[doc = "Field `SWRST` writer - Software ResetNOTE: Reset to PUF must be held active long enough to reset complete macro."]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG, Swrstselect>;
impl<'a, REG> SwrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn swrst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstselect::Swrst0)
    }
    #[doc = "Reset registers and internal state"]
    #[inline(always)]
    pub fn swrst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstselect::Swrst1)
    }
}
#[doc = "Clock Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableselect {
    #[doc = "0: PUF clock is disabled and Macro held in reset and any access to QK register would cause PSLVERR."]
    Enable0 = 0,
    #[doc = "1: PUF clock is enabled."]
    Enable1 = 1,
}
impl From<Enableselect> for bool {
    #[inline(always)]
    fn from(variant: Enableselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Clock Enable bit"]
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
    #[doc = "PUF clock is disabled and Macro held in reset and any access to QK register would cause PSLVERR."]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == Enableselect::Enable0
    }
    #[doc = "PUF clock is enabled."]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == Enableselect::Enable1
    }
}
#[doc = "Field `ENABLE` writer - Clock Enable bit"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enableselect>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PUF clock is disabled and Macro held in reset and any access to QK register would cause PSLVERR."]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable0)
    }
    #[doc = "PUF clock is enabled."]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable1)
    }
}
#[doc = "Privileged Access Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Privselect {
    #[doc = "0: PUF registers accessible in privileged and unprivileged modes."]
    Priv0 = 0,
    #[doc = "1: PUF registers only accessible in privileged mode."]
    Priv1 = 1,
}
impl From<Privselect> for bool {
    #[inline(always)]
    fn from(variant: Privselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV` reader - Privileged Access Only"]
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
    #[doc = "PUF registers accessible in privileged and unprivileged modes."]
    #[inline(always)]
    pub fn is_priv_0(&self) -> bool {
        *self == Privselect::Priv0
    }
    #[doc = "PUF registers only accessible in privileged mode."]
    #[inline(always)]
    pub fn is_priv_1(&self) -> bool {
        *self == Privselect::Priv1
    }
}
#[doc = "Field `PRIV` writer - Privileged Access Only"]
pub type PrivW<'a, REG> = crate::BitWriter<'a, REG, Privselect>;
impl<'a, REG> PrivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PUF registers accessible in privileged and unprivileged modes."]
    #[inline(always)]
    pub fn priv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Priv0)
    }
    #[doc = "PUF registers only accessible in privileged mode."]
    #[inline(always)]
    pub fn priv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Priv1)
    }
}
#[doc = "Field `RUNSTDBY` reader - RUNSTDBY Enable bit - not used by EIP HWWriting this register has no affect on the EIPNote: Currently this bit is present, but has no functionality. This bit is part of the IP wrapper but has no use for the EIP."]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - RUNSTDBY Enable bit - not used by EIP HWWriting this register has no affect on the EIPNote: Currently this bit is present, but has no functionality. This bit is part of the IP wrapper but has no use for the EIP."]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software ResetNOTE: Reset to PUF must be held active long enough to reset complete macro."]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Enable bit"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privileged Access Only"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - RUNSTDBY Enable bit - not used by EIP HWWriting this register has no affect on the EIPNote: Currently this bit is present, but has no functionality. This bit is part of the IP wrapper but has no use for the EIP."]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software ResetNOTE: Reset to PUF must be held active long enough to reset complete macro."]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Enable bit"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Privileged Access Only"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PrivW<'_, CtrlaSpec> {
        PrivW::new(self, 2)
    }
    #[doc = "Bit 6 - RUNSTDBY Enable bit - not used by EIP HWWriting this register has no affect on the EIPNote: Currently this bit is present, but has no functionality. This bit is part of the IP wrapper but has no use for the EIP."]
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

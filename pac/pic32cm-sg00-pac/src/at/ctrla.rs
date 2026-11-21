#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset1 = Reset registers , Bus I/F and internal state0 = No effect"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset1 = Reset registers , Bus I/F and internal state0 = No effect"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Privileged Access Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Privselect {
    #[doc = "0: AT registers accessible in privileged and unprivileged modes."]
    Priv0 = 0,
    #[doc = "1: AT registers only accessible in privileged mode."]
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
    #[doc = "AT registers accessible in privileged and unprivileged modes."]
    #[inline(always)]
    pub fn is_priv_0(&self) -> bool {
        *self == Privselect::Priv0
    }
    #[doc = "AT registers only accessible in privileged mode."]
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
    #[doc = "AT registers accessible in privileged and unprivileged modes."]
    #[inline(always)]
    pub fn priv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Priv0)
    }
    #[doc = "AT registers only accessible in privileged mode."]
    #[inline(always)]
    pub fn priv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Priv1)
    }
}
#[doc = "Field `RUNSTDBY` reader - RUNSTDBY Enable Bit"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - RUNSTDBY Enable Bit"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset1 = Reset registers , Bus I/F and internal state0 = No effect"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Privileged Access Only"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - RUNSTDBY Enable Bit"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset1 = Reset registers , Bus I/F and internal state0 = No effect"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 2 - Privileged Access Only"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PrivW<'_, CtrlaSpec> {
        PrivW::new(self, 2)
    }
    #[doc = "Bit 6 - RUNSTDBY Enable Bit"]
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

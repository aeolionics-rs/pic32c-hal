#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset bit"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset bit"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableselect {
    #[doc = "0: Disable ROM read access and power down the ROM. The ROM PGEN input is driven high. The ROM will clamp outputs to 1 and disable all internal current sinks. Refer to the ROM compiler specification for details on powering down the ROM."]
    Enable0 = 0,
    #[doc = "1: Enable ROM read access, power up the ROM. The ROM PGEN input is driven low."]
    Enable1 = 1,
}
impl From<Enableselect> for bool {
    #[inline(always)]
    fn from(variant: Enableselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable bit"]
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
    #[doc = "Disable ROM read access and power down the ROM. The ROM PGEN input is driven high. The ROM will clamp outputs to 1 and disable all internal current sinks. Refer to the ROM compiler specification for details on powering down the ROM."]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == Enableselect::Enable0
    }
    #[doc = "Enable ROM read access, power up the ROM. The ROM PGEN input is driven low."]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == Enableselect::Enable1
    }
}
#[doc = "Field `ENABLE` writer - Enable bit"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enableselect>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ROM read access and power down the ROM. The ROM PGEN input is driven high. The ROM will clamp outputs to 1 and disable all internal current sinks. Refer to the ROM compiler specification for details on powering down the ROM."]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable0)
    }
    #[doc = "Enable ROM read access, power up the ROM. The ROM PGEN input is driven low."]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable1)
    }
}
#[doc = "Privileged Access Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Privselect {
    #[doc = "0: Macro register accessible in privileged and unprivileged modes"]
    Priv0 = 0,
    #[doc = "1: Macro registers only accessible in privileged mode"]
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
    #[doc = "Macro register accessible in privileged and unprivileged modes"]
    #[inline(always)]
    pub fn is_priv_0(&self) -> bool {
        *self == Privselect::Priv0
    }
    #[doc = "Macro registers only accessible in privileged mode"]
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
    #[doc = "Macro register accessible in privileged and unprivileged modes"]
    #[inline(always)]
    pub fn priv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Priv0)
    }
    #[doc = "Macro registers only accessible in privileged mode"]
    #[inline(always)]
    pub fn priv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Priv1)
    }
}
#[doc = "ROM Access Time Wait State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmwsselect {
    #[doc = "0: No wait states, single cycle access"]
    Prmws0 = 0,
    #[doc = "1: Access time extended 2 clock cycles"]
    Prmws1 = 1,
    #[doc = "2: Access time extended to 3 clock cycles"]
    Prmws2 = 2,
    #[doc = "7: Access time extended to 8 clock cycles"]
    Prmws7 = 7,
}
impl From<Prmwsselect> for u8 {
    #[inline(always)]
    fn from(variant: Prmwsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmwsselect {
    type Ux = u8;
}
impl crate::IsEnum for Prmwsselect {}
#[doc = "Field `PRMWS` reader - ROM Access Time Wait State"]
pub type PrmwsR = crate::FieldReader<Prmwsselect>;
impl PrmwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prmwsselect> {
        match self.bits {
            0 => Some(Prmwsselect::Prmws0),
            1 => Some(Prmwsselect::Prmws1),
            2 => Some(Prmwsselect::Prmws2),
            7 => Some(Prmwsselect::Prmws7),
            _ => None,
        }
    }
    #[doc = "No wait states, single cycle access"]
    #[inline(always)]
    pub fn is_prmws_0(&self) -> bool {
        *self == Prmwsselect::Prmws0
    }
    #[doc = "Access time extended 2 clock cycles"]
    #[inline(always)]
    pub fn is_prmws_1(&self) -> bool {
        *self == Prmwsselect::Prmws1
    }
    #[doc = "Access time extended to 3 clock cycles"]
    #[inline(always)]
    pub fn is_prmws_2(&self) -> bool {
        *self == Prmwsselect::Prmws2
    }
    #[doc = "Access time extended to 8 clock cycles"]
    #[inline(always)]
    pub fn is_prmws_7(&self) -> bool {
        *self == Prmwsselect::Prmws7
    }
}
#[doc = "Field `PRMWS` writer - ROM Access Time Wait State"]
pub type PrmwsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prmwsselect>;
impl<'a, REG> PrmwsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No wait states, single cycle access"]
    #[inline(always)]
    pub fn prmws_0(self) -> &'a mut crate::W<REG> {
        self.variant(Prmwsselect::Prmws0)
    }
    #[doc = "Access time extended 2 clock cycles"]
    #[inline(always)]
    pub fn prmws_1(self) -> &'a mut crate::W<REG> {
        self.variant(Prmwsselect::Prmws1)
    }
    #[doc = "Access time extended to 3 clock cycles"]
    #[inline(always)]
    pub fn prmws_2(self) -> &'a mut crate::W<REG> {
        self.variant(Prmwsselect::Prmws2)
    }
    #[doc = "Access time extended to 8 clock cycles"]
    #[inline(always)]
    pub fn prmws_7(self) -> &'a mut crate::W<REG> {
        self.variant(Prmwsselect::Prmws7)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable bit"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privileged Access Only"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:10 - ROM Access Time Wait State"]
    #[inline(always)]
    pub fn prmws(&self) -> PrmwsR {
        PrmwsR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset bit"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable bit"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Privileged Access Only"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PrivW<'_, CtrlaSpec> {
        PrivW::new(self, 2)
    }
    #[doc = "Bits 8:10 - ROM Access Time Wait State"]
    #[inline(always)]
    pub fn prmws(&mut self) -> PrmwsW<'_, CtrlaSpec> {
        PrmwsW::new(self, 8)
    }
}
#[doc = "Control A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRLA to value 0x02"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u32 = 0x02;
}

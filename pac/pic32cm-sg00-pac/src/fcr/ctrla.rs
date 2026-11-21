#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Privileged Access Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Privselect {
    #[doc = "0: Macro register accessible in privileged and unprivileged accesses."]
    Disable = 0,
    #[doc = "1: Macro registers only accessible in privileged accesses"]
    Enable = 1,
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
            false => Privselect::Disable,
            true => Privselect::Enable,
        }
    }
    #[doc = "Macro register accessible in privileged and unprivileged accesses."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Privselect::Disable
    }
    #[doc = "Macro registers only accessible in privileged accesses"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Privselect::Enable
    }
}
#[doc = "Field `PRIV` writer - Privileged Access Only"]
pub type PrivW<'a, REG> = crate::BitWriter<'a, REG, Privselect>;
impl<'a, REG> PrivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Macro register accessible in privileged and unprivileged accesses."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Disable)
    }
    #[doc = "Macro registers only accessible in privileged accesses"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Enable)
    }
}
#[doc = "Flash Access Time Defined in terms of AHB Clock Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fwsselect {
    #[doc = "0: Zero Wait States"]
    Fws0 = 0,
    #[doc = "1: One Wait State"]
    Fws1 = 1,
    #[doc = "14: Fourteen Wait States"]
    Fws14 = 14,
    #[doc = "15: Fifteen Wait States"]
    Fws15 = 15,
}
impl From<Fwsselect> for u8 {
    #[inline(always)]
    fn from(variant: Fwsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fwsselect {
    type Ux = u8;
}
impl crate::IsEnum for Fwsselect {}
#[doc = "Field `FWS` reader - Flash Access Time Defined in terms of AHB Clock Wait States"]
pub type FwsR = crate::FieldReader<Fwsselect>;
impl FwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fwsselect> {
        match self.bits {
            0 => Some(Fwsselect::Fws0),
            1 => Some(Fwsselect::Fws1),
            14 => Some(Fwsselect::Fws14),
            15 => Some(Fwsselect::Fws15),
            _ => None,
        }
    }
    #[doc = "Zero Wait States"]
    #[inline(always)]
    pub fn is_fws_0(&self) -> bool {
        *self == Fwsselect::Fws0
    }
    #[doc = "One Wait State"]
    #[inline(always)]
    pub fn is_fws_1(&self) -> bool {
        *self == Fwsselect::Fws1
    }
    #[doc = "Fourteen Wait States"]
    #[inline(always)]
    pub fn is_fws_14(&self) -> bool {
        *self == Fwsselect::Fws14
    }
    #[doc = "Fifteen Wait States"]
    #[inline(always)]
    pub fn is_fws_15(&self) -> bool {
        *self == Fwsselect::Fws15
    }
}
#[doc = "Field `FWS` writer - Flash Access Time Defined in terms of AHB Clock Wait States"]
pub type FwsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Fwsselect>;
impl<'a, REG> FwsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero Wait States"]
    #[inline(always)]
    pub fn fws_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fwsselect::Fws0)
    }
    #[doc = "One Wait State"]
    #[inline(always)]
    pub fn fws_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fwsselect::Fws1)
    }
    #[doc = "Fourteen Wait States"]
    #[inline(always)]
    pub fn fws_14(self) -> &'a mut crate::W<REG> {
        self.variant(Fwsselect::Fws14)
    }
    #[doc = "Fifteen Wait States"]
    #[inline(always)]
    pub fn fws_15(self) -> &'a mut crate::W<REG> {
        self.variant(Fwsselect::Fws15)
    }
}
#[doc = "Address Wait State Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adrwsselect {
    #[doc = "0: Add 0 Address Wait States - allowing for higher performance at lower clock frequencies"]
    Zero = 0,
    #[doc = "1: Add 1 Address Wait State - allowing for higher clock frequencies"]
    One = 1,
}
impl From<Adrwsselect> for bool {
    #[inline(always)]
    fn from(variant: Adrwsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRWS` reader - Address Wait State Enable"]
pub type AdrwsR = crate::BitReader<Adrwsselect>;
impl AdrwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adrwsselect {
        match self.bits {
            false => Adrwsselect::Zero,
            true => Adrwsselect::One,
        }
    }
    #[doc = "Add 0 Address Wait States - allowing for higher performance at lower clock frequencies"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Adrwsselect::Zero
    }
    #[doc = "Add 1 Address Wait State - allowing for higher clock frequencies"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Adrwsselect::One
    }
}
#[doc = "Field `ADRWS` writer - Address Wait State Enable"]
pub type AdrwsW<'a, REG> = crate::BitWriter<'a, REG, Adrwsselect>;
impl<'a, REG> AdrwsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Add 0 Address Wait States - allowing for higher performance at lower clock frequencies"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Adrwsselect::Zero)
    }
    #[doc = "Add 1 Address Wait State - allowing for higher clock frequencies"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Adrwsselect::One)
    }
}
#[doc = "Automatic Wait State Enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autowsselect {
    #[doc = "0: Use FWS: Total flash wait states are ADRWS + FWS"]
    Disable = 0,
    #[doc = "1: Use Automatic wait states: Total flash wait states are ADRWS + Taws"]
    Enable = 1,
}
impl From<Autowsselect> for bool {
    #[inline(always)]
    fn from(variant: Autowsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOWS` reader - Automatic Wait State Enable."]
pub type AutowsR = crate::BitReader<Autowsselect>;
impl AutowsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autowsselect {
        match self.bits {
            false => Autowsselect::Disable,
            true => Autowsselect::Enable,
        }
    }
    #[doc = "Use FWS: Total flash wait states are ADRWS + FWS"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Autowsselect::Disable
    }
    #[doc = "Use Automatic wait states: Total flash wait states are ADRWS + Taws"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Autowsselect::Enable
    }
}
#[doc = "Field `AUTOWS` writer - Automatic Wait State Enable."]
pub type AutowsW<'a, REG> = crate::BitWriter<'a, REG, Autowsselect>;
impl<'a, REG> AutowsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use FWS: Total flash wait states are ADRWS + FWS"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Autowsselect::Disable)
    }
    #[doc = "Use Automatic wait states: Total flash wait states are ADRWS + Taws"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Autowsselect::Enable)
    }
}
#[doc = "Read Buffer Wait StatesPer AHB port control of deterministic or zero wait state read buffer access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdbufwsselect {
    #[doc = "0: 0 Wait States"]
    Zerows = 0,
    #[doc = "1: ADRWS + FWS wait state for hits to AHB read buffer"]
    Deterministic = 1,
}
impl From<Rdbufwsselect> for bool {
    #[inline(always)]
    fn from(variant: Rdbufwsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBUFWS` reader - Read Buffer Wait StatesPer AHB port control of deterministic or zero wait state read buffer access."]
pub type RdbufwsR = crate::BitReader<Rdbufwsselect>;
impl RdbufwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdbufwsselect {
        match self.bits {
            false => Rdbufwsselect::Zerows,
            true => Rdbufwsselect::Deterministic,
        }
    }
    #[doc = "0 Wait States"]
    #[inline(always)]
    pub fn is_zerows(&self) -> bool {
        *self == Rdbufwsselect::Zerows
    }
    #[doc = "ADRWS + FWS wait state for hits to AHB read buffer"]
    #[inline(always)]
    pub fn is_deterministic(&self) -> bool {
        *self == Rdbufwsselect::Deterministic
    }
}
#[doc = "Field `RDBUFWS` writer - Read Buffer Wait StatesPer AHB port control of deterministic or zero wait state read buffer access."]
pub type RdbufwsW<'a, REG> = crate::BitWriter<'a, REG, Rdbufwsselect>;
impl<'a, REG> RdbufwsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0 Wait States"]
    #[inline(always)]
    pub fn zerows(self) -> &'a mut crate::W<REG> {
        self.variant(Rdbufwsselect::Zerows)
    }
    #[doc = "ADRWS + FWS wait state for hits to AHB read buffer"]
    #[inline(always)]
    pub fn deterministic(self) -> &'a mut crate::W<REG> {
        self.variant(Rdbufwsselect::Deterministic)
    }
}
impl R {
    #[doc = "Bit 2 - Privileged Access Only"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Flash Access Time Defined in terms of AHB Clock Wait States"]
    #[inline(always)]
    pub fn fws(&self) -> FwsR {
        FwsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Address Wait State Enable"]
    #[inline(always)]
    pub fn adrws(&self) -> AdrwsR {
        AdrwsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Automatic Wait State Enable."]
    #[inline(always)]
    pub fn autows(&self) -> AutowsR {
        AutowsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Read Buffer Wait StatesPer AHB port control of deterministic or zero wait state read buffer access."]
    #[inline(always)]
    pub fn rdbufws(&self) -> RdbufwsR {
        RdbufwsR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Privileged Access Only"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PrivW<'_, CtrlaSpec> {
        PrivW::new(self, 2)
    }
    #[doc = "Bits 8:11 - Flash Access Time Defined in terms of AHB Clock Wait States"]
    #[inline(always)]
    pub fn fws(&mut self) -> FwsW<'_, CtrlaSpec> {
        FwsW::new(self, 8)
    }
    #[doc = "Bit 14 - Address Wait State Enable"]
    #[inline(always)]
    pub fn adrws(&mut self) -> AdrwsW<'_, CtrlaSpec> {
        AdrwsW::new(self, 14)
    }
    #[doc = "Bit 15 - Automatic Wait State Enable."]
    #[inline(always)]
    pub fn autows(&mut self) -> AutowsW<'_, CtrlaSpec> {
        AutowsW::new(self, 15)
    }
    #[doc = "Bit 16 - Read Buffer Wait StatesPer AHB port control of deterministic or zero wait state read buffer access."]
    #[inline(always)]
    pub fn rdbufws(&mut self) -> RdbufwsW<'_, CtrlaSpec> {
        RdbufwsW::new(self, 16)
    }
}
#[doc = "CTRL A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRLA to value 0x8000"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u32 = 0x8000;
}

#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Set NVM Power Reduction Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prmselect {
    #[doc = "0: No Action"]
    None = 0,
    #[doc = "1: Toggle the Flash Power Mode"]
    Toggle = 1,
}
impl From<Prmselect> for bool {
    #[inline(always)]
    fn from(variant: Prmselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRM` reader - Set NVM Power Reduction Mode"]
pub type PrmR = crate::BitReader<Prmselect>;
impl PrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prmselect {
        match self.bits {
            false => Prmselect::None,
            true => Prmselect::Toggle,
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Prmselect::None
    }
    #[doc = "Toggle the Flash Power Mode"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Prmselect::Toggle
    }
}
#[doc = "Field `PRM` writer - Set NVM Power Reduction Mode"]
pub type PrmW<'a, REG> = crate::BitWriter<'a, REG, Prmselect>;
impl<'a, REG> PrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Prmselect::None)
    }
    #[doc = "Toggle the Flash Power Mode"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Prmselect::Toggle)
    }
}
#[doc = "Cache InvalidateCache invalidation takes 1 clock cycle, but may be delayed by an active read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cheinvselect {
    #[doc = "0: No Action"]
    None = 0,
    #[doc = "1: Invalidate Cache"]
    Inv = 1,
}
impl From<Cheinvselect> for bool {
    #[inline(always)]
    fn from(variant: Cheinvselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEINV` reader - Cache InvalidateCache invalidation takes 1 clock cycle, but may be delayed by an active read."]
pub type CheinvR = crate::BitReader<Cheinvselect>;
impl CheinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cheinvselect {
        match self.bits {
            false => Cheinvselect::None,
            true => Cheinvselect::Inv,
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cheinvselect::None
    }
    #[doc = "Invalidate Cache"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Cheinvselect::Inv
    }
}
#[doc = "Field `CHEINV` writer - Cache InvalidateCache invalidation takes 1 clock cycle, but may be delayed by an active read."]
pub type CheinvW<'a, REG> = crate::BitWriter<'a, REG, Cheinvselect>;
impl<'a, REG> CheinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Cheinvselect::None)
    }
    #[doc = "Invalidate Cache"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Cheinvselect::Inv)
    }
}
#[doc = "NVM Power Reduction Mode selection during System Standby Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slpselect {
    #[doc = "0: Maintain state of STATUS.PRM on entry to Standby"]
    Manual = 0,
    #[doc = "1: RSVD"]
    Rsvd01 = 1,
    #[doc = "2: Enter Flash Hibernate on entry to Standby w/ wakeup to Auto Standby"]
    Autofast = 2,
    #[doc = "3: Enter Flash Hibernate on entry to Standby w/ wakeup on first access"]
    Autoslow = 3,
}
impl From<Slpselect> for u8 {
    #[inline(always)]
    fn from(variant: Slpselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slpselect {
    type Ux = u8;
}
impl crate::IsEnum for Slpselect {}
#[doc = "Field `SLP` reader - NVM Power Reduction Mode selection during System Standby Sleep"]
pub type SlpR = crate::FieldReader<Slpselect>;
impl SlpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slpselect {
        match self.bits {
            0 => Slpselect::Manual,
            1 => Slpselect::Rsvd01,
            2 => Slpselect::Autofast,
            3 => Slpselect::Autoslow,
            _ => unreachable!(),
        }
    }
    #[doc = "Maintain state of STATUS.PRM on entry to Standby"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Slpselect::Manual
    }
    #[doc = "RSVD"]
    #[inline(always)]
    pub fn is_rsvd01(&self) -> bool {
        *self == Slpselect::Rsvd01
    }
    #[doc = "Enter Flash Hibernate on entry to Standby w/ wakeup to Auto Standby"]
    #[inline(always)]
    pub fn is_autofast(&self) -> bool {
        *self == Slpselect::Autofast
    }
    #[doc = "Enter Flash Hibernate on entry to Standby w/ wakeup on first access"]
    #[inline(always)]
    pub fn is_autoslow(&self) -> bool {
        *self == Slpselect::Autoslow
    }
}
#[doc = "Field `SLP` writer - NVM Power Reduction Mode selection during System Standby Sleep"]
pub type SlpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Slpselect, crate::Safe>;
impl<'a, REG> SlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maintain state of STATUS.PRM on entry to Standby"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Slpselect::Manual)
    }
    #[doc = "RSVD"]
    #[inline(always)]
    pub fn rsvd01(self) -> &'a mut crate::W<REG> {
        self.variant(Slpselect::Rsvd01)
    }
    #[doc = "Enter Flash Hibernate on entry to Standby w/ wakeup to Auto Standby"]
    #[inline(always)]
    pub fn autofast(self) -> &'a mut crate::W<REG> {
        self.variant(Slpselect::Autofast)
    }
    #[doc = "Enter Flash Hibernate on entry to Standby w/ wakeup on first access"]
    #[inline(always)]
    pub fn autoslow(self) -> &'a mut crate::W<REG> {
        self.variant(Slpselect::Autoslow)
    }
}
#[doc = "Cache Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chedisselect {
    #[doc = "0: Cache Enabled; Accesses to flash use AUTOWS/FSW selection."]
    Enable = 0,
    #[doc = "1: Cache Disabled; Interface uses RDBUFWS selection."]
    Disable = 1,
}
impl From<Chedisselect> for bool {
    #[inline(always)]
    fn from(variant: Chedisselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEDIS` reader - Cache Disable"]
pub type ChedisR = crate::BitReader<Chedisselect>;
impl ChedisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chedisselect {
        match self.bits {
            false => Chedisselect::Enable,
            true => Chedisselect::Disable,
        }
    }
    #[doc = "Cache Enabled; Accesses to flash use AUTOWS/FSW selection."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chedisselect::Enable
    }
    #[doc = "Cache Disabled; Interface uses RDBUFWS selection."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chedisselect::Disable
    }
}
#[doc = "Field `CHEDIS` writer - Cache Disable"]
pub type ChedisW<'a, REG> = crate::BitWriter<'a, REG, Chedisselect>;
impl<'a, REG> ChedisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cache Enabled; Accesses to flash use AUTOWS/FSW selection."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chedisselect::Enable)
    }
    #[doc = "Cache Disabled; Interface uses RDBUFWS selection."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chedisselect::Disable)
    }
}
#[doc = "Prefetch Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prefdisselect {
    #[doc = "0: Predictive Prefetch Enabled"]
    Enable = 0,
    #[doc = "1: Predictive Prefetch Disabled"]
    Disable = 1,
}
impl From<Prefdisselect> for bool {
    #[inline(always)]
    fn from(variant: Prefdisselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREFDIS` reader - Prefetch Disable"]
pub type PrefdisR = crate::BitReader<Prefdisselect>;
impl PrefdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prefdisselect {
        match self.bits {
            false => Prefdisselect::Enable,
            true => Prefdisselect::Disable,
        }
    }
    #[doc = "Predictive Prefetch Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Prefdisselect::Enable
    }
    #[doc = "Predictive Prefetch Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Prefdisselect::Disable
    }
}
#[doc = "Field `PREFDIS` writer - Prefetch Disable"]
pub type PrefdisW<'a, REG> = crate::BitWriter<'a, REG, Prefdisselect>;
impl<'a, REG> PrefdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Predictive Prefetch Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Prefdisselect::Enable)
    }
    #[doc = "Predictive Prefetch Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Prefdisselect::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - Set NVM Power Reduction Mode"]
    #[inline(always)]
    pub fn prm(&self) -> PrmR {
        PrmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Cache InvalidateCache invalidation takes 1 clock cycle, but may be delayed by an active read."]
    #[inline(always)]
    pub fn cheinv(&self) -> CheinvR {
        CheinvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - NVM Power Reduction Mode selection during System Standby Sleep"]
    #[inline(always)]
    pub fn slp(&self) -> SlpR {
        SlpR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Cache Disable"]
    #[inline(always)]
    pub fn chedis(&self) -> ChedisR {
        ChedisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Prefetch Disable"]
    #[inline(always)]
    pub fn prefdis(&self) -> PrefdisR {
        PrefdisR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set NVM Power Reduction Mode"]
    #[inline(always)]
    pub fn prm(&mut self) -> PrmW<'_, CtrlbSpec> {
        PrmW::new(self, 0)
    }
    #[doc = "Bit 7 - Cache InvalidateCache invalidation takes 1 clock cycle, but may be delayed by an active read."]
    #[inline(always)]
    pub fn cheinv(&mut self) -> CheinvW<'_, CtrlbSpec> {
        CheinvW::new(self, 7)
    }
    #[doc = "Bits 8:9 - NVM Power Reduction Mode selection during System Standby Sleep"]
    #[inline(always)]
    pub fn slp(&mut self) -> SlpW<'_, CtrlbSpec> {
        SlpW::new(self, 8)
    }
    #[doc = "Bit 16 - Cache Disable"]
    #[inline(always)]
    pub fn chedis(&mut self) -> ChedisW<'_, CtrlbSpec> {
        ChedisW::new(self, 16)
    }
    #[doc = "Bit 17 - Prefetch Disable"]
    #[inline(always)]
    pub fn prefdis(&mut self) -> PrefdisW<'_, CtrlbSpec> {
        PrefdisW::new(self, 17)
    }
}
#[doc = "CTRL B REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}

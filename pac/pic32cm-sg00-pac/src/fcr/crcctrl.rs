#[doc = "Register `CRCCTRL` reader"]
pub type R = crate::R<CrcctrlSpec>;
#[doc = "Register `CRCCTRL` writer"]
pub type W = crate::W<CrcctrlSpec>;
#[doc = "CRC Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcrstselect {
    #[doc = "0: No Action"]
    None = 0,
    #[doc = "1: Resets all CRC SFR bits."]
    Reset = 1,
}
impl From<Crcrstselect> for bool {
    #[inline(always)]
    fn from(variant: Crcrstselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCRST` reader - CRC Reset"]
pub type CrcrstR = crate::BitReader<Crcrstselect>;
impl CrcrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcrstselect {
        match self.bits {
            false => Crcrstselect::None,
            true => Crcrstselect::Reset,
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Crcrstselect::None
    }
    #[doc = "Resets all CRC SFR bits."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Crcrstselect::Reset
    }
}
#[doc = "Field `CRCRST` writer - CRC Reset"]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG, Crcrstselect>;
impl<'a, REG> CrcrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Crcrstselect::None)
    }
    #[doc = "Resets all CRC SFR bits."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Crcrstselect::Reset)
    }
}
#[doc = "Start CRC Calculation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcenselect {
    #[doc = "0: Stops CRC calculation."]
    Disable = 0,
    #[doc = "1: CRC Calculation Enabled (w/ start on write to 1)."]
    Enable = 1,
}
impl From<Crcenselect> for bool {
    #[inline(always)]
    fn from(variant: Crcenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEN` reader - Start CRC Calculation"]
pub type CrcenR = crate::BitReader<Crcenselect>;
impl CrcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcenselect {
        match self.bits {
            false => Crcenselect::Disable,
            true => Crcenselect::Enable,
        }
    }
    #[doc = "Stops CRC calculation."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Crcenselect::Disable
    }
    #[doc = "CRC Calculation Enabled (w/ start on write to 1)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Crcenselect::Enable
    }
}
#[doc = "Field `CRCEN` writer - Start CRC Calculation"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG, Crcenselect>;
impl<'a, REG> CrcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stops CRC calculation."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Crcenselect::Disable)
    }
    #[doc = "CRC Calculation Enabled (w/ start on write to 1)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Crcenselect::Enable)
    }
}
#[doc = "CRC Run in Standby\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Runstdbyselect {
    #[doc = "0: CRC Halts in Standby"]
    Halt = 0,
    #[doc = "1: CRC Runs in Standby but only if STATUS.PRM=0 (i.e. Flash is in Auto Standby)"]
    Run = 1,
}
impl From<Runstdbyselect> for bool {
    #[inline(always)]
    fn from(variant: Runstdbyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUNSTDBY` reader - CRC Run in Standby"]
pub type RunstdbyR = crate::BitReader<Runstdbyselect>;
impl RunstdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Runstdbyselect {
        match self.bits {
            false => Runstdbyselect::Halt,
            true => Runstdbyselect::Run,
        }
    }
    #[doc = "CRC Halts in Standby"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == Runstdbyselect::Halt
    }
    #[doc = "CRC Runs in Standby but only if STATUS.PRM=0 (i.e. Flash is in Auto Standby)"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Runstdbyselect::Run
    }
}
#[doc = "Field `RUNSTDBY` writer - CRC Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG, Runstdbyselect>;
impl<'a, REG> RunstdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC Halts in Standby"]
    #[inline(always)]
    pub fn halt(self) -> &'a mut crate::W<REG> {
        self.variant(Runstdbyselect::Halt)
    }
    #[doc = "CRC Runs in Standby but only if STATUS.PRM=0 (i.e. Flash is in Auto Standby)"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Runstdbyselect::Run)
    }
}
impl R {
    #[doc = "Bit 0 - CRC Reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start CRC Calculation"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - CRC Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Reset"]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CrcrstW<'_, CrcctrlSpec> {
        CrcrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Start CRC Calculation"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, CrcctrlSpec> {
        CrcenW::new(self, 1)
    }
    #[doc = "Bit 5 - CRC Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, CrcctrlSpec> {
        RunstdbyW::new(self, 5)
    }
}
#[doc = "CRC Control REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcctrlSpec;
impl crate::RegisterSpec for CrcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcctrl::R`](R) reader structure"]
impl crate::Readable for CrcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`crcctrl::W`](W) writer structure"]
impl crate::Writable for CrcctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCCTRL to value 0"]
impl crate::Resettable for CrcctrlSpec {}

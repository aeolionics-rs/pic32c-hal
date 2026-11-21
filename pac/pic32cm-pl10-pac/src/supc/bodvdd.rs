#[doc = "Register `BODVDD` reader"]
pub type R = crate::R<BodvddSpec>;
#[doc = "Register `BODVDD` writer"]
pub type W = crate::W<BodvddSpec>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "BOD Configuration in Standby Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stdbycfgselect {
    #[doc = "0: Continuous Mode"]
    Cont = 0,
    #[doc = "1: Sampling Mode"]
    Samp = 1,
}
impl From<Stdbycfgselect> for bool {
    #[inline(always)]
    fn from(variant: Stdbycfgselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDBYCFG` reader - BOD Configuration in Standby Sleep Mode"]
pub type StdbycfgR = crate::BitReader<Stdbycfgselect>;
impl StdbycfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stdbycfgselect {
        match self.bits {
            false => Stdbycfgselect::Cont,
            true => Stdbycfgselect::Samp,
        }
    }
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == Stdbycfgselect::Cont
    }
    #[doc = "Sampling Mode"]
    #[inline(always)]
    pub fn is_samp(&self) -> bool {
        *self == Stdbycfgselect::Samp
    }
}
#[doc = "Field `STDBYCFG` writer - BOD Configuration in Standby Sleep Mode"]
pub type StdbycfgW<'a, REG> = crate::BitWriter<'a, REG, Stdbycfgselect>;
impl<'a, REG> StdbycfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn cont(self) -> &'a mut crate::W<REG> {
        self.variant(Stdbycfgselect::Cont)
    }
    #[doc = "Sampling Mode"]
    #[inline(always)]
    pub fn samp(self) -> &'a mut crate::W<REG> {
        self.variant(Stdbycfgselect::Samp)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "BOD Configuration in Active Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Actcfgselect {
    #[doc = "0: Continuous Mode"]
    Cont = 0,
    #[doc = "1: Sampling Mode"]
    Samp = 1,
}
impl From<Actcfgselect> for bool {
    #[inline(always)]
    fn from(variant: Actcfgselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTCFG` reader - BOD Configuration in Active Mode"]
pub type ActcfgR = crate::BitReader<Actcfgselect>;
impl ActcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Actcfgselect {
        match self.bits {
            false => Actcfgselect::Cont,
            true => Actcfgselect::Samp,
        }
    }
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == Actcfgselect::Cont
    }
    #[doc = "Sampling Mode"]
    #[inline(always)]
    pub fn is_samp(&self) -> bool {
        *self == Actcfgselect::Samp
    }
}
#[doc = "Field `ACTCFG` writer - BOD Configuration in Active Mode"]
pub type ActcfgW<'a, REG> = crate::BitWriter<'a, REG, Actcfgselect>;
impl<'a, REG> ActcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn cont(self) -> &'a mut crate::W<REG> {
        self.variant(Actcfgselect::Cont)
    }
    #[doc = "Sampling Mode"]
    #[inline(always)]
    pub fn samp(self) -> &'a mut crate::W<REG> {
        self.variant(Actcfgselect::Samp)
    }
}
#[doc = "Sampling Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sampfreqselect {
    #[doc = "0: 128 Hz Sampling Frequency"]
    _128hz = 0,
    #[doc = "1: 32 Hz Sampling Frequency"]
    _32hz = 1,
}
impl From<Sampfreqselect> for bool {
    #[inline(always)]
    fn from(variant: Sampfreqselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPFREQ` reader - Sampling Frequency"]
pub type SampfreqR = crate::BitReader<Sampfreqselect>;
impl SampfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sampfreqselect {
        match self.bits {
            false => Sampfreqselect::_128hz,
            true => Sampfreqselect::_32hz,
        }
    }
    #[doc = "128 Hz Sampling Frequency"]
    #[inline(always)]
    pub fn is_128hz(&self) -> bool {
        *self == Sampfreqselect::_128hz
    }
    #[doc = "32 Hz Sampling Frequency"]
    #[inline(always)]
    pub fn is_32hz(&self) -> bool {
        *self == Sampfreqselect::_32hz
    }
}
#[doc = "Field `SAMPFREQ` writer - Sampling Frequency"]
pub type SampfreqW<'a, REG> = crate::BitWriter<'a, REG, Sampfreqselect>;
impl<'a, REG> SampfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "128 Hz Sampling Frequency"]
    #[inline(always)]
    pub fn _128hz(self) -> &'a mut crate::W<REG> {
        self.variant(Sampfreqselect::_128hz)
    }
    #[doc = "32 Hz Sampling Frequency"]
    #[inline(always)]
    pub fn _32hz(self) -> &'a mut crate::W<REG> {
        self.variant(Sampfreqselect::_32hz)
    }
}
#[doc = "BOD Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Levelselect {
    #[doc = "0: 1.90V"]
    Bodlevel0 = 0,
    #[doc = "1: 2.45V"]
    Bodlevel1 = 1,
    #[doc = "2: 2.70V"]
    Bodlevel2 = 2,
    #[doc = "3: 2.85V"]
    Bodlevel3 = 3,
}
impl From<Levelselect> for u8 {
    #[inline(always)]
    fn from(variant: Levelselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Levelselect {
    type Ux = u8;
}
impl crate::IsEnum for Levelselect {}
#[doc = "Field `LEVEL` reader - BOD Level"]
pub type LevelR = crate::FieldReader<Levelselect>;
impl LevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Levelselect {
        match self.bits {
            0 => Levelselect::Bodlevel0,
            1 => Levelselect::Bodlevel1,
            2 => Levelselect::Bodlevel2,
            3 => Levelselect::Bodlevel3,
            _ => unreachable!(),
        }
    }
    #[doc = "1.90V"]
    #[inline(always)]
    pub fn is_bodlevel0(&self) -> bool {
        *self == Levelselect::Bodlevel0
    }
    #[doc = "2.45V"]
    #[inline(always)]
    pub fn is_bodlevel1(&self) -> bool {
        *self == Levelselect::Bodlevel1
    }
    #[doc = "2.70V"]
    #[inline(always)]
    pub fn is_bodlevel2(&self) -> bool {
        *self == Levelselect::Bodlevel2
    }
    #[doc = "2.85V"]
    #[inline(always)]
    pub fn is_bodlevel3(&self) -> bool {
        *self == Levelselect::Bodlevel3
    }
}
#[doc = "Field `LEVEL` writer - BOD Level"]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Levelselect, crate::Safe>;
impl<'a, REG> LevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.90V"]
    #[inline(always)]
    pub fn bodlevel0(self) -> &'a mut crate::W<REG> {
        self.variant(Levelselect::Bodlevel0)
    }
    #[doc = "2.45V"]
    #[inline(always)]
    pub fn bodlevel1(self) -> &'a mut crate::W<REG> {
        self.variant(Levelselect::Bodlevel1)
    }
    #[doc = "2.70V"]
    #[inline(always)]
    pub fn bodlevel2(self) -> &'a mut crate::W<REG> {
        self.variant(Levelselect::Bodlevel2)
    }
    #[doc = "2.85V"]
    #[inline(always)]
    pub fn bodlevel3(self) -> &'a mut crate::W<REG> {
        self.variant(Levelselect::Bodlevel3)
    }
}
#[doc = "Voltage Level Monitor Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vlmlvlselect {
    #[doc = "0: VLM is disabled"]
    Off = 0,
    #[doc = "1: The VLM threshold is 5% above the BOD threshold"]
    _5above = 1,
    #[doc = "2: The VLM threshold is 15% above the BOD threshold"]
    _15above = 2,
    #[doc = "3: The VLM threshold is 25% above the BOD threshold"]
    _25above = 3,
}
impl From<Vlmlvlselect> for u8 {
    #[inline(always)]
    fn from(variant: Vlmlvlselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vlmlvlselect {
    type Ux = u8;
}
impl crate::IsEnum for Vlmlvlselect {}
#[doc = "Field `VLMLVL` reader - Voltage Level Monitor Level"]
pub type VlmlvlR = crate::FieldReader<Vlmlvlselect>;
impl VlmlvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vlmlvlselect {
        match self.bits {
            0 => Vlmlvlselect::Off,
            1 => Vlmlvlselect::_5above,
            2 => Vlmlvlselect::_15above,
            3 => Vlmlvlselect::_25above,
            _ => unreachable!(),
        }
    }
    #[doc = "VLM is disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Vlmlvlselect::Off
    }
    #[doc = "The VLM threshold is 5% above the BOD threshold"]
    #[inline(always)]
    pub fn is_5above(&self) -> bool {
        *self == Vlmlvlselect::_5above
    }
    #[doc = "The VLM threshold is 15% above the BOD threshold"]
    #[inline(always)]
    pub fn is_15above(&self) -> bool {
        *self == Vlmlvlselect::_15above
    }
    #[doc = "The VLM threshold is 25% above the BOD threshold"]
    #[inline(always)]
    pub fn is_25above(&self) -> bool {
        *self == Vlmlvlselect::_25above
    }
}
#[doc = "Field `VLMLVL` writer - Voltage Level Monitor Level"]
pub type VlmlvlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vlmlvlselect, crate::Safe>;
impl<'a, REG> VlmlvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VLM is disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Vlmlvlselect::Off)
    }
    #[doc = "The VLM threshold is 5% above the BOD threshold"]
    #[inline(always)]
    pub fn _5above(self) -> &'a mut crate::W<REG> {
        self.variant(Vlmlvlselect::_5above)
    }
    #[doc = "The VLM threshold is 15% above the BOD threshold"]
    #[inline(always)]
    pub fn _15above(self) -> &'a mut crate::W<REG> {
        self.variant(Vlmlvlselect::_15above)
    }
    #[doc = "The VLM threshold is 25% above the BOD threshold"]
    #[inline(always)]
    pub fn _25above(self) -> &'a mut crate::W<REG> {
        self.variant(Vlmlvlselect::_25above)
    }
}
#[doc = "Voltage Level Monitor Interrupt Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vlmcfgselect {
    #[doc = "0: VDD falls below the VLM threshold"]
    Falling = 0,
    #[doc = "1: VDD rises above the VLM threshold"]
    Rising = 1,
    #[doc = "2: VDD crosses the VLM threshold"]
    Both = 2,
}
impl From<Vlmcfgselect> for u8 {
    #[inline(always)]
    fn from(variant: Vlmcfgselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vlmcfgselect {
    type Ux = u8;
}
impl crate::IsEnum for Vlmcfgselect {}
#[doc = "Field `VLMCFG` reader - Voltage Level Monitor Interrupt Configuration"]
pub type VlmcfgR = crate::FieldReader<Vlmcfgselect>;
impl VlmcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vlmcfgselect> {
        match self.bits {
            0 => Some(Vlmcfgselect::Falling),
            1 => Some(Vlmcfgselect::Rising),
            2 => Some(Vlmcfgselect::Both),
            _ => None,
        }
    }
    #[doc = "VDD falls below the VLM threshold"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Vlmcfgselect::Falling
    }
    #[doc = "VDD rises above the VLM threshold"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Vlmcfgselect::Rising
    }
    #[doc = "VDD crosses the VLM threshold"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Vlmcfgselect::Both
    }
}
#[doc = "Field `VLMCFG` writer - Voltage Level Monitor Interrupt Configuration"]
pub type VlmcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vlmcfgselect>;
impl<'a, REG> VlmcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VDD falls below the VLM threshold"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Vlmcfgselect::Falling)
    }
    #[doc = "VDD rises above the VLM threshold"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Vlmcfgselect::Rising)
    }
    #[doc = "VDD crosses the VLM threshold"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Vlmcfgselect::Both)
    }
}
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - BOD Configuration in Standby Sleep Mode"]
    #[inline(always)]
    pub fn stdbycfg(&self) -> StdbycfgR {
        StdbycfgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - BOD Configuration in Active Mode"]
    #[inline(always)]
    pub fn actcfg(&self) -> ActcfgR {
        ActcfgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Sampling Frequency"]
    #[inline(always)]
    pub fn sampfreq(&self) -> SampfreqR {
        SampfreqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - BOD Level"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Voltage Level Monitor Level"]
    #[inline(always)]
    pub fn vlmlvl(&self) -> VlmlvlR {
        VlmlvlR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Voltage Level Monitor Interrupt Configuration"]
    #[inline(always)]
    pub fn vlmcfg(&self) -> VlmcfgR {
        VlmcfgR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 31 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, BodvddSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 5 - BOD Configuration in Standby Sleep Mode"]
    #[inline(always)]
    pub fn stdbycfg(&mut self) -> StdbycfgW<'_, BodvddSpec> {
        StdbycfgW::new(self, 5)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, BodvddSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 8 - BOD Configuration in Active Mode"]
    #[inline(always)]
    pub fn actcfg(&mut self) -> ActcfgW<'_, BodvddSpec> {
        ActcfgW::new(self, 8)
    }
    #[doc = "Bit 12 - Sampling Frequency"]
    #[inline(always)]
    pub fn sampfreq(&mut self) -> SampfreqW<'_, BodvddSpec> {
        SampfreqW::new(self, 12)
    }
    #[doc = "Bits 16:17 - BOD Level"]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, BodvddSpec> {
        LevelW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Voltage Level Monitor Level"]
    #[inline(always)]
    pub fn vlmlvl(&mut self) -> VlmlvlW<'_, BodvddSpec> {
        VlmlvlW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Voltage Level Monitor Interrupt Configuration"]
    #[inline(always)]
    pub fn vlmcfg(&mut self) -> VlmcfgW<'_, BodvddSpec> {
        VlmcfgW::new(self, 26)
    }
    #[doc = "Bit 31 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WrtlockW<'_, BodvddSpec> {
        WrtlockW::new(self, 31)
    }
}
#[doc = "VDD Brown-Out Detector Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bodvdd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodvdd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BodvddSpec;
impl crate::RegisterSpec for BodvddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bodvdd::R`](R) reader structure"]
impl crate::Readable for BodvddSpec {}
#[doc = "`write(|w| ..)` method takes [`bodvdd::W`](W) writer structure"]
impl crate::Writable for BodvddSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BODVDD to value 0"]
impl crate::Resettable for BodvddSpec {}

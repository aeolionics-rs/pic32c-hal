#[doc = "Register `WAVE_DUALSLOPE_MODE` reader"]
pub type R = crate::R<WaveDualslopeModeSpec>;
#[doc = "Register `WAVE_DUALSLOPE_MODE` writer"]
pub type W = crate::W<WaveDualslopeModeSpec>;
#[doc = "Channel 0 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol0select {
    #[doc = "0: Compare output is set to ~DIR when TCC counter matches CCn value"]
    Normal = 0,
    #[doc = "1: Compare output is set to DIR when TCC counter matches CCn value."]
    Inverted = 1,
}
impl From<Pol0select> for bool {
    #[inline(always)]
    fn from(variant: Pol0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL0` reader - Channel 0 Polarity"]
pub type Pol0R = crate::BitReader<Pol0select>;
impl Pol0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol0select {
        match self.bits {
            false => Pol0select::Normal,
            true => Pol0select::Inverted,
        }
    }
    #[doc = "Compare output is set to ~DIR when TCC counter matches CCn value"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Pol0select::Normal
    }
    #[doc = "Compare output is set to DIR when TCC counter matches CCn value."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Pol0select::Inverted
    }
}
#[doc = "Field `POL0` writer - Channel 0 Polarity"]
pub type Pol0W<'a, REG> = crate::BitWriter<'a, REG, Pol0select>;
impl<'a, REG> Pol0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare output is set to ~DIR when TCC counter matches CCn value"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Pol0select::Normal)
    }
    #[doc = "Compare output is set to DIR when TCC counter matches CCn value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Pol0select::Inverted)
    }
}
#[doc = "Channel 1 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol1select {
    #[doc = "0: Compare output is set to ~DIR when TCC counter matches CCn value"]
    Normal = 0,
    #[doc = "1: Compare output is set to DIR when TCC counter matches CCn value."]
    Inverted = 1,
}
impl From<Pol1select> for bool {
    #[inline(always)]
    fn from(variant: Pol1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL1` reader - Channel 1 Polarity"]
pub type Pol1R = crate::BitReader<Pol1select>;
impl Pol1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol1select {
        match self.bits {
            false => Pol1select::Normal,
            true => Pol1select::Inverted,
        }
    }
    #[doc = "Compare output is set to ~DIR when TCC counter matches CCn value"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Pol1select::Normal
    }
    #[doc = "Compare output is set to DIR when TCC counter matches CCn value."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Pol1select::Inverted
    }
}
#[doc = "Field `POL1` writer - Channel 1 Polarity"]
pub type Pol1W<'a, REG> = crate::BitWriter<'a, REG, Pol1select>;
impl<'a, REG> Pol1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare output is set to ~DIR when TCC counter matches CCn value"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Pol1select::Normal)
    }
    #[doc = "Compare output is set to DIR when TCC counter matches CCn value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Pol1select::Inverted)
    }
}
#[doc = "Channel 2 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol2select {
    #[doc = "0: Compare output is set to ~DIR when TCC counter matches CCn value"]
    Normal = 0,
    #[doc = "1: Compare output is set to DIR when TCC counter matches CCn value."]
    Inverted = 1,
}
impl From<Pol2select> for bool {
    #[inline(always)]
    fn from(variant: Pol2select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL2` reader - Channel 2 Polarity"]
pub type Pol2R = crate::BitReader<Pol2select>;
impl Pol2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol2select {
        match self.bits {
            false => Pol2select::Normal,
            true => Pol2select::Inverted,
        }
    }
    #[doc = "Compare output is set to ~DIR when TCC counter matches CCn value"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Pol2select::Normal
    }
    #[doc = "Compare output is set to DIR when TCC counter matches CCn value."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Pol2select::Inverted
    }
}
#[doc = "Field `POL2` writer - Channel 2 Polarity"]
pub type Pol2W<'a, REG> = crate::BitWriter<'a, REG, Pol2select>;
impl<'a, REG> Pol2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare output is set to ~DIR when TCC counter matches CCn value"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Pol2select::Normal)
    }
    #[doc = "Compare output is set to DIR when TCC counter matches CCn value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Pol2select::Inverted)
    }
}
#[doc = "Channel 3 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol3select {
    #[doc = "0: Compare output is set to ~DIR when TCC counter matches CCn value"]
    Normal = 0,
    #[doc = "1: Compare output is set to DIR when TCC counter matches CCn value."]
    Inverted = 1,
}
impl From<Pol3select> for bool {
    #[inline(always)]
    fn from(variant: Pol3select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL3` reader - Channel 3 Polarity"]
pub type Pol3R = crate::BitReader<Pol3select>;
impl Pol3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol3select {
        match self.bits {
            false => Pol3select::Normal,
            true => Pol3select::Inverted,
        }
    }
    #[doc = "Compare output is set to ~DIR when TCC counter matches CCn value"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Pol3select::Normal
    }
    #[doc = "Compare output is set to DIR when TCC counter matches CCn value."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Pol3select::Inverted
    }
}
#[doc = "Field `POL3` writer - Channel 3 Polarity"]
pub type Pol3W<'a, REG> = crate::BitWriter<'a, REG, Pol3select>;
impl<'a, REG> Pol3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare output is set to ~DIR when TCC counter matches CCn value"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Pol3select::Normal)
    }
    #[doc = "Compare output is set to DIR when TCC counter matches CCn value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Pol3select::Inverted)
    }
}
impl R {
    #[doc = "Bit 16 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&self) -> Pol0R {
        Pol0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> Pol1R {
        Pol1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> Pol2R {
        Pol2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&self) -> Pol3R {
        Pol3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&mut self) -> Pol0W<'_, WaveDualslopeModeSpec> {
        Pol0W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&mut self) -> Pol1W<'_, WaveDualslopeModeSpec> {
        Pol1W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&mut self) -> Pol2W<'_, WaveDualslopeModeSpec> {
        Pol2W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&mut self) -> Pol3W<'_, WaveDualslopeModeSpec> {
        Pol3W::new(self, 19)
    }
}
#[doc = "Waveform\n\nYou can [`read`](crate::Reg::read) this register and get [`wave_dualslope_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wave_dualslope_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaveDualslopeModeSpec;
impl crate::RegisterSpec for WaveDualslopeModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wave_dualslope_mode::R`](R) reader structure"]
impl crate::Readable for WaveDualslopeModeSpec {}
#[doc = "`write(|w| ..)` method takes [`wave_dualslope_mode::W`](W) writer structure"]
impl crate::Writable for WaveDualslopeModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAVE_DUALSLOPE_MODE to value 0"]
impl crate::Resettable for WaveDualslopeModeSpec {}

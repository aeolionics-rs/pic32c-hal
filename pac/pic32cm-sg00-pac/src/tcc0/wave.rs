#[doc = "Register `WAVE` reader"]
pub type R = crate::R<WaveSpec>;
#[doc = "Register `WAVE` writer"]
pub type W = crate::W<WaveSpec>;
#[doc = "Waveform Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wavegenselect {
    #[doc = "0: Normal frequency"]
    Nfrq = 0,
    #[doc = "1: Match frequency"]
    Mfrq = 1,
    #[doc = "2: Normal PWM"]
    Npwm = 2,
    #[doc = "3: Dual compare PWM"]
    Dpwm = 3,
    #[doc = "4: Dual-slope critical"]
    Dscritical = 4,
    #[doc = "5: Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    Dsbottom = 5,
    #[doc = "6: Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    Dsboth = 6,
    #[doc = "7: Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    Dstop = 7,
}
impl From<Wavegenselect> for u8 {
    #[inline(always)]
    fn from(variant: Wavegenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wavegenselect {
    type Ux = u8;
}
impl crate::IsEnum for Wavegenselect {}
#[doc = "Field `WAVEGEN` reader - Waveform Generation"]
pub type WavegenR = crate::FieldReader<Wavegenselect>;
impl WavegenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wavegenselect {
        match self.bits {
            0 => Wavegenselect::Nfrq,
            1 => Wavegenselect::Mfrq,
            2 => Wavegenselect::Npwm,
            3 => Wavegenselect::Dpwm,
            4 => Wavegenselect::Dscritical,
            5 => Wavegenselect::Dsbottom,
            6 => Wavegenselect::Dsboth,
            7 => Wavegenselect::Dstop,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == Wavegenselect::Nfrq
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == Wavegenselect::Mfrq
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == Wavegenselect::Npwm
    }
    #[doc = "Dual compare PWM"]
    #[inline(always)]
    pub fn is_dpwm(&self) -> bool {
        *self == Wavegenselect::Dpwm
    }
    #[doc = "Dual-slope critical"]
    #[inline(always)]
    pub fn is_dscritical(&self) -> bool {
        *self == Wavegenselect::Dscritical
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline(always)]
    pub fn is_dsbottom(&self) -> bool {
        *self == Wavegenselect::Dsbottom
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline(always)]
    pub fn is_dsboth(&self) -> bool {
        *self == Wavegenselect::Dsboth
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline(always)]
    pub fn is_dstop(&self) -> bool {
        *self == Wavegenselect::Dstop
    }
}
#[doc = "Field `WAVEGEN` writer - Waveform Generation"]
pub type WavegenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wavegenselect, crate::Safe>;
impl<'a, REG> WavegenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Nfrq)
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Mfrq)
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Npwm)
    }
    #[doc = "Dual compare PWM"]
    #[inline(always)]
    pub fn dpwm(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Dpwm)
    }
    #[doc = "Dual-slope critical"]
    #[inline(always)]
    pub fn dscritical(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Dscritical)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline(always)]
    pub fn dsbottom(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Dsbottom)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline(always)]
    pub fn dsboth(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Dsboth)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline(always)]
    pub fn dstop(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Dstop)
    }
}
#[doc = "Ramp Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rampselect {
    #[doc = "0: RAMP1 operation"]
    Ramp1 = 0,
    #[doc = "1: Alternative RAMP2 operation"]
    Ramp2a = 1,
    #[doc = "2: RAMP2 operation"]
    Ramp2 = 2,
    #[doc = "3: Critical RAMP2 operation"]
    Ramp2c = 3,
    #[doc = "4: Critical Swapped RAMP2 operation"]
    Ramp2cs = 4,
}
impl From<Rampselect> for u8 {
    #[inline(always)]
    fn from(variant: Rampselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rampselect {
    type Ux = u8;
}
impl crate::IsEnum for Rampselect {}
#[doc = "Field `RAMP` reader - Ramp Mode"]
pub type RampR = crate::FieldReader<Rampselect>;
impl RampR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rampselect> {
        match self.bits {
            0 => Some(Rampselect::Ramp1),
            1 => Some(Rampselect::Ramp2a),
            2 => Some(Rampselect::Ramp2),
            3 => Some(Rampselect::Ramp2c),
            4 => Some(Rampselect::Ramp2cs),
            _ => None,
        }
    }
    #[doc = "RAMP1 operation"]
    #[inline(always)]
    pub fn is_ramp1(&self) -> bool {
        *self == Rampselect::Ramp1
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline(always)]
    pub fn is_ramp2a(&self) -> bool {
        *self == Rampselect::Ramp2a
    }
    #[doc = "RAMP2 operation"]
    #[inline(always)]
    pub fn is_ramp2(&self) -> bool {
        *self == Rampselect::Ramp2
    }
    #[doc = "Critical RAMP2 operation"]
    #[inline(always)]
    pub fn is_ramp2c(&self) -> bool {
        *self == Rampselect::Ramp2c
    }
    #[doc = "Critical Swapped RAMP2 operation"]
    #[inline(always)]
    pub fn is_ramp2cs(&self) -> bool {
        *self == Rampselect::Ramp2cs
    }
}
#[doc = "Field `RAMP` writer - Ramp Mode"]
pub type RampW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rampselect>;
impl<'a, REG> RampW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RAMP1 operation"]
    #[inline(always)]
    pub fn ramp1(self) -> &'a mut crate::W<REG> {
        self.variant(Rampselect::Ramp1)
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2a(self) -> &'a mut crate::W<REG> {
        self.variant(Rampselect::Ramp2a)
    }
    #[doc = "RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2(self) -> &'a mut crate::W<REG> {
        self.variant(Rampselect::Ramp2)
    }
    #[doc = "Critical RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2c(self) -> &'a mut crate::W<REG> {
        self.variant(Rampselect::Ramp2c)
    }
    #[doc = "Critical Swapped RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2cs(self) -> &'a mut crate::W<REG> {
        self.variant(Rampselect::Ramp2cs)
    }
}
#[doc = "Field `CIPEREN` reader - Circular period Enable"]
pub type CiperenR = crate::BitReader;
#[doc = "Field `CIPEREN` writer - Circular period Enable"]
pub type CiperenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICCEN0` reader - Circular Channel 0 Enable"]
pub type Ciccen0R = crate::BitReader;
#[doc = "Field `CICCEN0` writer - Circular Channel 0 Enable"]
pub type Ciccen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICCEN1` reader - Circular Channel 1 Enable"]
pub type Ciccen1R = crate::BitReader;
#[doc = "Field `CICCEN1` writer - Circular Channel 1 Enable"]
pub type Ciccen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL0` reader - Channel 0 Polarity"]
pub type Pol0R = crate::BitReader;
#[doc = "Field `POL0` writer - Channel 0 Polarity"]
pub type Pol0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL1` reader - Channel 1 Polarity"]
pub type Pol1R = crate::BitReader;
#[doc = "Field `POL1` writer - Channel 1 Polarity"]
pub type Pol1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Waveform Generation"]
    #[inline(always)]
    pub fn wavegen(&self) -> WavegenR {
        WavegenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Ramp Mode"]
    #[inline(always)]
    pub fn ramp(&self) -> RampR {
        RampR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Circular period Enable"]
    #[inline(always)]
    pub fn ciperen(&self) -> CiperenR {
        CiperenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable"]
    #[inline(always)]
    pub fn ciccen0(&self) -> Ciccen0R {
        Ciccen0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable"]
    #[inline(always)]
    pub fn ciccen1(&self) -> Ciccen1R {
        Ciccen1R::new(((self.bits >> 9) & 1) != 0)
    }
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
}
impl W {
    #[doc = "Bits 0:2 - Waveform Generation"]
    #[inline(always)]
    pub fn wavegen(&mut self) -> WavegenW<'_, WaveSpec> {
        WavegenW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Ramp Mode"]
    #[inline(always)]
    pub fn ramp(&mut self) -> RampW<'_, WaveSpec> {
        RampW::new(self, 4)
    }
    #[doc = "Bit 7 - Circular period Enable"]
    #[inline(always)]
    pub fn ciperen(&mut self) -> CiperenW<'_, WaveSpec> {
        CiperenW::new(self, 7)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable"]
    #[inline(always)]
    pub fn ciccen0(&mut self) -> Ciccen0W<'_, WaveSpec> {
        Ciccen0W::new(self, 8)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable"]
    #[inline(always)]
    pub fn ciccen1(&mut self) -> Ciccen1W<'_, WaveSpec> {
        Ciccen1W::new(self, 9)
    }
    #[doc = "Bit 16 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&mut self) -> Pol0W<'_, WaveSpec> {
        Pol0W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&mut self) -> Pol1W<'_, WaveSpec> {
        Pol1W::new(self, 17)
    }
}
#[doc = "Waveform Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wave::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wave::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaveSpec;
impl crate::RegisterSpec for WaveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wave::R`](R) reader structure"]
impl crate::Readable for WaveSpec {}
#[doc = "`write(|w| ..)` method takes [`wave::W`](W) writer structure"]
impl crate::Writable for WaveSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAVE to value 0"]
impl crate::Resettable for WaveSpec {}

#[doc = "Register `WAVE` reader"]
pub type R = crate::R<WaveSpec>;
#[doc = "Register `WAVE` writer"]
pub type W = crate::W<WaveSpec>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wavegenselect {
    #[doc = "0: Normal Frequency Generation mode"]
    Nfrq = 0,
    #[doc = "1: Match Frequency Generation mode"]
    Mfrq = 1,
    #[doc = "2: Normal Pulse-Width Modulation mode"]
    Npwm = 2,
    #[doc = "3: Match Pulse-Width Modulation mode"]
    Mpwm = 3,
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
#[doc = "Field `WAVEGEN` reader - Waveform Generation Mode"]
pub type WavegenR = crate::FieldReader<Wavegenselect>;
impl WavegenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wavegenselect {
        match self.bits {
            0 => Wavegenselect::Nfrq,
            1 => Wavegenselect::Mfrq,
            2 => Wavegenselect::Npwm,
            3 => Wavegenselect::Mpwm,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal Frequency Generation mode"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == Wavegenselect::Nfrq
    }
    #[doc = "Match Frequency Generation mode"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == Wavegenselect::Mfrq
    }
    #[doc = "Normal Pulse-Width Modulation mode"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == Wavegenselect::Npwm
    }
    #[doc = "Match Pulse-Width Modulation mode"]
    #[inline(always)]
    pub fn is_mpwm(&self) -> bool {
        *self == Wavegenselect::Mpwm
    }
}
#[doc = "Field `WAVEGEN` writer - Waveform Generation Mode"]
pub type WavegenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wavegenselect, crate::Safe>;
impl<'a, REG> WavegenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Frequency Generation mode"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Nfrq)
    }
    #[doc = "Match Frequency Generation mode"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Mfrq)
    }
    #[doc = "Normal Pulse-Width Modulation mode"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Npwm)
    }
    #[doc = "Match Pulse-Width Modulation mode"]
    #[inline(always)]
    pub fn mpwm(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Mpwm)
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wavegen(&self) -> WavegenR {
        WavegenR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wavegen(&mut self) -> WavegenW<'_, WaveSpec> {
        WavegenW::new(self, 0)
    }
}
#[doc = "Waveform Generation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wave::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wave::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaveSpec;
impl crate::RegisterSpec for WaveSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wave::R`](R) reader structure"]
impl crate::Readable for WaveSpec {}
#[doc = "`write(|w| ..)` method takes [`wave::W`](W) writer structure"]
impl crate::Writable for WaveSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAVE to value 0"]
impl crate::Resettable for WaveSpec {}

#[doc = "Register `OSC8M` reader"]
pub type R = crate::R<Osc8mSpec>;
#[doc = "Register `OSC8M` writer"]
pub type W = crate::W<Osc8mSpec>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Oscillator Prescaler\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescselect {
    #[doc = "0: `0`"]
    Presc0 = 0,
    #[doc = "1: `1`"]
    Presc1 = 1,
    #[doc = "2: `10`"]
    Presc2 = 2,
    #[doc = "3: `11`"]
    Presc3 = 3,
}
impl From<Prescselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescselect {}
#[doc = "Field `PRESC` reader - Oscillator Prescaler"]
pub type PrescR = crate::FieldReader<Prescselect>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescselect {
        match self.bits {
            0 => Prescselect::Presc0,
            1 => Prescselect::Presc1,
            2 => Prescselect::Presc2,
            3 => Prescselect::Presc3,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_presc_0(&self) -> bool {
        *self == Prescselect::Presc0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_presc_1(&self) -> bool {
        *self == Prescselect::Presc1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_presc_2(&self) -> bool {
        *self == Prescselect::Presc2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_presc_3(&self) -> bool {
        *self == Prescselect::Presc3
    }
}
#[doc = "Field `PRESC` writer - Oscillator Prescaler"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prescselect, crate::Safe>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn presc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Prescselect::Presc0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn presc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescselect::Presc1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn presc_2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescselect::Presc2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn presc_3(self) -> &'a mut crate::W<REG> {
        self.variant(Prescselect::Presc3)
    }
}
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub type CalibR = crate::FieldReader<u16>;
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub type CalibW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Oscillator Frequency Range\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frangeselect {
    #[doc = "0: 4 to 6MHz"]
    Frange0 = 0,
    #[doc = "1: 6 to 8MHz"]
    Frange1 = 1,
    #[doc = "2: 8 to 11MHz"]
    Frange2 = 2,
    #[doc = "3: 11 to 15MHz"]
    Frange3 = 3,
}
impl From<Frangeselect> for u8 {
    #[inline(always)]
    fn from(variant: Frangeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frangeselect {
    type Ux = u8;
}
impl crate::IsEnum for Frangeselect {}
#[doc = "Field `FRANGE` reader - Oscillator Frequency Range"]
pub type FrangeR = crate::FieldReader<Frangeselect>;
impl FrangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frangeselect {
        match self.bits {
            0 => Frangeselect::Frange0,
            1 => Frangeselect::Frange1,
            2 => Frangeselect::Frange2,
            3 => Frangeselect::Frange3,
            _ => unreachable!(),
        }
    }
    #[doc = "4 to 6MHz"]
    #[inline(always)]
    pub fn is_frange_0(&self) -> bool {
        *self == Frangeselect::Frange0
    }
    #[doc = "6 to 8MHz"]
    #[inline(always)]
    pub fn is_frange_1(&self) -> bool {
        *self == Frangeselect::Frange1
    }
    #[doc = "8 to 11MHz"]
    #[inline(always)]
    pub fn is_frange_2(&self) -> bool {
        *self == Frangeselect::Frange2
    }
    #[doc = "11 to 15MHz"]
    #[inline(always)]
    pub fn is_frange_3(&self) -> bool {
        *self == Frangeselect::Frange3
    }
}
#[doc = "Field `FRANGE` writer - Oscillator Frequency Range"]
pub type FrangeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Frangeselect, crate::Safe>;
impl<'a, REG> FrangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 to 6MHz"]
    #[inline(always)]
    pub fn frange_0(self) -> &'a mut crate::W<REG> {
        self.variant(Frangeselect::Frange0)
    }
    #[doc = "6 to 8MHz"]
    #[inline(always)]
    pub fn frange_1(self) -> &'a mut crate::W<REG> {
        self.variant(Frangeselect::Frange1)
    }
    #[doc = "8 to 11MHz"]
    #[inline(always)]
    pub fn frange_2(self) -> &'a mut crate::W<REG> {
        self.variant(Frangeselect::Frange2)
    }
    #[doc = "11 to 15MHz"]
    #[inline(always)]
    pub fn frange_3(self) -> &'a mut crate::W<REG> {
        self.variant(Frangeselect::Frange3)
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Oscillator Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:27 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CalibR {
        CalibR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - Oscillator Frequency Range"]
    #[inline(always)]
    pub fn frange(&self) -> FrangeR {
        FrangeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Osc8mSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, Osc8mSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<'_, Osc8mSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Oscillator Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, Osc8mSpec> {
        PrescW::new(self, 8)
    }
    #[doc = "Bits 16:27 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&mut self) -> CalibW<'_, Osc8mSpec> {
        CalibW::new(self, 16)
    }
    #[doc = "Bits 30:31 - Oscillator Frequency Range"]
    #[inline(always)]
    pub fn frange(&mut self) -> FrangeW<'_, Osc8mSpec> {
        FrangeW::new(self, 30)
    }
}
#[doc = "8MHz Internal Oscillator (OSC8M) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`osc8m::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc8m::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc8mSpec;
impl crate::RegisterSpec for Osc8mSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc8m::R`](R) reader structure"]
impl crate::Readable for Osc8mSpec {}
#[doc = "`write(|w| ..)` method takes [`osc8m::W`](W) writer structure"]
impl crate::Writable for Osc8mSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSC8M to value 0x8707_0382"]
impl crate::Resettable for Osc8mSpec {
    const RESET_VALUE: u32 = 0x8707_0382;
}

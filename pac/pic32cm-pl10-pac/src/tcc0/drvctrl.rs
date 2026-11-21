#[doc = "Register `DRVCTRL` reader"]
pub type R = crate::R<DrvctrlSpec>;
#[doc = "Register `DRVCTRL` writer"]
pub type W = crate::W<DrvctrlSpec>;
#[doc = "Field `NRE0` reader - Non-Recoverable State 0 Output Enable"]
pub type Nre0R = crate::BitReader;
#[doc = "Field `NRE0` writer - Non-Recoverable State 0 Output Enable"]
pub type Nre0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRE1` reader - Non-Recoverable State 1 Output Enable"]
pub type Nre1R = crate::BitReader;
#[doc = "Field `NRE1` writer - Non-Recoverable State 1 Output Enable"]
pub type Nre1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRE2` reader - Non-Recoverable State 2 Output Enable"]
pub type Nre2R = crate::BitReader;
#[doc = "Field `NRE2` writer - Non-Recoverable State 2 Output Enable"]
pub type Nre2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRE3` reader - Non-Recoverable State 3 Output Enable"]
pub type Nre3R = crate::BitReader;
#[doc = "Field `NRE3` writer - Non-Recoverable State 3 Output Enable"]
pub type Nre3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRV0` reader - Non-Recoverable State 0 Output Value"]
pub type Nrv0R = crate::BitReader;
#[doc = "Field `NRV0` writer - Non-Recoverable State 0 Output Value"]
pub type Nrv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRV1` reader - Non-Recoverable State 1 Output Value"]
pub type Nrv1R = crate::BitReader;
#[doc = "Field `NRV1` writer - Non-Recoverable State 1 Output Value"]
pub type Nrv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRV2` reader - Non-Recoverable State 2 Output Value"]
pub type Nrv2R = crate::BitReader;
#[doc = "Field `NRV2` writer - Non-Recoverable State 2 Output Value"]
pub type Nrv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRV3` reader - Non-Recoverable State 3 Output Value"]
pub type Nrv3R = crate::BitReader;
#[doc = "Field `NRV3` writer - Non-Recoverable State 3 Output Value"]
pub type Nrv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEN0` reader - Waveform Output 0 Inversion"]
pub type Inven0R = crate::BitReader;
#[doc = "Field `INVEN0` writer - Waveform Output 0 Inversion"]
pub type Inven0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEN1` reader - Waveform Output 1 Inversion"]
pub type Inven1R = crate::BitReader;
#[doc = "Field `INVEN1` writer - Waveform Output 1 Inversion"]
pub type Inven1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEN2` reader - Waveform Output 2 Inversion"]
pub type Inven2R = crate::BitReader;
#[doc = "Field `INVEN2` writer - Waveform Output 2 Inversion"]
pub type Inven2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEN3` reader - Waveform Output 3 Inversion"]
pub type Inven3R = crate::BitReader;
#[doc = "Field `INVEN3` writer - Waveform Output 3 Inversion"]
pub type Inven3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTERVAL0` reader - Non-Recoverable Fault Input 0 Filter Value"]
pub type Filterval0R = crate::FieldReader;
#[doc = "Field `FILTERVAL0` writer - Non-Recoverable Fault Input 0 Filter Value"]
pub type Filterval0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FILTERVAL1` reader - Non-Recoverable Fault Input 1 Filter Value"]
pub type Filterval1R = crate::FieldReader;
#[doc = "Field `FILTERVAL1` writer - Non-Recoverable Fault Input 1 Filter Value"]
pub type Filterval1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Non-Recoverable State 0 Output Enable"]
    #[inline(always)]
    pub fn nre0(&self) -> Nre0R {
        Nre0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-Recoverable State 1 Output Enable"]
    #[inline(always)]
    pub fn nre1(&self) -> Nre1R {
        Nre1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-Recoverable State 2 Output Enable"]
    #[inline(always)]
    pub fn nre2(&self) -> Nre2R {
        Nre2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Non-Recoverable State 3 Output Enable"]
    #[inline(always)]
    pub fn nre3(&self) -> Nre3R {
        Nre3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Non-Recoverable State 0 Output Value"]
    #[inline(always)]
    pub fn nrv0(&self) -> Nrv0R {
        Nrv0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Non-Recoverable State 1 Output Value"]
    #[inline(always)]
    pub fn nrv1(&self) -> Nrv1R {
        Nrv1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non-Recoverable State 2 Output Value"]
    #[inline(always)]
    pub fn nrv2(&self) -> Nrv2R {
        Nrv2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable State 3 Output Value"]
    #[inline(always)]
    pub fn nrv3(&self) -> Nrv3R {
        Nrv3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Waveform Output 0 Inversion"]
    #[inline(always)]
    pub fn inven0(&self) -> Inven0R {
        Inven0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Waveform Output 1 Inversion"]
    #[inline(always)]
    pub fn inven1(&self) -> Inven1R {
        Inven1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Waveform Output 2 Inversion"]
    #[inline(always)]
    pub fn inven2(&self) -> Inven2R {
        Inven2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Waveform Output 3 Inversion"]
    #[inline(always)]
    pub fn inven3(&self) -> Inven3R {
        Inven3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Non-Recoverable Fault Input 0 Filter Value"]
    #[inline(always)]
    pub fn filterval0(&self) -> Filterval0R {
        Filterval0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Non-Recoverable Fault Input 1 Filter Value"]
    #[inline(always)]
    pub fn filterval1(&self) -> Filterval1R {
        Filterval1R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Recoverable State 0 Output Enable"]
    #[inline(always)]
    pub fn nre0(&mut self) -> Nre0W<'_, DrvctrlSpec> {
        Nre0W::new(self, 0)
    }
    #[doc = "Bit 1 - Non-Recoverable State 1 Output Enable"]
    #[inline(always)]
    pub fn nre1(&mut self) -> Nre1W<'_, DrvctrlSpec> {
        Nre1W::new(self, 1)
    }
    #[doc = "Bit 2 - Non-Recoverable State 2 Output Enable"]
    #[inline(always)]
    pub fn nre2(&mut self) -> Nre2W<'_, DrvctrlSpec> {
        Nre2W::new(self, 2)
    }
    #[doc = "Bit 3 - Non-Recoverable State 3 Output Enable"]
    #[inline(always)]
    pub fn nre3(&mut self) -> Nre3W<'_, DrvctrlSpec> {
        Nre3W::new(self, 3)
    }
    #[doc = "Bit 8 - Non-Recoverable State 0 Output Value"]
    #[inline(always)]
    pub fn nrv0(&mut self) -> Nrv0W<'_, DrvctrlSpec> {
        Nrv0W::new(self, 8)
    }
    #[doc = "Bit 9 - Non-Recoverable State 1 Output Value"]
    #[inline(always)]
    pub fn nrv1(&mut self) -> Nrv1W<'_, DrvctrlSpec> {
        Nrv1W::new(self, 9)
    }
    #[doc = "Bit 10 - Non-Recoverable State 2 Output Value"]
    #[inline(always)]
    pub fn nrv2(&mut self) -> Nrv2W<'_, DrvctrlSpec> {
        Nrv2W::new(self, 10)
    }
    #[doc = "Bit 11 - Non-Recoverable State 3 Output Value"]
    #[inline(always)]
    pub fn nrv3(&mut self) -> Nrv3W<'_, DrvctrlSpec> {
        Nrv3W::new(self, 11)
    }
    #[doc = "Bit 16 - Waveform Output 0 Inversion"]
    #[inline(always)]
    pub fn inven0(&mut self) -> Inven0W<'_, DrvctrlSpec> {
        Inven0W::new(self, 16)
    }
    #[doc = "Bit 17 - Waveform Output 1 Inversion"]
    #[inline(always)]
    pub fn inven1(&mut self) -> Inven1W<'_, DrvctrlSpec> {
        Inven1W::new(self, 17)
    }
    #[doc = "Bit 18 - Waveform Output 2 Inversion"]
    #[inline(always)]
    pub fn inven2(&mut self) -> Inven2W<'_, DrvctrlSpec> {
        Inven2W::new(self, 18)
    }
    #[doc = "Bit 19 - Waveform Output 3 Inversion"]
    #[inline(always)]
    pub fn inven3(&mut self) -> Inven3W<'_, DrvctrlSpec> {
        Inven3W::new(self, 19)
    }
    #[doc = "Bits 24:27 - Non-Recoverable Fault Input 0 Filter Value"]
    #[inline(always)]
    pub fn filterval0(&mut self) -> Filterval0W<'_, DrvctrlSpec> {
        Filterval0W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Non-Recoverable Fault Input 1 Filter Value"]
    #[inline(always)]
    pub fn filterval1(&mut self) -> Filterval1W<'_, DrvctrlSpec> {
        Filterval1W::new(self, 28)
    }
}
#[doc = "Driver Control\n\nYou can [`read`](crate::Reg::read) this register and get [`drvctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drvctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrvctrlSpec;
impl crate::RegisterSpec for DrvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drvctrl::R`](R) reader structure"]
impl crate::Readable for DrvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`drvctrl::W`](W) writer structure"]
impl crate::Writable for DrvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DRVCTRL to value 0"]
impl crate::Resettable for DrvctrlSpec {}

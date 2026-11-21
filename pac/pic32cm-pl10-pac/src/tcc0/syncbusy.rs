#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - SWRST Synchronization Busy"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - ENABLE Synchronization Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `CTRLB` reader - CTRLB Synchronization Busy"]
pub type CtrlbR = crate::BitReader;
#[doc = "Field `STATUS` reader - STATUS Synchronization Busy"]
pub type StatusR = crate::BitReader;
#[doc = "Field `COUNT` reader - COUNT Synchronization Busy"]
pub type CountR = crate::BitReader;
#[doc = "Field `PATT` reader - PATT Synchronization Busy"]
pub type PattR = crate::BitReader;
#[doc = "Field `WAVE` reader - WAVE Synchronization Busy"]
pub type WaveR = crate::BitReader;
#[doc = "Field `PER` reader - PER Synchronization Busy"]
pub type PerR = crate::BitReader;
#[doc = "Field `CC0` reader - Compare/Capture Channel 0 Synchronization Busy"]
pub type Cc0R = crate::BitReader;
#[doc = "Field `CC1` reader - Compare/Capture Channel 1 Synchronization Busy"]
pub type Cc1R = crate::BitReader;
#[doc = "Field `CC2` reader - Compare/Capture Channel 2 Synchronization Busy"]
pub type Cc2R = crate::BitReader;
#[doc = "Field `CC3` reader - Compare/Capture Channel 3 Synchronization Busy"]
pub type Cc3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SWRST Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTRLB Synchronization Busy"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CtrlbR {
        CtrlbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STATUS Synchronization Busy"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - COUNT Synchronization Busy"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PATT Synchronization Busy"]
    #[inline(always)]
    pub fn patt(&self) -> PattR {
        PattR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WAVE Synchronization Busy"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PER Synchronization Busy"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare/Capture Channel 0 Synchronization Busy"]
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare/Capture Channel 1 Synchronization Busy"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Compare/Capture Channel 2 Synchronization Busy"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Compare/Capture Channel 3 Synchronization Busy"]
    #[inline(always)]
    pub fn cc3(&self) -> Cc3R {
        Cc3R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}

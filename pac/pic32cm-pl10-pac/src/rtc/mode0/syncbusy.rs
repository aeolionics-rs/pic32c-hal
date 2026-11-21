#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software Reset Synchronization Busy"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - Enable Synchronization Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `FREQCORR` reader - Frequency Correction Synchronization Busy"]
pub type FreqcorrR = crate::BitReader;
#[doc = "Field `COUNT` reader - Counter Value Synchronization Busy"]
pub type CountR = crate::BitReader;
#[doc = "Field `COMP0` reader - Compare 0 Value Synchronization Busy"]
pub type Comp0R = crate::BitReader;
#[doc = "Field `COUNTSYNC` reader - COUNT Read Resynchronization Enable Synchronization Busy"]
pub type CountsyncR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frequency Correction Synchronization Busy"]
    #[inline(always)]
    pub fn freqcorr(&self) -> FreqcorrR {
        FreqcorrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter Value Synchronization Busy"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare 0 Value Synchronization Busy"]
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - COUNT Read Resynchronization Enable Synchronization Busy"]
    #[inline(always)]
    pub fn countsync(&self) -> CountsyncR {
        CountsyncR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "MODE0 Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}

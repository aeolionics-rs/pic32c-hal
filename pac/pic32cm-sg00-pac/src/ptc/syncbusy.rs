#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - SWRST Synchronization Busy --- Synchronizing Busy bit for swrst"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - ENABLE Synchronization Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `CTRLB` reader - Control B Synchronization Busy"]
pub type CtrlbR = crate::BitReader;
#[doc = "Field `CTRLC` reader - Control C Synchronization Busy"]
pub type CtrlcR = crate::BitReader;
#[doc = "Field `CTRLD` reader - Control D Synchronization Busy"]
pub type CtrldR = crate::BitReader;
#[doc = "Field `SWTRIG` reader - Software Trigger Synchronization Busy"]
pub type SwtrigR = crate::BitReader;
#[doc = "Field `WINLT` reader - Window Comparator Low Threshold Synchronization Busy"]
pub type WinltR = crate::BitReader;
#[doc = "Field `WINHT` reader - Window Comparator High Threshold Synchronization Busy"]
pub type WinhtR = crate::BitReader;
#[doc = "Field `XSEL1` reader - X-lines Selection 1 Synchronization Busy"]
pub type Xsel1R = crate::BitReader;
#[doc = "Field `XSEL2` reader - X-lines Selection 2 Synchronization Busy"]
pub type Xsel2R = crate::BitReader;
#[doc = "Field `YSEL1` reader - Y-lines Selection 1 Synchronization Busy"]
pub type Ysel1R = crate::BitReader;
#[doc = "Field `YSEL2` reader - Y-lines Selection 2 Synchronization Busy"]
pub type Ysel2R = crate::BitReader;
#[doc = "Field `POLCTRL1` reader - Polarity Control 1 Synchronization Busy"]
pub type Polctrl1R = crate::BitReader;
#[doc = "Field `POLCTRL2` reader - Polarity Control 2 Synchronization Busy"]
pub type Polctrl2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SWRST Synchronization Busy --- Synchronizing Busy bit for swrst"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control B Synchronization Busy"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CtrlbR {
        CtrlbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control C Synchronization Busy"]
    #[inline(always)]
    pub fn ctrlc(&self) -> CtrlcR {
        CtrlcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control D Synchronization Busy"]
    #[inline(always)]
    pub fn ctrld(&self) -> CtrldR {
        CtrldR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software Trigger Synchronization Busy"]
    #[inline(always)]
    pub fn swtrig(&self) -> SwtrigR {
        SwtrigR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Window Comparator Low Threshold Synchronization Busy"]
    #[inline(always)]
    pub fn winlt(&self) -> WinltR {
        WinltR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Window Comparator High Threshold Synchronization Busy"]
    #[inline(always)]
    pub fn winht(&self) -> WinhtR {
        WinhtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - X-lines Selection 1 Synchronization Busy"]
    #[inline(always)]
    pub fn xsel1(&self) -> Xsel1R {
        Xsel1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - X-lines Selection 2 Synchronization Busy"]
    #[inline(always)]
    pub fn xsel2(&self) -> Xsel2R {
        Xsel2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Y-lines Selection 1 Synchronization Busy"]
    #[inline(always)]
    pub fn ysel1(&self) -> Ysel1R {
        Ysel1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Y-lines Selection 2 Synchronization Busy"]
    #[inline(always)]
    pub fn ysel2(&self) -> Ysel2R {
        Ysel2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Polarity Control 1 Synchronization Busy"]
    #[inline(always)]
    pub fn polctrl1(&self) -> Polctrl1R {
        Polctrl1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Polarity Control 2 Synchronization Busy"]
    #[inline(always)]
    pub fn polctrl2(&self) -> Polctrl2R {
        Polctrl2R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Synchronization Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}

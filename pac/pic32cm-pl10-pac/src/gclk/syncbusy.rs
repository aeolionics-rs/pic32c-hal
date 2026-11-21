#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `GENCTRL0` reader - Generator Control 0 Synchronization Busy"]
pub type Genctrl0R = crate::BitReader;
#[doc = "Field `GENCTRL1` reader - Generator Control 1 Synchronization Busy"]
pub type Genctrl1R = crate::BitReader;
#[doc = "Field `GENCTRL2` reader - Generator Control 2 Synchronization Busy"]
pub type Genctrl2R = crate::BitReader;
#[doc = "Field `GENCTRL3` reader - Generator Control 3 Synchronization Busy"]
pub type Genctrl3R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Generator Control 0 Synchronization Busy"]
    #[inline(always)]
    pub fn genctrl0(&self) -> Genctrl0R {
        Genctrl0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generator Control 1 Synchronization Busy"]
    #[inline(always)]
    pub fn genctrl1(&self) -> Genctrl1R {
        Genctrl1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generator Control 2 Synchronization Busy"]
    #[inline(always)]
    pub fn genctrl2(&self) -> Genctrl2R {
        Genctrl2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generator Control 3 Synchronization Busy"]
    #[inline(always)]
    pub fn genctrl3(&self) -> Genctrl3R {
        Genctrl3R::new(((self.bits >> 5) & 1) != 0)
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

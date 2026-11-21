#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software Reset Synchronization Busy1 = SWRST synchronization is busy0 = SWRST synchronization is NOT busy"]
pub type SwrstR = crate::BitReader;
#[doc = "Macro Enable Synchronization Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableselect {
    #[doc = "0: ENABLE synchronization is NOT busy."]
    Enable0 = 0,
    #[doc = "1: ENABLE synchronization is busy."]
    Enable1 = 1,
}
impl From<Enableselect> for bool {
    #[inline(always)]
    fn from(variant: Enableselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Macro Enable Synchronization Busy"]
pub type EnableR = crate::BitReader<Enableselect>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enableselect {
        match self.bits {
            false => Enableselect::Enable0,
            true => Enableselect::Enable1,
        }
    }
    #[doc = "ENABLE synchronization is NOT busy."]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == Enableselect::Enable0
    }
    #[doc = "ENABLE synchronization is busy."]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == Enableselect::Enable1
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Busy1 = SWRST synchronization is busy0 = SWRST synchronization is NOT busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Macro Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYNCBUSY Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}

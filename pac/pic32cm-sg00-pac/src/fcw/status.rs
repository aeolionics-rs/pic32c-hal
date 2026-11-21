#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "NVM Busy Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busyselect {
    #[doc = "0: NVM Not Busy"]
    Idle = 0,
    #[doc = "1: NVM Busy - All SFR bits are not writable"]
    Busy = 1,
}
impl From<Busyselect> for bool {
    #[inline(always)]
    fn from(variant: Busyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - NVM Busy Status"]
pub type BusyR = crate::BitReader<Busyselect>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busyselect {
        match self.bits {
            false => Busyselect::Idle,
            true => Busyselect::Busy,
        }
    }
    #[doc = "NVM Not Busy"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Busyselect::Idle
    }
    #[doc = "NVM Busy - All SFR bits are not writable"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busyselect::Busy
    }
}
#[doc = "High Temp Detect Ready Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htdrdyselect {
    #[doc = "0: HTD Not Ready"]
    Notready = 0,
    #[doc = "1: HTD Ready"]
    Ready = 1,
}
impl From<Htdrdyselect> for bool {
    #[inline(always)]
    fn from(variant: Htdrdyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTDRDY` reader - High Temp Detect Ready Status"]
pub type HtdrdyR = crate::BitReader<Htdrdyselect>;
impl HtdrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Htdrdyselect {
        match self.bits {
            false => Htdrdyselect::Notready,
            true => Htdrdyselect::Ready,
        }
    }
    #[doc = "HTD Not Ready"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == Htdrdyselect::Notready
    }
    #[doc = "HTD Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Htdrdyselect::Ready
    }
}
impl R {
    #[doc = "Bit 0 - NVM Busy Status"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - High Temp Detect Ready Status"]
    #[inline(always)]
    pub fn htdrdy(&self) -> HtdrdyR {
        HtdrdyR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Status REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}

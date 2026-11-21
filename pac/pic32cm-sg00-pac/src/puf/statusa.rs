#[doc = "Register `STATUSA` reader"]
pub type R = crate::R<StatusaSpec>;
#[doc = "PUF Busy indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busyselect {
    #[doc = "0: Macro can accept commands."]
    Busy0 = 0,
    #[doc = "1: Macro has not completed the power up or power down sequence and cannot accept any other commands."]
    Busy1 = 1,
}
impl From<Busyselect> for bool {
    #[inline(always)]
    fn from(variant: Busyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - PUF Busy indicator"]
pub type BusyR = crate::BitReader<Busyselect>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busyselect {
        match self.bits {
            false => Busyselect::Busy0,
            true => Busyselect::Busy1,
        }
    }
    #[doc = "Macro can accept commands."]
    #[inline(always)]
    pub fn is_busy_0(&self) -> bool {
        *self == Busyselect::Busy0
    }
    #[doc = "Macro has not completed the power up or power down sequence and cannot accept any other commands."]
    #[inline(always)]
    pub fn is_busy_1(&self) -> bool {
        *self == Busyselect::Busy1
    }
}
impl R {
    #[doc = "Bit 0 - PUF Busy indicator"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusaSpec;
impl crate::RegisterSpec for StatusaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusa::R`](R) reader structure"]
impl crate::Readable for StatusaSpec {}
#[doc = "`reset()` method sets STATUSA to value 0"]
impl crate::Resettable for StatusaSpec {}

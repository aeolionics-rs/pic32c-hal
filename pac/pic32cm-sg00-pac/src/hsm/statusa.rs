#[doc = "Register `STATUSA` reader"]
pub type R = crate::R<StatusaSpec>;
#[doc = "OTP Lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Otplockselect {
    #[doc = "1: OTP lock bit set. Device not in IC manufacturing."]
    Otplock1 = 1,
}
impl From<Otplockselect> for bool {
    #[inline(always)]
    fn from(variant: Otplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTPLOCK` reader - OTP Lock bit"]
pub type OtplockR = crate::BitReader<Otplockselect>;
impl OtplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Otplockselect> {
        match self.bits {
            true => Some(Otplockselect::Otplock1),
            _ => None,
        }
    }
    #[doc = "OTP lock bit set. Device not in IC manufacturing."]
    #[inline(always)]
    pub fn is_otplock_1(&self) -> bool {
        *self == Otplockselect::Otplock1
    }
}
impl R {
    #[doc = "Bit 0 - OTP Lock bit"]
    #[inline(always)]
    pub fn otplock(&self) -> OtplockR {
        OtplockR::new((self.bits & 1) != 0)
    }
}
#[doc = "STATUS Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusaSpec;
impl crate::RegisterSpec for StatusaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusa::R`](R) reader structure"]
impl crate::Readable for StatusaSpec {}
#[doc = "`reset()` method sets STATUSA to value 0"]
impl crate::Resettable for StatusaSpec {}

#[doc = "Register `ENTRY2` reader"]
pub type R = crate::R<Entry2Spec>;
#[doc = "CoreSight Entry Present\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epresselect {
    #[doc = "0: Entry not present"]
    Epres0 = 0,
    #[doc = "1: Entry present"]
    Epres1 = 1,
}
impl From<Epresselect> for bool {
    #[inline(always)]
    fn from(variant: Epresselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPRES` reader - CoreSight Entry Present"]
pub type EpresR = crate::BitReader<Epresselect>;
impl EpresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epresselect {
        match self.bits {
            false => Epresselect::Epres0,
            true => Epresselect::Epres1,
        }
    }
    #[doc = "Entry not present"]
    #[inline(always)]
    pub fn is_epres_0(&self) -> bool {
        *self == Epresselect::Epres0
    }
    #[doc = "Entry present"]
    #[inline(always)]
    pub fn is_epres_1(&self) -> bool {
        *self == Epresselect::Epres1
    }
}
#[doc = "CoreSight Rom Table Format\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmtselect {
    #[doc = "0: 8-bit format"]
    Fmt0 = 0,
    #[doc = "1: 32-bit format"]
    Fmt1 = 1,
}
impl From<Fmtselect> for bool {
    #[inline(always)]
    fn from(variant: Fmtselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMT` reader - CoreSight Rom Table Format"]
pub type FmtR = crate::BitReader<Fmtselect>;
impl FmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmtselect {
        match self.bits {
            false => Fmtselect::Fmt0,
            true => Fmtselect::Fmt1,
        }
    }
    #[doc = "8-bit format"]
    #[inline(always)]
    pub fn is_fmt_0(&self) -> bool {
        *self == Fmtselect::Fmt0
    }
    #[doc = "32-bit format"]
    #[inline(always)]
    pub fn is_fmt_1(&self) -> bool {
        *self == Fmtselect::Fmt1
    }
}
#[doc = "Field `ADDOFF` reader - CoreSight ROM Table Address Offset"]
pub type AddoffR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - CoreSight Entry Present"]
    #[inline(always)]
    pub fn epres(&self) -> EpresR {
        EpresR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CoreSight Rom Table Format"]
    #[inline(always)]
    pub fn fmt(&self) -> FmtR {
        FmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 12:31 - CoreSight ROM Table Address Offset"]
    #[inline(always)]
    pub fn addoff(&self) -> AddoffR {
        AddoffR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[doc = "CoreSight ROM Table Entry N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`entry2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Entry2Spec;
impl crate::RegisterSpec for Entry2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry2::R`](R) reader structure"]
impl crate::Readable for Entry2Spec {}
#[doc = "`reset()` method sets ENTRY2 to value 0xbbff_f002"]
impl crate::Resettable for Entry2Spec {
    const RESET_VALUE: u32 = 0xbbff_f002;
}

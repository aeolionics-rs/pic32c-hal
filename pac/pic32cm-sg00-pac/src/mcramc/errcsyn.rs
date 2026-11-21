#[doc = "Register `ERRCSYN` reader"]
pub type R = crate::R<ErrcsynSpec>;
#[doc = "ECC SECDED Error Capture Syndrome\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ercsynselect {
    #[doc = "0: Not a Single bit error."]
    Ercsyn0 = 0,
    #[doc = "1: Single bit error."]
    Ercsyn1 = 1,
}
impl From<Ercsynselect> for u8 {
    #[inline(always)]
    fn from(variant: Ercsynselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ercsynselect {
    type Ux = u8;
}
impl crate::IsEnum for Ercsynselect {}
#[doc = "Field `ERCSYN` reader - ECC SECDED Error Capture Syndrome"]
pub type ErcsynR = crate::FieldReader<Ercsynselect>;
impl ErcsynR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ercsynselect> {
        match self.bits {
            0 => Some(Ercsynselect::Ercsyn0),
            1 => Some(Ercsynselect::Ercsyn1),
            _ => None,
        }
    }
    #[doc = "Not a Single bit error."]
    #[inline(always)]
    pub fn is_ercsyn_0(&self) -> bool {
        *self == Ercsynselect::Ercsyn0
    }
    #[doc = "Single bit error."]
    #[inline(always)]
    pub fn is_ercsyn_1(&self) -> bool {
        *self == Ercsynselect::Ercsyn1
    }
}
#[doc = "Field `ERR1` reader - ECC Single Bit Error"]
pub type Err1R = crate::BitReader;
#[doc = "ECC Double Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Err2select {
    #[doc = "0: Not a Double bit error."]
    Err2_0 = 0,
    #[doc = "1: Double bit error."]
    Err2_1 = 1,
}
impl From<Err2select> for bool {
    #[inline(always)]
    fn from(variant: Err2select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR2` reader - ECC Double Bit Error"]
pub type Err2R = crate::BitReader<Err2select>;
impl Err2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Err2select {
        match self.bits {
            false => Err2select::Err2_0,
            true => Err2select::Err2_1,
        }
    }
    #[doc = "Not a Double bit error."]
    #[inline(always)]
    pub fn is_err2_0(&self) -> bool {
        *self == Err2select::Err2_0
    }
    #[doc = "Double bit error."]
    #[inline(always)]
    pub fn is_err2_1(&self) -> bool {
        *self == Err2select::Err2_1
    }
}
impl R {
    #[doc = "Bits 0:7 - ECC SECDED Error Capture Syndrome"]
    #[inline(always)]
    pub fn ercsyn(&self) -> ErcsynR {
        ErcsynR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - ECC Single Bit Error"]
    #[inline(always)]
    pub fn err1(&self) -> Err1R {
        Err1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ECC Double Bit Error"]
    #[inline(always)]
    pub fn err2(&self) -> Err2R {
        Err2R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Error Capture Syndrome Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errcsyn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrcsynSpec;
impl crate::RegisterSpec for ErrcsynSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errcsyn::R`](R) reader structure"]
impl crate::Readable for ErrcsynSpec {}
#[doc = "`reset()` method sets ERRCSYN to value 0"]
impl crate::Resettable for ErrcsynSpec {}

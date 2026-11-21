#[doc = "Register `FFLTSYN` reader"]
pub type R = crate::R<FfltsynSpec>;
#[doc = "Field `SECSYN` reader - Single Error Correction Syndrome"]
pub type SecsynR = crate::FieldReader<u16>;
#[doc = "DED Syndrome\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dedsynselect {
    #[doc = "0: Calculated Overall Parity Concurs with Read Overall Parity"]
    Dedsyn0 = 0,
    #[doc = "1: Calculated Overall Parity Differs from Read Overall Parity"]
    Dedsyn1 = 1,
}
impl From<Dedsynselect> for bool {
    #[inline(always)]
    fn from(variant: Dedsynselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEDSYN` reader - DED Syndrome"]
pub type DedsynR = crate::BitReader<Dedsynselect>;
impl DedsynR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dedsynselect {
        match self.bits {
            false => Dedsynselect::Dedsyn0,
            true => Dedsynselect::Dedsyn1,
        }
    }
    #[doc = "Calculated Overall Parity Concurs with Read Overall Parity"]
    #[inline(always)]
    pub fn is_dedsyn_0(&self) -> bool {
        *self == Dedsynselect::Dedsyn0
    }
    #[doc = "Calculated Overall Parity Differs from Read Overall Parity"]
    #[inline(always)]
    pub fn is_dedsyn_1(&self) -> bool {
        *self == Dedsynselect::Dedsyn1
    }
}
#[doc = "Double Error Detected &amp; Single Error Corrected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dserrselect {
    #[doc = "0: No Errors"]
    None = 0,
    #[doc = "1: Single Error Corrected"]
    Single = 1,
    #[doc = "2: Double Error Detected"]
    Double10 = 2,
    #[doc = "3: Double Error Detected"]
    Double11 = 3,
}
impl From<Dserrselect> for u8 {
    #[inline(always)]
    fn from(variant: Dserrselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dserrselect {
    type Ux = u8;
}
impl crate::IsEnum for Dserrselect {}
#[doc = "Field `DSERR` reader - Double Error Detected &amp; Single Error Corrected"]
pub type DserrR = crate::FieldReader<Dserrselect>;
impl DserrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dserrselect {
        match self.bits {
            0 => Dserrselect::None,
            1 => Dserrselect::Single,
            2 => Dserrselect::Double10,
            3 => Dserrselect::Double11,
            _ => unreachable!(),
        }
    }
    #[doc = "No Errors"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dserrselect::None
    }
    #[doc = "Single Error Corrected"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Dserrselect::Single
    }
    #[doc = "Double Error Detected"]
    #[inline(always)]
    pub fn is_double_10(&self) -> bool {
        *self == Dserrselect::Double10
    }
    #[doc = "Double Error Detected"]
    #[inline(always)]
    pub fn is_double_11(&self) -> bool {
        *self == Dserrselect::Double11
    }
}
#[doc = "ECC Control bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cerrselect {
    #[doc = "0: No Control bit Error (ECCSTAT either 111 or 000)"]
    None = 0,
    #[doc = "1: Single Control Bit Error"]
    Single = 1,
}
impl From<Cerrselect> for bool {
    #[inline(always)]
    fn from(variant: Cerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERR` reader - ECC Control bit Error"]
pub type CerrR = crate::BitReader<Cerrselect>;
impl CerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cerrselect {
        match self.bits {
            false => Cerrselect::None,
            true => Cerrselect::Single,
        }
    }
    #[doc = "No Control bit Error (ECCSTAT either 111 or 000)"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cerrselect::None
    }
    #[doc = "Single Control Bit Error"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Cerrselect::Single
    }
}
#[doc = "Parity vs ECC Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctlstatselect {
    #[doc = "0: Calculation used ECC (i.e. programming used quad write)"]
    Usedecc0 = 0,
    #[doc = "1: Calculation used ECC (i.e. programming used quad write)"]
    Usedecc1 = 1,
    #[doc = "2: Calculation used ECC (i.e. programming used quad write)"]
    Usedecc10 = 2,
    #[doc = "3: Calculation used Parity (i.e. programming used single write)"]
    Usedparity11 = 3,
    #[doc = "4: Calculation used ECC (i.e. programming used quad write)"]
    Usedecc100 = 4,
    #[doc = "5: Calculation used Parity (i.e. programming used single write)"]
    Usedparity101 = 5,
    #[doc = "6: Calculation used Parity (i.e. programming used single write)"]
    Usedparity110 = 6,
    #[doc = "7: Calculation used Parity (i.e. programming used single write)"]
    Usedparity111 = 7,
}
impl From<Ctlstatselect> for u8 {
    #[inline(always)]
    fn from(variant: Ctlstatselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctlstatselect {
    type Ux = u8;
}
impl crate::IsEnum for Ctlstatselect {}
#[doc = "Field `CTLSTAT` reader - Parity vs ECC Control Status"]
pub type CtlstatR = crate::FieldReader<Ctlstatselect>;
impl CtlstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctlstatselect {
        match self.bits {
            0 => Ctlstatselect::Usedecc0,
            1 => Ctlstatselect::Usedecc1,
            2 => Ctlstatselect::Usedecc10,
            3 => Ctlstatselect::Usedparity11,
            4 => Ctlstatselect::Usedecc100,
            5 => Ctlstatselect::Usedparity101,
            6 => Ctlstatselect::Usedparity110,
            7 => Ctlstatselect::Usedparity111,
            _ => unreachable!(),
        }
    }
    #[doc = "Calculation used ECC (i.e. programming used quad write)"]
    #[inline(always)]
    pub fn is_usedecc_0(&self) -> bool {
        *self == Ctlstatselect::Usedecc0
    }
    #[doc = "Calculation used ECC (i.e. programming used quad write)"]
    #[inline(always)]
    pub fn is_usedecc_1(&self) -> bool {
        *self == Ctlstatselect::Usedecc1
    }
    #[doc = "Calculation used ECC (i.e. programming used quad write)"]
    #[inline(always)]
    pub fn is_usedecc_10(&self) -> bool {
        *self == Ctlstatselect::Usedecc10
    }
    #[doc = "Calculation used Parity (i.e. programming used single write)"]
    #[inline(always)]
    pub fn is_usedparity_11(&self) -> bool {
        *self == Ctlstatselect::Usedparity11
    }
    #[doc = "Calculation used ECC (i.e. programming used quad write)"]
    #[inline(always)]
    pub fn is_usedecc_100(&self) -> bool {
        *self == Ctlstatselect::Usedecc100
    }
    #[doc = "Calculation used Parity (i.e. programming used single write)"]
    #[inline(always)]
    pub fn is_usedparity_101(&self) -> bool {
        *self == Ctlstatselect::Usedparity101
    }
    #[doc = "Calculation used Parity (i.e. programming used single write)"]
    #[inline(always)]
    pub fn is_usedparity_110(&self) -> bool {
        *self == Ctlstatselect::Usedparity110
    }
    #[doc = "Calculation used Parity (i.e. programming used single write)"]
    #[inline(always)]
    pub fn is_usedparity_111(&self) -> bool {
        *self == Ctlstatselect::Usedparity111
    }
}
#[doc = "Per Word (n=3:0) Parity Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Perrselect {
    #[doc = "0: No Parity Error on Word n"]
    None = 0,
    #[doc = "1: Parity Error on Word n"]
    Error = 1,
}
impl From<Perrselect> for u8 {
    #[inline(always)]
    fn from(variant: Perrselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Perrselect {
    type Ux = u8;
}
impl crate::IsEnum for Perrselect {}
#[doc = "Field `PERR` reader - Per Word (n=3:0) Parity Error Status"]
pub type PerrR = crate::FieldReader<Perrselect>;
impl PerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Perrselect> {
        match self.bits {
            0 => Some(Perrselect::None),
            1 => Some(Perrselect::Error),
            _ => None,
        }
    }
    #[doc = "No Parity Error on Word n"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Perrselect::None
    }
    #[doc = "Parity Error on Word n"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Perrselect::Error
    }
}
impl R {
    #[doc = "Bits 0:8 - Single Error Correction Syndrome"]
    #[inline(always)]
    pub fn secsyn(&self) -> SecsynR {
        SecsynR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - DED Syndrome"]
    #[inline(always)]
    pub fn dedsyn(&self) -> DedsynR {
        DedsynR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Double Error Detected &amp; Single Error Corrected"]
    #[inline(always)]
    pub fn dserr(&self) -> DserrR {
        DserrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - ECC Control bit Error"]
    #[inline(always)]
    pub fn cerr(&self) -> CerrR {
        CerrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Parity vs ECC Control Status"]
    #[inline(always)]
    pub fn ctlstat(&self) -> CtlstatR {
        CtlstatR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Per Word (n=3:0) Parity Error Status"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Flash ECC Fault Syndrome REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltsyn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfltsynSpec;
impl crate::RegisterSpec for FfltsynSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffltsyn::R`](R) reader structure"]
impl crate::Readable for FfltsynSpec {}
#[doc = "`reset()` method sets FFLTSYN to value 0"]
impl crate::Resettable for FfltsynSpec {}

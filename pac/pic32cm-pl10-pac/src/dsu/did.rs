#[doc = "Register `DID` reader"]
pub type R = crate::R<DidSpec>;
#[doc = "Field `FV` reader - Fixed Value of 1"]
pub type FvR = crate::BitReader;
#[doc = "Field `MANID` reader - JEDEC Manufacture ID"]
pub type ManidR = crate::FieldReader<u16>;
#[doc = "Field `PNDID` reader - Part Number Device ID"]
pub type PndidR = crate::FieldReader;
#[doc = "Field `PNMID` reader - Part Number Mask ID"]
pub type PnmidR = crate::FieldReader;
#[doc = "Field `VER` reader - Version Code"]
pub type VerR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Fixed Value of 1"]
    #[inline(always)]
    pub fn fv(&self) -> FvR {
        FvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - JEDEC Manufacture ID"]
    #[inline(always)]
    pub fn manid(&self) -> ManidR {
        ManidR::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:19 - Part Number Device ID"]
    #[inline(always)]
    pub fn pndid(&self) -> PndidR {
        PndidR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:27 - Part Number Mask ID"]
    #[inline(always)]
    pub fn pnmid(&self) -> PnmidR {
        PnmidR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - Version Code"]
    #[inline(always)]
    pub fn ver(&self) -> VerR {
        VerR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Device Identification\n\nYou can [`read`](crate::Reg::read) this register and get [`did::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DidSpec;
impl crate::RegisterSpec for DidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`did::R`](R) reader structure"]
impl crate::Readable for DidSpec {}
#[doc = "`reset()` method sets DID to value 0x0ba0_2053"]
impl crate::Resettable for DidSpec {
    const RESET_VALUE: u32 = 0x0ba0_2053;
}

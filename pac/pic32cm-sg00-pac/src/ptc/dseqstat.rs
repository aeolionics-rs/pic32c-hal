#[doc = "Register `DSEQSTAT` reader"]
pub type R = crate::R<DseqstatSpec>;
#[doc = "Field `CTRLD` reader - Control D."]
pub type CtrldR = crate::BitReader;
#[doc = "Field `WINLT` reader - Window Comparator Low Threshold"]
pub type WinltR = crate::BitReader;
#[doc = "Field `WINHT` reader - Window Comparator High Threshold"]
pub type WinhtR = crate::BitReader;
#[doc = "Field `XSEL` reader - X-line n Selection lower/upper half word"]
pub type XselR = crate::FieldReader;
#[doc = "Field `YSEL` reader - Y-line n Selection lower/upper half word"]
pub type YselR = crate::FieldReader;
#[doc = "Field `POL` reader - Polarity n lower/upper half word"]
pub type PolR = crate::FieldReader;
#[doc = "Field `CC` reader - Compensation Capacitor"]
pub type CcR = crate::BitReader;
#[doc = "Field `CI` reader - Integration Capacitor"]
pub type CiR = crate::BitReader;
#[doc = "Field `CSD` reader - Charge Share Delay"]
pub type CsdR = crate::BitReader;
#[doc = "Field `BUSY` reader - Sequencing Busy"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Control D."]
    #[inline(always)]
    pub fn ctrld(&self) -> CtrldR {
        CtrldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Window Comparator Low Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WinltR {
        WinltR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Comparator High Threshold"]
    #[inline(always)]
    pub fn winht(&self) -> WinhtR {
        WinhtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - X-line n Selection lower/upper half word"]
    #[inline(always)]
    pub fn xsel(&self) -> XselR {
        XselR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - Y-line n Selection lower/upper half word"]
    #[inline(always)]
    pub fn ysel(&self) -> YselR {
        YselR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - Polarity n lower/upper half word"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Compensation Capacitor"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Integration Capacitor"]
    #[inline(always)]
    pub fn ci(&self) -> CiR {
        CiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Charge Share Delay"]
    #[inline(always)]
    pub fn csd(&self) -> CsdR {
        CsdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Sequencing Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DMA SEQUENCE STATUS - Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dseqstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DseqstatSpec;
impl crate::RegisterSpec for DseqstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dseqstat::R`](R) reader structure"]
impl crate::Readable for DseqstatSpec {}
#[doc = "`reset()` method sets DSEQSTAT to value 0"]
impl crate::Resettable for DseqstatSpec {}

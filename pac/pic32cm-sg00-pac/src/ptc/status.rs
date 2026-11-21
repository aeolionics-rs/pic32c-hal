#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `PTCBUSY` reader - PTC Busy Status"]
pub type PtcbusyR = crate::BitReader;
#[doc = "Field `DAD` reader - Digital Accumulation Done"]
pub type DadR = crate::BitReader;
#[doc = "Field `WCC` reader - Window Comparator Counter"]
pub type WccR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - PTC Busy Status"]
    #[inline(always)]
    pub fn ptcbusy(&self) -> PtcbusyR {
        PtcbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Digital Accumulation Done"]
    #[inline(always)]
    pub fn dad(&self) -> DadR {
        DadR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Window Comparator Counter"]
    #[inline(always)]
    pub fn wcc(&self) -> WccR {
        WccR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
#[doc = "STATUS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}

#[doc = "Register `TSS2` reader"]
pub type R = crate::R<Tss2Spec>;
#[doc = "Field `TSP` reader - Timestamp Pointer"]
pub type TspR = crate::FieldReader;
#[doc = "Field `NTSG` reader - Number of Timestamps Generic"]
pub type NtsgR = crate::FieldReader;
#[doc = "Field `ITBG` reader - Internal Timebase and SOF select Generic"]
pub type ItbgR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Timestamp Pointer"]
    #[inline(always)]
    pub fn tsp(&self) -> TspR {
        TspR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Number of Timestamps Generic"]
    #[inline(always)]
    pub fn ntsg(&self) -> NtsgR {
        NtsgR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Internal Timebase and SOF select Generic"]
    #[inline(always)]
    pub fn itbg(&self) -> ItbgR {
        ItbgR::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "Timestamp Status 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tss2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tss2Spec;
impl crate::RegisterSpec for Tss2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tss2::R`](R) reader structure"]
impl crate::Readable for Tss2Spec {}
#[doc = "`reset()` method sets TSS2 to value 0xa000"]
impl crate::Resettable for Tss2Spec {
    const RESET_VALUE: u32 = 0xa000;
}

#[doc = "Register `TSS1` reader"]
pub type R = crate::R<Tss1Spec>;
#[doc = "Field `TSL` reader - Timestamp Lost"]
pub type TslR = crate::FieldReader<u16>;
#[doc = "Field `TSN` reader - Timestamp New"]
pub type TsnR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Lost"]
    #[inline(always)]
    pub fn tsl(&self) -> TslR {
        TslR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Timestamp New"]
    #[inline(always)]
    pub fn tsn(&self) -> TsnR {
        TsnR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Timestamp Status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tss1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tss1Spec;
impl crate::RegisterSpec for Tss1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tss1::R`](R) reader structure"]
impl crate::Readable for Tss1Spec {}
#[doc = "`reset()` method sets TSS1 to value 0"]
impl crate::Resettable for Tss1Spec {}

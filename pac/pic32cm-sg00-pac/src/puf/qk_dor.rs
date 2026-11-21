#[doc = "Register `QK_DOR` reader"]
pub type R = crate::R<QkDorSpec>;
#[doc = "Field `qk_do` reader - Output data from QuiddiKey"]
pub type QkDoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Output data from QuiddiKey"]
    #[inline(always)]
    pub fn qk_do(&self) -> QkDoR {
        QkDoR::new(self.bits)
    }
}
#[doc = "Data Output register; The Data Output register is used to transfer data from QuiddiKey. It can only be read when input data is available (indicated by the qk_do_request signal). When read at any other moment, or when written, an error response is generated. Depending on the way the system is configured and connected, this may generate an exception.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_dor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkDorSpec;
impl crate::RegisterSpec for QkDorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_dor::R`](R) reader structure"]
impl crate::Readable for QkDorSpec {}
#[doc = "`reset()` method sets QK_DOR to value 0"]
impl crate::Resettable for QkDorSpec {}

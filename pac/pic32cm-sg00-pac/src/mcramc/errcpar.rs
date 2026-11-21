#[doc = "Register `ERRCPAR` reader"]
pub type R = crate::R<ErrcparSpec>;
#[doc = "Field `ERCPAR` reader - ECC SECDED Error Capture Parity"]
pub type ErcparR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - ECC SECDED Error Capture Parity"]
    #[inline(always)]
    pub fn ercpar(&self) -> ErcparR {
        ErcparR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Error Capture Parity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errcpar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrcparSpec;
impl crate::RegisterSpec for ErrcparSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errcpar::R`](R) reader structure"]
impl crate::Readable for ErrcparSpec {}
#[doc = "`reset()` method sets ERRCPAR to value 0"]
impl crate::Resettable for ErrcparSpec {}

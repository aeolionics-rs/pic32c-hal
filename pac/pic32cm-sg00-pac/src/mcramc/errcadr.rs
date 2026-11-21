#[doc = "Register `ERRCADR` reader"]
pub type R = crate::R<ErrcadrSpec>;
#[doc = "Field `ERCADR` reader - ECC SECDED Error Capture Address"]
pub type ErcadrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - ECC SECDED Error Capture Address"]
    #[inline(always)]
    pub fn ercadr(&self) -> ErcadrR {
        ErcadrR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Error Capture Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errcadr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrcadrSpec;
impl crate::RegisterSpec for ErrcadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errcadr::R`](R) reader structure"]
impl crate::Readable for ErrcadrSpec {}
#[doc = "`reset()` method sets ERRCADR to value 0"]
impl crate::Resettable for ErrcadrSpec {}

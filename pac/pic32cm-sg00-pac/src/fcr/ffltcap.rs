#[doc = "Register `FFLTCAP` reader"]
pub type R = crate::R<FfltcapSpec>;
#[doc = "Field `FLTADR` reader - Fault Capture Address"]
pub type FltadrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Fault Capture Address"]
    #[inline(always)]
    pub fn fltadr(&self) -> FltadrR {
        FltadrR::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "Flash ECC Fault Capture Address REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltcap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfltcapSpec;
impl crate::RegisterSpec for FfltcapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffltcap::R`](R) reader structure"]
impl crate::Readable for FfltcapSpec {}
#[doc = "`reset()` method sets FFLTCAP to value 0"]
impl crate::Resettable for FfltcapSpec {}

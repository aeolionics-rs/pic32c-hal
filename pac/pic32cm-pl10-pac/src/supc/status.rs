#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `BODVDDRDY` reader - BODVDD Ready"]
pub type BodvddrdyR = crate::BitReader;
#[doc = "Field `VLM` reader - Voltage Level Monitor"]
pub type VlmR = crate::BitReader;
#[doc = "Field `VDDIO2OK` reader - VDDIO2 OK"]
pub type Vddio2okR = crate::BitReader;
#[doc = "Field `VDDIO2LPMPOR` reader - VDDIO2 Low Power Mode POR"]
pub type Vddio2lpmporR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    pub fn bodvddrdy(&self) -> BodvddrdyR {
        BodvddrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Level Monitor"]
    #[inline(always)]
    pub fn vlm(&self) -> VlmR {
        VlmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VDDIO2 OK"]
    #[inline(always)]
    pub fn vddio2ok(&self) -> Vddio2okR {
        Vddio2okR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VDDIO2 Low Power Mode POR"]
    #[inline(always)]
    pub fn vddio2lpmpor(&self) -> Vddio2lpmporR {
        Vddio2lpmporR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}

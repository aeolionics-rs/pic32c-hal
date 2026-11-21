#[doc = "Register `PID0` reader"]
pub type R = crate::R<Pid0Spec>;
#[doc = "Field `PART0` reader - CoreSight Part Number Bits\\[7:0\\]"]
pub type Part0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Part Number Bits\\[7:0\\]"]
    #[inline(always)]
    pub fn part0(&self) -> Part0R {
        Part0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CoreSight Peripheral Identification 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid0Spec;
impl crate::RegisterSpec for Pid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid0::R`](R) reader structure"]
impl crate::Readable for Pid0Spec {}
#[doc = "`reset()` method sets PID0 to value 0xd0"]
impl crate::Resettable for Pid0Spec {
    const RESET_VALUE: u32 = 0xd0;
}

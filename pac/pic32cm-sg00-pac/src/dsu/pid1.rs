#[doc = "Register `PID1` reader"]
pub type R = crate::R<Pid1Spec>;
#[doc = "Field `PART1` reader - CoreSight Part Number Bits\\[11:8\\]"]
pub type Part1R = crate::FieldReader;
#[doc = "Field `DES0` reader - CoreSight JEP106 Identification Code Bits \\[3:0\\]"]
pub type Des0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - CoreSight Part Number Bits\\[11:8\\]"]
    #[inline(always)]
    pub fn part1(&self) -> Part1R {
        Part1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CoreSight JEP106 Identification Code Bits \\[3:0\\]"]
    #[inline(always)]
    pub fn des0(&self) -> Des0R {
        Des0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "CoreSight Peripheral Identification 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid1Spec;
impl crate::RegisterSpec for Pid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid1::R`](R) reader structure"]
impl crate::Readable for Pid1Spec {}
#[doc = "`reset()` method sets PID1 to value 0x9c"]
impl crate::Resettable for Pid1Spec {
    const RESET_VALUE: u32 = 0x9c;
}

#[doc = "Register `STATUSA` reader"]
pub type R = crate::R<StatusaSpec>;
#[doc = "Field `DONE` reader - Done Status"]
pub type DoneR = crate::BitReader;
#[doc = "Field `CRSTEXT0` reader - CPU0 Reset Extension Status"]
pub type Crstext0R = crate::BitReader;
#[doc = "Field `BREXT0` reader - Boot ROM 0 Phase Extension Status"]
pub type Brext0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Done Status"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - CPU0 Reset Extension Status"]
    #[inline(always)]
    pub fn crstext0(&self) -> Crstext0R {
        Crstext0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Boot ROM 0 Phase Extension Status"]
    #[inline(always)]
    pub fn brext0(&self) -> Brext0R {
        Brext0R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Status A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusaSpec;
impl crate::RegisterSpec for StatusaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusa::R`](R) reader structure"]
impl crate::Readable for StatusaSpec {}
#[doc = "`reset()` method sets STATUSA to value 0"]
impl crate::Resettable for StatusaSpec {}

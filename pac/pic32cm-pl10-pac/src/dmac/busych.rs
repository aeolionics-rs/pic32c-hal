#[doc = "Register `BUSYCH` reader"]
pub type R = crate::R<BusychSpec>;
#[doc = "Field `BUSYCH0` reader - Busy Channel 0"]
pub type Busych0R = crate::BitReader;
#[doc = "Field `BUSYCH1` reader - Busy Channel 1"]
pub type Busych1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Busy Channel 0"]
    #[inline(always)]
    pub fn busych0(&self) -> Busych0R {
        Busych0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy Channel 1"]
    #[inline(always)]
    pub fn busych1(&self) -> Busych1R {
        Busych1R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Busy Channels\n\nYou can [`read`](crate::Reg::read) this register and get [`busych::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusychSpec;
impl crate::RegisterSpec for BusychSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busych::R`](R) reader structure"]
impl crate::Readable for BusychSpec {}
#[doc = "`reset()` method sets BUSYCH to value 0"]
impl crate::Resettable for BusychSpec {}

#[doc = "Register `MEIMR` reader"]
pub type R = crate::R<MeimrSpec>;
#[doc = "Field `MERR0` reader - Master 0 Access Error"]
pub type Merr0R = crate::BitReader;
#[doc = "Field `MERR1` reader - Master 1 Access Error"]
pub type Merr1R = crate::BitReader;
#[doc = "Field `MERR2` reader - Master 2 Access Error"]
pub type Merr2R = crate::BitReader;
#[doc = "Field `MERR3` reader - Master 3 Access Error"]
pub type Merr3R = crate::BitReader;
#[doc = "Field `MERR4` reader - Master 4 Access Error"]
pub type Merr4R = crate::BitReader;
#[doc = "Field `MERR5` reader - Master 5 Access Error"]
pub type Merr5R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Master 0 Access Error"]
    #[inline(always)]
    pub fn merr0(&self) -> Merr0R {
        Merr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master 1 Access Error"]
    #[inline(always)]
    pub fn merr1(&self) -> Merr1R {
        Merr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master 2 Access Error"]
    #[inline(always)]
    pub fn merr2(&self) -> Merr2R {
        Merr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master 3 Access Error"]
    #[inline(always)]
    pub fn merr3(&self) -> Merr3R {
        Merr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master 4 Access Error"]
    #[inline(always)]
    pub fn merr4(&self) -> Merr4R {
        Merr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master 5 Access Error"]
    #[inline(always)]
    pub fn merr5(&self) -> Merr5R {
        Merr5R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Master Error Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`meimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MeimrSpec;
impl crate::RegisterSpec for MeimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meimr::R`](R) reader structure"]
impl crate::Readable for MeimrSpec {}
#[doc = "`reset()` method sets MEIMR to value 0"]
impl crate::Resettable for MeimrSpec {}

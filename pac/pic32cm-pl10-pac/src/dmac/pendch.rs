#[doc = "Register `PENDCH` reader"]
pub type R = crate::R<PendchSpec>;
#[doc = "Field `PENDCH0` reader - Pending Channel 0"]
pub type Pendch0R = crate::BitReader;
#[doc = "Field `PENDCH1` reader - Pending Channel 1"]
pub type Pendch1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Pending Channel 0"]
    #[inline(always)]
    pub fn pendch0(&self) -> Pendch0R {
        Pendch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending Channel 1"]
    #[inline(always)]
    pub fn pendch1(&self) -> Pendch1R {
        Pendch1R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Pending Channels\n\nYou can [`read`](crate::Reg::read) this register and get [`pendch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PendchSpec;
impl crate::RegisterSpec for PendchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pendch::R`](R) reader structure"]
impl crate::Readable for PendchSpec {}
#[doc = "`reset()` method sets PENDCH to value 0"]
impl crate::Resettable for PendchSpec {}

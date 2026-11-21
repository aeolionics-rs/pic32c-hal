#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "NVM Power Reduction Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prmselect {
    #[doc = "0: Flash is Ready (for read or NVMOP)"]
    Prm0 = 0,
    #[doc = "1: Flash is Powered Down and wakes on first access (read or NVMOP)"]
    Prm1 = 1,
}
impl From<Prmselect> for bool {
    #[inline(always)]
    fn from(variant: Prmselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRM` reader - NVM Power Reduction Mode Status"]
pub type PrmR = crate::BitReader<Prmselect>;
impl PrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prmselect {
        match self.bits {
            false => Prmselect::Prm0,
            true => Prmselect::Prm1,
        }
    }
    #[doc = "Flash is Ready (for read or NVMOP)"]
    #[inline(always)]
    pub fn is_prm_0(&self) -> bool {
        *self == Prmselect::Prm0
    }
    #[doc = "Flash is Powered Down and wakes on first access (read or NVMOP)"]
    #[inline(always)]
    pub fn is_prm_1(&self) -> bool {
        *self == Prmselect::Prm1
    }
}
impl R {
    #[doc = "Bit 0 - NVM Power Reduction Mode Status"]
    #[inline(always)]
    pub fn prm(&self) -> PrmR {
        PrmR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0xe000_0000"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0xe000_0000;
}

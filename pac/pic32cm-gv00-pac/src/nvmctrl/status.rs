#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `PRM` reader - Power Reduction Mode"]
pub type PrmR = crate::BitReader;
#[doc = "Field `PRM` writer - Power Reduction Mode"]
pub type PrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOAD` reader - NVM Page Buffer Active Loading"]
pub type LoadR = crate::BitReader;
#[doc = "Field `LOAD` writer - NVM Page Buffer Active Loading"]
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGE` reader - Programming Error Status"]
pub type ProgeR = crate::BitReader;
#[doc = "Field `PROGE` writer - Programming Error Status"]
pub type ProgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKE` reader - Lock Error Status"]
pub type LockeR = crate::BitReader;
#[doc = "Field `LOCKE` writer - Lock Error Status"]
pub type LockeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVME` reader - NVM Error"]
pub type NvmeR = crate::BitReader;
#[doc = "Field `NVME` writer - NVM Error"]
pub type NvmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SB` reader - Security Bit Status"]
pub type SbR = crate::BitReader;
#[doc = "Field `SB` writer - Security Bit Status"]
pub type SbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Reduction Mode"]
    #[inline(always)]
    pub fn prm(&self) -> PrmR {
        PrmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NVM Page Buffer Active Loading"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Programming Error Status"]
    #[inline(always)]
    pub fn proge(&self) -> ProgeR {
        ProgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock Error Status"]
    #[inline(always)]
    pub fn locke(&self) -> LockeR {
        LockeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NVM Error"]
    #[inline(always)]
    pub fn nvme(&self) -> NvmeR {
        NvmeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Security Bit Status"]
    #[inline(always)]
    pub fn sb(&self) -> SbR {
        SbR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Reduction Mode"]
    #[inline(always)]
    pub fn prm(&mut self) -> PrmW<'_, StatusSpec> {
        PrmW::new(self, 0)
    }
    #[doc = "Bit 1 - NVM Page Buffer Active Loading"]
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<'_, StatusSpec> {
        LoadW::new(self, 1)
    }
    #[doc = "Bit 2 - Programming Error Status"]
    #[inline(always)]
    pub fn proge(&mut self) -> ProgeW<'_, StatusSpec> {
        ProgeW::new(self, 2)
    }
    #[doc = "Bit 3 - Lock Error Status"]
    #[inline(always)]
    pub fn locke(&mut self) -> LockeW<'_, StatusSpec> {
        LockeW::new(self, 3)
    }
    #[doc = "Bit 4 - NVM Error"]
    #[inline(always)]
    pub fn nvme(&mut self) -> NvmeW<'_, StatusSpec> {
        NvmeW::new(self, 4)
    }
    #[doc = "Bit 8 - Security Bit Status"]
    #[inline(always)]
    pub fn sb(&mut self) -> SbW<'_, StatusSpec> {
        SbW::new(self, 8)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}

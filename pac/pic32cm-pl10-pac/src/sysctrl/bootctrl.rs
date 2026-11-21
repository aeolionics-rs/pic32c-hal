#[doc = "Register `BOOTCTRL` reader"]
pub type R = crate::R<BootctrlSpec>;
#[doc = "Register `BOOTCTRL` writer"]
pub type W = crate::W<BootctrlSpec>;
#[doc = "Field `SYSINIT` reader - System Initialization Ongoing"]
pub type SysinitR = crate::BitReader;
#[doc = "Field `SYSINIT` writer - System Initialization Ongoing"]
pub type SysinitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREXTEN` reader - BREXT Enable"]
pub type BrextenR = crate::BitReader;
#[doc = "Field `BREXTEN` writer - BREXT Enable"]
pub type BrextenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEPARK` reader - Force Park Mode Entry"]
pub type ForceparkR = crate::BitReader;
#[doc = "Field `FORCEPARK` writer - Force Park Mode Entry"]
pub type ForceparkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System Initialization Ongoing"]
    #[inline(always)]
    pub fn sysinit(&self) -> SysinitR {
        SysinitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BREXT Enable"]
    #[inline(always)]
    pub fn brexten(&self) -> BrextenR {
        BrextenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force Park Mode Entry"]
    #[inline(always)]
    pub fn forcepark(&self) -> ForceparkR {
        ForceparkR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Initialization Ongoing"]
    #[inline(always)]
    pub fn sysinit(&mut self) -> SysinitW<'_, BootctrlSpec> {
        SysinitW::new(self, 0)
    }
    #[doc = "Bit 1 - BREXT Enable"]
    #[inline(always)]
    pub fn brexten(&mut self) -> BrextenW<'_, BootctrlSpec> {
        BrextenW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Park Mode Entry"]
    #[inline(always)]
    pub fn forcepark(&mut self) -> ForceparkW<'_, BootctrlSpec> {
        ForceparkW::new(self, 2)
    }
}
#[doc = "Boot Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bootctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootctrlSpec;
impl crate::RegisterSpec for BootctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootctrl::R`](R) reader structure"]
impl crate::Readable for BootctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bootctrl::W`](W) writer structure"]
impl crate::Writable for BootctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOTCTRL to value 0"]
impl crate::Resettable for BootctrlSpec {}

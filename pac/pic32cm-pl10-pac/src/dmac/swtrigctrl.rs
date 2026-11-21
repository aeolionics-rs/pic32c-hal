#[doc = "Register `SWTRIGCTRL` reader"]
pub type R = crate::R<SwtrigctrlSpec>;
#[doc = "Register `SWTRIGCTRL` writer"]
pub type W = crate::W<SwtrigctrlSpec>;
#[doc = "Field `SWTRIG0` reader - Channel 0 Software Trigger"]
pub type Swtrig0R = crate::BitReader;
#[doc = "Field `SWTRIG0` writer - Channel 0 Software Trigger"]
pub type Swtrig0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG1` reader - Channel 1 Software Trigger"]
pub type Swtrig1R = crate::BitReader;
#[doc = "Field `SWTRIG1` writer - Channel 1 Software Trigger"]
pub type Swtrig1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    pub fn swtrig0(&self) -> Swtrig0R {
        Swtrig0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    pub fn swtrig1(&self) -> Swtrig1R {
        Swtrig1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    pub fn swtrig0(&mut self) -> Swtrig0W<'_, SwtrigctrlSpec> {
        Swtrig0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> Swtrig1W<'_, SwtrigctrlSpec> {
        Swtrig1W::new(self, 1)
    }
}
#[doc = "Software Trigger Control\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrigctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrigctrlSpec;
impl crate::RegisterSpec for SwtrigctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrigctrl::R`](R) reader structure"]
impl crate::Readable for SwtrigctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`swtrigctrl::W`](W) writer structure"]
impl crate::Writable for SwtrigctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWTRIGCTRL to value 0"]
impl crate::Resettable for SwtrigctrlSpec {}

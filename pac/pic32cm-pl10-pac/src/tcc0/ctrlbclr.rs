#[doc = "Register `CTRLBCLR` reader"]
pub type R = crate::R<CtrlbclrSpec>;
#[doc = "Register `CTRLBCLR` writer"]
pub type W = crate::W<CtrlbclrSpec>;
#[doc = "Field `DIR` reader - Counter Direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Counter Direction"]
pub type DirW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LUPD` reader - Lock Update"]
pub type LupdR = crate::BitReader;
#[doc = "Field `LUPD` writer - Lock Update"]
pub type LupdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ONESHOT` reader - One-Shot"]
pub type OneshotR = crate::BitReader;
#[doc = "Field `ONESHOT` writer - One-Shot"]
pub type OneshotW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IDXCMD` reader - Ramp Index Command"]
pub type IdxcmdR = crate::FieldReader;
#[doc = "Field `IDXCMD` writer - Ramp Index Command"]
pub type IdxcmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMD` reader - TCC Command"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `CMD` writer - TCC Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LupdR {
        LupdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline(always)]
    pub fn idxcmd(&self) -> IdxcmdR {
        IdxcmdR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bits 5:7 - TCC Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, CtrlbclrSpec> {
        DirW::new(self, 0)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&mut self) -> LupdW<'_, CtrlbclrSpec> {
        LupdW::new(self, 1)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<'_, CtrlbclrSpec> {
        OneshotW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline(always)]
    pub fn idxcmd(&mut self) -> IdxcmdW<'_, CtrlbclrSpec> {
        IdxcmdW::new(self, 3)
    }
    #[doc = "Bits 5:7 - TCC Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, CtrlbclrSpec> {
        CmdW::new(self, 5)
    }
}
#[doc = "Control B Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbclrSpec;
impl crate::RegisterSpec for CtrlbclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlbclr::R`](R) reader structure"]
impl crate::Readable for CtrlbclrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlbclr::W`](W) writer structure"]
impl crate::Writable for CtrlbclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0xff;
}
#[doc = "`reset()` method sets CTRLBCLR to value 0"]
impl crate::Resettable for CtrlbclrSpec {}

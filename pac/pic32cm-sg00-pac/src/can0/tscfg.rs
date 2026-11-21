#[doc = "Register `TSCFG` reader"]
pub type R = crate::R<TscfgSpec>;
#[doc = "Register `TSCFG` writer"]
pub type W = crate::W<TscfgSpec>;
#[doc = "Field `TSUE` reader - Timestamp Unit Enable"]
pub type TsueR = crate::BitReader;
#[doc = "Field `TSUE` writer - Timestamp Unit Enable"]
pub type TsueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCS` reader - Timebase Counter Select"]
pub type TbcsR = crate::BitReader;
#[doc = "Field `TBCS` writer - Timebase Counter Select"]
pub type TbcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCP` reader - Select Capturing Position"]
pub type ScpR = crate::BitReader;
#[doc = "Field `SCP` writer - Select Capturing Position"]
pub type ScpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBPRE` reader - Time Base Prescaler"]
pub type TbpreR = crate::FieldReader;
#[doc = "Field `TBPRE` writer - Time Base Prescaler"]
pub type TbpreW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Timestamp Unit Enable"]
    #[inline(always)]
    pub fn tsue(&self) -> TsueR {
        TsueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timebase Counter Select"]
    #[inline(always)]
    pub fn tbcs(&self) -> TbcsR {
        TbcsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select Capturing Position"]
    #[inline(always)]
    pub fn scp(&self) -> ScpR {
        ScpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Time Base Prescaler"]
    #[inline(always)]
    pub fn tbpre(&self) -> TbpreR {
        TbpreR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Unit Enable"]
    #[inline(always)]
    pub fn tsue(&mut self) -> TsueW<'_, TscfgSpec> {
        TsueW::new(self, 0)
    }
    #[doc = "Bit 1 - Timebase Counter Select"]
    #[inline(always)]
    pub fn tbcs(&mut self) -> TbcsW<'_, TscfgSpec> {
        TbcsW::new(self, 1)
    }
    #[doc = "Bit 2 - Select Capturing Position"]
    #[inline(always)]
    pub fn scp(&mut self) -> ScpW<'_, TscfgSpec> {
        ScpW::new(self, 2)
    }
    #[doc = "Bits 8:15 - Time Base Prescaler"]
    #[inline(always)]
    pub fn tbpre(&mut self) -> TbpreW<'_, TscfgSpec> {
        TbpreW::new(self, 8)
    }
}
#[doc = "Timestamp Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tscfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscfgSpec;
impl crate::RegisterSpec for TscfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscfg::R`](R) reader structure"]
impl crate::Readable for TscfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tscfg::W`](W) writer structure"]
impl crate::Writable for TscfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSCFG to value 0"]
impl crate::Resettable for TscfgSpec {}

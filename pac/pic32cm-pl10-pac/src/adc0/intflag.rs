#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `RESRDY` reader - Result Ready Interrupt Flag"]
pub type ResrdyR = crate::BitReader;
#[doc = "Field `RESRDY` writer - Result Ready Interrupt Flag"]
pub type ResrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SAMPRDY` reader - Sample Ready Interrupt Flag"]
pub type SamprdyR = crate::BitReader;
#[doc = "Field `SAMPRDY` writer - Sample Ready Interrupt Flag"]
pub type SamprdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WCMP` reader - Window Comparator Interrupt Flag"]
pub type WcmpR = crate::BitReader;
#[doc = "Field `WCMP` writer - Window Comparator Interrupt Flag"]
pub type WcmpW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RESOVR` reader - Result Overwrite Interrupt Flag"]
pub type ResovrR = crate::BitReader;
#[doc = "Field `RESOVR` writer - Result Overwrite Interrupt Flag"]
pub type ResovrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SAMPOVR` reader - Sample Overwrite Interrupt Flag"]
pub type SampovrR = crate::BitReader;
#[doc = "Field `SAMPOVR` writer - Sample Overwrite Interrupt Flag"]
pub type SampovrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRIGOVR` reader - Trigger Overrun Interrupt Flag"]
pub type TrigovrR = crate::BitReader;
#[doc = "Field `TRIGOVR` writer - Trigger Overrun Interrupt Flag"]
pub type TrigovrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Result Ready Interrupt Flag"]
    #[inline(always)]
    pub fn resrdy(&self) -> ResrdyR {
        ResrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sample Ready Interrupt Flag"]
    #[inline(always)]
    pub fn samprdy(&self) -> SamprdyR {
        SamprdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Comparator Interrupt Flag"]
    #[inline(always)]
    pub fn wcmp(&self) -> WcmpR {
        WcmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result Overwrite Interrupt Flag"]
    #[inline(always)]
    pub fn resovr(&self) -> ResovrR {
        ResovrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sample Overwrite Interrupt Flag"]
    #[inline(always)]
    pub fn sampovr(&self) -> SampovrR {
        SampovrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Overrun Interrupt Flag"]
    #[inline(always)]
    pub fn trigovr(&self) -> TrigovrR {
        TrigovrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Ready Interrupt Flag"]
    #[inline(always)]
    pub fn resrdy(&mut self) -> ResrdyW<'_, IntflagSpec> {
        ResrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Sample Ready Interrupt Flag"]
    #[inline(always)]
    pub fn samprdy(&mut self) -> SamprdyW<'_, IntflagSpec> {
        SamprdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Window Comparator Interrupt Flag"]
    #[inline(always)]
    pub fn wcmp(&mut self) -> WcmpW<'_, IntflagSpec> {
        WcmpW::new(self, 2)
    }
    #[doc = "Bit 3 - Result Overwrite Interrupt Flag"]
    #[inline(always)]
    pub fn resovr(&mut self) -> ResovrW<'_, IntflagSpec> {
        ResovrW::new(self, 3)
    }
    #[doc = "Bit 4 - Sample Overwrite Interrupt Flag"]
    #[inline(always)]
    pub fn sampovr(&mut self) -> SampovrW<'_, IntflagSpec> {
        SampovrW::new(self, 4)
    }
    #[doc = "Bit 5 - Trigger Overrun Interrupt Flag"]
    #[inline(always)]
    pub fn trigovr(&mut self) -> TrigovrW<'_, IntflagSpec> {
        TrigovrW::new(self, 5)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}

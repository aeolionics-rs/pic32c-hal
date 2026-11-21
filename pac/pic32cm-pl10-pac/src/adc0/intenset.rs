#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `RESRDY` reader - Result Ready Interrupt Enable"]
pub type ResrdyR = crate::BitReader;
#[doc = "Field `RESRDY` writer - Result Ready Interrupt Enable"]
pub type ResrdyW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SAMPRDY` reader - Sample Ready Interrupt Enable"]
pub type SamprdyR = crate::BitReader;
#[doc = "Field `SAMPRDY` writer - Sample Ready Interrupt Enable"]
pub type SamprdyW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `WCMP` reader - Window Comparator Interrupt Enable"]
pub type WcmpR = crate::BitReader;
#[doc = "Field `WCMP` writer - Window Comparator Interrupt Enable"]
pub type WcmpW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `RESOVR` reader - Result Overwrite Interrupt Enable"]
pub type ResovrR = crate::BitReader;
#[doc = "Field `RESOVR` writer - Result Overwrite Interrupt Enable"]
pub type ResovrW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SAMPOVR` reader - Sample Overwrite Interrupt Enable"]
pub type SampovrR = crate::BitReader;
#[doc = "Field `SAMPOVR` writer - Sample Overwrite Interrupt Enable"]
pub type SampovrW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `TRIGOVR` reader - Trigger Overrun Interrupt Enable"]
pub type TrigovrR = crate::BitReader;
#[doc = "Field `TRIGOVR` writer - Trigger Overrun Interrupt Enable"]
pub type TrigovrW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - Result Ready Interrupt Enable"]
    #[inline(always)]
    pub fn resrdy(&self) -> ResrdyR {
        ResrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sample Ready Interrupt Enable"]
    #[inline(always)]
    pub fn samprdy(&self) -> SamprdyR {
        SamprdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Comparator Interrupt Enable"]
    #[inline(always)]
    pub fn wcmp(&self) -> WcmpR {
        WcmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn resovr(&self) -> ResovrR {
        ResovrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sample Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn sampovr(&self) -> SampovrR {
        SampovrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn trigovr(&self) -> TrigovrR {
        TrigovrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Ready Interrupt Enable"]
    #[inline(always)]
    pub fn resrdy(&mut self) -> ResrdyW<'_, IntensetSpec> {
        ResrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Sample Ready Interrupt Enable"]
    #[inline(always)]
    pub fn samprdy(&mut self) -> SamprdyW<'_, IntensetSpec> {
        SamprdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Window Comparator Interrupt Enable"]
    #[inline(always)]
    pub fn wcmp(&mut self) -> WcmpW<'_, IntensetSpec> {
        WcmpW::new(self, 2)
    }
    #[doc = "Bit 3 - Result Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn resovr(&mut self) -> ResovrW<'_, IntensetSpec> {
        ResovrW::new(self, 3)
    }
    #[doc = "Bit 4 - Sample Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn sampovr(&mut self) -> SampovrW<'_, IntensetSpec> {
        SampovrW::new(self, 4)
    }
    #[doc = "Bit 5 - Trigger Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn trigovr(&mut self) -> TrigovrW<'_, IntensetSpec> {
        TrigovrW::new(self, 5)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}

#[doc = "Register `DMACTRL` reader"]
pub type R = crate::R<DmactrlSpec>;
#[doc = "Register `DMACTRL` writer"]
pub type W = crate::W<DmactrlSpec>;
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACR0` reader - DMA CORE0 Enable"]
pub type Dmacr0R = crate::BitReader;
#[doc = "Field `DMACR0` writer - DMA CORE0 Enable"]
pub type Dmacr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMABL` reader - DMA System RAM Buffer Length"]
pub type DmablR = crate::FieldReader;
#[doc = "Field `DMABL` writer - DMA System RAM Buffer Length"]
pub type DmablW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA CORE0 Enable"]
    #[inline(always)]
    pub fn dmacr0(&self) -> Dmacr0R {
        Dmacr0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - DMA System RAM Buffer Length"]
    #[inline(always)]
    pub fn dmabl(&self) -> DmablR {
        DmablR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, DmactrlSpec> {
        DmaenW::new(self, 1)
    }
    #[doc = "Bit 4 - DMA CORE0 Enable"]
    #[inline(always)]
    pub fn dmacr0(&mut self) -> Dmacr0W<'_, DmactrlSpec> {
        Dmacr0W::new(self, 4)
    }
    #[doc = "Bits 8:10 - DMA System RAM Buffer Length"]
    #[inline(always)]
    pub fn dmabl(&mut self) -> DmablW<'_, DmactrlSpec> {
        DmablW::new(self, 8)
    }
}
#[doc = "DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactrlSpec;
impl crate::RegisterSpec for DmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactrl::R`](R) reader structure"]
impl crate::Readable for DmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactrl::W`](W) writer structure"]
impl crate::Writable for DmactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTRL to value 0"]
impl crate::Resettable for DmactrlSpec {}

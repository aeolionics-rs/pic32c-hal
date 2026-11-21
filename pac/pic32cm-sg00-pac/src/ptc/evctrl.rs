#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `STCONVEI` reader - Start Conversion Event Input Enable"]
pub type StconveiR = crate::BitReader;
#[doc = "Field `STCONVEI` writer - Start Conversion Event Input Enable"]
pub type StconveiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSEQREI` reader - DMA Sequencing Restart Event Input Enable"]
pub type DseqreiR = crate::BitReader;
#[doc = "Field `DSEQREI` writer - DMA Sequencing Restart Event Input Enable"]
pub type DseqreiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STCONVINV` reader - Start Conversion Event Invert Enable"]
pub type StconvinvR = crate::BitReader;
#[doc = "Field `STCONVINV` writer - Start Conversion Event Invert Enable"]
pub type StconvinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSEQRINV` reader - DMA Sequencing Restart Event Invert Enable"]
pub type DseqrinvR = crate::BitReader;
#[doc = "Field `DSEQRINV` writer - DMA Sequencing Restart Event Invert Enable"]
pub type DseqrinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCEO` reader - End of Conversion Event Output Enable"]
pub type EoceoR = crate::BitReader;
#[doc = "Field `EOCEO` writer - End of Conversion Event Output Enable"]
pub type EoceoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCOMPEO` reader - Window Comparator Event Output Enable"]
pub type WcompeoR = crate::BitReader;
#[doc = "Field `WCOMPEO` writer - Window Comparator Event Output Enable"]
pub type WcompeoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Conversion Event Input Enable"]
    #[inline(always)]
    pub fn stconvei(&self) -> StconveiR {
        StconveiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Sequencing Restart Event Input Enable"]
    #[inline(always)]
    pub fn dseqrei(&self) -> DseqreiR {
        DseqreiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Conversion Event Invert Enable"]
    #[inline(always)]
    pub fn stconvinv(&self) -> StconvinvR {
        StconvinvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Sequencing Restart Event Invert Enable"]
    #[inline(always)]
    pub fn dseqrinv(&self) -> DseqrinvR {
        DseqrinvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Conversion Event Output Enable"]
    #[inline(always)]
    pub fn eoceo(&self) -> EoceoR {
        EoceoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window Comparator Event Output Enable"]
    #[inline(always)]
    pub fn wcompeo(&self) -> WcompeoR {
        WcompeoR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Event Input Enable"]
    #[inline(always)]
    pub fn stconvei(&mut self) -> StconveiW<'_, EvctrlSpec> {
        StconveiW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Sequencing Restart Event Input Enable"]
    #[inline(always)]
    pub fn dseqrei(&mut self) -> DseqreiW<'_, EvctrlSpec> {
        DseqreiW::new(self, 1)
    }
    #[doc = "Bit 2 - Start Conversion Event Invert Enable"]
    #[inline(always)]
    pub fn stconvinv(&mut self) -> StconvinvW<'_, EvctrlSpec> {
        StconvinvW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Sequencing Restart Event Invert Enable"]
    #[inline(always)]
    pub fn dseqrinv(&mut self) -> DseqrinvW<'_, EvctrlSpec> {
        DseqrinvW::new(self, 3)
    }
    #[doc = "Bit 4 - End of Conversion Event Output Enable"]
    #[inline(always)]
    pub fn eoceo(&mut self) -> EoceoW<'_, EvctrlSpec> {
        EoceoW::new(self, 4)
    }
    #[doc = "Bit 5 - Window Comparator Event Output Enable"]
    #[inline(always)]
    pub fn wcompeo(&mut self) -> WcompeoW<'_, EvctrlSpec> {
        WcompeoW::new(self, 5)
    }
}
#[doc = "EVENT CONTROL REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {}

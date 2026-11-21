#[doc = "Register `CTRLD` reader"]
pub type R = crate::R<CtrldSpec>;
#[doc = "Register `CTRLD` writer"]
pub type W = crate::W<CtrldSpec>;
#[doc = "Field `DAN` reader - Digital Accumulation Number"]
pub type DanR = crate::FieldReader;
#[doc = "Field `DAN` writer - Digital Accumulation Number"]
pub type DanW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AAN` reader - Analog Accumulation Number"]
pub type AanR = crate::FieldReader;
#[doc = "Field `AAN` writer - Analog Accumulation Number"]
pub type AanW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WINCM` reader - Window Comparator Mode"]
pub type WincmR = crate::FieldReader;
#[doc = "Field `WINCM` writer - Window Comparator Mode"]
pub type WincmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WINSS` reader - Window Single Sample"]
pub type WinssR = crate::BitReader;
#[doc = "Field `WINSS` writer - Window Single Sample"]
pub type WinssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTCIEN` reader - External Integration Capacitor Enable"]
pub type ExtcienR = crate::FieldReader;
#[doc = "Field `EXTCIEN` writer - External Integration Capacitor Enable"]
pub type ExtcienW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `POLICC` reader - Polarity of Internal Compensation Capacitor"]
pub type PoliccR = crate::BitReader;
#[doc = "Field `POLICC` writer - Polarity of Internal Compensation Capacitor"]
pub type PoliccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSEN` reader - DSEN Driven Shield Enable."]
pub type DsenR = crate::BitReader;
#[doc = "Field `DSEN` writer - DSEN Driven Shield Enable."]
pub type DsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSEQSTOP` reader - Stop DMA Sequence"]
pub type DseqstopR = crate::BitReader;
#[doc = "Field `DSEQSTOP` writer - Stop DMA Sequence"]
pub type DseqstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Digital Accumulation Number"]
    #[inline(always)]
    pub fn dan(&self) -> DanR {
        DanR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Analog Accumulation Number"]
    #[inline(always)]
    pub fn aan(&self) -> AanR {
        AanR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Window Comparator Mode"]
    #[inline(always)]
    pub fn wincm(&self) -> WincmR {
        WincmR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Window Single Sample"]
    #[inline(always)]
    pub fn winss(&self) -> WinssR {
        WinssR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - External Integration Capacitor Enable"]
    #[inline(always)]
    pub fn extcien(&self) -> ExtcienR {
        ExtcienR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Polarity of Internal Compensation Capacitor"]
    #[inline(always)]
    pub fn policc(&self) -> PoliccR {
        PoliccR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DSEN Driven Shield Enable."]
    #[inline(always)]
    pub fn dsen(&self) -> DsenR {
        DsenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - Stop DMA Sequence"]
    #[inline(always)]
    pub fn dseqstop(&self) -> DseqstopR {
        DseqstopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Digital Accumulation Number"]
    #[inline(always)]
    pub fn dan(&mut self) -> DanW<'_, CtrldSpec> {
        DanW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Analog Accumulation Number"]
    #[inline(always)]
    pub fn aan(&mut self) -> AanW<'_, CtrldSpec> {
        AanW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Window Comparator Mode"]
    #[inline(always)]
    pub fn wincm(&mut self) -> WincmW<'_, CtrldSpec> {
        WincmW::new(self, 16)
    }
    #[doc = "Bit 19 - Window Single Sample"]
    #[inline(always)]
    pub fn winss(&mut self) -> WinssW<'_, CtrldSpec> {
        WinssW::new(self, 19)
    }
    #[doc = "Bits 20:21 - External Integration Capacitor Enable"]
    #[inline(always)]
    pub fn extcien(&mut self) -> ExtcienW<'_, CtrldSpec> {
        ExtcienW::new(self, 20)
    }
    #[doc = "Bit 23 - Polarity of Internal Compensation Capacitor"]
    #[inline(always)]
    pub fn policc(&mut self) -> PoliccW<'_, CtrldSpec> {
        PoliccW::new(self, 23)
    }
    #[doc = "Bit 24 - DSEN Driven Shield Enable."]
    #[inline(always)]
    pub fn dsen(&mut self) -> DsenW<'_, CtrldSpec> {
        DsenW::new(self, 24)
    }
    #[doc = "Bit 31 - Stop DMA Sequence"]
    #[inline(always)]
    pub fn dseqstop(&mut self) -> DseqstopW<'_, CtrldSpec> {
        DseqstopW::new(self, 31)
    }
}
#[doc = "CONTROL D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrldSpec;
impl crate::RegisterSpec for CtrldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrld::R`](R) reader structure"]
impl crate::Readable for CtrldSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrld::W`](W) writer structure"]
impl crate::Writable for CtrldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for CtrldSpec {}

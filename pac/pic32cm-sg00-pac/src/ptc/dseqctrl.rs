#[doc = "Register `DSEQCTRL` reader"]
pub type R = crate::R<DseqctrlSpec>;
#[doc = "Register `DSEQCTRL` writer"]
pub type W = crate::W<DseqctrlSpec>;
#[doc = "Field `CTRLD` reader - Control D."]
pub type CtrldR = crate::BitReader;
#[doc = "Field `CTRLD` writer - Control D."]
pub type CtrldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINLT` reader - Window Comparator Low Threshold"]
pub type WinltR = crate::BitReader;
#[doc = "Field `WINLT` writer - Window Comparator Low Threshold"]
pub type WinltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINHT` reader - Window Comparator High Threshold"]
pub type WinhtR = crate::BitReader;
#[doc = "Field `WINHT` writer - Window Comparator High Threshold"]
pub type WinhtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XSEL` reader - X-line n Selection lower/upper half word"]
pub type XselR = crate::FieldReader;
#[doc = "Field `XSEL` writer - X-line n Selection lower/upper half word"]
pub type XselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YSEL` reader - Y-line n Selection lower/uper half word"]
pub type YselR = crate::FieldReader;
#[doc = "Field `YSEL` writer - Y-line n Selection lower/uper half word"]
pub type YselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `POL` reader - Polarity n lower/upper half word"]
pub type PolR = crate::FieldReader;
#[doc = "Field `POL` writer - Polarity n lower/upper half word"]
pub type PolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC` reader - Compensation Capacitor"]
pub type CcR = crate::BitReader;
#[doc = "Field `CC` writer - Compensation Capacitor"]
pub type CcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CI` reader - Integration Capacitor"]
pub type CiR = crate::BitReader;
#[doc = "Field `CI` writer - Integration Capacitor"]
pub type CiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSD` reader - Charge Share Delay"]
pub type CsdR = crate::BitReader;
#[doc = "Field `CSD` writer - Charge Share Delay"]
pub type CsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSTART` reader - PTC Auto-Start Conversion."]
pub type AutostartR = crate::BitReader;
#[doc = "Field `AUTOSTART` writer - PTC Auto-Start Conversion."]
pub type AutostartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Control D."]
    #[inline(always)]
    pub fn ctrld(&self) -> CtrldR {
        CtrldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Window Comparator Low Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WinltR {
        WinltR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Comparator High Threshold"]
    #[inline(always)]
    pub fn winht(&self) -> WinhtR {
        WinhtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - X-line n Selection lower/upper half word"]
    #[inline(always)]
    pub fn xsel(&self) -> XselR {
        XselR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - Y-line n Selection lower/uper half word"]
    #[inline(always)]
    pub fn ysel(&self) -> YselR {
        YselR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - Polarity n lower/upper half word"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Compensation Capacitor"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Integration Capacitor"]
    #[inline(always)]
    pub fn ci(&self) -> CiR {
        CiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Charge Share Delay"]
    #[inline(always)]
    pub fn csd(&self) -> CsdR {
        CsdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - PTC Auto-Start Conversion."]
    #[inline(always)]
    pub fn autostart(&self) -> AutostartR {
        AutostartR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control D."]
    #[inline(always)]
    pub fn ctrld(&mut self) -> CtrldW<'_, DseqctrlSpec> {
        CtrldW::new(self, 0)
    }
    #[doc = "Bit 1 - Window Comparator Low Threshold"]
    #[inline(always)]
    pub fn winlt(&mut self) -> WinltW<'_, DseqctrlSpec> {
        WinltW::new(self, 1)
    }
    #[doc = "Bit 2 - Window Comparator High Threshold"]
    #[inline(always)]
    pub fn winht(&mut self) -> WinhtW<'_, DseqctrlSpec> {
        WinhtW::new(self, 2)
    }
    #[doc = "Bits 3:6 - X-line n Selection lower/upper half word"]
    #[inline(always)]
    pub fn xsel(&mut self) -> XselW<'_, DseqctrlSpec> {
        XselW::new(self, 3)
    }
    #[doc = "Bits 7:10 - Y-line n Selection lower/uper half word"]
    #[inline(always)]
    pub fn ysel(&mut self) -> YselW<'_, DseqctrlSpec> {
        YselW::new(self, 7)
    }
    #[doc = "Bits 11:14 - Polarity n lower/upper half word"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, DseqctrlSpec> {
        PolW::new(self, 11)
    }
    #[doc = "Bit 15 - Compensation Capacitor"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<'_, DseqctrlSpec> {
        CcW::new(self, 15)
    }
    #[doc = "Bit 16 - Integration Capacitor"]
    #[inline(always)]
    pub fn ci(&mut self) -> CiW<'_, DseqctrlSpec> {
        CiW::new(self, 16)
    }
    #[doc = "Bit 17 - Charge Share Delay"]
    #[inline(always)]
    pub fn csd(&mut self) -> CsdW<'_, DseqctrlSpec> {
        CsdW::new(self, 17)
    }
    #[doc = "Bit 31 - PTC Auto-Start Conversion."]
    #[inline(always)]
    pub fn autostart(&mut self) -> AutostartW<'_, DseqctrlSpec> {
        AutostartW::new(self, 31)
    }
}
#[doc = "DMA SEQUENCE CTRL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dseqctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dseqctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DseqctrlSpec;
impl crate::RegisterSpec for DseqctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dseqctrl::R`](R) reader structure"]
impl crate::Readable for DseqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dseqctrl::W`](W) writer structure"]
impl crate::Writable for DseqctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSEQCTRL to value 0"]
impl crate::Resettable for DseqctrlSpec {}

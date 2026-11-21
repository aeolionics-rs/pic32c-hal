#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `CTSM` reader - Capacitive Touch Sensing Mode"]
pub type CtsmR = crate::FieldReader;
#[doc = "Field `CTSM` writer - Capacitive Touch Sensing Mode"]
pub type CtsmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INTSEL` reader - Interrupt Selection"]
pub type IntselR = crate::BitReader;
#[doc = "Field `INTSEL` writer - Interrupt Selection"]
pub type IntselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDS` reader - Sampling Delay Selection."]
pub type SdsR = crate::FieldReader;
#[doc = "Field `SDS` writer - Sampling Delay Selection."]
pub type SdsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ASDV` reader - Automatic Sampling Delay Variation."]
pub type AsdvR = crate::BitReader;
#[doc = "Field `ASDV` writer - Automatic Sampling Delay Variation."]
pub type AsdvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDS` reader - Channel Change Delay Selection"]
pub type CcdsR = crate::FieldReader;
#[doc = "Field `CCDS` writer - Channel Change Delay Selection"]
pub type CcdsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSEL` reader - Resistor Selection"]
pub type RselR = crate::FieldReader;
#[doc = "Field `RSEL` writer - Resistor Selection"]
pub type RselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Capacitive Touch Sensing Mode"]
    #[inline(always)]
    pub fn ctsm(&self) -> CtsmR {
        CtsmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&self) -> IntselR {
        IntselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Sampling Delay Selection."]
    #[inline(always)]
    pub fn sds(&self) -> SdsR {
        SdsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Automatic Sampling Delay Variation."]
    #[inline(always)]
    pub fn asdv(&self) -> AsdvR {
        AsdvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Channel Change Delay Selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CcdsR {
        CcdsR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Resistor Selection"]
    #[inline(always)]
    pub fn rsel(&self) -> RselR {
        RselR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Capacitive Touch Sensing Mode"]
    #[inline(always)]
    pub fn ctsm(&mut self) -> CtsmW<'_, CtrlbSpec> {
        CtsmW::new(self, 0)
    }
    #[doc = "Bit 4 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&mut self) -> IntselW<'_, CtrlbSpec> {
        IntselW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Sampling Delay Selection."]
    #[inline(always)]
    pub fn sds(&mut self) -> SdsW<'_, CtrlbSpec> {
        SdsW::new(self, 8)
    }
    #[doc = "Bit 12 - Automatic Sampling Delay Variation."]
    #[inline(always)]
    pub fn asdv(&mut self) -> AsdvW<'_, CtrlbSpec> {
        AsdvW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Channel Change Delay Selection"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CcdsW<'_, CtrlbSpec> {
        CcdsW::new(self, 13)
    }
    #[doc = "Bits 20:22 - Resistor Selection"]
    #[inline(always)]
    pub fn rsel(&mut self) -> RselW<'_, CtrlbSpec> {
        RselW::new(self, 20)
    }
}
#[doc = "CONTROL B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}

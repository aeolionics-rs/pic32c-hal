#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckselselect {
    #[doc = "0: The EIC is clocked by GCLK_EIC"]
    GclkEic = 0,
    #[doc = "1: The EIC is clocked by CLK_OSC32K"]
    ClkOsc32k = 1,
}
impl From<Ckselselect> for bool {
    #[inline(always)]
    fn from(variant: Ckselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKSEL` reader - Clock Selection"]
pub type CkselR = crate::BitReader<Ckselselect>;
impl CkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckselselect {
        match self.bits {
            false => Ckselselect::GclkEic,
            true => Ckselselect::ClkOsc32k,
        }
    }
    #[doc = "The EIC is clocked by GCLK_EIC"]
    #[inline(always)]
    pub fn is_gclk_eic(&self) -> bool {
        *self == Ckselselect::GclkEic
    }
    #[doc = "The EIC is clocked by CLK_OSC32K"]
    #[inline(always)]
    pub fn is_clk_osc32k(&self) -> bool {
        *self == Ckselselect::ClkOsc32k
    }
}
#[doc = "Field `CKSEL` writer - Clock Selection"]
pub type CkselW<'a, REG> = crate::BitWriter<'a, REG, Ckselselect>;
impl<'a, REG> CkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The EIC is clocked by GCLK_EIC"]
    #[inline(always)]
    pub fn gclk_eic(self) -> &'a mut crate::W<REG> {
        self.variant(Ckselselect::GclkEic)
    }
    #[doc = "The EIC is clocked by CLK_OSC32K"]
    #[inline(always)]
    pub fn clk_osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Ckselselect::ClkOsc32k)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Selection"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 4 - Clock Selection"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CkselW<'_, CtrlaSpec> {
        CkselW::new(self, 4)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}

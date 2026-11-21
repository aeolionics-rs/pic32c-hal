#[doc = "Register `PINCFG[%s]` reader"]
pub type R = crate::R<PincfgSpec>;
#[doc = "Register `PINCFG[%s]` writer"]
pub type W = crate::W<PincfgSpec>;
#[doc = "Field `PMUXEN` reader - Peripheral Multiplexer Enable"]
pub type PmuxenR = crate::BitReader;
#[doc = "Field `PMUXEN` writer - Peripheral Multiplexer Enable"]
pub type PmuxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN` reader - Input Buffer Enable"]
pub type InenR = crate::BitReader;
#[doc = "Field `INEN` writer - Input Buffer Enable"]
pub type InenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULLEN` reader - Pull Enable"]
pub type PullenR = crate::BitReader;
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub type PullenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEWLIM` reader - Output Driver Slew Rate Limit Enable"]
pub type SlewlimR = crate::BitReader;
#[doc = "Field `SLEWLIM` writer - Output Driver Slew Rate Limit Enable"]
pub type SlewlimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&self) -> PmuxenR {
        PmuxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Buffer Enable"]
    #[inline(always)]
    pub fn inen(&self) -> InenR {
        InenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&self) -> PullenR {
        PullenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Output Driver Slew Rate Limit Enable"]
    #[inline(always)]
    pub fn slewlim(&self) -> SlewlimR {
        SlewlimR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PmuxenW<'_, PincfgSpec> {
        PmuxenW::new(self, 0)
    }
    #[doc = "Bit 1 - Input Buffer Enable"]
    #[inline(always)]
    pub fn inen(&mut self) -> InenW<'_, PincfgSpec> {
        InenW::new(self, 1)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&mut self) -> PullenW<'_, PincfgSpec> {
        PullenW::new(self, 2)
    }
    #[doc = "Bit 4 - Output Driver Slew Rate Limit Enable"]
    #[inline(always)]
    pub fn slewlim(&mut self) -> SlewlimW<'_, PincfgSpec> {
        SlewlimW::new(self, 4)
    }
}
#[doc = "Pin Configuration #\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PincfgSpec;
impl crate::RegisterSpec for PincfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pincfg::R`](R) reader structure"]
impl crate::Readable for PincfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pincfg::W`](W) writer structure"]
impl crate::Writable for PincfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINCFG[%s] to value 0"]
impl crate::Resettable for PincfgSpec {}

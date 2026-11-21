#[doc = "Register `INTFLAGSET` reader"]
pub type R = crate::R<IntflagsetSpec>;
#[doc = "Register `INTFLAGSET` writer"]
pub type W = crate::W<IntflagsetSpec>;
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready"]
pub type Xosc32krdyR = crate::BitReader;
#[doc = "Field `XOSC32KRDY` writer - XOSC32K Ready"]
pub type Xosc32krdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OSC32KRDY` reader - OSC32K Ready"]
pub type Osc32krdyR = crate::BitReader;
#[doc = "Field `OSC32KRDY` writer - OSC32K Ready"]
pub type Osc32krdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLKFAIL` reader - XOSC32K Clock Failure Detected"]
pub type ClkfailR = crate::BitReader;
#[doc = "Field `CLKFAIL` writer - XOSC32K Clock Failure Detected"]
pub type ClkfailW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> Xosc32krdyR {
        Xosc32krdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OSC32K Ready"]
    #[inline(always)]
    pub fn osc32krdy(&self) -> Osc32krdyR {
        Osc32krdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detected"]
    #[inline(always)]
    pub fn clkfail(&self) -> ClkfailR {
        ClkfailR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&mut self) -> Xosc32krdyW<'_, IntflagsetSpec> {
        Xosc32krdyW::new(self, 0)
    }
    #[doc = "Bit 1 - OSC32K Ready"]
    #[inline(always)]
    pub fn osc32krdy(&mut self) -> Osc32krdyW<'_, IntflagsetSpec> {
        Osc32krdyW::new(self, 1)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detected"]
    #[inline(always)]
    pub fn clkfail(&mut self) -> ClkfailW<'_, IntflagsetSpec> {
        ClkfailW::new(self, 2)
    }
}
#[doc = "Interrupt Flag Software Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagsetSpec;
impl crate::RegisterSpec for IntflagsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagset::R`](R) reader structure"]
impl crate::Readable for IntflagsetSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagset::W`](W) writer structure"]
impl crate::Writable for IntflagsetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets INTFLAGSET to value 0"]
impl crate::Resettable for IntflagsetSpec {}

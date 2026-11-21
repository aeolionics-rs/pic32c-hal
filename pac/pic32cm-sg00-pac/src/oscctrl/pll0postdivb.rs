#[doc = "Register `PLL0POSTDIVB` reader"]
pub type R = crate::R<Pll0postdivbSpec>;
#[doc = "Register `PLL0POSTDIVB` writer"]
pub type W = crate::W<Pll0postdivbSpec>;
#[doc = "Field `POSTDIV4` reader - PLL output 4 clock division factor"]
pub type Postdiv4R = crate::FieldReader;
#[doc = "Field `POSTDIV4` writer - PLL output 4 clock division factor"]
pub type Postdiv4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OUTEN4` reader - PLL output 4 enable"]
pub type Outen4R = crate::BitReader;
#[doc = "Field `OUTEN4` writer - PLL output 4 enable"]
pub type Outen4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - PLL output 4 clock division factor"]
    #[inline(always)]
    pub fn postdiv4(&self) -> Postdiv4R {
        Postdiv4R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - PLL output 4 enable"]
    #[inline(always)]
    pub fn outen4(&self) -> Outen4R {
        Outen4R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLL output 4 clock division factor"]
    #[inline(always)]
    pub fn postdiv4(&mut self) -> Postdiv4W<'_, Pll0postdivbSpec> {
        Postdiv4W::new(self, 0)
    }
    #[doc = "Bit 7 - PLL output 4 enable"]
    #[inline(always)]
    pub fn outen4(&mut self) -> Outen4W<'_, Pll0postdivbSpec> {
        Outen4W::new(self, 7)
    }
}
#[doc = "PLL output clock divider B\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0postdivb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0postdivb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0postdivbSpec;
impl crate::RegisterSpec for Pll0postdivbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0postdivb::R`](R) reader structure"]
impl crate::Readable for Pll0postdivbSpec {}
#[doc = "`write(|w| ..)` method takes [`pll0postdivb::W`](W) writer structure"]
impl crate::Writable for Pll0postdivbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL0POSTDIVB to value 0x20"]
impl crate::Resettable for Pll0postdivbSpec {
    const RESET_VALUE: u32 = 0x20;
}

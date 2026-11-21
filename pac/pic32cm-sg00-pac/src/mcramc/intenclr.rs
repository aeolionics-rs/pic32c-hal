#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `SERREN` reader - Single Bit Error Interrupt Enable, Clear"]
pub type SerrenR = crate::BitReader;
#[doc = "Field `SERREN` writer - Single Bit Error Interrupt Enable, Clear"]
pub type SerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DERREN` reader - Double Bit Error Interrupt Enable, Clear"]
pub type DerrenR = crate::BitReader;
#[doc = "Field `DERREN` writer - Double Bit Error Interrupt Enable, Clear"]
pub type DerrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single Bit Error Interrupt Enable, Clear"]
    #[inline(always)]
    pub fn serren(&self) -> SerrenR {
        SerrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Double Bit Error Interrupt Enable, Clear"]
    #[inline(always)]
    pub fn derren(&self) -> DerrenR {
        DerrenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit Error Interrupt Enable, Clear"]
    #[inline(always)]
    pub fn serren(&mut self) -> SerrenW<'_, IntenclrSpec> {
        SerrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Double Bit Error Interrupt Enable, Clear"]
    #[inline(always)]
    pub fn derren(&mut self) -> DerrenW<'_, IntenclrSpec> {
        DerrenW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}

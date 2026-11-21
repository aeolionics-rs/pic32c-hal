#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `EOC` reader - End of Conversion Enable Clear."]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - End of Conversion Enable Clear."]
pub type EocW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WCOMP` reader - Window Comparator Enable Clear."]
pub type WcompR = crate::BitReader;
#[doc = "Field `WCOMP` writer - Window Comparator Enable Clear."]
pub type WcompW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ACRRDY` reader - Analog Core Ready Enable Clear."]
pub type AcrrdyR = crate::BitReader;
#[doc = "Field `ACRRDY` writer - Analog Core Ready Enable Clear."]
pub type AcrrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of Conversion Enable Clear."]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Window Comparator Enable Clear."]
    #[inline(always)]
    pub fn wcomp(&self) -> WcompR {
        WcompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Core Ready Enable Clear."]
    #[inline(always)]
    pub fn acrrdy(&self) -> AcrrdyR {
        AcrrdyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of Conversion Enable Clear."]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<'_, IntenclrSpec> {
        EocW::new(self, 0)
    }
    #[doc = "Bit 1 - Window Comparator Enable Clear."]
    #[inline(always)]
    pub fn wcomp(&mut self) -> WcompW<'_, IntenclrSpec> {
        WcompW::new(self, 1)
    }
    #[doc = "Bit 7 - Analog Core Ready Enable Clear."]
    #[inline(always)]
    pub fn acrrdy(&mut self) -> AcrrdyW<'_, IntenclrSpec> {
        AcrrdyW::new(self, 7)
    }
}
#[doc = "INTERRUPT ENABLE CLEAR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x83;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}

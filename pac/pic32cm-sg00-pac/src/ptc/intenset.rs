#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `EOC` reader - End of Conversion Enable Set."]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - End of Conversion Enable Set."]
pub type EocW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `WCOMP` reader - Window Comparator Enable Set."]
pub type WcompR = crate::BitReader;
#[doc = "Field `WCOMP` writer - Window Comparator Enable Set."]
pub type WcompW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ACRRDY` reader - Analog Core Ready Enable Set."]
pub type AcrrdyR = crate::BitReader;
#[doc = "Field `ACRRDY` writer - Analog Core Ready Enable Set."]
pub type AcrrdyW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of Conversion Enable Set."]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Window Comparator Enable Set."]
    #[inline(always)]
    pub fn wcomp(&self) -> WcompR {
        WcompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Core Ready Enable Set."]
    #[inline(always)]
    pub fn acrrdy(&self) -> AcrrdyR {
        AcrrdyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of Conversion Enable Set."]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<'_, IntensetSpec> {
        EocW::new(self, 0)
    }
    #[doc = "Bit 1 - Window Comparator Enable Set."]
    #[inline(always)]
    pub fn wcomp(&mut self) -> WcompW<'_, IntensetSpec> {
        WcompW::new(self, 1)
    }
    #[doc = "Bit 7 - Analog Core Ready Enable Set."]
    #[inline(always)]
    pub fn acrrdy(&mut self) -> AcrrdyW<'_, IntensetSpec> {
        AcrrdyW::new(self, 7)
    }
}
#[doc = "INTERRUPT ENABLE SET Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x83;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}

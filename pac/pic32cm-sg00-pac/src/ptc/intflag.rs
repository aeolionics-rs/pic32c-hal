#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `EOC` reader - End of Conversion Flag"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - End of Conversion Flag"]
pub type EocW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WCOMP` reader - Window Comparator Flag"]
pub type WcompR = crate::BitReader;
#[doc = "Field `WCOMP` writer - Window Comparator Flag"]
pub type WcompW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ACRRDY` reader - Analog Core Ready Flag"]
pub type AcrrdyR = crate::BitReader;
#[doc = "Field `ACRRDY` writer - Analog Core Ready Flag"]
pub type AcrrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of Conversion Flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Window Comparator Flag"]
    #[inline(always)]
    pub fn wcomp(&self) -> WcompR {
        WcompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Core Ready Flag"]
    #[inline(always)]
    pub fn acrrdy(&self) -> AcrrdyR {
        AcrrdyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of Conversion Flag"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<'_, IntflagSpec> {
        EocW::new(self, 0)
    }
    #[doc = "Bit 1 - Window Comparator Flag"]
    #[inline(always)]
    pub fn wcomp(&mut self) -> WcompW<'_, IntflagSpec> {
        WcompW::new(self, 1)
    }
    #[doc = "Bit 7 - Analog Core Ready Flag"]
    #[inline(always)]
    pub fn acrrdy(&mut self) -> AcrrdyW<'_, IntflagSpec> {
        AcrrdyW::new(self, 7)
    }
}
#[doc = "INTERRUPT FLAG STATUS and CLEAR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x83;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}

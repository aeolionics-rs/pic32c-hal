#[doc = "Register `DFLLMUL` reader"]
pub type R = crate::R<DfllmulSpec>;
#[doc = "Register `DFLLMUL` writer"]
pub type W = crate::W<DfllmulSpec>;
#[doc = "Field `MUL` reader - Multiplication Value"]
pub type MulR = crate::FieldReader<u16>;
#[doc = "Field `MUL` writer - Multiplication Value"]
pub type MulW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FSTEP` reader - Maximum Fine Step Size"]
pub type FstepR = crate::FieldReader<u16>;
#[doc = "Field `FSTEP` writer - Maximum Fine Step Size"]
pub type FstepW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CSTEP` reader - Maximum Coarse Step Size"]
pub type CstepR = crate::FieldReader;
#[doc = "Field `CSTEP` writer - Maximum Coarse Step Size"]
pub type CstepW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - Multiplication Value"]
    #[inline(always)]
    pub fn mul(&self) -> MulR {
        MulR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Maximum Fine Step Size"]
    #[inline(always)]
    pub fn fstep(&self) -> FstepR {
        FstepR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - Maximum Coarse Step Size"]
    #[inline(always)]
    pub fn cstep(&self) -> CstepR {
        CstepR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Multiplication Value"]
    #[inline(always)]
    pub fn mul(&mut self) -> MulW<'_, DfllmulSpec> {
        MulW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Maximum Fine Step Size"]
    #[inline(always)]
    pub fn fstep(&mut self) -> FstepW<'_, DfllmulSpec> {
        FstepW::new(self, 16)
    }
    #[doc = "Bits 26:31 - Maximum Coarse Step Size"]
    #[inline(always)]
    pub fn cstep(&mut self) -> CstepW<'_, DfllmulSpec> {
        CstepW::new(self, 26)
    }
}
#[doc = "DFLL Multiplier\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllmul::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllmul::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfllmulSpec;
impl crate::RegisterSpec for DfllmulSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfllmul::R`](R) reader structure"]
impl crate::Readable for DfllmulSpec {}
#[doc = "`write(|w| ..)` method takes [`dfllmul::W`](W) writer structure"]
impl crate::Writable for DfllmulSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFLLMUL to value 0"]
impl crate::Resettable for DfllmulSpec {}

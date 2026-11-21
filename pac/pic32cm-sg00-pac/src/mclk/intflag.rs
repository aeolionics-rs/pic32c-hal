#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `CKRDY` writer - Clock Ready Interrupt Flag"]
pub type CkrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clock Ready Interrupt Flag"]
    #[inline(always)]
    pub fn ckrdy(&mut self) -> CkrdyW<'_, IntflagSpec> {
        CkrdyW::new(self, 0)
    }
}
#[doc = "Interrupt Flag Status Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets INTFLAG to value 0x01"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u32 = 0x01;
}

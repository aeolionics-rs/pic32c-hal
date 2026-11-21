#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Clock Ready Interrupt Enable Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckrdyselect {
    #[doc = "0: The Clock Ready interrupt is disabled"]
    Ckrdy0 = 0,
    #[doc = "1: The Clock Ready interrupt is enabled."]
    Ckrdy1 = 1,
}
impl From<Ckrdyselect> for bool {
    #[inline(always)]
    fn from(variant: Ckrdyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKRDY` reader - Clock Ready Interrupt Enable Clear"]
pub type CkrdyR = crate::BitReader<Ckrdyselect>;
impl CkrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckrdyselect {
        match self.bits {
            false => Ckrdyselect::Ckrdy0,
            true => Ckrdyselect::Ckrdy1,
        }
    }
    #[doc = "The Clock Ready interrupt is disabled"]
    #[inline(always)]
    pub fn is_ckrdy_0(&self) -> bool {
        *self == Ckrdyselect::Ckrdy0
    }
    #[doc = "The Clock Ready interrupt is enabled."]
    #[inline(always)]
    pub fn is_ckrdy_1(&self) -> bool {
        *self == Ckrdyselect::Ckrdy1
    }
}
#[doc = "Field `CKRDY` writer - Clock Ready Interrupt Enable Clear"]
pub type CkrdyW<'a, REG> = crate::BitWriter1C<'a, REG, Ckrdyselect>;
impl<'a, REG> CkrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Clock Ready interrupt is disabled"]
    #[inline(always)]
    pub fn ckrdy_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckrdyselect::Ckrdy0)
    }
    #[doc = "The Clock Ready interrupt is enabled."]
    #[inline(always)]
    pub fn ckrdy_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckrdyselect::Ckrdy1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Ready Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ckrdy(&self) -> CkrdyR {
        CkrdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Ready Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ckrdy(&mut self) -> CkrdyW<'_, IntenclrSpec> {
        CkrdyW::new(self, 0)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}

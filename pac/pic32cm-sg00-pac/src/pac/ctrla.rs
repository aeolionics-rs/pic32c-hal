#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Privileged Access Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Privselect {
    #[doc = "0: Macro register accessible in privileged and unprivileged accesses."]
    Disable = 0,
    #[doc = "1: Macro registers only accessible in privileged accesses"]
    Enable = 1,
}
impl From<Privselect> for bool {
    #[inline(always)]
    fn from(variant: Privselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV` reader - Privileged Access Only"]
pub type PrivR = crate::BitReader<Privselect>;
impl PrivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Privselect {
        match self.bits {
            false => Privselect::Disable,
            true => Privselect::Enable,
        }
    }
    #[doc = "Macro register accessible in privileged and unprivileged accesses."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Privselect::Disable
    }
    #[doc = "Macro registers only accessible in privileged accesses"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Privselect::Enable
    }
}
#[doc = "Field `PRIV` writer - Privileged Access Only"]
pub type PrivW<'a, REG> = crate::BitWriter<'a, REG, Privselect>;
impl<'a, REG> PrivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Macro register accessible in privileged and unprivileged accesses."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Disable)
    }
    #[doc = "Macro registers only accessible in privileged accesses"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Privselect::Enable)
    }
}
impl R {
    #[doc = "Bit 2 - Privileged Access Only"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Privileged Access Only"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PrivW<'_, CtrlaSpec> {
        PrivW::new(self, 2)
    }
}
#[doc = "Control A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}

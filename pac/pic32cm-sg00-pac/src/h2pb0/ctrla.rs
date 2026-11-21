#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `PRIV` reader - PRIV privileged access protection bit"]
pub type PrivR = crate::BitReader;
#[doc = "Field `PRIV` writer - PRIV privileged access protection bit"]
pub type PrivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECEN` reader - SECEN security enable bit"]
pub type SecenR = crate::BitReader;
#[doc = "Field `SECEN` writer - SECEN security enable bit"]
pub type SecenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - PRIV privileged access protection bit"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SECEN security enable bit"]
    #[inline(always)]
    pub fn secen(&self) -> SecenR {
        SecenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PRIV privileged access protection bit"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PrivW<'_, CtrlaSpec> {
        PrivW::new(self, 2)
    }
    #[doc = "Bit 3 - SECEN security enable bit"]
    #[inline(always)]
    pub fn secen(&mut self) -> SecenW<'_, CtrlaSpec> {
        SecenW::new(self, 3)
    }
}
#[doc = "H2PB CONTROL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

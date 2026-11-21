#[doc = "Register `INTFLAGB` reader"]
pub type R = crate::R<IntflagbSpec>;
#[doc = "Register `INTFLAGB` writer"]
pub type W = crate::W<IntflagbSpec>;
#[doc = "Field `PORT_` reader - PORT"]
pub type Port_R = crate::BitReader;
#[doc = "Field `PORT_` writer - PORT"]
pub type Port_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DSU_` reader - DSU"]
pub type Dsu_R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU"]
pub type Dsu_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL"]
pub type Nvmctrl_R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL"]
pub type Nvmctrl_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DMAC_` reader - DMAC"]
pub type Dmac_R = crate::BitReader;
#[doc = "Field `DMAC_` writer - DMAC"]
pub type Dmac_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MTB_` reader - MTB"]
pub type Mtb_R = crate::BitReader;
#[doc = "Field `MTB_` writer - MTB"]
pub type Mtb_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HMATRIXHS_` reader - HMATRIXHS"]
pub type Hmatrixhs_R = crate::BitReader;
#[doc = "Field `HMATRIXHS_` writer - HMATRIXHS"]
pub type Hmatrixhs_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - PORT"]
    #[inline(always)]
    pub fn port_(&self) -> Port_R {
        Port_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    pub fn dsu_(&self) -> Dsu_R {
        Dsu_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> Nvmctrl_R {
        Nvmctrl_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC"]
    #[inline(always)]
    pub fn dmac_(&self) -> Dmac_R {
        Dmac_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MTB"]
    #[inline(always)]
    pub fn mtb_(&self) -> Mtb_R {
        Mtb_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HMATRIXHS"]
    #[inline(always)]
    pub fn hmatrixhs_(&self) -> Hmatrixhs_R {
        Hmatrixhs_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORT"]
    #[inline(always)]
    pub fn port_(&mut self) -> Port_W<'_, IntflagbSpec> {
        Port_W::new(self, 0)
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    pub fn dsu_(&mut self) -> Dsu_W<'_, IntflagbSpec> {
        Dsu_W::new(self, 1)
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    pub fn nvmctrl_(&mut self) -> Nvmctrl_W<'_, IntflagbSpec> {
        Nvmctrl_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMAC"]
    #[inline(always)]
    pub fn dmac_(&mut self) -> Dmac_W<'_, IntflagbSpec> {
        Dmac_W::new(self, 3)
    }
    #[doc = "Bit 4 - MTB"]
    #[inline(always)]
    pub fn mtb_(&mut self) -> Mtb_W<'_, IntflagbSpec> {
        Mtb_W::new(self, 4)
    }
    #[doc = "Bit 5 - HMATRIXHS"]
    #[inline(always)]
    pub fn hmatrixhs_(&mut self) -> Hmatrixhs_W<'_, IntflagbSpec> {
        Hmatrixhs_W::new(self, 5)
    }
}
#[doc = "Peripheral Interrupt Flag Status and Clear B\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagbSpec;
impl crate::RegisterSpec for IntflagbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagb::R`](R) reader structure"]
impl crate::Readable for IntflagbSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagb::W`](W) writer structure"]
impl crate::Writable for IntflagbSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets INTFLAGB to value 0"]
impl crate::Resettable for IntflagbSpec {}

#[doc = "Register `AHBMASK` reader"]
pub type R = crate::R<AhbmaskSpec>;
#[doc = "Register `AHBMASK` writer"]
pub type W = crate::W<AhbmaskSpec>;
#[doc = "Field `APBA_` reader - APBA AHB Clock Mask"]
pub type Apba_R = crate::BitReader;
#[doc = "Field `APBA_` writer - APBA AHB Clock Mask"]
pub type Apba_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBB_` reader - APBB AHB Clock Mask"]
pub type Apbb_R = crate::BitReader;
#[doc = "Field `APBB_` writer - APBB AHB Clock Mask"]
pub type Apbb_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBC_` reader - APBC AHB Clock Mask"]
pub type Apbc_R = crate::BitReader;
#[doc = "Field `APBC_` writer - APBC AHB Clock Mask"]
pub type Apbc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSU_` reader - DSU AHB Clock Mask"]
pub type Dsu_R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU AHB Clock Mask"]
pub type Dsu_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMATRIXHS_` reader - HMATRIXHS AHB Clock Mask"]
pub type Hmatrixhs_R = crate::BitReader;
#[doc = "Field `HMATRIXHS_` writer - HMATRIXHS AHB Clock Mask"]
pub type Hmatrixhs_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL AHB Clock Mask"]
pub type Nvmctrl_R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL AHB Clock Mask"]
pub type Nvmctrl_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRAM_` reader - HSRAM AHB Clock Mask"]
pub type Hsram_R = crate::BitReader;
#[doc = "Field `HSRAM_` writer - HSRAM AHB Clock Mask"]
pub type Hsram_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAC_` reader - DMAC AHB Clock Mask"]
pub type Dmac_R = crate::BitReader;
#[doc = "Field `DMAC_` writer - DMAC AHB Clock Mask"]
pub type Dmac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAC_` reader - PAC AHB Clock Mask"]
pub type Pac_R = crate::BitReader;
#[doc = "Field `PAC_` writer - PAC AHB Clock Mask"]
pub type Pac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROM_` reader - BROM AHB Clock Mask"]
pub type Brom_R = crate::BitReader;
#[doc = "Field `BROM_` writer - BROM AHB Clock Mask"]
pub type Brom_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - APBA AHB Clock Mask"]
    #[inline(always)]
    pub fn apba_(&self) -> Apba_R {
        Apba_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - APBB AHB Clock Mask"]
    #[inline(always)]
    pub fn apbb_(&self) -> Apbb_R {
        Apbb_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APBC AHB Clock Mask"]
    #[inline(always)]
    pub fn apbc_(&self) -> Apbc_R {
        Apbc_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&self) -> Dsu_R {
        Dsu_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HMATRIXHS AHB Clock Mask"]
    #[inline(always)]
    pub fn hmatrixhs_(&self) -> Hmatrixhs_R {
        Hmatrixhs_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> Nvmctrl_R {
        Nvmctrl_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_(&self) -> Hsram_R {
        Hsram_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&self) -> Dmac_R {
        Dmac_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&self) -> Pac_R {
        Pac_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BROM AHB Clock Mask"]
    #[inline(always)]
    pub fn brom_(&self) -> Brom_R {
        Brom_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - APBA AHB Clock Mask"]
    #[inline(always)]
    pub fn apba_(&mut self) -> Apba_W<'_, AhbmaskSpec> {
        Apba_W::new(self, 0)
    }
    #[doc = "Bit 1 - APBB AHB Clock Mask"]
    #[inline(always)]
    pub fn apbb_(&mut self) -> Apbb_W<'_, AhbmaskSpec> {
        Apbb_W::new(self, 1)
    }
    #[doc = "Bit 2 - APBC AHB Clock Mask"]
    #[inline(always)]
    pub fn apbc_(&mut self) -> Apbc_W<'_, AhbmaskSpec> {
        Apbc_W::new(self, 2)
    }
    #[doc = "Bit 3 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&mut self) -> Dsu_W<'_, AhbmaskSpec> {
        Dsu_W::new(self, 3)
    }
    #[doc = "Bit 4 - HMATRIXHS AHB Clock Mask"]
    #[inline(always)]
    pub fn hmatrixhs_(&mut self) -> Hmatrixhs_W<'_, AhbmaskSpec> {
        Hmatrixhs_W::new(self, 4)
    }
    #[doc = "Bit 5 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&mut self) -> Nvmctrl_W<'_, AhbmaskSpec> {
        Nvmctrl_W::new(self, 5)
    }
    #[doc = "Bit 6 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_(&mut self) -> Hsram_W<'_, AhbmaskSpec> {
        Hsram_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&mut self) -> Dmac_W<'_, AhbmaskSpec> {
        Dmac_W::new(self, 7)
    }
    #[doc = "Bit 8 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&mut self) -> Pac_W<'_, AhbmaskSpec> {
        Pac_W::new(self, 8)
    }
    #[doc = "Bit 9 - BROM AHB Clock Mask"]
    #[inline(always)]
    pub fn brom_(&mut self) -> Brom_W<'_, AhbmaskSpec> {
        Brom_W::new(self, 9)
    }
}
#[doc = "AHB Bridge Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmaskSpec;
impl crate::RegisterSpec for AhbmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmask::R`](R) reader structure"]
impl crate::Readable for AhbmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbmask::W`](W) writer structure"]
impl crate::Writable for AhbmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBMASK to value 0x03ff"]
impl crate::Resettable for AhbmaskSpec {
    const RESET_VALUE: u32 = 0x03ff;
}

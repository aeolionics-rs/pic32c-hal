#[doc = "Register `INTFLAGAHB` reader"]
pub type R = crate::R<IntflagahbSpec>;
#[doc = "Register `INTFLAGAHB` writer"]
pub type W = crate::W<IntflagahbSpec>;
#[doc = "Field `FLASH_` reader - FLASH"]
pub type Flash_R = crate::BitReader;
#[doc = "Field `FLASH_` writer - FLASH"]
pub type Flash_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HSRAMCM0P_` reader - HSRAMCM0P"]
pub type Hsramcm0p_R = crate::BitReader;
#[doc = "Field `HSRAMCM0P_` writer - HSRAMCM0P"]
pub type Hsramcm0p_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HSRAMDSU_` reader - HSRAMDSU"]
pub type Hsramdsu_R = crate::BitReader;
#[doc = "Field `HSRAMDSU_` writer - HSRAMDSU"]
pub type Hsramdsu_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `APBB_` reader - APBB"]
pub type Apbb_R = crate::BitReader;
#[doc = "Field `APBB_` writer - APBB"]
pub type Apbb_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `APBA_` reader - APBA"]
pub type Apba_R = crate::BitReader;
#[doc = "Field `APBA_` writer - APBA"]
pub type Apba_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `APBC_` reader - APBC"]
pub type Apbc_R = crate::BitReader;
#[doc = "Field `APBC_` writer - APBC"]
pub type Apbc_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SRAMDMAC_` reader - SRAMDMAC"]
pub type Sramdmac_R = crate::BitReader;
#[doc = "Field `SRAMDMAC_` writer - SRAMDMAC"]
pub type Sramdmac_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BROM_` reader - BROM"]
pub type Brom_R = crate::BitReader;
#[doc = "Field `BROM_` writer - BROM"]
pub type Brom_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&self) -> Flash_R {
        Flash_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    pub fn hsramcm0p_(&self) -> Hsramcm0p_R {
        Hsramcm0p_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    pub fn hsramdsu_(&self) -> Hsramdsu_R {
        Hsramdsu_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - APBB"]
    #[inline(always)]
    pub fn apbb_(&self) -> Apbb_R {
        Apbb_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APBA"]
    #[inline(always)]
    pub fn apba_(&self) -> Apba_R {
        Apba_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - APBC"]
    #[inline(always)]
    pub fn apbc_(&self) -> Apbc_R {
        Apbc_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAMDMAC"]
    #[inline(always)]
    pub fn sramdmac_(&self) -> Sramdmac_R {
        Sramdmac_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BROM"]
    #[inline(always)]
    pub fn brom_(&self) -> Brom_R {
        Brom_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&mut self) -> Flash_W<'_, IntflagahbSpec> {
        Flash_W::new(self, 0)
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    pub fn hsramcm0p_(&mut self) -> Hsramcm0p_W<'_, IntflagahbSpec> {
        Hsramcm0p_W::new(self, 1)
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    pub fn hsramdsu_(&mut self) -> Hsramdsu_W<'_, IntflagahbSpec> {
        Hsramdsu_W::new(self, 2)
    }
    #[doc = "Bit 3 - APBB"]
    #[inline(always)]
    pub fn apbb_(&mut self) -> Apbb_W<'_, IntflagahbSpec> {
        Apbb_W::new(self, 3)
    }
    #[doc = "Bit 4 - APBA"]
    #[inline(always)]
    pub fn apba_(&mut self) -> Apba_W<'_, IntflagahbSpec> {
        Apba_W::new(self, 4)
    }
    #[doc = "Bit 5 - APBC"]
    #[inline(always)]
    pub fn apbc_(&mut self) -> Apbc_W<'_, IntflagahbSpec> {
        Apbc_W::new(self, 5)
    }
    #[doc = "Bit 6 - SRAMDMAC"]
    #[inline(always)]
    pub fn sramdmac_(&mut self) -> Sramdmac_W<'_, IntflagahbSpec> {
        Sramdmac_W::new(self, 6)
    }
    #[doc = "Bit 7 - BROM"]
    #[inline(always)]
    pub fn brom_(&mut self) -> Brom_W<'_, IntflagahbSpec> {
        Brom_W::new(self, 7)
    }
}
#[doc = "AHB Client Bus Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagahb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagahb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagahbSpec;
impl crate::RegisterSpec for IntflagahbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagahb::R`](R) reader structure"]
impl crate::Readable for IntflagahbSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagahb::W`](W) writer structure"]
impl crate::Writable for IntflagahbSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets INTFLAGAHB to value 0"]
impl crate::Resettable for IntflagahbSpec {}

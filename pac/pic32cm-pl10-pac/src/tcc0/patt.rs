#[doc = "Register `PATT` reader"]
pub type R = crate::R<PattSpec>;
#[doc = "Register `PATT` writer"]
pub type W = crate::W<PattSpec>;
#[doc = "Field `PGE0` reader - Pattern Generation Output 0 Enable"]
pub type Pge0R = crate::BitReader;
#[doc = "Field `PGE0` writer - Pattern Generation Output 0 Enable"]
pub type Pge0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE1` reader - Pattern Generation Output 1 Enable"]
pub type Pge1R = crate::BitReader;
#[doc = "Field `PGE1` writer - Pattern Generation Output 1 Enable"]
pub type Pge1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE2` reader - Pattern Generation Output 2 Enable"]
pub type Pge2R = crate::BitReader;
#[doc = "Field `PGE2` writer - Pattern Generation Output 2 Enable"]
pub type Pge2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE3` reader - Pattern Generation Output 3 Enable"]
pub type Pge3R = crate::BitReader;
#[doc = "Field `PGE3` writer - Pattern Generation Output 3 Enable"]
pub type Pge3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV0` reader - Pattern Generation Output 0 Value"]
pub type Pgv0R = crate::BitReader;
#[doc = "Field `PGV0` writer - Pattern Generation Output 0 Value"]
pub type Pgv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV1` reader - Pattern Generation Output 1 Value"]
pub type Pgv1R = crate::BitReader;
#[doc = "Field `PGV1` writer - Pattern Generation Output 1 Value"]
pub type Pgv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV2` reader - Pattern Generation Output 2 Value"]
pub type Pgv2R = crate::BitReader;
#[doc = "Field `PGV2` writer - Pattern Generation Output 2 Value"]
pub type Pgv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV3` reader - Pattern Generation Output 3 Value"]
pub type Pgv3R = crate::BitReader;
#[doc = "Field `PGV3` writer - Pattern Generation Output 3 Value"]
pub type Pgv3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pattern Generation Output 0 Enable"]
    #[inline(always)]
    pub fn pge0(&self) -> Pge0R {
        Pge0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pattern Generation Output 1 Enable"]
    #[inline(always)]
    pub fn pge1(&self) -> Pge1R {
        Pge1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pattern Generation Output 2 Enable"]
    #[inline(always)]
    pub fn pge2(&self) -> Pge2R {
        Pge2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pattern Generation Output 3 Enable"]
    #[inline(always)]
    pub fn pge3(&self) -> Pge3R {
        Pge3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Pattern Generation Output 0 Value"]
    #[inline(always)]
    pub fn pgv0(&self) -> Pgv0R {
        Pgv0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pattern Generation Output 1 Value"]
    #[inline(always)]
    pub fn pgv1(&self) -> Pgv1R {
        Pgv1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pattern Generation Output 2 Value"]
    #[inline(always)]
    pub fn pgv2(&self) -> Pgv2R {
        Pgv2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pattern Generation Output 3 Value"]
    #[inline(always)]
    pub fn pgv3(&self) -> Pgv3R {
        Pgv3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern Generation Output 0 Enable"]
    #[inline(always)]
    pub fn pge0(&mut self) -> Pge0W<'_, PattSpec> {
        Pge0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pattern Generation Output 1 Enable"]
    #[inline(always)]
    pub fn pge1(&mut self) -> Pge1W<'_, PattSpec> {
        Pge1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pattern Generation Output 2 Enable"]
    #[inline(always)]
    pub fn pge2(&mut self) -> Pge2W<'_, PattSpec> {
        Pge2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pattern Generation Output 3 Enable"]
    #[inline(always)]
    pub fn pge3(&mut self) -> Pge3W<'_, PattSpec> {
        Pge3W::new(self, 3)
    }
    #[doc = "Bit 8 - Pattern Generation Output 0 Value"]
    #[inline(always)]
    pub fn pgv0(&mut self) -> Pgv0W<'_, PattSpec> {
        Pgv0W::new(self, 8)
    }
    #[doc = "Bit 9 - Pattern Generation Output 1 Value"]
    #[inline(always)]
    pub fn pgv1(&mut self) -> Pgv1W<'_, PattSpec> {
        Pgv1W::new(self, 9)
    }
    #[doc = "Bit 10 - Pattern Generation Output 2 Value"]
    #[inline(always)]
    pub fn pgv2(&mut self) -> Pgv2W<'_, PattSpec> {
        Pgv2W::new(self, 10)
    }
    #[doc = "Bit 11 - Pattern Generation Output 3 Value"]
    #[inline(always)]
    pub fn pgv3(&mut self) -> Pgv3W<'_, PattSpec> {
        Pgv3W::new(self, 11)
    }
}
#[doc = "Pattern\n\nYou can [`read`](crate::Reg::read) this register and get [`patt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PattSpec;
impl crate::RegisterSpec for PattSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`patt::R`](R) reader structure"]
impl crate::Readable for PattSpec {}
#[doc = "`write(|w| ..)` method takes [`patt::W`](W) writer structure"]
impl crate::Writable for PattSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PATT to value 0"]
impl crate::Resettable for PattSpec {}

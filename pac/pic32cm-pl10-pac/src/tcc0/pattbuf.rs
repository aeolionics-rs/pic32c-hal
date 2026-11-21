#[doc = "Register `PATTBUF` reader"]
pub type R = crate::R<PattbufSpec>;
#[doc = "Register `PATTBUF` writer"]
pub type W = crate::W<PattbufSpec>;
#[doc = "Field `PGEB0` reader - Pattern Generation Output Enable Buffer 0"]
pub type Pgeb0R = crate::BitReader;
#[doc = "Field `PGEB0` writer - Pattern Generation Output Enable Buffer 0"]
pub type Pgeb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGEB1` reader - Pattern Generation Output Enable Buffer 1"]
pub type Pgeb1R = crate::BitReader;
#[doc = "Field `PGEB1` writer - Pattern Generation Output Enable Buffer 1"]
pub type Pgeb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGEB2` reader - Pattern Generation Output Enable Buffer 2"]
pub type Pgeb2R = crate::BitReader;
#[doc = "Field `PGEB2` writer - Pattern Generation Output Enable Buffer 2"]
pub type Pgeb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGEB3` reader - Pattern Generation Output Enable Buffer 3"]
pub type Pgeb3R = crate::BitReader;
#[doc = "Field `PGEB3` writer - Pattern Generation Output Enable Buffer 3"]
pub type Pgeb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB0` reader - Pattern Generation Output Value Buffer 0"]
pub type Pgvb0R = crate::BitReader;
#[doc = "Field `PGVB0` writer - Pattern Generation Output Value Buffer 0"]
pub type Pgvb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB1` reader - Pattern Generation Output Value Buffer 1"]
pub type Pgvb1R = crate::BitReader;
#[doc = "Field `PGVB1` writer - Pattern Generation Output Value Buffer 1"]
pub type Pgvb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB2` reader - Pattern Generation Output Value Buffer 2"]
pub type Pgvb2R = crate::BitReader;
#[doc = "Field `PGVB2` writer - Pattern Generation Output Value Buffer 2"]
pub type Pgvb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB3` reader - Pattern Generation Output Value Buffer 3"]
pub type Pgvb3R = crate::BitReader;
#[doc = "Field `PGVB3` writer - Pattern Generation Output Value Buffer 3"]
pub type Pgvb3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pattern Generation Output Enable Buffer 0"]
    #[inline(always)]
    pub fn pgeb0(&self) -> Pgeb0R {
        Pgeb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pattern Generation Output Enable Buffer 1"]
    #[inline(always)]
    pub fn pgeb1(&self) -> Pgeb1R {
        Pgeb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pattern Generation Output Enable Buffer 2"]
    #[inline(always)]
    pub fn pgeb2(&self) -> Pgeb2R {
        Pgeb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pattern Generation Output Enable Buffer 3"]
    #[inline(always)]
    pub fn pgeb3(&self) -> Pgeb3R {
        Pgeb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Pattern Generation Output Value Buffer 0"]
    #[inline(always)]
    pub fn pgvb0(&self) -> Pgvb0R {
        Pgvb0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pattern Generation Output Value Buffer 1"]
    #[inline(always)]
    pub fn pgvb1(&self) -> Pgvb1R {
        Pgvb1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pattern Generation Output Value Buffer 2"]
    #[inline(always)]
    pub fn pgvb2(&self) -> Pgvb2R {
        Pgvb2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pattern Generation Output Value Buffer 3"]
    #[inline(always)]
    pub fn pgvb3(&self) -> Pgvb3R {
        Pgvb3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern Generation Output Enable Buffer 0"]
    #[inline(always)]
    pub fn pgeb0(&mut self) -> Pgeb0W<'_, PattbufSpec> {
        Pgeb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pattern Generation Output Enable Buffer 1"]
    #[inline(always)]
    pub fn pgeb1(&mut self) -> Pgeb1W<'_, PattbufSpec> {
        Pgeb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pattern Generation Output Enable Buffer 2"]
    #[inline(always)]
    pub fn pgeb2(&mut self) -> Pgeb2W<'_, PattbufSpec> {
        Pgeb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pattern Generation Output Enable Buffer 3"]
    #[inline(always)]
    pub fn pgeb3(&mut self) -> Pgeb3W<'_, PattbufSpec> {
        Pgeb3W::new(self, 3)
    }
    #[doc = "Bit 8 - Pattern Generation Output Value Buffer 0"]
    #[inline(always)]
    pub fn pgvb0(&mut self) -> Pgvb0W<'_, PattbufSpec> {
        Pgvb0W::new(self, 8)
    }
    #[doc = "Bit 9 - Pattern Generation Output Value Buffer 1"]
    #[inline(always)]
    pub fn pgvb1(&mut self) -> Pgvb1W<'_, PattbufSpec> {
        Pgvb1W::new(self, 9)
    }
    #[doc = "Bit 10 - Pattern Generation Output Value Buffer 2"]
    #[inline(always)]
    pub fn pgvb2(&mut self) -> Pgvb2W<'_, PattbufSpec> {
        Pgvb2W::new(self, 10)
    }
    #[doc = "Bit 11 - Pattern Generation Output Value Buffer 3"]
    #[inline(always)]
    pub fn pgvb3(&mut self) -> Pgvb3W<'_, PattbufSpec> {
        Pgvb3W::new(self, 11)
    }
}
#[doc = "Pattern Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`pattbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pattbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PattbufSpec;
impl crate::RegisterSpec for PattbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pattbuf::R`](R) reader structure"]
impl crate::Readable for PattbufSpec {}
#[doc = "`write(|w| ..)` method takes [`pattbuf::W`](W) writer structure"]
impl crate::Writable for PattbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PATTBUF to value 0"]
impl crate::Resettable for PattbufSpec {}

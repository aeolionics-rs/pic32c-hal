#[doc = "Register `ASYNCH` reader"]
pub type R = crate::R<AsynchSpec>;
#[doc = "Register `ASYNCH` writer"]
pub type W = crate::W<AsynchSpec>;
#[doc = "Field `ASYNCH0` reader - Asynchronous Edge Detection Mode 0"]
pub type Asynch0R = crate::BitReader;
#[doc = "Field `ASYNCH0` writer - Asynchronous Edge Detection Mode 0"]
pub type Asynch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH1` reader - Asynchronous Edge Detection Mode 1"]
pub type Asynch1R = crate::BitReader;
#[doc = "Field `ASYNCH1` writer - Asynchronous Edge Detection Mode 1"]
pub type Asynch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH2` reader - Asynchronous Edge Detection Mode 2"]
pub type Asynch2R = crate::BitReader;
#[doc = "Field `ASYNCH2` writer - Asynchronous Edge Detection Mode 2"]
pub type Asynch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH3` reader - Asynchronous Edge Detection Mode 3"]
pub type Asynch3R = crate::BitReader;
#[doc = "Field `ASYNCH3` writer - Asynchronous Edge Detection Mode 3"]
pub type Asynch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH4` reader - Asynchronous Edge Detection Mode 4"]
pub type Asynch4R = crate::BitReader;
#[doc = "Field `ASYNCH4` writer - Asynchronous Edge Detection Mode 4"]
pub type Asynch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH5` reader - Asynchronous Edge Detection Mode 5"]
pub type Asynch5R = crate::BitReader;
#[doc = "Field `ASYNCH5` writer - Asynchronous Edge Detection Mode 5"]
pub type Asynch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH6` reader - Asynchronous Edge Detection Mode 6"]
pub type Asynch6R = crate::BitReader;
#[doc = "Field `ASYNCH6` writer - Asynchronous Edge Detection Mode 6"]
pub type Asynch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH7` reader - Asynchronous Edge Detection Mode 7"]
pub type Asynch7R = crate::BitReader;
#[doc = "Field `ASYNCH7` writer - Asynchronous Edge Detection Mode 7"]
pub type Asynch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH8` reader - Asynchronous Edge Detection Mode 8"]
pub type Asynch8R = crate::BitReader;
#[doc = "Field `ASYNCH8` writer - Asynchronous Edge Detection Mode 8"]
pub type Asynch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH9` reader - Asynchronous Edge Detection Mode 9"]
pub type Asynch9R = crate::BitReader;
#[doc = "Field `ASYNCH9` writer - Asynchronous Edge Detection Mode 9"]
pub type Asynch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH10` reader - Asynchronous Edge Detection Mode 10"]
pub type Asynch10R = crate::BitReader;
#[doc = "Field `ASYNCH10` writer - Asynchronous Edge Detection Mode 10"]
pub type Asynch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH11` reader - Asynchronous Edge Detection Mode 11"]
pub type Asynch11R = crate::BitReader;
#[doc = "Field `ASYNCH11` writer - Asynchronous Edge Detection Mode 11"]
pub type Asynch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH12` reader - Asynchronous Edge Detection Mode 12"]
pub type Asynch12R = crate::BitReader;
#[doc = "Field `ASYNCH12` writer - Asynchronous Edge Detection Mode 12"]
pub type Asynch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH13` reader - Asynchronous Edge Detection Mode 13"]
pub type Asynch13R = crate::BitReader;
#[doc = "Field `ASYNCH13` writer - Asynchronous Edge Detection Mode 13"]
pub type Asynch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH14` reader - Asynchronous Edge Detection Mode 14"]
pub type Asynch14R = crate::BitReader;
#[doc = "Field `ASYNCH14` writer - Asynchronous Edge Detection Mode 14"]
pub type Asynch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH15` reader - Asynchronous Edge Detection Mode 15"]
pub type Asynch15R = crate::BitReader;
#[doc = "Field `ASYNCH15` writer - Asynchronous Edge Detection Mode 15"]
pub type Asynch15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Asynchronous Edge Detection Mode 0"]
    #[inline(always)]
    pub fn asynch0(&self) -> Asynch0R {
        Asynch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Asynchronous Edge Detection Mode 1"]
    #[inline(always)]
    pub fn asynch1(&self) -> Asynch1R {
        Asynch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Asynchronous Edge Detection Mode 2"]
    #[inline(always)]
    pub fn asynch2(&self) -> Asynch2R {
        Asynch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronous Edge Detection Mode 3"]
    #[inline(always)]
    pub fn asynch3(&self) -> Asynch3R {
        Asynch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous Edge Detection Mode 4"]
    #[inline(always)]
    pub fn asynch4(&self) -> Asynch4R {
        Asynch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Edge Detection Mode 5"]
    #[inline(always)]
    pub fn asynch5(&self) -> Asynch5R {
        Asynch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Asynchronous Edge Detection Mode 6"]
    #[inline(always)]
    pub fn asynch6(&self) -> Asynch6R {
        Asynch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Asynchronous Edge Detection Mode 7"]
    #[inline(always)]
    pub fn asynch7(&self) -> Asynch7R {
        Asynch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Asynchronous Edge Detection Mode 8"]
    #[inline(always)]
    pub fn asynch8(&self) -> Asynch8R {
        Asynch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Asynchronous Edge Detection Mode 9"]
    #[inline(always)]
    pub fn asynch9(&self) -> Asynch9R {
        Asynch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Asynchronous Edge Detection Mode 10"]
    #[inline(always)]
    pub fn asynch10(&self) -> Asynch10R {
        Asynch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Asynchronous Edge Detection Mode 11"]
    #[inline(always)]
    pub fn asynch11(&self) -> Asynch11R {
        Asynch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Asynchronous Edge Detection Mode 12"]
    #[inline(always)]
    pub fn asynch12(&self) -> Asynch12R {
        Asynch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Asynchronous Edge Detection Mode 13"]
    #[inline(always)]
    pub fn asynch13(&self) -> Asynch13R {
        Asynch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Asynchronous Edge Detection Mode 14"]
    #[inline(always)]
    pub fn asynch14(&self) -> Asynch14R {
        Asynch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Asynchronous Edge Detection Mode 15"]
    #[inline(always)]
    pub fn asynch15(&self) -> Asynch15R {
        Asynch15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asynchronous Edge Detection Mode 0"]
    #[inline(always)]
    pub fn asynch0(&mut self) -> Asynch0W<'_, AsynchSpec> {
        Asynch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Asynchronous Edge Detection Mode 1"]
    #[inline(always)]
    pub fn asynch1(&mut self) -> Asynch1W<'_, AsynchSpec> {
        Asynch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Asynchronous Edge Detection Mode 2"]
    #[inline(always)]
    pub fn asynch2(&mut self) -> Asynch2W<'_, AsynchSpec> {
        Asynch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Asynchronous Edge Detection Mode 3"]
    #[inline(always)]
    pub fn asynch3(&mut self) -> Asynch3W<'_, AsynchSpec> {
        Asynch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Asynchronous Edge Detection Mode 4"]
    #[inline(always)]
    pub fn asynch4(&mut self) -> Asynch4W<'_, AsynchSpec> {
        Asynch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Asynchronous Edge Detection Mode 5"]
    #[inline(always)]
    pub fn asynch5(&mut self) -> Asynch5W<'_, AsynchSpec> {
        Asynch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Asynchronous Edge Detection Mode 6"]
    #[inline(always)]
    pub fn asynch6(&mut self) -> Asynch6W<'_, AsynchSpec> {
        Asynch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Asynchronous Edge Detection Mode 7"]
    #[inline(always)]
    pub fn asynch7(&mut self) -> Asynch7W<'_, AsynchSpec> {
        Asynch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Asynchronous Edge Detection Mode 8"]
    #[inline(always)]
    pub fn asynch8(&mut self) -> Asynch8W<'_, AsynchSpec> {
        Asynch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Asynchronous Edge Detection Mode 9"]
    #[inline(always)]
    pub fn asynch9(&mut self) -> Asynch9W<'_, AsynchSpec> {
        Asynch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Asynchronous Edge Detection Mode 10"]
    #[inline(always)]
    pub fn asynch10(&mut self) -> Asynch10W<'_, AsynchSpec> {
        Asynch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Asynchronous Edge Detection Mode 11"]
    #[inline(always)]
    pub fn asynch11(&mut self) -> Asynch11W<'_, AsynchSpec> {
        Asynch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Asynchronous Edge Detection Mode 12"]
    #[inline(always)]
    pub fn asynch12(&mut self) -> Asynch12W<'_, AsynchSpec> {
        Asynch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Asynchronous Edge Detection Mode 13"]
    #[inline(always)]
    pub fn asynch13(&mut self) -> Asynch13W<'_, AsynchSpec> {
        Asynch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Asynchronous Edge Detection Mode 14"]
    #[inline(always)]
    pub fn asynch14(&mut self) -> Asynch14W<'_, AsynchSpec> {
        Asynch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Asynchronous Edge Detection Mode 15"]
    #[inline(always)]
    pub fn asynch15(&mut self) -> Asynch15W<'_, AsynchSpec> {
        Asynch15W::new(self, 15)
    }
}
#[doc = "External Interrupt Asynchronous Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`asynch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asynch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsynchSpec;
impl crate::RegisterSpec for AsynchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asynch::R`](R) reader structure"]
impl crate::Readable for AsynchSpec {}
#[doc = "`write(|w| ..)` method takes [`asynch::W`](W) writer structure"]
impl crate::Writable for AsynchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASYNCH to value 0"]
impl crate::Resettable for AsynchSpec {}

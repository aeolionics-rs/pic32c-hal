#[doc = "Register `MEIDR` writer"]
pub type W = crate::W<MeidrSpec>;
#[doc = "Field `MERR0` writer - Master 0 Access Error"]
pub type Merr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR1` writer - Master 1 Access Error"]
pub type Merr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR2` writer - Master 2 Access Error"]
pub type Merr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR3` writer - Master 3 Access Error"]
pub type Merr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR4` writer - Master 4 Access Error"]
pub type Merr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR5` writer - Master 5 Access Error"]
pub type Merr5W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Master 0 Access Error"]
    #[inline(always)]
    pub fn merr0(&mut self) -> Merr0W<'_, MeidrSpec> {
        Merr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Master 1 Access Error"]
    #[inline(always)]
    pub fn merr1(&mut self) -> Merr1W<'_, MeidrSpec> {
        Merr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Master 2 Access Error"]
    #[inline(always)]
    pub fn merr2(&mut self) -> Merr2W<'_, MeidrSpec> {
        Merr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Master 3 Access Error"]
    #[inline(always)]
    pub fn merr3(&mut self) -> Merr3W<'_, MeidrSpec> {
        Merr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Master 4 Access Error"]
    #[inline(always)]
    pub fn merr4(&mut self) -> Merr4W<'_, MeidrSpec> {
        Merr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Master 5 Access Error"]
    #[inline(always)]
    pub fn merr5(&mut self) -> Merr5W<'_, MeidrSpec> {
        Merr5W::new(self, 5)
    }
}
#[doc = "Master Error Interrupt Disable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MeidrSpec;
impl crate::RegisterSpec for MeidrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`meidr::W`](W) writer structure"]
impl crate::Writable for MeidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEIDR to value 0"]
impl crate::Resettable for MeidrSpec {}

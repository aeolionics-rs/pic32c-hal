#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `COMPEO0` reader - Comparator 0 Event Output Enable"]
pub type Compeo0R = crate::BitReader;
#[doc = "Field `COMPEO0` writer - Comparator 0 Event Output Enable"]
pub type Compeo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEO1` reader - Comparator 1 Event Output Enable"]
pub type Compeo1R = crate::BitReader;
#[doc = "Field `COMPEO1` writer - Comparator 1 Event Output Enable"]
pub type Compeo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINEO0` reader - Window 0 Event Output Enable"]
pub type Wineo0R = crate::BitReader;
#[doc = "Field `WINEO0` writer - Window 0 Event Output Enable"]
pub type Wineo0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&self) -> Compeo0R {
        Compeo0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&self) -> Compeo1R {
        Compeo1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&self) -> Wineo0R {
        Wineo0R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&mut self) -> Compeo0W<'_, EvctrlSpec> {
        Compeo0W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&mut self) -> Compeo1W<'_, EvctrlSpec> {
        Compeo1W::new(self, 1)
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&mut self) -> Wineo0W<'_, EvctrlSpec> {
        Wineo0W::new(self, 4)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {}

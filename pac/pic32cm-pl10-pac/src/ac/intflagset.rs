#[doc = "Register `INTFLAGSET` reader"]
pub type R = crate::R<IntflagsetSpec>;
#[doc = "Register `INTFLAGSET` writer"]
pub type W = crate::W<IntflagsetSpec>;
#[doc = "Field `COMP0` reader - Comparator 0 Interrupt Flag"]
pub type Comp0R = crate::BitReader;
#[doc = "Field `COMP0` writer - Comparator 0 Interrupt Flag"]
pub type Comp0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMP1` reader - Comparator 1 Interrupt Flag"]
pub type Comp1R = crate::BitReader;
#[doc = "Field `COMP1` writer - Comparator 1 Interrupt Flag"]
pub type Comp1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WIN0` reader - Window 0 Interrupt Flag"]
pub type Win0R = crate::BitReader;
#[doc = "Field `WIN0` writer - Window 0 Interrupt Flag"]
pub type Win0W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Interrupt Flag"]
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Flag"]
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Window 0 Interrupt Flag"]
    #[inline(always)]
    pub fn win0(&self) -> Win0R {
        Win0R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Interrupt Flag"]
    #[inline(always)]
    pub fn comp0(&mut self) -> Comp0W<'_, IntflagsetSpec> {
        Comp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Flag"]
    #[inline(always)]
    pub fn comp1(&mut self) -> Comp1W<'_, IntflagsetSpec> {
        Comp1W::new(self, 1)
    }
    #[doc = "Bit 4 - Window 0 Interrupt Flag"]
    #[inline(always)]
    pub fn win0(&mut self) -> Win0W<'_, IntflagsetSpec> {
        Win0W::new(self, 4)
    }
}
#[doc = "Interrupt Flag Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagsetSpec;
impl crate::RegisterSpec for IntflagsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagset::R`](R) reader structure"]
impl crate::Readable for IntflagsetSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagset::W`](W) writer structure"]
impl crate::Writable for IntflagsetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x13;
}
#[doc = "`reset()` method sets INTFLAGSET to value 0"]
impl crate::Resettable for IntflagsetSpec {}

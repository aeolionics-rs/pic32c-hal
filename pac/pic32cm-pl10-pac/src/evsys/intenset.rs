#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `OVR0` reader - Overrun Channel 0 Interrupt Enable"]
pub type Ovr0R = crate::BitReader;
#[doc = "Field `OVR0` writer - Overrun Channel 0 Interrupt Enable"]
pub type Ovr0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `OVR1` reader - Overrun Channel 1 Interrupt Enable"]
pub type Ovr1R = crate::BitReader;
#[doc = "Field `OVR1` writer - Overrun Channel 1 Interrupt Enable"]
pub type Ovr1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `OVR2` reader - Overrun Channel 2 Interrupt Enable"]
pub type Ovr2R = crate::BitReader;
#[doc = "Field `OVR2` writer - Overrun Channel 2 Interrupt Enable"]
pub type Ovr2W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `OVR3` reader - Overrun Channel 3 Interrupt Enable"]
pub type Ovr3R = crate::BitReader;
#[doc = "Field `OVR3` writer - Overrun Channel 3 Interrupt Enable"]
pub type Ovr3W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `EVD0` reader - Event Detected Channel 0 Interrupt Enable"]
pub type Evd0R = crate::BitReader;
#[doc = "Field `EVD0` writer - Event Detected Channel 0 Interrupt Enable"]
pub type Evd0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `EVD1` reader - Event Detected Channel 1 Interrupt Enable"]
pub type Evd1R = crate::BitReader;
#[doc = "Field `EVD1` writer - Event Detected Channel 1 Interrupt Enable"]
pub type Evd1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `EVD2` reader - Event Detected Channel 2 Interrupt Enable"]
pub type Evd2R = crate::BitReader;
#[doc = "Field `EVD2` writer - Event Detected Channel 2 Interrupt Enable"]
pub type Evd2W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `EVD3` reader - Event Detected Channel 3 Interrupt Enable"]
pub type Evd3R = crate::BitReader;
#[doc = "Field `EVD3` writer - Event Detected Channel 3 Interrupt Enable"]
pub type Evd3W<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overrun Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ovr0(&self) -> Ovr0R {
        Ovr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ovr1(&self) -> Ovr1R {
        Ovr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overrun Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ovr2(&self) -> Ovr2R {
        Ovr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ovr3(&self) -> Ovr3R {
        Ovr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Event Detected Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn evd0(&self) -> Evd0R {
        Evd0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event Detected Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn evd1(&self) -> Evd1R {
        Evd1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event Detected Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn evd2(&self) -> Evd2R {
        Evd2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Event Detected Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn evd3(&self) -> Evd3R {
        Evd3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ovr0(&mut self) -> Ovr0W<'_, IntensetSpec> {
        Ovr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Overrun Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ovr1(&mut self) -> Ovr1W<'_, IntensetSpec> {
        Ovr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Overrun Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ovr2(&mut self) -> Ovr2W<'_, IntensetSpec> {
        Ovr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ovr3(&mut self) -> Ovr3W<'_, IntensetSpec> {
        Ovr3W::new(self, 3)
    }
    #[doc = "Bit 16 - Event Detected Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn evd0(&mut self) -> Evd0W<'_, IntensetSpec> {
        Evd0W::new(self, 16)
    }
    #[doc = "Bit 17 - Event Detected Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn evd1(&mut self) -> Evd1W<'_, IntensetSpec> {
        Evd1W::new(self, 17)
    }
    #[doc = "Bit 18 - Event Detected Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn evd2(&mut self) -> Evd2W<'_, IntensetSpec> {
        Evd2W::new(self, 18)
    }
    #[doc = "Bit 19 - Event Detected Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn evd3(&mut self) -> Evd3W<'_, IntensetSpec> {
        Evd3W::new(self, 19)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_000f;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}

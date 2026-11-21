#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `OVR0` reader - Overrun Channel 0"]
pub type Ovr0R = crate::BitReader;
#[doc = "Field `OVR0` writer - Overrun Channel 0"]
pub type Ovr0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVR1` reader - Overrun Channel 1"]
pub type Ovr1R = crate::BitReader;
#[doc = "Field `OVR1` writer - Overrun Channel 1"]
pub type Ovr1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVR2` reader - Overrun Channel 2"]
pub type Ovr2R = crate::BitReader;
#[doc = "Field `OVR2` writer - Overrun Channel 2"]
pub type Ovr2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVR3` reader - Overrun Channel 3"]
pub type Ovr3R = crate::BitReader;
#[doc = "Field `OVR3` writer - Overrun Channel 3"]
pub type Ovr3W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EVD0` reader - Event Detected Channel 0"]
pub type Evd0R = crate::BitReader;
#[doc = "Field `EVD0` writer - Event Detected Channel 0"]
pub type Evd0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EVD1` reader - Event Detected Channel 1"]
pub type Evd1R = crate::BitReader;
#[doc = "Field `EVD1` writer - Event Detected Channel 1"]
pub type Evd1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EVD2` reader - Event Detected Channel 2"]
pub type Evd2R = crate::BitReader;
#[doc = "Field `EVD2` writer - Event Detected Channel 2"]
pub type Evd2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EVD3` reader - Event Detected Channel 3"]
pub type Evd3R = crate::BitReader;
#[doc = "Field `EVD3` writer - Event Detected Channel 3"]
pub type Evd3W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overrun Channel 0"]
    #[inline(always)]
    pub fn ovr0(&self) -> Ovr0R {
        Ovr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Channel 1"]
    #[inline(always)]
    pub fn ovr1(&self) -> Ovr1R {
        Ovr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overrun Channel 2"]
    #[inline(always)]
    pub fn ovr2(&self) -> Ovr2R {
        Ovr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Channel 3"]
    #[inline(always)]
    pub fn ovr3(&self) -> Ovr3R {
        Ovr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Event Detected Channel 0"]
    #[inline(always)]
    pub fn evd0(&self) -> Evd0R {
        Evd0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event Detected Channel 1"]
    #[inline(always)]
    pub fn evd1(&self) -> Evd1R {
        Evd1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event Detected Channel 2"]
    #[inline(always)]
    pub fn evd2(&self) -> Evd2R {
        Evd2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Event Detected Channel 3"]
    #[inline(always)]
    pub fn evd3(&self) -> Evd3R {
        Evd3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun Channel 0"]
    #[inline(always)]
    pub fn ovr0(&mut self) -> Ovr0W<'_, IntflagSpec> {
        Ovr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Overrun Channel 1"]
    #[inline(always)]
    pub fn ovr1(&mut self) -> Ovr1W<'_, IntflagSpec> {
        Ovr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Overrun Channel 2"]
    #[inline(always)]
    pub fn ovr2(&mut self) -> Ovr2W<'_, IntflagSpec> {
        Ovr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Channel 3"]
    #[inline(always)]
    pub fn ovr3(&mut self) -> Ovr3W<'_, IntflagSpec> {
        Ovr3W::new(self, 3)
    }
    #[doc = "Bit 16 - Event Detected Channel 0"]
    #[inline(always)]
    pub fn evd0(&mut self) -> Evd0W<'_, IntflagSpec> {
        Evd0W::new(self, 16)
    }
    #[doc = "Bit 17 - Event Detected Channel 1"]
    #[inline(always)]
    pub fn evd1(&mut self) -> Evd1W<'_, IntflagSpec> {
        Evd1W::new(self, 17)
    }
    #[doc = "Bit 18 - Event Detected Channel 2"]
    #[inline(always)]
    pub fn evd2(&mut self) -> Evd2W<'_, IntflagSpec> {
        Evd2W::new(self, 18)
    }
    #[doc = "Bit 19 - Event Detected Channel 3"]
    #[inline(always)]
    pub fn evd3(&mut self) -> Evd3W<'_, IntflagSpec> {
        Evd3W::new(self, 19)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_000f;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}

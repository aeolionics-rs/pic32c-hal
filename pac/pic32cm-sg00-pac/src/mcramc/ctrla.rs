#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstselect {
    #[doc = "0: No effect."]
    Swrst0 = 0,
    #[doc = "1: Reset the MCRAMC. A software-triggered hardware reset of the MCRAMC user interface is performed."]
    Swrst1 = 1,
}
impl From<Swrstselect> for bool {
    #[inline(always)]
    fn from(variant: Swrstselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader<Swrstselect>;
impl SwrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrstselect {
        match self.bits {
            false => Swrstselect::Swrst0,
            true => Swrstselect::Swrst1,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_swrst_0(&self) -> bool {
        *self == Swrstselect::Swrst0
    }
    #[doc = "Reset the MCRAMC. A software-triggered hardware reset of the MCRAMC user interface is performed."]
    #[inline(always)]
    pub fn is_swrst_1(&self) -> bool {
        *self == Swrstselect::Swrst1
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG, Swrstselect>;
impl<'a, REG> SwrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn swrst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstselect::Swrst0)
    }
    #[doc = "Reset the MCRAMC. A software-triggered hardware reset of the MCRAMC user interface is performed."]
    #[inline(always)]
    pub fn swrst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstselect::Swrst1)
    }
}
#[doc = "ECC Decoder Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableselect {
    #[doc = "0: ECC decoding is disabled."]
    Enable0 = 0,
    #[doc = "1: ECC decoding is enabled."]
    Enable1 = 1,
}
impl From<Enableselect> for bool {
    #[inline(always)]
    fn from(variant: Enableselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - ECC Decoder Enable"]
pub type EnableR = crate::BitReader<Enableselect>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enableselect {
        match self.bits {
            false => Enableselect::Enable0,
            true => Enableselect::Enable1,
        }
    }
    #[doc = "ECC decoding is disabled."]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == Enableselect::Enable0
    }
    #[doc = "ECC decoding is enabled."]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == Enableselect::Enable1
    }
}
#[doc = "Field `ENABLE` writer - ECC Decoder Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enableselect>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC decoding is disabled."]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable0)
    }
    #[doc = "ECC decoding is enabled."]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable1)
    }
}
#[doc = "Field `PRIV` reader - Priviledged Access Enable"]
pub type PrivR = crate::BitReader;
#[doc = "Field `PRIV` writer - Priviledged Access Enable"]
pub type PrivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WS` reader - Wait State Enable"]
pub type WsR = crate::BitReader;
#[doc = "Field `WS` writer - Wait State Enable"]
pub type WsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC Decoder Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Priviledged Access Enable"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Wait State Enable"]
    #[inline(always)]
    pub fn ws(&self) -> WsR {
        WsR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - ECC Decoder Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Priviledged Access Enable"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PrivW<'_, CtrlaSpec> {
        PrivW::new(self, 2)
    }
    #[doc = "Bit 8 - Wait State Enable"]
    #[inline(always)]
    pub fn ws(&mut self) -> WsW<'_, CtrlaSpec> {
        WsW::new(self, 8)
    }
}
#[doc = "Control Enable A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0x02"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u32 = 0x02;
}

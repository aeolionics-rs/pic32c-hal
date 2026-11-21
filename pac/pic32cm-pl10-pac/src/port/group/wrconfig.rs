#[doc = "Register `WRCONFIG` writer"]
pub type W = crate::W<WrconfigSpec>;
#[doc = "Field `PINMASK` writer - Pin Mask for Multiple Pin Configuration"]
pub type PinmaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PMUXEN` writer - Peripheral Multiplexer Enable"]
pub type PmuxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN` writer - Input Enable"]
pub type InenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub type PullenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEWLIM` writer - Output Driver Slew Rate Limit Enable"]
pub type SlewlimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Peripheral Multiplexing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmuxselect {
    #[doc = "0: Peripheral function A selected"]
    A = 0,
    #[doc = "1: Peripheral function B selected"]
    B = 1,
    #[doc = "2: Peripheral function C selected"]
    C = 2,
    #[doc = "3: Peripheral function D selected"]
    D = 3,
    #[doc = "4: Peripheral function E selected"]
    E = 4,
    #[doc = "5: Peripheral function F selected"]
    F = 5,
    #[doc = "6: Peripheral function G selected"]
    G = 6,
    #[doc = "7: Peripheral function H selected"]
    H = 7,
    #[doc = "8: Peripheral function I selected"]
    I = 8,
    #[doc = "9: Peripheral function J selected"]
    J = 9,
}
impl From<Pmuxselect> for u8 {
    #[inline(always)]
    fn from(variant: Pmuxselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmuxselect {
    type Ux = u8;
}
impl crate::IsEnum for Pmuxselect {}
#[doc = "Field `PMUX` writer - Peripheral Multiplexing"]
pub type PmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pmuxselect>;
impl<'a, REG> PmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxselect::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxselect::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxselect::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxselect::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn e(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxselect::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxselect::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn g(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxselect::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxselect::H)
    }
    #[doc = "Peripheral function I selected"]
    #[inline(always)]
    pub fn i(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxselect::I)
    }
    #[doc = "Peripheral function J selected"]
    #[inline(always)]
    pub fn j(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxselect::J)
    }
}
#[doc = "Field `WRPMUX` writer - Write PMUX"]
pub type WrpmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPINCFG` writer - Write PINCFG"]
pub type WrpincfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWSEL` writer - Half-Word Select"]
pub type HwselW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:15 - Pin Mask for Multiple Pin Configuration"]
    #[inline(always)]
    pub fn pinmask(&mut self) -> PinmaskW<'_, WrconfigSpec> {
        PinmaskW::new(self, 0)
    }
    #[doc = "Bit 16 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PmuxenW<'_, WrconfigSpec> {
        PmuxenW::new(self, 16)
    }
    #[doc = "Bit 17 - Input Enable"]
    #[inline(always)]
    pub fn inen(&mut self) -> InenW<'_, WrconfigSpec> {
        InenW::new(self, 17)
    }
    #[doc = "Bit 18 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&mut self) -> PullenW<'_, WrconfigSpec> {
        PullenW::new(self, 18)
    }
    #[doc = "Bit 20 - Output Driver Slew Rate Limit Enable"]
    #[inline(always)]
    pub fn slewlim(&mut self) -> SlewlimW<'_, WrconfigSpec> {
        SlewlimW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Peripheral Multiplexing"]
    #[inline(always)]
    pub fn pmux(&mut self) -> PmuxW<'_, WrconfigSpec> {
        PmuxW::new(self, 24)
    }
    #[doc = "Bit 28 - Write PMUX"]
    #[inline(always)]
    pub fn wrpmux(&mut self) -> WrpmuxW<'_, WrconfigSpec> {
        WrpmuxW::new(self, 28)
    }
    #[doc = "Bit 30 - Write PINCFG"]
    #[inline(always)]
    pub fn wrpincfg(&mut self) -> WrpincfgW<'_, WrconfigSpec> {
        WrpincfgW::new(self, 30)
    }
    #[doc = "Bit 31 - Half-Word Select"]
    #[inline(always)]
    pub fn hwsel(&mut self) -> HwselW<'_, WrconfigSpec> {
        HwselW::new(self, 31)
    }
}
#[doc = "Write Configuration\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrconfig::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrconfigSpec;
impl crate::RegisterSpec for WrconfigSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wrconfig::W`](W) writer structure"]
impl crate::Writable for WrconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRCONFIG to value 0"]
impl crate::Resettable for WrconfigSpec {}

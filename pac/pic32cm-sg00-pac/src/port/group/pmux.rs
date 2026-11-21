#[doc = "Register `PMUX[%s]` reader"]
pub type R = crate::R<PmuxSpec>;
#[doc = "Register `PMUX[%s]` writer"]
pub type W = crate::W<PmuxSpec>;
#[doc = "Peripheral Multiplexing for Even-Numbered Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmuxeselect {
    #[doc = "0: Peripheral function A selected"]
    A = 0,
    #[doc = "1: Peripheral function B selected"]
    B = 1,
    #[doc = "3: Peripheral function D selected"]
    D = 3,
    #[doc = "5: Peripheral function F selected"]
    F = 5,
    #[doc = "7: Peripheral function H selected"]
    H = 7,
    #[doc = "8: Peripheral function I selected"]
    I = 8,
    #[doc = "10: Peripheral function K selected"]
    K = 10,
    #[doc = "15: Peripheral function P selected"]
    P = 15,
}
impl From<Pmuxeselect> for u8 {
    #[inline(always)]
    fn from(variant: Pmuxeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmuxeselect {
    type Ux = u8;
}
impl crate::IsEnum for Pmuxeselect {}
#[doc = "Field `PMUXE` reader - Peripheral Multiplexing for Even-Numbered Pin"]
pub type PmuxeR = crate::FieldReader<Pmuxeselect>;
impl PmuxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pmuxeselect> {
        match self.bits {
            0 => Some(Pmuxeselect::A),
            1 => Some(Pmuxeselect::B),
            3 => Some(Pmuxeselect::D),
            5 => Some(Pmuxeselect::F),
            7 => Some(Pmuxeselect::H),
            8 => Some(Pmuxeselect::I),
            10 => Some(Pmuxeselect::K),
            15 => Some(Pmuxeselect::P),
            _ => None,
        }
    }
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Pmuxeselect::A
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == Pmuxeselect::B
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == Pmuxeselect::D
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == Pmuxeselect::F
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == Pmuxeselect::H
    }
    #[doc = "Peripheral function I selected"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == Pmuxeselect::I
    }
    #[doc = "Peripheral function K selected"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == Pmuxeselect::K
    }
    #[doc = "Peripheral function P selected"]
    #[inline(always)]
    pub fn is_p(&self) -> bool {
        *self == Pmuxeselect::P
    }
}
#[doc = "Field `PMUXE` writer - Peripheral Multiplexing for Even-Numbered Pin"]
pub type PmuxeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pmuxeselect>;
impl<'a, REG> PmuxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::B)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::D)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::F)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::H)
    }
    #[doc = "Peripheral function I selected"]
    #[inline(always)]
    pub fn i(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::I)
    }
    #[doc = "Peripheral function K selected"]
    #[inline(always)]
    pub fn k(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::K)
    }
    #[doc = "Peripheral function P selected"]
    #[inline(always)]
    pub fn p(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::P)
    }
}
#[doc = "Peripheral Multiplexing for Odd-Numbered Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmuxoselect {
    #[doc = "0: Peripheral function A selected"]
    A = 0,
    #[doc = "1: Peripheral function B selected"]
    B = 1,
    #[doc = "3: Peripheral function D selected"]
    D = 3,
    #[doc = "5: Peripheral function F selected"]
    F = 5,
    #[doc = "7: Peripheral function H selected"]
    H = 7,
    #[doc = "8: Peripheral function I selected"]
    I = 8,
    #[doc = "10: Peripheral function K selected"]
    K = 10,
    #[doc = "15: Peripheral function P selected"]
    P = 15,
}
impl From<Pmuxoselect> for u8 {
    #[inline(always)]
    fn from(variant: Pmuxoselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmuxoselect {
    type Ux = u8;
}
impl crate::IsEnum for Pmuxoselect {}
#[doc = "Field `PMUXO` reader - Peripheral Multiplexing for Odd-Numbered Pin"]
pub type PmuxoR = crate::FieldReader<Pmuxoselect>;
impl PmuxoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pmuxoselect> {
        match self.bits {
            0 => Some(Pmuxoselect::A),
            1 => Some(Pmuxoselect::B),
            3 => Some(Pmuxoselect::D),
            5 => Some(Pmuxoselect::F),
            7 => Some(Pmuxoselect::H),
            8 => Some(Pmuxoselect::I),
            10 => Some(Pmuxoselect::K),
            15 => Some(Pmuxoselect::P),
            _ => None,
        }
    }
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Pmuxoselect::A
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == Pmuxoselect::B
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == Pmuxoselect::D
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == Pmuxoselect::F
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == Pmuxoselect::H
    }
    #[doc = "Peripheral function I selected"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == Pmuxoselect::I
    }
    #[doc = "Peripheral function K selected"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == Pmuxoselect::K
    }
    #[doc = "Peripheral function P selected"]
    #[inline(always)]
    pub fn is_p(&self) -> bool {
        *self == Pmuxoselect::P
    }
}
#[doc = "Field `PMUXO` writer - Peripheral Multiplexing for Odd-Numbered Pin"]
pub type PmuxoW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pmuxoselect>;
impl<'a, REG> PmuxoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::B)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::D)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::F)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::H)
    }
    #[doc = "Peripheral function I selected"]
    #[inline(always)]
    pub fn i(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::I)
    }
    #[doc = "Peripheral function K selected"]
    #[inline(always)]
    pub fn k(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::K)
    }
    #[doc = "Peripheral function P selected"]
    #[inline(always)]
    pub fn p(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::P)
    }
}
impl R {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxe(&self) -> PmuxeR {
        PmuxeR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxo(&self) -> PmuxoR {
        PmuxoR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxe(&mut self) -> PmuxeW<'_, PmuxSpec> {
        PmuxeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxo(&mut self) -> PmuxoW<'_, PmuxSpec> {
        PmuxoW::new(self, 4)
    }
}
#[doc = "Peripheral Multiplexing\n\nYou can [`read`](crate::Reg::read) this register and get [`pmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuxSpec;
impl crate::RegisterSpec for PmuxSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmux::R`](R) reader structure"]
impl crate::Readable for PmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`pmux::W`](W) writer structure"]
impl crate::Writable for PmuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMUX[%s] to value 0"]
impl crate::Resettable for PmuxSpec {}

#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Internal Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Intrefselect {
    #[doc = "0: Internal 1.024V reference"]
    _1v024 = 0,
    #[doc = "1: Internal 2.048V reference"]
    _2v048 = 1,
    #[doc = "2: Internal 4.096V reference"]
    _4v096 = 2,
    #[doc = "3: Internal 2.500V reference"]
    _2v500 = 3,
}
impl From<Intrefselect> for u8 {
    #[inline(always)]
    fn from(variant: Intrefselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Intrefselect {
    type Ux = u8;
}
impl crate::IsEnum for Intrefselect {}
#[doc = "Field `INTREF` reader - Internal Reference Selection"]
pub type IntrefR = crate::FieldReader<Intrefselect>;
impl IntrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intrefselect {
        match self.bits {
            0 => Intrefselect::_1v024,
            1 => Intrefselect::_2v048,
            2 => Intrefselect::_4v096,
            3 => Intrefselect::_2v500,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal 1.024V reference"]
    #[inline(always)]
    pub fn is_1v024(&self) -> bool {
        *self == Intrefselect::_1v024
    }
    #[doc = "Internal 2.048V reference"]
    #[inline(always)]
    pub fn is_2v048(&self) -> bool {
        *self == Intrefselect::_2v048
    }
    #[doc = "Internal 4.096V reference"]
    #[inline(always)]
    pub fn is_4v096(&self) -> bool {
        *self == Intrefselect::_4v096
    }
    #[doc = "Internal 2.500V reference"]
    #[inline(always)]
    pub fn is_2v500(&self) -> bool {
        *self == Intrefselect::_2v500
    }
}
#[doc = "Field `INTREF` writer - Internal Reference Selection"]
pub type IntrefW<'a, REG> = crate::FieldWriter<'a, REG, 2, Intrefselect, crate::Safe>;
impl<'a, REG> IntrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 1.024V reference"]
    #[inline(always)]
    pub fn _1v024(self) -> &'a mut crate::W<REG> {
        self.variant(Intrefselect::_1v024)
    }
    #[doc = "Internal 2.048V reference"]
    #[inline(always)]
    pub fn _2v048(self) -> &'a mut crate::W<REG> {
        self.variant(Intrefselect::_2v048)
    }
    #[doc = "Internal 4.096V reference"]
    #[inline(always)]
    pub fn _4v096(self) -> &'a mut crate::W<REG> {
        self.variant(Intrefselect::_4v096)
    }
    #[doc = "Internal 2.500V reference"]
    #[inline(always)]
    pub fn _2v500(self) -> &'a mut crate::W<REG> {
        self.variant(Intrefselect::_2v500)
    }
}
impl R {
    #[doc = "Bits 0:1 - Internal Reference Selection"]
    #[inline(always)]
    pub fn intref(&self) -> IntrefR {
        IntrefR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Internal Reference Selection"]
    #[inline(always)]
    pub fn intref(&mut self) -> IntrefW<'_, CtrlbSpec> {
        IntrefW::new(self, 0)
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}

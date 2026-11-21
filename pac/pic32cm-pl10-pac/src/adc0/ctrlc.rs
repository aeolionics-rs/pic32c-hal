#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refselselect {
    #[doc = "0: VDD"]
    Vdd = 0,
    #[doc = "2: External pin used as reference"]
    Extvref = 2,
    #[doc = "4: Internal reference 1.024V"]
    _1v024 = 4,
    #[doc = "5: Internal reference 2.048V"]
    _2v048 = 5,
    #[doc = "6: Internal reference 4.096V"]
    _4v096 = 6,
    #[doc = "7: Internal reference 2.500V"]
    _2v500 = 7,
}
impl From<Refselselect> for u8 {
    #[inline(always)]
    fn from(variant: Refselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refselselect {
    type Ux = u8;
}
impl crate::IsEnum for Refselselect {}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type RefselR = crate::FieldReader<Refselselect>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refselselect> {
        match self.bits {
            0 => Some(Refselselect::Vdd),
            2 => Some(Refselselect::Extvref),
            4 => Some(Refselselect::_1v024),
            5 => Some(Refselselect::_2v048),
            6 => Some(Refselselect::_4v096),
            7 => Some(Refselselect::_2v500),
            _ => None,
        }
    }
    #[doc = "VDD"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == Refselselect::Vdd
    }
    #[doc = "External pin used as reference"]
    #[inline(always)]
    pub fn is_extvref(&self) -> bool {
        *self == Refselselect::Extvref
    }
    #[doc = "Internal reference 1.024V"]
    #[inline(always)]
    pub fn is_1v024(&self) -> bool {
        *self == Refselselect::_1v024
    }
    #[doc = "Internal reference 2.048V"]
    #[inline(always)]
    pub fn is_2v048(&self) -> bool {
        *self == Refselselect::_2v048
    }
    #[doc = "Internal reference 4.096V"]
    #[inline(always)]
    pub fn is_4v096(&self) -> bool {
        *self == Refselselect::_4v096
    }
    #[doc = "Internal reference 2.500V"]
    #[inline(always)]
    pub fn is_2v500(&self) -> bool {
        *self == Refselselect::_2v500
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Refselselect>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Vdd)
    }
    #[doc = "External pin used as reference"]
    #[inline(always)]
    pub fn extvref(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Extvref)
    }
    #[doc = "Internal reference 1.024V"]
    #[inline(always)]
    pub fn _1v024(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::_1v024)
    }
    #[doc = "Internal reference 2.048V"]
    #[inline(always)]
    pub fn _2v048(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::_2v048)
    }
    #[doc = "Internal reference 4.096V"]
    #[inline(always)]
    pub fn _4v096(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::_4v096)
    }
    #[doc = "Internal reference 2.500V"]
    #[inline(always)]
    pub fn _2v500(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::_2v500)
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, CtrlcSpec> {
        RefselW::new(self, 0)
    }
}
#[doc = "Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {}

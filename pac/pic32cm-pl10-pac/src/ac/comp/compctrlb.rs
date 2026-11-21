#[doc = "Register `COMPCTRLB` reader"]
pub type R = crate::R<CompctrlbSpec>;
#[doc = "Register `COMPCTRLB` writer"]
pub type W = crate::W<CompctrlbSpec>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refselselect {
    #[doc = "0: VDD"]
    Vdd = 0,
    #[doc = "1: Internal reference voltage, as defined by the Internal Reference Selection bit field in the Control B register (CTRLB.INTREF)"]
    Intvref = 1,
    #[doc = "2: External reference pin"]
    Extvref = 2,
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
            1 => Some(Refselselect::Intvref),
            2 => Some(Refselselect::Extvref),
            _ => None,
        }
    }
    #[doc = "VDD"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == Refselselect::Vdd
    }
    #[doc = "Internal reference voltage, as defined by the Internal Reference Selection bit field in the Control B register (CTRLB.INTREF)"]
    #[inline(always)]
    pub fn is_intvref(&self) -> bool {
        *self == Refselselect::Intvref
    }
    #[doc = "External reference pin"]
    #[inline(always)]
    pub fn is_extvref(&self) -> bool {
        *self == Refselselect::Extvref
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refselselect>;
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
    #[doc = "Internal reference voltage, as defined by the Internal Reference Selection bit field in the Control B register (CTRLB.INTREF)"]
    #[inline(always)]
    pub fn intvref(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Intvref)
    }
    #[doc = "External reference pin"]
    #[inline(always)]
    pub fn extvref(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Extvref)
    }
}
impl R {
    #[doc = "Bits 0:1 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, CompctrlbSpec> {
        RefselW::new(self, 0)
    }
}
#[doc = "Comparator # Comparator Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`compctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompctrlbSpec;
impl crate::RegisterSpec for CompctrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compctrlb::R`](R) reader structure"]
impl crate::Readable for CompctrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`compctrlb::W`](W) writer structure"]
impl crate::Writable for CompctrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPCTRLB to value 0"]
impl crate::Resettable for CompctrlbSpec {}

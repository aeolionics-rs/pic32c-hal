#[doc = "Register `GENDIV` reader"]
pub type R = crate::R<GendivSpec>;
#[doc = "Register `GENDIV` writer"]
pub type W = crate::W<GendivSpec>;
#[doc = "Generic Clock Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idselect {
    #[doc = "0: Generic clock generator 0"]
    Gclk0 = 0,
    #[doc = "1: Generic clock generator 1"]
    Gclk1 = 1,
    #[doc = "2: Generic clock generator 2"]
    Gclk2 = 2,
    #[doc = "3: Generic clock generator 3"]
    Gclk3 = 3,
    #[doc = "4: Generic clock generator 4"]
    Gclk4 = 4,
    #[doc = "5: Generic clock generator 5"]
    Gclk5 = 5,
    #[doc = "6: Generic clock generator 6"]
    Gclk6 = 6,
    #[doc = "7: Generic clock generator 7"]
    Gclk7 = 7,
}
impl From<Idselect> for u8 {
    #[inline(always)]
    fn from(variant: Idselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idselect {
    type Ux = u8;
}
impl crate::IsEnum for Idselect {}
#[doc = "Field `ID` reader - Generic Clock Generator Selection"]
pub type IdR = crate::FieldReader<Idselect>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idselect> {
        match self.bits {
            0 => Some(Idselect::Gclk0),
            1 => Some(Idselect::Gclk1),
            2 => Some(Idselect::Gclk2),
            3 => Some(Idselect::Gclk3),
            4 => Some(Idselect::Gclk4),
            5 => Some(Idselect::Gclk5),
            6 => Some(Idselect::Gclk6),
            7 => Some(Idselect::Gclk7),
            _ => None,
        }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == Idselect::Gclk0
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == Idselect::Gclk1
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == Idselect::Gclk2
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == Idselect::Gclk3
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == Idselect::Gclk4
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == Idselect::Gclk5
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == Idselect::Gclk6
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == Idselect::Gclk7
    }
}
#[doc = "Field `ID` writer - Generic Clock Generator Selection"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 4, Idselect>;
impl<'a, REG> IdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Gclk0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Gclk1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Gclk2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Gclk3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Gclk4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Gclk5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Gclk6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Gclk7)
    }
}
#[doc = "Field `DIV` reader - Division Factor"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Division Factor"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, GendivSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, GendivSpec> {
        DivW::new(self, 8)
    }
}
#[doc = "Generic Clock Generator Division\n\nYou can [`read`](crate::Reg::read) this register and get [`gendiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gendiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GendivSpec;
impl crate::RegisterSpec for GendivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gendiv::R`](R) reader structure"]
impl crate::Readable for GendivSpec {}
#[doc = "`write(|w| ..)` method takes [`gendiv::W`](W) writer structure"]
impl crate::Writable for GendivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GENDIV to value 0"]
impl crate::Resettable for GendivSpec {}

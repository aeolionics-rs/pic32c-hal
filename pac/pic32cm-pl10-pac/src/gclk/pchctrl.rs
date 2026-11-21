#[doc = "Register `PCHCTRL[%s]` reader"]
pub type R = crate::R<PchctrlSpec>;
#[doc = "Register `PCHCTRL[%s]` writer"]
pub type W = crate::W<PchctrlSpec>;
#[doc = "Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Genselect {
    #[doc = "0: Generic Clock Generator 0"]
    Gclk0 = 0,
    #[doc = "1: Generic Clock Generator 1"]
    Gclk1 = 1,
    #[doc = "2: Generic Clock Generator 2"]
    Gclk2 = 2,
    #[doc = "3: Generic Clock Generator 3"]
    Gclk3 = 3,
}
impl From<Genselect> for u8 {
    #[inline(always)]
    fn from(variant: Genselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Genselect {
    type Ux = u8;
}
impl crate::IsEnum for Genselect {}
#[doc = "Field `GEN` reader - Generator Selection"]
pub type GenR = crate::FieldReader<Genselect>;
impl GenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Genselect> {
        match self.bits {
            0 => Some(Genselect::Gclk0),
            1 => Some(Genselect::Gclk1),
            2 => Some(Genselect::Gclk2),
            3 => Some(Genselect::Gclk3),
            _ => None,
        }
    }
    #[doc = "Generic Clock Generator 0"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == Genselect::Gclk0
    }
    #[doc = "Generic Clock Generator 1"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == Genselect::Gclk1
    }
    #[doc = "Generic Clock Generator 2"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == Genselect::Gclk2
    }
    #[doc = "Generic Clock Generator 3"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == Genselect::Gclk3
    }
}
#[doc = "Field `GEN` writer - Generator Selection"]
pub type GenW<'a, REG> = crate::FieldWriter<'a, REG, 4, Genselect>;
impl<'a, REG> GenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generic Clock Generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk0)
    }
    #[doc = "Generic Clock Generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk1)
    }
    #[doc = "Generic Clock Generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk2)
    }
    #[doc = "Generic Clock Generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk3)
    }
}
#[doc = "Field `CHEN` reader - Channel Enable"]
pub type ChenR = crate::BitReader;
#[doc = "Field `CHEN` writer - Channel Enable"]
pub type ChenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Generator Selection"]
    #[inline(always)]
    pub fn gen_(&self) -> GenR {
        GenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline(always)]
    pub fn chen(&self) -> ChenR {
        ChenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generator Selection"]
    #[inline(always)]
    pub fn gen_(&mut self) -> GenW<'_, PchctrlSpec> {
        GenW::new(self, 0)
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline(always)]
    pub fn chen(&mut self) -> ChenW<'_, PchctrlSpec> {
        ChenW::new(self, 6)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WrtlockW<'_, PchctrlSpec> {
        WrtlockW::new(self, 7)
    }
}
#[doc = "Peripheral Channel Control #\n\nYou can [`read`](crate::Reg::read) this register and get [`pchctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pchctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PchctrlSpec;
impl crate::RegisterSpec for PchctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pchctrl::R`](R) reader structure"]
impl crate::Readable for PchctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pchctrl::W`](W) writer structure"]
impl crate::Writable for PchctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCHCTRL[%s] to value 0"]
impl crate::Resettable for PchctrlSpec {}

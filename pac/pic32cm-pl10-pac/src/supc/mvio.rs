#[doc = "Register `MVIO` reader"]
pub type R = crate::R<MvioSpec>;
#[doc = "Register `MVIO` writer"]
pub type W = crate::W<MvioSpec>;
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modeselect {
    #[doc = "0: MVIO operating in dual supply mode"]
    Dual = 0,
    #[doc = "1: MVIO operating in Single supply mode"]
    Single = 1,
}
impl From<Modeselect> for bool {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Operation Mode"]
pub type ModeR = crate::BitReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modeselect {
        match self.bits {
            false => Modeselect::Dual,
            true => Modeselect::Single,
        }
    }
    #[doc = "MVIO operating in dual supply mode"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == Modeselect::Dual
    }
    #[doc = "MVIO operating in Single supply mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Modeselect::Single
    }
}
#[doc = "Field `MODE` writer - Operation Mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MVIO operating in dual supply mode"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Dual)
    }
    #[doc = "MVIO operating in Single supply mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Single)
    }
}
#[doc = "VDDIO2 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vddio2cfgselect {
    #[doc = "0: VDDIO2 with voltage protection enabled"]
    Normal = 0,
    #[doc = "1: VDDIO2 with voltage protection disabled"]
    Lowpower = 1,
    #[doc = "2: VDDIO2 domain is disabled. I/O pins are not available."]
    Vddio2off = 2,
}
impl From<Vddio2cfgselect> for u8 {
    #[inline(always)]
    fn from(variant: Vddio2cfgselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vddio2cfgselect {
    type Ux = u8;
}
impl crate::IsEnum for Vddio2cfgselect {}
#[doc = "Field `VDDIO2CFG` reader - VDDIO2 Configuration"]
pub type Vddio2cfgR = crate::FieldReader<Vddio2cfgselect>;
impl Vddio2cfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vddio2cfgselect> {
        match self.bits {
            0 => Some(Vddio2cfgselect::Normal),
            1 => Some(Vddio2cfgselect::Lowpower),
            2 => Some(Vddio2cfgselect::Vddio2off),
            _ => None,
        }
    }
    #[doc = "VDDIO2 with voltage protection enabled"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Vddio2cfgselect::Normal
    }
    #[doc = "VDDIO2 with voltage protection disabled"]
    #[inline(always)]
    pub fn is_lowpower(&self) -> bool {
        *self == Vddio2cfgselect::Lowpower
    }
    #[doc = "VDDIO2 domain is disabled. I/O pins are not available."]
    #[inline(always)]
    pub fn is_vddio2off(&self) -> bool {
        *self == Vddio2cfgselect::Vddio2off
    }
}
#[doc = "Field `VDDIO2CFG` writer - VDDIO2 Configuration"]
pub type Vddio2cfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vddio2cfgselect>;
impl<'a, REG> Vddio2cfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VDDIO2 with voltage protection enabled"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio2cfgselect::Normal)
    }
    #[doc = "VDDIO2 with voltage protection disabled"]
    #[inline(always)]
    pub fn lowpower(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio2cfgselect::Lowpower)
    }
    #[doc = "VDDIO2 domain is disabled. I/O pins are not available."]
    #[inline(always)]
    pub fn vddio2off(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio2cfgselect::Vddio2off)
    }
}
impl R {
    #[doc = "Bit 0 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - VDDIO2 Configuration"]
    #[inline(always)]
    pub fn vddio2cfg(&self) -> Vddio2cfgR {
        Vddio2cfgR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, MvioSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - VDDIO2 Configuration"]
    #[inline(always)]
    pub fn vddio2cfg(&mut self) -> Vddio2cfgW<'_, MvioSpec> {
        Vddio2cfgW::new(self, 2)
    }
}
#[doc = "MVIO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mvio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mvio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MvioSpec;
impl crate::RegisterSpec for MvioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mvio::R`](R) reader structure"]
impl crate::Readable for MvioSpec {}
#[doc = "`write(|w| ..)` method takes [`mvio::W`](W) writer structure"]
impl crate::Writable for MvioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MVIO to value 0"]
impl crate::Resettable for MvioSpec {}

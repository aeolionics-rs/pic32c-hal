#[doc = "Register `WINCTRL` reader"]
pub type R = crate::R<WinctrlSpec>;
#[doc = "Register `WINCTRL` writer"]
pub type W = crate::W<WinctrlSpec>;
#[doc = "Window Comparator Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Winmodeselect {
    #[doc = "0: No Window Comparison"]
    None = 0,
    #[doc = "1: OUTPUT 'less than' WINLT"]
    Below = 1,
    #[doc = "2: OUTPUT 'greater than' WINHT"]
    Above = 2,
    #[doc = "3: WINLT 'less than' OUTPUT 'less than' WINHT"]
    Inside = 3,
    #[doc = "4: OUTPUT 'less than' WINLT or OUTPUT 'greater than'WINHT"]
    Outside = 4,
}
impl From<Winmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Winmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Winmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Winmodeselect {}
#[doc = "Field `WINMODE` reader - Window Comparator Mode"]
pub type WinmodeR = crate::FieldReader<Winmodeselect>;
impl WinmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Winmodeselect> {
        match self.bits {
            0 => Some(Winmodeselect::None),
            1 => Some(Winmodeselect::Below),
            2 => Some(Winmodeselect::Above),
            3 => Some(Winmodeselect::Inside),
            4 => Some(Winmodeselect::Outside),
            _ => None,
        }
    }
    #[doc = "No Window Comparison"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Winmodeselect::None
    }
    #[doc = "OUTPUT 'less than' WINLT"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == Winmodeselect::Below
    }
    #[doc = "OUTPUT 'greater than' WINHT"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == Winmodeselect::Above
    }
    #[doc = "WINLT 'less than' OUTPUT 'less than' WINHT"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == Winmodeselect::Inside
    }
    #[doc = "OUTPUT 'less than' WINLT or OUTPUT 'greater than'WINHT"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == Winmodeselect::Outside
    }
}
#[doc = "Field `WINMODE` writer - Window Comparator Mode"]
pub type WinmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Winmodeselect>;
impl<'a, REG> WinmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Window Comparison"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::None)
    }
    #[doc = "OUTPUT 'less than' WINLT"]
    #[inline(always)]
    pub fn below(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Below)
    }
    #[doc = "OUTPUT 'greater than' WINHT"]
    #[inline(always)]
    pub fn above(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Above)
    }
    #[doc = "WINLT 'less than' OUTPUT 'less than' WINHT"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Inside)
    }
    #[doc = "OUTPUT 'less than' WINLT or OUTPUT 'greater than'WINHT"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Outside)
    }
}
#[doc = "Window Mode Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winsrcselect {
    #[doc = "0: Use RESULT\\[15:0\\] as Window Comparator source"]
    Result = 0,
    #[doc = "1: Use SAMPLE\\[15:0\\] as Window Comparator source"]
    Sample = 1,
}
impl From<Winsrcselect> for bool {
    #[inline(always)]
    fn from(variant: Winsrcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINSRC` reader - Window Mode Source"]
pub type WinsrcR = crate::BitReader<Winsrcselect>;
impl WinsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Winsrcselect {
        match self.bits {
            false => Winsrcselect::Result,
            true => Winsrcselect::Sample,
        }
    }
    #[doc = "Use RESULT\\[15:0\\] as Window Comparator source"]
    #[inline(always)]
    pub fn is_result(&self) -> bool {
        *self == Winsrcselect::Result
    }
    #[doc = "Use SAMPLE\\[15:0\\] as Window Comparator source"]
    #[inline(always)]
    pub fn is_sample(&self) -> bool {
        *self == Winsrcselect::Sample
    }
}
#[doc = "Field `WINSRC` writer - Window Mode Source"]
pub type WinsrcW<'a, REG> = crate::BitWriter<'a, REG, Winsrcselect>;
impl<'a, REG> WinsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use RESULT\\[15:0\\] as Window Comparator source"]
    #[inline(always)]
    pub fn result(self) -> &'a mut crate::W<REG> {
        self.variant(Winsrcselect::Result)
    }
    #[doc = "Use SAMPLE\\[15:0\\] as Window Comparator source"]
    #[inline(always)]
    pub fn sample(self) -> &'a mut crate::W<REG> {
        self.variant(Winsrcselect::Sample)
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Comparator Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WinmodeR {
        WinmodeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Window Mode Source"]
    #[inline(always)]
    pub fn winsrc(&self) -> WinsrcR {
        WinsrcR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Comparator Mode"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WinmodeW<'_, WinctrlSpec> {
        WinmodeW::new(self, 0)
    }
    #[doc = "Bit 3 - Window Mode Source"]
    #[inline(always)]
    pub fn winsrc(&mut self) -> WinsrcW<'_, WinctrlSpec> {
        WinsrcW::new(self, 3)
    }
}
#[doc = "Window Control\n\nYou can [`read`](crate::Reg::read) this register and get [`winctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinctrlSpec;
impl crate::RegisterSpec for WinctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winctrl::R`](R) reader structure"]
impl crate::Readable for WinctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`winctrl::W`](W) writer structure"]
impl crate::Writable for WinctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WINCTRL to value 0"]
impl crate::Resettable for WinctrlSpec {}

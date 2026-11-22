#[doc = "Register `GENCTRL` reader"]
pub type R = crate::R<GenctrlSpec>;
#[doc = "Register `GENCTRL` writer"]
pub type W = crate::W<GenctrlSpec>;
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
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srcselect {
    #[doc = "0: XOSC oscillator output"]
    Xosc = 0,
    #[doc = "1: Generator input pad"]
    Gclkin = 1,
    #[doc = "2: Generic clock generator 1 output"]
    Gclkgen1 = 2,
    #[doc = "3: OSCULP32K oscillator output"]
    Osculp32k = 3,
    #[doc = "4: OSC32K oscillator output"]
    Osc32k = 4,
    #[doc = "5: XOSC32K oscillator output"]
    Xosc32k = 5,
    #[doc = "6: OSC8M oscillator output"]
    Osc8m = 6,
    #[doc = "7: DFLL48M output"]
    Dfll48m = 7,
}
impl From<Srcselect> for u8 {
    #[inline(always)]
    fn from(variant: Srcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srcselect {
    type Ux = u8;
}
impl crate::IsEnum for Srcselect {}
#[doc = "Field `SRC` reader - Source Select"]
pub type SrcR = crate::FieldReader<Srcselect>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Srcselect> {
        match self.bits {
            0 => Some(Srcselect::Xosc),
            1 => Some(Srcselect::Gclkin),
            2 => Some(Srcselect::Gclkgen1),
            3 => Some(Srcselect::Osculp32k),
            4 => Some(Srcselect::Osc32k),
            5 => Some(Srcselect::Xosc32k),
            6 => Some(Srcselect::Osc8m),
            7 => Some(Srcselect::Dfll48m),
            _ => None,
        }
    }
    #[doc = "XOSC oscillator output"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == Srcselect::Xosc
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn is_gclkin(&self) -> bool {
        *self == Srcselect::Gclkin
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn is_gclkgen1(&self) -> bool {
        *self == Srcselect::Gclkgen1
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn is_osculp32k(&self) -> bool {
        *self == Srcselect::Osculp32k
    }
    #[doc = "OSC32K oscillator output"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == Srcselect::Osc32k
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == Srcselect::Xosc32k
    }
    #[doc = "OSC8M oscillator output"]
    #[inline(always)]
    pub fn is_osc8m(&self) -> bool {
        *self == Srcselect::Osc8m
    }
    #[doc = "DFLL48M output"]
    #[inline(always)]
    pub fn is_dfll48m(&self) -> bool {
        *self == Srcselect::Dfll48m
    }
}
#[doc = "Field `SRC` writer - Source Select"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 5, Srcselect>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XOSC oscillator output"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Xosc)
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn gclkin(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Gclkin)
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn gclkgen1(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Gclkgen1)
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn osculp32k(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Osculp32k)
    }
    #[doc = "OSC32K oscillator output"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Osc32k)
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Xosc32k)
    }
    #[doc = "OSC8M oscillator output"]
    #[inline(always)]
    pub fn osc8m(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Osc8m)
    }
    #[doc = "DFLL48M output"]
    #[inline(always)]
    pub fn dfll48m(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Dfll48m)
    }
}
#[doc = "Field `GENEN` reader - Generic Clock Generator Enable"]
pub type GenenR = crate::BitReader;
#[doc = "Field `GENEN` writer - Generic Clock Generator Enable"]
pub type GenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDC` reader - Improve Duty Cycle"]
pub type IdcR = crate::BitReader;
#[doc = "Field `IDC` writer - Improve Duty Cycle"]
pub type IdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOV` reader - Output Off Value"]
pub type OovR = crate::BitReader;
#[doc = "Field `OOV` writer - Output Off Value"]
pub type OovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` reader - Output Enable"]
pub type OeR = crate::BitReader;
#[doc = "Field `OE` writer - Output Enable"]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Divide Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Divselselect {
    #[doc = "0: Divide input directly by divider factor"]
    Div1 = 0,
    #[doc = "1: Divide input by 2^(divider factor+ 1)"]
    Div2 = 1,
}
impl From<Divselselect> for bool {
    #[inline(always)]
    fn from(variant: Divselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVSEL` reader - Divide Selection"]
pub type DivselR = crate::BitReader<Divselselect>;
impl DivselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divselselect {
        match self.bits {
            false => Divselselect::Div1,
            true => Divselselect::Div2,
        }
    }
    #[doc = "Divide input directly by divider factor"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Divselselect::Div1
    }
    #[doc = "Divide input by 2^(divider factor+ 1)"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Divselselect::Div2
    }
}
#[doc = "Field `DIVSEL` writer - Divide Selection"]
pub type DivselW<'a, REG> = crate::BitWriter<'a, REG, Divselselect>;
impl<'a, REG> DivselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Divide input directly by divider factor"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Divselselect::Div1)
    }
    #[doc = "Divide input by 2^(divider factor+ 1)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Divselselect::Div2)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&self) -> GenenR {
        GenenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&self) -> IdcR {
        IdcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&self) -> OovR {
        OovR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&self) -> DivselR {
        DivselR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, GenctrlSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, GenctrlSpec> {
        SrcW::new(self, 8)
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&mut self) -> GenenW<'_, GenctrlSpec> {
        GenenW::new(self, 16)
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&mut self) -> IdcW<'_, GenctrlSpec> {
        IdcW::new(self, 17)
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&mut self) -> OovW<'_, GenctrlSpec> {
        OovW::new(self, 18)
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline(always)]
    pub fn oe(&mut self) -> OeW<'_, GenctrlSpec> {
        OeW::new(self, 19)
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&mut self) -> DivselW<'_, GenctrlSpec> {
        DivselW::new(self, 20)
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, GenctrlSpec> {
        RunstdbyW::new(self, 21)
    }
}
#[doc = "Generic Clock Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`genctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenctrlSpec;
impl crate::RegisterSpec for GenctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`genctrl::R`](R) reader structure"]
impl crate::Readable for GenctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`genctrl::W`](W) writer structure"]
impl crate::Writable for GenctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GENCTRL to value 0"]
impl crate::Resettable for GenctrlSpec {}

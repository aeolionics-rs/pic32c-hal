#[doc = "Register `INPUTCTRL` reader"]
pub type R = crate::R<InputctrlSpec>;
#[doc = "Register `INPUTCTRL` writer"]
pub type W = crate::W<InputctrlSpec>;
#[doc = "Negative Input Multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxnegselect {
    #[doc = "2: ADC AIN2 Pin"]
    Ain2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    Ain3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    Ain4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    Ain5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    Ain6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    Ain7 = 7,
    #[doc = "16: ADC AIN16 Pin"]
    Ain16 = 16,
    #[doc = "17: ADC AIN17 Pin"]
    Ain17 = 17,
    #[doc = "18: ADC AIN18 Pin"]
    Ain18 = 18,
    #[doc = "19: ADC AIN19 Pin"]
    Ain19 = 19,
    #[doc = "20: ADC AIN20 Pin"]
    Ain20 = 20,
    #[doc = "21: ADC AIN21 Pin"]
    Ain21 = 21,
    #[doc = "22: ADC AIN22 Pin"]
    Ain22 = 22,
    #[doc = "23: ADC AIN23 Pin"]
    Ain23 = 23,
    #[doc = "24: ADC AIN24 Pin"]
    Ain24 = 24,
    #[doc = "25: ADC AIN25 Pin"]
    Ain25 = 25,
    #[doc = "26: ADC AIN26 Pin"]
    Ain26 = 26,
    #[doc = "27: ADC AIN27 Pin"]
    Ain27 = 27,
    #[doc = "28: ADC AIN28 Pin"]
    Ain28 = 28,
    #[doc = "29: ADC AIN29 Pin"]
    Ain29 = 29,
    #[doc = "31: ADC AIN31 Pin"]
    Ain31 = 31,
    #[doc = "40: ADC AIN40 Pin"]
    Ain40 = 40,
    #[doc = "41: ADC AIN41 Pin"]
    Ain41 = 41,
    #[doc = "42: ADC AIN42 Pin"]
    Ain42 = 42,
    #[doc = "43: ADC AIN43 Pin"]
    Ain43 = 43,
    #[doc = "44: ADC AIN44 Pin"]
    Ain44 = 44,
    #[doc = "45: ADC AIN45 Pin"]
    Ain45 = 45,
    #[doc = "46: ADC AIN46 Pin"]
    Ain46 = 46,
    #[doc = "47: ADC AIN47 Pin"]
    Ain47 = 47,
    #[doc = "96: Analog Ground"]
    Gnd = 96,
    #[doc = "97: Positive Reference"]
    Vrefp = 97,
}
impl From<Muxnegselect> for u8 {
    #[inline(always)]
    fn from(variant: Muxnegselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Muxnegselect {
    type Ux = u8;
}
impl crate::IsEnum for Muxnegselect {}
#[doc = "Field `MUXNEG` reader - Negative Input Multiplexer"]
pub type MuxnegR = crate::FieldReader<Muxnegselect>;
impl MuxnegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Muxnegselect> {
        match self.bits {
            2 => Some(Muxnegselect::Ain2),
            3 => Some(Muxnegselect::Ain3),
            4 => Some(Muxnegselect::Ain4),
            5 => Some(Muxnegselect::Ain5),
            6 => Some(Muxnegselect::Ain6),
            7 => Some(Muxnegselect::Ain7),
            16 => Some(Muxnegselect::Ain16),
            17 => Some(Muxnegselect::Ain17),
            18 => Some(Muxnegselect::Ain18),
            19 => Some(Muxnegselect::Ain19),
            20 => Some(Muxnegselect::Ain20),
            21 => Some(Muxnegselect::Ain21),
            22 => Some(Muxnegselect::Ain22),
            23 => Some(Muxnegselect::Ain23),
            24 => Some(Muxnegselect::Ain24),
            25 => Some(Muxnegselect::Ain25),
            26 => Some(Muxnegselect::Ain26),
            27 => Some(Muxnegselect::Ain27),
            28 => Some(Muxnegselect::Ain28),
            29 => Some(Muxnegselect::Ain29),
            31 => Some(Muxnegselect::Ain31),
            40 => Some(Muxnegselect::Ain40),
            41 => Some(Muxnegselect::Ain41),
            42 => Some(Muxnegselect::Ain42),
            43 => Some(Muxnegselect::Ain43),
            44 => Some(Muxnegselect::Ain44),
            45 => Some(Muxnegselect::Ain45),
            46 => Some(Muxnegselect::Ain46),
            47 => Some(Muxnegselect::Ain47),
            96 => Some(Muxnegselect::Gnd),
            97 => Some(Muxnegselect::Vrefp),
            _ => None,
        }
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == Muxnegselect::Ain2
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == Muxnegselect::Ain3
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == Muxnegselect::Ain4
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == Muxnegselect::Ain5
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == Muxnegselect::Ain6
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == Muxnegselect::Ain7
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn is_ain16(&self) -> bool {
        *self == Muxnegselect::Ain16
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn is_ain17(&self) -> bool {
        *self == Muxnegselect::Ain17
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn is_ain18(&self) -> bool {
        *self == Muxnegselect::Ain18
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn is_ain19(&self) -> bool {
        *self == Muxnegselect::Ain19
    }
    #[doc = "ADC AIN20 Pin"]
    #[inline(always)]
    pub fn is_ain20(&self) -> bool {
        *self == Muxnegselect::Ain20
    }
    #[doc = "ADC AIN21 Pin"]
    #[inline(always)]
    pub fn is_ain21(&self) -> bool {
        *self == Muxnegselect::Ain21
    }
    #[doc = "ADC AIN22 Pin"]
    #[inline(always)]
    pub fn is_ain22(&self) -> bool {
        *self == Muxnegselect::Ain22
    }
    #[doc = "ADC AIN23 Pin"]
    #[inline(always)]
    pub fn is_ain23(&self) -> bool {
        *self == Muxnegselect::Ain23
    }
    #[doc = "ADC AIN24 Pin"]
    #[inline(always)]
    pub fn is_ain24(&self) -> bool {
        *self == Muxnegselect::Ain24
    }
    #[doc = "ADC AIN25 Pin"]
    #[inline(always)]
    pub fn is_ain25(&self) -> bool {
        *self == Muxnegselect::Ain25
    }
    #[doc = "ADC AIN26 Pin"]
    #[inline(always)]
    pub fn is_ain26(&self) -> bool {
        *self == Muxnegselect::Ain26
    }
    #[doc = "ADC AIN27 Pin"]
    #[inline(always)]
    pub fn is_ain27(&self) -> bool {
        *self == Muxnegselect::Ain27
    }
    #[doc = "ADC AIN28 Pin"]
    #[inline(always)]
    pub fn is_ain28(&self) -> bool {
        *self == Muxnegselect::Ain28
    }
    #[doc = "ADC AIN29 Pin"]
    #[inline(always)]
    pub fn is_ain29(&self) -> bool {
        *self == Muxnegselect::Ain29
    }
    #[doc = "ADC AIN31 Pin"]
    #[inline(always)]
    pub fn is_ain31(&self) -> bool {
        *self == Muxnegselect::Ain31
    }
    #[doc = "ADC AIN40 Pin"]
    #[inline(always)]
    pub fn is_ain40(&self) -> bool {
        *self == Muxnegselect::Ain40
    }
    #[doc = "ADC AIN41 Pin"]
    #[inline(always)]
    pub fn is_ain41(&self) -> bool {
        *self == Muxnegselect::Ain41
    }
    #[doc = "ADC AIN42 Pin"]
    #[inline(always)]
    pub fn is_ain42(&self) -> bool {
        *self == Muxnegselect::Ain42
    }
    #[doc = "ADC AIN43 Pin"]
    #[inline(always)]
    pub fn is_ain43(&self) -> bool {
        *self == Muxnegselect::Ain43
    }
    #[doc = "ADC AIN44 Pin"]
    #[inline(always)]
    pub fn is_ain44(&self) -> bool {
        *self == Muxnegselect::Ain44
    }
    #[doc = "ADC AIN45 Pin"]
    #[inline(always)]
    pub fn is_ain45(&self) -> bool {
        *self == Muxnegselect::Ain45
    }
    #[doc = "ADC AIN46 Pin"]
    #[inline(always)]
    pub fn is_ain46(&self) -> bool {
        *self == Muxnegselect::Ain46
    }
    #[doc = "ADC AIN47 Pin"]
    #[inline(always)]
    pub fn is_ain47(&self) -> bool {
        *self == Muxnegselect::Ain47
    }
    #[doc = "Analog Ground"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == Muxnegselect::Gnd
    }
    #[doc = "Positive Reference"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == Muxnegselect::Vrefp
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input Multiplexer"]
pub type MuxnegW<'a, REG> = crate::FieldWriter<'a, REG, 7, Muxnegselect>;
impl<'a, REG> MuxnegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain7)
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn ain16(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain16)
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn ain17(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain17)
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn ain18(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain18)
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn ain19(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain19)
    }
    #[doc = "ADC AIN20 Pin"]
    #[inline(always)]
    pub fn ain20(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain20)
    }
    #[doc = "ADC AIN21 Pin"]
    #[inline(always)]
    pub fn ain21(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain21)
    }
    #[doc = "ADC AIN22 Pin"]
    #[inline(always)]
    pub fn ain22(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain22)
    }
    #[doc = "ADC AIN23 Pin"]
    #[inline(always)]
    pub fn ain23(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain23)
    }
    #[doc = "ADC AIN24 Pin"]
    #[inline(always)]
    pub fn ain24(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain24)
    }
    #[doc = "ADC AIN25 Pin"]
    #[inline(always)]
    pub fn ain25(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain25)
    }
    #[doc = "ADC AIN26 Pin"]
    #[inline(always)]
    pub fn ain26(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain26)
    }
    #[doc = "ADC AIN27 Pin"]
    #[inline(always)]
    pub fn ain27(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain27)
    }
    #[doc = "ADC AIN28 Pin"]
    #[inline(always)]
    pub fn ain28(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain28)
    }
    #[doc = "ADC AIN29 Pin"]
    #[inline(always)]
    pub fn ain29(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain29)
    }
    #[doc = "ADC AIN31 Pin"]
    #[inline(always)]
    pub fn ain31(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain31)
    }
    #[doc = "ADC AIN40 Pin"]
    #[inline(always)]
    pub fn ain40(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain40)
    }
    #[doc = "ADC AIN41 Pin"]
    #[inline(always)]
    pub fn ain41(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain41)
    }
    #[doc = "ADC AIN42 Pin"]
    #[inline(always)]
    pub fn ain42(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain42)
    }
    #[doc = "ADC AIN43 Pin"]
    #[inline(always)]
    pub fn ain43(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain43)
    }
    #[doc = "ADC AIN44 Pin"]
    #[inline(always)]
    pub fn ain44(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain44)
    }
    #[doc = "ADC AIN45 Pin"]
    #[inline(always)]
    pub fn ain45(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain45)
    }
    #[doc = "ADC AIN46 Pin"]
    #[inline(always)]
    pub fn ain46(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain46)
    }
    #[doc = "ADC AIN47 Pin"]
    #[inline(always)]
    pub fn ain47(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain47)
    }
    #[doc = "Analog Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Gnd)
    }
    #[doc = "Positive Reference"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Vrefp)
    }
}
#[doc = "Positive Input Multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxposselect {
    #[doc = "2: ADC AIN2 Pin"]
    Ain2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    Ain3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    Ain4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    Ain5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    Ain6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    Ain7 = 7,
    #[doc = "16: ADC AIN16 Pin"]
    Ain16 = 16,
    #[doc = "17: ADC AIN17 Pin"]
    Ain17 = 17,
    #[doc = "18: ADC AIN18 Pin"]
    Ain18 = 18,
    #[doc = "19: ADC AIN19 Pin"]
    Ain19 = 19,
    #[doc = "20: ADC AIN20 Pin"]
    Ain20 = 20,
    #[doc = "21: ADC AIN21 Pin"]
    Ain21 = 21,
    #[doc = "22: ADC AIN22 Pin"]
    Ain22 = 22,
    #[doc = "23: ADC AIN23 Pin"]
    Ain23 = 23,
    #[doc = "24: ADC AIN24 Pin"]
    Ain24 = 24,
    #[doc = "25: ADC AIN25 Pin"]
    Ain25 = 25,
    #[doc = "26: ADC AIN26 Pin"]
    Ain26 = 26,
    #[doc = "27: ADC AIN27 Pin"]
    Ain27 = 27,
    #[doc = "28: ADC AIN28 Pin"]
    Ain28 = 28,
    #[doc = "29: ADC AIN29 Pin"]
    Ain29 = 29,
    #[doc = "31: ADC AIN31 Pin"]
    Ain31 = 31,
    #[doc = "40: ADC AIN40 Pin"]
    Ain40 = 40,
    #[doc = "41: ADC AIN41 Pin"]
    Ain41 = 41,
    #[doc = "42: ADC AIN42 Pin"]
    Ain42 = 42,
    #[doc = "43: ADC AIN43 Pin"]
    Ain43 = 43,
    #[doc = "44: ADC AIN44 Pin"]
    Ain44 = 44,
    #[doc = "45: ADC AIN45 Pin"]
    Ain45 = 45,
    #[doc = "46: ADC AIN46 Pin"]
    Ain46 = 46,
    #[doc = "47: ADC AIN47 Pin"]
    Ain47 = 47,
    #[doc = "96: Analog Ground"]
    Gnd = 96,
    #[doc = "97: 1/4 Scaled Core Supply"]
    Vddcorediv4 = 97,
    #[doc = "98: Temperature Sensor"]
    Tempsense = 98,
    #[doc = "100: 1/10 Scaled VDDIO Supply"]
    Vddiodiv10 = 100,
    #[doc = "101: 1/10 Scaled VDDIO2 Supply"]
    Vddio2div10 = 101,
    #[doc = "105: Output from AC VREFSCALE0"]
    Acvrefscale0 = 105,
    #[doc = "106: Output from AC VREFSCALE1"]
    Acvrefscale1 = 106,
    #[doc = "112: Test Buffer"]
    Testbuf = 112,
    #[doc = "126: PTC"]
    Ptc = 126,
}
impl From<Muxposselect> for u8 {
    #[inline(always)]
    fn from(variant: Muxposselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Muxposselect {
    type Ux = u8;
}
impl crate::IsEnum for Muxposselect {}
#[doc = "Field `MUXPOS` reader - Positive Input Multiplexer"]
pub type MuxposR = crate::FieldReader<Muxposselect>;
impl MuxposR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Muxposselect> {
        match self.bits {
            2 => Some(Muxposselect::Ain2),
            3 => Some(Muxposselect::Ain3),
            4 => Some(Muxposselect::Ain4),
            5 => Some(Muxposselect::Ain5),
            6 => Some(Muxposselect::Ain6),
            7 => Some(Muxposselect::Ain7),
            16 => Some(Muxposselect::Ain16),
            17 => Some(Muxposselect::Ain17),
            18 => Some(Muxposselect::Ain18),
            19 => Some(Muxposselect::Ain19),
            20 => Some(Muxposselect::Ain20),
            21 => Some(Muxposselect::Ain21),
            22 => Some(Muxposselect::Ain22),
            23 => Some(Muxposselect::Ain23),
            24 => Some(Muxposselect::Ain24),
            25 => Some(Muxposselect::Ain25),
            26 => Some(Muxposselect::Ain26),
            27 => Some(Muxposselect::Ain27),
            28 => Some(Muxposselect::Ain28),
            29 => Some(Muxposselect::Ain29),
            31 => Some(Muxposselect::Ain31),
            40 => Some(Muxposselect::Ain40),
            41 => Some(Muxposselect::Ain41),
            42 => Some(Muxposselect::Ain42),
            43 => Some(Muxposselect::Ain43),
            44 => Some(Muxposselect::Ain44),
            45 => Some(Muxposselect::Ain45),
            46 => Some(Muxposselect::Ain46),
            47 => Some(Muxposselect::Ain47),
            96 => Some(Muxposselect::Gnd),
            97 => Some(Muxposselect::Vddcorediv4),
            98 => Some(Muxposselect::Tempsense),
            100 => Some(Muxposselect::Vddiodiv10),
            101 => Some(Muxposselect::Vddio2div10),
            105 => Some(Muxposselect::Acvrefscale0),
            106 => Some(Muxposselect::Acvrefscale1),
            112 => Some(Muxposselect::Testbuf),
            126 => Some(Muxposselect::Ptc),
            _ => None,
        }
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == Muxposselect::Ain2
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == Muxposselect::Ain3
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == Muxposselect::Ain4
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == Muxposselect::Ain5
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == Muxposselect::Ain6
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == Muxposselect::Ain7
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn is_ain16(&self) -> bool {
        *self == Muxposselect::Ain16
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn is_ain17(&self) -> bool {
        *self == Muxposselect::Ain17
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn is_ain18(&self) -> bool {
        *self == Muxposselect::Ain18
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn is_ain19(&self) -> bool {
        *self == Muxposselect::Ain19
    }
    #[doc = "ADC AIN20 Pin"]
    #[inline(always)]
    pub fn is_ain20(&self) -> bool {
        *self == Muxposselect::Ain20
    }
    #[doc = "ADC AIN21 Pin"]
    #[inline(always)]
    pub fn is_ain21(&self) -> bool {
        *self == Muxposselect::Ain21
    }
    #[doc = "ADC AIN22 Pin"]
    #[inline(always)]
    pub fn is_ain22(&self) -> bool {
        *self == Muxposselect::Ain22
    }
    #[doc = "ADC AIN23 Pin"]
    #[inline(always)]
    pub fn is_ain23(&self) -> bool {
        *self == Muxposselect::Ain23
    }
    #[doc = "ADC AIN24 Pin"]
    #[inline(always)]
    pub fn is_ain24(&self) -> bool {
        *self == Muxposselect::Ain24
    }
    #[doc = "ADC AIN25 Pin"]
    #[inline(always)]
    pub fn is_ain25(&self) -> bool {
        *self == Muxposselect::Ain25
    }
    #[doc = "ADC AIN26 Pin"]
    #[inline(always)]
    pub fn is_ain26(&self) -> bool {
        *self == Muxposselect::Ain26
    }
    #[doc = "ADC AIN27 Pin"]
    #[inline(always)]
    pub fn is_ain27(&self) -> bool {
        *self == Muxposselect::Ain27
    }
    #[doc = "ADC AIN28 Pin"]
    #[inline(always)]
    pub fn is_ain28(&self) -> bool {
        *self == Muxposselect::Ain28
    }
    #[doc = "ADC AIN29 Pin"]
    #[inline(always)]
    pub fn is_ain29(&self) -> bool {
        *self == Muxposselect::Ain29
    }
    #[doc = "ADC AIN31 Pin"]
    #[inline(always)]
    pub fn is_ain31(&self) -> bool {
        *self == Muxposselect::Ain31
    }
    #[doc = "ADC AIN40 Pin"]
    #[inline(always)]
    pub fn is_ain40(&self) -> bool {
        *self == Muxposselect::Ain40
    }
    #[doc = "ADC AIN41 Pin"]
    #[inline(always)]
    pub fn is_ain41(&self) -> bool {
        *self == Muxposselect::Ain41
    }
    #[doc = "ADC AIN42 Pin"]
    #[inline(always)]
    pub fn is_ain42(&self) -> bool {
        *self == Muxposselect::Ain42
    }
    #[doc = "ADC AIN43 Pin"]
    #[inline(always)]
    pub fn is_ain43(&self) -> bool {
        *self == Muxposselect::Ain43
    }
    #[doc = "ADC AIN44 Pin"]
    #[inline(always)]
    pub fn is_ain44(&self) -> bool {
        *self == Muxposselect::Ain44
    }
    #[doc = "ADC AIN45 Pin"]
    #[inline(always)]
    pub fn is_ain45(&self) -> bool {
        *self == Muxposselect::Ain45
    }
    #[doc = "ADC AIN46 Pin"]
    #[inline(always)]
    pub fn is_ain46(&self) -> bool {
        *self == Muxposselect::Ain46
    }
    #[doc = "ADC AIN47 Pin"]
    #[inline(always)]
    pub fn is_ain47(&self) -> bool {
        *self == Muxposselect::Ain47
    }
    #[doc = "Analog Ground"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == Muxposselect::Gnd
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn is_vddcorediv4(&self) -> bool {
        *self == Muxposselect::Vddcorediv4
    }
    #[doc = "Temperature Sensor"]
    #[inline(always)]
    pub fn is_tempsense(&self) -> bool {
        *self == Muxposselect::Tempsense
    }
    #[doc = "1/10 Scaled VDDIO Supply"]
    #[inline(always)]
    pub fn is_vddiodiv10(&self) -> bool {
        *self == Muxposselect::Vddiodiv10
    }
    #[doc = "1/10 Scaled VDDIO2 Supply"]
    #[inline(always)]
    pub fn is_vddio2div10(&self) -> bool {
        *self == Muxposselect::Vddio2div10
    }
    #[doc = "Output from AC VREFSCALE0"]
    #[inline(always)]
    pub fn is_acvrefscale0(&self) -> bool {
        *self == Muxposselect::Acvrefscale0
    }
    #[doc = "Output from AC VREFSCALE1"]
    #[inline(always)]
    pub fn is_acvrefscale1(&self) -> bool {
        *self == Muxposselect::Acvrefscale1
    }
    #[doc = "Test Buffer"]
    #[inline(always)]
    pub fn is_testbuf(&self) -> bool {
        *self == Muxposselect::Testbuf
    }
    #[doc = "PTC"]
    #[inline(always)]
    pub fn is_ptc(&self) -> bool {
        *self == Muxposselect::Ptc
    }
}
#[doc = "Field `MUXPOS` writer - Positive Input Multiplexer"]
pub type MuxposW<'a, REG> = crate::FieldWriter<'a, REG, 7, Muxposselect>;
impl<'a, REG> MuxposW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain7)
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn ain16(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain16)
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn ain17(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain17)
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn ain18(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain18)
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn ain19(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain19)
    }
    #[doc = "ADC AIN20 Pin"]
    #[inline(always)]
    pub fn ain20(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain20)
    }
    #[doc = "ADC AIN21 Pin"]
    #[inline(always)]
    pub fn ain21(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain21)
    }
    #[doc = "ADC AIN22 Pin"]
    #[inline(always)]
    pub fn ain22(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain22)
    }
    #[doc = "ADC AIN23 Pin"]
    #[inline(always)]
    pub fn ain23(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain23)
    }
    #[doc = "ADC AIN24 Pin"]
    #[inline(always)]
    pub fn ain24(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain24)
    }
    #[doc = "ADC AIN25 Pin"]
    #[inline(always)]
    pub fn ain25(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain25)
    }
    #[doc = "ADC AIN26 Pin"]
    #[inline(always)]
    pub fn ain26(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain26)
    }
    #[doc = "ADC AIN27 Pin"]
    #[inline(always)]
    pub fn ain27(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain27)
    }
    #[doc = "ADC AIN28 Pin"]
    #[inline(always)]
    pub fn ain28(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain28)
    }
    #[doc = "ADC AIN29 Pin"]
    #[inline(always)]
    pub fn ain29(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain29)
    }
    #[doc = "ADC AIN31 Pin"]
    #[inline(always)]
    pub fn ain31(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain31)
    }
    #[doc = "ADC AIN40 Pin"]
    #[inline(always)]
    pub fn ain40(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain40)
    }
    #[doc = "ADC AIN41 Pin"]
    #[inline(always)]
    pub fn ain41(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain41)
    }
    #[doc = "ADC AIN42 Pin"]
    #[inline(always)]
    pub fn ain42(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain42)
    }
    #[doc = "ADC AIN43 Pin"]
    #[inline(always)]
    pub fn ain43(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain43)
    }
    #[doc = "ADC AIN44 Pin"]
    #[inline(always)]
    pub fn ain44(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain44)
    }
    #[doc = "ADC AIN45 Pin"]
    #[inline(always)]
    pub fn ain45(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain45)
    }
    #[doc = "ADC AIN46 Pin"]
    #[inline(always)]
    pub fn ain46(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain46)
    }
    #[doc = "ADC AIN47 Pin"]
    #[inline(always)]
    pub fn ain47(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain47)
    }
    #[doc = "Analog Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Gnd)
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn vddcorediv4(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Vddcorediv4)
    }
    #[doc = "Temperature Sensor"]
    #[inline(always)]
    pub fn tempsense(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Tempsense)
    }
    #[doc = "1/10 Scaled VDDIO Supply"]
    #[inline(always)]
    pub fn vddiodiv10(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Vddiodiv10)
    }
    #[doc = "1/10 Scaled VDDIO2 Supply"]
    #[inline(always)]
    pub fn vddio2div10(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Vddio2div10)
    }
    #[doc = "Output from AC VREFSCALE0"]
    #[inline(always)]
    pub fn acvrefscale0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Acvrefscale0)
    }
    #[doc = "Output from AC VREFSCALE1"]
    #[inline(always)]
    pub fn acvrefscale1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Acvrefscale1)
    }
    #[doc = "Test Buffer"]
    #[inline(always)]
    pub fn testbuf(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Testbuf)
    }
    #[doc = "PTC"]
    #[inline(always)]
    pub fn ptc(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ptc)
    }
}
impl R {
    #[doc = "Bits 0:6 - Negative Input Multiplexer"]
    #[inline(always)]
    pub fn muxneg(&self) -> MuxnegR {
        MuxnegR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Positive Input Multiplexer"]
    #[inline(always)]
    pub fn muxpos(&self) -> MuxposR {
        MuxposR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Negative Input Multiplexer"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MuxnegW<'_, InputctrlSpec> {
        MuxnegW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Positive Input Multiplexer"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MuxposW<'_, InputctrlSpec> {
        MuxposW::new(self, 8)
    }
}
#[doc = "Input Multiplexer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`inputctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputctrlSpec;
impl crate::RegisterSpec for InputctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputctrl::R`](R) reader structure"]
impl crate::Readable for InputctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`inputctrl::W`](W) writer structure"]
impl crate::Writable for InputctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTCTRL to value 0"]
impl crate::Resettable for InputctrlSpec {}

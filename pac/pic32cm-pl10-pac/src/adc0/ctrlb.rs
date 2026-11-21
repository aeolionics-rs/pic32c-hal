#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescalerselect {
    #[doc = "0: APB clock divided by 2"]
    Div2 = 0,
    #[doc = "1: APB clock divided by 3"]
    Div3 = 1,
    #[doc = "2: APB clock divided by 4"]
    Div4 = 2,
    #[doc = "3: APB clock divided by 5"]
    Div5 = 3,
    #[doc = "4: APB clock divided by 6"]
    Div6 = 4,
    #[doc = "5: APB clock divided by 7"]
    Div7 = 5,
    #[doc = "6: APB clock divided by 8"]
    Div8 = 6,
    #[doc = "7: APB clock divided by 9"]
    Div9 = 7,
    #[doc = "8: APB clock divided by 10"]
    Div10 = 8,
    #[doc = "9: APB clock divided by 11"]
    Div11 = 9,
    #[doc = "10: APB clock divided by 12"]
    Div12 = 10,
    #[doc = "11: APB clock divided by 13"]
    Div13 = 11,
    #[doc = "12: APB clock divided by 14"]
    Div14 = 12,
    #[doc = "13: APB clock divided by 15"]
    Div15 = 13,
    #[doc = "14: APB clock divided by 16"]
    Div16 = 14,
    #[doc = "15: APB clock divided by 17"]
    Div17 = 15,
    #[doc = "16: APB clock divided by 18"]
    Div18 = 16,
    #[doc = "17: APB clock divided by 19"]
    Div19 = 17,
    #[doc = "18: APB clock divided by 20"]
    Div20 = 18,
    #[doc = "19: APB clock divided by 21"]
    Div21 = 19,
    #[doc = "20: APB clock divided by 22"]
    Div22 = 20,
    #[doc = "21: APB clock divided by 23"]
    Div23 = 21,
    #[doc = "22: APB clock divided by 24"]
    Div24 = 22,
    #[doc = "23: APB clock divided by 25"]
    Div25 = 23,
    #[doc = "24: APB clock divided by 26"]
    Div26 = 24,
    #[doc = "25: APB clock divided by 27"]
    Div27 = 25,
    #[doc = "26: APB clock divided by 28"]
    Div28 = 26,
    #[doc = "27: APB clock divided by 29"]
    Div29 = 27,
    #[doc = "28: APB clock divided by 30"]
    Div30 = 28,
    #[doc = "29: APB clock divided by 31"]
    Div31 = 29,
    #[doc = "30: APB clock divided by 32"]
    Div32 = 30,
}
impl From<Prescalerselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescalerselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescalerselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescalerselect {}
#[doc = "Field `PRESCALER` reader - Prescaler"]
pub type PrescalerR = crate::FieldReader<Prescalerselect>;
impl PrescalerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prescalerselect> {
        match self.bits {
            0 => Some(Prescalerselect::Div2),
            1 => Some(Prescalerselect::Div3),
            2 => Some(Prescalerselect::Div4),
            3 => Some(Prescalerselect::Div5),
            4 => Some(Prescalerselect::Div6),
            5 => Some(Prescalerselect::Div7),
            6 => Some(Prescalerselect::Div8),
            7 => Some(Prescalerselect::Div9),
            8 => Some(Prescalerselect::Div10),
            9 => Some(Prescalerselect::Div11),
            10 => Some(Prescalerselect::Div12),
            11 => Some(Prescalerselect::Div13),
            12 => Some(Prescalerselect::Div14),
            13 => Some(Prescalerselect::Div15),
            14 => Some(Prescalerselect::Div16),
            15 => Some(Prescalerselect::Div17),
            16 => Some(Prescalerselect::Div18),
            17 => Some(Prescalerselect::Div19),
            18 => Some(Prescalerselect::Div20),
            19 => Some(Prescalerselect::Div21),
            20 => Some(Prescalerselect::Div22),
            21 => Some(Prescalerselect::Div23),
            22 => Some(Prescalerselect::Div24),
            23 => Some(Prescalerselect::Div25),
            24 => Some(Prescalerselect::Div26),
            25 => Some(Prescalerselect::Div27),
            26 => Some(Prescalerselect::Div28),
            27 => Some(Prescalerselect::Div29),
            28 => Some(Prescalerselect::Div30),
            29 => Some(Prescalerselect::Div31),
            30 => Some(Prescalerselect::Div32),
            _ => None,
        }
    }
    #[doc = "APB clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescalerselect::Div2
    }
    #[doc = "APB clock divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Prescalerselect::Div3
    }
    #[doc = "APB clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescalerselect::Div4
    }
    #[doc = "APB clock divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Prescalerselect::Div5
    }
    #[doc = "APB clock divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Prescalerselect::Div6
    }
    #[doc = "APB clock divided by 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Prescalerselect::Div7
    }
    #[doc = "APB clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescalerselect::Div8
    }
    #[doc = "APB clock divided by 9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == Prescalerselect::Div9
    }
    #[doc = "APB clock divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Prescalerselect::Div10
    }
    #[doc = "APB clock divided by 11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == Prescalerselect::Div11
    }
    #[doc = "APB clock divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Prescalerselect::Div12
    }
    #[doc = "APB clock divided by 13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == Prescalerselect::Div13
    }
    #[doc = "APB clock divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Prescalerselect::Div14
    }
    #[doc = "APB clock divided by 15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == Prescalerselect::Div15
    }
    #[doc = "APB clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescalerselect::Div16
    }
    #[doc = "APB clock divided by 17"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == Prescalerselect::Div17
    }
    #[doc = "APB clock divided by 18"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == Prescalerselect::Div18
    }
    #[doc = "APB clock divided by 19"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == Prescalerselect::Div19
    }
    #[doc = "APB clock divided by 20"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == Prescalerselect::Div20
    }
    #[doc = "APB clock divided by 21"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == Prescalerselect::Div21
    }
    #[doc = "APB clock divided by 22"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == Prescalerselect::Div22
    }
    #[doc = "APB clock divided by 23"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == Prescalerselect::Div23
    }
    #[doc = "APB clock divided by 24"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == Prescalerselect::Div24
    }
    #[doc = "APB clock divided by 25"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == Prescalerselect::Div25
    }
    #[doc = "APB clock divided by 26"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == Prescalerselect::Div26
    }
    #[doc = "APB clock divided by 27"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == Prescalerselect::Div27
    }
    #[doc = "APB clock divided by 28"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == Prescalerselect::Div28
    }
    #[doc = "APB clock divided by 29"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == Prescalerselect::Div29
    }
    #[doc = "APB clock divided by 30"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == Prescalerselect::Div30
    }
    #[doc = "APB clock divided by 31"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == Prescalerselect::Div31
    }
    #[doc = "APB clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Prescalerselect::Div32
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 5, Prescalerselect>;
impl<'a, REG> PrescalerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div2)
    }
    #[doc = "APB clock divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div3)
    }
    #[doc = "APB clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div4)
    }
    #[doc = "APB clock divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div5)
    }
    #[doc = "APB clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div6)
    }
    #[doc = "APB clock divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div7)
    }
    #[doc = "APB clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div8)
    }
    #[doc = "APB clock divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div9)
    }
    #[doc = "APB clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div10)
    }
    #[doc = "APB clock divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div11)
    }
    #[doc = "APB clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div12)
    }
    #[doc = "APB clock divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div13)
    }
    #[doc = "APB clock divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div14)
    }
    #[doc = "APB clock divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div15)
    }
    #[doc = "APB clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div16)
    }
    #[doc = "APB clock divided by 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div17)
    }
    #[doc = "APB clock divided by 18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div18)
    }
    #[doc = "APB clock divided by 19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div19)
    }
    #[doc = "APB clock divided by 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div20)
    }
    #[doc = "APB clock divided by 21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div21)
    }
    #[doc = "APB clock divided by 22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div22)
    }
    #[doc = "APB clock divided by 23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div23)
    }
    #[doc = "APB clock divided by 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div24)
    }
    #[doc = "APB clock divided by 25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div25)
    }
    #[doc = "APB clock divided by 26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div26)
    }
    #[doc = "APB clock divided by 27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div27)
    }
    #[doc = "APB clock divided by 28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div28)
    }
    #[doc = "APB clock divided by 29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div29)
    }
    #[doc = "APB clock divided by 30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div30)
    }
    #[doc = "APB clock divided by 31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div31)
    }
    #[doc = "APB clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div32)
    }
}
#[doc = "Field `TIMEBASE` reader - Timebase"]
pub type TimebaseR = crate::FieldReader;
#[doc = "Field `TIMEBASE` writer - Timebase"]
pub type TimebaseW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Timebase"]
    #[inline(always)]
    pub fn timebase(&self) -> TimebaseR {
        TimebaseR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PrescalerW<'_, CtrlbSpec> {
        PrescalerW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Timebase"]
    #[inline(always)]
    pub fn timebase(&mut self) -> TimebaseW<'_, CtrlbSpec> {
        TimebaseW::new(self, 8)
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

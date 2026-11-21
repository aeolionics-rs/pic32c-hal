#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Event 0 Pin Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pid0select {
    #[doc = "0: Event actions will be executed on pin 0"]
    Pin0 = 0,
    #[doc = "1: Event actions will be executed on pin 1"]
    Pin1 = 1,
    #[doc = "2: Event actions will be executed on pin 2"]
    Pin2 = 2,
    #[doc = "3: Event actions will be executed on pin 3"]
    Pin3 = 3,
    #[doc = "4: Event actions will be executed on pin 4"]
    Pin4 = 4,
    #[doc = "5: Event actions will be executed on pin 5"]
    Pin5 = 5,
    #[doc = "6: Event actions will be executed on pin 6"]
    Pin6 = 6,
    #[doc = "7: Event actions will be executed on pin 7"]
    Pin7 = 7,
    #[doc = "8: Event actions will be executed on pin 8"]
    Pin8 = 8,
    #[doc = "9: Event actions will be executed on pin 9"]
    Pin9 = 9,
    #[doc = "10: Event actions will be executed on pin 10"]
    Pin10 = 10,
    #[doc = "11: Event actions will be executed on pin 11"]
    Pin11 = 11,
    #[doc = "12: Event actions will be executed on pin 12"]
    Pin12 = 12,
    #[doc = "13: Event actions will be executed on pin 13"]
    Pin13 = 13,
    #[doc = "14: Event actions will be executed on pin 14"]
    Pin14 = 14,
    #[doc = "15: Event actions will be executed on pin 15"]
    Pin15 = 15,
    #[doc = "16: Event actions will be executed on pin 16"]
    Pin16 = 16,
    #[doc = "17: Event actions will be executed on pin 17"]
    Pin17 = 17,
    #[doc = "18: Event actions will be executed on pin 18"]
    Pin18 = 18,
    #[doc = "19: Event actions will be executed on pin 19"]
    Pin19 = 19,
    #[doc = "20: Event actions will be executed on pin 20"]
    Pin20 = 20,
    #[doc = "21: Event actions will be executed on pin 21"]
    Pin21 = 21,
    #[doc = "22: Event actions will be executed on pin 22"]
    Pin22 = 22,
    #[doc = "23: Event actions will be executed on pin 23"]
    Pin23 = 23,
    #[doc = "24: Event actions will be executed on pin 24"]
    Pin24 = 24,
    #[doc = "25: Event actions will be executed on pin 25"]
    Pin25 = 25,
    #[doc = "26: Event actions will be executed on pin 26"]
    Pin26 = 26,
    #[doc = "27: Event actions will be executed on pin 27"]
    Pin27 = 27,
    #[doc = "28: Event actions will be executed on pin 28"]
    Pin28 = 28,
    #[doc = "29: Event actions will be executed on pin 29"]
    Pin29 = 29,
    #[doc = "30: Event actions will be executed on pin 30"]
    Pin30 = 30,
    #[doc = "31: Event actions will be executed on pin 31"]
    Pin31 = 31,
}
impl From<Pid0select> for u8 {
    #[inline(always)]
    fn from(variant: Pid0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pid0select {
    type Ux = u8;
}
impl crate::IsEnum for Pid0select {}
#[doc = "Field `PID0` reader - Event 0 Pin Identifier"]
pub type Pid0R = crate::FieldReader<Pid0select>;
impl Pid0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pid0select {
        match self.bits {
            0 => Pid0select::Pin0,
            1 => Pid0select::Pin1,
            2 => Pid0select::Pin2,
            3 => Pid0select::Pin3,
            4 => Pid0select::Pin4,
            5 => Pid0select::Pin5,
            6 => Pid0select::Pin6,
            7 => Pid0select::Pin7,
            8 => Pid0select::Pin8,
            9 => Pid0select::Pin9,
            10 => Pid0select::Pin10,
            11 => Pid0select::Pin11,
            12 => Pid0select::Pin12,
            13 => Pid0select::Pin13,
            14 => Pid0select::Pin14,
            15 => Pid0select::Pin15,
            16 => Pid0select::Pin16,
            17 => Pid0select::Pin17,
            18 => Pid0select::Pin18,
            19 => Pid0select::Pin19,
            20 => Pid0select::Pin20,
            21 => Pid0select::Pin21,
            22 => Pid0select::Pin22,
            23 => Pid0select::Pin23,
            24 => Pid0select::Pin24,
            25 => Pid0select::Pin25,
            26 => Pid0select::Pin26,
            27 => Pid0select::Pin27,
            28 => Pid0select::Pin28,
            29 => Pid0select::Pin29,
            30 => Pid0select::Pin30,
            31 => Pid0select::Pin31,
            _ => unreachable!(),
        }
    }
    #[doc = "Event actions will be executed on pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Pid0select::Pin0
    }
    #[doc = "Event actions will be executed on pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Pid0select::Pin1
    }
    #[doc = "Event actions will be executed on pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Pid0select::Pin2
    }
    #[doc = "Event actions will be executed on pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Pid0select::Pin3
    }
    #[doc = "Event actions will be executed on pin 4"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == Pid0select::Pin4
    }
    #[doc = "Event actions will be executed on pin 5"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == Pid0select::Pin5
    }
    #[doc = "Event actions will be executed on pin 6"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == Pid0select::Pin6
    }
    #[doc = "Event actions will be executed on pin 7"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == Pid0select::Pin7
    }
    #[doc = "Event actions will be executed on pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Pid0select::Pin8
    }
    #[doc = "Event actions will be executed on pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Pid0select::Pin9
    }
    #[doc = "Event actions will be executed on pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Pid0select::Pin10
    }
    #[doc = "Event actions will be executed on pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Pid0select::Pin11
    }
    #[doc = "Event actions will be executed on pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == Pid0select::Pin12
    }
    #[doc = "Event actions will be executed on pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == Pid0select::Pin13
    }
    #[doc = "Event actions will be executed on pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == Pid0select::Pin14
    }
    #[doc = "Event actions will be executed on pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == Pid0select::Pin15
    }
    #[doc = "Event actions will be executed on pin 16"]
    #[inline(always)]
    pub fn is_pin16(&self) -> bool {
        *self == Pid0select::Pin16
    }
    #[doc = "Event actions will be executed on pin 17"]
    #[inline(always)]
    pub fn is_pin17(&self) -> bool {
        *self == Pid0select::Pin17
    }
    #[doc = "Event actions will be executed on pin 18"]
    #[inline(always)]
    pub fn is_pin18(&self) -> bool {
        *self == Pid0select::Pin18
    }
    #[doc = "Event actions will be executed on pin 19"]
    #[inline(always)]
    pub fn is_pin19(&self) -> bool {
        *self == Pid0select::Pin19
    }
    #[doc = "Event actions will be executed on pin 20"]
    #[inline(always)]
    pub fn is_pin20(&self) -> bool {
        *self == Pid0select::Pin20
    }
    #[doc = "Event actions will be executed on pin 21"]
    #[inline(always)]
    pub fn is_pin21(&self) -> bool {
        *self == Pid0select::Pin21
    }
    #[doc = "Event actions will be executed on pin 22"]
    #[inline(always)]
    pub fn is_pin22(&self) -> bool {
        *self == Pid0select::Pin22
    }
    #[doc = "Event actions will be executed on pin 23"]
    #[inline(always)]
    pub fn is_pin23(&self) -> bool {
        *self == Pid0select::Pin23
    }
    #[doc = "Event actions will be executed on pin 24"]
    #[inline(always)]
    pub fn is_pin24(&self) -> bool {
        *self == Pid0select::Pin24
    }
    #[doc = "Event actions will be executed on pin 25"]
    #[inline(always)]
    pub fn is_pin25(&self) -> bool {
        *self == Pid0select::Pin25
    }
    #[doc = "Event actions will be executed on pin 26"]
    #[inline(always)]
    pub fn is_pin26(&self) -> bool {
        *self == Pid0select::Pin26
    }
    #[doc = "Event actions will be executed on pin 27"]
    #[inline(always)]
    pub fn is_pin27(&self) -> bool {
        *self == Pid0select::Pin27
    }
    #[doc = "Event actions will be executed on pin 28"]
    #[inline(always)]
    pub fn is_pin28(&self) -> bool {
        *self == Pid0select::Pin28
    }
    #[doc = "Event actions will be executed on pin 29"]
    #[inline(always)]
    pub fn is_pin29(&self) -> bool {
        *self == Pid0select::Pin29
    }
    #[doc = "Event actions will be executed on pin 30"]
    #[inline(always)]
    pub fn is_pin30(&self) -> bool {
        *self == Pid0select::Pin30
    }
    #[doc = "Event actions will be executed on pin 31"]
    #[inline(always)]
    pub fn is_pin31(&self) -> bool {
        *self == Pid0select::Pin31
    }
}
#[doc = "Field `PID0` writer - Event 0 Pin Identifier"]
pub type Pid0W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pid0select, crate::Safe>;
impl<'a, REG> Pid0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event actions will be executed on pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin0)
    }
    #[doc = "Event actions will be executed on pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin1)
    }
    #[doc = "Event actions will be executed on pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin2)
    }
    #[doc = "Event actions will be executed on pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin3)
    }
    #[doc = "Event actions will be executed on pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin4)
    }
    #[doc = "Event actions will be executed on pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin5)
    }
    #[doc = "Event actions will be executed on pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin6)
    }
    #[doc = "Event actions will be executed on pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin7)
    }
    #[doc = "Event actions will be executed on pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin8)
    }
    #[doc = "Event actions will be executed on pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin9)
    }
    #[doc = "Event actions will be executed on pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin10)
    }
    #[doc = "Event actions will be executed on pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin11)
    }
    #[doc = "Event actions will be executed on pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin12)
    }
    #[doc = "Event actions will be executed on pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin13)
    }
    #[doc = "Event actions will be executed on pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin14)
    }
    #[doc = "Event actions will be executed on pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin15)
    }
    #[doc = "Event actions will be executed on pin 16"]
    #[inline(always)]
    pub fn pin16(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin16)
    }
    #[doc = "Event actions will be executed on pin 17"]
    #[inline(always)]
    pub fn pin17(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin17)
    }
    #[doc = "Event actions will be executed on pin 18"]
    #[inline(always)]
    pub fn pin18(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin18)
    }
    #[doc = "Event actions will be executed on pin 19"]
    #[inline(always)]
    pub fn pin19(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin19)
    }
    #[doc = "Event actions will be executed on pin 20"]
    #[inline(always)]
    pub fn pin20(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin20)
    }
    #[doc = "Event actions will be executed on pin 21"]
    #[inline(always)]
    pub fn pin21(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin21)
    }
    #[doc = "Event actions will be executed on pin 22"]
    #[inline(always)]
    pub fn pin22(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin22)
    }
    #[doc = "Event actions will be executed on pin 23"]
    #[inline(always)]
    pub fn pin23(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin23)
    }
    #[doc = "Event actions will be executed on pin 24"]
    #[inline(always)]
    pub fn pin24(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin24)
    }
    #[doc = "Event actions will be executed on pin 25"]
    #[inline(always)]
    pub fn pin25(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin25)
    }
    #[doc = "Event actions will be executed on pin 26"]
    #[inline(always)]
    pub fn pin26(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin26)
    }
    #[doc = "Event actions will be executed on pin 27"]
    #[inline(always)]
    pub fn pin27(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin27)
    }
    #[doc = "Event actions will be executed on pin 28"]
    #[inline(always)]
    pub fn pin28(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin28)
    }
    #[doc = "Event actions will be executed on pin 29"]
    #[inline(always)]
    pub fn pin29(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin29)
    }
    #[doc = "Event actions will be executed on pin 30"]
    #[inline(always)]
    pub fn pin30(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin30)
    }
    #[doc = "Event actions will be executed on pin 31"]
    #[inline(always)]
    pub fn pin31(self) -> &'a mut crate::W<REG> {
        self.variant(Pid0select::Pin31)
    }
}
#[doc = "Event 0 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evact0select {
    #[doc = "0: The output value of pin PIDn will be set to the level of the event"]
    Out = 0,
    #[doc = "1: Set the output value of pin PIDn on the rising edge of the event"]
    Set = 1,
    #[doc = "2: Clear the output value of pin PIDn on the rising edge of the event"]
    Clr = 2,
    #[doc = "3: Toggle the output value of pin PIDn on the rising edge of the event"]
    Tgl = 3,
}
impl From<Evact0select> for u8 {
    #[inline(always)]
    fn from(variant: Evact0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evact0select {
    type Ux = u8;
}
impl crate::IsEnum for Evact0select {}
#[doc = "Field `EVACT0` reader - Event 0 Action"]
pub type Evact0R = crate::FieldReader<Evact0select>;
impl Evact0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evact0select {
        match self.bits {
            0 => Evact0select::Out,
            1 => Evact0select::Set,
            2 => Evact0select::Clr,
            3 => Evact0select::Tgl,
            _ => unreachable!(),
        }
    }
    #[doc = "The output value of pin PIDn will be set to the level of the event"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Evact0select::Out
    }
    #[doc = "Set the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Evact0select::Set
    }
    #[doc = "Clear the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Evact0select::Clr
    }
    #[doc = "Toggle the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_tgl(&self) -> bool {
        *self == Evact0select::Tgl
    }
}
#[doc = "Field `EVACT0` writer - Event 0 Action"]
pub type Evact0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Evact0select, crate::Safe>;
impl<'a, REG> Evact0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The output value of pin PIDn will be set to the level of the event"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Out)
    }
    #[doc = "Set the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Set)
    }
    #[doc = "Clear the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Clr)
    }
    #[doc = "Toggle the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn tgl(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Tgl)
    }
}
#[doc = "Field `PORTEI0` reader - PORT Event 0 Input Enable"]
pub type Portei0R = crate::BitReader;
#[doc = "Field `PORTEI0` writer - PORT Event 0 Input Enable"]
pub type Portei0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Event 1 Pin Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pid1select {
    #[doc = "0: Event actions will be executed on pin 0"]
    Pin0 = 0,
    #[doc = "1: Event actions will be executed on pin 1"]
    Pin1 = 1,
    #[doc = "2: Event actions will be executed on pin 2"]
    Pin2 = 2,
    #[doc = "3: Event actions will be executed on pin 3"]
    Pin3 = 3,
    #[doc = "4: Event actions will be executed on pin 4"]
    Pin4 = 4,
    #[doc = "5: Event actions will be executed on pin 5"]
    Pin5 = 5,
    #[doc = "6: Event actions will be executed on pin 6"]
    Pin6 = 6,
    #[doc = "7: Event actions will be executed on pin 7"]
    Pin7 = 7,
    #[doc = "8: Event actions will be executed on pin 8"]
    Pin8 = 8,
    #[doc = "9: Event actions will be executed on pin 9"]
    Pin9 = 9,
    #[doc = "10: Event actions will be executed on pin 10"]
    Pin10 = 10,
    #[doc = "11: Event actions will be executed on pin 11"]
    Pin11 = 11,
    #[doc = "12: Event actions will be executed on pin 12"]
    Pin12 = 12,
    #[doc = "13: Event actions will be executed on pin 13"]
    Pin13 = 13,
    #[doc = "14: Event actions will be executed on pin 14"]
    Pin14 = 14,
    #[doc = "15: Event actions will be executed on pin 15"]
    Pin15 = 15,
    #[doc = "16: Event actions will be executed on pin 16"]
    Pin16 = 16,
    #[doc = "17: Event actions will be executed on pin 17"]
    Pin17 = 17,
    #[doc = "18: Event actions will be executed on pin 18"]
    Pin18 = 18,
    #[doc = "19: Event actions will be executed on pin 19"]
    Pin19 = 19,
    #[doc = "20: Event actions will be executed on pin 20"]
    Pin20 = 20,
    #[doc = "21: Event actions will be executed on pin 21"]
    Pin21 = 21,
    #[doc = "22: Event actions will be executed on pin 22"]
    Pin22 = 22,
    #[doc = "23: Event actions will be executed on pin 23"]
    Pin23 = 23,
    #[doc = "24: Event actions will be executed on pin 24"]
    Pin24 = 24,
    #[doc = "25: Event actions will be executed on pin 25"]
    Pin25 = 25,
    #[doc = "26: Event actions will be executed on pin 26"]
    Pin26 = 26,
    #[doc = "27: Event actions will be executed on pin 27"]
    Pin27 = 27,
    #[doc = "28: Event actions will be executed on pin 28"]
    Pin28 = 28,
    #[doc = "29: Event actions will be executed on pin 29"]
    Pin29 = 29,
    #[doc = "30: Event actions will be executed on pin 30"]
    Pin30 = 30,
    #[doc = "31: Event actions will be executed on pin 31"]
    Pin31 = 31,
}
impl From<Pid1select> for u8 {
    #[inline(always)]
    fn from(variant: Pid1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pid1select {
    type Ux = u8;
}
impl crate::IsEnum for Pid1select {}
#[doc = "Field `PID1` reader - Event 1 Pin Identifier"]
pub type Pid1R = crate::FieldReader<Pid1select>;
impl Pid1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pid1select {
        match self.bits {
            0 => Pid1select::Pin0,
            1 => Pid1select::Pin1,
            2 => Pid1select::Pin2,
            3 => Pid1select::Pin3,
            4 => Pid1select::Pin4,
            5 => Pid1select::Pin5,
            6 => Pid1select::Pin6,
            7 => Pid1select::Pin7,
            8 => Pid1select::Pin8,
            9 => Pid1select::Pin9,
            10 => Pid1select::Pin10,
            11 => Pid1select::Pin11,
            12 => Pid1select::Pin12,
            13 => Pid1select::Pin13,
            14 => Pid1select::Pin14,
            15 => Pid1select::Pin15,
            16 => Pid1select::Pin16,
            17 => Pid1select::Pin17,
            18 => Pid1select::Pin18,
            19 => Pid1select::Pin19,
            20 => Pid1select::Pin20,
            21 => Pid1select::Pin21,
            22 => Pid1select::Pin22,
            23 => Pid1select::Pin23,
            24 => Pid1select::Pin24,
            25 => Pid1select::Pin25,
            26 => Pid1select::Pin26,
            27 => Pid1select::Pin27,
            28 => Pid1select::Pin28,
            29 => Pid1select::Pin29,
            30 => Pid1select::Pin30,
            31 => Pid1select::Pin31,
            _ => unreachable!(),
        }
    }
    #[doc = "Event actions will be executed on pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Pid1select::Pin0
    }
    #[doc = "Event actions will be executed on pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Pid1select::Pin1
    }
    #[doc = "Event actions will be executed on pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Pid1select::Pin2
    }
    #[doc = "Event actions will be executed on pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Pid1select::Pin3
    }
    #[doc = "Event actions will be executed on pin 4"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == Pid1select::Pin4
    }
    #[doc = "Event actions will be executed on pin 5"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == Pid1select::Pin5
    }
    #[doc = "Event actions will be executed on pin 6"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == Pid1select::Pin6
    }
    #[doc = "Event actions will be executed on pin 7"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == Pid1select::Pin7
    }
    #[doc = "Event actions will be executed on pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Pid1select::Pin8
    }
    #[doc = "Event actions will be executed on pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Pid1select::Pin9
    }
    #[doc = "Event actions will be executed on pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Pid1select::Pin10
    }
    #[doc = "Event actions will be executed on pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Pid1select::Pin11
    }
    #[doc = "Event actions will be executed on pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == Pid1select::Pin12
    }
    #[doc = "Event actions will be executed on pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == Pid1select::Pin13
    }
    #[doc = "Event actions will be executed on pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == Pid1select::Pin14
    }
    #[doc = "Event actions will be executed on pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == Pid1select::Pin15
    }
    #[doc = "Event actions will be executed on pin 16"]
    #[inline(always)]
    pub fn is_pin16(&self) -> bool {
        *self == Pid1select::Pin16
    }
    #[doc = "Event actions will be executed on pin 17"]
    #[inline(always)]
    pub fn is_pin17(&self) -> bool {
        *self == Pid1select::Pin17
    }
    #[doc = "Event actions will be executed on pin 18"]
    #[inline(always)]
    pub fn is_pin18(&self) -> bool {
        *self == Pid1select::Pin18
    }
    #[doc = "Event actions will be executed on pin 19"]
    #[inline(always)]
    pub fn is_pin19(&self) -> bool {
        *self == Pid1select::Pin19
    }
    #[doc = "Event actions will be executed on pin 20"]
    #[inline(always)]
    pub fn is_pin20(&self) -> bool {
        *self == Pid1select::Pin20
    }
    #[doc = "Event actions will be executed on pin 21"]
    #[inline(always)]
    pub fn is_pin21(&self) -> bool {
        *self == Pid1select::Pin21
    }
    #[doc = "Event actions will be executed on pin 22"]
    #[inline(always)]
    pub fn is_pin22(&self) -> bool {
        *self == Pid1select::Pin22
    }
    #[doc = "Event actions will be executed on pin 23"]
    #[inline(always)]
    pub fn is_pin23(&self) -> bool {
        *self == Pid1select::Pin23
    }
    #[doc = "Event actions will be executed on pin 24"]
    #[inline(always)]
    pub fn is_pin24(&self) -> bool {
        *self == Pid1select::Pin24
    }
    #[doc = "Event actions will be executed on pin 25"]
    #[inline(always)]
    pub fn is_pin25(&self) -> bool {
        *self == Pid1select::Pin25
    }
    #[doc = "Event actions will be executed on pin 26"]
    #[inline(always)]
    pub fn is_pin26(&self) -> bool {
        *self == Pid1select::Pin26
    }
    #[doc = "Event actions will be executed on pin 27"]
    #[inline(always)]
    pub fn is_pin27(&self) -> bool {
        *self == Pid1select::Pin27
    }
    #[doc = "Event actions will be executed on pin 28"]
    #[inline(always)]
    pub fn is_pin28(&self) -> bool {
        *self == Pid1select::Pin28
    }
    #[doc = "Event actions will be executed on pin 29"]
    #[inline(always)]
    pub fn is_pin29(&self) -> bool {
        *self == Pid1select::Pin29
    }
    #[doc = "Event actions will be executed on pin 30"]
    #[inline(always)]
    pub fn is_pin30(&self) -> bool {
        *self == Pid1select::Pin30
    }
    #[doc = "Event actions will be executed on pin 31"]
    #[inline(always)]
    pub fn is_pin31(&self) -> bool {
        *self == Pid1select::Pin31
    }
}
#[doc = "Field `PID1` writer - Event 1 Pin Identifier"]
pub type Pid1W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pid1select, crate::Safe>;
impl<'a, REG> Pid1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event actions will be executed on pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin0)
    }
    #[doc = "Event actions will be executed on pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin1)
    }
    #[doc = "Event actions will be executed on pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin2)
    }
    #[doc = "Event actions will be executed on pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin3)
    }
    #[doc = "Event actions will be executed on pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin4)
    }
    #[doc = "Event actions will be executed on pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin5)
    }
    #[doc = "Event actions will be executed on pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin6)
    }
    #[doc = "Event actions will be executed on pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin7)
    }
    #[doc = "Event actions will be executed on pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin8)
    }
    #[doc = "Event actions will be executed on pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin9)
    }
    #[doc = "Event actions will be executed on pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin10)
    }
    #[doc = "Event actions will be executed on pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin11)
    }
    #[doc = "Event actions will be executed on pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin12)
    }
    #[doc = "Event actions will be executed on pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin13)
    }
    #[doc = "Event actions will be executed on pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin14)
    }
    #[doc = "Event actions will be executed on pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin15)
    }
    #[doc = "Event actions will be executed on pin 16"]
    #[inline(always)]
    pub fn pin16(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin16)
    }
    #[doc = "Event actions will be executed on pin 17"]
    #[inline(always)]
    pub fn pin17(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin17)
    }
    #[doc = "Event actions will be executed on pin 18"]
    #[inline(always)]
    pub fn pin18(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin18)
    }
    #[doc = "Event actions will be executed on pin 19"]
    #[inline(always)]
    pub fn pin19(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin19)
    }
    #[doc = "Event actions will be executed on pin 20"]
    #[inline(always)]
    pub fn pin20(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin20)
    }
    #[doc = "Event actions will be executed on pin 21"]
    #[inline(always)]
    pub fn pin21(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin21)
    }
    #[doc = "Event actions will be executed on pin 22"]
    #[inline(always)]
    pub fn pin22(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin22)
    }
    #[doc = "Event actions will be executed on pin 23"]
    #[inline(always)]
    pub fn pin23(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin23)
    }
    #[doc = "Event actions will be executed on pin 24"]
    #[inline(always)]
    pub fn pin24(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin24)
    }
    #[doc = "Event actions will be executed on pin 25"]
    #[inline(always)]
    pub fn pin25(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin25)
    }
    #[doc = "Event actions will be executed on pin 26"]
    #[inline(always)]
    pub fn pin26(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin26)
    }
    #[doc = "Event actions will be executed on pin 27"]
    #[inline(always)]
    pub fn pin27(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin27)
    }
    #[doc = "Event actions will be executed on pin 28"]
    #[inline(always)]
    pub fn pin28(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin28)
    }
    #[doc = "Event actions will be executed on pin 29"]
    #[inline(always)]
    pub fn pin29(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin29)
    }
    #[doc = "Event actions will be executed on pin 30"]
    #[inline(always)]
    pub fn pin30(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin30)
    }
    #[doc = "Event actions will be executed on pin 31"]
    #[inline(always)]
    pub fn pin31(self) -> &'a mut crate::W<REG> {
        self.variant(Pid1select::Pin31)
    }
}
#[doc = "Event 1 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evact1select {
    #[doc = "0: The output value of pin PIDn will be set to the level of the event"]
    Out = 0,
    #[doc = "1: Set the output value of pin PIDn on the rising edge of the event"]
    Set = 1,
    #[doc = "2: Clear the output value of pin PIDn on the rising edge of the event"]
    Clr = 2,
    #[doc = "3: Toggle the output value of pin PIDn on the rising edge of the event"]
    Tgl = 3,
}
impl From<Evact1select> for u8 {
    #[inline(always)]
    fn from(variant: Evact1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evact1select {
    type Ux = u8;
}
impl crate::IsEnum for Evact1select {}
#[doc = "Field `EVACT1` reader - Event 1 Action"]
pub type Evact1R = crate::FieldReader<Evact1select>;
impl Evact1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evact1select {
        match self.bits {
            0 => Evact1select::Out,
            1 => Evact1select::Set,
            2 => Evact1select::Clr,
            3 => Evact1select::Tgl,
            _ => unreachable!(),
        }
    }
    #[doc = "The output value of pin PIDn will be set to the level of the event"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Evact1select::Out
    }
    #[doc = "Set the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Evact1select::Set
    }
    #[doc = "Clear the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Evact1select::Clr
    }
    #[doc = "Toggle the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_tgl(&self) -> bool {
        *self == Evact1select::Tgl
    }
}
#[doc = "Field `EVACT1` writer - Event 1 Action"]
pub type Evact1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Evact1select, crate::Safe>;
impl<'a, REG> Evact1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The output value of pin PIDn will be set to the level of the event"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Out)
    }
    #[doc = "Set the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Set)
    }
    #[doc = "Clear the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Clr)
    }
    #[doc = "Toggle the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn tgl(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Tgl)
    }
}
#[doc = "Field `PORTEI1` reader - PORT Event 1 Input Enable"]
pub type Portei1R = crate::BitReader;
#[doc = "Field `PORTEI1` writer - PORT Event 1 Input Enable"]
pub type Portei1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Event 2 Pin Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pid2select {
    #[doc = "0: Event actions will be executed on pin 0"]
    Pin0 = 0,
    #[doc = "1: Event actions will be executed on pin 1"]
    Pin1 = 1,
    #[doc = "2: Event actions will be executed on pin 2"]
    Pin2 = 2,
    #[doc = "3: Event actions will be executed on pin 3"]
    Pin3 = 3,
    #[doc = "4: Event actions will be executed on pin 4"]
    Pin4 = 4,
    #[doc = "5: Event actions will be executed on pin 5"]
    Pin5 = 5,
    #[doc = "6: Event actions will be executed on pin 6"]
    Pin6 = 6,
    #[doc = "7: Event actions will be executed on pin 7"]
    Pin7 = 7,
    #[doc = "8: Event actions will be executed on pin 8"]
    Pin8 = 8,
    #[doc = "9: Event actions will be executed on pin 9"]
    Pin9 = 9,
    #[doc = "10: Event actions will be executed on pin 10"]
    Pin10 = 10,
    #[doc = "11: Event actions will be executed on pin 11"]
    Pin11 = 11,
    #[doc = "12: Event actions will be executed on pin 12"]
    Pin12 = 12,
    #[doc = "13: Event actions will be executed on pin 13"]
    Pin13 = 13,
    #[doc = "14: Event actions will be executed on pin 14"]
    Pin14 = 14,
    #[doc = "15: Event actions will be executed on pin 15"]
    Pin15 = 15,
    #[doc = "16: Event actions will be executed on pin 16"]
    Pin16 = 16,
    #[doc = "17: Event actions will be executed on pin 17"]
    Pin17 = 17,
    #[doc = "18: Event actions will be executed on pin 18"]
    Pin18 = 18,
    #[doc = "19: Event actions will be executed on pin 19"]
    Pin19 = 19,
    #[doc = "20: Event actions will be executed on pin 20"]
    Pin20 = 20,
    #[doc = "21: Event actions will be executed on pin 21"]
    Pin21 = 21,
    #[doc = "22: Event actions will be executed on pin 22"]
    Pin22 = 22,
    #[doc = "23: Event actions will be executed on pin 23"]
    Pin23 = 23,
    #[doc = "24: Event actions will be executed on pin 24"]
    Pin24 = 24,
    #[doc = "25: Event actions will be executed on pin 25"]
    Pin25 = 25,
    #[doc = "26: Event actions will be executed on pin 26"]
    Pin26 = 26,
    #[doc = "27: Event actions will be executed on pin 27"]
    Pin27 = 27,
    #[doc = "28: Event actions will be executed on pin 28"]
    Pin28 = 28,
    #[doc = "29: Event actions will be executed on pin 29"]
    Pin29 = 29,
    #[doc = "30: Event actions will be executed on pin 30"]
    Pin30 = 30,
    #[doc = "31: Event actions will be executed on pin 31"]
    Pin31 = 31,
}
impl From<Pid2select> for u8 {
    #[inline(always)]
    fn from(variant: Pid2select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pid2select {
    type Ux = u8;
}
impl crate::IsEnum for Pid2select {}
#[doc = "Field `PID2` reader - Event 2 Pin Identifier"]
pub type Pid2R = crate::FieldReader<Pid2select>;
impl Pid2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pid2select {
        match self.bits {
            0 => Pid2select::Pin0,
            1 => Pid2select::Pin1,
            2 => Pid2select::Pin2,
            3 => Pid2select::Pin3,
            4 => Pid2select::Pin4,
            5 => Pid2select::Pin5,
            6 => Pid2select::Pin6,
            7 => Pid2select::Pin7,
            8 => Pid2select::Pin8,
            9 => Pid2select::Pin9,
            10 => Pid2select::Pin10,
            11 => Pid2select::Pin11,
            12 => Pid2select::Pin12,
            13 => Pid2select::Pin13,
            14 => Pid2select::Pin14,
            15 => Pid2select::Pin15,
            16 => Pid2select::Pin16,
            17 => Pid2select::Pin17,
            18 => Pid2select::Pin18,
            19 => Pid2select::Pin19,
            20 => Pid2select::Pin20,
            21 => Pid2select::Pin21,
            22 => Pid2select::Pin22,
            23 => Pid2select::Pin23,
            24 => Pid2select::Pin24,
            25 => Pid2select::Pin25,
            26 => Pid2select::Pin26,
            27 => Pid2select::Pin27,
            28 => Pid2select::Pin28,
            29 => Pid2select::Pin29,
            30 => Pid2select::Pin30,
            31 => Pid2select::Pin31,
            _ => unreachable!(),
        }
    }
    #[doc = "Event actions will be executed on pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Pid2select::Pin0
    }
    #[doc = "Event actions will be executed on pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Pid2select::Pin1
    }
    #[doc = "Event actions will be executed on pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Pid2select::Pin2
    }
    #[doc = "Event actions will be executed on pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Pid2select::Pin3
    }
    #[doc = "Event actions will be executed on pin 4"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == Pid2select::Pin4
    }
    #[doc = "Event actions will be executed on pin 5"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == Pid2select::Pin5
    }
    #[doc = "Event actions will be executed on pin 6"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == Pid2select::Pin6
    }
    #[doc = "Event actions will be executed on pin 7"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == Pid2select::Pin7
    }
    #[doc = "Event actions will be executed on pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Pid2select::Pin8
    }
    #[doc = "Event actions will be executed on pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Pid2select::Pin9
    }
    #[doc = "Event actions will be executed on pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Pid2select::Pin10
    }
    #[doc = "Event actions will be executed on pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Pid2select::Pin11
    }
    #[doc = "Event actions will be executed on pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == Pid2select::Pin12
    }
    #[doc = "Event actions will be executed on pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == Pid2select::Pin13
    }
    #[doc = "Event actions will be executed on pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == Pid2select::Pin14
    }
    #[doc = "Event actions will be executed on pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == Pid2select::Pin15
    }
    #[doc = "Event actions will be executed on pin 16"]
    #[inline(always)]
    pub fn is_pin16(&self) -> bool {
        *self == Pid2select::Pin16
    }
    #[doc = "Event actions will be executed on pin 17"]
    #[inline(always)]
    pub fn is_pin17(&self) -> bool {
        *self == Pid2select::Pin17
    }
    #[doc = "Event actions will be executed on pin 18"]
    #[inline(always)]
    pub fn is_pin18(&self) -> bool {
        *self == Pid2select::Pin18
    }
    #[doc = "Event actions will be executed on pin 19"]
    #[inline(always)]
    pub fn is_pin19(&self) -> bool {
        *self == Pid2select::Pin19
    }
    #[doc = "Event actions will be executed on pin 20"]
    #[inline(always)]
    pub fn is_pin20(&self) -> bool {
        *self == Pid2select::Pin20
    }
    #[doc = "Event actions will be executed on pin 21"]
    #[inline(always)]
    pub fn is_pin21(&self) -> bool {
        *self == Pid2select::Pin21
    }
    #[doc = "Event actions will be executed on pin 22"]
    #[inline(always)]
    pub fn is_pin22(&self) -> bool {
        *self == Pid2select::Pin22
    }
    #[doc = "Event actions will be executed on pin 23"]
    #[inline(always)]
    pub fn is_pin23(&self) -> bool {
        *self == Pid2select::Pin23
    }
    #[doc = "Event actions will be executed on pin 24"]
    #[inline(always)]
    pub fn is_pin24(&self) -> bool {
        *self == Pid2select::Pin24
    }
    #[doc = "Event actions will be executed on pin 25"]
    #[inline(always)]
    pub fn is_pin25(&self) -> bool {
        *self == Pid2select::Pin25
    }
    #[doc = "Event actions will be executed on pin 26"]
    #[inline(always)]
    pub fn is_pin26(&self) -> bool {
        *self == Pid2select::Pin26
    }
    #[doc = "Event actions will be executed on pin 27"]
    #[inline(always)]
    pub fn is_pin27(&self) -> bool {
        *self == Pid2select::Pin27
    }
    #[doc = "Event actions will be executed on pin 28"]
    #[inline(always)]
    pub fn is_pin28(&self) -> bool {
        *self == Pid2select::Pin28
    }
    #[doc = "Event actions will be executed on pin 29"]
    #[inline(always)]
    pub fn is_pin29(&self) -> bool {
        *self == Pid2select::Pin29
    }
    #[doc = "Event actions will be executed on pin 30"]
    #[inline(always)]
    pub fn is_pin30(&self) -> bool {
        *self == Pid2select::Pin30
    }
    #[doc = "Event actions will be executed on pin 31"]
    #[inline(always)]
    pub fn is_pin31(&self) -> bool {
        *self == Pid2select::Pin31
    }
}
#[doc = "Field `PID2` writer - Event 2 Pin Identifier"]
pub type Pid2W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pid2select, crate::Safe>;
impl<'a, REG> Pid2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event actions will be executed on pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin0)
    }
    #[doc = "Event actions will be executed on pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin1)
    }
    #[doc = "Event actions will be executed on pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin2)
    }
    #[doc = "Event actions will be executed on pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin3)
    }
    #[doc = "Event actions will be executed on pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin4)
    }
    #[doc = "Event actions will be executed on pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin5)
    }
    #[doc = "Event actions will be executed on pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin6)
    }
    #[doc = "Event actions will be executed on pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin7)
    }
    #[doc = "Event actions will be executed on pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin8)
    }
    #[doc = "Event actions will be executed on pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin9)
    }
    #[doc = "Event actions will be executed on pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin10)
    }
    #[doc = "Event actions will be executed on pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin11)
    }
    #[doc = "Event actions will be executed on pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin12)
    }
    #[doc = "Event actions will be executed on pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin13)
    }
    #[doc = "Event actions will be executed on pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin14)
    }
    #[doc = "Event actions will be executed on pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin15)
    }
    #[doc = "Event actions will be executed on pin 16"]
    #[inline(always)]
    pub fn pin16(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin16)
    }
    #[doc = "Event actions will be executed on pin 17"]
    #[inline(always)]
    pub fn pin17(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin17)
    }
    #[doc = "Event actions will be executed on pin 18"]
    #[inline(always)]
    pub fn pin18(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin18)
    }
    #[doc = "Event actions will be executed on pin 19"]
    #[inline(always)]
    pub fn pin19(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin19)
    }
    #[doc = "Event actions will be executed on pin 20"]
    #[inline(always)]
    pub fn pin20(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin20)
    }
    #[doc = "Event actions will be executed on pin 21"]
    #[inline(always)]
    pub fn pin21(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin21)
    }
    #[doc = "Event actions will be executed on pin 22"]
    #[inline(always)]
    pub fn pin22(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin22)
    }
    #[doc = "Event actions will be executed on pin 23"]
    #[inline(always)]
    pub fn pin23(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin23)
    }
    #[doc = "Event actions will be executed on pin 24"]
    #[inline(always)]
    pub fn pin24(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin24)
    }
    #[doc = "Event actions will be executed on pin 25"]
    #[inline(always)]
    pub fn pin25(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin25)
    }
    #[doc = "Event actions will be executed on pin 26"]
    #[inline(always)]
    pub fn pin26(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin26)
    }
    #[doc = "Event actions will be executed on pin 27"]
    #[inline(always)]
    pub fn pin27(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin27)
    }
    #[doc = "Event actions will be executed on pin 28"]
    #[inline(always)]
    pub fn pin28(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin28)
    }
    #[doc = "Event actions will be executed on pin 29"]
    #[inline(always)]
    pub fn pin29(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin29)
    }
    #[doc = "Event actions will be executed on pin 30"]
    #[inline(always)]
    pub fn pin30(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin30)
    }
    #[doc = "Event actions will be executed on pin 31"]
    #[inline(always)]
    pub fn pin31(self) -> &'a mut crate::W<REG> {
        self.variant(Pid2select::Pin31)
    }
}
#[doc = "Event 2 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evact2select {
    #[doc = "0: The output value of pin PIDn will be set to the level of the event"]
    Out = 0,
    #[doc = "1: Set the output value of pin PIDn on the rising edge of the event"]
    Set = 1,
    #[doc = "2: Clear the output value of pin PIDn on the rising edge of the event"]
    Clr = 2,
    #[doc = "3: Toggle the output value of pin PIDn on the rising edge of the event"]
    Tgl = 3,
}
impl From<Evact2select> for u8 {
    #[inline(always)]
    fn from(variant: Evact2select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evact2select {
    type Ux = u8;
}
impl crate::IsEnum for Evact2select {}
#[doc = "Field `EVACT2` reader - Event 2 Action"]
pub type Evact2R = crate::FieldReader<Evact2select>;
impl Evact2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evact2select {
        match self.bits {
            0 => Evact2select::Out,
            1 => Evact2select::Set,
            2 => Evact2select::Clr,
            3 => Evact2select::Tgl,
            _ => unreachable!(),
        }
    }
    #[doc = "The output value of pin PIDn will be set to the level of the event"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Evact2select::Out
    }
    #[doc = "Set the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Evact2select::Set
    }
    #[doc = "Clear the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Evact2select::Clr
    }
    #[doc = "Toggle the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_tgl(&self) -> bool {
        *self == Evact2select::Tgl
    }
}
#[doc = "Field `EVACT2` writer - Event 2 Action"]
pub type Evact2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Evact2select, crate::Safe>;
impl<'a, REG> Evact2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The output value of pin PIDn will be set to the level of the event"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Evact2select::Out)
    }
    #[doc = "Set the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Evact2select::Set)
    }
    #[doc = "Clear the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Evact2select::Clr)
    }
    #[doc = "Toggle the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn tgl(self) -> &'a mut crate::W<REG> {
        self.variant(Evact2select::Tgl)
    }
}
#[doc = "Field `PORTEI2` reader - PORT Event 2 Input Enable"]
pub type Portei2R = crate::BitReader;
#[doc = "Field `PORTEI2` writer - PORT Event 2 Input Enable"]
pub type Portei2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Event 3 Pin Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pid3select {
    #[doc = "0: Event actions will be executed on pin 0"]
    Pin0 = 0,
    #[doc = "1: Event actions will be executed on pin 1"]
    Pin1 = 1,
    #[doc = "2: Event actions will be executed on pin 2"]
    Pin2 = 2,
    #[doc = "3: Event actions will be executed on pin 3"]
    Pin3 = 3,
    #[doc = "4: Event actions will be executed on pin 4"]
    Pin4 = 4,
    #[doc = "5: Event actions will be executed on pin 5"]
    Pin5 = 5,
    #[doc = "6: Event actions will be executed on pin 6"]
    Pin6 = 6,
    #[doc = "7: Event actions will be executed on pin 7"]
    Pin7 = 7,
    #[doc = "8: Event actions will be executed on pin 8"]
    Pin8 = 8,
    #[doc = "9: Event actions will be executed on pin 9"]
    Pin9 = 9,
    #[doc = "10: Event actions will be executed on pin 10"]
    Pin10 = 10,
    #[doc = "11: Event actions will be executed on pin 11"]
    Pin11 = 11,
    #[doc = "12: Event actions will be executed on pin 12"]
    Pin12 = 12,
    #[doc = "13: Event actions will be executed on pin 13"]
    Pin13 = 13,
    #[doc = "14: Event actions will be executed on pin 14"]
    Pin14 = 14,
    #[doc = "15: Event actions will be executed on pin 15"]
    Pin15 = 15,
    #[doc = "16: Event actions will be executed on pin 16"]
    Pin16 = 16,
    #[doc = "17: Event actions will be executed on pin 17"]
    Pin17 = 17,
    #[doc = "18: Event actions will be executed on pin 18"]
    Pin18 = 18,
    #[doc = "19: Event actions will be executed on pin 19"]
    Pin19 = 19,
    #[doc = "20: Event actions will be executed on pin 20"]
    Pin20 = 20,
    #[doc = "21: Event actions will be executed on pin 21"]
    Pin21 = 21,
    #[doc = "22: Event actions will be executed on pin 22"]
    Pin22 = 22,
    #[doc = "23: Event actions will be executed on pin 23"]
    Pin23 = 23,
    #[doc = "24: Event actions will be executed on pin 24"]
    Pin24 = 24,
    #[doc = "25: Event actions will be executed on pin 25"]
    Pin25 = 25,
    #[doc = "26: Event actions will be executed on pin 26"]
    Pin26 = 26,
    #[doc = "27: Event actions will be executed on pin 27"]
    Pin27 = 27,
    #[doc = "28: Event actions will be executed on pin 28"]
    Pin28 = 28,
    #[doc = "29: Event actions will be executed on pin 29"]
    Pin29 = 29,
    #[doc = "30: Event actions will be executed on pin 30"]
    Pin30 = 30,
    #[doc = "31: Event actions will be executed on pin 31"]
    Pin31 = 31,
}
impl From<Pid3select> for u8 {
    #[inline(always)]
    fn from(variant: Pid3select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pid3select {
    type Ux = u8;
}
impl crate::IsEnum for Pid3select {}
#[doc = "Field `PID3` reader - Event 3 Pin Identifier"]
pub type Pid3R = crate::FieldReader<Pid3select>;
impl Pid3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pid3select {
        match self.bits {
            0 => Pid3select::Pin0,
            1 => Pid3select::Pin1,
            2 => Pid3select::Pin2,
            3 => Pid3select::Pin3,
            4 => Pid3select::Pin4,
            5 => Pid3select::Pin5,
            6 => Pid3select::Pin6,
            7 => Pid3select::Pin7,
            8 => Pid3select::Pin8,
            9 => Pid3select::Pin9,
            10 => Pid3select::Pin10,
            11 => Pid3select::Pin11,
            12 => Pid3select::Pin12,
            13 => Pid3select::Pin13,
            14 => Pid3select::Pin14,
            15 => Pid3select::Pin15,
            16 => Pid3select::Pin16,
            17 => Pid3select::Pin17,
            18 => Pid3select::Pin18,
            19 => Pid3select::Pin19,
            20 => Pid3select::Pin20,
            21 => Pid3select::Pin21,
            22 => Pid3select::Pin22,
            23 => Pid3select::Pin23,
            24 => Pid3select::Pin24,
            25 => Pid3select::Pin25,
            26 => Pid3select::Pin26,
            27 => Pid3select::Pin27,
            28 => Pid3select::Pin28,
            29 => Pid3select::Pin29,
            30 => Pid3select::Pin30,
            31 => Pid3select::Pin31,
            _ => unreachable!(),
        }
    }
    #[doc = "Event actions will be executed on pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Pid3select::Pin0
    }
    #[doc = "Event actions will be executed on pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Pid3select::Pin1
    }
    #[doc = "Event actions will be executed on pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Pid3select::Pin2
    }
    #[doc = "Event actions will be executed on pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Pid3select::Pin3
    }
    #[doc = "Event actions will be executed on pin 4"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == Pid3select::Pin4
    }
    #[doc = "Event actions will be executed on pin 5"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == Pid3select::Pin5
    }
    #[doc = "Event actions will be executed on pin 6"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == Pid3select::Pin6
    }
    #[doc = "Event actions will be executed on pin 7"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == Pid3select::Pin7
    }
    #[doc = "Event actions will be executed on pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Pid3select::Pin8
    }
    #[doc = "Event actions will be executed on pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Pid3select::Pin9
    }
    #[doc = "Event actions will be executed on pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Pid3select::Pin10
    }
    #[doc = "Event actions will be executed on pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Pid3select::Pin11
    }
    #[doc = "Event actions will be executed on pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == Pid3select::Pin12
    }
    #[doc = "Event actions will be executed on pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == Pid3select::Pin13
    }
    #[doc = "Event actions will be executed on pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == Pid3select::Pin14
    }
    #[doc = "Event actions will be executed on pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == Pid3select::Pin15
    }
    #[doc = "Event actions will be executed on pin 16"]
    #[inline(always)]
    pub fn is_pin16(&self) -> bool {
        *self == Pid3select::Pin16
    }
    #[doc = "Event actions will be executed on pin 17"]
    #[inline(always)]
    pub fn is_pin17(&self) -> bool {
        *self == Pid3select::Pin17
    }
    #[doc = "Event actions will be executed on pin 18"]
    #[inline(always)]
    pub fn is_pin18(&self) -> bool {
        *self == Pid3select::Pin18
    }
    #[doc = "Event actions will be executed on pin 19"]
    #[inline(always)]
    pub fn is_pin19(&self) -> bool {
        *self == Pid3select::Pin19
    }
    #[doc = "Event actions will be executed on pin 20"]
    #[inline(always)]
    pub fn is_pin20(&self) -> bool {
        *self == Pid3select::Pin20
    }
    #[doc = "Event actions will be executed on pin 21"]
    #[inline(always)]
    pub fn is_pin21(&self) -> bool {
        *self == Pid3select::Pin21
    }
    #[doc = "Event actions will be executed on pin 22"]
    #[inline(always)]
    pub fn is_pin22(&self) -> bool {
        *self == Pid3select::Pin22
    }
    #[doc = "Event actions will be executed on pin 23"]
    #[inline(always)]
    pub fn is_pin23(&self) -> bool {
        *self == Pid3select::Pin23
    }
    #[doc = "Event actions will be executed on pin 24"]
    #[inline(always)]
    pub fn is_pin24(&self) -> bool {
        *self == Pid3select::Pin24
    }
    #[doc = "Event actions will be executed on pin 25"]
    #[inline(always)]
    pub fn is_pin25(&self) -> bool {
        *self == Pid3select::Pin25
    }
    #[doc = "Event actions will be executed on pin 26"]
    #[inline(always)]
    pub fn is_pin26(&self) -> bool {
        *self == Pid3select::Pin26
    }
    #[doc = "Event actions will be executed on pin 27"]
    #[inline(always)]
    pub fn is_pin27(&self) -> bool {
        *self == Pid3select::Pin27
    }
    #[doc = "Event actions will be executed on pin 28"]
    #[inline(always)]
    pub fn is_pin28(&self) -> bool {
        *self == Pid3select::Pin28
    }
    #[doc = "Event actions will be executed on pin 29"]
    #[inline(always)]
    pub fn is_pin29(&self) -> bool {
        *self == Pid3select::Pin29
    }
    #[doc = "Event actions will be executed on pin 30"]
    #[inline(always)]
    pub fn is_pin30(&self) -> bool {
        *self == Pid3select::Pin30
    }
    #[doc = "Event actions will be executed on pin 31"]
    #[inline(always)]
    pub fn is_pin31(&self) -> bool {
        *self == Pid3select::Pin31
    }
}
#[doc = "Field `PID3` writer - Event 3 Pin Identifier"]
pub type Pid3W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pid3select, crate::Safe>;
impl<'a, REG> Pid3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event actions will be executed on pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin0)
    }
    #[doc = "Event actions will be executed on pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin1)
    }
    #[doc = "Event actions will be executed on pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin2)
    }
    #[doc = "Event actions will be executed on pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin3)
    }
    #[doc = "Event actions will be executed on pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin4)
    }
    #[doc = "Event actions will be executed on pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin5)
    }
    #[doc = "Event actions will be executed on pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin6)
    }
    #[doc = "Event actions will be executed on pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin7)
    }
    #[doc = "Event actions will be executed on pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin8)
    }
    #[doc = "Event actions will be executed on pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin9)
    }
    #[doc = "Event actions will be executed on pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin10)
    }
    #[doc = "Event actions will be executed on pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin11)
    }
    #[doc = "Event actions will be executed on pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin12)
    }
    #[doc = "Event actions will be executed on pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin13)
    }
    #[doc = "Event actions will be executed on pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin14)
    }
    #[doc = "Event actions will be executed on pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin15)
    }
    #[doc = "Event actions will be executed on pin 16"]
    #[inline(always)]
    pub fn pin16(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin16)
    }
    #[doc = "Event actions will be executed on pin 17"]
    #[inline(always)]
    pub fn pin17(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin17)
    }
    #[doc = "Event actions will be executed on pin 18"]
    #[inline(always)]
    pub fn pin18(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin18)
    }
    #[doc = "Event actions will be executed on pin 19"]
    #[inline(always)]
    pub fn pin19(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin19)
    }
    #[doc = "Event actions will be executed on pin 20"]
    #[inline(always)]
    pub fn pin20(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin20)
    }
    #[doc = "Event actions will be executed on pin 21"]
    #[inline(always)]
    pub fn pin21(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin21)
    }
    #[doc = "Event actions will be executed on pin 22"]
    #[inline(always)]
    pub fn pin22(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin22)
    }
    #[doc = "Event actions will be executed on pin 23"]
    #[inline(always)]
    pub fn pin23(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin23)
    }
    #[doc = "Event actions will be executed on pin 24"]
    #[inline(always)]
    pub fn pin24(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin24)
    }
    #[doc = "Event actions will be executed on pin 25"]
    #[inline(always)]
    pub fn pin25(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin25)
    }
    #[doc = "Event actions will be executed on pin 26"]
    #[inline(always)]
    pub fn pin26(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin26)
    }
    #[doc = "Event actions will be executed on pin 27"]
    #[inline(always)]
    pub fn pin27(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin27)
    }
    #[doc = "Event actions will be executed on pin 28"]
    #[inline(always)]
    pub fn pin28(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin28)
    }
    #[doc = "Event actions will be executed on pin 29"]
    #[inline(always)]
    pub fn pin29(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin29)
    }
    #[doc = "Event actions will be executed on pin 30"]
    #[inline(always)]
    pub fn pin30(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin30)
    }
    #[doc = "Event actions will be executed on pin 31"]
    #[inline(always)]
    pub fn pin31(self) -> &'a mut crate::W<REG> {
        self.variant(Pid3select::Pin31)
    }
}
#[doc = "Event 3 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evact3select {
    #[doc = "0: The output value of pin PIDn will be set to the level of the event"]
    Out = 0,
    #[doc = "1: Set the output value of pin PIDn on the rising edge of the event"]
    Set = 1,
    #[doc = "2: Clear the output value of pin PIDn on the rising edge of the event"]
    Clr = 2,
    #[doc = "3: Toggle the output value of pin PIDn on the rising edge of the event"]
    Tgl = 3,
}
impl From<Evact3select> for u8 {
    #[inline(always)]
    fn from(variant: Evact3select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evact3select {
    type Ux = u8;
}
impl crate::IsEnum for Evact3select {}
#[doc = "Field `EVACT3` reader - Event 3 Action"]
pub type Evact3R = crate::FieldReader<Evact3select>;
impl Evact3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evact3select {
        match self.bits {
            0 => Evact3select::Out,
            1 => Evact3select::Set,
            2 => Evact3select::Clr,
            3 => Evact3select::Tgl,
            _ => unreachable!(),
        }
    }
    #[doc = "The output value of pin PIDn will be set to the level of the event"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Evact3select::Out
    }
    #[doc = "Set the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Evact3select::Set
    }
    #[doc = "Clear the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Evact3select::Clr
    }
    #[doc = "Toggle the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn is_tgl(&self) -> bool {
        *self == Evact3select::Tgl
    }
}
#[doc = "Field `EVACT3` writer - Event 3 Action"]
pub type Evact3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Evact3select, crate::Safe>;
impl<'a, REG> Evact3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The output value of pin PIDn will be set to the level of the event"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Evact3select::Out)
    }
    #[doc = "Set the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Evact3select::Set)
    }
    #[doc = "Clear the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Evact3select::Clr)
    }
    #[doc = "Toggle the output value of pin PIDn on the rising edge of the event"]
    #[inline(always)]
    pub fn tgl(self) -> &'a mut crate::W<REG> {
        self.variant(Evact3select::Tgl)
    }
}
#[doc = "Field `PORTEI3` reader - PORT Event 3 Input Enable"]
pub type Portei3R = crate::BitReader;
#[doc = "Field `PORTEI3` writer - PORT Event 3 Input Enable"]
pub type Portei3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Event 0 Pin Identifier"]
    #[inline(always)]
    pub fn pid0(&self) -> Pid0R {
        Pid0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Event 0 Action"]
    #[inline(always)]
    pub fn evact0(&self) -> Evact0R {
        Evact0R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - PORT Event 0 Input Enable"]
    #[inline(always)]
    pub fn portei0(&self) -> Portei0R {
        Portei0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Event 1 Pin Identifier"]
    #[inline(always)]
    pub fn pid1(&self) -> Pid1R {
        Pid1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Event 1 Action"]
    #[inline(always)]
    pub fn evact1(&self) -> Evact1R {
        Evact1R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - PORT Event 1 Input Enable"]
    #[inline(always)]
    pub fn portei1(&self) -> Portei1R {
        Portei1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Event 2 Pin Identifier"]
    #[inline(always)]
    pub fn pid2(&self) -> Pid2R {
        Pid2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Event 2 Action"]
    #[inline(always)]
    pub fn evact2(&self) -> Evact2R {
        Evact2R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - PORT Event 2 Input Enable"]
    #[inline(always)]
    pub fn portei2(&self) -> Portei2R {
        Portei2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Event 3 Pin Identifier"]
    #[inline(always)]
    pub fn pid3(&self) -> Pid3R {
        Pid3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - Event 3 Action"]
    #[inline(always)]
    pub fn evact3(&self) -> Evact3R {
        Evact3R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - PORT Event 3 Input Enable"]
    #[inline(always)]
    pub fn portei3(&self) -> Portei3R {
        Portei3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Event 0 Pin Identifier"]
    #[inline(always)]
    pub fn pid0(&mut self) -> Pid0W<'_, EvctrlSpec> {
        Pid0W::new(self, 0)
    }
    #[doc = "Bits 5:6 - Event 0 Action"]
    #[inline(always)]
    pub fn evact0(&mut self) -> Evact0W<'_, EvctrlSpec> {
        Evact0W::new(self, 5)
    }
    #[doc = "Bit 7 - PORT Event 0 Input Enable"]
    #[inline(always)]
    pub fn portei0(&mut self) -> Portei0W<'_, EvctrlSpec> {
        Portei0W::new(self, 7)
    }
    #[doc = "Bits 8:12 - Event 1 Pin Identifier"]
    #[inline(always)]
    pub fn pid1(&mut self) -> Pid1W<'_, EvctrlSpec> {
        Pid1W::new(self, 8)
    }
    #[doc = "Bits 13:14 - Event 1 Action"]
    #[inline(always)]
    pub fn evact1(&mut self) -> Evact1W<'_, EvctrlSpec> {
        Evact1W::new(self, 13)
    }
    #[doc = "Bit 15 - PORT Event 1 Input Enable"]
    #[inline(always)]
    pub fn portei1(&mut self) -> Portei1W<'_, EvctrlSpec> {
        Portei1W::new(self, 15)
    }
    #[doc = "Bits 16:20 - Event 2 Pin Identifier"]
    #[inline(always)]
    pub fn pid2(&mut self) -> Pid2W<'_, EvctrlSpec> {
        Pid2W::new(self, 16)
    }
    #[doc = "Bits 21:22 - Event 2 Action"]
    #[inline(always)]
    pub fn evact2(&mut self) -> Evact2W<'_, EvctrlSpec> {
        Evact2W::new(self, 21)
    }
    #[doc = "Bit 23 - PORT Event 2 Input Enable"]
    #[inline(always)]
    pub fn portei2(&mut self) -> Portei2W<'_, EvctrlSpec> {
        Portei2W::new(self, 23)
    }
    #[doc = "Bits 24:28 - Event 3 Pin Identifier"]
    #[inline(always)]
    pub fn pid3(&mut self) -> Pid3W<'_, EvctrlSpec> {
        Pid3W::new(self, 24)
    }
    #[doc = "Bits 29:30 - Event 3 Action"]
    #[inline(always)]
    pub fn evact3(&mut self) -> Evact3W<'_, EvctrlSpec> {
        Evact3W::new(self, 29)
    }
    #[doc = "Bit 31 - PORT Event 3 Input Enable"]
    #[inline(always)]
    pub fn portei3(&mut self) -> Portei3W<'_, EvctrlSpec> {
        Portei3W::new(self, 31)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {}

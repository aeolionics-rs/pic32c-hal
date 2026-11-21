#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `GTIME` reader - Guard Time"]
pub type GtimeR = crate::FieldReader;
#[doc = "Field `GTIME` writer - Guard Time"]
pub type GtimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "LIN Host Break Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Brklenselect {
    #[doc = "0: The break field transmission is 13 bit times"]
    _13BitTime = 0,
    #[doc = "1: The break field transmission is 17 bit times"]
    _17BitTime = 1,
    #[doc = "2: The break field transmission is 21 bit times"]
    _21BitTime = 2,
    #[doc = "3: The break field transmission is 26 bit times"]
    _26BitTime = 3,
}
impl From<Brklenselect> for u8 {
    #[inline(always)]
    fn from(variant: Brklenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Brklenselect {
    type Ux = u8;
}
impl crate::IsEnum for Brklenselect {}
#[doc = "Field `BRKLEN` reader - LIN Host Break Length"]
pub type BrklenR = crate::FieldReader<Brklenselect>;
impl BrklenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brklenselect {
        match self.bits {
            0 => Brklenselect::_13BitTime,
            1 => Brklenselect::_17BitTime,
            2 => Brklenselect::_21BitTime,
            3 => Brklenselect::_26BitTime,
            _ => unreachable!(),
        }
    }
    #[doc = "The break field transmission is 13 bit times"]
    #[inline(always)]
    pub fn is_13_bit_time(&self) -> bool {
        *self == Brklenselect::_13BitTime
    }
    #[doc = "The break field transmission is 17 bit times"]
    #[inline(always)]
    pub fn is_17_bit_time(&self) -> bool {
        *self == Brklenselect::_17BitTime
    }
    #[doc = "The break field transmission is 21 bit times"]
    #[inline(always)]
    pub fn is_21_bit_time(&self) -> bool {
        *self == Brklenselect::_21BitTime
    }
    #[doc = "The break field transmission is 26 bit times"]
    #[inline(always)]
    pub fn is_26_bit_time(&self) -> bool {
        *self == Brklenselect::_26BitTime
    }
}
#[doc = "Field `BRKLEN` writer - LIN Host Break Length"]
pub type BrklenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Brklenselect, crate::Safe>;
impl<'a, REG> BrklenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The break field transmission is 13 bit times"]
    #[inline(always)]
    pub fn _13_bit_time(self) -> &'a mut crate::W<REG> {
        self.variant(Brklenselect::_13BitTime)
    }
    #[doc = "The break field transmission is 17 bit times"]
    #[inline(always)]
    pub fn _17_bit_time(self) -> &'a mut crate::W<REG> {
        self.variant(Brklenselect::_17BitTime)
    }
    #[doc = "The break field transmission is 21 bit times"]
    #[inline(always)]
    pub fn _21_bit_time(self) -> &'a mut crate::W<REG> {
        self.variant(Brklenselect::_21BitTime)
    }
    #[doc = "The break field transmission is 26 bit times"]
    #[inline(always)]
    pub fn _26_bit_time(self) -> &'a mut crate::W<REG> {
        self.variant(Brklenselect::_26BitTime)
    }
}
#[doc = "LIN Host Header Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hdrdlyselect {
    #[doc = "0: Delay between break and sync transmission is one bit time. Delay between sync and ID transmission is one bit time."]
    _1BitTime = 0,
    #[doc = "1: Delay between break and sync transmission is four bit times. Delay between sync and ID transmission is four bit times."]
    _4BitTime = 1,
    #[doc = "2: Delay between break and sync transmission is eight bit times. Delay between sync and ID transmission is four bit times."]
    _8BitTime = 2,
    #[doc = "3: Delay between break and sync transmission is 14 bit times. Delay between sync and ID transmission is four bit times."]
    _14BitTime = 3,
}
impl From<Hdrdlyselect> for u8 {
    #[inline(always)]
    fn from(variant: Hdrdlyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hdrdlyselect {
    type Ux = u8;
}
impl crate::IsEnum for Hdrdlyselect {}
#[doc = "Field `HDRDLY` reader - LIN Host Header Delay"]
pub type HdrdlyR = crate::FieldReader<Hdrdlyselect>;
impl HdrdlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdrdlyselect {
        match self.bits {
            0 => Hdrdlyselect::_1BitTime,
            1 => Hdrdlyselect::_4BitTime,
            2 => Hdrdlyselect::_8BitTime,
            3 => Hdrdlyselect::_14BitTime,
            _ => unreachable!(),
        }
    }
    #[doc = "Delay between break and sync transmission is one bit time. Delay between sync and ID transmission is one bit time."]
    #[inline(always)]
    pub fn is_1_bit_time(&self) -> bool {
        *self == Hdrdlyselect::_1BitTime
    }
    #[doc = "Delay between break and sync transmission is four bit times. Delay between sync and ID transmission is four bit times."]
    #[inline(always)]
    pub fn is_4_bit_time(&self) -> bool {
        *self == Hdrdlyselect::_4BitTime
    }
    #[doc = "Delay between break and sync transmission is eight bit times. Delay between sync and ID transmission is four bit times."]
    #[inline(always)]
    pub fn is_8_bit_time(&self) -> bool {
        *self == Hdrdlyselect::_8BitTime
    }
    #[doc = "Delay between break and sync transmission is 14 bit times. Delay between sync and ID transmission is four bit times."]
    #[inline(always)]
    pub fn is_14_bit_time(&self) -> bool {
        *self == Hdrdlyselect::_14BitTime
    }
}
#[doc = "Field `HDRDLY` writer - LIN Host Header Delay"]
pub type HdrdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hdrdlyselect, crate::Safe>;
impl<'a, REG> HdrdlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Delay between break and sync transmission is one bit time. Delay between sync and ID transmission is one bit time."]
    #[inline(always)]
    pub fn _1_bit_time(self) -> &'a mut crate::W<REG> {
        self.variant(Hdrdlyselect::_1BitTime)
    }
    #[doc = "Delay between break and sync transmission is four bit times. Delay between sync and ID transmission is four bit times."]
    #[inline(always)]
    pub fn _4_bit_time(self) -> &'a mut crate::W<REG> {
        self.variant(Hdrdlyselect::_4BitTime)
    }
    #[doc = "Delay between break and sync transmission is eight bit times. Delay between sync and ID transmission is four bit times."]
    #[inline(always)]
    pub fn _8_bit_time(self) -> &'a mut crate::W<REG> {
        self.variant(Hdrdlyselect::_8BitTime)
    }
    #[doc = "Delay between break and sync transmission is 14 bit times. Delay between sync and ID transmission is four bit times."]
    #[inline(always)]
    pub fn _14_bit_time(self) -> &'a mut crate::W<REG> {
        self.variant(Hdrdlyselect::_14BitTime)
    }
}
impl R {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&self) -> GtimeR {
        GtimeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - LIN Host Break Length"]
    #[inline(always)]
    pub fn brklen(&self) -> BrklenR {
        BrklenR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LIN Host Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&self) -> HdrdlyR {
        HdrdlyR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&mut self) -> GtimeW<'_, CtrlcSpec> {
        GtimeW::new(self, 0)
    }
    #[doc = "Bits 8:9 - LIN Host Break Length"]
    #[inline(always)]
    pub fn brklen(&mut self) -> BrklenW<'_, CtrlcSpec> {
        BrklenW::new(self, 8)
    }
    #[doc = "Bits 10:11 - LIN Host Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&mut self) -> HdrdlyW<'_, CtrlcSpec> {
        HdrdlyW::new(self, 10)
    }
}
#[doc = "USART Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Generic Clock Selection ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idselect {
    #[doc = "0: DFLL48M Reference"]
    Dfll48m = 0,
    #[doc = "1: WDT"]
    Wdt = 1,
    #[doc = "2: RTC"]
    Rtc = 2,
    #[doc = "3: EIC"]
    Eic = 3,
    #[doc = "4: EVSYS_CHANNEL_0"]
    EvsysChannel0 = 4,
    #[doc = "5: EVSYS_CHANNEL_1"]
    EvsysChannel1 = 5,
    #[doc = "6: EVSYS_CHANNEL_2"]
    EvsysChannel2 = 6,
    #[doc = "7: EVSYS_CHANNEL_3"]
    EvsysChannel3 = 7,
    #[doc = "8: EVSYS_CHANNEL_4"]
    EvsysChannel4 = 8,
    #[doc = "9: EVSYS_CHANNEL_5"]
    EvsysChannel5 = 9,
    #[doc = "10: EVSYS_CHANNEL_6"]
    EvsysChannel6 = 10,
    #[doc = "11: EVSYS_CHANNEL_7"]
    EvsysChannel7 = 11,
    #[doc = "12: SERCOMx_SLOW"]
    SercomxSlow = 12,
    #[doc = "13: SERCOM0_CORE"]
    Sercom0Core = 13,
    #[doc = "14: SERCOM1_CORE"]
    Sercom1Core = 14,
    #[doc = "15: SERCOM2_CORE"]
    Sercom2Core = 15,
    #[doc = "16: SERCOM3_CORE"]
    Sercom3Core = 16,
    #[doc = "17: SERCOM4_CORE"]
    Sercom4Core = 17,
    #[doc = "18: SERCOM5_CORE"]
    Sercom5Core = 18,
    #[doc = "19: TC0,TC1"]
    Tc0Tc1 = 19,
    #[doc = "20: TC2,TC3"]
    Tc2Tc3 = 20,
    #[doc = "21: TC4,TC5"]
    Tc4Tc5 = 21,
    #[doc = "22: TC6,TC7"]
    Tc6Tc7 = 22,
    #[doc = "23: ADC"]
    Adc = 23,
    #[doc = "24: AC_DIG"]
    AcDig = 24,
    #[doc = "25: AC_ANA"]
    AcAna = 25,
    #[doc = "26: DAC"]
    Dac = 26,
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
#[doc = "Field `ID` reader - Generic Clock Selection ID"]
pub type IdR = crate::FieldReader<Idselect>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idselect> {
        match self.bits {
            0 => Some(Idselect::Dfll48m),
            1 => Some(Idselect::Wdt),
            2 => Some(Idselect::Rtc),
            3 => Some(Idselect::Eic),
            4 => Some(Idselect::EvsysChannel0),
            5 => Some(Idselect::EvsysChannel1),
            6 => Some(Idselect::EvsysChannel2),
            7 => Some(Idselect::EvsysChannel3),
            8 => Some(Idselect::EvsysChannel4),
            9 => Some(Idselect::EvsysChannel5),
            10 => Some(Idselect::EvsysChannel6),
            11 => Some(Idselect::EvsysChannel7),
            12 => Some(Idselect::SercomxSlow),
            13 => Some(Idselect::Sercom0Core),
            14 => Some(Idselect::Sercom1Core),
            15 => Some(Idselect::Sercom2Core),
            16 => Some(Idselect::Sercom3Core),
            17 => Some(Idselect::Sercom4Core),
            18 => Some(Idselect::Sercom5Core),
            19 => Some(Idselect::Tc0Tc1),
            20 => Some(Idselect::Tc2Tc3),
            21 => Some(Idselect::Tc4Tc5),
            22 => Some(Idselect::Tc6Tc7),
            23 => Some(Idselect::Adc),
            24 => Some(Idselect::AcDig),
            25 => Some(Idselect::AcAna),
            26 => Some(Idselect::Dac),
            _ => None,
        }
    }
    #[doc = "DFLL48M Reference"]
    #[inline(always)]
    pub fn is_dfll48m(&self) -> bool {
        *self == Idselect::Dfll48m
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub fn is_wdt(&self) -> bool {
        *self == Idselect::Wdt
    }
    #[doc = "RTC"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == Idselect::Rtc
    }
    #[doc = "EIC"]
    #[inline(always)]
    pub fn is_eic(&self) -> bool {
        *self == Idselect::Eic
    }
    #[doc = "EVSYS_CHANNEL_0"]
    #[inline(always)]
    pub fn is_evsys_channel_0(&self) -> bool {
        *self == Idselect::EvsysChannel0
    }
    #[doc = "EVSYS_CHANNEL_1"]
    #[inline(always)]
    pub fn is_evsys_channel_1(&self) -> bool {
        *self == Idselect::EvsysChannel1
    }
    #[doc = "EVSYS_CHANNEL_2"]
    #[inline(always)]
    pub fn is_evsys_channel_2(&self) -> bool {
        *self == Idselect::EvsysChannel2
    }
    #[doc = "EVSYS_CHANNEL_3"]
    #[inline(always)]
    pub fn is_evsys_channel_3(&self) -> bool {
        *self == Idselect::EvsysChannel3
    }
    #[doc = "EVSYS_CHANNEL_4"]
    #[inline(always)]
    pub fn is_evsys_channel_4(&self) -> bool {
        *self == Idselect::EvsysChannel4
    }
    #[doc = "EVSYS_CHANNEL_5"]
    #[inline(always)]
    pub fn is_evsys_channel_5(&self) -> bool {
        *self == Idselect::EvsysChannel5
    }
    #[doc = "EVSYS_CHANNEL_6"]
    #[inline(always)]
    pub fn is_evsys_channel_6(&self) -> bool {
        *self == Idselect::EvsysChannel6
    }
    #[doc = "EVSYS_CHANNEL_7"]
    #[inline(always)]
    pub fn is_evsys_channel_7(&self) -> bool {
        *self == Idselect::EvsysChannel7
    }
    #[doc = "SERCOMx_SLOW"]
    #[inline(always)]
    pub fn is_sercomx_slow(&self) -> bool {
        *self == Idselect::SercomxSlow
    }
    #[doc = "SERCOM0_CORE"]
    #[inline(always)]
    pub fn is_sercom0_core(&self) -> bool {
        *self == Idselect::Sercom0Core
    }
    #[doc = "SERCOM1_CORE"]
    #[inline(always)]
    pub fn is_sercom1_core(&self) -> bool {
        *self == Idselect::Sercom1Core
    }
    #[doc = "SERCOM2_CORE"]
    #[inline(always)]
    pub fn is_sercom2_core(&self) -> bool {
        *self == Idselect::Sercom2Core
    }
    #[doc = "SERCOM3_CORE"]
    #[inline(always)]
    pub fn is_sercom3_core(&self) -> bool {
        *self == Idselect::Sercom3Core
    }
    #[doc = "SERCOM4_CORE"]
    #[inline(always)]
    pub fn is_sercom4_core(&self) -> bool {
        *self == Idselect::Sercom4Core
    }
    #[doc = "SERCOM5_CORE"]
    #[inline(always)]
    pub fn is_sercom5_core(&self) -> bool {
        *self == Idselect::Sercom5Core
    }
    #[doc = "TC0,TC1"]
    #[inline(always)]
    pub fn is_tc0_tc1(&self) -> bool {
        *self == Idselect::Tc0Tc1
    }
    #[doc = "TC2,TC3"]
    #[inline(always)]
    pub fn is_tc2_tc3(&self) -> bool {
        *self == Idselect::Tc2Tc3
    }
    #[doc = "TC4,TC5"]
    #[inline(always)]
    pub fn is_tc4_tc5(&self) -> bool {
        *self == Idselect::Tc4Tc5
    }
    #[doc = "TC6,TC7"]
    #[inline(always)]
    pub fn is_tc6_tc7(&self) -> bool {
        *self == Idselect::Tc6Tc7
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == Idselect::Adc
    }
    #[doc = "AC_DIG"]
    #[inline(always)]
    pub fn is_ac_dig(&self) -> bool {
        *self == Idselect::AcDig
    }
    #[doc = "AC_ANA"]
    #[inline(always)]
    pub fn is_ac_ana(&self) -> bool {
        *self == Idselect::AcAna
    }
    #[doc = "DAC"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Idselect::Dac
    }
}
#[doc = "Field `ID` writer - Generic Clock Selection ID"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 6, Idselect>;
impl<'a, REG> IdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DFLL48M Reference"]
    #[inline(always)]
    pub fn dfll48m(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Dfll48m)
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub fn wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Wdt)
    }
    #[doc = "RTC"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Rtc)
    }
    #[doc = "EIC"]
    #[inline(always)]
    pub fn eic(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Eic)
    }
    #[doc = "EVSYS_CHANNEL_0"]
    #[inline(always)]
    pub fn evsys_channel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::EvsysChannel0)
    }
    #[doc = "EVSYS_CHANNEL_1"]
    #[inline(always)]
    pub fn evsys_channel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::EvsysChannel1)
    }
    #[doc = "EVSYS_CHANNEL_2"]
    #[inline(always)]
    pub fn evsys_channel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::EvsysChannel2)
    }
    #[doc = "EVSYS_CHANNEL_3"]
    #[inline(always)]
    pub fn evsys_channel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::EvsysChannel3)
    }
    #[doc = "EVSYS_CHANNEL_4"]
    #[inline(always)]
    pub fn evsys_channel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::EvsysChannel4)
    }
    #[doc = "EVSYS_CHANNEL_5"]
    #[inline(always)]
    pub fn evsys_channel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::EvsysChannel5)
    }
    #[doc = "EVSYS_CHANNEL_6"]
    #[inline(always)]
    pub fn evsys_channel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::EvsysChannel6)
    }
    #[doc = "EVSYS_CHANNEL_7"]
    #[inline(always)]
    pub fn evsys_channel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::EvsysChannel7)
    }
    #[doc = "SERCOMx_SLOW"]
    #[inline(always)]
    pub fn sercomx_slow(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::SercomxSlow)
    }
    #[doc = "SERCOM0_CORE"]
    #[inline(always)]
    pub fn sercom0_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom0Core)
    }
    #[doc = "SERCOM1_CORE"]
    #[inline(always)]
    pub fn sercom1_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom1Core)
    }
    #[doc = "SERCOM2_CORE"]
    #[inline(always)]
    pub fn sercom2_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom2Core)
    }
    #[doc = "SERCOM3_CORE"]
    #[inline(always)]
    pub fn sercom3_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom3Core)
    }
    #[doc = "SERCOM4_CORE"]
    #[inline(always)]
    pub fn sercom4_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom4Core)
    }
    #[doc = "SERCOM5_CORE"]
    #[inline(always)]
    pub fn sercom5_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom5Core)
    }
    #[doc = "TC0,TC1"]
    #[inline(always)]
    pub fn tc0_tc1(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Tc0Tc1)
    }
    #[doc = "TC2,TC3"]
    #[inline(always)]
    pub fn tc2_tc3(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Tc2Tc3)
    }
    #[doc = "TC4,TC5"]
    #[inline(always)]
    pub fn tc4_tc5(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Tc4Tc5)
    }
    #[doc = "TC6,TC7"]
    #[inline(always)]
    pub fn tc6_tc7(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Tc6Tc7)
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Adc)
    }
    #[doc = "AC_DIG"]
    #[inline(always)]
    pub fn ac_dig(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::AcDig)
    }
    #[doc = "AC_ANA"]
    #[inline(always)]
    pub fn ac_ana(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::AcAna)
    }
    #[doc = "DAC"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Dac)
    }
}
#[doc = "Generic Clock Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Genselect {
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
#[doc = "Field `GEN` reader - Generic Clock Generator"]
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
            4 => Some(Genselect::Gclk4),
            5 => Some(Genselect::Gclk5),
            6 => Some(Genselect::Gclk6),
            7 => Some(Genselect::Gclk7),
            _ => None,
        }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == Genselect::Gclk0
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == Genselect::Gclk1
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == Genselect::Gclk2
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == Genselect::Gclk3
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == Genselect::Gclk4
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == Genselect::Gclk5
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == Genselect::Gclk6
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == Genselect::Gclk7
    }
}
#[doc = "Field `GEN` writer - Generic Clock Generator"]
pub type GenW<'a, REG> = crate::FieldWriter<'a, REG, 4, Genselect>;
impl<'a, REG> GenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk7)
    }
}
#[doc = "Field `CLKEN` reader - Clock Enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock Enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen_(&self) -> GenR {
        GenR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, ClkctrlSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen_(&mut self) -> GenW<'_, ClkctrlSpec> {
        GenW::new(self, 8)
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, ClkctrlSpec> {
        ClkenW::new(self, 14)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WrtlockW<'_, ClkctrlSpec> {
        WrtlockW::new(self, 15)
    }
}
#[doc = "Generic Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for ClkctrlSpec {}

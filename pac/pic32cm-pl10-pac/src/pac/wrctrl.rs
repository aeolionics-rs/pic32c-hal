#[doc = "Register `WRCTRL` reader"]
pub type R = crate::R<WrctrlSpec>;
#[doc = "Register `WRCTRL` writer"]
pub type W = crate::W<WrctrlSpec>;
#[doc = "Peripheral Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Peridselect {
    #[doc = "0: PAC - APBA Peripheral"]
    Pac = 0,
    #[doc = "1: PM - APBA Peripheral"]
    Pm = 1,
    #[doc = "2: MCLK - APBA Peripheral"]
    Mclk = 2,
    #[doc = "3: RSTC - APBA Peripheral"]
    Rstc = 3,
    #[doc = "4: OSCCTRL - APBA Peripheral"]
    Oscctrl = 4,
    #[doc = "5: OSC32KCTRL - APBA Peripheral"]
    Osc32kctrl = 5,
    #[doc = "6: SUPC - APBA Peripheral"]
    Supc = 6,
    #[doc = "7: GCLK - APBA Peripheral"]
    Gclk = 7,
    #[doc = "8: WDT - APBA Peripheral"]
    Wdt = 8,
    #[doc = "9: RTC - APBA Peripheral"]
    Rtc = 9,
    #[doc = "10: EIC - APBA Peripheral"]
    Eic = 10,
    #[doc = "32: PORT - APBB Peripheral"]
    Port = 32,
    #[doc = "33: DSU - APBB Peripheral"]
    Dsu = 33,
    #[doc = "34: NVMCTRL - APBB Peripheral"]
    Nvmctrl = 34,
    #[doc = "35: DMAC - APBB Peripheral"]
    Dmac = 35,
    #[doc = "36: MTB - APBB Peripheral"]
    Mtb = 36,
    #[doc = "37: HMATRIXHS - APBB Peripheral"]
    Hmatrixhs = 37,
    #[doc = "64: EVSYS - APBC Peripheral"]
    Evsys = 64,
    #[doc = "65: SERCOM0 - APBC Peripheral"]
    Sercom0 = 65,
    #[doc = "66: SERCOM1 - APBC Peripheral"]
    Sercom1 = 66,
    #[doc = "67: TC0 - APBC Peripheral"]
    Tc0 = 67,
    #[doc = "68: TC1 - APBC Peripheral"]
    Tc1 = 68,
    #[doc = "69: TC2 - APBC Peripheral"]
    Tc2 = 69,
    #[doc = "70: TCC0 - APBC Peripheral"]
    Tcc0 = 70,
    #[doc = "71: ADC0 - APBC Peripheral"]
    Adc0 = 71,
    #[doc = "72: AC - APBC Peripheral"]
    Ac = 72,
    #[doc = "73: CCL - APBC Peripheral"]
    Ccl = 73,
    #[doc = "74: PTC - APBC Peripheral"]
    Ptc = 74,
    #[doc = "75: SYSCTRL - APBC Peripheral"]
    Sysctrl = 75,
}
impl From<Peridselect> for u16 {
    #[inline(always)]
    fn from(variant: Peridselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Peridselect {
    type Ux = u16;
}
impl crate::IsEnum for Peridselect {}
#[doc = "Field `PERID` reader - Peripheral Identifier"]
pub type PeridR = crate::FieldReader<Peridselect>;
impl PeridR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Peridselect> {
        match self.bits {
            0 => Some(Peridselect::Pac),
            1 => Some(Peridselect::Pm),
            2 => Some(Peridselect::Mclk),
            3 => Some(Peridselect::Rstc),
            4 => Some(Peridselect::Oscctrl),
            5 => Some(Peridselect::Osc32kctrl),
            6 => Some(Peridselect::Supc),
            7 => Some(Peridselect::Gclk),
            8 => Some(Peridselect::Wdt),
            9 => Some(Peridselect::Rtc),
            10 => Some(Peridselect::Eic),
            32 => Some(Peridselect::Port),
            33 => Some(Peridselect::Dsu),
            34 => Some(Peridselect::Nvmctrl),
            35 => Some(Peridselect::Dmac),
            36 => Some(Peridselect::Mtb),
            37 => Some(Peridselect::Hmatrixhs),
            64 => Some(Peridselect::Evsys),
            65 => Some(Peridselect::Sercom0),
            66 => Some(Peridselect::Sercom1),
            67 => Some(Peridselect::Tc0),
            68 => Some(Peridselect::Tc1),
            69 => Some(Peridselect::Tc2),
            70 => Some(Peridselect::Tcc0),
            71 => Some(Peridselect::Adc0),
            72 => Some(Peridselect::Ac),
            73 => Some(Peridselect::Ccl),
            74 => Some(Peridselect::Ptc),
            75 => Some(Peridselect::Sysctrl),
            _ => None,
        }
    }
    #[doc = "PAC - APBA Peripheral"]
    #[inline(always)]
    pub fn is_pac(&self) -> bool {
        *self == Peridselect::Pac
    }
    #[doc = "PM - APBA Peripheral"]
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == Peridselect::Pm
    }
    #[doc = "MCLK - APBA Peripheral"]
    #[inline(always)]
    pub fn is_mclk(&self) -> bool {
        *self == Peridselect::Mclk
    }
    #[doc = "RSTC - APBA Peripheral"]
    #[inline(always)]
    pub fn is_rstc(&self) -> bool {
        *self == Peridselect::Rstc
    }
    #[doc = "OSCCTRL - APBA Peripheral"]
    #[inline(always)]
    pub fn is_oscctrl(&self) -> bool {
        *self == Peridselect::Oscctrl
    }
    #[doc = "OSC32KCTRL - APBA Peripheral"]
    #[inline(always)]
    pub fn is_osc32kctrl(&self) -> bool {
        *self == Peridselect::Osc32kctrl
    }
    #[doc = "SUPC - APBA Peripheral"]
    #[inline(always)]
    pub fn is_supc(&self) -> bool {
        *self == Peridselect::Supc
    }
    #[doc = "GCLK - APBA Peripheral"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == Peridselect::Gclk
    }
    #[doc = "WDT - APBA Peripheral"]
    #[inline(always)]
    pub fn is_wdt(&self) -> bool {
        *self == Peridselect::Wdt
    }
    #[doc = "RTC - APBA Peripheral"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == Peridselect::Rtc
    }
    #[doc = "EIC - APBA Peripheral"]
    #[inline(always)]
    pub fn is_eic(&self) -> bool {
        *self == Peridselect::Eic
    }
    #[doc = "PORT - APBB Peripheral"]
    #[inline(always)]
    pub fn is_port(&self) -> bool {
        *self == Peridselect::Port
    }
    #[doc = "DSU - APBB Peripheral"]
    #[inline(always)]
    pub fn is_dsu(&self) -> bool {
        *self == Peridselect::Dsu
    }
    #[doc = "NVMCTRL - APBB Peripheral"]
    #[inline(always)]
    pub fn is_nvmctrl(&self) -> bool {
        *self == Peridselect::Nvmctrl
    }
    #[doc = "DMAC - APBB Peripheral"]
    #[inline(always)]
    pub fn is_dmac(&self) -> bool {
        *self == Peridselect::Dmac
    }
    #[doc = "MTB - APBB Peripheral"]
    #[inline(always)]
    pub fn is_mtb(&self) -> bool {
        *self == Peridselect::Mtb
    }
    #[doc = "HMATRIXHS - APBB Peripheral"]
    #[inline(always)]
    pub fn is_hmatrixhs(&self) -> bool {
        *self == Peridselect::Hmatrixhs
    }
    #[doc = "EVSYS - APBC Peripheral"]
    #[inline(always)]
    pub fn is_evsys(&self) -> bool {
        *self == Peridselect::Evsys
    }
    #[doc = "SERCOM0 - APBC Peripheral"]
    #[inline(always)]
    pub fn is_sercom0(&self) -> bool {
        *self == Peridselect::Sercom0
    }
    #[doc = "SERCOM1 - APBC Peripheral"]
    #[inline(always)]
    pub fn is_sercom1(&self) -> bool {
        *self == Peridselect::Sercom1
    }
    #[doc = "TC0 - APBC Peripheral"]
    #[inline(always)]
    pub fn is_tc0(&self) -> bool {
        *self == Peridselect::Tc0
    }
    #[doc = "TC1 - APBC Peripheral"]
    #[inline(always)]
    pub fn is_tc1(&self) -> bool {
        *self == Peridselect::Tc1
    }
    #[doc = "TC2 - APBC Peripheral"]
    #[inline(always)]
    pub fn is_tc2(&self) -> bool {
        *self == Peridselect::Tc2
    }
    #[doc = "TCC0 - APBC Peripheral"]
    #[inline(always)]
    pub fn is_tcc0(&self) -> bool {
        *self == Peridselect::Tcc0
    }
    #[doc = "ADC0 - APBC Peripheral"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Peridselect::Adc0
    }
    #[doc = "AC - APBC Peripheral"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        *self == Peridselect::Ac
    }
    #[doc = "CCL - APBC Peripheral"]
    #[inline(always)]
    pub fn is_ccl(&self) -> bool {
        *self == Peridselect::Ccl
    }
    #[doc = "PTC - APBC Peripheral"]
    #[inline(always)]
    pub fn is_ptc(&self) -> bool {
        *self == Peridselect::Ptc
    }
    #[doc = "SYSCTRL - APBC Peripheral"]
    #[inline(always)]
    pub fn is_sysctrl(&self) -> bool {
        *self == Peridselect::Sysctrl
    }
}
#[doc = "Field `PERID` writer - Peripheral Identifier"]
pub type PeridW<'a, REG> = crate::FieldWriter<'a, REG, 16, Peridselect>;
impl<'a, REG> PeridW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "PAC - APBA Peripheral"]
    #[inline(always)]
    pub fn pac(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Pac)
    }
    #[doc = "PM - APBA Peripheral"]
    #[inline(always)]
    pub fn pm(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Pm)
    }
    #[doc = "MCLK - APBA Peripheral"]
    #[inline(always)]
    pub fn mclk(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Mclk)
    }
    #[doc = "RSTC - APBA Peripheral"]
    #[inline(always)]
    pub fn rstc(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Rstc)
    }
    #[doc = "OSCCTRL - APBA Peripheral"]
    #[inline(always)]
    pub fn oscctrl(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Oscctrl)
    }
    #[doc = "OSC32KCTRL - APBA Peripheral"]
    #[inline(always)]
    pub fn osc32kctrl(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Osc32kctrl)
    }
    #[doc = "SUPC - APBA Peripheral"]
    #[inline(always)]
    pub fn supc(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Supc)
    }
    #[doc = "GCLK - APBA Peripheral"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Gclk)
    }
    #[doc = "WDT - APBA Peripheral"]
    #[inline(always)]
    pub fn wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Wdt)
    }
    #[doc = "RTC - APBA Peripheral"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Rtc)
    }
    #[doc = "EIC - APBA Peripheral"]
    #[inline(always)]
    pub fn eic(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Eic)
    }
    #[doc = "PORT - APBB Peripheral"]
    #[inline(always)]
    pub fn port(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Port)
    }
    #[doc = "DSU - APBB Peripheral"]
    #[inline(always)]
    pub fn dsu(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Dsu)
    }
    #[doc = "NVMCTRL - APBB Peripheral"]
    #[inline(always)]
    pub fn nvmctrl(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Nvmctrl)
    }
    #[doc = "DMAC - APBB Peripheral"]
    #[inline(always)]
    pub fn dmac(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Dmac)
    }
    #[doc = "MTB - APBB Peripheral"]
    #[inline(always)]
    pub fn mtb(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Mtb)
    }
    #[doc = "HMATRIXHS - APBB Peripheral"]
    #[inline(always)]
    pub fn hmatrixhs(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Hmatrixhs)
    }
    #[doc = "EVSYS - APBC Peripheral"]
    #[inline(always)]
    pub fn evsys(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Evsys)
    }
    #[doc = "SERCOM0 - APBC Peripheral"]
    #[inline(always)]
    pub fn sercom0(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Sercom0)
    }
    #[doc = "SERCOM1 - APBC Peripheral"]
    #[inline(always)]
    pub fn sercom1(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Sercom1)
    }
    #[doc = "TC0 - APBC Peripheral"]
    #[inline(always)]
    pub fn tc0(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc0)
    }
    #[doc = "TC1 - APBC Peripheral"]
    #[inline(always)]
    pub fn tc1(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc1)
    }
    #[doc = "TC2 - APBC Peripheral"]
    #[inline(always)]
    pub fn tc2(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc2)
    }
    #[doc = "TCC0 - APBC Peripheral"]
    #[inline(always)]
    pub fn tcc0(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tcc0)
    }
    #[doc = "ADC0 - APBC Peripheral"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Adc0)
    }
    #[doc = "AC - APBC Peripheral"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Ac)
    }
    #[doc = "CCL - APBC Peripheral"]
    #[inline(always)]
    pub fn ccl(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Ccl)
    }
    #[doc = "PTC - APBC Peripheral"]
    #[inline(always)]
    pub fn ptc(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Ptc)
    }
    #[doc = "SYSCTRL - APBC Peripheral"]
    #[inline(always)]
    pub fn sysctrl(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Sysctrl)
    }
}
#[doc = "Peripheral Access Control Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyselect {
    #[doc = "0: No action"]
    Off = 0,
    #[doc = "1: Clear the peripheral write-control"]
    Clr = 1,
    #[doc = "2: Set the peripheral write-control"]
    Set = 2,
    #[doc = "3: Set and lock the peripheral write-control until the next hardware reset"]
    Setlck = 3,
}
impl From<Keyselect> for u8 {
    #[inline(always)]
    fn from(variant: Keyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyselect {
    type Ux = u8;
}
impl crate::IsEnum for Keyselect {}
#[doc = "Field `KEY` reader - Peripheral Access Control Key"]
pub type KeyR = crate::FieldReader<Keyselect>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyselect> {
        match self.bits {
            0 => Some(Keyselect::Off),
            1 => Some(Keyselect::Clr),
            2 => Some(Keyselect::Set),
            3 => Some(Keyselect::Setlck),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Keyselect::Off
    }
    #[doc = "Clear the peripheral write-control"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Keyselect::Clr
    }
    #[doc = "Set the peripheral write-control"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Keyselect::Set
    }
    #[doc = "Set and lock the peripheral write-control until the next hardware reset"]
    #[inline(always)]
    pub fn is_setlck(&self) -> bool {
        *self == Keyselect::Setlck
    }
}
#[doc = "Field `KEY` writer - Peripheral Access Control Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Keyselect>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Off)
    }
    #[doc = "Clear the peripheral write-control"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Clr)
    }
    #[doc = "Set the peripheral write-control"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Set)
    }
    #[doc = "Set and lock the peripheral write-control until the next hardware reset"]
    #[inline(always)]
    pub fn setlck(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Setlck)
    }
}
impl R {
    #[doc = "Bits 0:15 - Peripheral Identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PeridR {
        PeridR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Peripheral Access Control Key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Peripheral Identifier"]
    #[inline(always)]
    pub fn perid(&mut self) -> PeridW<'_, WrctrlSpec> {
        PeridW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Peripheral Access Control Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, WrctrlSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Write Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrctrlSpec;
impl crate::RegisterSpec for WrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrctrl::R`](R) reader structure"]
impl crate::Readable for WrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wrctrl::W`](W) writer structure"]
impl crate::Writable for WrctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRCTRL to value 0"]
impl crate::Resettable for WrctrlSpec {}

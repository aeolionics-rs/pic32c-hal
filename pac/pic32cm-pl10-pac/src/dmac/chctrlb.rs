#[doc = "Register `CHCTRLB` reader"]
pub type R = crate::R<ChctrlbSpec>;
#[doc = "Register `CHCTRLB` writer"]
pub type W = crate::W<ChctrlbSpec>;
#[doc = "Event Input Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evactselect {
    #[doc = "0: No action"]
    Noact = 0,
    #[doc = "1: Normal transfer and conditional transfer on strobe trigger"]
    Trig = 1,
    #[doc = "2: Conditional transfer trigger"]
    Ctrig = 2,
    #[doc = "3: Conditional block transfer"]
    Cblock = 3,
    #[doc = "4: Channel suspend operation"]
    Suspend = 4,
    #[doc = "5: Channel resume operation"]
    Resume = 5,
    #[doc = "6: Skip next block suspend action"]
    Sskip = 6,
}
impl From<Evactselect> for u8 {
    #[inline(always)]
    fn from(variant: Evactselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evactselect {
    type Ux = u8;
}
impl crate::IsEnum for Evactselect {}
#[doc = "Field `EVACT` reader - Event Input Action"]
pub type EvactR = crate::FieldReader<Evactselect>;
impl EvactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Evactselect> {
        match self.bits {
            0 => Some(Evactselect::Noact),
            1 => Some(Evactselect::Trig),
            2 => Some(Evactselect::Ctrig),
            3 => Some(Evactselect::Cblock),
            4 => Some(Evactselect::Suspend),
            5 => Some(Evactselect::Resume),
            6 => Some(Evactselect::Sskip),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == Evactselect::Noact
    }
    #[doc = "Normal transfer and conditional transfer on strobe trigger"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Evactselect::Trig
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn is_ctrig(&self) -> bool {
        *self == Evactselect::Ctrig
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn is_cblock(&self) -> bool {
        *self == Evactselect::Cblock
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Evactselect::Suspend
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Evactselect::Resume
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn is_sskip(&self) -> bool {
        *self == Evactselect::Sskip
    }
}
#[doc = "Field `EVACT` writer - Event Input Action"]
pub type EvactW<'a, REG> = crate::FieldWriter<'a, REG, 3, Evactselect>;
impl<'a, REG> EvactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Noact)
    }
    #[doc = "Normal transfer and conditional transfer on strobe trigger"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Trig)
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn ctrig(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Ctrig)
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn cblock(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Cblock)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Suspend)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Resume)
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn sskip(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Sskip)
    }
}
#[doc = "Field `EVIE` reader - Channel Event Input Enable"]
pub type EvieR = crate::BitReader;
#[doc = "Field `EVIE` writer - Channel Event Input Enable"]
pub type EvieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOE` reader - Channel Event Output Enable"]
pub type EvoeR = crate::BitReader;
#[doc = "Field `EVOE` writer - Channel Event Output Enable"]
pub type EvoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel Arbitration Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvlselect {
    #[doc = "0: Channel priority level 0"]
    Lvl0 = 0,
    #[doc = "1: Channel priority level 1"]
    Lvl1 = 1,
    #[doc = "2: Channel priority level 2"]
    Lvl2 = 2,
    #[doc = "3: Channel priority level 3"]
    Lvl3 = 3,
}
impl From<Lvlselect> for u8 {
    #[inline(always)]
    fn from(variant: Lvlselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvlselect {
    type Ux = u8;
}
impl crate::IsEnum for Lvlselect {}
#[doc = "Field `LVL` reader - Channel Arbitration Level"]
pub type LvlR = crate::FieldReader<Lvlselect>;
impl LvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lvlselect> {
        match self.bits {
            0 => Some(Lvlselect::Lvl0),
            1 => Some(Lvlselect::Lvl1),
            2 => Some(Lvlselect::Lvl2),
            3 => Some(Lvlselect::Lvl3),
            _ => None,
        }
    }
    #[doc = "Channel priority level 0"]
    #[inline(always)]
    pub fn is_lvl0(&self) -> bool {
        *self == Lvlselect::Lvl0
    }
    #[doc = "Channel priority level 1"]
    #[inline(always)]
    pub fn is_lvl1(&self) -> bool {
        *self == Lvlselect::Lvl1
    }
    #[doc = "Channel priority level 2"]
    #[inline(always)]
    pub fn is_lvl2(&self) -> bool {
        *self == Lvlselect::Lvl2
    }
    #[doc = "Channel priority level 3"]
    #[inline(always)]
    pub fn is_lvl3(&self) -> bool {
        *self == Lvlselect::Lvl3
    }
}
#[doc = "Field `LVL` writer - Channel Arbitration Level"]
pub type LvlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lvlselect>;
impl<'a, REG> LvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel priority level 0"]
    #[inline(always)]
    pub fn lvl0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlselect::Lvl0)
    }
    #[doc = "Channel priority level 1"]
    #[inline(always)]
    pub fn lvl1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlselect::Lvl1)
    }
    #[doc = "Channel priority level 2"]
    #[inline(always)]
    pub fn lvl2(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlselect::Lvl2)
    }
    #[doc = "Channel priority level 3"]
    #[inline(always)]
    pub fn lvl3(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlselect::Lvl3)
    }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsrcselect {
    #[doc = "0: Only software/event triggers"]
    Disable = 0,
    #[doc = "1: SERCOM0 RX Trigger"]
    Sercom0Rx = 1,
    #[doc = "2: SERCOM0 TX Trigger"]
    Sercom0Tx = 2,
    #[doc = "3: SERCOM1 RX Trigger"]
    Sercom1Rx = 3,
    #[doc = "4: SERCOM1 TX Trigger"]
    Sercom1Tx = 4,
    #[doc = "5: TC0 Overflow Trigger"]
    Tc0Ovf = 5,
    #[doc = "6: TC0 Match/Compare 0 Trigger"]
    Tc0Mc0 = 6,
    #[doc = "7: TC0 Match/Compare 1 Trigger"]
    Tc0Mc1 = 7,
    #[doc = "8: TC1 Overflow Trigger"]
    Tc1Ovf = 8,
    #[doc = "9: TC1 Match/Compare 0 Trigger"]
    Tc1Mc0 = 9,
    #[doc = "10: TC1 Match/Compare 1 Trigger"]
    Tc1Mc1 = 10,
    #[doc = "11: TC2 Overflow Trigger"]
    Tc2Ovf = 11,
    #[doc = "12: TC2 Match/Compare 0 Trigger"]
    Tc2Mc0 = 12,
    #[doc = "13: TC2 Match/Compare 1 Trigger"]
    Tc2Mc1 = 13,
    #[doc = "14: TCC0 Overflow Trigger"]
    Tcc0Ovf = 14,
    #[doc = "15: TCC0 Match/Compare 0 Trigger"]
    Tcc0Mc0 = 15,
    #[doc = "16: TCC0 Match/Compare 1 Trigger"]
    Tcc0Mc1 = 16,
    #[doc = "17: TCC0 Match/Compare 2 Trigger"]
    Tcc0Mc2 = 17,
    #[doc = "18: TCC0 Match/Compare 3 Trigger"]
    Tcc0Mc3 = 18,
    #[doc = "19: ADC0 Restult Ready Trigger"]
    Adc0Resrdy = 19,
    #[doc = "20: ADC0 Sample Ready Trigger"]
    Adc0Samprdy = 20,
    #[doc = "21: DSU Debug Communication Channel 0"]
    DsuDcc0 = 21,
    #[doc = "22: DSU Debug Communication Channel 1"]
    DsuDcc1 = 22,
}
impl From<Trigsrcselect> for u8 {
    #[inline(always)]
    fn from(variant: Trigsrcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsrcselect {
    type Ux = u8;
}
impl crate::IsEnum for Trigsrcselect {}
#[doc = "Field `TRIGSRC` reader - Trigger Source"]
pub type TrigsrcR = crate::FieldReader<Trigsrcselect>;
impl TrigsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigsrcselect> {
        match self.bits {
            0 => Some(Trigsrcselect::Disable),
            1 => Some(Trigsrcselect::Sercom0Rx),
            2 => Some(Trigsrcselect::Sercom0Tx),
            3 => Some(Trigsrcselect::Sercom1Rx),
            4 => Some(Trigsrcselect::Sercom1Tx),
            5 => Some(Trigsrcselect::Tc0Ovf),
            6 => Some(Trigsrcselect::Tc0Mc0),
            7 => Some(Trigsrcselect::Tc0Mc1),
            8 => Some(Trigsrcselect::Tc1Ovf),
            9 => Some(Trigsrcselect::Tc1Mc0),
            10 => Some(Trigsrcselect::Tc1Mc1),
            11 => Some(Trigsrcselect::Tc2Ovf),
            12 => Some(Trigsrcselect::Tc2Mc0),
            13 => Some(Trigsrcselect::Tc2Mc1),
            14 => Some(Trigsrcselect::Tcc0Ovf),
            15 => Some(Trigsrcselect::Tcc0Mc0),
            16 => Some(Trigsrcselect::Tcc0Mc1),
            17 => Some(Trigsrcselect::Tcc0Mc2),
            18 => Some(Trigsrcselect::Tcc0Mc3),
            19 => Some(Trigsrcselect::Adc0Resrdy),
            20 => Some(Trigsrcselect::Adc0Samprdy),
            21 => Some(Trigsrcselect::DsuDcc0),
            22 => Some(Trigsrcselect::DsuDcc1),
            _ => None,
        }
    }
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Trigsrcselect::Disable
    }
    #[doc = "SERCOM0 RX Trigger"]
    #[inline(always)]
    pub fn is_sercom0_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom0Rx
    }
    #[doc = "SERCOM0 TX Trigger"]
    #[inline(always)]
    pub fn is_sercom0_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom0Tx
    }
    #[doc = "SERCOM1 RX Trigger"]
    #[inline(always)]
    pub fn is_sercom1_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom1Rx
    }
    #[doc = "SERCOM1 TX Trigger"]
    #[inline(always)]
    pub fn is_sercom1_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom1Tx
    }
    #[doc = "TC0 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tc0_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc0Ovf
    }
    #[doc = "TC0 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tc0_mc0(&self) -> bool {
        *self == Trigsrcselect::Tc0Mc0
    }
    #[doc = "TC0 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tc0_mc1(&self) -> bool {
        *self == Trigsrcselect::Tc0Mc1
    }
    #[doc = "TC1 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tc1_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc1Ovf
    }
    #[doc = "TC1 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tc1_mc0(&self) -> bool {
        *self == Trigsrcselect::Tc1Mc0
    }
    #[doc = "TC1 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tc1_mc1(&self) -> bool {
        *self == Trigsrcselect::Tc1Mc1
    }
    #[doc = "TC2 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tc2_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc2Ovf
    }
    #[doc = "TC2 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tc2_mc0(&self) -> bool {
        *self == Trigsrcselect::Tc2Mc0
    }
    #[doc = "TC2 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tc2_mc1(&self) -> bool {
        *self == Trigsrcselect::Tc2Mc1
    }
    #[doc = "TCC0 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tcc0_ovf(&self) -> bool {
        *self == Trigsrcselect::Tcc0Ovf
    }
    #[doc = "TCC0 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc0(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc0
    }
    #[doc = "TCC0 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc1(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc1
    }
    #[doc = "TCC0 Match/Compare 2 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc2(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc2
    }
    #[doc = "TCC0 Match/Compare 3 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc3(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc3
    }
    #[doc = "ADC0 Restult Ready Trigger"]
    #[inline(always)]
    pub fn is_adc0_resrdy(&self) -> bool {
        *self == Trigsrcselect::Adc0Resrdy
    }
    #[doc = "ADC0 Sample Ready Trigger"]
    #[inline(always)]
    pub fn is_adc0_samprdy(&self) -> bool {
        *self == Trigsrcselect::Adc0Samprdy
    }
    #[doc = "DSU Debug Communication Channel 0"]
    #[inline(always)]
    pub fn is_dsu_dcc0(&self) -> bool {
        *self == Trigsrcselect::DsuDcc0
    }
    #[doc = "DSU Debug Communication Channel 1"]
    #[inline(always)]
    pub fn is_dsu_dcc1(&self) -> bool {
        *self == Trigsrcselect::DsuDcc1
    }
}
#[doc = "Field `TRIGSRC` writer - Trigger Source"]
pub type TrigsrcW<'a, REG> = crate::FieldWriter<'a, REG, 8, Trigsrcselect>;
impl<'a, REG> TrigsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Disable)
    }
    #[doc = "SERCOM0 RX Trigger"]
    #[inline(always)]
    pub fn sercom0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom0Rx)
    }
    #[doc = "SERCOM0 TX Trigger"]
    #[inline(always)]
    pub fn sercom0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom0Tx)
    }
    #[doc = "SERCOM1 RX Trigger"]
    #[inline(always)]
    pub fn sercom1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom1Rx)
    }
    #[doc = "SERCOM1 TX Trigger"]
    #[inline(always)]
    pub fn sercom1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom1Tx)
    }
    #[doc = "TC0 Overflow Trigger"]
    #[inline(always)]
    pub fn tc0_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc0Ovf)
    }
    #[doc = "TC0 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc0_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc0Mc0)
    }
    #[doc = "TC0 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc0_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc0Mc1)
    }
    #[doc = "TC1 Overflow Trigger"]
    #[inline(always)]
    pub fn tc1_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc1Ovf)
    }
    #[doc = "TC1 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc1_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc1Mc0)
    }
    #[doc = "TC1 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc1_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc1Mc1)
    }
    #[doc = "TC2 Overflow Trigger"]
    #[inline(always)]
    pub fn tc2_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc2Ovf)
    }
    #[doc = "TC2 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc2_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc2Mc0)
    }
    #[doc = "TC2 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc2_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc2Mc1)
    }
    #[doc = "TCC0 Overflow Trigger"]
    #[inline(always)]
    pub fn tcc0_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Ovf)
    }
    #[doc = "TCC0 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc0)
    }
    #[doc = "TCC0 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc1)
    }
    #[doc = "TCC0 Match/Compare 2 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc2)
    }
    #[doc = "TCC0 Match/Compare 3 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc3)
    }
    #[doc = "ADC0 Restult Ready Trigger"]
    #[inline(always)]
    pub fn adc0_resrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Adc0Resrdy)
    }
    #[doc = "ADC0 Sample Ready Trigger"]
    #[inline(always)]
    pub fn adc0_samprdy(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Adc0Samprdy)
    }
    #[doc = "DSU Debug Communication Channel 0"]
    #[inline(always)]
    pub fn dsu_dcc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::DsuDcc0)
    }
    #[doc = "DSU Debug Communication Channel 1"]
    #[inline(always)]
    pub fn dsu_dcc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::DsuDcc1)
    }
}
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigactselect {
    #[doc = "0: One trigger required for each block transfer"]
    Block = 0,
    #[doc = "2: One trigger required for each beat transfer"]
    Beat = 2,
    #[doc = "3: One trigger required for each transaction"]
    Transaction = 3,
}
impl From<Trigactselect> for u8 {
    #[inline(always)]
    fn from(variant: Trigactselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigactselect {
    type Ux = u8;
}
impl crate::IsEnum for Trigactselect {}
#[doc = "Field `TRIGACT` reader - Trigger Action"]
pub type TrigactR = crate::FieldReader<Trigactselect>;
impl TrigactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigactselect> {
        match self.bits {
            0 => Some(Trigactselect::Block),
            2 => Some(Trigactselect::Beat),
            3 => Some(Trigactselect::Transaction),
            _ => None,
        }
    }
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == Trigactselect::Block
    }
    #[doc = "One trigger required for each beat transfer"]
    #[inline(always)]
    pub fn is_beat(&self) -> bool {
        *self == Trigactselect::Beat
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == Trigactselect::Transaction
    }
}
#[doc = "Field `TRIGACT` writer - Trigger Action"]
pub type TrigactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigactselect>;
impl<'a, REG> TrigactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Block)
    }
    #[doc = "One trigger required for each beat transfer"]
    #[inline(always)]
    pub fn beat(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Beat)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Transaction)
    }
}
#[doc = "Software Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdselect {
    #[doc = "0: No action"]
    Noact = 0,
    #[doc = "1: Channel suspend operation"]
    Suspend = 1,
    #[doc = "2: Channel resume operation"]
    Resume = 2,
}
impl From<Cmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmdselect {}
#[doc = "Field `CMD` reader - Software Command"]
pub type CmdR = crate::FieldReader<Cmdselect>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdselect> {
        match self.bits {
            0 => Some(Cmdselect::Noact),
            1 => Some(Cmdselect::Suspend),
            2 => Some(Cmdselect::Resume),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == Cmdselect::Noact
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Cmdselect::Suspend
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Cmdselect::Resume
    }
}
#[doc = "Field `CMD` writer - Software Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmdselect>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Noact)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Suspend)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Resume)
    }
}
impl R {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    pub fn evact(&self) -> EvactR {
        EvactR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&self) -> EvieR {
        EvieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EvoeR {
        EvoeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Channel Arbitration Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LvlR {
        LvlR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TrigsrcR {
        TrigsrcR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TrigactR {
        TrigactR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    pub fn evact(&mut self) -> EvactW<'_, ChctrlbSpec> {
        EvactW::new(self, 0)
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&mut self) -> EvieW<'_, ChctrlbSpec> {
        EvieW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&mut self) -> EvoeW<'_, ChctrlbSpec> {
        EvoeW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Channel Arbitration Level"]
    #[inline(always)]
    pub fn lvl(&mut self) -> LvlW<'_, ChctrlbSpec> {
        LvlW::new(self, 5)
    }
    #[doc = "Bits 8:15 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&mut self) -> TrigsrcW<'_, ChctrlbSpec> {
        TrigsrcW::new(self, 8)
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&mut self) -> TrigactW<'_, ChctrlbSpec> {
        TrigactW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, ChctrlbSpec> {
        CmdW::new(self, 24)
    }
}
#[doc = "Channel Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctrlbSpec;
impl crate::RegisterSpec for ChctrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctrlb::R`](R) reader structure"]
impl crate::Readable for ChctrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`chctrlb::W`](W) writer structure"]
impl crate::Writable for ChctrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTRLB to value 0"]
impl crate::Resettable for ChctrlbSpec {}

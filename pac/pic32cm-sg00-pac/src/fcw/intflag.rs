#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "NVM Operation Done Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doneselect {
    #[doc = "0: NVMOP Not Done"]
    Done0 = 0,
    #[doc = "1: NVMOP Done"]
    Done1 = 1,
}
impl From<Doneselect> for bool {
    #[inline(always)]
    fn from(variant: Doneselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - NVM Operation Done Flag Bit"]
pub type DoneR = crate::BitReader<Doneselect>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doneselect {
        match self.bits {
            false => Doneselect::Done0,
            true => Doneselect::Done1,
        }
    }
    #[doc = "NVMOP Not Done"]
    #[inline(always)]
    pub fn is_done_0(&self) -> bool {
        *self == Doneselect::Done0
    }
    #[doc = "NVMOP Done"]
    #[inline(always)]
    pub fn is_done_1(&self) -> bool {
        *self == Doneselect::Done1
    }
}
#[doc = "Field `DONE` writer - NVM Operation Done Flag Bit"]
pub type DoneW<'a, REG> = crate::BitWriter1C<'a, REG, Doneselect>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NVMOP Not Done"]
    #[inline(always)]
    pub fn done_0(self) -> &'a mut crate::W<REG> {
        self.variant(Doneselect::Done0)
    }
    #[doc = "NVMOP Done"]
    #[inline(always)]
    pub fn done_1(self) -> &'a mut crate::W<REG> {
        self.variant(Doneselect::Done1)
    }
}
#[doc = "Key Error Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyerrselect {
    #[doc = "0: No Key Error"]
    Keyerr0 = 0,
    #[doc = "1: Key Error"]
    Keyerr1 = 1,
}
impl From<Keyerrselect> for bool {
    #[inline(always)]
    fn from(variant: Keyerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYERR` reader - Key Error Flag Bit"]
pub type KeyerrR = crate::BitReader<Keyerrselect>;
impl KeyerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keyerrselect {
        match self.bits {
            false => Keyerrselect::Keyerr0,
            true => Keyerrselect::Keyerr1,
        }
    }
    #[doc = "No Key Error"]
    #[inline(always)]
    pub fn is_keyerr_0(&self) -> bool {
        *self == Keyerrselect::Keyerr0
    }
    #[doc = "Key Error"]
    #[inline(always)]
    pub fn is_keyerr_1(&self) -> bool {
        *self == Keyerrselect::Keyerr1
    }
}
#[doc = "Field `KEYERR` writer - Key Error Flag Bit"]
pub type KeyerrW<'a, REG> = crate::BitWriter1C<'a, REG, Keyerrselect>;
impl<'a, REG> KeyerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Key Error"]
    #[inline(always)]
    pub fn keyerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Keyerrselect::Keyerr0)
    }
    #[doc = "Key Error"]
    #[inline(always)]
    pub fn keyerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Keyerrselect::Keyerr1)
    }
}
#[doc = "Configuration Error Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfgerrselect {
    #[doc = "0: No CFG Error"]
    Cfgerr0 = 0,
    #[doc = "1: CFC Error"]
    Cfgerr1 = 1,
}
impl From<Cfgerrselect> for bool {
    #[inline(always)]
    fn from(variant: Cfgerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGERR` reader - Configuration Error Flag Bit"]
pub type CfgerrR = crate::BitReader<Cfgerrselect>;
impl CfgerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfgerrselect {
        match self.bits {
            false => Cfgerrselect::Cfgerr0,
            true => Cfgerrselect::Cfgerr1,
        }
    }
    #[doc = "No CFG Error"]
    #[inline(always)]
    pub fn is_cfgerr_0(&self) -> bool {
        *self == Cfgerrselect::Cfgerr0
    }
    #[doc = "CFC Error"]
    #[inline(always)]
    pub fn is_cfgerr_1(&self) -> bool {
        *self == Cfgerrselect::Cfgerr1
    }
}
#[doc = "Field `CFGERR` writer - Configuration Error Flag Bit"]
pub type CfgerrW<'a, REG> = crate::BitWriter1C<'a, REG, Cfgerrselect>;
impl<'a, REG> CfgerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CFG Error"]
    #[inline(always)]
    pub fn cfgerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgerrselect::Cfgerr0)
    }
    #[doc = "CFC Error"]
    #[inline(always)]
    pub fn cfgerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgerrselect::Cfgerr1)
    }
}
#[doc = "FIFO Underrun during Row Write Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoerrselect {
    #[doc = "0: No FIFO Error"]
    Fifoerr0 = 0,
    #[doc = "1: FIFO Error"]
    Fifoerr1 = 1,
}
impl From<Fifoerrselect> for bool {
    #[inline(always)]
    fn from(variant: Fifoerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOERR` reader - FIFO Underrun during Row Write Flag Bit"]
pub type FifoerrR = crate::BitReader<Fifoerrselect>;
impl FifoerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifoerrselect {
        match self.bits {
            false => Fifoerrselect::Fifoerr0,
            true => Fifoerrselect::Fifoerr1,
        }
    }
    #[doc = "No FIFO Error"]
    #[inline(always)]
    pub fn is_fifoerr_0(&self) -> bool {
        *self == Fifoerrselect::Fifoerr0
    }
    #[doc = "FIFO Error"]
    #[inline(always)]
    pub fn is_fifoerr_1(&self) -> bool {
        *self == Fifoerrselect::Fifoerr1
    }
}
#[doc = "Field `FIFOERR` writer - FIFO Underrun during Row Write Flag Bit"]
pub type FifoerrW<'a, REG> = crate::BitWriter1C<'a, REG, Fifoerrselect>;
impl<'a, REG> FifoerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No FIFO Error"]
    #[inline(always)]
    pub fn fifoerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoerrselect::Fifoerr0)
    }
    #[doc = "FIFO Error"]
    #[inline(always)]
    pub fn fifoerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoerrselect::Fifoerr1)
    }
}
#[doc = "AHB Bus Error during Row Write Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Buserrselect {
    #[doc = "0: No Bus Error"]
    Buserr0 = 0,
    #[doc = "1: Bus Error"]
    Buserr1 = 1,
}
impl From<Buserrselect> for bool {
    #[inline(always)]
    fn from(variant: Buserrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSERR` reader - AHB Bus Error during Row Write Flag Bit"]
pub type BuserrR = crate::BitReader<Buserrselect>;
impl BuserrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Buserrselect {
        match self.bits {
            false => Buserrselect::Buserr0,
            true => Buserrselect::Buserr1,
        }
    }
    #[doc = "No Bus Error"]
    #[inline(always)]
    pub fn is_buserr_0(&self) -> bool {
        *self == Buserrselect::Buserr0
    }
    #[doc = "Bus Error"]
    #[inline(always)]
    pub fn is_buserr_1(&self) -> bool {
        *self == Buserrselect::Buserr1
    }
}
#[doc = "Field `BUSERR` writer - AHB Bus Error during Row Write Flag Bit"]
pub type BuserrW<'a, REG> = crate::BitWriter1C<'a, REG, Buserrselect>;
impl<'a, REG> BuserrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Bus Error"]
    #[inline(always)]
    pub fn buserr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Buserrselect::Buserr0)
    }
    #[doc = "Bus Error"]
    #[inline(always)]
    pub fn buserr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Buserrselect::Buserr1)
    }
}
#[doc = "Write Protection Error Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wperrselect {
    #[doc = "0: No Write Protection Error"]
    Wperr0 = 0,
    #[doc = "1: Write Protection Error"]
    Wperr1 = 1,
}
impl From<Wperrselect> for bool {
    #[inline(always)]
    fn from(variant: Wperrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPERR` reader - Write Protection Error Flag Bit"]
pub type WperrR = crate::BitReader<Wperrselect>;
impl WperrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wperrselect {
        match self.bits {
            false => Wperrselect::Wperr0,
            true => Wperrselect::Wperr1,
        }
    }
    #[doc = "No Write Protection Error"]
    #[inline(always)]
    pub fn is_wperr_0(&self) -> bool {
        *self == Wperrselect::Wperr0
    }
    #[doc = "Write Protection Error"]
    #[inline(always)]
    pub fn is_wperr_1(&self) -> bool {
        *self == Wperrselect::Wperr1
    }
}
#[doc = "Field `WPERR` writer - Write Protection Error Flag Bit"]
pub type WperrW<'a, REG> = crate::BitWriter1C<'a, REG, Wperrselect>;
impl<'a, REG> WperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Write Protection Error"]
    #[inline(always)]
    pub fn wperr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wperrselect::Wperr0)
    }
    #[doc = "Write Protection Error"]
    #[inline(always)]
    pub fn wperr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wperrselect::Wperr1)
    }
}
#[doc = "NVMOP Error Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operrselect {
    #[doc = "0: No NVMOP Error"]
    Operr0 = 0,
    #[doc = "1: Selected Operation is Disabled Error"]
    Operr1 = 1,
}
impl From<Operrselect> for bool {
    #[inline(always)]
    fn from(variant: Operrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPERR` reader - NVMOP Error Flag Bit"]
pub type OperrR = crate::BitReader<Operrselect>;
impl OperrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Operrselect {
        match self.bits {
            false => Operrselect::Operr0,
            true => Operrselect::Operr1,
        }
    }
    #[doc = "No NVMOP Error"]
    #[inline(always)]
    pub fn is_operr_0(&self) -> bool {
        *self == Operrselect::Operr0
    }
    #[doc = "Selected Operation is Disabled Error"]
    #[inline(always)]
    pub fn is_operr_1(&self) -> bool {
        *self == Operrselect::Operr1
    }
}
#[doc = "Field `OPERR` writer - NVMOP Error Flag Bit"]
pub type OperrW<'a, REG> = crate::BitWriter1C<'a, REG, Operrselect>;
impl<'a, REG> OperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No NVMOP Error"]
    #[inline(always)]
    pub fn operr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Operrselect::Operr0)
    }
    #[doc = "Selected Operation is Disabled Error"]
    #[inline(always)]
    pub fn operr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Operrselect::Operr1)
    }
}
#[doc = "High Temperature Detect Error Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htdpgmselect {
    #[doc = "0: High Temp NOT Detected"]
    Htdpgm0 = 0,
    #[doc = "1: High Temp Detected (possible data corruption, verify operation)"]
    Htdpgm1 = 1,
}
impl From<Htdpgmselect> for bool {
    #[inline(always)]
    fn from(variant: Htdpgmselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTDPGM` reader - High Temperature Detect Error Flag Bit"]
pub type HtdpgmR = crate::BitReader<Htdpgmselect>;
impl HtdpgmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Htdpgmselect {
        match self.bits {
            false => Htdpgmselect::Htdpgm0,
            true => Htdpgmselect::Htdpgm1,
        }
    }
    #[doc = "High Temp NOT Detected"]
    #[inline(always)]
    pub fn is_htdpgm_0(&self) -> bool {
        *self == Htdpgmselect::Htdpgm0
    }
    #[doc = "High Temp Detected (possible data corruption, verify operation)"]
    #[inline(always)]
    pub fn is_htdpgm_1(&self) -> bool {
        *self == Htdpgmselect::Htdpgm1
    }
}
#[doc = "Field `HTDPGM` writer - High Temperature Detect Error Flag Bit"]
pub type HtdpgmW<'a, REG> = crate::BitWriter1C<'a, REG, Htdpgmselect>;
impl<'a, REG> HtdpgmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High Temp NOT Detected"]
    #[inline(always)]
    pub fn htdpgm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Htdpgmselect::Htdpgm0)
    }
    #[doc = "High Temp Detected (possible data corruption, verify operation)"]
    #[inline(always)]
    pub fn htdpgm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Htdpgmselect::Htdpgm1)
    }
}
#[doc = "Reset or Brown Out Detect Error Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsterrselect {
    #[doc = "0: No Reset and Voltage level OK during write/erase"]
    Rsterr0 = 0,
    #[doc = "1: A reset or Low Voltage Detected (possible data corruption, verify data)"]
    Rsterr1 = 1,
}
impl From<Rsterrselect> for bool {
    #[inline(always)]
    fn from(variant: Rsterrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTERR` reader - Reset or Brown Out Detect Error Flag Bit"]
pub type RsterrR = crate::BitReader<Rsterrselect>;
impl RsterrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsterrselect {
        match self.bits {
            false => Rsterrselect::Rsterr0,
            true => Rsterrselect::Rsterr1,
        }
    }
    #[doc = "No Reset and Voltage level OK during write/erase"]
    #[inline(always)]
    pub fn is_rsterr_0(&self) -> bool {
        *self == Rsterrselect::Rsterr0
    }
    #[doc = "A reset or Low Voltage Detected (possible data corruption, verify data)"]
    #[inline(always)]
    pub fn is_rsterr_1(&self) -> bool {
        *self == Rsterrselect::Rsterr1
    }
}
#[doc = "Field `RSTERR` writer - Reset or Brown Out Detect Error Flag Bit"]
pub type RsterrW<'a, REG> = crate::BitWriter1C<'a, REG, Rsterrselect>;
impl<'a, REG> RsterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Reset and Voltage level OK during write/erase"]
    #[inline(always)]
    pub fn rsterr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsterrselect::Rsterr0)
    }
    #[doc = "A reset or Low Voltage Detected (possible data corruption, verify data)"]
    #[inline(always)]
    pub fn rsterr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsterrselect::Rsterr1)
    }
}
#[doc = "Write Error Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrerrselect {
    #[doc = "0: The Write/Erase sequence completed normally"]
    Wrerr0 = 0,
    #[doc = "1: The Write/Erase sequence did not complete successfully"]
    Wrerr1 = 1,
}
impl From<Wrerrselect> for bool {
    #[inline(always)]
    fn from(variant: Wrerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRERR` reader - Write Error Flag Bit"]
pub type WrerrR = crate::BitReader<Wrerrselect>;
impl WrerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrerrselect {
        match self.bits {
            false => Wrerrselect::Wrerr0,
            true => Wrerrselect::Wrerr1,
        }
    }
    #[doc = "The Write/Erase sequence completed normally"]
    #[inline(always)]
    pub fn is_wrerr_0(&self) -> bool {
        *self == Wrerrselect::Wrerr0
    }
    #[doc = "The Write/Erase sequence did not complete successfully"]
    #[inline(always)]
    pub fn is_wrerr_1(&self) -> bool {
        *self == Wrerrselect::Wrerr1
    }
}
#[doc = "Field `WRERR` writer - Write Error Flag Bit"]
pub type WrerrW<'a, REG> = crate::BitWriter1C<'a, REG, Wrerrselect>;
impl<'a, REG> WrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Write/Erase sequence completed normally"]
    #[inline(always)]
    pub fn wrerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wrerrselect::Wrerr0)
    }
    #[doc = "The Write/Erase sequence did not complete successfully"]
    #[inline(always)]
    pub fn wrerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wrerrselect::Wrerr1)
    }
}
impl R {
    #[doc = "Bit 0 - NVM Operation Done Flag Bit"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key Error Flag Bit"]
    #[inline(always)]
    pub fn keyerr(&self) -> KeyerrR {
        KeyerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configuration Error Flag Bit"]
    #[inline(always)]
    pub fn cfgerr(&self) -> CfgerrR {
        CfgerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Underrun during Row Write Flag Bit"]
    #[inline(always)]
    pub fn fifoerr(&self) -> FifoerrR {
        FifoerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB Bus Error during Row Write Flag Bit"]
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protection Error Flag Bit"]
    #[inline(always)]
    pub fn wperr(&self) -> WperrR {
        WperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NVMOP Error Flag Bit"]
    #[inline(always)]
    pub fn operr(&self) -> OperrR {
        OperrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - High Temperature Detect Error Flag Bit"]
    #[inline(always)]
    pub fn htdpgm(&self) -> HtdpgmR {
        HtdpgmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset or Brown Out Detect Error Flag Bit"]
    #[inline(always)]
    pub fn rsterr(&self) -> RsterrR {
        RsterrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write Error Flag Bit"]
    #[inline(always)]
    pub fn wrerr(&self) -> WrerrR {
        WrerrR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVM Operation Done Flag Bit"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntflagSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Key Error Flag Bit"]
    #[inline(always)]
    pub fn keyerr(&mut self) -> KeyerrW<'_, IntflagSpec> {
        KeyerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Configuration Error Flag Bit"]
    #[inline(always)]
    pub fn cfgerr(&mut self) -> CfgerrW<'_, IntflagSpec> {
        CfgerrW::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO Underrun during Row Write Flag Bit"]
    #[inline(always)]
    pub fn fifoerr(&mut self) -> FifoerrW<'_, IntflagSpec> {
        FifoerrW::new(self, 3)
    }
    #[doc = "Bit 4 - AHB Bus Error during Row Write Flag Bit"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BuserrW<'_, IntflagSpec> {
        BuserrW::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protection Error Flag Bit"]
    #[inline(always)]
    pub fn wperr(&mut self) -> WperrW<'_, IntflagSpec> {
        WperrW::new(self, 5)
    }
    #[doc = "Bit 6 - NVMOP Error Flag Bit"]
    #[inline(always)]
    pub fn operr(&mut self) -> OperrW<'_, IntflagSpec> {
        OperrW::new(self, 6)
    }
    #[doc = "Bit 8 - High Temperature Detect Error Flag Bit"]
    #[inline(always)]
    pub fn htdpgm(&mut self) -> HtdpgmW<'_, IntflagSpec> {
        HtdpgmW::new(self, 8)
    }
    #[doc = "Bit 12 - Reset or Brown Out Detect Error Flag Bit"]
    #[inline(always)]
    pub fn rsterr(&mut self) -> RsterrW<'_, IntflagSpec> {
        RsterrW::new(self, 12)
    }
    #[doc = "Bit 13 - Write Error Flag Bit"]
    #[inline(always)]
    pub fn wrerr(&mut self) -> WrerrW<'_, IntflagSpec> {
        WrerrW::new(self, 13)
    }
}
#[doc = "Interrupt Flag REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x317f;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}

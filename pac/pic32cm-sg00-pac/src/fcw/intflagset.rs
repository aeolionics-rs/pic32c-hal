#[doc = "Register `INTFLAGSET` reader"]
pub type R = crate::R<IntflagsetSpec>;
#[doc = "Register `INTFLAGSET` writer"]
pub type W = crate::W<IntflagsetSpec>;
#[doc = "NVM Operation Done Set Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doneselect {
    #[doc = "0: No effect"]
    Done0 = 0,
    #[doc = "1: Set Interrupt Pending"]
    Done1 = 1,
}
impl From<Doneselect> for bool {
    #[inline(always)]
    fn from(variant: Doneselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - NVM Operation Done Set Flag Bit"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_done_0(&self) -> bool {
        *self == Doneselect::Done0
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn is_done_1(&self) -> bool {
        *self == Doneselect::Done1
    }
}
#[doc = "Field `DONE` writer - NVM Operation Done Set Flag Bit"]
pub type DoneW<'a, REG> = crate::BitWriter1C<'a, REG, Doneselect>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn done_0(self) -> &'a mut crate::W<REG> {
        self.variant(Doneselect::Done0)
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn done_1(self) -> &'a mut crate::W<REG> {
        self.variant(Doneselect::Done1)
    }
}
#[doc = "Key Error Set Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyerrselect {
    #[doc = "0: No effect"]
    Keyerr0 = 0,
    #[doc = "1: Set Interrupt Pending"]
    Keyerr1 = 1,
}
impl From<Keyerrselect> for bool {
    #[inline(always)]
    fn from(variant: Keyerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYERR` reader - Key Error Set Flag Bit"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_keyerr_0(&self) -> bool {
        *self == Keyerrselect::Keyerr0
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn is_keyerr_1(&self) -> bool {
        *self == Keyerrselect::Keyerr1
    }
}
#[doc = "Field `KEYERR` writer - Key Error Set Flag Bit"]
pub type KeyerrW<'a, REG> = crate::BitWriter1C<'a, REG, Keyerrselect>;
impl<'a, REG> KeyerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn keyerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Keyerrselect::Keyerr0)
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn keyerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Keyerrselect::Keyerr1)
    }
}
#[doc = "Configuration Error Set Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfgerrselect {
    #[doc = "0: No effect"]
    Cfgerr0 = 0,
    #[doc = "1: Set Interrupt Pending"]
    Cfgerr1 = 1,
}
impl From<Cfgerrselect> for bool {
    #[inline(always)]
    fn from(variant: Cfgerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGERR` reader - Configuration Error Set Flag Bit"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_cfgerr_0(&self) -> bool {
        *self == Cfgerrselect::Cfgerr0
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn is_cfgerr_1(&self) -> bool {
        *self == Cfgerrselect::Cfgerr1
    }
}
#[doc = "Field `CFGERR` writer - Configuration Error Set Flag Bit"]
pub type CfgerrW<'a, REG> = crate::BitWriter1C<'a, REG, Cfgerrselect>;
impl<'a, REG> CfgerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn cfgerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgerrselect::Cfgerr0)
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn cfgerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgerrselect::Cfgerr1)
    }
}
#[doc = "FIFO Underrun during Row Write Set Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoerrselect {
    #[doc = "0: No effect"]
    Fifoerr0 = 0,
    #[doc = "1: Set Interrupt Pending"]
    Fifoerr1 = 1,
}
impl From<Fifoerrselect> for bool {
    #[inline(always)]
    fn from(variant: Fifoerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOERR` reader - FIFO Underrun during Row Write Set Flag Bit"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_fifoerr_0(&self) -> bool {
        *self == Fifoerrselect::Fifoerr0
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn is_fifoerr_1(&self) -> bool {
        *self == Fifoerrselect::Fifoerr1
    }
}
#[doc = "Field `FIFOERR` writer - FIFO Underrun during Row Write Set Flag Bit"]
pub type FifoerrW<'a, REG> = crate::BitWriter1C<'a, REG, Fifoerrselect>;
impl<'a, REG> FifoerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn fifoerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoerrselect::Fifoerr0)
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn fifoerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoerrselect::Fifoerr1)
    }
}
#[doc = "AHB Bus Error during Row Write Set Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Buserrselect {
    #[doc = "0: No effect"]
    Buserr0 = 0,
    #[doc = "1: Set Interrupt Pending"]
    Buserr1 = 1,
}
impl From<Buserrselect> for bool {
    #[inline(always)]
    fn from(variant: Buserrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSERR` reader - AHB Bus Error during Row Write Set Flag Bit"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_buserr_0(&self) -> bool {
        *self == Buserrselect::Buserr0
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn is_buserr_1(&self) -> bool {
        *self == Buserrselect::Buserr1
    }
}
#[doc = "Field `BUSERR` writer - AHB Bus Error during Row Write Set Flag Bit"]
pub type BuserrW<'a, REG> = crate::BitWriter1C<'a, REG, Buserrselect>;
impl<'a, REG> BuserrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn buserr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Buserrselect::Buserr0)
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn buserr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Buserrselect::Buserr1)
    }
}
#[doc = "Write Protection Error Set Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wperrselect {
    #[doc = "0: No effect"]
    Wperr0 = 0,
    #[doc = "1: Set Interrupt Pending"]
    Wperr1 = 1,
}
impl From<Wperrselect> for bool {
    #[inline(always)]
    fn from(variant: Wperrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPERR` reader - Write Protection Error Set Flag Bit"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_wperr_0(&self) -> bool {
        *self == Wperrselect::Wperr0
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn is_wperr_1(&self) -> bool {
        *self == Wperrselect::Wperr1
    }
}
#[doc = "Field `WPERR` writer - Write Protection Error Set Flag Bit"]
pub type WperrW<'a, REG> = crate::BitWriter1C<'a, REG, Wperrselect>;
impl<'a, REG> WperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn wperr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wperrselect::Wperr0)
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn wperr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wperrselect::Wperr1)
    }
}
#[doc = "NVMOP Error Set Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operrselect {
    #[doc = "0: No effect"]
    Operr0 = 0,
    #[doc = "1: Set Interrupt Pending"]
    Operr1 = 1,
}
impl From<Operrselect> for bool {
    #[inline(always)]
    fn from(variant: Operrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPERR` reader - NVMOP Error Set Flag Bit"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_operr_0(&self) -> bool {
        *self == Operrselect::Operr0
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn is_operr_1(&self) -> bool {
        *self == Operrselect::Operr1
    }
}
#[doc = "Field `OPERR` writer - NVMOP Error Set Flag Bit"]
pub type OperrW<'a, REG> = crate::BitWriter1C<'a, REG, Operrselect>;
impl<'a, REG> OperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn operr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Operrselect::Operr0)
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn operr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Operrselect::Operr1)
    }
}
#[doc = "High Temperature Detect Error Set Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htdpgmselect {
    #[doc = "0: No effect"]
    Htdpgm0 = 0,
    #[doc = "1: Set Interrupt Pending"]
    Htdpgm1 = 1,
}
impl From<Htdpgmselect> for bool {
    #[inline(always)]
    fn from(variant: Htdpgmselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTDPGM` reader - High Temperature Detect Error Set Flag Bit"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_htdpgm_0(&self) -> bool {
        *self == Htdpgmselect::Htdpgm0
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn is_htdpgm_1(&self) -> bool {
        *self == Htdpgmselect::Htdpgm1
    }
}
#[doc = "Field `HTDPGM` writer - High Temperature Detect Error Set Flag Bit"]
pub type HtdpgmW<'a, REG> = crate::BitWriter1C<'a, REG, Htdpgmselect>;
impl<'a, REG> HtdpgmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn htdpgm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Htdpgmselect::Htdpgm0)
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn htdpgm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Htdpgmselect::Htdpgm1)
    }
}
#[doc = "Reset or Brown Out Detect Error Set Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsterrselect {
    #[doc = "0: No effect"]
    Rsterr0 = 0,
    #[doc = "1: Set Interrupt Pending"]
    Rsterr1 = 1,
}
impl From<Rsterrselect> for bool {
    #[inline(always)]
    fn from(variant: Rsterrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTERR` reader - Reset or Brown Out Detect Error Set Flag Bit"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_rsterr_0(&self) -> bool {
        *self == Rsterrselect::Rsterr0
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn is_rsterr_1(&self) -> bool {
        *self == Rsterrselect::Rsterr1
    }
}
#[doc = "Field `RSTERR` writer - Reset or Brown Out Detect Error Set Flag Bit"]
pub type RsterrW<'a, REG> = crate::BitWriter1C<'a, REG, Rsterrselect>;
impl<'a, REG> RsterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn rsterr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsterrselect::Rsterr0)
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn rsterr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsterrselect::Rsterr1)
    }
}
#[doc = "Write Error Set Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrerrselect {
    #[doc = "0: No effect"]
    Wrerr0 = 0,
    #[doc = "1: Set Interrupt Pending"]
    Wrerr1 = 1,
}
impl From<Wrerrselect> for bool {
    #[inline(always)]
    fn from(variant: Wrerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRERR` reader - Write Error Set Flag Bit"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_wrerr_0(&self) -> bool {
        *self == Wrerrselect::Wrerr0
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn is_wrerr_1(&self) -> bool {
        *self == Wrerrselect::Wrerr1
    }
}
#[doc = "Field `WRERR` writer - Write Error Set Flag Bit"]
pub type WrerrW<'a, REG> = crate::BitWriter1C<'a, REG, Wrerrselect>;
impl<'a, REG> WrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn wrerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wrerrselect::Wrerr0)
    }
    #[doc = "Set Interrupt Pending"]
    #[inline(always)]
    pub fn wrerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wrerrselect::Wrerr1)
    }
}
impl R {
    #[doc = "Bit 0 - NVM Operation Done Set Flag Bit"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key Error Set Flag Bit"]
    #[inline(always)]
    pub fn keyerr(&self) -> KeyerrR {
        KeyerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configuration Error Set Flag Bit"]
    #[inline(always)]
    pub fn cfgerr(&self) -> CfgerrR {
        CfgerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Underrun during Row Write Set Flag Bit"]
    #[inline(always)]
    pub fn fifoerr(&self) -> FifoerrR {
        FifoerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB Bus Error during Row Write Set Flag Bit"]
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protection Error Set Flag Bit"]
    #[inline(always)]
    pub fn wperr(&self) -> WperrR {
        WperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NVMOP Error Set Flag Bit"]
    #[inline(always)]
    pub fn operr(&self) -> OperrR {
        OperrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - High Temperature Detect Error Set Flag Bit"]
    #[inline(always)]
    pub fn htdpgm(&self) -> HtdpgmR {
        HtdpgmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset or Brown Out Detect Error Set Flag Bit"]
    #[inline(always)]
    pub fn rsterr(&self) -> RsterrR {
        RsterrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write Error Set Flag Bit"]
    #[inline(always)]
    pub fn wrerr(&self) -> WrerrR {
        WrerrR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVM Operation Done Set Flag Bit"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntflagsetSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Key Error Set Flag Bit"]
    #[inline(always)]
    pub fn keyerr(&mut self) -> KeyerrW<'_, IntflagsetSpec> {
        KeyerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Configuration Error Set Flag Bit"]
    #[inline(always)]
    pub fn cfgerr(&mut self) -> CfgerrW<'_, IntflagsetSpec> {
        CfgerrW::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO Underrun during Row Write Set Flag Bit"]
    #[inline(always)]
    pub fn fifoerr(&mut self) -> FifoerrW<'_, IntflagsetSpec> {
        FifoerrW::new(self, 3)
    }
    #[doc = "Bit 4 - AHB Bus Error during Row Write Set Flag Bit"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BuserrW<'_, IntflagsetSpec> {
        BuserrW::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protection Error Set Flag Bit"]
    #[inline(always)]
    pub fn wperr(&mut self) -> WperrW<'_, IntflagsetSpec> {
        WperrW::new(self, 5)
    }
    #[doc = "Bit 6 - NVMOP Error Set Flag Bit"]
    #[inline(always)]
    pub fn operr(&mut self) -> OperrW<'_, IntflagsetSpec> {
        OperrW::new(self, 6)
    }
    #[doc = "Bit 8 - High Temperature Detect Error Set Flag Bit"]
    #[inline(always)]
    pub fn htdpgm(&mut self) -> HtdpgmW<'_, IntflagsetSpec> {
        HtdpgmW::new(self, 8)
    }
    #[doc = "Bit 12 - Reset or Brown Out Detect Error Set Flag Bit"]
    #[inline(always)]
    pub fn rsterr(&mut self) -> RsterrW<'_, IntflagsetSpec> {
        RsterrW::new(self, 12)
    }
    #[doc = "Bit 13 - Write Error Set Flag Bit"]
    #[inline(always)]
    pub fn wrerr(&mut self) -> WrerrW<'_, IntflagsetSpec> {
        WrerrW::new(self, 13)
    }
}
#[doc = "Interrupt Flag Set REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagsetSpec;
impl crate::RegisterSpec for IntflagsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagset::R`](R) reader structure"]
impl crate::Readable for IntflagsetSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagset::W`](W) writer structure"]
impl crate::Writable for IntflagsetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x317f;
}
#[doc = "`reset()` method sets INTFLAGSET to value 0"]
impl crate::Resettable for IntflagsetSpec {}

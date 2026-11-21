#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "NVM Operation Done Interrupt Set Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doneselect {
    #[doc = "0: Interrupt Disabled"]
    Done0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Done1 = 1,
}
impl From<Doneselect> for bool {
    #[inline(always)]
    fn from(variant: Doneselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - NVM Operation Done Interrupt Set Enable"]
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
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_done_0(&self) -> bool {
        *self == Doneselect::Done0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_done_1(&self) -> bool {
        *self == Doneselect::Done1
    }
}
#[doc = "Field `DONE` writer - NVM Operation Done Interrupt Set Enable"]
pub type DoneW<'a, REG> = crate::BitWriter1S<'a, REG, Doneselect>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn done_0(self) -> &'a mut crate::W<REG> {
        self.variant(Doneselect::Done0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn done_1(self) -> &'a mut crate::W<REG> {
        self.variant(Doneselect::Done1)
    }
}
#[doc = "Key Error Interrupt Set Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyerrselect {
    #[doc = "0: Interrupt Disabled"]
    Keyerr0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Keyerr1 = 1,
}
impl From<Keyerrselect> for bool {
    #[inline(always)]
    fn from(variant: Keyerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYERR` reader - Key Error Interrupt Set Enable"]
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
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_keyerr_0(&self) -> bool {
        *self == Keyerrselect::Keyerr0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_keyerr_1(&self) -> bool {
        *self == Keyerrselect::Keyerr1
    }
}
#[doc = "Field `KEYERR` writer - Key Error Interrupt Set Enable"]
pub type KeyerrW<'a, REG> = crate::BitWriter1S<'a, REG, Keyerrselect>;
impl<'a, REG> KeyerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn keyerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Keyerrselect::Keyerr0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn keyerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Keyerrselect::Keyerr1)
    }
}
#[doc = "Configuration Error Interrupt Set Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfgerrselect {
    #[doc = "0: Interrupt Disabled"]
    Cfgerr0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Cfgerr1 = 1,
}
impl From<Cfgerrselect> for bool {
    #[inline(always)]
    fn from(variant: Cfgerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGERR` reader - Configuration Error Interrupt Set Enable"]
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
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_cfgerr_0(&self) -> bool {
        *self == Cfgerrselect::Cfgerr0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_cfgerr_1(&self) -> bool {
        *self == Cfgerrselect::Cfgerr1
    }
}
#[doc = "Field `CFGERR` writer - Configuration Error Interrupt Set Enable"]
pub type CfgerrW<'a, REG> = crate::BitWriter1S<'a, REG, Cfgerrselect>;
impl<'a, REG> CfgerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn cfgerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgerrselect::Cfgerr0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn cfgerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgerrselect::Cfgerr1)
    }
}
#[doc = "FIFO Underrun during Row Write Interrupt Set Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoerrselect {
    #[doc = "0: Interrupt Disabled"]
    Fifoerr0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Fifoerr1 = 1,
}
impl From<Fifoerrselect> for bool {
    #[inline(always)]
    fn from(variant: Fifoerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOERR` reader - FIFO Underrun during Row Write Interrupt Set Enable"]
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
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_fifoerr_0(&self) -> bool {
        *self == Fifoerrselect::Fifoerr0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_fifoerr_1(&self) -> bool {
        *self == Fifoerrselect::Fifoerr1
    }
}
#[doc = "Field `FIFOERR` writer - FIFO Underrun during Row Write Interrupt Set Enable"]
pub type FifoerrW<'a, REG> = crate::BitWriter1S<'a, REG, Fifoerrselect>;
impl<'a, REG> FifoerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn fifoerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoerrselect::Fifoerr0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn fifoerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoerrselect::Fifoerr1)
    }
}
#[doc = "AHB Bus Error during Row Write Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Buserrselect {
    #[doc = "0: Interrupt Disabled"]
    Buserr0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Buserr1 = 1,
}
impl From<Buserrselect> for bool {
    #[inline(always)]
    fn from(variant: Buserrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSERR` reader - AHB Bus Error during Row Write Interrupt Enable"]
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
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_buserr_0(&self) -> bool {
        *self == Buserrselect::Buserr0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_buserr_1(&self) -> bool {
        *self == Buserrselect::Buserr1
    }
}
#[doc = "Field `BUSERR` writer - AHB Bus Error during Row Write Interrupt Enable"]
pub type BuserrW<'a, REG> = crate::BitWriter1S<'a, REG, Buserrselect>;
impl<'a, REG> BuserrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn buserr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Buserrselect::Buserr0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn buserr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Buserrselect::Buserr1)
    }
}
#[doc = "Write Protection Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wperrselect {
    #[doc = "0: Interrupt Disabled"]
    Wperr0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Wperr1 = 1,
}
impl From<Wperrselect> for bool {
    #[inline(always)]
    fn from(variant: Wperrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPERR` reader - Write Protection Error Interrupt Enable"]
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
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_wperr_0(&self) -> bool {
        *self == Wperrselect::Wperr0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_wperr_1(&self) -> bool {
        *self == Wperrselect::Wperr1
    }
}
#[doc = "Field `WPERR` writer - Write Protection Error Interrupt Enable"]
pub type WperrW<'a, REG> = crate::BitWriter1S<'a, REG, Wperrselect>;
impl<'a, REG> WperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn wperr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wperrselect::Wperr0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn wperr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wperrselect::Wperr1)
    }
}
#[doc = "NVMOP Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operrselect {
    #[doc = "0: Interrupt Disabled"]
    Operr0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Operr1 = 1,
}
impl From<Operrselect> for bool {
    #[inline(always)]
    fn from(variant: Operrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPERR` reader - NVMOP Error Interrupt Enable"]
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
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_operr_0(&self) -> bool {
        *self == Operrselect::Operr0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_operr_1(&self) -> bool {
        *self == Operrselect::Operr1
    }
}
#[doc = "Field `OPERR` writer - NVMOP Error Interrupt Enable"]
pub type OperrW<'a, REG> = crate::BitWriter1S<'a, REG, Operrselect>;
impl<'a, REG> OperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn operr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Operrselect::Operr0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn operr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Operrselect::Operr1)
    }
}
#[doc = "High Temperature Detect Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htdpgmselect {
    #[doc = "0: Interrupt Disabled"]
    Htdpgm0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Htdpgm1 = 1,
}
impl From<Htdpgmselect> for bool {
    #[inline(always)]
    fn from(variant: Htdpgmselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTDPGM` reader - High Temperature Detect Error Interrupt Enable"]
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
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_htdpgm_0(&self) -> bool {
        *self == Htdpgmselect::Htdpgm0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_htdpgm_1(&self) -> bool {
        *self == Htdpgmselect::Htdpgm1
    }
}
#[doc = "Field `HTDPGM` writer - High Temperature Detect Error Interrupt Enable"]
pub type HtdpgmW<'a, REG> = crate::BitWriter1S<'a, REG, Htdpgmselect>;
impl<'a, REG> HtdpgmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn htdpgm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Htdpgmselect::Htdpgm0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn htdpgm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Htdpgmselect::Htdpgm1)
    }
}
#[doc = "Reset or Brown Out Detect Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsterrselect {
    #[doc = "0: Interrupt Disabled"]
    Rsterr0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Rsterr1 = 1,
}
impl From<Rsterrselect> for bool {
    #[inline(always)]
    fn from(variant: Rsterrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTERR` reader - Reset or Brown Out Detect Error Interrupt Enable"]
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
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_rsterr_0(&self) -> bool {
        *self == Rsterrselect::Rsterr0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_rsterr_1(&self) -> bool {
        *self == Rsterrselect::Rsterr1
    }
}
#[doc = "Field `RSTERR` writer - Reset or Brown Out Detect Error Interrupt Enable"]
pub type RsterrW<'a, REG> = crate::BitWriter1S<'a, REG, Rsterrselect>;
impl<'a, REG> RsterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn rsterr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsterrselect::Rsterr0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn rsterr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsterrselect::Rsterr1)
    }
}
#[doc = "Write Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrerrselect {
    #[doc = "0: Interrupt Disabled"]
    Wrerr0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Wrerr1 = 1,
}
impl From<Wrerrselect> for bool {
    #[inline(always)]
    fn from(variant: Wrerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRERR` reader - Write Error Interrupt Enable"]
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
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_wrerr_0(&self) -> bool {
        *self == Wrerrselect::Wrerr0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_wrerr_1(&self) -> bool {
        *self == Wrerrselect::Wrerr1
    }
}
#[doc = "Field `WRERR` writer - Write Error Interrupt Enable"]
pub type WrerrW<'a, REG> = crate::BitWriter1S<'a, REG, Wrerrselect>;
impl<'a, REG> WrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn wrerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wrerrselect::Wrerr0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn wrerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wrerrselect::Wrerr1)
    }
}
impl R {
    #[doc = "Bit 0 - NVM Operation Done Interrupt Set Enable"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key Error Interrupt Set Enable"]
    #[inline(always)]
    pub fn keyerr(&self) -> KeyerrR {
        KeyerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configuration Error Interrupt Set Enable"]
    #[inline(always)]
    pub fn cfgerr(&self) -> CfgerrR {
        CfgerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Underrun during Row Write Interrupt Set Enable"]
    #[inline(always)]
    pub fn fifoerr(&self) -> FifoerrR {
        FifoerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB Bus Error during Row Write Interrupt Enable"]
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protection Error Interrupt Enable"]
    #[inline(always)]
    pub fn wperr(&self) -> WperrR {
        WperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NVMOP Error Interrupt Enable"]
    #[inline(always)]
    pub fn operr(&self) -> OperrR {
        OperrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - High Temperature Detect Error Interrupt Enable"]
    #[inline(always)]
    pub fn htdpgm(&self) -> HtdpgmR {
        HtdpgmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset or Brown Out Detect Error Interrupt Enable"]
    #[inline(always)]
    pub fn rsterr(&self) -> RsterrR {
        RsterrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn wrerr(&self) -> WrerrR {
        WrerrR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVM Operation Done Interrupt Set Enable"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntensetSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Key Error Interrupt Set Enable"]
    #[inline(always)]
    pub fn keyerr(&mut self) -> KeyerrW<'_, IntensetSpec> {
        KeyerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Configuration Error Interrupt Set Enable"]
    #[inline(always)]
    pub fn cfgerr(&mut self) -> CfgerrW<'_, IntensetSpec> {
        CfgerrW::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO Underrun during Row Write Interrupt Set Enable"]
    #[inline(always)]
    pub fn fifoerr(&mut self) -> FifoerrW<'_, IntensetSpec> {
        FifoerrW::new(self, 3)
    }
    #[doc = "Bit 4 - AHB Bus Error during Row Write Interrupt Enable"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BuserrW<'_, IntensetSpec> {
        BuserrW::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protection Error Interrupt Enable"]
    #[inline(always)]
    pub fn wperr(&mut self) -> WperrW<'_, IntensetSpec> {
        WperrW::new(self, 5)
    }
    #[doc = "Bit 6 - NVMOP Error Interrupt Enable"]
    #[inline(always)]
    pub fn operr(&mut self) -> OperrW<'_, IntensetSpec> {
        OperrW::new(self, 6)
    }
    #[doc = "Bit 8 - High Temperature Detect Error Interrupt Enable"]
    #[inline(always)]
    pub fn htdpgm(&mut self) -> HtdpgmW<'_, IntensetSpec> {
        HtdpgmW::new(self, 8)
    }
    #[doc = "Bit 12 - Reset or Brown Out Detect Error Interrupt Enable"]
    #[inline(always)]
    pub fn rsterr(&mut self) -> RsterrW<'_, IntensetSpec> {
        RsterrW::new(self, 12)
    }
    #[doc = "Bit 13 - Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn wrerr(&mut self) -> WrerrW<'_, IntensetSpec> {
        WrerrW::new(self, 13)
    }
}
#[doc = "Interrupt Enable Set REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x317f;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}

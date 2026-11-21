#[doc = "Register `CRP` reader"]
pub type R = crate::R<CrpSpec>;
#[doc = "Register `CRP` writer"]
pub type W = crate::W<CrpSpec>;
#[doc = "Panel 1 Read Protect BootCfg1A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bc1arpselect {
    #[doc = "0: Read Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Read Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Bc1arpselect> for bool {
    #[inline(always)]
    fn from(variant: Bc1arpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC1ARP` reader - Panel 1 Read Protect BootCfg1A"]
pub type Bc1arpR = crate::BitReader<Bc1arpselect>;
impl Bc1arpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bc1arpselect {
        match self.bits {
            false => Bc1arpselect::Disable,
            true => Bc1arpselect::Enable,
        }
    }
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bc1arpselect::Disable
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bc1arpselect::Enable
    }
}
#[doc = "Field `BC1ARP` writer - Panel 1 Read Protect BootCfg1A"]
pub type Bc1arpW<'a, REG> = crate::BitWriter<'a, REG, Bc1arpselect>;
impl<'a, REG> Bc1arpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1arpselect::Disable)
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1arpselect::Enable)
    }
}
#[doc = "Panel 1 Read Protect UserOTP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uo1rpselect {
    #[doc = "0: Read Protection for this Page is Disabled"]
    Disable = 0,
}
impl From<Uo1rpselect> for bool {
    #[inline(always)]
    fn from(variant: Uo1rpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UO1RP` reader - Panel 1 Read Protect UserOTP1"]
pub type Uo1rpR = crate::BitReader<Uo1rpselect>;
impl Uo1rpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uo1rpselect> {
        match self.bits {
            false => Some(Uo1rpselect::Disable),
            _ => None,
        }
    }
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Uo1rpselect::Disable
    }
}
#[doc = "Field `UO1RP` writer - Panel 1 Read Protect UserOTP1"]
pub type Uo1rpW<'a, REG> = crate::BitWriter<'a, REG, Uo1rpselect>;
impl<'a, REG> Uo1rpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Uo1rpselect::Disable)
    }
}
#[doc = "Panel 1 Read Protect BootCfg1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bc1rpselect {
    #[doc = "0: Read Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Read Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Bc1rpselect> for bool {
    #[inline(always)]
    fn from(variant: Bc1rpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC1RP` reader - Panel 1 Read Protect BootCfg1"]
pub type Bc1rpR = crate::BitReader<Bc1rpselect>;
impl Bc1rpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bc1rpselect {
        match self.bits {
            false => Bc1rpselect::Disable,
            true => Bc1rpselect::Enable,
        }
    }
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bc1rpselect::Disable
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bc1rpselect::Enable
    }
}
#[doc = "Field `BC1RP` writer - Panel 1 Read Protect BootCfg1"]
pub type Bc1rpW<'a, REG> = crate::BitWriter<'a, REG, Bc1rpselect>;
impl<'a, REG> Bc1rpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1rpselect::Disable)
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1rpselect::Enable)
    }
}
#[doc = "Panel 1 Read Protect ROMCfg\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcrpselect {
    #[doc = "0: Read Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Read Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Rcrpselect> for bool {
    #[inline(always)]
    fn from(variant: Rcrpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCRP` reader - Panel 1 Read Protect ROMCfg"]
pub type RcrpR = crate::BitReader<Rcrpselect>;
impl RcrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcrpselect {
        match self.bits {
            false => Rcrpselect::Disable,
            true => Rcrpselect::Enable,
        }
    }
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rcrpselect::Disable
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rcrpselect::Enable
    }
}
#[doc = "Field `RCRP` writer - Panel 1 Read Protect ROMCfg"]
pub type RcrpW<'a, REG> = crate::BitWriter<'a, REG, Rcrpselect>;
impl<'a, REG> RcrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrpselect::Disable)
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrpselect::Enable)
    }
}
#[doc = "Panel 1 Read Protect VSSn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vssrp0select {
    #[doc = "0: Read Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Read Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Vssrp0select> for bool {
    #[inline(always)]
    fn from(variant: Vssrp0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSSRP0` reader - Panel 1 Read Protect VSSn"]
pub type Vssrp0R = crate::BitReader<Vssrp0select>;
impl Vssrp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vssrp0select {
        match self.bits {
            false => Vssrp0select::Disable,
            true => Vssrp0select::Enable,
        }
    }
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Vssrp0select::Disable
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Vssrp0select::Enable
    }
}
#[doc = "Field `VSSRP0` writer - Panel 1 Read Protect VSSn"]
pub type Vssrp0W<'a, REG> = crate::BitWriter<'a, REG, Vssrp0select>;
impl<'a, REG> Vssrp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Vssrp0select::Disable)
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Vssrp0select::Enable)
    }
}
#[doc = "Panel 1 Read Protect VSSn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vssrp1select {
    #[doc = "0: Read Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Read Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Vssrp1select> for bool {
    #[inline(always)]
    fn from(variant: Vssrp1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSSRP1` reader - Panel 1 Read Protect VSSn"]
pub type Vssrp1R = crate::BitReader<Vssrp1select>;
impl Vssrp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vssrp1select {
        match self.bits {
            false => Vssrp1select::Disable,
            true => Vssrp1select::Enable,
        }
    }
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Vssrp1select::Disable
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Vssrp1select::Enable
    }
}
#[doc = "Field `VSSRP1` writer - Panel 1 Read Protect VSSn"]
pub type Vssrp1W<'a, REG> = crate::BitWriter<'a, REG, Vssrp1select>;
impl<'a, REG> Vssrp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Vssrp1select::Disable)
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Vssrp1select::Enable)
    }
}
#[doc = "Panel 1 Read Protect Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trpselect {
    #[doc = "0: Read Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Read Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Trpselect> for bool {
    #[inline(always)]
    fn from(variant: Trpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRP` reader - Panel 1 Read Protect Test"]
pub type TrpR = crate::BitReader<Trpselect>;
impl TrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trpselect {
        match self.bits {
            false => Trpselect::Disable,
            true => Trpselect::Enable,
        }
    }
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Trpselect::Disable
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Trpselect::Enable
    }
}
#[doc = "Field `TRP` writer - Panel 1 Read Protect Test"]
pub type TrpW<'a, REG> = crate::BitWriter<'a, REG, Trpselect>;
impl<'a, REG> TrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Trpselect::Disable)
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Trpselect::Enable)
    }
}
#[doc = "Panel 1 Read Protect CalOTP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corpselect {
    #[doc = "0: Read Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Read Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Corpselect> for bool {
    #[inline(always)]
    fn from(variant: Corpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORP` reader - Panel 1 Read Protect CalOTP"]
pub type CorpR = crate::BitReader<Corpselect>;
impl CorpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Corpselect {
        match self.bits {
            false => Corpselect::Disable,
            true => Corpselect::Enable,
        }
    }
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Corpselect::Disable
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Corpselect::Enable
    }
}
#[doc = "Field `CORP` writer - Panel 1 Read Protect CalOTP"]
pub type CorpW<'a, REG> = crate::BitWriter<'a, REG, Corpselect>;
impl<'a, REG> CorpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Corpselect::Disable)
    }
    #[doc = "Read Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Corpselect::Enable)
    }
}
#[doc = "Panel 1 Lock Read Protection BootCfg1A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bc1arplockselect {
    #[doc = "0: The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Bc1arplockselect> for bool {
    #[inline(always)]
    fn from(variant: Bc1arplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC1ARPLOCK` reader - Panel 1 Lock Read Protection BootCfg1A"]
pub type Bc1arplockR = crate::BitReader<Bc1arplockselect>;
impl Bc1arplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bc1arplockselect {
        match self.bits {
            false => Bc1arplockselect::Unlocked,
            true => Bc1arplockselect::Locked,
        }
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Bc1arplockselect::Unlocked
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Bc1arplockselect::Locked
    }
}
#[doc = "Field `BC1ARPLOCK` writer - Panel 1 Lock Read Protection BootCfg1A"]
pub type Bc1arplockW<'a, REG> = crate::BitWriter<'a, REG, Bc1arplockselect>;
impl<'a, REG> Bc1arplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1arplockselect::Unlocked)
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1arplockselect::Locked)
    }
}
#[doc = "Panel 1 Lock Read Protection UserOTP1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uo1rplockselect {
    #[doc = "1: The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Uo1rplockselect> for bool {
    #[inline(always)]
    fn from(variant: Uo1rplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UO1RPLOCK` reader - Panel 1 Lock Read Protection UserOTP1"]
pub type Uo1rplockR = crate::BitReader<Uo1rplockselect>;
impl Uo1rplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uo1rplockselect> {
        match self.bits {
            true => Some(Uo1rplockselect::Locked),
            _ => None,
        }
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Uo1rplockselect::Locked
    }
}
#[doc = "Field `UO1RPLOCK` writer - Panel 1 Lock Read Protection UserOTP1"]
pub type Uo1rplockW<'a, REG> = crate::BitWriter<'a, REG, Uo1rplockselect>;
impl<'a, REG> Uo1rplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Uo1rplockselect::Locked)
    }
}
#[doc = "Panel 1 Lock Read Protection BootCfg1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bc1rplockselect {
    #[doc = "0: The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Bc1rplockselect> for bool {
    #[inline(always)]
    fn from(variant: Bc1rplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC1RPLOCK` reader - Panel 1 Lock Read Protection BootCfg1"]
pub type Bc1rplockR = crate::BitReader<Bc1rplockselect>;
impl Bc1rplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bc1rplockselect {
        match self.bits {
            false => Bc1rplockselect::Unlocked,
            true => Bc1rplockselect::Locked,
        }
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Bc1rplockselect::Unlocked
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Bc1rplockselect::Locked
    }
}
#[doc = "Field `BC1RPLOCK` writer - Panel 1 Lock Read Protection BootCfg1"]
pub type Bc1rplockW<'a, REG> = crate::BitWriter<'a, REG, Bc1rplockselect>;
impl<'a, REG> Bc1rplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1rplockselect::Unlocked)
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1rplockselect::Locked)
    }
}
#[doc = "Panel 1 Lock Read Protection ROMCfg\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcrplockselect {
    #[doc = "0: The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Rcrplockselect> for bool {
    #[inline(always)]
    fn from(variant: Rcrplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCRPLOCK` reader - Panel 1 Lock Read Protection ROMCfg"]
pub type RcrplockR = crate::BitReader<Rcrplockselect>;
impl RcrplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcrplockselect {
        match self.bits {
            false => Rcrplockselect::Unlocked,
            true => Rcrplockselect::Locked,
        }
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Rcrplockselect::Unlocked
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Rcrplockselect::Locked
    }
}
#[doc = "Field `RCRPLOCK` writer - Panel 1 Lock Read Protection ROMCfg"]
pub type RcrplockW<'a, REG> = crate::BitWriter<'a, REG, Rcrplockselect>;
impl<'a, REG> RcrplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrplockselect::Unlocked)
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrplockselect::Locked)
    }
}
#[doc = "Panel 1 Lock Read Protection VSSn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vssrplock0select {
    #[doc = "0: The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Vssrplock0select> for bool {
    #[inline(always)]
    fn from(variant: Vssrplock0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSSRPLOCK0` reader - Panel 1 Lock Read Protection VSSn"]
pub type Vssrplock0R = crate::BitReader<Vssrplock0select>;
impl Vssrplock0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vssrplock0select {
        match self.bits {
            false => Vssrplock0select::Unlocked,
            true => Vssrplock0select::Locked,
        }
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Vssrplock0select::Unlocked
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Vssrplock0select::Locked
    }
}
#[doc = "Field `VSSRPLOCK0` writer - Panel 1 Lock Read Protection VSSn"]
pub type Vssrplock0W<'a, REG> = crate::BitWriter<'a, REG, Vssrplock0select>;
impl<'a, REG> Vssrplock0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Vssrplock0select::Unlocked)
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Vssrplock0select::Locked)
    }
}
#[doc = "Panel 1 Lock Read Protection VSSn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vssrplock1select {
    #[doc = "0: The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Vssrplock1select> for bool {
    #[inline(always)]
    fn from(variant: Vssrplock1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSSRPLOCK1` reader - Panel 1 Lock Read Protection VSSn"]
pub type Vssrplock1R = crate::BitReader<Vssrplock1select>;
impl Vssrplock1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vssrplock1select {
        match self.bits {
            false => Vssrplock1select::Unlocked,
            true => Vssrplock1select::Locked,
        }
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Vssrplock1select::Unlocked
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Vssrplock1select::Locked
    }
}
#[doc = "Field `VSSRPLOCK1` writer - Panel 1 Lock Read Protection VSSn"]
pub type Vssrplock1W<'a, REG> = crate::BitWriter<'a, REG, Vssrplock1select>;
impl<'a, REG> Vssrplock1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Read Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Vssrplock1select::Unlocked)
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Vssrplock1select::Locked)
    }
}
#[doc = "Panel 1 Lock Read Protection Test\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trplockselect {
    #[doc = "1: The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Trplockselect> for bool {
    #[inline(always)]
    fn from(variant: Trplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRPLOCK` reader - Panel 1 Lock Read Protection Test"]
pub type TrplockR = crate::BitReader<Trplockselect>;
impl TrplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trplockselect> {
        match self.bits {
            true => Some(Trplockselect::Locked),
            _ => None,
        }
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Trplockselect::Locked
    }
}
#[doc = "Field `TRPLOCK` writer - Panel 1 Lock Read Protection Test"]
pub type TrplockW<'a, REG> = crate::BitWriter<'a, REG, Trplockselect>;
impl<'a, REG> TrplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Trplockselect::Locked)
    }
}
#[doc = "Panel 1 Lock Read Protection CalOTP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corplockselect {
    #[doc = "1: The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Corplockselect> for bool {
    #[inline(always)]
    fn from(variant: Corplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORPLOCK` reader - Panel 1 Lock Read Protection CalOTP"]
pub type CorplockR = crate::BitReader<Corplockselect>;
impl CorplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Corplockselect> {
        match self.bits {
            true => Some(Corplockselect::Locked),
            _ => None,
        }
    }
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Corplockselect::Locked
    }
}
#[doc = "Field `CORPLOCK` writer - Panel 1 Lock Read Protection CalOTP"]
pub type CorplockW<'a, REG> = crate::BitWriter<'a, REG, Corplockselect>;
impl<'a, REG> CorplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Read Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Corplockselect::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Panel 1 Read Protect BootCfg1A"]
    #[inline(always)]
    pub fn bc1arp(&self) -> Bc1arpR {
        Bc1arpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Panel 1 Read Protect UserOTP1"]
    #[inline(always)]
    pub fn uo1rp(&self) -> Uo1rpR {
        Uo1rpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Panel 1 Read Protect BootCfg1"]
    #[inline(always)]
    pub fn bc1rp(&self) -> Bc1rpR {
        Bc1rpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Panel 1 Read Protect ROMCfg"]
    #[inline(always)]
    pub fn rcrp(&self) -> RcrpR {
        RcrpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Panel 1 Read Protect VSSn"]
    #[inline(always)]
    pub fn vssrp0(&self) -> Vssrp0R {
        Vssrp0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Panel 1 Read Protect VSSn"]
    #[inline(always)]
    pub fn vssrp1(&self) -> Vssrp1R {
        Vssrp1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Panel 1 Read Protect Test"]
    #[inline(always)]
    pub fn trp(&self) -> TrpR {
        TrpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Panel 1 Read Protect CalOTP"]
    #[inline(always)]
    pub fn corp(&self) -> CorpR {
        CorpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Panel 1 Lock Read Protection BootCfg1A"]
    #[inline(always)]
    pub fn bc1arplock(&self) -> Bc1arplockR {
        Bc1arplockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Panel 1 Lock Read Protection UserOTP1"]
    #[inline(always)]
    pub fn uo1rplock(&self) -> Uo1rplockR {
        Uo1rplockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Panel 1 Lock Read Protection BootCfg1"]
    #[inline(always)]
    pub fn bc1rplock(&self) -> Bc1rplockR {
        Bc1rplockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Panel 1 Lock Read Protection ROMCfg"]
    #[inline(always)]
    pub fn rcrplock(&self) -> RcrplockR {
        RcrplockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Panel 1 Lock Read Protection VSSn"]
    #[inline(always)]
    pub fn vssrplock0(&self) -> Vssrplock0R {
        Vssrplock0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Panel 1 Lock Read Protection VSSn"]
    #[inline(always)]
    pub fn vssrplock1(&self) -> Vssrplock1R {
        Vssrplock1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Panel 1 Lock Read Protection Test"]
    #[inline(always)]
    pub fn trplock(&self) -> TrplockR {
        TrplockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Panel 1 Lock Read Protection CalOTP"]
    #[inline(always)]
    pub fn corplock(&self) -> CorplockR {
        CorplockR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Panel 1 Read Protect BootCfg1A"]
    #[inline(always)]
    pub fn bc1arp(&mut self) -> Bc1arpW<'_, CrpSpec> {
        Bc1arpW::new(self, 0)
    }
    #[doc = "Bit 1 - Panel 1 Read Protect UserOTP1"]
    #[inline(always)]
    pub fn uo1rp(&mut self) -> Uo1rpW<'_, CrpSpec> {
        Uo1rpW::new(self, 1)
    }
    #[doc = "Bit 2 - Panel 1 Read Protect BootCfg1"]
    #[inline(always)]
    pub fn bc1rp(&mut self) -> Bc1rpW<'_, CrpSpec> {
        Bc1rpW::new(self, 2)
    }
    #[doc = "Bit 3 - Panel 1 Read Protect ROMCfg"]
    #[inline(always)]
    pub fn rcrp(&mut self) -> RcrpW<'_, CrpSpec> {
        RcrpW::new(self, 3)
    }
    #[doc = "Bit 4 - Panel 1 Read Protect VSSn"]
    #[inline(always)]
    pub fn vssrp0(&mut self) -> Vssrp0W<'_, CrpSpec> {
        Vssrp0W::new(self, 4)
    }
    #[doc = "Bit 5 - Panel 1 Read Protect VSSn"]
    #[inline(always)]
    pub fn vssrp1(&mut self) -> Vssrp1W<'_, CrpSpec> {
        Vssrp1W::new(self, 5)
    }
    #[doc = "Bit 6 - Panel 1 Read Protect Test"]
    #[inline(always)]
    pub fn trp(&mut self) -> TrpW<'_, CrpSpec> {
        TrpW::new(self, 6)
    }
    #[doc = "Bit 7 - Panel 1 Read Protect CalOTP"]
    #[inline(always)]
    pub fn corp(&mut self) -> CorpW<'_, CrpSpec> {
        CorpW::new(self, 7)
    }
    #[doc = "Bit 16 - Panel 1 Lock Read Protection BootCfg1A"]
    #[inline(always)]
    pub fn bc1arplock(&mut self) -> Bc1arplockW<'_, CrpSpec> {
        Bc1arplockW::new(self, 16)
    }
    #[doc = "Bit 17 - Panel 1 Lock Read Protection UserOTP1"]
    #[inline(always)]
    pub fn uo1rplock(&mut self) -> Uo1rplockW<'_, CrpSpec> {
        Uo1rplockW::new(self, 17)
    }
    #[doc = "Bit 18 - Panel 1 Lock Read Protection BootCfg1"]
    #[inline(always)]
    pub fn bc1rplock(&mut self) -> Bc1rplockW<'_, CrpSpec> {
        Bc1rplockW::new(self, 18)
    }
    #[doc = "Bit 19 - Panel 1 Lock Read Protection ROMCfg"]
    #[inline(always)]
    pub fn rcrplock(&mut self) -> RcrplockW<'_, CrpSpec> {
        RcrplockW::new(self, 19)
    }
    #[doc = "Bit 20 - Panel 1 Lock Read Protection VSSn"]
    #[inline(always)]
    pub fn vssrplock0(&mut self) -> Vssrplock0W<'_, CrpSpec> {
        Vssrplock0W::new(self, 20)
    }
    #[doc = "Bit 21 - Panel 1 Lock Read Protection VSSn"]
    #[inline(always)]
    pub fn vssrplock1(&mut self) -> Vssrplock1W<'_, CrpSpec> {
        Vssrplock1W::new(self, 21)
    }
    #[doc = "Bit 22 - Panel 1 Lock Read Protection Test"]
    #[inline(always)]
    pub fn trplock(&mut self) -> TrplockW<'_, CrpSpec> {
        TrplockW::new(self, 22)
    }
    #[doc = "Bit 23 - Panel 1 Lock Read Protection CalOTP"]
    #[inline(always)]
    pub fn corplock(&mut self) -> CorplockW<'_, CrpSpec> {
        CorplockW::new(self, 23)
    }
}
#[doc = "CFM Page Read Protection REGISTER (CFM8)\n\nYou can [`read`](crate::Reg::read) this register and get [`crp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrpSpec;
impl crate::RegisterSpec for CrpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crp::R`](R) reader structure"]
impl crate::Readable for CrpSpec {}
#[doc = "`write(|w| ..)` method takes [`crp::W`](W) writer structure"]
impl crate::Writable for CrpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRP to value 0x00c2_0000"]
impl crate::Resettable for CrpSpec {
    const RESET_VALUE: u32 = 0x00c2_0000;
}

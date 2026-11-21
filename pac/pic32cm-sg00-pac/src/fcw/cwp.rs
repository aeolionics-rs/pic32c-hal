#[doc = "Register `CWP` reader"]
pub type R = crate::R<CwpSpec>;
#[doc = "Register `CWP` writer"]
pub type W = crate::W<CwpSpec>;
#[doc = "Panel 1 Write Protect BootCfg1A\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bc1awpselect {
    #[doc = "0: Erase and Write Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Erase and Write Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Bc1awpselect> for bool {
    #[inline(always)]
    fn from(variant: Bc1awpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC1AWP` reader - Panel 1 Write Protect BootCfg1A"]
pub type Bc1awpR = crate::BitReader<Bc1awpselect>;
impl Bc1awpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bc1awpselect {
        match self.bits {
            false => Bc1awpselect::Disable,
            true => Bc1awpselect::Enable,
        }
    }
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bc1awpselect::Disable
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bc1awpselect::Enable
    }
}
#[doc = "Field `BC1AWP` writer - Panel 1 Write Protect BootCfg1A"]
pub type Bc1awpW<'a, REG> = crate::BitWriter<'a, REG, Bc1awpselect>;
impl<'a, REG> Bc1awpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1awpselect::Disable)
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1awpselect::Enable)
    }
}
#[doc = "Panel 1 Write Protect BootCfg1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bc1wpselect {
    #[doc = "0: Erase and Write Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Erase and Write Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Bc1wpselect> for bool {
    #[inline(always)]
    fn from(variant: Bc1wpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC1WP` reader - Panel 1 Write Protect BootCfg1"]
pub type Bc1wpR = crate::BitReader<Bc1wpselect>;
impl Bc1wpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bc1wpselect {
        match self.bits {
            false => Bc1wpselect::Disable,
            true => Bc1wpselect::Enable,
        }
    }
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bc1wpselect::Disable
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bc1wpselect::Enable
    }
}
#[doc = "Field `BC1WP` writer - Panel 1 Write Protect BootCfg1"]
pub type Bc1wpW<'a, REG> = crate::BitWriter<'a, REG, Bc1wpselect>;
impl<'a, REG> Bc1wpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1wpselect::Disable)
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1wpselect::Enable)
    }
}
#[doc = "Panel 1 Write Protect ROMCfg\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcwpselect {
    #[doc = "0: Erase and Write Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Erase and Write Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Rcwpselect> for bool {
    #[inline(always)]
    fn from(variant: Rcwpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCWP` reader - Panel 1 Write Protect ROMCfg"]
pub type RcwpR = crate::BitReader<Rcwpselect>;
impl RcwpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcwpselect {
        match self.bits {
            false => Rcwpselect::Disable,
            true => Rcwpselect::Enable,
        }
    }
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rcwpselect::Disable
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rcwpselect::Enable
    }
}
#[doc = "Field `RCWP` writer - Panel 1 Write Protect ROMCfg"]
pub type RcwpW<'a, REG> = crate::BitWriter<'a, REG, Rcwpselect>;
impl<'a, REG> RcwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rcwpselect::Disable)
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rcwpselect::Enable)
    }
}
#[doc = "Panel 1 Write Protect VSSn\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vsswp0select {
    #[doc = "0: Erase and Write Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Erase and Write Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Vsswp0select> for bool {
    #[inline(always)]
    fn from(variant: Vsswp0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSSWP0` reader - Panel 1 Write Protect VSSn"]
pub type Vsswp0R = crate::BitReader<Vsswp0select>;
impl Vsswp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vsswp0select {
        match self.bits {
            false => Vsswp0select::Disable,
            true => Vsswp0select::Enable,
        }
    }
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Vsswp0select::Disable
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Vsswp0select::Enable
    }
}
#[doc = "Field `VSSWP0` writer - Panel 1 Write Protect VSSn"]
pub type Vsswp0W<'a, REG> = crate::BitWriter<'a, REG, Vsswp0select>;
impl<'a, REG> Vsswp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Vsswp0select::Disable)
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Vsswp0select::Enable)
    }
}
#[doc = "Panel 1 Write Protect VSSn\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vsswp1select {
    #[doc = "0: Erase and Write Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Erase and Write Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Vsswp1select> for bool {
    #[inline(always)]
    fn from(variant: Vsswp1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSSWP1` reader - Panel 1 Write Protect VSSn"]
pub type Vsswp1R = crate::BitReader<Vsswp1select>;
impl Vsswp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vsswp1select {
        match self.bits {
            false => Vsswp1select::Disable,
            true => Vsswp1select::Enable,
        }
    }
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Vsswp1select::Disable
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Vsswp1select::Enable
    }
}
#[doc = "Field `VSSWP1` writer - Panel 1 Write Protect VSSn"]
pub type Vsswp1W<'a, REG> = crate::BitWriter<'a, REG, Vsswp1select>;
impl<'a, REG> Vsswp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Vsswp1select::Disable)
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Vsswp1select::Enable)
    }
}
#[doc = "Panel 1 Write Protect Test\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Twpselect {
    #[doc = "0: Erase and Write Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Erase and Write Protection for this Page is Enable"]
    Enable = 1,
}
impl From<Twpselect> for bool {
    #[inline(always)]
    fn from(variant: Twpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWP` reader - Panel 1 Write Protect Test"]
pub type TwpR = crate::BitReader<Twpselect>;
impl TwpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Twpselect {
        match self.bits {
            false => Twpselect::Disable,
            true => Twpselect::Enable,
        }
    }
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Twpselect::Disable
    }
    #[doc = "Erase and Write Protection for this Page is Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Twpselect::Enable
    }
}
#[doc = "Field `TWP` writer - Panel 1 Write Protect Test"]
pub type TwpW<'a, REG> = crate::BitWriter<'a, REG, Twpselect>;
impl<'a, REG> TwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Twpselect::Disable)
    }
    #[doc = "Erase and Write Protection for this Page is Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Twpselect::Enable)
    }
}
#[doc = "Panel 1 Write Protect CalOTP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cowpselect {
    #[doc = "0: Erase and Write Protection for this Page is Disabled"]
    Disable = 0,
    #[doc = "1: Erase and Write Protection for this Page is Enabled"]
    Enable = 1,
}
impl From<Cowpselect> for bool {
    #[inline(always)]
    fn from(variant: Cowpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COWP` reader - Panel 1 Write Protect CalOTP"]
pub type CowpR = crate::BitReader<Cowpselect>;
impl CowpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cowpselect {
        match self.bits {
            false => Cowpselect::Disable,
            true => Cowpselect::Enable,
        }
    }
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cowpselect::Disable
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cowpselect::Enable
    }
}
#[doc = "Field `COWP` writer - Panel 1 Write Protect CalOTP"]
pub type CowpW<'a, REG> = crate::BitWriter<'a, REG, Cowpselect>;
impl<'a, REG> CowpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase and Write Protection for this Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cowpselect::Disable)
    }
    #[doc = "Erase and Write Protection for this Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cowpselect::Enable)
    }
}
#[doc = "Panel 1 Lock Write Protection BootCfg1A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bc1awplockselect {
    #[doc = "0: The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Bc1awplockselect> for bool {
    #[inline(always)]
    fn from(variant: Bc1awplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC1AWPLOCK` reader - Panel 1 Lock Write Protection BootCfg1A"]
pub type Bc1awplockR = crate::BitReader<Bc1awplockselect>;
impl Bc1awplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bc1awplockselect {
        match self.bits {
            false => Bc1awplockselect::Unlocked,
            true => Bc1awplockselect::Locked,
        }
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Bc1awplockselect::Unlocked
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Bc1awplockselect::Locked
    }
}
#[doc = "Field `BC1AWPLOCK` writer - Panel 1 Lock Write Protection BootCfg1A"]
pub type Bc1awplockW<'a, REG> = crate::BitWriter<'a, REG, Bc1awplockselect>;
impl<'a, REG> Bc1awplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1awplockselect::Unlocked)
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1awplockselect::Locked)
    }
}
#[doc = "Panel 1 Lock Write Protection BootCfg1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bc1wplockselect {
    #[doc = "0: The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Bc1wplockselect> for bool {
    #[inline(always)]
    fn from(variant: Bc1wplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC1WPLOCK` reader - Panel 1 Lock Write Protection BootCfg1"]
pub type Bc1wplockR = crate::BitReader<Bc1wplockselect>;
impl Bc1wplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bc1wplockselect {
        match self.bits {
            false => Bc1wplockselect::Unlocked,
            true => Bc1wplockselect::Locked,
        }
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Bc1wplockselect::Unlocked
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Bc1wplockselect::Locked
    }
}
#[doc = "Field `BC1WPLOCK` writer - Panel 1 Lock Write Protection BootCfg1"]
pub type Bc1wplockW<'a, REG> = crate::BitWriter<'a, REG, Bc1wplockselect>;
impl<'a, REG> Bc1wplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1wplockselect::Unlocked)
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Bc1wplockselect::Locked)
    }
}
#[doc = "Panel 1 Lock Write Protection ROMCfg\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcwplockselect {
    #[doc = "0: The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Rcwplockselect> for bool {
    #[inline(always)]
    fn from(variant: Rcwplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCWPLOCK` reader - Panel 1 Lock Write Protection ROMCfg"]
pub type RcwplockR = crate::BitReader<Rcwplockselect>;
impl RcwplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcwplockselect {
        match self.bits {
            false => Rcwplockselect::Unlocked,
            true => Rcwplockselect::Locked,
        }
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Rcwplockselect::Unlocked
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Rcwplockselect::Locked
    }
}
#[doc = "Field `RCWPLOCK` writer - Panel 1 Lock Write Protection ROMCfg"]
pub type RcwplockW<'a, REG> = crate::BitWriter<'a, REG, Rcwplockselect>;
impl<'a, REG> RcwplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Rcwplockselect::Unlocked)
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Rcwplockselect::Locked)
    }
}
#[doc = "Panel 1 Lock Write Protection VSSn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vsswplock0select {
    #[doc = "0: The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Vsswplock0select> for bool {
    #[inline(always)]
    fn from(variant: Vsswplock0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSSWPLOCK0` reader - Panel 1 Lock Write Protection VSSn"]
pub type Vsswplock0R = crate::BitReader<Vsswplock0select>;
impl Vsswplock0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vsswplock0select {
        match self.bits {
            false => Vsswplock0select::Unlocked,
            true => Vsswplock0select::Locked,
        }
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Vsswplock0select::Unlocked
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Vsswplock0select::Locked
    }
}
#[doc = "Field `VSSWPLOCK0` writer - Panel 1 Lock Write Protection VSSn"]
pub type Vsswplock0W<'a, REG> = crate::BitWriter<'a, REG, Vsswplock0select>;
impl<'a, REG> Vsswplock0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Vsswplock0select::Unlocked)
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Vsswplock0select::Locked)
    }
}
#[doc = "Panel 1 Lock Write Protection VSSn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vsswplock1select {
    #[doc = "0: The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Vsswplock1select> for bool {
    #[inline(always)]
    fn from(variant: Vsswplock1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSSWPLOCK1` reader - Panel 1 Lock Write Protection VSSn"]
pub type Vsswplock1R = crate::BitReader<Vsswplock1select>;
impl Vsswplock1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vsswplock1select {
        match self.bits {
            false => Vsswplock1select::Unlocked,
            true => Vsswplock1select::Locked,
        }
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Vsswplock1select::Unlocked
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Vsswplock1select::Locked
    }
}
#[doc = "Field `VSSWPLOCK1` writer - Panel 1 Lock Write Protection VSSn"]
pub type Vsswplock1W<'a, REG> = crate::BitWriter<'a, REG, Vsswplock1select>;
impl<'a, REG> Vsswplock1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Vsswplock1select::Unlocked)
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Vsswplock1select::Locked)
    }
}
#[doc = "Panel 1 Lock Write Protection Test\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Twplockselect {
    #[doc = "0: The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Twplockselect> for bool {
    #[inline(always)]
    fn from(variant: Twplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWPLOCK` reader - Panel 1 Lock Write Protection Test"]
pub type TwplockR = crate::BitReader<Twplockselect>;
impl TwplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Twplockselect {
        match self.bits {
            false => Twplockselect::Unlocked,
            true => Twplockselect::Locked,
        }
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Twplockselect::Unlocked
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Twplockselect::Locked
    }
}
#[doc = "Field `TWPLOCK` writer - Panel 1 Lock Write Protection Test"]
pub type TwplockW<'a, REG> = crate::BitWriter<'a, REG, Twplockselect>;
impl<'a, REG> TwplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Twplockselect::Unlocked)
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Twplockselect::Locked)
    }
}
#[doc = "Panel 1 Lock Write Protection CalOTP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cowplockselect {
    #[doc = "0: The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Cowplockselect> for bool {
    #[inline(always)]
    fn from(variant: Cowplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COWPLOCK` reader - Panel 1 Lock Write Protection CalOTP"]
pub type CowplockR = crate::BitReader<Cowplockselect>;
impl CowplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cowplockselect {
        match self.bits {
            false => Cowplockselect::Unlocked,
            true => Cowplockselect::Locked,
        }
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Cowplockselect::Unlocked
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Cowplockselect::Locked
    }
}
#[doc = "Field `COWPLOCK` writer - Panel 1 Lock Write Protection CalOTP"]
pub type CowplockW<'a, REG> = crate::BitWriter<'a, REG, Cowplockselect>;
impl<'a, REG> CowplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Lock &amp; Write Protect bits for this Page are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Cowplockselect::Unlocked)
    }
    #[doc = "The Lock &amp; Write Protect bits for this Page are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Cowplockselect::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Panel 1 Write Protect BootCfg1A"]
    #[inline(always)]
    pub fn bc1awp(&self) -> Bc1awpR {
        Bc1awpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Panel 1 Write Protect BootCfg1"]
    #[inline(always)]
    pub fn bc1wp(&self) -> Bc1wpR {
        Bc1wpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Panel 1 Write Protect ROMCfg"]
    #[inline(always)]
    pub fn rcwp(&self) -> RcwpR {
        RcwpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Panel 1 Write Protect VSSn"]
    #[inline(always)]
    pub fn vsswp0(&self) -> Vsswp0R {
        Vsswp0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Panel 1 Write Protect VSSn"]
    #[inline(always)]
    pub fn vsswp1(&self) -> Vsswp1R {
        Vsswp1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Panel 1 Write Protect Test"]
    #[inline(always)]
    pub fn twp(&self) -> TwpR {
        TwpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Panel 1 Write Protect CalOTP"]
    #[inline(always)]
    pub fn cowp(&self) -> CowpR {
        CowpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Panel 1 Lock Write Protection BootCfg1A"]
    #[inline(always)]
    pub fn bc1awplock(&self) -> Bc1awplockR {
        Bc1awplockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Panel 1 Lock Write Protection BootCfg1"]
    #[inline(always)]
    pub fn bc1wplock(&self) -> Bc1wplockR {
        Bc1wplockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Panel 1 Lock Write Protection ROMCfg"]
    #[inline(always)]
    pub fn rcwplock(&self) -> RcwplockR {
        RcwplockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Panel 1 Lock Write Protection VSSn"]
    #[inline(always)]
    pub fn vsswplock0(&self) -> Vsswplock0R {
        Vsswplock0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Panel 1 Lock Write Protection VSSn"]
    #[inline(always)]
    pub fn vsswplock1(&self) -> Vsswplock1R {
        Vsswplock1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Panel 1 Lock Write Protection Test"]
    #[inline(always)]
    pub fn twplock(&self) -> TwplockR {
        TwplockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Panel 1 Lock Write Protection CalOTP"]
    #[inline(always)]
    pub fn cowplock(&self) -> CowplockR {
        CowplockR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Panel 1 Write Protect BootCfg1A"]
    #[inline(always)]
    pub fn bc1awp(&mut self) -> Bc1awpW<'_, CwpSpec> {
        Bc1awpW::new(self, 0)
    }
    #[doc = "Bit 2 - Panel 1 Write Protect BootCfg1"]
    #[inline(always)]
    pub fn bc1wp(&mut self) -> Bc1wpW<'_, CwpSpec> {
        Bc1wpW::new(self, 2)
    }
    #[doc = "Bit 3 - Panel 1 Write Protect ROMCfg"]
    #[inline(always)]
    pub fn rcwp(&mut self) -> RcwpW<'_, CwpSpec> {
        RcwpW::new(self, 3)
    }
    #[doc = "Bit 4 - Panel 1 Write Protect VSSn"]
    #[inline(always)]
    pub fn vsswp0(&mut self) -> Vsswp0W<'_, CwpSpec> {
        Vsswp0W::new(self, 4)
    }
    #[doc = "Bit 5 - Panel 1 Write Protect VSSn"]
    #[inline(always)]
    pub fn vsswp1(&mut self) -> Vsswp1W<'_, CwpSpec> {
        Vsswp1W::new(self, 5)
    }
    #[doc = "Bit 6 - Panel 1 Write Protect Test"]
    #[inline(always)]
    pub fn twp(&mut self) -> TwpW<'_, CwpSpec> {
        TwpW::new(self, 6)
    }
    #[doc = "Bit 7 - Panel 1 Write Protect CalOTP"]
    #[inline(always)]
    pub fn cowp(&mut self) -> CowpW<'_, CwpSpec> {
        CowpW::new(self, 7)
    }
    #[doc = "Bit 16 - Panel 1 Lock Write Protection BootCfg1A"]
    #[inline(always)]
    pub fn bc1awplock(&mut self) -> Bc1awplockW<'_, CwpSpec> {
        Bc1awplockW::new(self, 16)
    }
    #[doc = "Bit 18 - Panel 1 Lock Write Protection BootCfg1"]
    #[inline(always)]
    pub fn bc1wplock(&mut self) -> Bc1wplockW<'_, CwpSpec> {
        Bc1wplockW::new(self, 18)
    }
    #[doc = "Bit 19 - Panel 1 Lock Write Protection ROMCfg"]
    #[inline(always)]
    pub fn rcwplock(&mut self) -> RcwplockW<'_, CwpSpec> {
        RcwplockW::new(self, 19)
    }
    #[doc = "Bit 20 - Panel 1 Lock Write Protection VSSn"]
    #[inline(always)]
    pub fn vsswplock0(&mut self) -> Vsswplock0W<'_, CwpSpec> {
        Vsswplock0W::new(self, 20)
    }
    #[doc = "Bit 21 - Panel 1 Lock Write Protection VSSn"]
    #[inline(always)]
    pub fn vsswplock1(&mut self) -> Vsswplock1W<'_, CwpSpec> {
        Vsswplock1W::new(self, 21)
    }
    #[doc = "Bit 22 - Panel 1 Lock Write Protection Test"]
    #[inline(always)]
    pub fn twplock(&mut self) -> TwplockW<'_, CwpSpec> {
        TwplockW::new(self, 22)
    }
    #[doc = "Bit 23 - Panel 1 Lock Write Protection CalOTP"]
    #[inline(always)]
    pub fn cowplock(&mut self) -> CowplockW<'_, CwpSpec> {
        CowplockW::new(self, 23)
    }
}
#[doc = "CFM Page Write Protect REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`cwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwpSpec;
impl crate::RegisterSpec for CwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwp::R`](R) reader structure"]
impl crate::Readable for CwpSpec {}
#[doc = "`write(|w| ..)` method takes [`cwp::W`](W) writer structure"]
impl crate::Writable for CwpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CWP to value 0x08c0_08fd"]
impl crate::Resettable for CwpSpec {
    const RESET_VALUE: u32 = 0x08c0_08fd;
}

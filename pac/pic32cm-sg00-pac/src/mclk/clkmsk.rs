#[doc = "Register `CLKMSK[%s]` reader"]
pub type R = crate::R<ClkmskSpec>;
#[doc = "Register `CLKMSK[%s]` writer"]
pub type W = crate::W<ClkmskSpec>;
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask0select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask0_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask0_1 = 1,
}
impl From<Mask0select> for bool {
    #[inline(always)]
    fn from(variant: Mask0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK0` reader - Peripheral Clock Enable Mask"]
pub type Mask0R = crate::BitReader<Mask0select>;
impl Mask0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask0select {
        match self.bits {
            false => Mask0select::Mask0_0,
            true => Mask0select::Mask0_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask0_0(&self) -> bool {
        *self == Mask0select::Mask0_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask0_1(&self) -> bool {
        *self == Mask0select::Mask0_1
    }
}
#[doc = "Field `MASK0` writer - Peripheral Clock Enable Mask"]
pub type Mask0W<'a, REG> = crate::BitWriter<'a, REG, Mask0select>;
impl<'a, REG> Mask0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask0select::Mask0_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask0select::Mask0_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask1select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask1_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask1_1 = 1,
}
impl From<Mask1select> for bool {
    #[inline(always)]
    fn from(variant: Mask1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK1` reader - Peripheral Clock Enable Mask"]
pub type Mask1R = crate::BitReader<Mask1select>;
impl Mask1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask1select {
        match self.bits {
            false => Mask1select::Mask1_0,
            true => Mask1select::Mask1_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask1_0(&self) -> bool {
        *self == Mask1select::Mask1_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask1_1(&self) -> bool {
        *self == Mask1select::Mask1_1
    }
}
#[doc = "Field `MASK1` writer - Peripheral Clock Enable Mask"]
pub type Mask1W<'a, REG> = crate::BitWriter<'a, REG, Mask1select>;
impl<'a, REG> Mask1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask1select::Mask1_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask1select::Mask1_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask2select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask2_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask2_1 = 1,
}
impl From<Mask2select> for bool {
    #[inline(always)]
    fn from(variant: Mask2select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK2` reader - Peripheral Clock Enable Mask"]
pub type Mask2R = crate::BitReader<Mask2select>;
impl Mask2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask2select {
        match self.bits {
            false => Mask2select::Mask2_0,
            true => Mask2select::Mask2_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask2_0(&self) -> bool {
        *self == Mask2select::Mask2_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask2_1(&self) -> bool {
        *self == Mask2select::Mask2_1
    }
}
#[doc = "Field `MASK2` writer - Peripheral Clock Enable Mask"]
pub type Mask2W<'a, REG> = crate::BitWriter<'a, REG, Mask2select>;
impl<'a, REG> Mask2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask2select::Mask2_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask2select::Mask2_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask3select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask3_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask3_1 = 1,
}
impl From<Mask3select> for bool {
    #[inline(always)]
    fn from(variant: Mask3select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK3` reader - Peripheral Clock Enable Mask"]
pub type Mask3R = crate::BitReader<Mask3select>;
impl Mask3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask3select {
        match self.bits {
            false => Mask3select::Mask3_0,
            true => Mask3select::Mask3_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask3_0(&self) -> bool {
        *self == Mask3select::Mask3_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask3_1(&self) -> bool {
        *self == Mask3select::Mask3_1
    }
}
#[doc = "Field `MASK3` writer - Peripheral Clock Enable Mask"]
pub type Mask3W<'a, REG> = crate::BitWriter<'a, REG, Mask3select>;
impl<'a, REG> Mask3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask3select::Mask3_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask3select::Mask3_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask4select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask4_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask4_1 = 1,
}
impl From<Mask4select> for bool {
    #[inline(always)]
    fn from(variant: Mask4select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK4` reader - Peripheral Clock Enable Mask"]
pub type Mask4R = crate::BitReader<Mask4select>;
impl Mask4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask4select {
        match self.bits {
            false => Mask4select::Mask4_0,
            true => Mask4select::Mask4_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask4_0(&self) -> bool {
        *self == Mask4select::Mask4_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask4_1(&self) -> bool {
        *self == Mask4select::Mask4_1
    }
}
#[doc = "Field `MASK4` writer - Peripheral Clock Enable Mask"]
pub type Mask4W<'a, REG> = crate::BitWriter<'a, REG, Mask4select>;
impl<'a, REG> Mask4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask4_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask4select::Mask4_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask4_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask4select::Mask4_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask5select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask5_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask5_1 = 1,
}
impl From<Mask5select> for bool {
    #[inline(always)]
    fn from(variant: Mask5select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK5` reader - Peripheral Clock Enable Mask"]
pub type Mask5R = crate::BitReader<Mask5select>;
impl Mask5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask5select {
        match self.bits {
            false => Mask5select::Mask5_0,
            true => Mask5select::Mask5_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask5_0(&self) -> bool {
        *self == Mask5select::Mask5_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask5_1(&self) -> bool {
        *self == Mask5select::Mask5_1
    }
}
#[doc = "Field `MASK5` writer - Peripheral Clock Enable Mask"]
pub type Mask5W<'a, REG> = crate::BitWriter<'a, REG, Mask5select>;
impl<'a, REG> Mask5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask5select::Mask5_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask5select::Mask5_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask6select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask6_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask6_1 = 1,
}
impl From<Mask6select> for bool {
    #[inline(always)]
    fn from(variant: Mask6select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK6` reader - Peripheral Clock Enable Mask"]
pub type Mask6R = crate::BitReader<Mask6select>;
impl Mask6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask6select {
        match self.bits {
            false => Mask6select::Mask6_0,
            true => Mask6select::Mask6_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask6_0(&self) -> bool {
        *self == Mask6select::Mask6_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask6_1(&self) -> bool {
        *self == Mask6select::Mask6_1
    }
}
#[doc = "Field `MASK6` writer - Peripheral Clock Enable Mask"]
pub type Mask6W<'a, REG> = crate::BitWriter<'a, REG, Mask6select>;
impl<'a, REG> Mask6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask6_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask6select::Mask6_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask6_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask6select::Mask6_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask7select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask7_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask7_1 = 1,
}
impl From<Mask7select> for bool {
    #[inline(always)]
    fn from(variant: Mask7select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK7` reader - Peripheral Clock Enable Mask"]
pub type Mask7R = crate::BitReader<Mask7select>;
impl Mask7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask7select {
        match self.bits {
            false => Mask7select::Mask7_0,
            true => Mask7select::Mask7_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask7_0(&self) -> bool {
        *self == Mask7select::Mask7_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask7_1(&self) -> bool {
        *self == Mask7select::Mask7_1
    }
}
#[doc = "Field `MASK7` writer - Peripheral Clock Enable Mask"]
pub type Mask7W<'a, REG> = crate::BitWriter<'a, REG, Mask7select>;
impl<'a, REG> Mask7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask7_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask7select::Mask7_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask7_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask7select::Mask7_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask8select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask8_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask8_1 = 1,
}
impl From<Mask8select> for bool {
    #[inline(always)]
    fn from(variant: Mask8select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK8` reader - Peripheral Clock Enable Mask"]
pub type Mask8R = crate::BitReader<Mask8select>;
impl Mask8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask8select {
        match self.bits {
            false => Mask8select::Mask8_0,
            true => Mask8select::Mask8_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask8_0(&self) -> bool {
        *self == Mask8select::Mask8_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask8_1(&self) -> bool {
        *self == Mask8select::Mask8_1
    }
}
#[doc = "Field `MASK8` writer - Peripheral Clock Enable Mask"]
pub type Mask8W<'a, REG> = crate::BitWriter<'a, REG, Mask8select>;
impl<'a, REG> Mask8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask8_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask8select::Mask8_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask8_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask8select::Mask8_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask9select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask9_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask9_1 = 1,
}
impl From<Mask9select> for bool {
    #[inline(always)]
    fn from(variant: Mask9select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK9` reader - Peripheral Clock Enable Mask"]
pub type Mask9R = crate::BitReader<Mask9select>;
impl Mask9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask9select {
        match self.bits {
            false => Mask9select::Mask9_0,
            true => Mask9select::Mask9_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask9_0(&self) -> bool {
        *self == Mask9select::Mask9_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask9_1(&self) -> bool {
        *self == Mask9select::Mask9_1
    }
}
#[doc = "Field `MASK9` writer - Peripheral Clock Enable Mask"]
pub type Mask9W<'a, REG> = crate::BitWriter<'a, REG, Mask9select>;
impl<'a, REG> Mask9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask9_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask9select::Mask9_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask9_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask9select::Mask9_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask10select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask10_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask10_1 = 1,
}
impl From<Mask10select> for bool {
    #[inline(always)]
    fn from(variant: Mask10select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK10` reader - Peripheral Clock Enable Mask"]
pub type Mask10R = crate::BitReader<Mask10select>;
impl Mask10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask10select {
        match self.bits {
            false => Mask10select::Mask10_0,
            true => Mask10select::Mask10_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask10_0(&self) -> bool {
        *self == Mask10select::Mask10_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask10_1(&self) -> bool {
        *self == Mask10select::Mask10_1
    }
}
#[doc = "Field `MASK10` writer - Peripheral Clock Enable Mask"]
pub type Mask10W<'a, REG> = crate::BitWriter<'a, REG, Mask10select>;
impl<'a, REG> Mask10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask10_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask10select::Mask10_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask10_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask10select::Mask10_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask11select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask11_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask11_1 = 1,
}
impl From<Mask11select> for bool {
    #[inline(always)]
    fn from(variant: Mask11select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK11` reader - Peripheral Clock Enable Mask"]
pub type Mask11R = crate::BitReader<Mask11select>;
impl Mask11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask11select {
        match self.bits {
            false => Mask11select::Mask11_0,
            true => Mask11select::Mask11_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask11_0(&self) -> bool {
        *self == Mask11select::Mask11_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask11_1(&self) -> bool {
        *self == Mask11select::Mask11_1
    }
}
#[doc = "Field `MASK11` writer - Peripheral Clock Enable Mask"]
pub type Mask11W<'a, REG> = crate::BitWriter<'a, REG, Mask11select>;
impl<'a, REG> Mask11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask11_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask11select::Mask11_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask11_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask11select::Mask11_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask12select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask12_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask12_1 = 1,
}
impl From<Mask12select> for bool {
    #[inline(always)]
    fn from(variant: Mask12select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK12` reader - Peripheral Clock Enable Mask"]
pub type Mask12R = crate::BitReader<Mask12select>;
impl Mask12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask12select {
        match self.bits {
            false => Mask12select::Mask12_0,
            true => Mask12select::Mask12_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask12_0(&self) -> bool {
        *self == Mask12select::Mask12_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask12_1(&self) -> bool {
        *self == Mask12select::Mask12_1
    }
}
#[doc = "Field `MASK12` writer - Peripheral Clock Enable Mask"]
pub type Mask12W<'a, REG> = crate::BitWriter<'a, REG, Mask12select>;
impl<'a, REG> Mask12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask12_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask12select::Mask12_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask12_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask12select::Mask12_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask13select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask13_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask13_1 = 1,
}
impl From<Mask13select> for bool {
    #[inline(always)]
    fn from(variant: Mask13select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK13` reader - Peripheral Clock Enable Mask"]
pub type Mask13R = crate::BitReader<Mask13select>;
impl Mask13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask13select {
        match self.bits {
            false => Mask13select::Mask13_0,
            true => Mask13select::Mask13_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask13_0(&self) -> bool {
        *self == Mask13select::Mask13_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask13_1(&self) -> bool {
        *self == Mask13select::Mask13_1
    }
}
#[doc = "Field `MASK13` writer - Peripheral Clock Enable Mask"]
pub type Mask13W<'a, REG> = crate::BitWriter<'a, REG, Mask13select>;
impl<'a, REG> Mask13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask13_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask13select::Mask13_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask13_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask13select::Mask13_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask14select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask14_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask14_1 = 1,
}
impl From<Mask14select> for bool {
    #[inline(always)]
    fn from(variant: Mask14select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK14` reader - Peripheral Clock Enable Mask"]
pub type Mask14R = crate::BitReader<Mask14select>;
impl Mask14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask14select {
        match self.bits {
            false => Mask14select::Mask14_0,
            true => Mask14select::Mask14_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask14_0(&self) -> bool {
        *self == Mask14select::Mask14_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask14_1(&self) -> bool {
        *self == Mask14select::Mask14_1
    }
}
#[doc = "Field `MASK14` writer - Peripheral Clock Enable Mask"]
pub type Mask14W<'a, REG> = crate::BitWriter<'a, REG, Mask14select>;
impl<'a, REG> Mask14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask14_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask14select::Mask14_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask14_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask14select::Mask14_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask15select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask15_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask15_1 = 1,
}
impl From<Mask15select> for bool {
    #[inline(always)]
    fn from(variant: Mask15select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK15` reader - Peripheral Clock Enable Mask"]
pub type Mask15R = crate::BitReader<Mask15select>;
impl Mask15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask15select {
        match self.bits {
            false => Mask15select::Mask15_0,
            true => Mask15select::Mask15_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask15_0(&self) -> bool {
        *self == Mask15select::Mask15_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask15_1(&self) -> bool {
        *self == Mask15select::Mask15_1
    }
}
#[doc = "Field `MASK15` writer - Peripheral Clock Enable Mask"]
pub type Mask15W<'a, REG> = crate::BitWriter<'a, REG, Mask15select>;
impl<'a, REG> Mask15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask15_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask15select::Mask15_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask15_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask15select::Mask15_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask16select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask16_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask16_1 = 1,
}
impl From<Mask16select> for bool {
    #[inline(always)]
    fn from(variant: Mask16select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK16` reader - Peripheral Clock Enable Mask"]
pub type Mask16R = crate::BitReader<Mask16select>;
impl Mask16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask16select {
        match self.bits {
            false => Mask16select::Mask16_0,
            true => Mask16select::Mask16_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask16_0(&self) -> bool {
        *self == Mask16select::Mask16_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask16_1(&self) -> bool {
        *self == Mask16select::Mask16_1
    }
}
#[doc = "Field `MASK16` writer - Peripheral Clock Enable Mask"]
pub type Mask16W<'a, REG> = crate::BitWriter<'a, REG, Mask16select>;
impl<'a, REG> Mask16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask16_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask16select::Mask16_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask16_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask16select::Mask16_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask17select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask17_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask17_1 = 1,
}
impl From<Mask17select> for bool {
    #[inline(always)]
    fn from(variant: Mask17select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK17` reader - Peripheral Clock Enable Mask"]
pub type Mask17R = crate::BitReader<Mask17select>;
impl Mask17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask17select {
        match self.bits {
            false => Mask17select::Mask17_0,
            true => Mask17select::Mask17_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask17_0(&self) -> bool {
        *self == Mask17select::Mask17_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask17_1(&self) -> bool {
        *self == Mask17select::Mask17_1
    }
}
#[doc = "Field `MASK17` writer - Peripheral Clock Enable Mask"]
pub type Mask17W<'a, REG> = crate::BitWriter<'a, REG, Mask17select>;
impl<'a, REG> Mask17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask17_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask17select::Mask17_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask17_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask17select::Mask17_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask18select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask18_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask18_1 = 1,
}
impl From<Mask18select> for bool {
    #[inline(always)]
    fn from(variant: Mask18select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK18` reader - Peripheral Clock Enable Mask"]
pub type Mask18R = crate::BitReader<Mask18select>;
impl Mask18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask18select {
        match self.bits {
            false => Mask18select::Mask18_0,
            true => Mask18select::Mask18_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask18_0(&self) -> bool {
        *self == Mask18select::Mask18_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask18_1(&self) -> bool {
        *self == Mask18select::Mask18_1
    }
}
#[doc = "Field `MASK18` writer - Peripheral Clock Enable Mask"]
pub type Mask18W<'a, REG> = crate::BitWriter<'a, REG, Mask18select>;
impl<'a, REG> Mask18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask18_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask18select::Mask18_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask18_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask18select::Mask18_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask19select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask19_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask19_1 = 1,
}
impl From<Mask19select> for bool {
    #[inline(always)]
    fn from(variant: Mask19select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK19` reader - Peripheral Clock Enable Mask"]
pub type Mask19R = crate::BitReader<Mask19select>;
impl Mask19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask19select {
        match self.bits {
            false => Mask19select::Mask19_0,
            true => Mask19select::Mask19_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask19_0(&self) -> bool {
        *self == Mask19select::Mask19_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask19_1(&self) -> bool {
        *self == Mask19select::Mask19_1
    }
}
#[doc = "Field `MASK19` writer - Peripheral Clock Enable Mask"]
pub type Mask19W<'a, REG> = crate::BitWriter<'a, REG, Mask19select>;
impl<'a, REG> Mask19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask19_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask19select::Mask19_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask19_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask19select::Mask19_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask20select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask20_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask20_1 = 1,
}
impl From<Mask20select> for bool {
    #[inline(always)]
    fn from(variant: Mask20select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK20` reader - Peripheral Clock Enable Mask"]
pub type Mask20R = crate::BitReader<Mask20select>;
impl Mask20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask20select {
        match self.bits {
            false => Mask20select::Mask20_0,
            true => Mask20select::Mask20_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask20_0(&self) -> bool {
        *self == Mask20select::Mask20_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask20_1(&self) -> bool {
        *self == Mask20select::Mask20_1
    }
}
#[doc = "Field `MASK20` writer - Peripheral Clock Enable Mask"]
pub type Mask20W<'a, REG> = crate::BitWriter<'a, REG, Mask20select>;
impl<'a, REG> Mask20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask20_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask20select::Mask20_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask20_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask20select::Mask20_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask21select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask21_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask21_1 = 1,
}
impl From<Mask21select> for bool {
    #[inline(always)]
    fn from(variant: Mask21select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK21` reader - Peripheral Clock Enable Mask"]
pub type Mask21R = crate::BitReader<Mask21select>;
impl Mask21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask21select {
        match self.bits {
            false => Mask21select::Mask21_0,
            true => Mask21select::Mask21_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask21_0(&self) -> bool {
        *self == Mask21select::Mask21_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask21_1(&self) -> bool {
        *self == Mask21select::Mask21_1
    }
}
#[doc = "Field `MASK21` writer - Peripheral Clock Enable Mask"]
pub type Mask21W<'a, REG> = crate::BitWriter<'a, REG, Mask21select>;
impl<'a, REG> Mask21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask21_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask21select::Mask21_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask21_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask21select::Mask21_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask22select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask22_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask22_1 = 1,
}
impl From<Mask22select> for bool {
    #[inline(always)]
    fn from(variant: Mask22select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK22` reader - Peripheral Clock Enable Mask"]
pub type Mask22R = crate::BitReader<Mask22select>;
impl Mask22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask22select {
        match self.bits {
            false => Mask22select::Mask22_0,
            true => Mask22select::Mask22_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask22_0(&self) -> bool {
        *self == Mask22select::Mask22_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask22_1(&self) -> bool {
        *self == Mask22select::Mask22_1
    }
}
#[doc = "Field `MASK22` writer - Peripheral Clock Enable Mask"]
pub type Mask22W<'a, REG> = crate::BitWriter<'a, REG, Mask22select>;
impl<'a, REG> Mask22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask22_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask22select::Mask22_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask22_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask22select::Mask22_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask23select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask23_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask23_1 = 1,
}
impl From<Mask23select> for bool {
    #[inline(always)]
    fn from(variant: Mask23select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK23` reader - Peripheral Clock Enable Mask"]
pub type Mask23R = crate::BitReader<Mask23select>;
impl Mask23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask23select {
        match self.bits {
            false => Mask23select::Mask23_0,
            true => Mask23select::Mask23_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask23_0(&self) -> bool {
        *self == Mask23select::Mask23_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask23_1(&self) -> bool {
        *self == Mask23select::Mask23_1
    }
}
#[doc = "Field `MASK23` writer - Peripheral Clock Enable Mask"]
pub type Mask23W<'a, REG> = crate::BitWriter<'a, REG, Mask23select>;
impl<'a, REG> Mask23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask23_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask23select::Mask23_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask23_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask23select::Mask23_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask24select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask24_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask24_1 = 1,
}
impl From<Mask24select> for bool {
    #[inline(always)]
    fn from(variant: Mask24select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK24` reader - Peripheral Clock Enable Mask"]
pub type Mask24R = crate::BitReader<Mask24select>;
impl Mask24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask24select {
        match self.bits {
            false => Mask24select::Mask24_0,
            true => Mask24select::Mask24_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask24_0(&self) -> bool {
        *self == Mask24select::Mask24_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask24_1(&self) -> bool {
        *self == Mask24select::Mask24_1
    }
}
#[doc = "Field `MASK24` writer - Peripheral Clock Enable Mask"]
pub type Mask24W<'a, REG> = crate::BitWriter<'a, REG, Mask24select>;
impl<'a, REG> Mask24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask24_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask24select::Mask24_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask24_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask24select::Mask24_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask25select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask25_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask25_1 = 1,
}
impl From<Mask25select> for bool {
    #[inline(always)]
    fn from(variant: Mask25select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK25` reader - Peripheral Clock Enable Mask"]
pub type Mask25R = crate::BitReader<Mask25select>;
impl Mask25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask25select {
        match self.bits {
            false => Mask25select::Mask25_0,
            true => Mask25select::Mask25_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask25_0(&self) -> bool {
        *self == Mask25select::Mask25_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask25_1(&self) -> bool {
        *self == Mask25select::Mask25_1
    }
}
#[doc = "Field `MASK25` writer - Peripheral Clock Enable Mask"]
pub type Mask25W<'a, REG> = crate::BitWriter<'a, REG, Mask25select>;
impl<'a, REG> Mask25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask25_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask25select::Mask25_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask25_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask25select::Mask25_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask26select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask26_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask26_1 = 1,
}
impl From<Mask26select> for bool {
    #[inline(always)]
    fn from(variant: Mask26select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK26` reader - Peripheral Clock Enable Mask"]
pub type Mask26R = crate::BitReader<Mask26select>;
impl Mask26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask26select {
        match self.bits {
            false => Mask26select::Mask26_0,
            true => Mask26select::Mask26_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask26_0(&self) -> bool {
        *self == Mask26select::Mask26_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask26_1(&self) -> bool {
        *self == Mask26select::Mask26_1
    }
}
#[doc = "Field `MASK26` writer - Peripheral Clock Enable Mask"]
pub type Mask26W<'a, REG> = crate::BitWriter<'a, REG, Mask26select>;
impl<'a, REG> Mask26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask26_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask26select::Mask26_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask26_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask26select::Mask26_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask27select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask27_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask27_1 = 1,
}
impl From<Mask27select> for bool {
    #[inline(always)]
    fn from(variant: Mask27select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK27` reader - Peripheral Clock Enable Mask"]
pub type Mask27R = crate::BitReader<Mask27select>;
impl Mask27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask27select {
        match self.bits {
            false => Mask27select::Mask27_0,
            true => Mask27select::Mask27_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask27_0(&self) -> bool {
        *self == Mask27select::Mask27_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask27_1(&self) -> bool {
        *self == Mask27select::Mask27_1
    }
}
#[doc = "Field `MASK27` writer - Peripheral Clock Enable Mask"]
pub type Mask27W<'a, REG> = crate::BitWriter<'a, REG, Mask27select>;
impl<'a, REG> Mask27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask27_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask27select::Mask27_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask27_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask27select::Mask27_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask28select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask28_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask28_1 = 1,
}
impl From<Mask28select> for bool {
    #[inline(always)]
    fn from(variant: Mask28select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK28` reader - Peripheral Clock Enable Mask"]
pub type Mask28R = crate::BitReader<Mask28select>;
impl Mask28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask28select {
        match self.bits {
            false => Mask28select::Mask28_0,
            true => Mask28select::Mask28_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask28_0(&self) -> bool {
        *self == Mask28select::Mask28_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask28_1(&self) -> bool {
        *self == Mask28select::Mask28_1
    }
}
#[doc = "Field `MASK28` writer - Peripheral Clock Enable Mask"]
pub type Mask28W<'a, REG> = crate::BitWriter<'a, REG, Mask28select>;
impl<'a, REG> Mask28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask28_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask28select::Mask28_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask28_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask28select::Mask28_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask29select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask29_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask29_1 = 1,
}
impl From<Mask29select> for bool {
    #[inline(always)]
    fn from(variant: Mask29select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK29` reader - Peripheral Clock Enable Mask"]
pub type Mask29R = crate::BitReader<Mask29select>;
impl Mask29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask29select {
        match self.bits {
            false => Mask29select::Mask29_0,
            true => Mask29select::Mask29_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask29_0(&self) -> bool {
        *self == Mask29select::Mask29_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask29_1(&self) -> bool {
        *self == Mask29select::Mask29_1
    }
}
#[doc = "Field `MASK29` writer - Peripheral Clock Enable Mask"]
pub type Mask29W<'a, REG> = crate::BitWriter<'a, REG, Mask29select>;
impl<'a, REG> Mask29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask29_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask29select::Mask29_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask29_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask29select::Mask29_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask30select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask30_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask30_1 = 1,
}
impl From<Mask30select> for bool {
    #[inline(always)]
    fn from(variant: Mask30select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK30` reader - Peripheral Clock Enable Mask"]
pub type Mask30R = crate::BitReader<Mask30select>;
impl Mask30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask30select {
        match self.bits {
            false => Mask30select::Mask30_0,
            true => Mask30select::Mask30_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask30_0(&self) -> bool {
        *self == Mask30select::Mask30_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask30_1(&self) -> bool {
        *self == Mask30select::Mask30_1
    }
}
#[doc = "Field `MASK30` writer - Peripheral Clock Enable Mask"]
pub type Mask30W<'a, REG> = crate::BitWriter<'a, REG, Mask30select>;
impl<'a, REG> Mask30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask30_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask30select::Mask30_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask30_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask30select::Mask30_1)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask31select {
    #[doc = "0: Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    Mask31_0 = 0,
    #[doc = "1: Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    Mask31_1 = 1,
}
impl From<Mask31select> for bool {
    #[inline(always)]
    fn from(variant: Mask31select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK31` reader - Peripheral Clock Enable Mask"]
pub type Mask31R = crate::BitReader<Mask31select>;
impl Mask31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask31select {
        match self.bits {
            false => Mask31select::Mask31_0,
            true => Mask31select::Mask31_1,
        }
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn is_mask31_0(&self) -> bool {
        *self == Mask31select::Mask31_0
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn is_mask31_1(&self) -> bool {
        *self == Mask31select::Mask31_1
    }
}
#[doc = "Field `MASK31` writer - Peripheral Clock Enable Mask"]
pub type Mask31W<'a, REG> = crate::BitWriter<'a, REG, Mask31select>;
impl<'a, REG> Mask31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] stopped"]
    #[inline(always)]
    pub fn mask31_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mask31select::Mask31_0)
    }
    #[doc = "Peripheral clock mclk_clk_periph\\[n*32+x\\] enabled"]
    #[inline(always)]
    pub fn mask31_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mask31select::Mask31_1)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask0(&self) -> Mask0R {
        Mask0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask1(&self) -> Mask1R {
        Mask1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask2(&self) -> Mask2R {
        Mask2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask3(&self) -> Mask3R {
        Mask3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask4(&self) -> Mask4R {
        Mask4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask5(&self) -> Mask5R {
        Mask5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask6(&self) -> Mask6R {
        Mask6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask7(&self) -> Mask7R {
        Mask7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask8(&self) -> Mask8R {
        Mask8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask9(&self) -> Mask9R {
        Mask9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask10(&self) -> Mask10R {
        Mask10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask11(&self) -> Mask11R {
        Mask11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask12(&self) -> Mask12R {
        Mask12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask13(&self) -> Mask13R {
        Mask13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask14(&self) -> Mask14R {
        Mask14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask15(&self) -> Mask15R {
        Mask15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask16(&self) -> Mask16R {
        Mask16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask17(&self) -> Mask17R {
        Mask17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask18(&self) -> Mask18R {
        Mask18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask19(&self) -> Mask19R {
        Mask19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask20(&self) -> Mask20R {
        Mask20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask21(&self) -> Mask21R {
        Mask21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask22(&self) -> Mask22R {
        Mask22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask23(&self) -> Mask23R {
        Mask23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask24(&self) -> Mask24R {
        Mask24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask25(&self) -> Mask25R {
        Mask25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask26(&self) -> Mask26R {
        Mask26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask27(&self) -> Mask27R {
        Mask27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask28(&self) -> Mask28R {
        Mask28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask29(&self) -> Mask29R {
        Mask29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask30(&self) -> Mask30R {
        Mask30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask31(&self) -> Mask31R {
        Mask31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask0(&mut self) -> Mask0W<'_, ClkmskSpec> {
        Mask0W::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask1(&mut self) -> Mask1W<'_, ClkmskSpec> {
        Mask1W::new(self, 1)
    }
    #[doc = "Bit 2 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask2(&mut self) -> Mask2W<'_, ClkmskSpec> {
        Mask2W::new(self, 2)
    }
    #[doc = "Bit 3 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask3(&mut self) -> Mask3W<'_, ClkmskSpec> {
        Mask3W::new(self, 3)
    }
    #[doc = "Bit 4 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask4(&mut self) -> Mask4W<'_, ClkmskSpec> {
        Mask4W::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask5(&mut self) -> Mask5W<'_, ClkmskSpec> {
        Mask5W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask6(&mut self) -> Mask6W<'_, ClkmskSpec> {
        Mask6W::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask7(&mut self) -> Mask7W<'_, ClkmskSpec> {
        Mask7W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask8(&mut self) -> Mask8W<'_, ClkmskSpec> {
        Mask8W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask9(&mut self) -> Mask9W<'_, ClkmskSpec> {
        Mask9W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask10(&mut self) -> Mask10W<'_, ClkmskSpec> {
        Mask10W::new(self, 10)
    }
    #[doc = "Bit 11 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask11(&mut self) -> Mask11W<'_, ClkmskSpec> {
        Mask11W::new(self, 11)
    }
    #[doc = "Bit 12 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask12(&mut self) -> Mask12W<'_, ClkmskSpec> {
        Mask12W::new(self, 12)
    }
    #[doc = "Bit 13 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask13(&mut self) -> Mask13W<'_, ClkmskSpec> {
        Mask13W::new(self, 13)
    }
    #[doc = "Bit 14 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask14(&mut self) -> Mask14W<'_, ClkmskSpec> {
        Mask14W::new(self, 14)
    }
    #[doc = "Bit 15 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask15(&mut self) -> Mask15W<'_, ClkmskSpec> {
        Mask15W::new(self, 15)
    }
    #[doc = "Bit 16 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask16(&mut self) -> Mask16W<'_, ClkmskSpec> {
        Mask16W::new(self, 16)
    }
    #[doc = "Bit 17 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask17(&mut self) -> Mask17W<'_, ClkmskSpec> {
        Mask17W::new(self, 17)
    }
    #[doc = "Bit 18 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask18(&mut self) -> Mask18W<'_, ClkmskSpec> {
        Mask18W::new(self, 18)
    }
    #[doc = "Bit 19 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask19(&mut self) -> Mask19W<'_, ClkmskSpec> {
        Mask19W::new(self, 19)
    }
    #[doc = "Bit 20 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask20(&mut self) -> Mask20W<'_, ClkmskSpec> {
        Mask20W::new(self, 20)
    }
    #[doc = "Bit 21 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask21(&mut self) -> Mask21W<'_, ClkmskSpec> {
        Mask21W::new(self, 21)
    }
    #[doc = "Bit 22 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask22(&mut self) -> Mask22W<'_, ClkmskSpec> {
        Mask22W::new(self, 22)
    }
    #[doc = "Bit 23 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask23(&mut self) -> Mask23W<'_, ClkmskSpec> {
        Mask23W::new(self, 23)
    }
    #[doc = "Bit 24 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask24(&mut self) -> Mask24W<'_, ClkmskSpec> {
        Mask24W::new(self, 24)
    }
    #[doc = "Bit 25 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask25(&mut self) -> Mask25W<'_, ClkmskSpec> {
        Mask25W::new(self, 25)
    }
    #[doc = "Bit 26 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask26(&mut self) -> Mask26W<'_, ClkmskSpec> {
        Mask26W::new(self, 26)
    }
    #[doc = "Bit 27 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask27(&mut self) -> Mask27W<'_, ClkmskSpec> {
        Mask27W::new(self, 27)
    }
    #[doc = "Bit 28 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask28(&mut self) -> Mask28W<'_, ClkmskSpec> {
        Mask28W::new(self, 28)
    }
    #[doc = "Bit 29 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask29(&mut self) -> Mask29W<'_, ClkmskSpec> {
        Mask29W::new(self, 29)
    }
    #[doc = "Bit 30 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask30(&mut self) -> Mask30W<'_, ClkmskSpec> {
        Mask30W::new(self, 30)
    }
    #[doc = "Bit 31 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask31(&mut self) -> Mask31W<'_, ClkmskSpec> {
        Mask31W::new(self, 31)
    }
}
#[doc = "Peripheral Clock Enable Mask n Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkmskSpec;
impl crate::RegisterSpec for ClkmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkmsk::R`](R) reader structure"]
impl crate::Readable for ClkmskSpec {}
#[doc = "`write(|w| ..)` method takes [`clkmsk::W`](W) writer structure"]
impl crate::Writable for ClkmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKMSK[%s] to value 0"]
impl crate::Resettable for ClkmskSpec {}

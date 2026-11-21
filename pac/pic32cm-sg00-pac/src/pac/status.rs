#[doc = "Register `STATUS[%s]` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid0select {
    #[doc = "0: Write Protection disabled"]
    Perid0_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid0_1 = 1,
}
impl From<Perid0select> for bool {
    #[inline(always)]
    fn from(variant: Perid0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID0` reader - PERID Write Protection Status"]
pub type Perid0R = crate::BitReader<Perid0select>;
impl Perid0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid0select {
        match self.bits {
            false => Perid0select::Perid0_0,
            true => Perid0select::Perid0_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid0_0(&self) -> bool {
        *self == Perid0select::Perid0_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid0_1(&self) -> bool {
        *self == Perid0select::Perid0_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid1select {
    #[doc = "0: Write Protection disabled"]
    Perid1_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid1_1 = 1,
}
impl From<Perid1select> for bool {
    #[inline(always)]
    fn from(variant: Perid1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID1` reader - PERID Write Protection Status"]
pub type Perid1R = crate::BitReader<Perid1select>;
impl Perid1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid1select {
        match self.bits {
            false => Perid1select::Perid1_0,
            true => Perid1select::Perid1_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid1_0(&self) -> bool {
        *self == Perid1select::Perid1_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid1_1(&self) -> bool {
        *self == Perid1select::Perid1_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid2select {
    #[doc = "0: Write Protection disabled"]
    Perid2_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid2_1 = 1,
}
impl From<Perid2select> for bool {
    #[inline(always)]
    fn from(variant: Perid2select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID2` reader - PERID Write Protection Status"]
pub type Perid2R = crate::BitReader<Perid2select>;
impl Perid2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid2select {
        match self.bits {
            false => Perid2select::Perid2_0,
            true => Perid2select::Perid2_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid2_0(&self) -> bool {
        *self == Perid2select::Perid2_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid2_1(&self) -> bool {
        *self == Perid2select::Perid2_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid3select {
    #[doc = "0: Write Protection disabled"]
    Perid3_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid3_1 = 1,
}
impl From<Perid3select> for bool {
    #[inline(always)]
    fn from(variant: Perid3select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID3` reader - PERID Write Protection Status"]
pub type Perid3R = crate::BitReader<Perid3select>;
impl Perid3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid3select {
        match self.bits {
            false => Perid3select::Perid3_0,
            true => Perid3select::Perid3_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid3_0(&self) -> bool {
        *self == Perid3select::Perid3_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid3_1(&self) -> bool {
        *self == Perid3select::Perid3_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid4select {
    #[doc = "0: Write Protection disabled"]
    Perid4_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid4_1 = 1,
}
impl From<Perid4select> for bool {
    #[inline(always)]
    fn from(variant: Perid4select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID4` reader - PERID Write Protection Status"]
pub type Perid4R = crate::BitReader<Perid4select>;
impl Perid4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid4select {
        match self.bits {
            false => Perid4select::Perid4_0,
            true => Perid4select::Perid4_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid4_0(&self) -> bool {
        *self == Perid4select::Perid4_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid4_1(&self) -> bool {
        *self == Perid4select::Perid4_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid5select {
    #[doc = "0: Write Protection disabled"]
    Perid5_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid5_1 = 1,
}
impl From<Perid5select> for bool {
    #[inline(always)]
    fn from(variant: Perid5select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID5` reader - PERID Write Protection Status"]
pub type Perid5R = crate::BitReader<Perid5select>;
impl Perid5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid5select {
        match self.bits {
            false => Perid5select::Perid5_0,
            true => Perid5select::Perid5_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid5_0(&self) -> bool {
        *self == Perid5select::Perid5_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid5_1(&self) -> bool {
        *self == Perid5select::Perid5_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid6select {
    #[doc = "0: Write Protection disabled"]
    Perid6_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid6_1 = 1,
}
impl From<Perid6select> for bool {
    #[inline(always)]
    fn from(variant: Perid6select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID6` reader - PERID Write Protection Status"]
pub type Perid6R = crate::BitReader<Perid6select>;
impl Perid6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid6select {
        match self.bits {
            false => Perid6select::Perid6_0,
            true => Perid6select::Perid6_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid6_0(&self) -> bool {
        *self == Perid6select::Perid6_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid6_1(&self) -> bool {
        *self == Perid6select::Perid6_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid7select {
    #[doc = "0: Write Protection disabled"]
    Perid7_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid7_1 = 1,
}
impl From<Perid7select> for bool {
    #[inline(always)]
    fn from(variant: Perid7select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID7` reader - PERID Write Protection Status"]
pub type Perid7R = crate::BitReader<Perid7select>;
impl Perid7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid7select {
        match self.bits {
            false => Perid7select::Perid7_0,
            true => Perid7select::Perid7_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid7_0(&self) -> bool {
        *self == Perid7select::Perid7_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid7_1(&self) -> bool {
        *self == Perid7select::Perid7_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid8select {
    #[doc = "0: Write Protection disabled"]
    Perid8_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid8_1 = 1,
}
impl From<Perid8select> for bool {
    #[inline(always)]
    fn from(variant: Perid8select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID8` reader - PERID Write Protection Status"]
pub type Perid8R = crate::BitReader<Perid8select>;
impl Perid8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid8select {
        match self.bits {
            false => Perid8select::Perid8_0,
            true => Perid8select::Perid8_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid8_0(&self) -> bool {
        *self == Perid8select::Perid8_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid8_1(&self) -> bool {
        *self == Perid8select::Perid8_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid9select {
    #[doc = "0: Write Protection disabled"]
    Perid9_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid9_1 = 1,
}
impl From<Perid9select> for bool {
    #[inline(always)]
    fn from(variant: Perid9select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID9` reader - PERID Write Protection Status"]
pub type Perid9R = crate::BitReader<Perid9select>;
impl Perid9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid9select {
        match self.bits {
            false => Perid9select::Perid9_0,
            true => Perid9select::Perid9_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid9_0(&self) -> bool {
        *self == Perid9select::Perid9_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid9_1(&self) -> bool {
        *self == Perid9select::Perid9_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid10select {
    #[doc = "0: Write Protection disabled"]
    Perid10_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid10_1 = 1,
}
impl From<Perid10select> for bool {
    #[inline(always)]
    fn from(variant: Perid10select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID10` reader - PERID Write Protection Status"]
pub type Perid10R = crate::BitReader<Perid10select>;
impl Perid10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid10select {
        match self.bits {
            false => Perid10select::Perid10_0,
            true => Perid10select::Perid10_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid10_0(&self) -> bool {
        *self == Perid10select::Perid10_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid10_1(&self) -> bool {
        *self == Perid10select::Perid10_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid11select {
    #[doc = "0: Write Protection disabled"]
    Perid11_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid11_1 = 1,
}
impl From<Perid11select> for bool {
    #[inline(always)]
    fn from(variant: Perid11select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID11` reader - PERID Write Protection Status"]
pub type Perid11R = crate::BitReader<Perid11select>;
impl Perid11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid11select {
        match self.bits {
            false => Perid11select::Perid11_0,
            true => Perid11select::Perid11_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid11_0(&self) -> bool {
        *self == Perid11select::Perid11_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid11_1(&self) -> bool {
        *self == Perid11select::Perid11_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid12select {
    #[doc = "0: Write Protection disabled"]
    Perid12_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid12_1 = 1,
}
impl From<Perid12select> for bool {
    #[inline(always)]
    fn from(variant: Perid12select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID12` reader - PERID Write Protection Status"]
pub type Perid12R = crate::BitReader<Perid12select>;
impl Perid12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid12select {
        match self.bits {
            false => Perid12select::Perid12_0,
            true => Perid12select::Perid12_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid12_0(&self) -> bool {
        *self == Perid12select::Perid12_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid12_1(&self) -> bool {
        *self == Perid12select::Perid12_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid13select {
    #[doc = "0: Write Protection disabled"]
    Perid13_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid13_1 = 1,
}
impl From<Perid13select> for bool {
    #[inline(always)]
    fn from(variant: Perid13select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID13` reader - PERID Write Protection Status"]
pub type Perid13R = crate::BitReader<Perid13select>;
impl Perid13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid13select {
        match self.bits {
            false => Perid13select::Perid13_0,
            true => Perid13select::Perid13_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid13_0(&self) -> bool {
        *self == Perid13select::Perid13_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid13_1(&self) -> bool {
        *self == Perid13select::Perid13_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid14select {
    #[doc = "0: Write Protection disabled"]
    Perid14_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid14_1 = 1,
}
impl From<Perid14select> for bool {
    #[inline(always)]
    fn from(variant: Perid14select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID14` reader - PERID Write Protection Status"]
pub type Perid14R = crate::BitReader<Perid14select>;
impl Perid14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid14select {
        match self.bits {
            false => Perid14select::Perid14_0,
            true => Perid14select::Perid14_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid14_0(&self) -> bool {
        *self == Perid14select::Perid14_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid14_1(&self) -> bool {
        *self == Perid14select::Perid14_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid15select {
    #[doc = "0: Write Protection disabled"]
    Perid15_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid15_1 = 1,
}
impl From<Perid15select> for bool {
    #[inline(always)]
    fn from(variant: Perid15select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID15` reader - PERID Write Protection Status"]
pub type Perid15R = crate::BitReader<Perid15select>;
impl Perid15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid15select {
        match self.bits {
            false => Perid15select::Perid15_0,
            true => Perid15select::Perid15_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid15_0(&self) -> bool {
        *self == Perid15select::Perid15_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid15_1(&self) -> bool {
        *self == Perid15select::Perid15_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid16select {
    #[doc = "0: Write Protection disabled"]
    Perid16_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid16_1 = 1,
}
impl From<Perid16select> for bool {
    #[inline(always)]
    fn from(variant: Perid16select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID16` reader - PERID Write Protection Status"]
pub type Perid16R = crate::BitReader<Perid16select>;
impl Perid16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid16select {
        match self.bits {
            false => Perid16select::Perid16_0,
            true => Perid16select::Perid16_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid16_0(&self) -> bool {
        *self == Perid16select::Perid16_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid16_1(&self) -> bool {
        *self == Perid16select::Perid16_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid17select {
    #[doc = "0: Write Protection disabled"]
    Perid17_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid17_1 = 1,
}
impl From<Perid17select> for bool {
    #[inline(always)]
    fn from(variant: Perid17select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID17` reader - PERID Write Protection Status"]
pub type Perid17R = crate::BitReader<Perid17select>;
impl Perid17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid17select {
        match self.bits {
            false => Perid17select::Perid17_0,
            true => Perid17select::Perid17_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid17_0(&self) -> bool {
        *self == Perid17select::Perid17_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid17_1(&self) -> bool {
        *self == Perid17select::Perid17_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid18select {
    #[doc = "0: Write Protection disabled"]
    Perid18_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid18_1 = 1,
}
impl From<Perid18select> for bool {
    #[inline(always)]
    fn from(variant: Perid18select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID18` reader - PERID Write Protection Status"]
pub type Perid18R = crate::BitReader<Perid18select>;
impl Perid18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid18select {
        match self.bits {
            false => Perid18select::Perid18_0,
            true => Perid18select::Perid18_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid18_0(&self) -> bool {
        *self == Perid18select::Perid18_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid18_1(&self) -> bool {
        *self == Perid18select::Perid18_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid19select {
    #[doc = "0: Write Protection disabled"]
    Perid19_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid19_1 = 1,
}
impl From<Perid19select> for bool {
    #[inline(always)]
    fn from(variant: Perid19select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID19` reader - PERID Write Protection Status"]
pub type Perid19R = crate::BitReader<Perid19select>;
impl Perid19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid19select {
        match self.bits {
            false => Perid19select::Perid19_0,
            true => Perid19select::Perid19_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid19_0(&self) -> bool {
        *self == Perid19select::Perid19_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid19_1(&self) -> bool {
        *self == Perid19select::Perid19_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid20select {
    #[doc = "0: Write Protection disabled"]
    Perid20_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid20_1 = 1,
}
impl From<Perid20select> for bool {
    #[inline(always)]
    fn from(variant: Perid20select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID20` reader - PERID Write Protection Status"]
pub type Perid20R = crate::BitReader<Perid20select>;
impl Perid20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid20select {
        match self.bits {
            false => Perid20select::Perid20_0,
            true => Perid20select::Perid20_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid20_0(&self) -> bool {
        *self == Perid20select::Perid20_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid20_1(&self) -> bool {
        *self == Perid20select::Perid20_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid21select {
    #[doc = "0: Write Protection disabled"]
    Perid21_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid21_1 = 1,
}
impl From<Perid21select> for bool {
    #[inline(always)]
    fn from(variant: Perid21select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID21` reader - PERID Write Protection Status"]
pub type Perid21R = crate::BitReader<Perid21select>;
impl Perid21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid21select {
        match self.bits {
            false => Perid21select::Perid21_0,
            true => Perid21select::Perid21_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid21_0(&self) -> bool {
        *self == Perid21select::Perid21_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid21_1(&self) -> bool {
        *self == Perid21select::Perid21_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid22select {
    #[doc = "0: Write Protection disabled"]
    Perid22_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid22_1 = 1,
}
impl From<Perid22select> for bool {
    #[inline(always)]
    fn from(variant: Perid22select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID22` reader - PERID Write Protection Status"]
pub type Perid22R = crate::BitReader<Perid22select>;
impl Perid22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid22select {
        match self.bits {
            false => Perid22select::Perid22_0,
            true => Perid22select::Perid22_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid22_0(&self) -> bool {
        *self == Perid22select::Perid22_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid22_1(&self) -> bool {
        *self == Perid22select::Perid22_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid23select {
    #[doc = "0: Write Protection disabled"]
    Perid23_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid23_1 = 1,
}
impl From<Perid23select> for bool {
    #[inline(always)]
    fn from(variant: Perid23select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID23` reader - PERID Write Protection Status"]
pub type Perid23R = crate::BitReader<Perid23select>;
impl Perid23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid23select {
        match self.bits {
            false => Perid23select::Perid23_0,
            true => Perid23select::Perid23_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid23_0(&self) -> bool {
        *self == Perid23select::Perid23_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid23_1(&self) -> bool {
        *self == Perid23select::Perid23_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid24select {
    #[doc = "0: Write Protection disabled"]
    Perid24_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid24_1 = 1,
}
impl From<Perid24select> for bool {
    #[inline(always)]
    fn from(variant: Perid24select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID24` reader - PERID Write Protection Status"]
pub type Perid24R = crate::BitReader<Perid24select>;
impl Perid24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid24select {
        match self.bits {
            false => Perid24select::Perid24_0,
            true => Perid24select::Perid24_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid24_0(&self) -> bool {
        *self == Perid24select::Perid24_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid24_1(&self) -> bool {
        *self == Perid24select::Perid24_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid25select {
    #[doc = "0: Write Protection disabled"]
    Perid25_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid25_1 = 1,
}
impl From<Perid25select> for bool {
    #[inline(always)]
    fn from(variant: Perid25select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID25` reader - PERID Write Protection Status"]
pub type Perid25R = crate::BitReader<Perid25select>;
impl Perid25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid25select {
        match self.bits {
            false => Perid25select::Perid25_0,
            true => Perid25select::Perid25_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid25_0(&self) -> bool {
        *self == Perid25select::Perid25_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid25_1(&self) -> bool {
        *self == Perid25select::Perid25_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid26select {
    #[doc = "0: Write Protection disabled"]
    Perid26_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid26_1 = 1,
}
impl From<Perid26select> for bool {
    #[inline(always)]
    fn from(variant: Perid26select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID26` reader - PERID Write Protection Status"]
pub type Perid26R = crate::BitReader<Perid26select>;
impl Perid26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid26select {
        match self.bits {
            false => Perid26select::Perid26_0,
            true => Perid26select::Perid26_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid26_0(&self) -> bool {
        *self == Perid26select::Perid26_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid26_1(&self) -> bool {
        *self == Perid26select::Perid26_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid27select {
    #[doc = "0: Write Protection disabled"]
    Perid27_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid27_1 = 1,
}
impl From<Perid27select> for bool {
    #[inline(always)]
    fn from(variant: Perid27select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID27` reader - PERID Write Protection Status"]
pub type Perid27R = crate::BitReader<Perid27select>;
impl Perid27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid27select {
        match self.bits {
            false => Perid27select::Perid27_0,
            true => Perid27select::Perid27_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid27_0(&self) -> bool {
        *self == Perid27select::Perid27_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid27_1(&self) -> bool {
        *self == Perid27select::Perid27_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid28select {
    #[doc = "0: Write Protection disabled"]
    Perid28_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid28_1 = 1,
}
impl From<Perid28select> for bool {
    #[inline(always)]
    fn from(variant: Perid28select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID28` reader - PERID Write Protection Status"]
pub type Perid28R = crate::BitReader<Perid28select>;
impl Perid28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid28select {
        match self.bits {
            false => Perid28select::Perid28_0,
            true => Perid28select::Perid28_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid28_0(&self) -> bool {
        *self == Perid28select::Perid28_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid28_1(&self) -> bool {
        *self == Perid28select::Perid28_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid29select {
    #[doc = "0: Write Protection disabled"]
    Perid29_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid29_1 = 1,
}
impl From<Perid29select> for bool {
    #[inline(always)]
    fn from(variant: Perid29select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID29` reader - PERID Write Protection Status"]
pub type Perid29R = crate::BitReader<Perid29select>;
impl Perid29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid29select {
        match self.bits {
            false => Perid29select::Perid29_0,
            true => Perid29select::Perid29_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid29_0(&self) -> bool {
        *self == Perid29select::Perid29_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid29_1(&self) -> bool {
        *self == Perid29select::Perid29_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid30select {
    #[doc = "0: Write Protection disabled"]
    Perid30_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid30_1 = 1,
}
impl From<Perid30select> for bool {
    #[inline(always)]
    fn from(variant: Perid30select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID30` reader - PERID Write Protection Status"]
pub type Perid30R = crate::BitReader<Perid30select>;
impl Perid30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid30select {
        match self.bits {
            false => Perid30select::Perid30_0,
            true => Perid30select::Perid30_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid30_0(&self) -> bool {
        *self == Perid30select::Perid30_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid30_1(&self) -> bool {
        *self == Perid30select::Perid30_1
    }
}
#[doc = "PERID Write Protection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perid31select {
    #[doc = "0: Write Protection disabled"]
    Perid31_0 = 0,
    #[doc = "1: Write Protection enabled"]
    Perid31_1 = 1,
}
impl From<Perid31select> for bool {
    #[inline(always)]
    fn from(variant: Perid31select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERID31` reader - PERID Write Protection Status"]
pub type Perid31R = crate::BitReader<Perid31select>;
impl Perid31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perid31select {
        match self.bits {
            false => Perid31select::Perid31_0,
            true => Perid31select::Perid31_1,
        }
    }
    #[doc = "Write Protection disabled"]
    #[inline(always)]
    pub fn is_perid31_0(&self) -> bool {
        *self == Perid31select::Perid31_0
    }
    #[doc = "Write Protection enabled"]
    #[inline(always)]
    pub fn is_perid31_1(&self) -> bool {
        *self == Perid31select::Perid31_1
    }
}
impl R {
    #[doc = "Bit 0 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid0(&self) -> Perid0R {
        Perid0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid1(&self) -> Perid1R {
        Perid1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid2(&self) -> Perid2R {
        Perid2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid3(&self) -> Perid3R {
        Perid3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid4(&self) -> Perid4R {
        Perid4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid5(&self) -> Perid5R {
        Perid5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid6(&self) -> Perid6R {
        Perid6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid7(&self) -> Perid7R {
        Perid7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid8(&self) -> Perid8R {
        Perid8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid9(&self) -> Perid9R {
        Perid9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid10(&self) -> Perid10R {
        Perid10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid11(&self) -> Perid11R {
        Perid11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid12(&self) -> Perid12R {
        Perid12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid13(&self) -> Perid13R {
        Perid13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid14(&self) -> Perid14R {
        Perid14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid15(&self) -> Perid15R {
        Perid15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid16(&self) -> Perid16R {
        Perid16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid17(&self) -> Perid17R {
        Perid17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid18(&self) -> Perid18R {
        Perid18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid19(&self) -> Perid19R {
        Perid19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid20(&self) -> Perid20R {
        Perid20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid21(&self) -> Perid21R {
        Perid21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid22(&self) -> Perid22R {
        Perid22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid23(&self) -> Perid23R {
        Perid23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid24(&self) -> Perid24R {
        Perid24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid25(&self) -> Perid25R {
        Perid25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid26(&self) -> Perid26R {
        Perid26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid27(&self) -> Perid27R {
        Perid27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid28(&self) -> Perid28R {
        Perid28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid29(&self) -> Perid29R {
        Perid29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid30(&self) -> Perid30R {
        Perid30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PERID Write Protection Status"]
    #[inline(always)]
    pub fn perid31(&self) -> Perid31R {
        Perid31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Peripheral Status n Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS[%s] to value 0"]
impl crate::Resettable for StatusSpec {}

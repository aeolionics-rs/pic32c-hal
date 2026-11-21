#[doc = "Register `STATUSB` reader"]
pub type R = crate::R<StatusbSpec>;
#[doc = "Boot ROM Communication Channel 0 Dirty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bccd0select {
    #[doc = "0: BCCDx.DATA register was read from."]
    Bccd0_0 = 0,
    #[doc = "1: BCCDx.DATA register was written to."]
    Bccd0_1 = 1,
}
impl From<Bccd0select> for bool {
    #[inline(always)]
    fn from(variant: Bccd0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCCD0` reader - Boot ROM Communication Channel 0 Dirty"]
pub type Bccd0R = crate::BitReader<Bccd0select>;
impl Bccd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bccd0select {
        match self.bits {
            false => Bccd0select::Bccd0_0,
            true => Bccd0select::Bccd0_1,
        }
    }
    #[doc = "BCCDx.DATA register was read from."]
    #[inline(always)]
    pub fn is_bccd0_0(&self) -> bool {
        *self == Bccd0select::Bccd0_0
    }
    #[doc = "BCCDx.DATA register was written to."]
    #[inline(always)]
    pub fn is_bccd0_1(&self) -> bool {
        *self == Bccd0select::Bccd0_1
    }
}
#[doc = "Boot ROM Communication Channel 1 Dirty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bccd1select {
    #[doc = "0: BCCDx.DATA register was read from."]
    Bccd1_0 = 0,
    #[doc = "1: BCCDx.DATA register was written to."]
    Bccd1_1 = 1,
}
impl From<Bccd1select> for bool {
    #[inline(always)]
    fn from(variant: Bccd1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCCD1` reader - Boot ROM Communication Channel 1 Dirty"]
pub type Bccd1R = crate::BitReader<Bccd1select>;
impl Bccd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bccd1select {
        match self.bits {
            false => Bccd1select::Bccd1_0,
            true => Bccd1select::Bccd1_1,
        }
    }
    #[doc = "BCCDx.DATA register was read from."]
    #[inline(always)]
    pub fn is_bccd1_0(&self) -> bool {
        *self == Bccd1select::Bccd1_0
    }
    #[doc = "BCCDx.DATA register was written to."]
    #[inline(always)]
    pub fn is_bccd1_1(&self) -> bool {
        *self == Bccd1select::Bccd1_1
    }
}
#[doc = "Debug Communication Channel 0 Dirty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dccd0select {
    #[doc = "0: DCCDx.DATA register was read from."]
    Dccd0_0 = 0,
    #[doc = "1: DCCDx.DATA register was written to."]
    Dccd0_1 = 1,
}
impl From<Dccd0select> for bool {
    #[inline(always)]
    fn from(variant: Dccd0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCCD0` reader - Debug Communication Channel 0 Dirty"]
pub type Dccd0R = crate::BitReader<Dccd0select>;
impl Dccd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dccd0select {
        match self.bits {
            false => Dccd0select::Dccd0_0,
            true => Dccd0select::Dccd0_1,
        }
    }
    #[doc = "DCCDx.DATA register was read from."]
    #[inline(always)]
    pub fn is_dccd0_0(&self) -> bool {
        *self == Dccd0select::Dccd0_0
    }
    #[doc = "DCCDx.DATA register was written to."]
    #[inline(always)]
    pub fn is_dccd0_1(&self) -> bool {
        *self == Dccd0select::Dccd0_1
    }
}
#[doc = "Debug Communication Channel 1 Dirty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dccd1select {
    #[doc = "0: DCCDx.DATA register was read from."]
    Dccd1_0 = 0,
    #[doc = "1: DCCDx.DATA register was written to."]
    Dccd1_1 = 1,
}
impl From<Dccd1select> for bool {
    #[inline(always)]
    fn from(variant: Dccd1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCCD1` reader - Debug Communication Channel 1 Dirty"]
pub type Dccd1R = crate::BitReader<Dccd1select>;
impl Dccd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dccd1select {
        match self.bits {
            false => Dccd1select::Dccd1_0,
            true => Dccd1select::Dccd1_1,
        }
    }
    #[doc = "DCCDx.DATA register was read from."]
    #[inline(always)]
    pub fn is_dccd1_0(&self) -> bool {
        *self == Dccd1select::Dccd1_0
    }
    #[doc = "DCCDx.DATA register was written to."]
    #[inline(always)]
    pub fn is_dccd1_1(&self) -> bool {
        *self == Dccd1select::Dccd1_1
    }
}
#[doc = "Debugger Present Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgpresselect {
    #[doc = "0: No debugger detected"]
    Dbgpres0 = 0,
    #[doc = "1: Debugger probe detected"]
    Dbgpres1 = 1,
}
impl From<Dbgpresselect> for bool {
    #[inline(always)]
    fn from(variant: Dbgpresselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGPRES` reader - Debugger Present Status"]
pub type DbgpresR = crate::BitReader<Dbgpresselect>;
impl DbgpresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgpresselect {
        match self.bits {
            false => Dbgpresselect::Dbgpres0,
            true => Dbgpresselect::Dbgpres1,
        }
    }
    #[doc = "No debugger detected"]
    #[inline(always)]
    pub fn is_dbgpres_0(&self) -> bool {
        *self == Dbgpresselect::Dbgpres0
    }
    #[doc = "Debugger probe detected"]
    #[inline(always)]
    pub fn is_dbgpres_1(&self) -> bool {
        *self == Dbgpresselect::Dbgpres1
    }
}
#[doc = "Hot-Plugging Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hpsselect {
    #[doc = "0: Hot-Plugging inactive"]
    Hps0 = 0,
    #[doc = "1: Hot-Plugging active"]
    Hps1 = 1,
}
impl From<Hpsselect> for bool {
    #[inline(always)]
    fn from(variant: Hpsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPS` reader - Hot-Plugging Status"]
pub type HpsR = crate::BitReader<Hpsselect>;
impl HpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hpsselect {
        match self.bits {
            false => Hpsselect::Hps0,
            true => Hpsselect::Hps1,
        }
    }
    #[doc = "Hot-Plugging inactive"]
    #[inline(always)]
    pub fn is_hps_0(&self) -> bool {
        *self == Hpsselect::Hps0
    }
    #[doc = "Hot-Plugging active"]
    #[inline(always)]
    pub fn is_hps_1(&self) -> bool {
        *self == Hpsselect::Hps1
    }
}
impl R {
    #[doc = "Bit 0 - Boot ROM Communication Channel 0 Dirty"]
    #[inline(always)]
    pub fn bccd0(&self) -> Bccd0R {
        Bccd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boot ROM Communication Channel 1 Dirty"]
    #[inline(always)]
    pub fn bccd1(&self) -> Bccd1R {
        Bccd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Communication Channel 0 Dirty"]
    #[inline(always)]
    pub fn dccd0(&self) -> Dccd0R {
        Dccd0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Communication Channel 1 Dirty"]
    #[inline(always)]
    pub fn dccd1(&self) -> Dccd1R {
        Dccd1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Debugger Present Status"]
    #[inline(always)]
    pub fn dbgpres(&self) -> DbgpresR {
        DbgpresR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Hot-Plugging Status"]
    #[inline(always)]
    pub fn hps(&self) -> HpsR {
        HpsR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Status B REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`statusb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusbSpec;
impl crate::RegisterSpec for StatusbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusb::R`](R) reader structure"]
impl crate::Readable for StatusbSpec {}
#[doc = "`reset()` method sets STATUSB to value 0"]
impl crate::Resettable for StatusbSpec {}

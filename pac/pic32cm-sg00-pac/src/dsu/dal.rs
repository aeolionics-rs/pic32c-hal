#[doc = "Register `DAL` reader"]
pub type R = crate::R<DalSpec>;
#[doc = "CPU 0 Debugger Access Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu0select {
    #[doc = "0: Debugger targeting CPU0 domain can only access the DSU external address space otherwise debugger access is disabled"]
    Secured = 0,
    #[doc = "1: Debugger can access only non-secure regions"]
    NsDebug = 1,
    #[doc = "2: Debugger can access secure and non-secure regions"]
    FullDebug = 2,
}
impl From<Cpu0select> for u8 {
    #[inline(always)]
    fn from(variant: Cpu0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu0select {
    type Ux = u8;
}
impl crate::IsEnum for Cpu0select {}
#[doc = "Field `CPU0` reader - CPU 0 Debugger Access Level"]
pub type Cpu0R = crate::FieldReader<Cpu0select>;
impl Cpu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu0select> {
        match self.bits {
            0 => Some(Cpu0select::Secured),
            1 => Some(Cpu0select::NsDebug),
            2 => Some(Cpu0select::FullDebug),
            _ => None,
        }
    }
    #[doc = "Debugger targeting CPU0 domain can only access the DSU external address space otherwise debugger access is disabled"]
    #[inline(always)]
    pub fn is_secured(&self) -> bool {
        *self == Cpu0select::Secured
    }
    #[doc = "Debugger can access only non-secure regions"]
    #[inline(always)]
    pub fn is_ns_debug(&self) -> bool {
        *self == Cpu0select::NsDebug
    }
    #[doc = "Debugger can access secure and non-secure regions"]
    #[inline(always)]
    pub fn is_full_debug(&self) -> bool {
        *self == Cpu0select::FullDebug
    }
}
#[doc = "CPU n Debugger Access Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu1select {
    #[doc = "0: Debugger access is disabled"]
    Secured = 0,
    #[doc = "1: Debugger can access only non-secure regions"]
    NsDebug = 1,
    #[doc = "2: Debugger has full access to debug CPUn"]
    FullDebug = 2,
    #[doc = "3: Unimplemented CPU"]
    Unimplemented = 3,
}
impl From<Cpu1select> for u8 {
    #[inline(always)]
    fn from(variant: Cpu1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu1select {
    type Ux = u8;
}
impl crate::IsEnum for Cpu1select {}
#[doc = "Field `CPU1` reader - CPU n Debugger Access Level"]
pub type Cpu1R = crate::FieldReader<Cpu1select>;
impl Cpu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1select {
        match self.bits {
            0 => Cpu1select::Secured,
            1 => Cpu1select::NsDebug,
            2 => Cpu1select::FullDebug,
            3 => Cpu1select::Unimplemented,
            _ => unreachable!(),
        }
    }
    #[doc = "Debugger access is disabled"]
    #[inline(always)]
    pub fn is_secured(&self) -> bool {
        *self == Cpu1select::Secured
    }
    #[doc = "Debugger can access only non-secure regions"]
    #[inline(always)]
    pub fn is_ns_debug(&self) -> bool {
        *self == Cpu1select::NsDebug
    }
    #[doc = "Debugger has full access to debug CPUn"]
    #[inline(always)]
    pub fn is_full_debug(&self) -> bool {
        *self == Cpu1select::FullDebug
    }
    #[doc = "Unimplemented CPU"]
    #[inline(always)]
    pub fn is_unimplemented(&self) -> bool {
        *self == Cpu1select::Unimplemented
    }
}
#[doc = "CPU n Debugger Access Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu2select {
    #[doc = "0: Debugger access is disabled"]
    Secured = 0,
    #[doc = "1: Debugger can access only non-secure regions"]
    NsDebug = 1,
    #[doc = "2: Debugger has full access to debug CPUn"]
    FullDebug = 2,
    #[doc = "3: Unimplemented CPU"]
    Unimplemented = 3,
}
impl From<Cpu2select> for u8 {
    #[inline(always)]
    fn from(variant: Cpu2select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu2select {
    type Ux = u8;
}
impl crate::IsEnum for Cpu2select {}
#[doc = "Field `CPU2` reader - CPU n Debugger Access Level"]
pub type Cpu2R = crate::FieldReader<Cpu2select>;
impl Cpu2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu2select {
        match self.bits {
            0 => Cpu2select::Secured,
            1 => Cpu2select::NsDebug,
            2 => Cpu2select::FullDebug,
            3 => Cpu2select::Unimplemented,
            _ => unreachable!(),
        }
    }
    #[doc = "Debugger access is disabled"]
    #[inline(always)]
    pub fn is_secured(&self) -> bool {
        *self == Cpu2select::Secured
    }
    #[doc = "Debugger can access only non-secure regions"]
    #[inline(always)]
    pub fn is_ns_debug(&self) -> bool {
        *self == Cpu2select::NsDebug
    }
    #[doc = "Debugger has full access to debug CPUn"]
    #[inline(always)]
    pub fn is_full_debug(&self) -> bool {
        *self == Cpu2select::FullDebug
    }
    #[doc = "Unimplemented CPU"]
    #[inline(always)]
    pub fn is_unimplemented(&self) -> bool {
        *self == Cpu2select::Unimplemented
    }
}
#[doc = "CPU n Debugger Access Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu3select {
    #[doc = "0: Debugger access is disabled"]
    Secured = 0,
    #[doc = "1: Debugger can access only non-secure regions"]
    NsDebug = 1,
    #[doc = "2: Debugger has full access to debug CPUn"]
    FullDebug = 2,
    #[doc = "3: Unimplemented CPU"]
    Unimplemented = 3,
}
impl From<Cpu3select> for u8 {
    #[inline(always)]
    fn from(variant: Cpu3select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu3select {
    type Ux = u8;
}
impl crate::IsEnum for Cpu3select {}
#[doc = "Field `CPU3` reader - CPU n Debugger Access Level"]
pub type Cpu3R = crate::FieldReader<Cpu3select>;
impl Cpu3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu3select {
        match self.bits {
            0 => Cpu3select::Secured,
            1 => Cpu3select::NsDebug,
            2 => Cpu3select::FullDebug,
            3 => Cpu3select::Unimplemented,
            _ => unreachable!(),
        }
    }
    #[doc = "Debugger access is disabled"]
    #[inline(always)]
    pub fn is_secured(&self) -> bool {
        *self == Cpu3select::Secured
    }
    #[doc = "Debugger can access only non-secure regions"]
    #[inline(always)]
    pub fn is_ns_debug(&self) -> bool {
        *self == Cpu3select::NsDebug
    }
    #[doc = "Debugger has full access to debug CPUn"]
    #[inline(always)]
    pub fn is_full_debug(&self) -> bool {
        *self == Cpu3select::FullDebug
    }
    #[doc = "Unimplemented CPU"]
    #[inline(always)]
    pub fn is_unimplemented(&self) -> bool {
        *self == Cpu3select::Unimplemented
    }
}
impl R {
    #[doc = "Bits 0:1 - CPU 0 Debugger Access Level"]
    #[inline(always)]
    pub fn cpu0(&self) -> Cpu0R {
        Cpu0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CPU n Debugger Access Level"]
    #[inline(always)]
    pub fn cpu1(&self) -> Cpu1R {
        Cpu1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CPU n Debugger Access Level"]
    #[inline(always)]
    pub fn cpu2(&self) -> Cpu2R {
        Cpu2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CPU n Debugger Access Level"]
    #[inline(always)]
    pub fn cpu3(&self) -> Cpu3R {
        Cpu3R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Debugger Access Level REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`dal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DalSpec;
impl crate::RegisterSpec for DalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dal::R`](R) reader structure"]
impl crate::Readable for DalSpec {}
#[doc = "`reset()` method sets DAL to value 0"]
impl crate::Resettable for DalSpec {}

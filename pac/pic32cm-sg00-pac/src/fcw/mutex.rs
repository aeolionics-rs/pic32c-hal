#[doc = "Register `MUTEX` reader"]
pub type R = crate::R<MutexSpec>;
#[doc = "Register `MUTEX` writer"]
pub type W = crate::W<MutexSpec>;
#[doc = "Flash Write Controller (FCW) Lock by Owner\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockselect {
    #[doc = "0: FCW is not locked"]
    Unlock = 0,
    #[doc = "1: FCW is locked by owner"]
    Lock = 1,
}
impl From<Lockselect> for bool {
    #[inline(always)]
    fn from(variant: Lockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Flash Write Controller (FCW) Lock by Owner"]
pub type LockR = crate::BitReader<Lockselect>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockselect {
        match self.bits {
            false => Lockselect::Unlock,
            true => Lockselect::Lock,
        }
    }
    #[doc = "FCW is not locked"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == Lockselect::Unlock
    }
    #[doc = "FCW is locked by owner"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == Lockselect::Lock
    }
}
#[doc = "Field `LOCK` writer - Flash Write Controller (FCW) Lock by Owner"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lockselect>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FCW is not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Lockselect::Unlock)
    }
    #[doc = "FCW is locked by owner"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Lockselect::Lock)
    }
}
#[doc = "Flash Write Controller (FCW) Owner ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ownerselect {
    #[doc = "0: No Owner - always 0 if LOCK=0"]
    None = 0,
    #[doc = "1: The System owns the FCW"]
    System = 1,
}
impl From<Ownerselect> for u8 {
    #[inline(always)]
    fn from(variant: Ownerselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ownerselect {
    type Ux = u8;
}
impl crate::IsEnum for Ownerselect {}
#[doc = "Field `OWNER` reader - Flash Write Controller (FCW) Owner ID"]
pub type OwnerR = crate::FieldReader<Ownerselect>;
impl OwnerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ownerselect> {
        match self.bits {
            0 => Some(Ownerselect::None),
            1 => Some(Ownerselect::System),
            _ => None,
        }
    }
    #[doc = "No Owner - always 0 if LOCK=0"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ownerselect::None
    }
    #[doc = "The System owns the FCW"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == Ownerselect::System
    }
}
#[doc = "Field `OWNER` writer - Flash Write Controller (FCW) Owner ID"]
pub type OwnerW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ownerselect>;
impl<'a, REG> OwnerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Owner - always 0 if LOCK=0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ownerselect::None)
    }
    #[doc = "The System owns the FCW"]
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(Ownerselect::System)
    }
}
impl R {
    #[doc = "Bit 0 - Flash Write Controller (FCW) Lock by Owner"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Flash Write Controller (FCW) Owner ID"]
    #[inline(always)]
    pub fn owner(&self) -> OwnerR {
        OwnerR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Write Controller (FCW) Lock by Owner"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, MutexSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Flash Write Controller (FCW) Owner ID"]
    #[inline(always)]
    pub fn owner(&mut self) -> OwnerW<'_, MutexSpec> {
        OwnerW::new(self, 1)
    }
}
#[doc = "MUTEX REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MutexSpec;
impl crate::RegisterSpec for MutexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mutex::R`](R) reader structure"]
impl crate::Readable for MutexSpec {}
#[doc = "`write(|w| ..)` method takes [`mutex::W`](W) writer structure"]
impl crate::Writable for MutexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MUTEX to value 0"]
impl crate::Resettable for MutexSpec {}

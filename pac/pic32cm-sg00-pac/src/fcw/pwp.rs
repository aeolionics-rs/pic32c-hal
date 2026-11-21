#[doc = "Register `PWP[%s]` reader"]
pub type R = crate::R<PwpSpec>;
#[doc = "Register `PWP[%s]` writer"]
pub type W = crate::W<PwpSpec>;
#[doc = "Field `PWPSIZE` reader - PWP Size in 4KB pages"]
pub type PwpsizeR = crate::FieldReader;
#[doc = "Field `PWPSIZE` writer - PWP Size in 4KB pages"]
pub type PwpsizeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Mirror PWP bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwpmirselect {
    #[doc = "0: PWP settings are NOT Mirrored"]
    Unmirror = 0,
    #[doc = "1: PWP settings are Mirrored"]
    Mirror = 1,
}
impl From<Pwpmirselect> for bool {
    #[inline(always)]
    fn from(variant: Pwpmirselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWPMIR` reader - Mirror PWP bit"]
pub type PwpmirR = crate::BitReader<Pwpmirselect>;
impl PwpmirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwpmirselect {
        match self.bits {
            false => Pwpmirselect::Unmirror,
            true => Pwpmirselect::Mirror,
        }
    }
    #[doc = "PWP settings are NOT Mirrored"]
    #[inline(always)]
    pub fn is_unmirror(&self) -> bool {
        *self == Pwpmirselect::Unmirror
    }
    #[doc = "PWP settings are Mirrored"]
    #[inline(always)]
    pub fn is_mirror(&self) -> bool {
        *self == Pwpmirselect::Mirror
    }
}
#[doc = "Field `PWPMIR` writer - Mirror PWP bit"]
pub type PwpmirW<'a, REG> = crate::BitWriter<'a, REG, Pwpmirselect>;
impl<'a, REG> PwpmirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWP settings are NOT Mirrored"]
    #[inline(always)]
    pub fn unmirror(self) -> &'a mut crate::W<REG> {
        self.variant(Pwpmirselect::Unmirror)
    }
    #[doc = "PWP settings are Mirrored"]
    #[inline(always)]
    pub fn mirror(self) -> &'a mut crate::W<REG> {
        self.variant(Pwpmirselect::Mirror)
    }
}
#[doc = "PWP Lock Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwplockselect {
    #[doc = "0: This register is Not Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: This register is Locked and cannot be modified"]
    Locked = 1,
}
impl From<Pwplockselect> for bool {
    #[inline(always)]
    fn from(variant: Pwplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWPLOCK` reader - PWP Lock Bit"]
pub type PwplockR = crate::BitReader<Pwplockselect>;
impl PwplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwplockselect {
        match self.bits {
            false => Pwplockselect::Unlocked,
            true => Pwplockselect::Locked,
        }
    }
    #[doc = "This register is Not Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Pwplockselect::Unlocked
    }
    #[doc = "This register is Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Pwplockselect::Locked
    }
}
#[doc = "Field `PWPLOCK` writer - PWP Lock Bit"]
pub type PwplockW<'a, REG> = crate::BitWriter<'a, REG, Pwplockselect>;
impl<'a, REG> PwplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This register is Not Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pwplockselect::Unlocked)
    }
    #[doc = "This register is Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Pwplockselect::Locked)
    }
}
#[doc = "PWP Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwpenselect {
    #[doc = "0: PWP is Not Enabled for the defined region"]
    Disable = 0,
    #[doc = "1: PWP is Enabled for the defined region"]
    Enable = 1,
}
impl From<Pwpenselect> for bool {
    #[inline(always)]
    fn from(variant: Pwpenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWPEN` reader - PWP Enable Bit"]
pub type PwpenR = crate::BitReader<Pwpenselect>;
impl PwpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwpenselect {
        match self.bits {
            false => Pwpenselect::Disable,
            true => Pwpenselect::Enable,
        }
    }
    #[doc = "PWP is Not Enabled for the defined region"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pwpenselect::Disable
    }
    #[doc = "PWP is Enabled for the defined region"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pwpenselect::Enable
    }
}
#[doc = "Field `PWPEN` writer - PWP Enable Bit"]
pub type PwpenW<'a, REG> = crate::BitWriter<'a, REG, Pwpenselect>;
impl<'a, REG> PwpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWP is Not Enabled for the defined region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwpenselect::Disable)
    }
    #[doc = "PWP is Enabled for the defined region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwpenselect::Enable)
    }
}
#[doc = "Field `PWPBASE` reader - PWP Base Address - 4KB Page Aligned"]
pub type PwpbaseR = crate::FieldReader;
#[doc = "Field `PWPBASE` writer - PWP Base Address - 4KB Page Aligned"]
pub type PwpbaseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - PWP Size in 4KB pages"]
    #[inline(always)]
    pub fn pwpsize(&self) -> PwpsizeR {
        PwpsizeR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 13 - Mirror PWP bit"]
    #[inline(always)]
    pub fn pwpmir(&self) -> PwpmirR {
        PwpmirR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PWP Lock Bit"]
    #[inline(always)]
    pub fn pwplock(&self) -> PwplockR {
        PwplockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PWP Enable Bit"]
    #[inline(always)]
    pub fn pwpen(&self) -> PwpenR {
        PwpenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - PWP Base Address - 4KB Page Aligned"]
    #[inline(always)]
    pub fn pwpbase(&self) -> PwpbaseR {
        PwpbaseR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - PWP Size in 4KB pages"]
    #[inline(always)]
    pub fn pwpsize(&mut self) -> PwpsizeW<'_, PwpSpec> {
        PwpsizeW::new(self, 0)
    }
    #[doc = "Bit 13 - Mirror PWP bit"]
    #[inline(always)]
    pub fn pwpmir(&mut self) -> PwpmirW<'_, PwpSpec> {
        PwpmirW::new(self, 13)
    }
    #[doc = "Bit 14 - PWP Lock Bit"]
    #[inline(always)]
    pub fn pwplock(&mut self) -> PwplockW<'_, PwpSpec> {
        PwplockW::new(self, 14)
    }
    #[doc = "Bit 15 - PWP Enable Bit"]
    #[inline(always)]
    pub fn pwpen(&mut self) -> PwpenW<'_, PwpSpec> {
        PwpenW::new(self, 15)
    }
    #[doc = "Bits 16:22 - PWP Base Address - 4KB Page Aligned"]
    #[inline(always)]
    pub fn pwpbase(&mut self) -> PwpbaseW<'_, PwpSpec> {
        PwpbaseW::new(self, 16)
    }
}
#[doc = "PFM Write Protect Region n REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`pwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwpSpec;
impl crate::RegisterSpec for PwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwp::R`](R) reader structure"]
impl crate::Readable for PwpSpec {}
#[doc = "`write(|w| ..)` method takes [`pwp::W`](W) writer structure"]
impl crate::Writable for PwpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWP[%s] to value 0"]
impl crate::Resettable for PwpSpec {}

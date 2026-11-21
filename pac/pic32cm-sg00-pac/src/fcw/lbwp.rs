#[doc = "Register `LBWP` reader"]
pub type R = crate::R<LbwpSpec>;
#[doc = "Register `LBWP` writer"]
pub type W = crate::W<LbwpSpec>;
#[doc = "Lower Boot Pages Write Protect Bits\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lbwpselect {
    #[doc = "0: Erase and Write Protection for this Lower Boot Page is Disabled"]
    Disable = 0,
    #[doc = "1: Erase and Write Protection for this Lower Boot Page is Enabled"]
    Enable = 1,
}
impl From<Lbwpselect> for u8 {
    #[inline(always)]
    fn from(variant: Lbwpselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lbwpselect {
    type Ux = u8;
}
impl crate::IsEnum for Lbwpselect {}
#[doc = "Field `LBWP` reader - Lower Boot Pages Write Protect Bits"]
pub type LbwpR = crate::FieldReader<Lbwpselect>;
impl LbwpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lbwpselect> {
        match self.bits {
            0 => Some(Lbwpselect::Disable),
            1 => Some(Lbwpselect::Enable),
            _ => None,
        }
    }
    #[doc = "Erase and Write Protection for this Lower Boot Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lbwpselect::Disable
    }
    #[doc = "Erase and Write Protection for this Lower Boot Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lbwpselect::Enable
    }
}
#[doc = "Field `LBWP` writer - Lower Boot Pages Write Protect Bits"]
pub type LbwpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lbwpselect>;
impl<'a, REG> LbwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Erase and Write Protection for this Lower Boot Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbwpselect::Disable)
    }
    #[doc = "Erase and Write Protection for this Lower Boot Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbwpselect::Enable)
    }
}
#[doc = "LBWP Lock Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbwplockselect {
    #[doc = "0: This register is NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: This register is Locked and cannot be modified"]
    Locked = 1,
}
impl From<Lbwplockselect> for bool {
    #[inline(always)]
    fn from(variant: Lbwplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBWPLOCK` reader - LBWP Lock Bit"]
pub type LbwplockR = crate::BitReader<Lbwplockselect>;
impl LbwplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbwplockselect {
        match self.bits {
            false => Lbwplockselect::Unlocked,
            true => Lbwplockselect::Locked,
        }
    }
    #[doc = "This register is NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lbwplockselect::Unlocked
    }
    #[doc = "This register is Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lbwplockselect::Locked
    }
}
#[doc = "Field `LBWPLOCK` writer - LBWP Lock Bit"]
pub type LbwplockW<'a, REG> = crate::BitWriter<'a, REG, Lbwplockselect>;
impl<'a, REG> LbwplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This register is NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lbwplockselect::Unlocked)
    }
    #[doc = "This register is Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lbwplockselect::Locked)
    }
}
impl R {
    #[doc = "Bits 0:1 - Lower Boot Pages Write Protect Bits"]
    #[inline(always)]
    pub fn lbwp(&self) -> LbwpR {
        LbwpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - LBWP Lock Bit"]
    #[inline(always)]
    pub fn lbwplock(&self) -> LbwplockR {
        LbwplockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Lower Boot Pages Write Protect Bits"]
    #[inline(always)]
    pub fn lbwp(&mut self) -> LbwpW<'_, LbwpSpec> {
        LbwpW::new(self, 0)
    }
    #[doc = "Bit 31 - LBWP Lock Bit"]
    #[inline(always)]
    pub fn lbwplock(&mut self) -> LbwplockW<'_, LbwpSpec> {
        LbwplockW::new(self, 31)
    }
}
#[doc = "Lower BFM Write Protect REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`lbwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lbwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LbwpSpec;
impl crate::RegisterSpec for LbwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lbwp::R`](R) reader structure"]
impl crate::Readable for LbwpSpec {}
#[doc = "`write(|w| ..)` method takes [`lbwp::W`](W) writer structure"]
impl crate::Writable for LbwpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LBWP to value 0x03"]
impl crate::Resettable for LbwpSpec {
    const RESET_VALUE: u32 = 0x03;
}

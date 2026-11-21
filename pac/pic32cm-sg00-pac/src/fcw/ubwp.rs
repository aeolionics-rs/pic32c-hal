#[doc = "Register `UBWP` reader"]
pub type R = crate::R<UbwpSpec>;
#[doc = "Register `UBWP` writer"]
pub type W = crate::W<UbwpSpec>;
#[doc = "Upper Boot Pages Write Protect Bits\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ubwpselect {
    #[doc = "0: Erase and Write Protection for this Upper Boot Page is Disabled"]
    Disable = 0,
    #[doc = "1: Erase and Write Protection for this Upper Boot Page is Enabled"]
    Enable = 1,
}
impl From<Ubwpselect> for u8 {
    #[inline(always)]
    fn from(variant: Ubwpselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ubwpselect {
    type Ux = u8;
}
impl crate::IsEnum for Ubwpselect {}
#[doc = "Field `UBWP` reader - Upper Boot Pages Write Protect Bits"]
pub type UbwpR = crate::FieldReader<Ubwpselect>;
impl UbwpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ubwpselect> {
        match self.bits {
            0 => Some(Ubwpselect::Disable),
            1 => Some(Ubwpselect::Enable),
            _ => None,
        }
    }
    #[doc = "Erase and Write Protection for this Upper Boot Page is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ubwpselect::Disable
    }
    #[doc = "Erase and Write Protection for this Upper Boot Page is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ubwpselect::Enable
    }
}
#[doc = "Field `UBWP` writer - Upper Boot Pages Write Protect Bits"]
pub type UbwpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ubwpselect>;
impl<'a, REG> UbwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Erase and Write Protection for this Upper Boot Page is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ubwpselect::Disable)
    }
    #[doc = "Erase and Write Protection for this Upper Boot Page is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ubwpselect::Enable)
    }
}
#[doc = "UBWP Lock Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ubwplockselect {
    #[doc = "0: This register is NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: This register is Locked and cannot be modified"]
    Locked = 1,
}
impl From<Ubwplockselect> for bool {
    #[inline(always)]
    fn from(variant: Ubwplockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UBWPLOCK` reader - UBWP Lock Bit"]
pub type UbwplockR = crate::BitReader<Ubwplockselect>;
impl UbwplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ubwplockselect {
        match self.bits {
            false => Ubwplockselect::Unlocked,
            true => Ubwplockselect::Locked,
        }
    }
    #[doc = "This register is NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Ubwplockselect::Unlocked
    }
    #[doc = "This register is Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Ubwplockselect::Locked
    }
}
#[doc = "Field `UBWPLOCK` writer - UBWP Lock Bit"]
pub type UbwplockW<'a, REG> = crate::BitWriter<'a, REG, Ubwplockselect>;
impl<'a, REG> UbwplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This register is NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Ubwplockselect::Unlocked)
    }
    #[doc = "This register is Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Ubwplockselect::Locked)
    }
}
impl R {
    #[doc = "Bits 0:1 - Upper Boot Pages Write Protect Bits"]
    #[inline(always)]
    pub fn ubwp(&self) -> UbwpR {
        UbwpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - UBWP Lock Bit"]
    #[inline(always)]
    pub fn ubwplock(&self) -> UbwplockR {
        UbwplockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Upper Boot Pages Write Protect Bits"]
    #[inline(always)]
    pub fn ubwp(&mut self) -> UbwpW<'_, UbwpSpec> {
        UbwpW::new(self, 0)
    }
    #[doc = "Bit 31 - UBWP Lock Bit"]
    #[inline(always)]
    pub fn ubwplock(&mut self) -> UbwplockW<'_, UbwpSpec> {
        UbwplockW::new(self, 31)
    }
}
#[doc = "Upper BFM Write Protect REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ubwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UbwpSpec;
impl crate::RegisterSpec for UbwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ubwp::R`](R) reader structure"]
impl crate::Readable for UbwpSpec {}
#[doc = "`write(|w| ..)` method takes [`ubwp::W`](W) writer structure"]
impl crate::Writable for UbwpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UBWP to value 0x03"]
impl crate::Resettable for UbwpSpec {
    const RESET_VALUE: u32 = 0x03;
}

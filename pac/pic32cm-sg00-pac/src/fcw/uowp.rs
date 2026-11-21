#[doc = "Register `UOWP` reader"]
pub type R = crate::R<UowpSpec>;
#[doc = "Register `UOWP` writer"]
pub type W = crate::W<UowpSpec>;
#[doc = "User OTP Page 1 Write Protect Row Bit\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uo1wprselect {
    #[doc = "0: Write Protection for User OTP Page 1 Row 'n' is Disabled"]
    Disable = 0,
    #[doc = "1: Write Protection for User OTP Page 1 Row 'n' is Enabled"]
    Enable = 1,
}
impl From<Uo1wprselect> for u8 {
    #[inline(always)]
    fn from(variant: Uo1wprselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uo1wprselect {
    type Ux = u8;
}
impl crate::IsEnum for Uo1wprselect {}
#[doc = "Field `UO1WPR` reader - User OTP Page 1 Write Protect Row Bit"]
pub type Uo1wprR = crate::FieldReader<Uo1wprselect>;
impl Uo1wprR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uo1wprselect> {
        match self.bits {
            0 => Some(Uo1wprselect::Disable),
            1 => Some(Uo1wprselect::Enable),
            _ => None,
        }
    }
    #[doc = "Write Protection for User OTP Page 1 Row 'n' is Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Uo1wprselect::Disable
    }
    #[doc = "Write Protection for User OTP Page 1 Row 'n' is Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Uo1wprselect::Enable
    }
}
#[doc = "Field `UO1WPR` writer - User OTP Page 1 Write Protect Row Bit"]
pub type Uo1wprW<'a, REG> = crate::FieldWriter<'a, REG, 4, Uo1wprselect>;
impl<'a, REG> Uo1wprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write Protection for User OTP Page 1 Row 'n' is Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Uo1wprselect::Disable)
    }
    #[doc = "Write Protection for User OTP Page 1 Row 'n' is Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Uo1wprselect::Enable)
    }
}
#[doc = "User OTP Page 1 WP Row Lock Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uo1wprlockselect {
    #[doc = "0: UO1WPR\\[n\\] &amp; UO1WPRLOCK\\[n\\] bits are NOT Locked and can be modified"]
    Unlocked = 0,
    #[doc = "1: UO1WPR\\[n\\] &amp; UO1WPRLOCK\\[n\\] bits are Locked and cannot be modified"]
    Locked = 1,
}
impl From<Uo1wprlockselect> for u8 {
    #[inline(always)]
    fn from(variant: Uo1wprlockselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uo1wprlockselect {
    type Ux = u8;
}
impl crate::IsEnum for Uo1wprlockselect {}
#[doc = "Field `UO1WPRLOCK` reader - User OTP Page 1 WP Row Lock Bit"]
pub type Uo1wprlockR = crate::FieldReader<Uo1wprlockselect>;
impl Uo1wprlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uo1wprlockselect> {
        match self.bits {
            0 => Some(Uo1wprlockselect::Unlocked),
            1 => Some(Uo1wprlockselect::Locked),
            _ => None,
        }
    }
    #[doc = "UO1WPR\\[n\\] &amp; UO1WPRLOCK\\[n\\] bits are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Uo1wprlockselect::Unlocked
    }
    #[doc = "UO1WPR\\[n\\] &amp; UO1WPRLOCK\\[n\\] bits are Locked and cannot be modified"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Uo1wprlockselect::Locked
    }
}
#[doc = "Field `UO1WPRLOCK` writer - User OTP Page 1 WP Row Lock Bit"]
pub type Uo1wprlockW<'a, REG> = crate::FieldWriter<'a, REG, 4, Uo1wprlockselect>;
impl<'a, REG> Uo1wprlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UO1WPR\\[n\\] &amp; UO1WPRLOCK\\[n\\] bits are NOT Locked and can be modified"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Uo1wprlockselect::Unlocked)
    }
    #[doc = "UO1WPR\\[n\\] &amp; UO1WPRLOCK\\[n\\] bits are Locked and cannot be modified"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Uo1wprlockselect::Locked)
    }
}
impl R {
    #[doc = "Bits 0:3 - User OTP Page 1 Write Protect Row Bit"]
    #[inline(always)]
    pub fn uo1wpr(&self) -> Uo1wprR {
        Uo1wprR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - User OTP Page 1 WP Row Lock Bit"]
    #[inline(always)]
    pub fn uo1wprlock(&self) -> Uo1wprlockR {
        Uo1wprlockR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User OTP Page 1 Write Protect Row Bit"]
    #[inline(always)]
    pub fn uo1wpr(&mut self) -> Uo1wprW<'_, UowpSpec> {
        Uo1wprW::new(self, 0)
    }
    #[doc = "Bits 16:19 - User OTP Page 1 WP Row Lock Bit"]
    #[inline(always)]
    pub fn uo1wprlock(&mut self) -> Uo1wprlockW<'_, UowpSpec> {
        Uo1wprlockW::new(self, 16)
    }
}
#[doc = "User OTP Write protection REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`uowp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uowp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UowpSpec;
impl crate::RegisterSpec for UowpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uowp::R`](R) reader structure"]
impl crate::Readable for UowpSpec {}
#[doc = "`write(|w| ..)` method takes [`uowp::W`](W) writer structure"]
impl crate::Writable for UowpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UOWP to value 0x0f"]
impl crate::Resettable for UowpSpec {
    const RESET_VALUE: u32 = 0x0f;
}

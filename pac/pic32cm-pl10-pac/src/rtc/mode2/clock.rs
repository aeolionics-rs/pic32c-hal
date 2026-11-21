#[doc = "Register `CLOCK` reader"]
pub type R = crate::R<ClockSpec>;
#[doc = "Register `CLOCK` writer"]
pub type W = crate::W<ClockSpec>;
#[doc = "Field `SECOND` reader - Second"]
pub type SecondR = crate::FieldReader;
#[doc = "Field `SECOND` writer - Second"]
pub type SecondW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MINUTE` reader - Minute"]
pub type MinuteR = crate::FieldReader;
#[doc = "Field `MINUTE` writer - Minute"]
pub type MinuteW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DAY` reader - Day"]
pub type DayR = crate::FieldReader;
#[doc = "Field `DAY` writer - Day"]
pub type DayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Month\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Monthselect {
    #[doc = "1: January"]
    Jan = 1,
    #[doc = "2: February"]
    Feb = 2,
    #[doc = "3: March"]
    Mar = 3,
    #[doc = "4: April"]
    Apr = 4,
    #[doc = "5: May"]
    May = 5,
    #[doc = "6: June"]
    Jun = 6,
    #[doc = "7: July"]
    Jul = 7,
    #[doc = "8: August"]
    Aug = 8,
    #[doc = "9: September"]
    Sep = 9,
    #[doc = "10: October"]
    Oct = 10,
    #[doc = "11: November"]
    Nov = 11,
    #[doc = "12: December"]
    Dec = 12,
}
impl From<Monthselect> for u8 {
    #[inline(always)]
    fn from(variant: Monthselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Monthselect {
    type Ux = u8;
}
impl crate::IsEnum for Monthselect {}
#[doc = "Field `MONTH` reader - Month"]
pub type MonthR = crate::FieldReader<Monthselect>;
impl MonthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Monthselect> {
        match self.bits {
            1 => Some(Monthselect::Jan),
            2 => Some(Monthselect::Feb),
            3 => Some(Monthselect::Mar),
            4 => Some(Monthselect::Apr),
            5 => Some(Monthselect::May),
            6 => Some(Monthselect::Jun),
            7 => Some(Monthselect::Jul),
            8 => Some(Monthselect::Aug),
            9 => Some(Monthselect::Sep),
            10 => Some(Monthselect::Oct),
            11 => Some(Monthselect::Nov),
            12 => Some(Monthselect::Dec),
            _ => None,
        }
    }
    #[doc = "January"]
    #[inline(always)]
    pub fn is_jan(&self) -> bool {
        *self == Monthselect::Jan
    }
    #[doc = "February"]
    #[inline(always)]
    pub fn is_feb(&self) -> bool {
        *self == Monthselect::Feb
    }
    #[doc = "March"]
    #[inline(always)]
    pub fn is_mar(&self) -> bool {
        *self == Monthselect::Mar
    }
    #[doc = "April"]
    #[inline(always)]
    pub fn is_apr(&self) -> bool {
        *self == Monthselect::Apr
    }
    #[doc = "May"]
    #[inline(always)]
    pub fn is_may(&self) -> bool {
        *self == Monthselect::May
    }
    #[doc = "June"]
    #[inline(always)]
    pub fn is_jun(&self) -> bool {
        *self == Monthselect::Jun
    }
    #[doc = "July"]
    #[inline(always)]
    pub fn is_jul(&self) -> bool {
        *self == Monthselect::Jul
    }
    #[doc = "August"]
    #[inline(always)]
    pub fn is_aug(&self) -> bool {
        *self == Monthselect::Aug
    }
    #[doc = "September"]
    #[inline(always)]
    pub fn is_sep(&self) -> bool {
        *self == Monthselect::Sep
    }
    #[doc = "October"]
    #[inline(always)]
    pub fn is_oct(&self) -> bool {
        *self == Monthselect::Oct
    }
    #[doc = "November"]
    #[inline(always)]
    pub fn is_nov(&self) -> bool {
        *self == Monthselect::Nov
    }
    #[doc = "December"]
    #[inline(always)]
    pub fn is_dec(&self) -> bool {
        *self == Monthselect::Dec
    }
}
#[doc = "Field `MONTH` writer - Month"]
pub type MonthW<'a, REG> = crate::FieldWriter<'a, REG, 4, Monthselect>;
impl<'a, REG> MonthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "January"]
    #[inline(always)]
    pub fn jan(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::Jan)
    }
    #[doc = "February"]
    #[inline(always)]
    pub fn feb(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::Feb)
    }
    #[doc = "March"]
    #[inline(always)]
    pub fn mar(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::Mar)
    }
    #[doc = "April"]
    #[inline(always)]
    pub fn apr(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::Apr)
    }
    #[doc = "May"]
    #[inline(always)]
    pub fn may(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::May)
    }
    #[doc = "June"]
    #[inline(always)]
    pub fn jun(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::Jun)
    }
    #[doc = "July"]
    #[inline(always)]
    pub fn jul(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::Jul)
    }
    #[doc = "August"]
    #[inline(always)]
    pub fn aug(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::Aug)
    }
    #[doc = "September"]
    #[inline(always)]
    pub fn sep(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::Sep)
    }
    #[doc = "October"]
    #[inline(always)]
    pub fn oct(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::Oct)
    }
    #[doc = "November"]
    #[inline(always)]
    pub fn nov(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::Nov)
    }
    #[doc = "December"]
    #[inline(always)]
    pub fn dec(self) -> &'a mut crate::W<REG> {
        self.variant(Monthselect::Dec)
    }
}
#[doc = "Field `YEAR` reader - Year"]
pub type YearR = crate::FieldReader;
#[doc = "Field `YEAR` writer - Year"]
pub type YearW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    pub fn second(&self) -> SecondR {
        SecondR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    pub fn minute(&self) -> MinuteR {
        MinuteR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    pub fn second(&mut self) -> SecondW<'_, ClockSpec> {
        SecondW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    pub fn minute(&mut self) -> MinuteW<'_, ClockSpec> {
        MinuteW::new(self, 6)
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    pub fn day(&mut self) -> DayW<'_, ClockSpec> {
        DayW::new(self, 17)
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    pub fn month(&mut self) -> MonthW<'_, ClockSpec> {
        MonthW::new(self, 22)
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    pub fn year(&mut self) -> YearW<'_, ClockSpec> {
        YearW::new(self, 26)
    }
}
#[doc = "MODE2 Clock Value\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockSpec;
impl crate::RegisterSpec for ClockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock::R`](R) reader structure"]
impl crate::Readable for ClockSpec {}
#[doc = "`write(|w| ..)` method takes [`clock::W`](W) writer structure"]
impl crate::Writable for ClockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCK to value 0"]
impl crate::Resettable for ClockSpec {}

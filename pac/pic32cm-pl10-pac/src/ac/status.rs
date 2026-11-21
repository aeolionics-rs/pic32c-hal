#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `COMPSTATE0` reader - Comparator 0 State"]
pub type Compstate0R = crate::BitReader;
#[doc = "Field `COMPSTATE1` reader - Comparator 1 State"]
pub type Compstate1R = crate::BitReader;
#[doc = "Window 0 State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Winstate0select {
    #[doc = "0: The input signal is above window n"]
    Above = 0,
    #[doc = "1: The input signal is inside window n"]
    Inside = 1,
    #[doc = "2: The input signal is below window n"]
    Below = 2,
}
impl From<Winstate0select> for u8 {
    #[inline(always)]
    fn from(variant: Winstate0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Winstate0select {
    type Ux = u8;
}
impl crate::IsEnum for Winstate0select {}
#[doc = "Field `WINSTATE0` reader - Window 0 State"]
pub type Winstate0R = crate::FieldReader<Winstate0select>;
impl Winstate0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Winstate0select> {
        match self.bits {
            0 => Some(Winstate0select::Above),
            1 => Some(Winstate0select::Inside),
            2 => Some(Winstate0select::Below),
            _ => None,
        }
    }
    #[doc = "The input signal is above window n"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == Winstate0select::Above
    }
    #[doc = "The input signal is inside window n"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == Winstate0select::Inside
    }
    #[doc = "The input signal is below window n"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == Winstate0select::Below
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 0 State"]
    #[inline(always)]
    pub fn compstate0(&self) -> Compstate0R {
        Compstate0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 State"]
    #[inline(always)]
    pub fn compstate1(&self) -> Compstate1R {
        Compstate1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Window 0 State"]
    #[inline(always)]
    pub fn winstate0(&self) -> Winstate0R {
        Winstate0R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}

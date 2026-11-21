#[doc = "Register `WPCTRL` reader"]
pub type R = crate::R<WpctrlSpec>;
#[doc = "Register `WPCTRL` writer"]
pub type W = crate::W<WpctrlSpec>;
#[doc = "Field `WPEN` reader - Write Protection Enable"]
pub type WpenR = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protection Enable"]
pub type WpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPLCK` reader - Write Protection Lock"]
pub type WplckR = crate::BitReader;
#[doc = "Field `WPLCK` writer - Write Protection Lock"]
pub type WplckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Wpkeyselect {
    #[doc = "5395284: Allow writes to the WPCTRL register"]
    Key = 5395284,
}
impl From<Wpkeyselect> for u32 {
    #[inline(always)]
    fn from(variant: Wpkeyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wpkeyselect {
    type Ux = u32;
}
impl crate::IsEnum for Wpkeyselect {}
#[doc = "Field `WPKEY` reader - Write Protection Key"]
pub type WpkeyR = crate::FieldReader<Wpkeyselect>;
impl WpkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wpkeyselect> {
        match self.bits {
            5395284 => Some(Wpkeyselect::Key),
            _ => None,
        }
    }
    #[doc = "Allow writes to the WPCTRL register"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == Wpkeyselect::Key
    }
}
#[doc = "Field `WPKEY` writer - Write Protection Key"]
pub type WpkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, Wpkeyselect>;
impl<'a, REG> WpkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Allow writes to the WPCTRL register"]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(Wpkeyselect::Key)
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WpenR {
        WpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protection Lock"]
    #[inline(always)]
    pub fn wplck(&self) -> WplckR {
        WplckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&self) -> WpkeyR {
        WpkeyR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&mut self) -> WpenW<'_, WpctrlSpec> {
        WpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Protection Lock"]
    #[inline(always)]
    pub fn wplck(&mut self) -> WplckW<'_, WpctrlSpec> {
        WplckW::new(self, 1)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&mut self) -> WpkeyW<'_, WpctrlSpec> {
        WpkeyW::new(self, 8)
    }
}
#[doc = "Write Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wpctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpctrlSpec;
impl crate::RegisterSpec for WpctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpctrl::R`](R) reader structure"]
impl crate::Readable for WpctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wpctrl::W`](W) writer structure"]
impl crate::Writable for WpctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WPCTRL to value 0"]
impl crate::Resettable for WpctrlSpec {}

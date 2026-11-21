#[doc = "Register `KEY` reader"]
pub type R = crate::R<KeySpec>;
#[doc = "Register `KEY` writer"]
pub type W = crate::W<KeySpec>;
#[doc = "Feature Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Featureselect {
    #[doc = "0: No selection, invalid selection, or invalid Key - Locks all SFR protected by KEY register"]
    Lockall = 0,
    #[doc = "1: WRKEY Value: Unlock SFR bits associated with Write/Erase"]
    Wrkey = 1,
    #[doc = "2: SWAPKEY Value: Unlock SFR bits associated with Panel Swapping"]
    Swapkey = 2,
    #[doc = "4: CFGKEY Value: Unlock SFR bits associated with general Flash configuration"]
    Cfgkey = 4,
}
impl From<Featureselect> for u8 {
    #[inline(always)]
    fn from(variant: Featureselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Featureselect {
    type Ux = u8;
}
impl crate::IsEnum for Featureselect {}
#[doc = "Field `FEATURE` reader - Feature Select"]
pub type FeatureR = crate::FieldReader<Featureselect>;
impl FeatureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Featureselect> {
        match self.bits {
            0 => Some(Featureselect::Lockall),
            1 => Some(Featureselect::Wrkey),
            2 => Some(Featureselect::Swapkey),
            4 => Some(Featureselect::Cfgkey),
            _ => None,
        }
    }
    #[doc = "No selection, invalid selection, or invalid Key - Locks all SFR protected by KEY register"]
    #[inline(always)]
    pub fn is_lockall(&self) -> bool {
        *self == Featureselect::Lockall
    }
    #[doc = "WRKEY Value: Unlock SFR bits associated with Write/Erase"]
    #[inline(always)]
    pub fn is_wrkey(&self) -> bool {
        *self == Featureselect::Wrkey
    }
    #[doc = "SWAPKEY Value: Unlock SFR bits associated with Panel Swapping"]
    #[inline(always)]
    pub fn is_swapkey(&self) -> bool {
        *self == Featureselect::Swapkey
    }
    #[doc = "CFGKEY Value: Unlock SFR bits associated with general Flash configuration"]
    #[inline(always)]
    pub fn is_cfgkey(&self) -> bool {
        *self == Featureselect::Cfgkey
    }
}
#[doc = "Field `FEATURE` writer - Feature Select"]
pub type FeatureW<'a, REG> = crate::FieldWriter<'a, REG, 8, Featureselect>;
impl<'a, REG> FeatureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No selection, invalid selection, or invalid Key - Locks all SFR protected by KEY register"]
    #[inline(always)]
    pub fn lockall(self) -> &'a mut crate::W<REG> {
        self.variant(Featureselect::Lockall)
    }
    #[doc = "WRKEY Value: Unlock SFR bits associated with Write/Erase"]
    #[inline(always)]
    pub fn wrkey(self) -> &'a mut crate::W<REG> {
        self.variant(Featureselect::Wrkey)
    }
    #[doc = "SWAPKEY Value: Unlock SFR bits associated with Panel Swapping"]
    #[inline(always)]
    pub fn swapkey(self) -> &'a mut crate::W<REG> {
        self.variant(Featureselect::Swapkey)
    }
    #[doc = "CFGKEY Value: Unlock SFR bits associated with general Flash configuration"]
    #[inline(always)]
    pub fn cfgkey(self) -> &'a mut crate::W<REG> {
        self.variant(Featureselect::Cfgkey)
    }
}
#[doc = "Field `CODE` reader - Unlock Code"]
pub type CodeR = crate::FieldReader<u32>;
#[doc = "Field `CODE` writer - Unlock Code"]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - Feature Select"]
    #[inline(always)]
    pub fn feature(&self) -> FeatureR {
        FeatureR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Unlock Code"]
    #[inline(always)]
    pub fn code(&self) -> CodeR {
        CodeR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Feature Select"]
    #[inline(always)]
    pub fn feature(&mut self) -> FeatureW<'_, KeySpec> {
        FeatureW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Unlock Code"]
    #[inline(always)]
    pub fn code(&mut self) -> CodeW<'_, KeySpec> {
        CodeW::new(self, 8)
    }
}
#[doc = "Feature Unlock Key REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeySpec;
impl crate::RegisterSpec for KeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key::R`](R) reader structure"]
impl crate::Readable for KeySpec {}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::Writable for KeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY to value 0"]
impl crate::Resettable for KeySpec {}

#[doc = "Register `anti_tamper__LEVELD` reader"]
pub type R = crate::R<AntiTamper_LeveldSpec>;
#[doc = "Field `LEVEL24` reader - Level 24."]
pub type Level24R = crate::FieldReader;
#[doc = "Field `LEVEL25` reader - Level 25."]
pub type Level25R = crate::FieldReader;
#[doc = "Field `LEVEL26` reader - Level 26."]
pub type Level26R = crate::FieldReader;
#[doc = "Field `LEVEL27` reader - Level 27."]
pub type Level27R = crate::FieldReader;
#[doc = "Field `LEVEL28` reader - Level 28."]
pub type Level28R = crate::FieldReader;
#[doc = "Field `LEVEL29` reader - Level 29."]
pub type Level29R = crate::FieldReader;
#[doc = "Field `LEVEL30` reader - Level 30."]
pub type Level30R = crate::FieldReader;
#[doc = "Field `LEVEL31` reader - Level 31."]
pub type Level31R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Level 24."]
    #[inline(always)]
    pub fn level24(&self) -> Level24R {
        Level24R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Level 25."]
    #[inline(always)]
    pub fn level25(&self) -> Level25R {
        Level25R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Level 26."]
    #[inline(always)]
    pub fn level26(&self) -> Level26R {
        Level26R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Level 27."]
    #[inline(always)]
    pub fn level27(&self) -> Level27R {
        Level27R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Level 28."]
    #[inline(always)]
    pub fn level28(&self) -> Level28R {
        Level28R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Level 29."]
    #[inline(always)]
    pub fn level29(&self) -> Level29R {
        Level29R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Level 30."]
    #[inline(always)]
    pub fn level30(&self) -> Level30R {
        Level30R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Level 31."]
    #[inline(always)]
    pub fn level31(&self) -> Level31R {
        Level31R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Tamper levels 24 to 31.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__leveld::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_LeveldSpec;
impl crate::RegisterSpec for AntiTamper_LeveldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__leveld::R`](R) reader structure"]
impl crate::Readable for AntiTamper_LeveldSpec {}
#[doc = "`reset()` method sets anti_tamper__LEVELD to value 0"]
impl crate::Resettable for AntiTamper_LeveldSpec {}

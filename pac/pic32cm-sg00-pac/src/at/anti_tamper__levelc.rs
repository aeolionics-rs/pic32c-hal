#[doc = "Register `anti_tamper__LEVELC` reader"]
pub type R = crate::R<AntiTamper_LevelcSpec>;
#[doc = "Field `LEVEL16` reader - Level 16."]
pub type Level16R = crate::FieldReader;
#[doc = "Field `LEVEL17` reader - Level 17."]
pub type Level17R = crate::FieldReader;
#[doc = "Field `LEVEL18` reader - Level 18."]
pub type Level18R = crate::FieldReader;
#[doc = "Field `LEVEL19` reader - Level 19."]
pub type Level19R = crate::FieldReader;
#[doc = "Field `LEVEL20` reader - Level 20."]
pub type Level20R = crate::FieldReader;
#[doc = "Field `LEVEL21` reader - Level 21."]
pub type Level21R = crate::FieldReader;
#[doc = "Field `LEVEL22` reader - Level 22."]
pub type Level22R = crate::FieldReader;
#[doc = "Field `LEVEL23` reader - Level 23."]
pub type Level23R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Level 16."]
    #[inline(always)]
    pub fn level16(&self) -> Level16R {
        Level16R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Level 17."]
    #[inline(always)]
    pub fn level17(&self) -> Level17R {
        Level17R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Level 18."]
    #[inline(always)]
    pub fn level18(&self) -> Level18R {
        Level18R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Level 19."]
    #[inline(always)]
    pub fn level19(&self) -> Level19R {
        Level19R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Level 20."]
    #[inline(always)]
    pub fn level20(&self) -> Level20R {
        Level20R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Level 21."]
    #[inline(always)]
    pub fn level21(&self) -> Level21R {
        Level21R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Level 22."]
    #[inline(always)]
    pub fn level22(&self) -> Level22R {
        Level22R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Level 23."]
    #[inline(always)]
    pub fn level23(&self) -> Level23R {
        Level23R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Tamper levels 16 to 23.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__levelc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_LevelcSpec;
impl crate::RegisterSpec for AntiTamper_LevelcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__levelc::R`](R) reader structure"]
impl crate::Readable for AntiTamper_LevelcSpec {}
#[doc = "`reset()` method sets anti_tamper__LEVELC to value 0"]
impl crate::Resettable for AntiTamper_LevelcSpec {}

#[doc = "Register `anti_tamper__LEVELB` reader"]
pub type R = crate::R<AntiTamper_LevelbSpec>;
#[doc = "Field `LEVEL8` reader - Level 8."]
pub type Level8R = crate::FieldReader;
#[doc = "Field `LEVEL9` reader - Level 9."]
pub type Level9R = crate::FieldReader;
#[doc = "Field `LEVEL10` reader - Level 10."]
pub type Level10R = crate::FieldReader;
#[doc = "Field `LEVEL11` reader - Level 11."]
pub type Level11R = crate::FieldReader;
#[doc = "Field `LEVEL12` reader - Level 12."]
pub type Level12R = crate::FieldReader;
#[doc = "Field `LEVEL13` reader - Level 13."]
pub type Level13R = crate::FieldReader;
#[doc = "Field `LEVEL14` reader - Level 14."]
pub type Level14R = crate::FieldReader;
#[doc = "Field `LEVEL15` reader - Level 15."]
pub type Level15R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Level 8."]
    #[inline(always)]
    pub fn level8(&self) -> Level8R {
        Level8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Level 9."]
    #[inline(always)]
    pub fn level9(&self) -> Level9R {
        Level9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Level 10."]
    #[inline(always)]
    pub fn level10(&self) -> Level10R {
        Level10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Level 11."]
    #[inline(always)]
    pub fn level11(&self) -> Level11R {
        Level11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Level 12."]
    #[inline(always)]
    pub fn level12(&self) -> Level12R {
        Level12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Level 13."]
    #[inline(always)]
    pub fn level13(&self) -> Level13R {
        Level13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Level 14."]
    #[inline(always)]
    pub fn level14(&self) -> Level14R {
        Level14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Level 15."]
    #[inline(always)]
    pub fn level15(&self) -> Level15R {
        Level15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Tamper levels 8 to 15.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__levelb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_LevelbSpec;
impl crate::RegisterSpec for AntiTamper_LevelbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__levelb::R`](R) reader structure"]
impl crate::Readable for AntiTamper_LevelbSpec {}
#[doc = "`reset()` method sets anti_tamper__LEVELB to value 0"]
impl crate::Resettable for AntiTamper_LevelbSpec {}

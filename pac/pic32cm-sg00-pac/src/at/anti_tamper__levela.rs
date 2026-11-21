#[doc = "Register `anti_tamper__LEVELA` reader"]
pub type R = crate::R<AntiTamper_LevelaSpec>;
#[doc = "Field `LEVEL0` reader - Level 0."]
pub type Level0R = crate::FieldReader;
#[doc = "Field `LEVEL1` reader - Level 1."]
pub type Level1R = crate::FieldReader;
#[doc = "Field `LEVEL2` reader - Level 2."]
pub type Level2R = crate::FieldReader;
#[doc = "Field `LEVEL3` reader - Level 3."]
pub type Level3R = crate::FieldReader;
#[doc = "Field `LEVEL4` reader - Level 4."]
pub type Level4R = crate::FieldReader;
#[doc = "Field `LEVEL5` reader - Level 5."]
pub type Level5R = crate::FieldReader;
#[doc = "Field `LEVEL6` reader - Level 6."]
pub type Level6R = crate::FieldReader;
#[doc = "Field `LEVEL7` reader - Level 7."]
pub type Level7R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Level 0."]
    #[inline(always)]
    pub fn level0(&self) -> Level0R {
        Level0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Level 1."]
    #[inline(always)]
    pub fn level1(&self) -> Level1R {
        Level1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Level 2."]
    #[inline(always)]
    pub fn level2(&self) -> Level2R {
        Level2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Level 3."]
    #[inline(always)]
    pub fn level3(&self) -> Level3R {
        Level3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Level 4."]
    #[inline(always)]
    pub fn level4(&self) -> Level4R {
        Level4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Level 5."]
    #[inline(always)]
    pub fn level5(&self) -> Level5R {
        Level5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Level 6."]
    #[inline(always)]
    pub fn level6(&self) -> Level6R {
        Level6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Level 7."]
    #[inline(always)]
    pub fn level7(&self) -> Level7R {
        Level7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Tamper levels 0 to 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__levela::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_LevelaSpec;
impl crate::RegisterSpec for AntiTamper_LevelaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__levela::R`](R) reader structure"]
impl crate::Readable for AntiTamper_LevelaSpec {}
#[doc = "`reset()` method sets anti_tamper__LEVELA to value 0"]
impl crate::Resettable for AntiTamper_LevelaSpec {}

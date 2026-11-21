#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `CCF` reader - Compensation Capacitor Fine Value"]
pub type CcfR = crate::FieldReader;
#[doc = "Field `CCF` writer - Compensation Capacitor Fine Value"]
pub type CcfW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCC` reader - Compensation Capacitor Coarse Value"]
pub type CccR = crate::FieldReader;
#[doc = "Field `CCC` writer - Compensation Capacitor Coarse Value"]
pub type CccW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CI` reader - Integration Capacitor Value"]
pub type CiR = crate::FieldReader;
#[doc = "Field `CI` writer - Integration Capacitor Value"]
pub type CiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CSD` reader - Charge Share Delay Value"]
pub type CsdR = crate::FieldReader;
#[doc = "Field `CSD` writer - Charge Share Delay Value"]
pub type CsdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - Compensation Capacitor Fine Value"]
    #[inline(always)]
    pub fn ccf(&self) -> CcfR {
        CcfR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Compensation Capacitor Coarse Value"]
    #[inline(always)]
    pub fn ccc(&self) -> CccR {
        CccR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Integration Capacitor Value"]
    #[inline(always)]
    pub fn ci(&self) -> CiR {
        CiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - Charge Share Delay Value"]
    #[inline(always)]
    pub fn csd(&self) -> CsdR {
        CsdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Compensation Capacitor Fine Value"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CcfW<'_, CtrlcSpec> {
        CcfW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Compensation Capacitor Coarse Value"]
    #[inline(always)]
    pub fn ccc(&mut self) -> CccW<'_, CtrlcSpec> {
        CccW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Integration Capacitor Value"]
    #[inline(always)]
    pub fn ci(&mut self) -> CiW<'_, CtrlcSpec> {
        CiW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Charge Share Delay Value"]
    #[inline(always)]
    pub fn csd(&mut self) -> CsdW<'_, CtrlcSpec> {
        CsdW::new(self, 24)
    }
}
#[doc = "CONTROL C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {}

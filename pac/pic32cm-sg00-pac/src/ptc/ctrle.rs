#[doc = "Register `CTRLE` reader"]
pub type R = crate::R<CtrleSpec>;
#[doc = "Register `CTRLE` writer"]
pub type W = crate::W<CtrleSpec>;
#[doc = "Field `AIPMPEN` reader - Analog Input Charge Pump Enable"]
pub type AipmpenR = crate::BitReader;
#[doc = "Field `AIPMPEN` writer - Analog Input Charge Pump Enable"]
pub type AipmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSC` reader - This field defines the PTC clock relative to the peripheral clock according to the table below."]
pub type PrscR = crate::FieldReader;
#[doc = "Field `PRSC` writer - This field defines the PTC clock relative to the peripheral clock according to the table below."]
pub type PrscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WKUPEXP` reader - Wake-Up Exponent"]
pub type WkupexpR = crate::FieldReader;
#[doc = "Field `WKUPEXP` writer - Wake-Up Exponent"]
pub type WkupexpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 4 - Analog Input Charge Pump Enable"]
    #[inline(always)]
    pub fn aipmpen(&self) -> AipmpenR {
        AipmpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - This field defines the PTC clock relative to the peripheral clock according to the table below."]
    #[inline(always)]
    pub fn prsc(&self) -> PrscR {
        PrscR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - Wake-Up Exponent"]
    #[inline(always)]
    pub fn wkupexp(&self) -> WkupexpR {
        WkupexpR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Analog Input Charge Pump Enable"]
    #[inline(always)]
    pub fn aipmpen(&mut self) -> AipmpenW<'_, CtrleSpec> {
        AipmpenW::new(self, 4)
    }
    #[doc = "Bits 8:10 - This field defines the PTC clock relative to the peripheral clock according to the table below."]
    #[inline(always)]
    pub fn prsc(&mut self) -> PrscW<'_, CtrleSpec> {
        PrscW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Wake-Up Exponent"]
    #[inline(always)]
    pub fn wkupexp(&mut self) -> WkupexpW<'_, CtrleSpec> {
        WkupexpW::new(self, 12)
    }
}
#[doc = "CONTROL E REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrleSpec;
impl crate::RegisterSpec for CtrleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrle::R`](R) reader structure"]
impl crate::Readable for CtrleSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrle::W`](W) writer structure"]
impl crate::Writable for CtrleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLE to value 0"]
impl crate::Resettable for CtrleSpec {}

#[doc = "Register `XOSCCAL` reader"]
pub type R = crate::R<XosccalSpec>;
#[doc = "Register `XOSCCAL` writer"]
pub type W = crate::W<XosccalSpec>;
#[doc = "Field `CAL` reader - XOSC Calibration"]
pub type CalR = crate::FieldReader<u16>;
#[doc = "Field `CAL` writer - XOSC Calibration"]
pub type CalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - XOSC Calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CalR {
        CalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - XOSC Calibration"]
    #[inline(always)]
    pub fn cal(&mut self) -> CalW<'_, XosccalSpec> {
        CalW::new(self, 0)
    }
}
#[doc = "XOSC Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xosccal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosccal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XosccalSpec;
impl crate::RegisterSpec for XosccalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xosccal::R`](R) reader structure"]
impl crate::Readable for XosccalSpec {}
#[doc = "`write(|w| ..)` method takes [`xosccal::W`](W) writer structure"]
impl crate::Writable for XosccalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XOSCCAL to value 0"]
impl crate::Resettable for XosccalSpec {}

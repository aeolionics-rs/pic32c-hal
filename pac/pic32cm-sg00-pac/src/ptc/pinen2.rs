#[doc = "Register `PINEN2` reader"]
pub type R = crate::R<Pinen2Spec>;
#[doc = "Register `PINEN2` writer"]
pub type W = crate::W<Pinen2Spec>;
#[doc = "Field `PINEN` reader - PTC-Line 63-32 Enable"]
pub type PinenR = crate::FieldReader;
#[doc = "Field `PINEN` writer - PTC-Line 63-32 Enable"]
pub type PinenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PTC-Line 63-32 Enable"]
    #[inline(always)]
    pub fn pinen(&self) -> PinenR {
        PinenR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PTC-Line 63-32 Enable"]
    #[inline(always)]
    pub fn pinen(&mut self) -> PinenW<'_, Pinen2Spec> {
        PinenW::new(self, 0)
    }
}
#[doc = "PTC Line Enable 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pinen2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinen2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinen2Spec;
impl crate::RegisterSpec for Pinen2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinen2::R`](R) reader structure"]
impl crate::Readable for Pinen2Spec {}
#[doc = "`write(|w| ..)` method takes [`pinen2::W`](W) writer structure"]
impl crate::Writable for Pinen2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINEN2 to value 0"]
impl crate::Resettable for Pinen2Spec {}

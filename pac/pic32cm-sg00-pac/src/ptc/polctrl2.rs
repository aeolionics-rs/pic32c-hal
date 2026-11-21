#[doc = "Register `POLCTRL2` reader"]
pub type R = crate::R<Polctrl2Spec>;
#[doc = "Register `POLCTRL2` writer"]
pub type W = crate::W<Polctrl2Spec>;
#[doc = "Field `POLARITY` reader - Choose X/Y lines Polarity \\[63:32\\]"]
pub type PolarityR = crate::FieldReader;
#[doc = "Field `POLARITY` writer - Choose X/Y lines Polarity \\[63:32\\]"]
pub type PolarityW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Choose X/Y lines Polarity \\[63:32\\]"]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Choose X/Y lines Polarity \\[63:32\\]"]
    #[inline(always)]
    pub fn polarity(&mut self) -> PolarityW<'_, Polctrl2Spec> {
        PolarityW::new(self, 0)
    }
}
#[doc = "Polarity Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`polctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Polctrl2Spec;
impl crate::RegisterSpec for Polctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polctrl2::R`](R) reader structure"]
impl crate::Readable for Polctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`polctrl2::W`](W) writer structure"]
impl crate::Writable for Polctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POLCTRL2 to value 0"]
impl crate::Resettable for Polctrl2Spec {}

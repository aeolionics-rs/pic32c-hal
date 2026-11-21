#[doc = "Register `POLCTRL1` reader"]
pub type R = crate::R<Polctrl1Spec>;
#[doc = "Register `POLCTRL1` writer"]
pub type W = crate::W<Polctrl1Spec>;
#[doc = "Field `POLARITY` reader - Choose X/Y lines Polarity \\[31:0\\]"]
pub type PolarityR = crate::FieldReader<u32>;
#[doc = "Field `POLARITY` writer - Choose X/Y lines Polarity \\[31:0\\]"]
pub type PolarityW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Choose X/Y lines Polarity \\[31:0\\]"]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Choose X/Y lines Polarity \\[31:0\\]"]
    #[inline(always)]
    pub fn polarity(&mut self) -> PolarityW<'_, Polctrl1Spec> {
        PolarityW::new(self, 0)
    }
}
#[doc = "Polarity Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`polctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Polctrl1Spec;
impl crate::RegisterSpec for Polctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polctrl1::R`](R) reader structure"]
impl crate::Readable for Polctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`polctrl1::W`](W) writer structure"]
impl crate::Writable for Polctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POLCTRL1 to value 0"]
impl crate::Resettable for Polctrl1Spec {}

#[doc = "Register `YSEL1` reader"]
pub type R = crate::R<Ysel1Spec>;
#[doc = "Register `YSEL1` writer"]
pub type W = crate::W<Ysel1Spec>;
#[doc = "Field `YSEL` reader - Y-Line Selection \\[31:0\\]"]
pub type YselR = crate::FieldReader<u32>;
#[doc = "Field `YSEL` writer - Y-Line Selection \\[31:0\\]"]
pub type YselW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Y-Line Selection \\[31:0\\]"]
    #[inline(always)]
    pub fn ysel(&self) -> YselR {
        YselR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Y-Line Selection \\[31:0\\]"]
    #[inline(always)]
    pub fn ysel(&mut self) -> YselW<'_, Ysel1Spec> {
        YselW::new(self, 0)
    }
}
#[doc = "Y Line Selection 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ysel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ysel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ysel1Spec;
impl crate::RegisterSpec for Ysel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ysel1::R`](R) reader structure"]
impl crate::Readable for Ysel1Spec {}
#[doc = "`write(|w| ..)` method takes [`ysel1::W`](W) writer structure"]
impl crate::Writable for Ysel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets YSEL1 to value 0"]
impl crate::Resettable for Ysel1Spec {}

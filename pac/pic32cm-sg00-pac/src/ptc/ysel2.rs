#[doc = "Register `YSEL2` reader"]
pub type R = crate::R<Ysel2Spec>;
#[doc = "Register `YSEL2` writer"]
pub type W = crate::W<Ysel2Spec>;
#[doc = "Field `YSEL` reader - Y-Line Selection \\[63:32\\]"]
pub type YselR = crate::FieldReader;
#[doc = "Field `YSEL` writer - Y-Line Selection \\[63:32\\]"]
pub type YselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Y-Line Selection \\[63:32\\]"]
    #[inline(always)]
    pub fn ysel(&self) -> YselR {
        YselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Y-Line Selection \\[63:32\\]"]
    #[inline(always)]
    pub fn ysel(&mut self) -> YselW<'_, Ysel2Spec> {
        YselW::new(self, 0)
    }
}
#[doc = "Y Line Selection 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ysel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ysel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ysel2Spec;
impl crate::RegisterSpec for Ysel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ysel2::R`](R) reader structure"]
impl crate::Readable for Ysel2Spec {}
#[doc = "`write(|w| ..)` method takes [`ysel2::W`](W) writer structure"]
impl crate::Writable for Ysel2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets YSEL2 to value 0"]
impl crate::Resettable for Ysel2Spec {}

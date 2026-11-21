#[doc = "Register `XSEL2` reader"]
pub type R = crate::R<Xsel2Spec>;
#[doc = "Register `XSEL2` writer"]
pub type W = crate::W<Xsel2Spec>;
#[doc = "Field `XSEL` reader - X-Line Selection \\[63:32\\]"]
pub type XselR = crate::FieldReader;
#[doc = "Field `XSEL` writer - X-Line Selection \\[63:32\\]"]
pub type XselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - X-Line Selection \\[63:32\\]"]
    #[inline(always)]
    pub fn xsel(&self) -> XselR {
        XselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - X-Line Selection \\[63:32\\]"]
    #[inline(always)]
    pub fn xsel(&mut self) -> XselW<'_, Xsel2Spec> {
        XselW::new(self, 0)
    }
}
#[doc = "X Line Selection 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xsel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xsel2Spec;
impl crate::RegisterSpec for Xsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xsel2::R`](R) reader structure"]
impl crate::Readable for Xsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`xsel2::W`](W) writer structure"]
impl crate::Writable for Xsel2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XSEL2 to value 0"]
impl crate::Resettable for Xsel2Spec {}

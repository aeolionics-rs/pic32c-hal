#[doc = "Register `XSEL1` reader"]
pub type R = crate::R<Xsel1Spec>;
#[doc = "Register `XSEL1` writer"]
pub type W = crate::W<Xsel1Spec>;
#[doc = "Field `XSEL` reader - X-Line Selection \\[31:0\\]"]
pub type XselR = crate::FieldReader<u32>;
#[doc = "Field `XSEL` writer - X-Line Selection \\[31:0\\]"]
pub type XselW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - X-Line Selection \\[31:0\\]"]
    #[inline(always)]
    pub fn xsel(&self) -> XselR {
        XselR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - X-Line Selection \\[31:0\\]"]
    #[inline(always)]
    pub fn xsel(&mut self) -> XselW<'_, Xsel1Spec> {
        XselW::new(self, 0)
    }
}
#[doc = "X Line Selection 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xsel1Spec;
impl crate::RegisterSpec for Xsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xsel1::R`](R) reader structure"]
impl crate::Readable for Xsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`xsel1::W`](W) writer structure"]
impl crate::Writable for Xsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XSEL1 to value 0"]
impl crate::Resettable for Xsel1Spec {}

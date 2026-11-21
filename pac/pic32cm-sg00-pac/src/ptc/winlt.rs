#[doc = "Register `WINLT` reader"]
pub type R = crate::R<WinltSpec>;
#[doc = "Register `WINLT` writer"]
pub type W = crate::W<WinltSpec>;
#[doc = "Field `WCLT` reader - Window Comparator Low Threshold"]
pub type WcltR = crate::FieldReader<u16>;
#[doc = "Field `WCLT` writer - Window Comparator Low Threshold"]
pub type WcltW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Window Comparator Low Threshold"]
    #[inline(always)]
    pub fn wclt(&self) -> WcltR {
        WcltR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window Comparator Low Threshold"]
    #[inline(always)]
    pub fn wclt(&mut self) -> WcltW<'_, WinltSpec> {
        WcltW::new(self, 0)
    }
}
#[doc = "Window Comparator Low Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`winlt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winlt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinltSpec;
impl crate::RegisterSpec for WinltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winlt::R`](R) reader structure"]
impl crate::Readable for WinltSpec {}
#[doc = "`write(|w| ..)` method takes [`winlt::W`](W) writer structure"]
impl crate::Writable for WinltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WINLT to value 0"]
impl crate::Resettable for WinltSpec {}

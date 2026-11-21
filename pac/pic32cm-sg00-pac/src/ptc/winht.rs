#[doc = "Register `WINHT` reader"]
pub type R = crate::R<WinhtSpec>;
#[doc = "Register `WINHT` writer"]
pub type W = crate::W<WinhtSpec>;
#[doc = "Field `WCHT` reader - Window Comparator High Threshold"]
pub type WchtR = crate::FieldReader<u16>;
#[doc = "Field `WCHT` writer - Window Comparator High Threshold"]
pub type WchtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Window Comparator High Threshold"]
    #[inline(always)]
    pub fn wcht(&self) -> WchtR {
        WchtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window Comparator High Threshold"]
    #[inline(always)]
    pub fn wcht(&mut self) -> WchtW<'_, WinhtSpec> {
        WchtW::new(self, 0)
    }
}
#[doc = "Window Comparator High Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`winht::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winht::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinhtSpec;
impl crate::RegisterSpec for WinhtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winht::R`](R) reader structure"]
impl crate::Readable for WinhtSpec {}
#[doc = "`write(|w| ..)` method takes [`winht::W`](W) writer structure"]
impl crate::Writable for WinhtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WINHT to value 0"]
impl crate::Resettable for WinhtSpec {}

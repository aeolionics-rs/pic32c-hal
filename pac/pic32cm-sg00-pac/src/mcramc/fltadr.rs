#[doc = "Register `FLTADR` reader"]
pub type R = crate::R<FltadrSpec>;
#[doc = "Register `FLTADR` writer"]
pub type W = crate::W<FltadrSpec>;
#[doc = "Field `FLTADR` reader - Fault Address Offset"]
pub type FltadrR = crate::FieldReader<u32>;
#[doc = "Field `FLTADR` writer - Fault Address Offset"]
pub type FltadrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Fault Address Offset"]
    #[inline(always)]
    pub fn fltadr(&self) -> FltadrR {
        FltadrR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Fault Address Offset"]
    #[inline(always)]
    pub fn fltadr(&mut self) -> FltadrW<'_, FltadrSpec> {
        FltadrW::new(self, 0)
    }
}
#[doc = "Fault Injection Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fltadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltadrSpec;
impl crate::RegisterSpec for FltadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltadr::R`](R) reader structure"]
impl crate::Readable for FltadrSpec {}
#[doc = "`write(|w| ..)` method takes [`fltadr::W`](W) writer structure"]
impl crate::Writable for FltadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLTADR to value 0"]
impl crate::Resettable for FltadrSpec {}

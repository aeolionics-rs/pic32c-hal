#[doc = "Register `FLTPTR` reader"]
pub type R = crate::R<FltptrSpec>;
#[doc = "Register `FLTPTR` writer"]
pub type W = crate::W<FltptrSpec>;
#[doc = "Field `FLT1PTR` reader - Single Fault Injection Bit Pointer"]
pub type Flt1ptrR = crate::FieldReader;
#[doc = "Field `FLT1PTR` writer - Single Fault Injection Bit Pointer"]
pub type Flt1ptrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FLT2PTR` reader - Double Fault Injection Bit Pointer"]
pub type Flt2ptrR = crate::FieldReader;
#[doc = "Field `FLT2PTR` writer - Double Fault Injection Bit Pointer"]
pub type Flt2ptrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Single Fault Injection Bit Pointer"]
    #[inline(always)]
    pub fn flt1ptr(&self) -> Flt1ptrR {
        Flt1ptrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Double Fault Injection Bit Pointer"]
    #[inline(always)]
    pub fn flt2ptr(&self) -> Flt2ptrR {
        Flt2ptrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Single Fault Injection Bit Pointer"]
    #[inline(always)]
    pub fn flt1ptr(&mut self) -> Flt1ptrW<'_, FltptrSpec> {
        Flt1ptrW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Double Fault Injection Bit Pointer"]
    #[inline(always)]
    pub fn flt2ptr(&mut self) -> Flt2ptrW<'_, FltptrSpec> {
        Flt2ptrW::new(self, 16)
    }
}
#[doc = "Fault Injection Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fltptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltptrSpec;
impl crate::RegisterSpec for FltptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltptr::R`](R) reader structure"]
impl crate::Readable for FltptrSpec {}
#[doc = "`write(|w| ..)` method takes [`fltptr::W`](W) writer structure"]
impl crate::Writable for FltptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLTPTR to value 0"]
impl crate::Resettable for FltptrSpec {}

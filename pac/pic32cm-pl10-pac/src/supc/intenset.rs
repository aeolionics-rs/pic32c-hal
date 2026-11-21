#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `BODVDDRDY` reader - BODVDD Ready Interrupt Enable"]
pub type BodvddrdyR = crate::BitReader;
#[doc = "Field `BODVDDRDY` writer - BODVDD Ready Interrupt Enable"]
pub type BodvddrdyW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `VLM` reader - Voltage Level Monitor Interrupt Enable"]
pub type VlmR = crate::BitReader;
#[doc = "Field `VLM` writer - Voltage Level Monitor Interrupt Enable"]
pub type VlmW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `VDDIO2OK` reader - VDDIO2 OK Interrupt Enable"]
pub type Vddio2okR = crate::BitReader;
#[doc = "Field `VDDIO2OK` writer - VDDIO2 OK Interrupt Enable"]
pub type Vddio2okW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `VDDIO2LPMPOR` reader - VDDIO2 Low Power Mode POR Interrupt Enable"]
pub type Vddio2lpmporR = crate::BitReader;
#[doc = "Field `VDDIO2LPMPOR` writer - VDDIO2 Low Power Mode POR Interrupt Enable"]
pub type Vddio2lpmporW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - BODVDD Ready Interrupt Enable"]
    #[inline(always)]
    pub fn bodvddrdy(&self) -> BodvddrdyR {
        BodvddrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Level Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn vlm(&self) -> VlmR {
        VlmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VDDIO2 OK Interrupt Enable"]
    #[inline(always)]
    pub fn vddio2ok(&self) -> Vddio2okR {
        Vddio2okR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VDDIO2 Low Power Mode POR Interrupt Enable"]
    #[inline(always)]
    pub fn vddio2lpmpor(&self) -> Vddio2lpmporR {
        Vddio2lpmporR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BODVDD Ready Interrupt Enable"]
    #[inline(always)]
    pub fn bodvddrdy(&mut self) -> BodvddrdyW<'_, IntensetSpec> {
        BodvddrdyW::new(self, 0)
    }
    #[doc = "Bit 3 - Voltage Level Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn vlm(&mut self) -> VlmW<'_, IntensetSpec> {
        VlmW::new(self, 3)
    }
    #[doc = "Bit 4 - VDDIO2 OK Interrupt Enable"]
    #[inline(always)]
    pub fn vddio2ok(&mut self) -> Vddio2okW<'_, IntensetSpec> {
        Vddio2okW::new(self, 4)
    }
    #[doc = "Bit 5 - VDDIO2 Low Power Mode POR Interrupt Enable"]
    #[inline(always)]
    pub fn vddio2lpmpor(&mut self) -> Vddio2lpmporW<'_, IntensetSpec> {
        Vddio2lpmporW::new(self, 5)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x39;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}

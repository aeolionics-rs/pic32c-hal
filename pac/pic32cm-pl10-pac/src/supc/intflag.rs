#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `BODVDDRDY` reader - BODVDD Ready"]
pub type BodvddrdyR = crate::BitReader;
#[doc = "Field `BODVDDRDY` writer - BODVDD Ready"]
pub type BodvddrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VLM` reader - Voltage Level Monitor"]
pub type VlmR = crate::BitReader;
#[doc = "Field `VLM` writer - Voltage Level Monitor"]
pub type VlmW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VDDIO2OK` reader - VDDIO2 OK"]
pub type Vddio2okR = crate::BitReader;
#[doc = "Field `VDDIO2OK` writer - VDDIO2 OK"]
pub type Vddio2okW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VDDIO2LPMPOR` reader - VDDIO2 Low Power Mode POR"]
pub type Vddio2lpmporR = crate::BitReader;
#[doc = "Field `VDDIO2LPMPOR` writer - VDDIO2 Low Power Mode POR"]
pub type Vddio2lpmporW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    pub fn bodvddrdy(&self) -> BodvddrdyR {
        BodvddrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Level Monitor"]
    #[inline(always)]
    pub fn vlm(&self) -> VlmR {
        VlmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VDDIO2 OK"]
    #[inline(always)]
    pub fn vddio2ok(&self) -> Vddio2okR {
        Vddio2okR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VDDIO2 Low Power Mode POR"]
    #[inline(always)]
    pub fn vddio2lpmpor(&self) -> Vddio2lpmporR {
        Vddio2lpmporR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    pub fn bodvddrdy(&mut self) -> BodvddrdyW<'_, IntflagSpec> {
        BodvddrdyW::new(self, 0)
    }
    #[doc = "Bit 3 - Voltage Level Monitor"]
    #[inline(always)]
    pub fn vlm(&mut self) -> VlmW<'_, IntflagSpec> {
        VlmW::new(self, 3)
    }
    #[doc = "Bit 4 - VDDIO2 OK"]
    #[inline(always)]
    pub fn vddio2ok(&mut self) -> Vddio2okW<'_, IntflagSpec> {
        Vddio2okW::new(self, 4)
    }
    #[doc = "Bit 5 - VDDIO2 Low Power Mode POR"]
    #[inline(always)]
    pub fn vddio2lpmpor(&mut self) -> Vddio2lpmporW<'_, IntflagSpec> {
        Vddio2lpmporW::new(self, 5)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x39;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}

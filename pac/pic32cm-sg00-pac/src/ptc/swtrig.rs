#[doc = "Register `SWTRIG` reader"]
pub type R = crate::R<SwtrigSpec>;
#[doc = "Register `SWTRIG` writer"]
pub type W = crate::W<SwtrigSpec>;
#[doc = "Field `STCONV` reader - Start Conversion"]
pub type StconvR = crate::BitReader;
#[doc = "Field `STCONV` writer - Start Conversion"]
pub type StconvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Conversion"]
    #[inline(always)]
    pub fn stconv(&self) -> StconvR {
        StconvR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion"]
    #[inline(always)]
    pub fn stconv(&mut self) -> StconvW<'_, SwtrigSpec> {
        StconvW::new(self, 0)
    }
}
#[doc = "Software Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrigSpec;
impl crate::RegisterSpec for SwtrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrig::R`](R) reader structure"]
impl crate::Readable for SwtrigSpec {}
#[doc = "`write(|w| ..)` method takes [`swtrig::W`](W) writer structure"]
impl crate::Writable for SwtrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWTRIG to value 0"]
impl crate::Resettable for SwtrigSpec {}

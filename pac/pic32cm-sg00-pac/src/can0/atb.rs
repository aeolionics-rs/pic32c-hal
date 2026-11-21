#[doc = "Register `ATB` reader"]
pub type R = crate::R<AtbSpec>;
#[doc = "Register `ATB` writer"]
pub type W = crate::W<AtbSpec>;
#[doc = "Field `TB` reader - Timebase for timestamp generation"]
pub type TbR = crate::FieldReader<u32>;
#[doc = "Field `TB` writer - Timebase for timestamp generation"]
pub type TbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timebase for timestamp generation"]
    #[inline(always)]
    pub fn tb(&self) -> TbR {
        TbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timebase for timestamp generation"]
    #[inline(always)]
    pub fn tb(&mut self) -> TbW<'_, AtbSpec> {
        TbW::new(self, 0)
    }
}
#[doc = "Actual Timebase\n\nYou can [`read`](crate::Reg::read) this register and get [`atb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtbSpec;
impl crate::RegisterSpec for AtbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atb::R`](R) reader structure"]
impl crate::Readable for AtbSpec {}
#[doc = "`write(|w| ..)` method takes [`atb::W`](W) writer structure"]
impl crate::Writable for AtbSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets ATB to value 0"]
impl crate::Resettable for AtbSpec {}

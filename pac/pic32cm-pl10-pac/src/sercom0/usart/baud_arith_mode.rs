#[doc = "Register `BAUD_ARITH_MODE` reader"]
pub type R = crate::R<BaudArithModeSpec>;
#[doc = "Register `BAUD_ARITH_MODE` writer"]
pub type W = crate::W<BaudArithModeSpec>;
#[doc = "Field `BAUD` reader - Baud Value"]
pub type BaudR = crate::FieldReader<u16>;
#[doc = "Field `BAUD` writer - Baud Value"]
pub type BaudW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Baud Value"]
    #[inline(always)]
    pub fn baud(&self) -> BaudR {
        BaudR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Value"]
    #[inline(always)]
    pub fn baud(&mut self) -> BaudW<'_, BaudArithModeSpec> {
        BaudW::new(self, 0)
    }
}
#[doc = "USART Baud\n\nYou can [`read`](crate::Reg::read) this register and get [`baud_arith_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud_arith_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudArithModeSpec;
impl crate::RegisterSpec for BaudArithModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`baud_arith_mode::R`](R) reader structure"]
impl crate::Readable for BaudArithModeSpec {}
#[doc = "`write(|w| ..)` method takes [`baud_arith_mode::W`](W) writer structure"]
impl crate::Writable for BaudArithModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAUD_ARITH_MODE to value 0"]
impl crate::Resettable for BaudArithModeSpec {}

#[doc = "Register `QK_IER` reader"]
pub type R = crate::R<QkIerSpec>;
#[doc = "Register `QK_IER` writer"]
pub type W = crate::W<QkIerSpec>;
#[doc = "Field `qk_int_en` reader - 0: Disables all QuiddiKey interrupts 1: Enables all QuiddiKey interrupts that are enabled in the Interrupt Mask register"]
pub type QkIntEnR = crate::BitReader;
#[doc = "Field `qk_int_en` writer - 0: Disables all QuiddiKey interrupts 1: Enables all QuiddiKey interrupts that are enabled in the Interrupt Mask register"]
pub type QkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Disables all QuiddiKey interrupts 1: Enables all QuiddiKey interrupts that are enabled in the Interrupt Mask register"]
    #[inline(always)]
    pub fn qk_int_en(&self) -> QkIntEnR {
        QkIntEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Disables all QuiddiKey interrupts 1: Enables all QuiddiKey interrupts that are enabled in the Interrupt Mask register"]
    #[inline(always)]
    pub fn qk_int_en(&mut self) -> QkIntEnW<'_, QkIerSpec> {
        QkIntEnW::new(self, 0)
    }
}
#[doc = "Interrupt Enable register; With the Interrupt Enable register all QuiddiKey interrupts can be enabled or disabled, without changing the Interrupt Mask register.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkIerSpec;
impl crate::RegisterSpec for QkIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_ier::R`](R) reader structure"]
impl crate::Readable for QkIerSpec {}
#[doc = "`write(|w| ..)` method takes [`qk_ier::W`](W) writer structure"]
impl crate::Writable for QkIerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QK_IER to value 0"]
impl crate::Resettable for QkIerSpec {}

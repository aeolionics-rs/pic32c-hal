#[doc = "Register `QK_ISR` reader"]
pub type R = crate::R<QkIsrSpec>;
#[doc = "Register `QK_ISR` writer"]
pub type W = crate::W<QkIsrSpec>;
#[doc = "Field `qk_int_busy` reader - A negative edge has occurred on qk_busy, which means that an operation has completed"]
pub type QkIntBusyR = crate::BitReader;
#[doc = "Field `qk_int_busy` writer - A negative edge has occurred on qk_busy, which means that an operation has completed"]
pub type QkIntBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_ok` reader - A positive edge has occurred on qk_ok, which means that an operation successfully completed"]
pub type QkIntOkR = crate::BitReader;
#[doc = "Field `qk_int_ok` writer - A positive edge has occurred on qk_ok, which means that an operation successfully completed"]
pub type QkIntOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_error` reader - A positive edge has occurred on qk_error, which means that an operation has failed"]
pub type QkIntErrorR = crate::BitReader;
#[doc = "Field `qk_int_error` writer - A positive edge has occurred on qk_error, which means that an operation has failed"]
pub type QkIntErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_zeroized` reader - A positive edge has occurred on qk_zeroized, which means that QuiddiKey has moved to the Zeroized or Locked state"]
pub type QkIntZeroizedR = crate::BitReader;
#[doc = "Field `qk_int_zeroized` writer - A positive edge has occurred on qk_zeroized, which means that QuiddiKey has moved to the Zeroized or Locked state"]
pub type QkIntZeroizedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_rejected` reader - A positive edge has occurred on qk_rejected, which means that a command was rejected"]
pub type QkIntRejectedR = crate::BitReader;
#[doc = "Field `qk_int_rejected` writer - A positive edge has occurred on qk_rejected, which means that a command was rejected"]
pub type QkIntRejectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_di_request` reader - A positive edge has occurred on qk_di_request, which means that a data in transfer is requested via the QK_DIR register"]
pub type QkIntDiRequestR = crate::BitReader;
#[doc = "Field `qk_int_di_request` writer - A positive edge has occurred on qk_di_request, which means that a data in transfer is requested via the QK_DIR register"]
pub type QkIntDiRequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_do_request` reader - A positive edge has occurred on qk_do_request, which means that a data out transfer is requested via the QK_DOR register"]
pub type QkIntDoRequestR = crate::BitReader;
#[doc = "Field `qk_int_do_request` writer - A positive edge has occurred on qk_do_request, which means that a data out transfer is requested via the QK_DOR register"]
pub type QkIntDoRequestW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - A negative edge has occurred on qk_busy, which means that an operation has completed"]
    #[inline(always)]
    pub fn qk_int_busy(&self) -> QkIntBusyR {
        QkIntBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A positive edge has occurred on qk_ok, which means that an operation successfully completed"]
    #[inline(always)]
    pub fn qk_int_ok(&self) -> QkIntOkR {
        QkIntOkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A positive edge has occurred on qk_error, which means that an operation has failed"]
    #[inline(always)]
    pub fn qk_int_error(&self) -> QkIntErrorR {
        QkIntErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A positive edge has occurred on qk_zeroized, which means that QuiddiKey has moved to the Zeroized or Locked state"]
    #[inline(always)]
    pub fn qk_int_zeroized(&self) -> QkIntZeroizedR {
        QkIntZeroizedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A positive edge has occurred on qk_rejected, which means that a command was rejected"]
    #[inline(always)]
    pub fn qk_int_rejected(&self) -> QkIntRejectedR {
        QkIntRejectedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A positive edge has occurred on qk_di_request, which means that a data in transfer is requested via the QK_DIR register"]
    #[inline(always)]
    pub fn qk_int_di_request(&self) -> QkIntDiRequestR {
        QkIntDiRequestR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A positive edge has occurred on qk_do_request, which means that a data out transfer is requested via the QK_DOR register"]
    #[inline(always)]
    pub fn qk_int_do_request(&self) -> QkIntDoRequestR {
        QkIntDoRequestR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A negative edge has occurred on qk_busy, which means that an operation has completed"]
    #[inline(always)]
    pub fn qk_int_busy(&mut self) -> QkIntBusyW<'_, QkIsrSpec> {
        QkIntBusyW::new(self, 0)
    }
    #[doc = "Bit 1 - A positive edge has occurred on qk_ok, which means that an operation successfully completed"]
    #[inline(always)]
    pub fn qk_int_ok(&mut self) -> QkIntOkW<'_, QkIsrSpec> {
        QkIntOkW::new(self, 1)
    }
    #[doc = "Bit 2 - A positive edge has occurred on qk_error, which means that an operation has failed"]
    #[inline(always)]
    pub fn qk_int_error(&mut self) -> QkIntErrorW<'_, QkIsrSpec> {
        QkIntErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - A positive edge has occurred on qk_zeroized, which means that QuiddiKey has moved to the Zeroized or Locked state"]
    #[inline(always)]
    pub fn qk_int_zeroized(&mut self) -> QkIntZeroizedW<'_, QkIsrSpec> {
        QkIntZeroizedW::new(self, 3)
    }
    #[doc = "Bit 4 - A positive edge has occurred on qk_rejected, which means that a command was rejected"]
    #[inline(always)]
    pub fn qk_int_rejected(&mut self) -> QkIntRejectedW<'_, QkIsrSpec> {
        QkIntRejectedW::new(self, 4)
    }
    #[doc = "Bit 5 - A positive edge has occurred on qk_di_request, which means that a data in transfer is requested via the QK_DIR register"]
    #[inline(always)]
    pub fn qk_int_di_request(&mut self) -> QkIntDiRequestW<'_, QkIsrSpec> {
        QkIntDiRequestW::new(self, 5)
    }
    #[doc = "Bit 6 - A positive edge has occurred on qk_do_request, which means that a data out transfer is requested via the QK_DOR register"]
    #[inline(always)]
    pub fn qk_int_do_request(&mut self) -> QkIntDoRequestW<'_, QkIsrSpec> {
        QkIntDoRequestW::new(self, 6)
    }
}
#[doc = "Interrupt Status register; The Interrupt Status register indicates which interrupt has occurred. Multiple interrupts may occur at the same time (e.g. the busy interrupt occurs together with one of the ok and error interrupts). Writing 1 to a bit, clears the corresponding bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkIsrSpec;
impl crate::RegisterSpec for QkIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_isr::R`](R) reader structure"]
impl crate::Readable for QkIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`qk_isr::W`](W) writer structure"]
impl crate::Writable for QkIsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QK_ISR to value 0"]
impl crate::Resettable for QkIsrSpec {}

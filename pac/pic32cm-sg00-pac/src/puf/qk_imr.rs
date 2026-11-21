#[doc = "Register `QK_IMR` reader"]
pub type R = crate::R<QkImrSpec>;
#[doc = "Register `QK_IMR` writer"]
pub type W = crate::W<QkImrSpec>;
#[doc = "Field `qk_int_en_busy` reader - Enables the busy interrupt"]
pub type QkIntEnBusyR = crate::BitReader;
#[doc = "Field `qk_int_en_busy` writer - Enables the busy interrupt"]
pub type QkIntEnBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_en_ok` reader - Enables the ok interrupt"]
pub type QkIntEnOkR = crate::BitReader;
#[doc = "Field `qk_int_en_ok` writer - Enables the ok interrupt"]
pub type QkIntEnOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_en_error` reader - Enables the error interrupt"]
pub type QkIntEnErrorR = crate::BitReader;
#[doc = "Field `qk_int_en_error` writer - Enables the error interrupt"]
pub type QkIntEnErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_en_zeroized` reader - Enables the zeroized interrupt"]
pub type QkIntEnZeroizedR = crate::BitReader;
#[doc = "Field `qk_int_en_zeroized` writer - Enables the zeroized interrupt"]
pub type QkIntEnZeroizedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_en_rejected` reader - Enables the rejected interrupt"]
pub type QkIntEnRejectedR = crate::BitReader;
#[doc = "Field `qk_int_en_rejected` writer - Enables the rejected interrupt"]
pub type QkIntEnRejectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_en_di_request` reader - Enables the data in request interrupt"]
pub type QkIntEnDiRequestR = crate::BitReader;
#[doc = "Field `qk_int_en_di_request` writer - Enables the data in request interrupt"]
pub type QkIntEnDiRequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_int_en_do_request` reader - Enables the data out request interrupt"]
pub type QkIntEnDoRequestR = crate::BitReader;
#[doc = "Field `qk_int_en_do_request` writer - Enables the data out request interrupt"]
pub type QkIntEnDoRequestW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the busy interrupt"]
    #[inline(always)]
    pub fn qk_int_en_busy(&self) -> QkIntEnBusyR {
        QkIntEnBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the ok interrupt"]
    #[inline(always)]
    pub fn qk_int_en_ok(&self) -> QkIntEnOkR {
        QkIntEnOkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the error interrupt"]
    #[inline(always)]
    pub fn qk_int_en_error(&self) -> QkIntEnErrorR {
        QkIntEnErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the zeroized interrupt"]
    #[inline(always)]
    pub fn qk_int_en_zeroized(&self) -> QkIntEnZeroizedR {
        QkIntEnZeroizedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the rejected interrupt"]
    #[inline(always)]
    pub fn qk_int_en_rejected(&self) -> QkIntEnRejectedR {
        QkIntEnRejectedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the data in request interrupt"]
    #[inline(always)]
    pub fn qk_int_en_di_request(&self) -> QkIntEnDiRequestR {
        QkIntEnDiRequestR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables the data out request interrupt"]
    #[inline(always)]
    pub fn qk_int_en_do_request(&self) -> QkIntEnDoRequestR {
        QkIntEnDoRequestR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the busy interrupt"]
    #[inline(always)]
    pub fn qk_int_en_busy(&mut self) -> QkIntEnBusyW<'_, QkImrSpec> {
        QkIntEnBusyW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the ok interrupt"]
    #[inline(always)]
    pub fn qk_int_en_ok(&mut self) -> QkIntEnOkW<'_, QkImrSpec> {
        QkIntEnOkW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the error interrupt"]
    #[inline(always)]
    pub fn qk_int_en_error(&mut self) -> QkIntEnErrorW<'_, QkImrSpec> {
        QkIntEnErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables the zeroized interrupt"]
    #[inline(always)]
    pub fn qk_int_en_zeroized(&mut self) -> QkIntEnZeroizedW<'_, QkImrSpec> {
        QkIntEnZeroizedW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables the rejected interrupt"]
    #[inline(always)]
    pub fn qk_int_en_rejected(&mut self) -> QkIntEnRejectedW<'_, QkImrSpec> {
        QkIntEnRejectedW::new(self, 4)
    }
    #[doc = "Bit 5 - Enables the data in request interrupt"]
    #[inline(always)]
    pub fn qk_int_en_di_request(&mut self) -> QkIntEnDiRequestW<'_, QkImrSpec> {
        QkIntEnDiRequestW::new(self, 5)
    }
    #[doc = "Bit 6 - Enables the data out request interrupt"]
    #[inline(always)]
    pub fn qk_int_en_do_request(&mut self) -> QkIntEnDoRequestW<'_, QkImrSpec> {
        QkIntEnDoRequestW::new(self, 6)
    }
}
#[doc = "Interrupt Mask register; Individual QuiddiKey interrupts can be enabled or disabled with the Interrupt Mask register.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkImrSpec;
impl crate::RegisterSpec for QkImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_imr::R`](R) reader structure"]
impl crate::Readable for QkImrSpec {}
#[doc = "`write(|w| ..)` method takes [`qk_imr::W`](W) writer structure"]
impl crate::Writable for QkImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QK_IMR to value 0"]
impl crate::Resettable for QkImrSpec {}

#[doc = "Register `QK_SR` reader"]
pub type R = crate::R<QkSrSpec>;
#[doc = "Register `QK_SR` writer"]
pub type W = crate::W<QkSrSpec>;
#[doc = "Field `qk_busy` reader - Operation is in progress"]
pub type QkBusyR = crate::BitReader;
#[doc = "Field `qk_busy` writer - Operation is in progress"]
pub type QkBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_ok` reader - Last operation was successful"]
pub type QkOkR = crate::BitReader;
#[doc = "Field `qk_ok` writer - Last operation was successful"]
pub type QkOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_error` reader - Last operation failed"]
pub type QkErrorR = crate::BitReader;
#[doc = "Field `qk_error` writer - Last operation failed"]
pub type QkErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_zeroized` reader - QuiddiKey is in Zeroized or Locked state"]
pub type QkZeroizedR = crate::BitReader;
#[doc = "Field `qk_zeroized` writer - QuiddiKey is in Zeroized or Locked state"]
pub type QkZeroizedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_rejected` reader - Read: Last command was rejected Write 1: Clears this bit"]
pub type QkRejectedR = crate::BitReader;
#[doc = "Field `qk_rejected` writer - Read: Last command was rejected Write 1: Clears this bit"]
pub type QkRejectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_di_request` reader - Request for data in transfer via the QK_DIR register"]
pub type QkDiRequestR = crate::BitReader;
#[doc = "Field `qk_di_request` writer - Request for data in transfer via the QK_DIR register"]
pub type QkDiRequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_do_request` reader - Request for data out transfer via the QK_DOR register"]
pub type QkDoRequestR = crate::BitReader;
#[doc = "Field `qk_do_request` writer - Request for data out transfer via the QK_DOR register"]
pub type QkDoRequestW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operation is in progress"]
    #[inline(always)]
    pub fn qk_busy(&self) -> QkBusyR {
        QkBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Last operation was successful"]
    #[inline(always)]
    pub fn qk_ok(&self) -> QkOkR {
        QkOkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Last operation failed"]
    #[inline(always)]
    pub fn qk_error(&self) -> QkErrorR {
        QkErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - QuiddiKey is in Zeroized or Locked state"]
    #[inline(always)]
    pub fn qk_zeroized(&self) -> QkZeroizedR {
        QkZeroizedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read: Last command was rejected Write 1: Clears this bit"]
    #[inline(always)]
    pub fn qk_rejected(&self) -> QkRejectedR {
        QkRejectedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Request for data in transfer via the QK_DIR register"]
    #[inline(always)]
    pub fn qk_di_request(&self) -> QkDiRequestR {
        QkDiRequestR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Request for data out transfer via the QK_DOR register"]
    #[inline(always)]
    pub fn qk_do_request(&self) -> QkDoRequestR {
        QkDoRequestR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation is in progress"]
    #[inline(always)]
    pub fn qk_busy(&mut self) -> QkBusyW<'_, QkSrSpec> {
        QkBusyW::new(self, 0)
    }
    #[doc = "Bit 1 - Last operation was successful"]
    #[inline(always)]
    pub fn qk_ok(&mut self) -> QkOkW<'_, QkSrSpec> {
        QkOkW::new(self, 1)
    }
    #[doc = "Bit 2 - Last operation failed"]
    #[inline(always)]
    pub fn qk_error(&mut self) -> QkErrorW<'_, QkSrSpec> {
        QkErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - QuiddiKey is in Zeroized or Locked state"]
    #[inline(always)]
    pub fn qk_zeroized(&mut self) -> QkZeroizedW<'_, QkSrSpec> {
        QkZeroizedW::new(self, 3)
    }
    #[doc = "Bit 4 - Read: Last command was rejected Write 1: Clears this bit"]
    #[inline(always)]
    pub fn qk_rejected(&mut self) -> QkRejectedW<'_, QkSrSpec> {
        QkRejectedW::new(self, 4)
    }
    #[doc = "Bit 5 - Request for data in transfer via the QK_DIR register"]
    #[inline(always)]
    pub fn qk_di_request(&mut self) -> QkDiRequestW<'_, QkSrSpec> {
        QkDiRequestW::new(self, 5)
    }
    #[doc = "Bit 6 - Request for data out transfer via the QK_DOR register"]
    #[inline(always)]
    pub fn qk_do_request(&mut self) -> QkDoRequestW<'_, QkSrSpec> {
        QkDoRequestW::new(self, 6)
    }
}
#[doc = "Status register; The Status register shows the current status of QuiddiKey and indicates whether a data transfer is requested.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkSrSpec;
impl crate::RegisterSpec for QkSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_sr::R`](R) reader structure"]
impl crate::Readable for QkSrSpec {}
#[doc = "`write(|w| ..)` method takes [`qk_sr::W`](W) writer structure"]
impl crate::Writable for QkSrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QK_SR to value 0x01"]
impl crate::Resettable for QkSrSpec {
    const RESET_VALUE: u32 = 0x01;
}

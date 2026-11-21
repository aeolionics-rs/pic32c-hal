#[doc = "Register `CryptoMaster_DMA_Registers__PUSH_LEN` reader"]
pub type R = crate::R<CryptoMasterDmaRegisters_PushLenSpec>;
#[doc = "Register `CryptoMaster_DMA_Registers__PUSH_LEN` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_PushLenSpec>;
#[doc = "Field `PUSH_LEN` reader - "]
pub type PushLenR = crate::FieldReader<u32>;
#[doc = "Field `PUSH_LEN` writer - "]
pub type PushLenW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `PUSH_CSTADDR` reader - "]
pub type PushCstaddrR = crate::BitReader;
#[doc = "Field `PUSH_CSTADDR` writer - "]
pub type PushCstaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSH_REALIGN` reader - "]
pub type PushRealignR = crate::BitReader;
#[doc = "Field `PUSH_REALIGN` writer - "]
pub type PushRealignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSH_DISCARD` reader - "]
pub type PushDiscardR = crate::BitReader;
#[doc = "Field `PUSH_DISCARD` writer - "]
pub type PushDiscardW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn push_len(&self) -> PushLenR {
        PushLenR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn push_cstaddr(&self) -> PushCstaddrR {
        PushCstaddrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn push_realign(&self) -> PushRealignR {
        PushRealignR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn push_discard(&self) -> PushDiscardR {
        PushDiscardR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn push_len(&mut self) -> PushLenW<'_, CryptoMasterDmaRegisters_PushLenSpec> {
        PushLenW::new(self, 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn push_cstaddr(&mut self) -> PushCstaddrW<'_, CryptoMasterDmaRegisters_PushLenSpec> {
        PushCstaddrW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn push_realign(&mut self) -> PushRealignW<'_, CryptoMasterDmaRegisters_PushLenSpec> {
        PushRealignW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn push_discard(&mut self) -> PushDiscardW<'_, CryptoMasterDmaRegisters_PushLenSpec> {
        PushDiscardW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__push_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__push_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_PushLenSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_PushLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_dma_registers__push_len::R`](R) reader structure"]
impl crate::Readable for CryptoMasterDmaRegisters_PushLenSpec {}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__push_len::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_PushLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__PUSH_LEN to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_PushLenSpec {}

#[doc = "Register `CryptoMaster_DMA_Registers__START` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_StartSpec>;
#[doc = "Field `START_FETCH` writer - "]
pub type StartFetchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_PUSH` writer - "]
pub type StartPushW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn start_fetch(&mut self) -> StartFetchW<'_, CryptoMasterDmaRegisters_StartSpec> {
        StartFetchW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn start_push(&mut self) -> StartPushW<'_, CryptoMasterDmaRegisters_StartSpec> {
        StartPushW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_StartSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_StartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__start::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_StartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__START to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_StartSpec {}

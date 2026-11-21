#[doc = "Register `CryptoMaster_DMA_Registers__INT_STATCLR` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_IntStatclrSpec>;
#[doc = "Field `INT_STATCLR` writer - "]
pub type IntStatclrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn int_statclr(&mut self) -> IntStatclrW<'_, CryptoMasterDmaRegisters_IntStatclrSpec> {
        IntStatclrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__int_statclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_IntStatclrSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_IntStatclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__int_statclr::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_IntStatclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__INT_STATCLR to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_IntStatclrSpec {}

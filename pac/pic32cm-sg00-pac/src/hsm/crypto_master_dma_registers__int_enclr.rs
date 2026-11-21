#[doc = "Register `CryptoMaster_DMA_Registers__INT_ENCLR` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_IntEnclrSpec>;
#[doc = "Field `INT_ENCLR` writer - "]
pub type IntEnclrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn int_enclr(&mut self) -> IntEnclrW<'_, CryptoMasterDmaRegisters_IntEnclrSpec> {
        IntEnclrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__int_enclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_IntEnclrSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_IntEnclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__int_enclr::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_IntEnclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__INT_ENCLR to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_IntEnclrSpec {}

#[doc = "Register `CryptoMaster_DMA_Registers__INT_ENSET` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_IntEnsetSpec>;
#[doc = "Field `INT_ENSET` writer - "]
pub type IntEnsetW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn int_enset(&mut self) -> IntEnsetW<'_, CryptoMasterDmaRegisters_IntEnsetSpec> {
        IntEnsetW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__int_enset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_IntEnsetSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_IntEnsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__int_enset::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_IntEnsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__INT_ENSET to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_IntEnsetSpec {}

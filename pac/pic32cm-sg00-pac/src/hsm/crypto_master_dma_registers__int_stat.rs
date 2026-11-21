#[doc = "Register `CryptoMaster_DMA_Registers__INT_STAT` reader"]
pub type R = crate::R<CryptoMasterDmaRegisters_IntStatSpec>;
#[doc = "Field `INT_STAT` reader - "]
pub type IntStatR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn int_stat(&self) -> IntStatR {
        IntStatR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__int_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_IntStatSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_IntStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_dma_registers__int_stat::R`](R) reader structure"]
impl crate::Readable for CryptoMasterDmaRegisters_IntStatSpec {}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__INT_STAT to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_IntStatSpec {}

#[doc = "Register `CryptoMaster_DMA_Registers__INT_STATRAW` reader"]
pub type R = crate::R<CryptoMasterDmaRegisters_IntStatrawSpec>;
#[doc = "Field `INT_STATRAW` reader - "]
pub type IntStatrawR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn int_statraw(&self) -> IntStatrawR {
        IntStatrawR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__int_statraw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_IntStatrawSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_IntStatrawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_dma_registers__int_statraw::R`](R) reader structure"]
impl crate::Readable for CryptoMasterDmaRegisters_IntStatrawSpec {}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__INT_STATRAW to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_IntStatrawSpec {}

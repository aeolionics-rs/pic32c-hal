#[doc = "Register `CryptoMaster_DMA_Registers__INT_EN` reader"]
pub type R = crate::R<CryptoMasterDmaRegisters_IntEnSpec>;
#[doc = "Register `CryptoMaster_DMA_Registers__INT_EN` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_IntEnSpec>;
#[doc = "Field `INT_EN` reader - "]
pub type IntEnR = crate::FieldReader;
#[doc = "Field `INT_EN` writer - "]
pub type IntEnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn int_en(&mut self) -> IntEnW<'_, CryptoMasterDmaRegisters_IntEnSpec> {
        IntEnW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_IntEnSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_IntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_dma_registers__int_en::R`](R) reader structure"]
impl crate::Readable for CryptoMasterDmaRegisters_IntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__int_en::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_IntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__INT_EN to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_IntEnSpec {}

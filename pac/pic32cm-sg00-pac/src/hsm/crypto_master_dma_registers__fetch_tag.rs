#[doc = "Register `CryptoMaster_DMA_Registers__FETCH_TAG` reader"]
pub type R = crate::R<CryptoMasterDmaRegisters_FetchTagSpec>;
#[doc = "Register `CryptoMaster_DMA_Registers__FETCH_TAG` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_FetchTagSpec>;
#[doc = "Field `FETCH_TAG` reader - "]
pub type FetchTagR = crate::FieldReader<u32>;
#[doc = "Field `FETCH_TAG` writer - "]
pub type FetchTagW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fetch_tag(&self) -> FetchTagR {
        FetchTagR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fetch_tag(&mut self) -> FetchTagW<'_, CryptoMasterDmaRegisters_FetchTagSpec> {
        FetchTagW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__fetch_tag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__fetch_tag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_FetchTagSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_FetchTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_dma_registers__fetch_tag::R`](R) reader structure"]
impl crate::Readable for CryptoMasterDmaRegisters_FetchTagSpec {}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__fetch_tag::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_FetchTagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__FETCH_TAG to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_FetchTagSpec {}

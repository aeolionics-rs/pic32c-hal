#[doc = "Register `CryptoMaster_DMA_Registers__FETCH_ADDR_MSB` reader"]
pub type R = crate::R<CryptoMasterDmaRegisters_FetchAddrMsbSpec>;
#[doc = "Register `CryptoMaster_DMA_Registers__FETCH_ADDR_MSB` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_FetchAddrMsbSpec>;
#[doc = "Field `FETCH_ADDR_MSB` reader - "]
pub type FetchAddrMsbR = crate::FieldReader<u32>;
#[doc = "Field `FETCH_ADDR_MSB` writer - "]
pub type FetchAddrMsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fetch_addr_msb(&self) -> FetchAddrMsbR {
        FetchAddrMsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fetch_addr_msb(&mut self) -> FetchAddrMsbW<'_, CryptoMasterDmaRegisters_FetchAddrMsbSpec> {
        FetchAddrMsbW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__fetch_addr_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__fetch_addr_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_FetchAddrMsbSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_FetchAddrMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_dma_registers__fetch_addr_msb::R`](R) reader structure"]
impl crate::Readable for CryptoMasterDmaRegisters_FetchAddrMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__fetch_addr_msb::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_FetchAddrMsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__FETCH_ADDR_MSB to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_FetchAddrMsbSpec {}

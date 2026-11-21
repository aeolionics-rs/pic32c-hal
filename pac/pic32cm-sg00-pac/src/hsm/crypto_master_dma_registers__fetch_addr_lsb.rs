#[doc = "Register `CryptoMaster_DMA_Registers__FETCH_ADDR_LSB` reader"]
pub type R = crate::R<CryptoMasterDmaRegisters_FetchAddrLsbSpec>;
#[doc = "Register `CryptoMaster_DMA_Registers__FETCH_ADDR_LSB` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_FetchAddrLsbSpec>;
#[doc = "Field `FETCH_ADDR_LSB` reader - "]
pub type FetchAddrLsbR = crate::FieldReader<u32>;
#[doc = "Field `FETCH_ADDR_LSB` writer - "]
pub type FetchAddrLsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fetch_addr_lsb(&self) -> FetchAddrLsbR {
        FetchAddrLsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fetch_addr_lsb(&mut self) -> FetchAddrLsbW<'_, CryptoMasterDmaRegisters_FetchAddrLsbSpec> {
        FetchAddrLsbW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__fetch_addr_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__fetch_addr_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_FetchAddrLsbSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_FetchAddrLsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_dma_registers__fetch_addr_lsb::R`](R) reader structure"]
impl crate::Readable for CryptoMasterDmaRegisters_FetchAddrLsbSpec {}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__fetch_addr_lsb::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_FetchAddrLsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__FETCH_ADDR_LSB to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_FetchAddrLsbSpec {}

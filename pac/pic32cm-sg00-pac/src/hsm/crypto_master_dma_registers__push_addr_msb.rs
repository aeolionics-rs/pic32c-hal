#[doc = "Register `CryptoMaster_DMA_Registers__PUSH_ADDR_MSB` reader"]
pub type R = crate::R<CryptoMasterDmaRegisters_PushAddrMsbSpec>;
#[doc = "Register `CryptoMaster_DMA_Registers__PUSH_ADDR_MSB` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_PushAddrMsbSpec>;
#[doc = "Field `PUSH_ADDR_MSB` reader - "]
pub type PushAddrMsbR = crate::FieldReader<u32>;
#[doc = "Field `PUSH_ADDR_MSB` writer - "]
pub type PushAddrMsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn push_addr_msb(&self) -> PushAddrMsbR {
        PushAddrMsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn push_addr_msb(&mut self) -> PushAddrMsbW<'_, CryptoMasterDmaRegisters_PushAddrMsbSpec> {
        PushAddrMsbW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__push_addr_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__push_addr_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_PushAddrMsbSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_PushAddrMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_dma_registers__push_addr_msb::R`](R) reader structure"]
impl crate::Readable for CryptoMasterDmaRegisters_PushAddrMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__push_addr_msb::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_PushAddrMsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__PUSH_ADDR_MSB to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_PushAddrMsbSpec {}

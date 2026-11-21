#[doc = "Register `CryptoMaster_DMA_Registers__FETCH_LEN` reader"]
pub type R = crate::R<CryptoMasterDmaRegisters_FetchLenSpec>;
#[doc = "Register `CryptoMaster_DMA_Registers__FETCH_LEN` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_FetchLenSpec>;
#[doc = "Field `FETCH_LEN` reader - "]
pub type FetchLenR = crate::FieldReader<u32>;
#[doc = "Field `FETCH_LEN` writer - "]
pub type FetchLenW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `FETCH_CSTADDR` reader - "]
pub type FetchCstaddrR = crate::BitReader;
#[doc = "Field `FETCH_CSTADDR` writer - "]
pub type FetchCstaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCH_REALIGN` reader - "]
pub type FetchRealignR = crate::BitReader;
#[doc = "Field `FETCH_REALIGN` writer - "]
pub type FetchRealignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCH_ZPADDING` reader - "]
pub type FetchZpaddingR = crate::BitReader;
#[doc = "Field `FETCH_ZPADDING` writer - "]
pub type FetchZpaddingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn fetch_len(&self) -> FetchLenR {
        FetchLenR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn fetch_cstaddr(&self) -> FetchCstaddrR {
        FetchCstaddrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fetch_realign(&self) -> FetchRealignR {
        FetchRealignR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn fetch_zpadding(&self) -> FetchZpaddingR {
        FetchZpaddingR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn fetch_len(&mut self) -> FetchLenW<'_, CryptoMasterDmaRegisters_FetchLenSpec> {
        FetchLenW::new(self, 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn fetch_cstaddr(&mut self) -> FetchCstaddrW<'_, CryptoMasterDmaRegisters_FetchLenSpec> {
        FetchCstaddrW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fetch_realign(&mut self) -> FetchRealignW<'_, CryptoMasterDmaRegisters_FetchLenSpec> {
        FetchRealignW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn fetch_zpadding(&mut self) -> FetchZpaddingW<'_, CryptoMasterDmaRegisters_FetchLenSpec> {
        FetchZpaddingW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__fetch_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__fetch_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_FetchLenSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_FetchLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_dma_registers__fetch_len::R`](R) reader structure"]
impl crate::Readable for CryptoMasterDmaRegisters_FetchLenSpec {}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__fetch_len::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_FetchLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__FETCH_LEN to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_FetchLenSpec {}

#[doc = "Register `CryptoMaster_DMA_Registers__STATUS` reader"]
pub type R = crate::R<CryptoMasterDmaRegisters_StatusSpec>;
#[doc = "Field `FETCH_BUSY` reader - "]
pub type FetchBusyR = crate::BitReader;
#[doc = "Field `PUSH_BUSY` reader - "]
pub type PushBusyR = crate::BitReader;
#[doc = "Field `FETCH_NOT_EMPTY` reader - "]
pub type FetchNotEmptyR = crate::BitReader;
#[doc = "Field `PUSH_WAITINGFIFO` reader - "]
pub type PushWaitingfifoR = crate::BitReader;
#[doc = "Field `SOFT_RST_BUSY` reader - "]
pub type SoftRstBusyR = crate::BitReader;
#[doc = "Field `PUSH_NBDATA` reader - "]
pub type PushNbdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fetch_busy(&self) -> FetchBusyR {
        FetchBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn push_busy(&self) -> PushBusyR {
        PushBusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fetch_not_empty(&self) -> FetchNotEmptyR {
        FetchNotEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn push_waitingfifo(&self) -> PushWaitingfifoR {
        PushWaitingfifoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn soft_rst_busy(&self) -> SoftRstBusyR {
        SoftRstBusyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn push_nbdata(&self) -> PushNbdataR {
        PushNbdataR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_StatusSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_dma_registers__status::R`](R) reader structure"]
impl crate::Readable for CryptoMasterDmaRegisters_StatusSpec {}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__STATUS to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_StatusSpec {}

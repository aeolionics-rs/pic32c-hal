#[doc = "Register `CryptoMaster_DMA_Registers__CONFIG` reader"]
pub type R = crate::R<CryptoMasterDmaRegisters_ConfigSpec>;
#[doc = "Register `CryptoMaster_DMA_Registers__CONFIG` writer"]
pub type W = crate::W<CryptoMasterDmaRegisters_ConfigSpec>;
#[doc = "Field `FETCH_CTRL_INDIRECT` reader - "]
pub type FetchCtrlIndirectR = crate::BitReader;
#[doc = "Field `FETCH_CTRL_INDIRECT` writer - "]
pub type FetchCtrlIndirectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSH_CTRL_INDIRECT` reader - "]
pub type PushCtrlIndirectR = crate::BitReader;
#[doc = "Field `PUSH_CTRL_INDIRECT` writer - "]
pub type PushCtrlIndirectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCH_STOP` reader - "]
pub type FetchStopR = crate::BitReader;
#[doc = "Field `FETCH_STOP` writer - "]
pub type FetchStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSH_STOP` reader - "]
pub type PushStopR = crate::BitReader;
#[doc = "Field `PUSH_STOP` writer - "]
pub type PushStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFT_RST` reader - "]
pub type SoftRstR = crate::BitReader;
#[doc = "Field `SOFT_RST` writer - "]
pub type SoftRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fetch_ctrl_indirect(&self) -> FetchCtrlIndirectR {
        FetchCtrlIndirectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn push_ctrl_indirect(&self) -> PushCtrlIndirectR {
        PushCtrlIndirectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fetch_stop(&self) -> FetchStopR {
        FetchStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn push_stop(&self) -> PushStopR {
        PushStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn soft_rst(&self) -> SoftRstR {
        SoftRstR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fetch_ctrl_indirect(&mut self) -> FetchCtrlIndirectW<'_, CryptoMasterDmaRegisters_ConfigSpec> {
        FetchCtrlIndirectW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn push_ctrl_indirect(&mut self) -> PushCtrlIndirectW<'_, CryptoMasterDmaRegisters_ConfigSpec> {
        PushCtrlIndirectW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fetch_stop(&mut self) -> FetchStopW<'_, CryptoMasterDmaRegisters_ConfigSpec> {
        FetchStopW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn push_stop(&mut self) -> PushStopW<'_, CryptoMasterDmaRegisters_ConfigSpec> {
        PushStopW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn soft_rst(&mut self) -> SoftRstW<'_, CryptoMasterDmaRegisters_ConfigSpec> {
        SoftRstW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterDmaRegisters_ConfigSpec;
impl crate::RegisterSpec for CryptoMasterDmaRegisters_ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_dma_registers__config::R`](R) reader structure"]
impl crate::Readable for CryptoMasterDmaRegisters_ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`crypto_master_dma_registers__config::W`](W) writer structure"]
impl crate::Writable for CryptoMasterDmaRegisters_ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CryptoMaster_DMA_Registers__CONFIG to value 0"]
impl crate::Resettable for CryptoMasterDmaRegisters_ConfigSpec {}

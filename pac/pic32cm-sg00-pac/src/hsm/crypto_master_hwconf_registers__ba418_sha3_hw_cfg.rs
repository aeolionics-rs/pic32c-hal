#[doc = "Register `CryptoMaster_HWConf_Registers__BA418_SHA3_HW_CFG` reader"]
pub type R = crate::R<CryptoMasterHwconfRegisters_Ba418Sha3HwCfgSpec>;
#[doc = "Field `BA418_SHA3_HW_CFG` reader - Generic g_Sha3CtxtEn value. BA418-SHA3 configuration."]
pub type Ba418Sha3HwCfgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Generic g_Sha3CtxtEn value. BA418-SHA3 configuration."]
    #[inline(always)]
    pub fn ba418_sha3_hw_cfg(&self) -> Ba418Sha3HwCfgR {
        Ba418Sha3HwCfgR::new((self.bits & 1) != 0)
    }
}
#[doc = "Generic g_Sha3CtxtEn value. BA418-SHA3 configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba418_sha3_hw_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterHwconfRegisters_Ba418Sha3HwCfgSpec;
impl crate::RegisterSpec for CryptoMasterHwconfRegisters_Ba418Sha3HwCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_hwconf_registers__ba418_sha3_hw_cfg::R`](R) reader structure"]
impl crate::Readable for CryptoMasterHwconfRegisters_Ba418Sha3HwCfgSpec {}
#[doc = "`reset()` method sets CryptoMaster_HWConf_Registers__BA418_SHA3_HW_CFG to value 0"]
impl crate::Resettable for CryptoMasterHwconfRegisters_Ba418Sha3HwCfgSpec {}

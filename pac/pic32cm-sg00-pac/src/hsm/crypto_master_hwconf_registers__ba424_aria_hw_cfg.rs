#[doc = "Register `CryptoMaster_HWConf_Registers__BA424_ARIA_HW_CFG` reader"]
pub type R = crate::R<CryptoMasterHwconfRegisters_Ba424AriaHwCfgSpec>;
#[doc = "Field `BA424_ARIA_HW_CFG` reader - Generic g_aria_modePoss value. BA424-Aria engine configuration."]
pub type Ba424AriaHwCfgR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Generic g_aria_modePoss value. BA424-Aria engine configuration."]
    #[inline(always)]
    pub fn ba424_aria_hw_cfg(&self) -> Ba424AriaHwCfgR {
        Ba424AriaHwCfgR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Generic g_aria_modePoss value. BA424-Aria engine configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba424_aria_hw_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterHwconfRegisters_Ba424AriaHwCfgSpec;
impl crate::RegisterSpec for CryptoMasterHwconfRegisters_Ba424AriaHwCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_hwconf_registers__ba424_aria_hw_cfg::R`](R) reader structure"]
impl crate::Readable for CryptoMasterHwconfRegisters_Ba424AriaHwCfgSpec {}
#[doc = "`reset()` method sets CryptoMaster_HWConf_Registers__BA424_ARIA_HW_CFG to value 0x017f"]
impl crate::Resettable for CryptoMasterHwconfRegisters_Ba424AriaHwCfgSpec {
    const RESET_VALUE: u32 = 0x017f;
}

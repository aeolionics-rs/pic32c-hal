#[doc = "Register `CryptoMaster_HWConf_Registers__BA419_SM4_HW_CFG` reader"]
pub type R = crate::R<CryptoMasterHwconfRegisters_Ba419Sm4HwCfgSpec>;
#[doc = "Field `BA419_SM4_HW_CFG` reader - Generic g_SM4ModesPoss value. BA419-SM4 engine configuration."]
pub type Ba419Sm4HwCfgR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Generic g_SM4ModesPoss value. BA419-SM4 engine configuration."]
    #[inline(always)]
    pub fn ba419_sm4_hw_cfg(&self) -> Ba419Sm4HwCfgR {
        Ba419Sm4HwCfgR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Generic g_SM4ModesPoss value. BA419-SM4 engine configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba419_sm4_hw_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterHwconfRegisters_Ba419Sm4HwCfgSpec;
impl crate::RegisterSpec for CryptoMasterHwconfRegisters_Ba419Sm4HwCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_hwconf_registers__ba419_sm4_hw_cfg::R`](R) reader structure"]
impl crate::Readable for CryptoMasterHwconfRegisters_Ba419Sm4HwCfgSpec {}
#[doc = "`reset()` method sets CryptoMaster_HWConf_Registers__BA419_SM4_HW_CFG to value 0x01ff"]
impl crate::Resettable for CryptoMasterHwconfRegisters_Ba419Sm4HwCfgSpec {
    const RESET_VALUE: u32 = 0x01ff;
}

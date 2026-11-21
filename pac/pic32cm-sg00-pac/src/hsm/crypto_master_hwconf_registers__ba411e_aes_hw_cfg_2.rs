#[doc = "Register `CryptoMaster_HWConf_Registers__BA411E_AES_HW_CFG_2` reader"]
pub type R = crate::R<CryptoMasterHwconfRegisters_Ba411eAesHwCfg2Spec>;
#[doc = "Field `BA411E_AES_HW_CFG_2_CTRSIZE` reader - Generic g_CtrSize value. BA411E-AES engine configuration."]
pub type Ba411eAesHwCfg2CtrsizeR = crate::FieldReader<u16>;
#[doc = "Field `BA411E_AES_HW_CFG_2_EXTKEYS` reader - Generic g_Ext_nb_AES_keys value. BA411E-AES engine configuration."]
pub type Ba411eAesHwCfg2ExtkeysR = crate::FieldReader;
#[doc = "Field `BA411E_AES_HW_CFG_2_IKGKEYS` reader - Generic g_IKG_nb_AES_keys value. BA411E-AES engine configuration."]
pub type Ba411eAesHwCfg2IkgkeysR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Generic g_CtrSize value. BA411E-AES engine configuration."]
    #[inline(always)]
    pub fn ba411e_aes_hw_cfg_2_ctrsize(&self) -> Ba411eAesHwCfg2CtrsizeR {
        Ba411eAesHwCfg2CtrsizeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 20:23 - Generic g_Ext_nb_AES_keys value. BA411E-AES engine configuration."]
    #[inline(always)]
    pub fn ba411e_aes_hw_cfg_2_extkeys(&self) -> Ba411eAesHwCfg2ExtkeysR {
        Ba411eAesHwCfg2ExtkeysR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Generic g_IKG_nb_AES_keys value. BA411E-AES engine configuration."]
    #[inline(always)]
    pub fn ba411e_aes_hw_cfg_2_ikgkeys(&self) -> Ba411eAesHwCfg2IkgkeysR {
        Ba411eAesHwCfg2IkgkeysR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Generic g_CtrSize value. BA411E-AES engine configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba411e_aes_hw_cfg_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterHwconfRegisters_Ba411eAesHwCfg2Spec;
impl crate::RegisterSpec for CryptoMasterHwconfRegisters_Ba411eAesHwCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_hwconf_registers__ba411e_aes_hw_cfg_2::R`](R) reader structure"]
impl crate::Readable for CryptoMasterHwconfRegisters_Ba411eAesHwCfg2Spec {}
#[doc = "`reset()` method sets CryptoMaster_HWConf_Registers__BA411E_AES_HW_CFG_2 to value 0x0020_0080"]
impl crate::Resettable for CryptoMasterHwconfRegisters_Ba411eAesHwCfg2Spec {
    const RESET_VALUE: u32 = 0x0020_0080;
}

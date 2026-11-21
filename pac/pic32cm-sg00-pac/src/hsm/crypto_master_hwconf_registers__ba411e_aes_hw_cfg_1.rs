#[doc = "Register `CryptoMaster_HWConf_Registers__BA411E_AES_HW_CFG_1` reader"]
pub type R = crate::R<CryptoMasterHwconfRegisters_Ba411eAesHwCfg1Spec>;
#[doc = "Field `BA411E_AES_HW_CFG_MODE` reader - Generic g_AesModesPoss value. BA411E-AES engine configuration."]
pub type Ba411eAesHwCfgModeR = crate::FieldReader<u16>;
#[doc = "Field `BA411E_AES_HW_CFG_CS` reader - Generic g_CS value. BA411E-AES engine configuration."]
pub type Ba411eAesHwCfgCsR = crate::BitReader;
#[doc = "Field `BA411E_AES_HW_CFG_MASKING` reader - Generic g_UseMasking value. BA411E-AES engine configuration."]
pub type Ba411eAesHwCfgMaskingR = crate::BitReader;
#[doc = "Field `BA411E_AES_HW_CFG_KEYSIZE` reader - Generic g_Keysize value. BA411E-AES engine configuration."]
pub type Ba411eAesHwCfgKeysizeR = crate::FieldReader;
#[doc = "Field `BA411E_AES_HW_CFG_CXSWITCH` reader - Generic g_CxSwitch value. BA411E-AES engine configuration."]
pub type Ba411eAesHwCfgCxswitchR = crate::BitReader;
#[doc = "Field `BA411E_AES_HW_CFG_GLITCHPROTECTION` reader - Generic g_GlitchProtection value. BA411E-AES engine configuration."]
pub type Ba411eAesHwCfgGlitchprotectionR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Generic g_AesModesPoss value. BA411E-AES engine configuration."]
    #[inline(always)]
    pub fn ba411e_aes_hw_cfg_mode(&self) -> Ba411eAesHwCfgModeR {
        Ba411eAesHwCfgModeR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Generic g_CS value. BA411E-AES engine configuration."]
    #[inline(always)]
    pub fn ba411e_aes_hw_cfg_cs(&self) -> Ba411eAesHwCfgCsR {
        Ba411eAesHwCfgCsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Generic g_UseMasking value. BA411E-AES engine configuration."]
    #[inline(always)]
    pub fn ba411e_aes_hw_cfg_masking(&self) -> Ba411eAesHwCfgMaskingR {
        Ba411eAesHwCfgMaskingR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Generic g_Keysize value. BA411E-AES engine configuration."]
    #[inline(always)]
    pub fn ba411e_aes_hw_cfg_keysize(&self) -> Ba411eAesHwCfgKeysizeR {
        Ba411eAesHwCfgKeysizeR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Generic g_CxSwitch value. BA411E-AES engine configuration."]
    #[inline(always)]
    pub fn ba411e_aes_hw_cfg_cxswitch(&self) -> Ba411eAesHwCfgCxswitchR {
        Ba411eAesHwCfgCxswitchR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Generic g_GlitchProtection value. BA411E-AES engine configuration."]
    #[inline(always)]
    pub fn ba411e_aes_hw_cfg_glitchprotection(&self) -> Ba411eAesHwCfgGlitchprotectionR {
        Ba411eAesHwCfgGlitchprotectionR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba411e_aes_hw_cfg_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterHwconfRegisters_Ba411eAesHwCfg1Spec;
impl crate::RegisterSpec for CryptoMasterHwconfRegisters_Ba411eAesHwCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_hwconf_registers__ba411e_aes_hw_cfg_1::R`](R) reader structure"]
impl crate::Readable for CryptoMasterHwconfRegisters_Ba411eAesHwCfg1Spec {}
#[doc = "`reset()` method sets CryptoMaster_HWConf_Registers__BA411E_AES_HW_CFG_1 to value 0x1f03_01ff"]
impl crate::Resettable for CryptoMasterHwconfRegisters_Ba411eAesHwCfg1Spec {
    const RESET_VALUE: u32 = 0x1f03_01ff;
}

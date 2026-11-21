#[doc = "Register `CryptoMaster_HWConf_Registers__BA413_HASH_HW_CFG` reader"]
pub type R = crate::R<CryptoMasterHwconfRegisters_Ba413HashHwCfgSpec>;
#[doc = "Field `BA413_HASH_HW_CFG_MASK` reader - Generic g_HashMaskFunc value. BA413-Hash engine configuration."]
pub type Ba413HashHwCfgMaskR = crate::FieldReader;
#[doc = "Field `BA413_HASH_HW_CFG_PADDING` reader - Generic g_HashPadding value. BA413-Hash engine configuration."]
pub type Ba413HashHwCfgPaddingR = crate::BitReader;
#[doc = "Field `BA413_HASH_HW_CFG_HMAC` reader - Generic g_HMAC_enabled value. BA413-Hash engine configuration."]
pub type Ba413HashHwCfgHmacR = crate::BitReader;
#[doc = "Field `BA413_HASH_HW_CFG_VERIFYDIGEST` reader - Generic g_HashVerifyDigest value. BA413-Hash engine configuration."]
pub type Ba413HashHwCfgVerifydigestR = crate::BitReader;
#[doc = "Field `BA413_HASH_HW_CFG_EXTKEYS` reader - Generic g_Ext_nb_Hash_keys value. BA413-Hash engine configuration."]
pub type Ba413HashHwCfgExtkeysR = crate::FieldReader;
#[doc = "Field `BA413_HASH_HW_CFG_IKGKEYS` reader - Generic g_IKG_nb_Hash_keys value. BA413-Hash engine configuration."]
pub type Ba413HashHwCfgIkgkeysR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Generic g_HashMaskFunc value. BA413-Hash engine configuration."]
    #[inline(always)]
    pub fn ba413_hash_hw_cfg_mask(&self) -> Ba413HashHwCfgMaskR {
        Ba413HashHwCfgMaskR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Generic g_HashPadding value. BA413-Hash engine configuration."]
    #[inline(always)]
    pub fn ba413_hash_hw_cfg_padding(&self) -> Ba413HashHwCfgPaddingR {
        Ba413HashHwCfgPaddingR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Generic g_HMAC_enabled value. BA413-Hash engine configuration."]
    #[inline(always)]
    pub fn ba413_hash_hw_cfg_hmac(&self) -> Ba413HashHwCfgHmacR {
        Ba413HashHwCfgHmacR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Generic g_HashVerifyDigest value. BA413-Hash engine configuration."]
    #[inline(always)]
    pub fn ba413_hash_hw_cfg_verifydigest(&self) -> Ba413HashHwCfgVerifydigestR {
        Ba413HashHwCfgVerifydigestR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Generic g_Ext_nb_Hash_keys value. BA413-Hash engine configuration."]
    #[inline(always)]
    pub fn ba413_hash_hw_cfg_extkeys(&self) -> Ba413HashHwCfgExtkeysR {
        Ba413HashHwCfgExtkeysR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Generic g_IKG_nb_Hash_keys value. BA413-Hash engine configuration."]
    #[inline(always)]
    pub fn ba413_hash_hw_cfg_ikgkeys(&self) -> Ba413HashHwCfgIkgkeysR {
        Ba413HashHwCfgIkgkeysR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba413_hash_hw_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterHwconfRegisters_Ba413HashHwCfgSpec;
impl crate::RegisterSpec for CryptoMasterHwconfRegisters_Ba413HashHwCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_hwconf_registers__ba413_hash_hw_cfg::R`](R) reader structure"]
impl crate::Readable for CryptoMasterHwconfRegisters_Ba413HashHwCfgSpec {}
#[doc = "`reset()` method sets CryptoMaster_HWConf_Registers__BA413_HASH_HW_CFG to value 0x0013_003e"]
impl crate::Resettable for CryptoMasterHwconfRegisters_Ba413HashHwCfgSpec {
    const RESET_VALUE: u32 = 0x0013_003e;
}

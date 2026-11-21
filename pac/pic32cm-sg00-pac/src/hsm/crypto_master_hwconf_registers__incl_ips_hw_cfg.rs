#[doc = "Register `CryptoMaster_HWConf_Registers__INCL_IPS_HW_CFG` reader"]
pub type R = crate::R<CryptoMasterHwconfRegisters_InclIpsHwCfgSpec>;
#[doc = "Field `BA411_AES_INCLUDED` reader - Generic g_IncludeAES value. BA411E-AES IP included if set"]
pub type Ba411AesIncludedR = crate::BitReader;
#[doc = "Field `BA415_HP_AES_GCM_INCLUDED` reader - Generic g_IncludeAESGCM value. BA415-HP-AES-GCM IP included if set"]
pub type Ba415HpAesGcmIncludedR = crate::BitReader;
#[doc = "Field `BA416_HP_AES_XTS_INCLUDED` reader - Generic g_IncludeAESXTS value. BA416-HP-AES-XTS IP included if set"]
pub type Ba416HpAesXtsIncludedR = crate::BitReader;
#[doc = "Field `BA412_DES_INCLUDED` reader - Generic g_IncludeDES value. BA412-3DES IP included if set"]
pub type Ba412DesIncludedR = crate::BitReader;
#[doc = "Field `BA413_HASH_INCLUDED` reader - Generic g_IncludeHASH value. BA413-HASH IP included if set"]
pub type Ba413HashIncludedR = crate::BitReader;
#[doc = "Field `BA417_CHACHAPOLY_INCLUDED` reader - Generic g_IncludeChachaPoly value. BA417-ChaChaPoly IP included if set"]
pub type Ba417ChachapolyIncludedR = crate::BitReader;
#[doc = "Field `BA418_SHA3_INCLUDED` reader - Generic g_IncludeSHA3 value. BA418-SHA3 IP included if set"]
pub type Ba418Sha3IncludedR = crate::BitReader;
#[doc = "Field `BA421_ZUC_INCLUDED` reader - Generic g_IncludeZUC value. BA421-ZUC IP included if set"]
pub type Ba421ZucIncludedR = crate::BitReader;
#[doc = "Field `BA419_SM4_INCLUDED` reader - Generic g_IncludeSM4 value. BA419-SM4 IP included if set"]
pub type Ba419Sm4IncludedR = crate::BitReader;
#[doc = "Field `BA414EP_PKE_INCLUDED` reader - Generic g_IncludePKE value. BA414EP-PKE IP included if set"]
pub type Ba414epPkeIncludedR = crate::BitReader;
#[doc = "Field `BA431_NDRNG_INCLUDED` reader - Generic g_IncludeNDRNG value. BA431-NDRNG IP included if set"]
pub type Ba431NdrngIncludedR = crate::BitReader;
#[doc = "Field `BA420_HP_CHACHAPOLY_INCLUDED` reader - Generic g_IncludeHPChachaPoly value. BA420-HP-ChaChaPoly IP included if set"]
pub type Ba420HpChachapolyIncludedR = crate::BitReader;
#[doc = "Field `BA423_SNOW3G_INCLUDED` reader - Generic g_IncludeSnow3G value. BA423-Snow3G IP included if set"]
pub type Ba423Snow3gIncludedR = crate::BitReader;
#[doc = "Field `BA422_KASUMI_INCLUDED` reader - Generic g_IncludeKasumi value. BA422-Kasumi IP included if set"]
pub type Ba422KasumiIncludedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Generic g_IncludeAES value. BA411E-AES IP included if set"]
    #[inline(always)]
    pub fn ba411_aes_included(&self) -> Ba411AesIncludedR {
        Ba411AesIncludedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generic g_IncludeAESGCM value. BA415-HP-AES-GCM IP included if set"]
    #[inline(always)]
    pub fn ba415_hp_aes_gcm_included(&self) -> Ba415HpAesGcmIncludedR {
        Ba415HpAesGcmIncludedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generic g_IncludeAESXTS value. BA416-HP-AES-XTS IP included if set"]
    #[inline(always)]
    pub fn ba416_hp_aes_xts_included(&self) -> Ba416HpAesXtsIncludedR {
        Ba416HpAesXtsIncludedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generic g_IncludeDES value. BA412-3DES IP included if set"]
    #[inline(always)]
    pub fn ba412_des_included(&self) -> Ba412DesIncludedR {
        Ba412DesIncludedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generic g_IncludeHASH value. BA413-HASH IP included if set"]
    #[inline(always)]
    pub fn ba413_hash_included(&self) -> Ba413HashIncludedR {
        Ba413HashIncludedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generic g_IncludeChachaPoly value. BA417-ChaChaPoly IP included if set"]
    #[inline(always)]
    pub fn ba417_chachapoly_included(&self) -> Ba417ChachapolyIncludedR {
        Ba417ChachapolyIncludedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generic g_IncludeSHA3 value. BA418-SHA3 IP included if set"]
    #[inline(always)]
    pub fn ba418_sha3_included(&self) -> Ba418Sha3IncludedR {
        Ba418Sha3IncludedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generic g_IncludeZUC value. BA421-ZUC IP included if set"]
    #[inline(always)]
    pub fn ba421_zuc_included(&self) -> Ba421ZucIncludedR {
        Ba421ZucIncludedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic g_IncludeSM4 value. BA419-SM4 IP included if set"]
    #[inline(always)]
    pub fn ba419_sm4_included(&self) -> Ba419Sm4IncludedR {
        Ba419Sm4IncludedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic g_IncludePKE value. BA414EP-PKE IP included if set"]
    #[inline(always)]
    pub fn ba414ep_pke_included(&self) -> Ba414epPkeIncludedR {
        Ba414epPkeIncludedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic g_IncludeNDRNG value. BA431-NDRNG IP included if set"]
    #[inline(always)]
    pub fn ba431_ndrng_included(&self) -> Ba431NdrngIncludedR {
        Ba431NdrngIncludedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generic g_IncludeHPChachaPoly value. BA420-HP-ChaChaPoly IP included if set"]
    #[inline(always)]
    pub fn ba420_hp_chachapoly_included(&self) -> Ba420HpChachapolyIncludedR {
        Ba420HpChachapolyIncludedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Generic g_IncludeSnow3G value. BA423-Snow3G IP included if set"]
    #[inline(always)]
    pub fn ba423_snow3g_included(&self) -> Ba423Snow3gIncludedR {
        Ba423Snow3gIncludedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Generic g_IncludeKasumi value. BA422-Kasumi IP included if set"]
    #[inline(always)]
    pub fn ba422_kasumi_included(&self) -> Ba422KasumiIncludedR {
        Ba422KasumiIncludedR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__incl_ips_hw_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoMasterHwconfRegisters_InclIpsHwCfgSpec;
impl crate::RegisterSpec for CryptoMasterHwconfRegisters_InclIpsHwCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_master_hwconf_registers__incl_ips_hw_cfg::R`](R) reader structure"]
impl crate::Readable for CryptoMasterHwconfRegisters_InclIpsHwCfgSpec {}
#[doc = "`reset()` method sets CryptoMaster_HWConf_Registers__INCL_IPS_HW_CFG to value 0x0611"]
impl crate::Resettable for CryptoMasterHwconfRegisters_InclIpsHwCfgSpec {
    const RESET_VALUE: u32 = 0x0611;
}

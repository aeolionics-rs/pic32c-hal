#[doc = "Register `QK_HW_INFO` reader"]
pub type R = crate::R<QkHwInfoSpec>;
#[doc = "Field `qk_config_sp_800_90` reader - 1: SP 800-90 option is included 0: SP 800-90 option is not included"]
pub type QkConfigSp800_90R = crate::BitReader;
#[doc = "Field `qk_config_bist` reader - 1: BIST is included 0: BIST is not included"]
pub type QkConfigBistR = crate::BitReader;
#[doc = "Field `qk_config_wrap` reader - 1: Wrap is included 0: Wrap is not included"]
pub type QkConfigWrapR = crate::BitReader;
#[doc = "QuiddiKey configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QkConfigTypeSelect {
    #[doc = "1: `1`"]
    IidQkSafe = 1,
    #[doc = "2: `10`"]
    IidQkPlus = 2,
}
impl From<QkConfigTypeSelect> for u8 {
    #[inline(always)]
    fn from(variant: QkConfigTypeSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QkConfigTypeSelect {
    type Ux = u8;
}
impl crate::IsEnum for QkConfigTypeSelect {}
#[doc = "Field `qk_config_type` reader - QuiddiKey configuration"]
pub type QkConfigTypeR = crate::FieldReader<QkConfigTypeSelect>;
impl QkConfigTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<QkConfigTypeSelect> {
        match self.bits {
            1 => Some(QkConfigTypeSelect::IidQkSafe),
            2 => Some(QkConfigTypeSelect::IidQkPlus),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iid_qk_safe(&self) -> bool {
        *self == QkConfigTypeSelect::IidQkSafe
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_iid_qk_plus(&self) -> bool {
        *self == QkConfigTypeSelect::IidQkPlus
    }
}
impl R {
    #[doc = "Bit 21 - 1: SP 800-90 option is included 0: SP 800-90 option is not included"]
    #[inline(always)]
    pub fn qk_config_sp_800_90(&self) -> QkConfigSp800_90R {
        QkConfigSp800_90R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: BIST is included 0: BIST is not included"]
    #[inline(always)]
    pub fn qk_config_bist(&self) -> QkConfigBistR {
        QkConfigBistR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Wrap is included 0: Wrap is not included"]
    #[inline(always)]
    pub fn qk_config_wrap(&self) -> QkConfigWrapR {
        QkConfigWrapR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:31 - QuiddiKey configuration"]
    #[inline(always)]
    pub fn qk_config_type(&self) -> QkConfigTypeR {
        QkConfigTypeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Module Information register; The Module Information register indicates which configuration of QuiddiKey is used for this module and which options it includes.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkHwInfoSpec;
impl crate::RegisterSpec for QkHwInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_hw_info::R`](R) reader structure"]
impl crate::Readable for QkHwInfoSpec {}
#[doc = "`reset()` method sets QK_HW_INFO to value 0x2140_0000"]
impl crate::Resettable for QkHwInfoSpec {
    const RESET_VALUE: u32 = 0x2140_0000;
}

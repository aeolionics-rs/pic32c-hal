#[doc = "Register `QK_ORR` reader"]
pub type R = crate::R<QkOrrSpec>;
#[doc = "Result code of last operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QkResultCodeSelect {
    #[doc = "0: `0`"]
    IidQkOk = 0,
    #[doc = "240: `11110000`"]
    IidQkErrProduct = 240,
    #[doc = "241: `11110001`"]
    IidQkErrProductPh2 = 241,
    #[doc = "242: `11110010`"]
    IidQkErrTransfer = 242,
    #[doc = "243: `11110011`"]
    IidQkErrTransferPh2 = 243,
    #[doc = "244: `11110100`"]
    IidQkErrAuth = 244,
    #[doc = "245: `11110101`"]
    IidQkErrAuthPh2 = 245,
    #[doc = "246: `11110110`"]
    IidQkErrPufQuality = 246,
    #[doc = "247: `11110111`"]
    IidQkErrContext = 247,
    #[doc = "248: `11111000`"]
    IidQkErrDestination = 248,
    #[doc = "255: `11111111`"]
    IidQkFailureSram = 255,
}
impl From<QkResultCodeSelect> for u8 {
    #[inline(always)]
    fn from(variant: QkResultCodeSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QkResultCodeSelect {
    type Ux = u8;
}
impl crate::IsEnum for QkResultCodeSelect {}
#[doc = "Field `qk_result_code` reader - Result code of last operation"]
pub type QkResultCodeR = crate::FieldReader<QkResultCodeSelect>;
impl QkResultCodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<QkResultCodeSelect> {
        match self.bits {
            0 => Some(QkResultCodeSelect::IidQkOk),
            240 => Some(QkResultCodeSelect::IidQkErrProduct),
            241 => Some(QkResultCodeSelect::IidQkErrProductPh2),
            242 => Some(QkResultCodeSelect::IidQkErrTransfer),
            243 => Some(QkResultCodeSelect::IidQkErrTransferPh2),
            244 => Some(QkResultCodeSelect::IidQkErrAuth),
            245 => Some(QkResultCodeSelect::IidQkErrAuthPh2),
            246 => Some(QkResultCodeSelect::IidQkErrPufQuality),
            247 => Some(QkResultCodeSelect::IidQkErrContext),
            248 => Some(QkResultCodeSelect::IidQkErrDestination),
            255 => Some(QkResultCodeSelect::IidQkFailureSram),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iid_qk_ok(&self) -> bool {
        *self == QkResultCodeSelect::IidQkOk
    }
    #[doc = "`11110000`"]
    #[inline(always)]
    pub fn is_iid_qk_err_product(&self) -> bool {
        *self == QkResultCodeSelect::IidQkErrProduct
    }
    #[doc = "`11110001`"]
    #[inline(always)]
    pub fn is_iid_qk_err_product_ph2(&self) -> bool {
        *self == QkResultCodeSelect::IidQkErrProductPh2
    }
    #[doc = "`11110010`"]
    #[inline(always)]
    pub fn is_iid_qk_err_transfer(&self) -> bool {
        *self == QkResultCodeSelect::IidQkErrTransfer
    }
    #[doc = "`11110011`"]
    #[inline(always)]
    pub fn is_iid_qk_err_transfer_ph2(&self) -> bool {
        *self == QkResultCodeSelect::IidQkErrTransferPh2
    }
    #[doc = "`11110100`"]
    #[inline(always)]
    pub fn is_iid_qk_err_auth(&self) -> bool {
        *self == QkResultCodeSelect::IidQkErrAuth
    }
    #[doc = "`11110101`"]
    #[inline(always)]
    pub fn is_iid_qk_err_auth_ph2(&self) -> bool {
        *self == QkResultCodeSelect::IidQkErrAuthPh2
    }
    #[doc = "`11110110`"]
    #[inline(always)]
    pub fn is_iid_qk_err_puf_quality(&self) -> bool {
        *self == QkResultCodeSelect::IidQkErrPufQuality
    }
    #[doc = "`11110111`"]
    #[inline(always)]
    pub fn is_iid_qk_err_context(&self) -> bool {
        *self == QkResultCodeSelect::IidQkErrContext
    }
    #[doc = "`11111000`"]
    #[inline(always)]
    pub fn is_iid_qk_err_destination(&self) -> bool {
        *self == QkResultCodeSelect::IidQkErrDestination
    }
    #[doc = "`11111111`"]
    #[inline(always)]
    pub fn is_iid_qk_failure_sram(&self) -> bool {
        *self == QkResultCodeSelect::IidQkFailureSram
    }
}
#[doc = "Number of last operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QkLastOperationSelect {
    #[doc = "0: `0`"]
    IidQkLoProgress = 0,
    #[doc = "1: `1`"]
    IidQkLoEnroll = 1,
    #[doc = "2: `10`"]
    IidQkLoStart = 2,
    #[doc = "3: `11`"]
    IidQkLoReconstruct = 3,
    #[doc = "5: `101`"]
    IidQkLoStop = 5,
    #[doc = "6: `110`"]
    IidQkLoGetKey = 6,
    #[doc = "7: `111`"]
    IidQkLoUnwrap = 7,
    #[doc = "8: `1000`"]
    IidQkLoWrapGenRnd = 8,
    #[doc = "9: `1001`"]
    IidQkLoWrap = 9,
    #[doc = "15: `1111`"]
    IidQkLoGenRnd = 15,
    #[doc = "30: `11110`"]
    IidQkLoTestMemory = 30,
    #[doc = "31: `11111`"]
    IidQkLoTestPuf = 31,
    #[doc = "32: `100000`"]
    IidQkLoInitialization = 32,
    #[doc = "47: `101111`"]
    IidQkLoZeroize = 47,
}
impl From<QkLastOperationSelect> for u8 {
    #[inline(always)]
    fn from(variant: QkLastOperationSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QkLastOperationSelect {
    type Ux = u8;
}
impl crate::IsEnum for QkLastOperationSelect {}
#[doc = "Field `qk_last_operation` reader - Number of last operation"]
pub type QkLastOperationR = crate::FieldReader<QkLastOperationSelect>;
impl QkLastOperationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<QkLastOperationSelect> {
        match self.bits {
            0 => Some(QkLastOperationSelect::IidQkLoProgress),
            1 => Some(QkLastOperationSelect::IidQkLoEnroll),
            2 => Some(QkLastOperationSelect::IidQkLoStart),
            3 => Some(QkLastOperationSelect::IidQkLoReconstruct),
            5 => Some(QkLastOperationSelect::IidQkLoStop),
            6 => Some(QkLastOperationSelect::IidQkLoGetKey),
            7 => Some(QkLastOperationSelect::IidQkLoUnwrap),
            8 => Some(QkLastOperationSelect::IidQkLoWrapGenRnd),
            9 => Some(QkLastOperationSelect::IidQkLoWrap),
            15 => Some(QkLastOperationSelect::IidQkLoGenRnd),
            30 => Some(QkLastOperationSelect::IidQkLoTestMemory),
            31 => Some(QkLastOperationSelect::IidQkLoTestPuf),
            32 => Some(QkLastOperationSelect::IidQkLoInitialization),
            47 => Some(QkLastOperationSelect::IidQkLoZeroize),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_progress(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoProgress
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_enroll(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoEnroll
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_start(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoStart
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_reconstruct(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoReconstruct
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_stop(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoStop
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_get_key(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoGetKey
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_unwrap(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoUnwrap
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_wrap_gen_rnd(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoWrapGenRnd
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_wrap(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoWrap
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_gen_rnd(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoGenRnd
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_test_memory(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoTestMemory
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_test_puf(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoTestPuf
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_initialization(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoInitialization
    }
    #[doc = "`101111`"]
    #[inline(always)]
    pub fn is_iid_qk_lo_zeroize(&self) -> bool {
        *self == QkLastOperationSelect::IidQkLoZeroize
    }
}
impl R {
    #[doc = "Bits 0:7 - Result code of last operation"]
    #[inline(always)]
    pub fn qk_result_code(&self) -> QkResultCodeR {
        QkResultCodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Number of last operation"]
    #[inline(always)]
    pub fn qk_last_operation(&self) -> QkLastOperationR {
        QkLastOperationR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Operation Result register; The Operation Result register provides the result code of the last operation (Initialization, Generate Random, etc.). The value in this register is only valid when qk_busy = 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_orr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkOrrSpec;
impl crate::RegisterSpec for QkOrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_orr::R`](R) reader structure"]
impl crate::Readable for QkOrrSpec {}
#[doc = "`reset()` method sets QK_ORR to value 0"]
impl crate::Resettable for QkOrrSpec {}

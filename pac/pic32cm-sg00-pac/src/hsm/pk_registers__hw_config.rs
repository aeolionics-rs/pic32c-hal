#[doc = "Register `PK_Registers__HwConfig` reader"]
pub type R = crate::R<PkRegisters_HwConfigSpec>;
#[doc = "Field `MaxOpSize` reader - Maximum operand size (number of bytes)."]
pub type MaxOpSizeR = crate::FieldReader<u16>;
#[doc = "Field `NbMult` reader - Number of multipliers: 0x0: 1 multiplier 0x1: 4 multipliers 0x2: 16 multipliers 0x4: 64 multipliers 0x8: 256 multipliers"]
pub type NbMultR = crate::FieldReader;
#[doc = "Field `PrimeField` reader - Support prime field."]
pub type PrimeFieldR = crate::BitReader;
#[doc = "Field `BinaryField` reader - Support binary field."]
pub type BinaryFieldR = crate::BitReader;
#[doc = "Field `ECC` reader - Support RAM error correction."]
pub type EccR = crate::BitReader;
#[doc = "Field `P256` reader - Support ECC P256 acceleration."]
pub type P256R = crate::BitReader;
#[doc = "Field `P384` reader - Support ECC P384 acceleration."]
pub type P384R = crate::BitReader;
#[doc = "Field `P521` reader - Support ECC P521 acceleration."]
pub type P521R = crate::BitReader;
#[doc = "Field `P192` reader - Support ECC P192 acceleration."]
pub type P192R = crate::BitReader;
#[doc = "Field `X25519` reader - Support Curve25519/Ed25519 acceleration."]
pub type X25519R = crate::BitReader;
#[doc = "Field `AHBMaster` reader - '0': Memory access through AHB Slave and internally in the PKE. '1': Memory access through AHB Master, outside the PKE."]
pub type AhbmasterR = crate::BitReader;
#[doc = "Field `DisableSM` reader - State of DisableSMx input (high when SM2/SM9 operations are disabled)."]
pub type DisableSmR = crate::BitReader;
#[doc = "Field `DisableClrMem` reader - State of DisableClrMem input (high when automatic clear of the RAM after reset is disabled)."]
pub type DisableClrMemR = crate::BitReader;
#[doc = "Field `DisableCM` reader - State of DisableCM input (high when counter-measures are disabled)."]
pub type DisableCmR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - Maximum operand size (number of bytes)."]
    #[inline(always)]
    pub fn max_op_size(&self) -> MaxOpSizeR {
        MaxOpSizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Number of multipliers: 0x0: 1 multiplier 0x1: 4 multipliers 0x2: 16 multipliers 0x4: 64 multipliers 0x8: 256 multipliers"]
    #[inline(always)]
    pub fn nb_mult(&self) -> NbMultR {
        NbMultR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Support prime field."]
    #[inline(always)]
    pub fn prime_field(&self) -> PrimeFieldR {
        PrimeFieldR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Support binary field."]
    #[inline(always)]
    pub fn binary_field(&self) -> BinaryFieldR {
        BinaryFieldR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Support RAM error correction."]
    #[inline(always)]
    pub fn ecc(&self) -> EccR {
        EccR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Support ECC P256 acceleration."]
    #[inline(always)]
    pub fn p256(&self) -> P256R {
        P256R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Support ECC P384 acceleration."]
    #[inline(always)]
    pub fn p384(&self) -> P384R {
        P384R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Support ECC P521 acceleration."]
    #[inline(always)]
    pub fn p521(&self) -> P521R {
        P521R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Support ECC P192 acceleration."]
    #[inline(always)]
    pub fn p192(&self) -> P192R {
        P192R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Support Curve25519/Ed25519 acceleration."]
    #[inline(always)]
    pub fn x25519(&self) -> X25519R {
        X25519R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - '0': Memory access through AHB Slave and internally in the PKE. '1': Memory access through AHB Master, outside the PKE."]
    #[inline(always)]
    pub fn ahbmaster(&self) -> AhbmasterR {
        AhbmasterR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - State of DisableSMx input (high when SM2/SM9 operations are disabled)."]
    #[inline(always)]
    pub fn disable_sm(&self) -> DisableSmR {
        DisableSmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - State of DisableClrMem input (high when automatic clear of the RAM after reset is disabled)."]
    #[inline(always)]
    pub fn disable_clr_mem(&self) -> DisableClrMemR {
        DisableClrMemR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - State of DisableCM input (high when counter-measures are disabled)."]
    #[inline(always)]
    pub fn disable_cm(&self) -> DisableCmR {
        DisableCmR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Hardware configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__hw_config::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkRegisters_HwConfigSpec;
impl crate::RegisterSpec for PkRegisters_HwConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pk_registers__hw_config::R`](R) reader structure"]
impl crate::Readable for PkRegisters_HwConfigSpec {}
#[doc = "`reset()` method sets PK_Registers__HwConfig to value 0x43f3_0200"]
impl crate::Resettable for PkRegisters_HwConfigSpec {
    const RESET_VALUE: u32 = 0x43f3_0200;
}

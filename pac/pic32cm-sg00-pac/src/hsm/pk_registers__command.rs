#[doc = "Register `PK_Registers__Command` reader"]
pub type R = crate::R<PkRegisters_CommandSpec>;
#[doc = "Register `PK_Registers__Command` writer"]
pub type W = crate::W<PkRegisters_CommandSpec>;
#[doc = "Field `OpeAddr` reader - This field defines the operation to be performed. See documentation for more details."]
pub type OpeAddrR = crate::FieldReader;
#[doc = "Field `OpeAddr` writer - This field defines the operation to be performed. See documentation for more details."]
pub type OpeAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FieldF` reader - '0': Field is GF(p) '1': Field is GF(2**m)"]
pub type FieldFR = crate::BitReader;
#[doc = "Field `FieldF` writer - '0': Field is GF(p) '1': Field is GF(2**m)"]
pub type FieldFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OpBytesM1` reader - This field defines the size (= number of bytes minus one) of the operands for the current operation. Possible values are limited by the maximum supported operand size. Examples: - 0x014 -> ECC on curve K-163 - 0x01F -> ECC on curve P-256 - 0x02F -> ECC on curve P-384 - 0x033 -> ECC on curve K-409 - 0x041 -> ECC on curve P-521 - 0x07F -> 1024-bit RSA - 0x09F -> 1280-bit RSA - 0x1FF -> 4096-bit RSA - 0x3FF -> 8192-bit RSA"]
pub type OpBytesM1R = crate::FieldReader<u16>;
#[doc = "Field `OpBytesM1` writer - This field defines the size (= number of bytes minus one) of the operands for the current operation. Possible values are limited by the maximum supported operand size. Examples: - 0x014 -> ECC on curve K-163 - 0x01F -> ECC on curve P-256 - 0x02F -> ECC on curve P-384 - 0x033 -> ECC on curve K-409 - 0x041 -> ECC on curve P-521 - 0x07F -> 1024-bit RSA - 0x09F -> 1280-bit RSA - 0x1FF -> 4096-bit RSA - 0x3FF -> 8192-bit RSA"]
pub type OpBytesM1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RandMod` reader - Enable randomization of modulus (counter-measure)."]
pub type RandModR = crate::BitReader;
#[doc = "Field `RandMod` writer - Enable randomization of modulus (counter-measure)."]
pub type RandModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SelCurve` reader - Enable accelerator for specific curve modulus: 0x0: No acceleration (default) 0x1: P256 0x2: P384 0x3: P521 0x4: P192 0x5: Curve25519 0x6: Ed25519 This field has no effect when the optional acceleration hardware is not included."]
pub type SelCurveR = crate::FieldReader;
#[doc = "Field `SelCurve` writer - Enable accelerator for specific curve modulus: 0x0: No acceleration (default) 0x1: P256 0x2: P384 0x3: P521 0x4: P192 0x5: Curve25519 0x6: Ed25519 This field has no effect when the optional acceleration hardware is not included."]
pub type SelCurveW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RandKE` reader - Enable randomization of exponent/scalar (counter-measure)."]
pub type RandKeR = crate::BitReader;
#[doc = "Field `RandKE` writer - Enable randomization of exponent/scalar (counter-measure)."]
pub type RandKeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RandProj` reader - Enable randomization of projective coordinates (counter-measure)."]
pub type RandProjR = crate::BitReader;
#[doc = "Field `RandProj` writer - Enable randomization of projective coordinates (counter-measure)."]
pub type RandProjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Edwards` reader - Enable Edwards curve (see operation description)."]
pub type EdwardsR = crate::BitReader;
#[doc = "Field `Edwards` writer - Enable Edwards curve (see operation description)."]
pub type EdwardsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SwapBytes` reader - Swap the bytes on AHB interface: '0': Native format (little endian) '1': Byte swapped (big endian) See also section 4.2.1. This bit must be programmed before writing/reading any data in data memory."]
pub type SwapBytesR = crate::BitReader;
#[doc = "Field `SwapBytes` writer - Swap the bytes on AHB interface: '0': Native format (little endian) '1': Byte swapped (big endian) See also section 4.2.1. This bit must be programmed before writing/reading any data in data memory."]
pub type SwapBytesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FlagA` reader - See operation description."]
pub type FlagAR = crate::BitReader;
#[doc = "Field `FlagA` writer - See operation description."]
pub type FlagAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FlagB` reader - See operation description."]
pub type FlagBR = crate::BitReader;
#[doc = "Field `FlagB` writer - See operation description."]
pub type FlagBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CalcR2` reader - This bit indicates if the IP has to calculate R**2 mod N for the next operation. This bit must be set to '1' when a new prime number has been programmed. This bit is used for primitive operations and ignored for the other operations. '0': don't recalculate R**2 mod N '1': re-calculate R**2 mod N"]
pub type CalcR2R = crate::BitReader;
#[doc = "Field `CalcR2` writer - This bit indicates if the IP has to calculate R**2 mod N for the next operation. This bit must be set to '1' when a new prime number has been programmed. This bit is used for primitive operations and ignored for the other operations. '0': don't recalculate R**2 mod N '1': re-calculate R**2 mod N"]
pub type CalcR2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - This field defines the operation to be performed. See documentation for more details."]
    #[inline(always)]
    pub fn ope_addr(&self) -> OpeAddrR {
        OpeAddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - '0': Field is GF(p) '1': Field is GF(2**m)"]
    #[inline(always)]
    pub fn field_f(&self) -> FieldFR {
        FieldFR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:17 - This field defines the size (= number of bytes minus one) of the operands for the current operation. Possible values are limited by the maximum supported operand size. Examples: - 0x014 -> ECC on curve K-163 - 0x01F -> ECC on curve P-256 - 0x02F -> ECC on curve P-384 - 0x033 -> ECC on curve K-409 - 0x041 -> ECC on curve P-521 - 0x07F -> 1024-bit RSA - 0x09F -> 1280-bit RSA - 0x1FF -> 4096-bit RSA - 0x3FF -> 8192-bit RSA"]
    #[inline(always)]
    pub fn op_bytes_m1(&self) -> OpBytesM1R {
        OpBytesM1R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bit 19 - Enable randomization of modulus (counter-measure)."]
    #[inline(always)]
    pub fn rand_mod(&self) -> RandModR {
        RandModR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Enable accelerator for specific curve modulus: 0x0: No acceleration (default) 0x1: P256 0x2: P384 0x3: P521 0x4: P192 0x5: Curve25519 0x6: Ed25519 This field has no effect when the optional acceleration hardware is not included."]
    #[inline(always)]
    pub fn sel_curve(&self) -> SelCurveR {
        SelCurveR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Enable randomization of exponent/scalar (counter-measure)."]
    #[inline(always)]
    pub fn rand_ke(&self) -> RandKeR {
        RandKeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable randomization of projective coordinates (counter-measure)."]
    #[inline(always)]
    pub fn rand_proj(&self) -> RandProjR {
        RandProjR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Edwards curve (see operation description)."]
    #[inline(always)]
    pub fn edwards(&self) -> EdwardsR {
        EdwardsR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Swap the bytes on AHB interface: '0': Native format (little endian) '1': Byte swapped (big endian) See also section 4.2.1. This bit must be programmed before writing/reading any data in data memory."]
    #[inline(always)]
    pub fn swap_bytes(&self) -> SwapBytesR {
        SwapBytesR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - See operation description."]
    #[inline(always)]
    pub fn flag_a(&self) -> FlagAR {
        FlagAR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - See operation description."]
    #[inline(always)]
    pub fn flag_b(&self) -> FlagBR {
        FlagBR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit indicates if the IP has to calculate R**2 mod N for the next operation. This bit must be set to '1' when a new prime number has been programmed. This bit is used for primitive operations and ignored for the other operations. '0': don't recalculate R**2 mod N '1': re-calculate R**2 mod N"]
    #[inline(always)]
    pub fn calc_r2(&self) -> CalcR2R {
        CalcR2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - This field defines the operation to be performed. See documentation for more details."]
    #[inline(always)]
    pub fn ope_addr(&mut self) -> OpeAddrW<'_, PkRegisters_CommandSpec> {
        OpeAddrW::new(self, 0)
    }
    #[doc = "Bit 7 - '0': Field is GF(p) '1': Field is GF(2**m)"]
    #[inline(always)]
    pub fn field_f(&mut self) -> FieldFW<'_, PkRegisters_CommandSpec> {
        FieldFW::new(self, 7)
    }
    #[doc = "Bits 8:17 - This field defines the size (= number of bytes minus one) of the operands for the current operation. Possible values are limited by the maximum supported operand size. Examples: - 0x014 -> ECC on curve K-163 - 0x01F -> ECC on curve P-256 - 0x02F -> ECC on curve P-384 - 0x033 -> ECC on curve K-409 - 0x041 -> ECC on curve P-521 - 0x07F -> 1024-bit RSA - 0x09F -> 1280-bit RSA - 0x1FF -> 4096-bit RSA - 0x3FF -> 8192-bit RSA"]
    #[inline(always)]
    pub fn op_bytes_m1(&mut self) -> OpBytesM1W<'_, PkRegisters_CommandSpec> {
        OpBytesM1W::new(self, 8)
    }
    #[doc = "Bit 19 - Enable randomization of modulus (counter-measure)."]
    #[inline(always)]
    pub fn rand_mod(&mut self) -> RandModW<'_, PkRegisters_CommandSpec> {
        RandModW::new(self, 19)
    }
    #[doc = "Bits 20:22 - Enable accelerator for specific curve modulus: 0x0: No acceleration (default) 0x1: P256 0x2: P384 0x3: P521 0x4: P192 0x5: Curve25519 0x6: Ed25519 This field has no effect when the optional acceleration hardware is not included."]
    #[inline(always)]
    pub fn sel_curve(&mut self) -> SelCurveW<'_, PkRegisters_CommandSpec> {
        SelCurveW::new(self, 20)
    }
    #[doc = "Bit 24 - Enable randomization of exponent/scalar (counter-measure)."]
    #[inline(always)]
    pub fn rand_ke(&mut self) -> RandKeW<'_, PkRegisters_CommandSpec> {
        RandKeW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable randomization of projective coordinates (counter-measure)."]
    #[inline(always)]
    pub fn rand_proj(&mut self) -> RandProjW<'_, PkRegisters_CommandSpec> {
        RandProjW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Edwards curve (see operation description)."]
    #[inline(always)]
    pub fn edwards(&mut self) -> EdwardsW<'_, PkRegisters_CommandSpec> {
        EdwardsW::new(self, 26)
    }
    #[doc = "Bit 28 - Swap the bytes on AHB interface: '0': Native format (little endian) '1': Byte swapped (big endian) See also section 4.2.1. This bit must be programmed before writing/reading any data in data memory."]
    #[inline(always)]
    pub fn swap_bytes(&mut self) -> SwapBytesW<'_, PkRegisters_CommandSpec> {
        SwapBytesW::new(self, 28)
    }
    #[doc = "Bit 29 - See operation description."]
    #[inline(always)]
    pub fn flag_a(&mut self) -> FlagAW<'_, PkRegisters_CommandSpec> {
        FlagAW::new(self, 29)
    }
    #[doc = "Bit 30 - See operation description."]
    #[inline(always)]
    pub fn flag_b(&mut self) -> FlagBW<'_, PkRegisters_CommandSpec> {
        FlagBW::new(self, 30)
    }
    #[doc = "Bit 31 - This bit indicates if the IP has to calculate R**2 mod N for the next operation. This bit must be set to '1' when a new prime number has been programmed. This bit is used for primitive operations and ignored for the other operations. '0': don't recalculate R**2 mod N '1': re-calculate R**2 mod N"]
    #[inline(always)]
    pub fn calc_r2(&mut self) -> CalcR2W<'_, PkRegisters_CommandSpec> {
        CalcR2W::new(self, 31)
    }
}
#[doc = "Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__command::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__command::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkRegisters_CommandSpec;
impl crate::RegisterSpec for PkRegisters_CommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pk_registers__command::R`](R) reader structure"]
impl crate::Readable for PkRegisters_CommandSpec {}
#[doc = "`write(|w| ..)` method takes [`pk_registers__command::W`](W) writer structure"]
impl crate::Writable for PkRegisters_CommandSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PK_Registers__Command to value 0x0f"]
impl crate::Resettable for PkRegisters_CommandSpec {
    const RESET_VALUE: u32 = 0x0f;
}

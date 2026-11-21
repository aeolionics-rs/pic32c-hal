#[doc = "Register `PK_Registers__Pointers` reader"]
pub type R = crate::R<PkRegisters_PointersSpec>;
#[doc = "Register `PK_Registers__Pointers` writer"]
pub type W = crate::W<PkRegisters_PointersSpec>;
#[doc = "Field `OpPtrA` reader - When executing primitive arithmetic operations, this pointer defines where operand A is located in memory (location 0x0 to 0xF)."]
pub type OpPtrAR = crate::FieldReader;
#[doc = "Field `OpPtrA` writer - When executing primitive arithmetic operations, this pointer defines where operand A is located in memory (location 0x0 to 0xF)."]
pub type OpPtrAW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OpPtrB` reader - When executing primitive arithmetic operations, this pointer defines where operand B is located in memory (location 0x0 to 0xF)."]
pub type OpPtrBR = crate::FieldReader;
#[doc = "Field `OpPtrB` writer - When executing primitive arithmetic operations, this pointer defines where operand B is located in memory (location 0x0 to 0xF)."]
pub type OpPtrBW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OpPtrC` reader - When executing primitive arithmetic operations, this pointer defines the location (0x0 to 0xF) where the result will be stored in memory."]
pub type OpPtrCR = crate::FieldReader;
#[doc = "Field `OpPtrC` writer - When executing primitive arithmetic operations, this pointer defines the location (0x0 to 0xF) where the result will be stored in memory."]
pub type OpPtrCW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OpPtrN` reader - When executing primitive arithmetic operations, this pointer defines the location where the modulus is located in memory (location 0x0 to 0xF)."]
pub type OpPtrNR = crate::FieldReader;
#[doc = "Field `OpPtrN` writer - When executing primitive arithmetic operations, this pointer defines the location where the modulus is located in memory (location 0x0 to 0xF)."]
pub type OpPtrNW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - When executing primitive arithmetic operations, this pointer defines where operand A is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    pub fn op_ptr_a(&self) -> OpPtrAR {
        OpPtrAR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - When executing primitive arithmetic operations, this pointer defines where operand B is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    pub fn op_ptr_b(&self) -> OpPtrBR {
        OpPtrBR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - When executing primitive arithmetic operations, this pointer defines the location (0x0 to 0xF) where the result will be stored in memory."]
    #[inline(always)]
    pub fn op_ptr_c(&self) -> OpPtrCR {
        OpPtrCR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - When executing primitive arithmetic operations, this pointer defines the location where the modulus is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    pub fn op_ptr_n(&self) -> OpPtrNR {
        OpPtrNR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - When executing primitive arithmetic operations, this pointer defines where operand A is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    pub fn op_ptr_a(&mut self) -> OpPtrAW<'_, PkRegisters_PointersSpec> {
        OpPtrAW::new(self, 0)
    }
    #[doc = "Bits 8:11 - When executing primitive arithmetic operations, this pointer defines where operand B is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    pub fn op_ptr_b(&mut self) -> OpPtrBW<'_, PkRegisters_PointersSpec> {
        OpPtrBW::new(self, 8)
    }
    #[doc = "Bits 16:19 - When executing primitive arithmetic operations, this pointer defines the location (0x0 to 0xF) where the result will be stored in memory."]
    #[inline(always)]
    pub fn op_ptr_c(&mut self) -> OpPtrCW<'_, PkRegisters_PointersSpec> {
        OpPtrCW::new(self, 16)
    }
    #[doc = "Bits 24:27 - When executing primitive arithmetic operations, this pointer defines the location where the modulus is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    pub fn op_ptr_n(&mut self) -> OpPtrNW<'_, PkRegisters_PointersSpec> {
        OpPtrNW::new(self, 24)
    }
}
#[doc = "Pointers register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__pointers::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__pointers::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkRegisters_PointersSpec;
impl crate::RegisterSpec for PkRegisters_PointersSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pk_registers__pointers::R`](R) reader structure"]
impl crate::Readable for PkRegisters_PointersSpec {}
#[doc = "`write(|w| ..)` method takes [`pk_registers__pointers::W`](W) writer structure"]
impl crate::Writable for PkRegisters_PointersSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PK_Registers__Pointers to value 0"]
impl crate::Resettable for PkRegisters_PointersSpec {}

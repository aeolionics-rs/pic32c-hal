#[doc = "Register `PK_Registers__OpSize` reader"]
pub type R = crate::R<PkRegisters_OpSizeSpec>;
#[doc = "Register `PK_Registers__OpSize` writer"]
pub type W = crate::W<PkRegisters_OpSizeSpec>;
#[doc = "Field `OpSize` reader - Operand size (number of bytes): 0x0100: 256 0x0209: 521 0x0800: 2048 0x0C00: 3072 0x1000: 4096 This register is used when the memory is accessed via AHB Master"]
pub type OpSizeR = crate::FieldReader<u16>;
#[doc = "Field `OpSize` writer - Operand size (number of bytes): 0x0100: 256 0x0209: 521 0x0800: 2048 0x0C00: 3072 0x1000: 4096 This register is used when the memory is accessed via AHB Master"]
pub type OpSizeW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Operand size (number of bytes): 0x0100: 256 0x0209: 521 0x0800: 2048 0x0C00: 3072 0x1000: 4096 This register is used when the memory is accessed via AHB Master"]
    #[inline(always)]
    pub fn op_size(&self) -> OpSizeR {
        OpSizeR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Operand size (number of bytes): 0x0100: 256 0x0209: 521 0x0800: 2048 0x0C00: 3072 0x1000: 4096 This register is used when the memory is accessed via AHB Master"]
    #[inline(always)]
    pub fn op_size(&mut self) -> OpSizeW<'_, PkRegisters_OpSizeSpec> {
        OpSizeW::new(self, 0)
    }
}
#[doc = "Operand size register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__op_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__op_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkRegisters_OpSizeSpec;
impl crate::RegisterSpec for PkRegisters_OpSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pk_registers__op_size::R`](R) reader structure"]
impl crate::Readable for PkRegisters_OpSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`pk_registers__op_size::W`](W) writer structure"]
impl crate::Writable for PkRegisters_OpSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PK_Registers__OpSize to value 0x1000"]
impl crate::Resettable for PkRegisters_OpSizeSpec {
    const RESET_VALUE: u32 = 0x1000;
}

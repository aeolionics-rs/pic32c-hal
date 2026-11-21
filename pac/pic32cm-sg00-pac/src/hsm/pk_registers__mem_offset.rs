#[doc = "Register `PK_Registers__MemOffset` reader"]
pub type R = crate::R<PkRegisters_MemOffsetSpec>;
#[doc = "Register `PK_Registers__MemOffset` writer"]
pub type W = crate::W<PkRegisters_MemOffsetSpec>;
#[doc = "Field `MemOffset` reader - Memory offset for AHB Master"]
pub type MemOffsetR = crate::FieldReader<u32>;
#[doc = "Field `MemOffset` writer - Memory offset for AHB Master"]
pub type MemOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory offset for AHB Master"]
    #[inline(always)]
    pub fn mem_offset(&self) -> MemOffsetR {
        MemOffsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory offset for AHB Master"]
    #[inline(always)]
    pub fn mem_offset(&mut self) -> MemOffsetW<'_, PkRegisters_MemOffsetSpec> {
        MemOffsetW::new(self, 0)
    }
}
#[doc = "Memory offset register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__mem_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__mem_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkRegisters_MemOffsetSpec;
impl crate::RegisterSpec for PkRegisters_MemOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pk_registers__mem_offset::R`](R) reader structure"]
impl crate::Readable for PkRegisters_MemOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`pk_registers__mem_offset::W`](W) writer structure"]
impl crate::Writable for PkRegisters_MemOffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PK_Registers__MemOffset to value 0"]
impl crate::Resettable for PkRegisters_MemOffsetSpec {}

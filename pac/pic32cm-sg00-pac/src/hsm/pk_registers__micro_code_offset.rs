#[doc = "Register `PK_Registers__MicroCodeOffset` reader"]
pub type R = crate::R<PkRegisters_MicroCodeOffsetSpec>;
#[doc = "Register `PK_Registers__MicroCodeOffset` writer"]
pub type W = crate::W<PkRegisters_MicroCodeOffsetSpec>;
#[doc = "Field `MicroCodeOffset` reader - MicroCode offset for AHB Master"]
pub type MicroCodeOffsetR = crate::FieldReader<u32>;
#[doc = "Field `MicroCodeOffset` writer - MicroCode offset for AHB Master"]
pub type MicroCodeOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MicroCode offset for AHB Master"]
    #[inline(always)]
    pub fn micro_code_offset(&self) -> MicroCodeOffsetR {
        MicroCodeOffsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MicroCode offset for AHB Master"]
    #[inline(always)]
    pub fn micro_code_offset(&mut self) -> MicroCodeOffsetW<'_, PkRegisters_MicroCodeOffsetSpec> {
        MicroCodeOffsetW::new(self, 0)
    }
}
#[doc = "MicroCode offset register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__micro_code_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__micro_code_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkRegisters_MicroCodeOffsetSpec;
impl crate::RegisterSpec for PkRegisters_MicroCodeOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pk_registers__micro_code_offset::R`](R) reader structure"]
impl crate::Readable for PkRegisters_MicroCodeOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`pk_registers__micro_code_offset::W`](W) writer structure"]
impl crate::Writable for PkRegisters_MicroCodeOffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PK_Registers__MicroCodeOffset to value 0"]
impl crate::Resettable for PkRegisters_MicroCodeOffsetSpec {}

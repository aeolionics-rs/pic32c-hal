#[doc = "Register `PK_Registers__Status` reader"]
pub type R = crate::R<PkRegisters_StatusSpec>;
#[doc = "Field `ErrorFlags` reader - These bits indicate an error condition. They are updated at the end of the operation. They are cleared when starting a new operation. See documentation for more details."]
pub type ErrorFlagsR = crate::FieldReader<u16>;
#[doc = "Field `PK_Busy` reader - This bit reflects the BUSY output value. It is set when the operation starts and it is cleared when the operation is finished."]
pub type PkBusyR = crate::BitReader;
#[doc = "Field `IntrptStatus` reader - This bit reflects the IRQ output value. It is set when the operation is finished. It is cleared when the CPU writes the bit 1 of Control Register."]
pub type IntrptStatusR = crate::BitReader;
#[doc = "Field `FailPtr` reader - These bits indicate which data location generated the error flag. They are not available for all error flags."]
pub type FailPtrR = crate::FieldReader;
impl R {
    #[doc = "Bits 4:15 - These bits indicate an error condition. They are updated at the end of the operation. They are cleared when starting a new operation. See documentation for more details."]
    #[inline(always)]
    pub fn error_flags(&self) -> ErrorFlagsR {
        ErrorFlagsR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - This bit reflects the BUSY output value. It is set when the operation starts and it is cleared when the operation is finished."]
    #[inline(always)]
    pub fn pk_busy(&self) -> PkBusyR {
        PkBusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit reflects the IRQ output value. It is set when the operation is finished. It is cleared when the CPU writes the bit 1 of Control Register."]
    #[inline(always)]
    pub fn intrpt_status(&self) -> IntrptStatusR {
        IntrptStatusR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:28 - These bits indicate which data location generated the error flag. They are not available for all error flags."]
    #[inline(always)]
    pub fn fail_ptr(&self) -> FailPtrR {
        FailPtrR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[doc = "Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkRegisters_StatusSpec;
impl crate::RegisterSpec for PkRegisters_StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pk_registers__status::R`](R) reader structure"]
impl crate::Readable for PkRegisters_StatusSpec {}
#[doc = "`reset()` method sets PK_Registers__Status to value 0"]
impl crate::Resettable for PkRegisters_StatusSpec {}

#[doc = "Register `QK_HW_ID` reader"]
pub type R = crate::R<QkHwIdSpec>;
#[doc = "Field `qk_hw_id` reader - Module identifier"]
pub type QkHwIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Module identifier"]
    #[inline(always)]
    pub fn qk_hw_id(&self) -> QkHwIdR {
        QkHwIdR::new(self.bits)
    }
}
#[doc = "Module Identifier register; The Module Identifier register provides the identifier of the QuiddiKey Engine netlist.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkHwIdSpec;
impl crate::RegisterSpec for QkHwIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_hw_id::R`](R) reader structure"]
impl crate::Readable for QkHwIdSpec {}
#[doc = "`reset()` method sets QK_HW_ID to value 0x2012_0702"]
impl crate::Resettable for QkHwIdSpec {
    const RESET_VALUE: u32 = 0x2012_0702;
}

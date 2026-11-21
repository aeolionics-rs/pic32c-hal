#[doc = "Register `QK_HW_RUC1` reader"]
pub type R = crate::R<QkHwRuc1Spec>;
#[doc = "Field `qk_restrict_user_context_1` reader - For each bit in the User_Context_1 field: 0: This bit can be used 1: This bit cannot be used"]
pub type QkRestrictUserContext1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - For each bit in the User_Context_1 field: 0: This bit can be used 1: This bit cannot be used"]
    #[inline(always)]
    pub fn qk_restrict_user_context_1(&self) -> QkRestrictUserContext1R {
        QkRestrictUserContext1R::new(self.bits)
    }
}
#[doc = "Module Restrict User Context 1 register; The Module Restrict User Context 1 register shows the values of the qk_restrict_user_context_1 signal on the signal interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_ruc1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkHwRuc1Spec;
impl crate::RegisterSpec for QkHwRuc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_hw_ruc1::R`](R) reader structure"]
impl crate::Readable for QkHwRuc1Spec {}
#[doc = "`reset()` method sets QK_HW_RUC1 to value 0"]
impl crate::Resettable for QkHwRuc1Spec {}

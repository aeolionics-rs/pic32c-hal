#[doc = "Register `QK_HW_RUC0` reader"]
pub type R = crate::R<QkHwRuc0Spec>;
#[doc = "Field `qk_restrict_user_context_0` reader - For each bit in the User_Context_0 field: 0: This bit can be used 1: This bit cannot be used"]
pub type QkRestrictUserContext0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - For each bit in the User_Context_0 field: 0: This bit can be used 1: This bit cannot be used"]
    #[inline(always)]
    pub fn qk_restrict_user_context_0(&self) -> QkRestrictUserContext0R {
        QkRestrictUserContext0R::new(self.bits)
    }
}
#[doc = "Module Restrict User Context 0 register; The Module Restrict User Context 0 register shows the values of the qk_restrict_user_context_0 signal on the signal interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_ruc0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkHwRuc0Spec;
impl crate::RegisterSpec for QkHwRuc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_hw_ruc0::R`](R) reader structure"]
impl crate::Readable for QkHwRuc0Spec {}
#[doc = "`reset()` method sets QK_HW_RUC0 to value 0"]
impl crate::Resettable for QkHwRuc0Spec {}

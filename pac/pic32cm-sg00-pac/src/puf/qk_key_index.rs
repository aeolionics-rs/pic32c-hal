#[doc = "Register `QK_KEY_INDEX` reader"]
pub type R = crate::R<QkKeyIndexSpec>;
#[doc = "Field `qk_key_inde` reader - Value of the Key Index"]
pub type QkKeyIndeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Value of the Key Index"]
    #[inline(always)]
    pub fn qk_key_inde(&self) -> QkKeyIndeR {
        QkKeyIndeR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Key Index register; The Key Index register provides the index of the key that is currently output.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_key_index::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkKeyIndexSpec;
impl crate::RegisterSpec for QkKeyIndexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_key_index::R`](R) reader structure"]
impl crate::Readable for QkKeyIndexSpec {}
#[doc = "`reset()` method sets QK_KEY_INDEX to value 0"]
impl crate::Resettable for QkKeyIndexSpec {}

#[doc = "Register `QK_DIR` writer"]
pub type W = crate::W<QkDirSpec>;
#[doc = "Field `qk_di` writer - Input data to QuiddiKey"]
pub type QkDiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input data to QuiddiKey"]
    #[inline(always)]
    pub fn qk_di(&mut self) -> QkDiW<'_, QkDirSpec> {
        QkDiW::new(self, 0)
    }
}
#[doc = "Data Input register; The Data Input register is used to transfer data to QuiddiKey. It can only be written when input data is requested (indicated by the qk_di_request signal). When written at any other moment, or when read, an error response is generated. Depending on the way the system is configured and connected, this may generate an exception.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_dir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkDirSpec;
impl crate::RegisterSpec for QkDirSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qk_dir::W`](W) writer structure"]
impl crate::Writable for QkDirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QK_DIR to value 0"]
impl crate::Resettable for QkDirSpec {}

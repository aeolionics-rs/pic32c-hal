#[doc = "Register `QK_IF_SR` reader"]
pub type R = crate::R<QkIfSrSpec>;
#[doc = "Register `QK_IF_SR` writer"]
pub type W = crate::W<QkIfSrSpec>;
#[doc = "Field `reg_if_error` reader - An error has occurred at the register interface"]
pub type RegIfErrorR = crate::BitReader;
#[doc = "Field `reg_if_error` writer - An error has occurred at the register interface"]
pub type RegIfErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - An error has occurred at the register interface"]
    #[inline(always)]
    pub fn reg_if_error(&self) -> RegIfErrorR {
        RegIfErrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An error has occurred at the register interface"]
    #[inline(always)]
    pub fn reg_if_error(&mut self) -> RegIfErrorW<'_, QkIfSrSpec> {
        RegIfErrorW::new(self, 0)
    }
}
#[doc = "Interface Status register; The status of the interface can be monitored with the Interface Status register. Writing 1 to a bit, clears the corresponding bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_if_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_if_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkIfSrSpec;
impl crate::RegisterSpec for QkIfSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_if_sr::R`](R) reader structure"]
impl crate::Readable for QkIfSrSpec {}
#[doc = "`write(|w| ..)` method takes [`qk_if_sr::W`](W) writer structure"]
impl crate::Writable for QkIfSrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QK_IF_SR to value 0"]
impl crate::Resettable for QkIfSrSpec {}

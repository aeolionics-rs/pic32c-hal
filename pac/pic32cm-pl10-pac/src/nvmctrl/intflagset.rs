#[doc = "Register `INTFLAGSET` reader"]
pub type R = crate::R<IntflagsetSpec>;
#[doc = "Register `INTFLAGSET` writer"]
pub type W = crate::W<IntflagsetSpec>;
#[doc = "Field `READY` reader - NVM Ready"]
pub type ReadyR = crate::BitReader;
#[doc = "Field `READY` writer - NVM Ready"]
pub type ReadyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERROR` reader - Error"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Error"]
pub type ErrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - NVM Ready"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVM Ready"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntflagsetSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Error"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntflagsetSpec> {
        ErrorW::new(self, 1)
    }
}
#[doc = "Interrupt Flag Software Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagsetSpec;
impl crate::RegisterSpec for IntflagsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagset::R`](R) reader structure"]
impl crate::Readable for IntflagsetSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagset::W`](W) writer structure"]
impl crate::Writable for IntflagsetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets INTFLAGSET to value 0"]
impl crate::Resettable for IntflagsetSpec {}

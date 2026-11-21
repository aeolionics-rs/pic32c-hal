#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `MB` reader - Host on Bus Interrupt Enable"]
pub type MbR = crate::BitReader;
#[doc = "Field `MB` writer - Host on Bus Interrupt Enable"]
pub type MbW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SB` reader - Client on Bus Interrupt Enable"]
pub type SbR = crate::BitReader;
#[doc = "Field `SB` writer - Client on Bus Interrupt Enable"]
pub type SbW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ERROR` reader - Error Interrupt Enable"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Error Interrupt Enable"]
pub type ErrorW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host on Bus Interrupt Enable"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Client on Bus Interrupt Enable"]
    #[inline(always)]
    pub fn sb(&self) -> SbR {
        SbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host on Bus Interrupt Enable"]
    #[inline(always)]
    pub fn mb(&mut self) -> MbW<'_, IntensetSpec> {
        MbW::new(self, 0)
    }
    #[doc = "Bit 1 - Client on Bus Interrupt Enable"]
    #[inline(always)]
    pub fn sb(&mut self) -> SbW<'_, IntensetSpec> {
        SbW::new(self, 1)
    }
    #[doc = "Bit 7 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntensetSpec> {
        ErrorW::new(self, 7)
    }
}
#[doc = "I2CM Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x83;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}

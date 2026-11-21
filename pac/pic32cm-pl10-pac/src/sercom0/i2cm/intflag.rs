#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `MB` reader - Host on Bus"]
pub type MbR = crate::BitReader;
#[doc = "Field `MB` writer - Host on Bus"]
pub type MbW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SB` reader - Client on Bus"]
pub type SbR = crate::BitReader;
#[doc = "Field `SB` writer - Client on Bus"]
pub type SbW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERROR` reader - Error"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Error"]
pub type ErrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host on Bus"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Client on Bus"]
    #[inline(always)]
    pub fn sb(&self) -> SbR {
        SbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Error"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host on Bus"]
    #[inline(always)]
    pub fn mb(&mut self) -> MbW<'_, IntflagSpec> {
        MbW::new(self, 0)
    }
    #[doc = "Bit 1 - Client on Bus"]
    #[inline(always)]
    pub fn sb(&mut self) -> SbW<'_, IntflagSpec> {
        SbW::new(self, 1)
    }
    #[doc = "Bit 7 - Error"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntflagSpec> {
        ErrorW::new(self, 7)
    }
}
#[doc = "I2CM Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x83;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}

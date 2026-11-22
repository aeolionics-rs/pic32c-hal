#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `MB` reader - Master on Bus"]
pub type MbR = crate::BitReader;
#[doc = "Field `MB` writer - Master on Bus"]
pub type MbW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SB` reader - Slave on Bus"]
pub type SbR = crate::BitReader;
#[doc = "Field `SB` writer - Slave on Bus"]
pub type SbW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Master on Bus"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave on Bus"]
    #[inline(always)]
    pub fn sb(&self) -> SbR {
        SbR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master on Bus"]
    #[inline(always)]
    pub fn mb(&mut self) -> MbW<'_, IntflagSpec> {
        MbW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave on Bus"]
    #[inline(always)]
    pub fn sb(&mut self) -> SbW<'_, IntflagSpec> {
        SbW::new(self, 1)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x03;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}

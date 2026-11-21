#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `DBG` reader - Debug Message Reception"]
pub type DbgR = crate::BitReader;
#[doc = "Field `DBG` writer - Debug Message Reception"]
pub type DbgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BERR` reader - AHB Bus Error Detection"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - AHB Bus Error Detection"]
pub type BerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Message Reception"]
    #[inline(always)]
    pub fn dbg(&self) -> DbgR {
        DbgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB Bus Error Detection"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Message Reception"]
    #[inline(always)]
    pub fn dbg(&mut self) -> DbgW<'_, IntflagSpec> {
        DbgW::new(self, 0)
    }
    #[doc = "Bit 1 - AHB Bus Error Detection"]
    #[inline(always)]
    pub fn berr(&mut self) -> BerrW<'_, IntflagSpec> {
        BerrW::new(self, 1)
    }
}
#[doc = "Interrupt Flag Status and Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}

#[doc = "Register `INTFLAGSET` reader"]
pub type R = crate::R<IntflagsetSpec>;
#[doc = "Register `INTFLAGSET` writer"]
pub type W = crate::W<IntflagsetSpec>;
#[doc = "Field `DBG` reader - Debug Message Interrupt Flag Set"]
pub type DbgR = crate::BitReader;
#[doc = "Field `DBG` writer - Debug Message Interrupt Flag Set"]
pub type DbgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BERR` reader - AHB Bus Error Detection Interrupt Flag Set"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - AHB Bus Error Detection Interrupt Flag Set"]
pub type BerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Message Interrupt Flag Set"]
    #[inline(always)]
    pub fn dbg(&self) -> DbgR {
        DbgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB Bus Error Detection Interrupt Flag Set"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Message Interrupt Flag Set"]
    #[inline(always)]
    pub fn dbg(&mut self) -> DbgW<'_, IntflagsetSpec> {
        DbgW::new(self, 0)
    }
    #[doc = "Bit 1 - AHB Bus Error Detection Interrupt Flag Set"]
    #[inline(always)]
    pub fn berr(&mut self) -> BerrW<'_, IntflagsetSpec> {
        BerrW::new(self, 1)
    }
}
#[doc = "Interrupt Flag Software Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

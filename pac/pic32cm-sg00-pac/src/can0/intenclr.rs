#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `DBG` reader - Debug Message Interrupt Disable"]
pub type DbgR = crate::BitReader;
#[doc = "Field `DBG` writer - Debug Message Interrupt Disable"]
pub type DbgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BERR` reader - Bus Error Interrupt Enable"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - Bus Error Interrupt Enable"]
pub type BerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Message Interrupt Disable"]
    #[inline(always)]
    pub fn dbg(&self) -> DbgR {
        DbgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Message Interrupt Disable"]
    #[inline(always)]
    pub fn dbg(&mut self) -> DbgW<'_, IntenclrSpec> {
        DbgW::new(self, 0)
    }
    #[doc = "Bit 1 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn berr(&mut self) -> BerrW<'_, IntenclrSpec> {
        BerrW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}

#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `PREC` reader - Stop Received Interrupt Enable"]
pub type PrecR = crate::BitReader;
#[doc = "Field `PREC` writer - Stop Received Interrupt Enable"]
pub type PrecW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AMATCH` reader - Address Match Interrupt Enable"]
pub type AmatchR = crate::BitReader;
#[doc = "Field `AMATCH` writer - Address Match Interrupt Enable"]
pub type AmatchW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DRDY` reader - Data Ready Interrupt Enable"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `DRDY` writer - Data Ready Interrupt Enable"]
pub type DrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERROR` reader - Error Interrupt Enable"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Error Interrupt Enable"]
pub type ErrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop Received Interrupt Enable"]
    #[inline(always)]
    pub fn prec(&self) -> PrecR {
        PrecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Match Interrupt Enable"]
    #[inline(always)]
    pub fn amatch(&self) -> AmatchR {
        AmatchR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Received Interrupt Enable"]
    #[inline(always)]
    pub fn prec(&mut self) -> PrecW<'_, IntenclrSpec> {
        PrecW::new(self, 0)
    }
    #[doc = "Bit 1 - Address Match Interrupt Enable"]
    #[inline(always)]
    pub fn amatch(&mut self) -> AmatchW<'_, IntenclrSpec> {
        AmatchW::new(self, 1)
    }
    #[doc = "Bit 2 - Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn drdy(&mut self) -> DrdyW<'_, IntenclrSpec> {
        DrdyW::new(self, 2)
    }
    #[doc = "Bit 7 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntenclrSpec> {
        ErrorW::new(self, 7)
    }
}
#[doc = "I2CS Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x87;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}

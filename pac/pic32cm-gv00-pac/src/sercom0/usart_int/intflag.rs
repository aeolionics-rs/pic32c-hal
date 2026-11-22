#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `DRE` reader - Data Register Empty"]
pub type DreR = crate::BitReader;
#[doc = "Field `DRE` writer - Data Register Empty"]
pub type DreW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXC` reader - Transmit Complete"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - Transmit Complete"]
pub type TxcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXC` reader - Receive Complete"]
pub type RxcR = crate::BitReader;
#[doc = "Field `RXC` writer - Receive Complete"]
pub type RxcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXS` reader - Receive Start Interrupt"]
pub type RxsR = crate::BitReader;
#[doc = "Field `RXS` writer - Receive Start Interrupt"]
pub type RxsW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Register Empty"]
    #[inline(always)]
    pub fn dre(&self) -> DreR {
        DreR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Complete"]
    #[inline(always)]
    pub fn rxc(&self) -> RxcR {
        RxcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Start Interrupt"]
    #[inline(always)]
    pub fn rxs(&self) -> RxsR {
        RxsR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Register Empty"]
    #[inline(always)]
    pub fn dre(&mut self) -> DreW<'_, IntflagSpec> {
        DreW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Complete"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IntflagSpec> {
        TxcW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Complete"]
    #[inline(always)]
    pub fn rxc(&mut self) -> RxcW<'_, IntflagSpec> {
        RxcW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive Start Interrupt"]
    #[inline(always)]
    pub fn rxs(&mut self) -> RxsW<'_, IntflagSpec> {
        RxsW::new(self, 3)
    }
}
#[doc = "USART_INT Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x0f;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}

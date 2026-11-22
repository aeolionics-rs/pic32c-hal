#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `PERR` reader - Parity Error"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Parity Error"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` reader - Frame Error"]
pub type FerrR = crate::BitReader;
#[doc = "Field `FERR` writer - Frame Error"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOVF` reader - Buffer Overflow"]
pub type BufovfR = crate::BitReader;
#[doc = "Field `BUFOVF` writer - Buffer Overflow"]
pub type BufovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCBUSY` reader - Synchronization Busy"]
pub type SyncbusyR = crate::BitReader;
#[doc = "Field `SYNCBUSY` writer - Synchronization Busy"]
pub type SyncbusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&self) -> BufovfR {
        BufovfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 15 - Synchronization Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SyncbusyR {
        SyncbusyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, StatusSpec> {
        PerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame Error"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<'_, StatusSpec> {
        FerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&mut self) -> BufovfW<'_, StatusSpec> {
        BufovfW::new(self, 2)
    }
    #[doc = "Bit 15 - Synchronization Busy"]
    #[inline(always)]
    pub fn syncbusy(&mut self) -> SyncbusyW<'_, StatusSpec> {
        SyncbusyW::new(self, 15)
    }
}
#[doc = "USART_EXT Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}

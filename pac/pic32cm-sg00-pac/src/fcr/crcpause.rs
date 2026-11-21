#[doc = "Register `CRCPAUSE` reader"]
pub type R = crate::R<CrcpauseSpec>;
#[doc = "Register `CRCPAUSE` writer"]
pub type W = crate::W<CrcpauseSpec>;
#[doc = "CRC Pause\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pauseselect {
    #[doc = "0: CRC Reads Flash as Required"]
    Disable = 0,
    #[doc = "1: Pause CRC Reads of Flash"]
    Enable = 1,
}
impl From<Pauseselect> for bool {
    #[inline(always)]
    fn from(variant: Pauseselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSE` reader - CRC Pause"]
pub type PauseR = crate::BitReader<Pauseselect>;
impl PauseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pauseselect {
        match self.bits {
            false => Pauseselect::Disable,
            true => Pauseselect::Enable,
        }
    }
    #[doc = "CRC Reads Flash as Required"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pauseselect::Disable
    }
    #[doc = "Pause CRC Reads of Flash"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pauseselect::Enable
    }
}
#[doc = "Field `PAUSE` writer - CRC Pause"]
pub type PauseW<'a, REG> = crate::BitWriter<'a, REG, Pauseselect>;
impl<'a, REG> PauseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC Reads Flash as Required"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pauseselect::Disable)
    }
    #[doc = "Pause CRC Reads of Flash"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pauseselect::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - CRC Pause"]
    #[inline(always)]
    pub fn pause(&self) -> PauseR {
        PauseR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Pause"]
    #[inline(always)]
    pub fn pause(&mut self) -> PauseW<'_, CrcpauseSpec> {
        PauseW::new(self, 0)
    }
}
#[doc = "CRC Pause REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcpauseSpec;
impl crate::RegisterSpec for CrcpauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcpause::R`](R) reader structure"]
impl crate::Readable for CrcpauseSpec {}
#[doc = "`write(|w| ..)` method takes [`crcpause::W`](W) writer structure"]
impl crate::Writable for CrcpauseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCPAUSE to value 0"]
impl crate::Resettable for CrcpauseSpec {}

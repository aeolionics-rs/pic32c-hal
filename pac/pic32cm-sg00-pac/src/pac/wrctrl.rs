#[doc = "Register `WRCTRL` writer"]
pub type W = crate::W<WrctrlSpec>;
#[doc = "Field `PERID` writer - Peripheral Identifier"]
pub type PeridW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Peripheral Access Control Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyselect {
    #[doc = "0: OFF - No Action"]
    Off = 0,
    #[doc = "1: CLEAR - Clear the peripheral write protection"]
    Clear = 1,
    #[doc = "2: SET - Set the peripheral write protection"]
    Set = 2,
    #[doc = "3: LOCK - Set and Lock the write protection state of the peripheral until the next reset"]
    Lock = 3,
}
impl From<Keyselect> for u8 {
    #[inline(always)]
    fn from(variant: Keyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyselect {
    type Ux = u8;
}
impl crate::IsEnum for Keyselect {}
#[doc = "Field `KEY` writer - Peripheral Access Control Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Keyselect>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFF - No Action"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Off)
    }
    #[doc = "CLEAR - Clear the peripheral write protection"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Clear)
    }
    #[doc = "SET - Set the peripheral write protection"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Set)
    }
    #[doc = "LOCK - Set and Lock the write protection state of the peripheral until the next reset"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Lock)
    }
}
impl W {
    #[doc = "Bits 0:7 - Peripheral Identifier"]
    #[inline(always)]
    pub fn perid(&mut self) -> PeridW<'_, WrctrlSpec> {
        PeridW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Peripheral Access Control Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, WrctrlSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Write Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrctrlSpec;
impl crate::RegisterSpec for WrctrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wrctrl::W`](W) writer structure"]
impl crate::Writable for WrctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRCTRL to value 0"]
impl crate::Resettable for WrctrlSpec {}

#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Perselect {
    #[doc = "0: 8 clock cycles"]
    Per8 = 0,
    #[doc = "1: 16 clock cycles"]
    Per16 = 1,
    #[doc = "2: 32 clock cycles"]
    Per32 = 2,
    #[doc = "3: 64 clock cycles"]
    Per64 = 3,
    #[doc = "4: 128 clock cycles"]
    Per128 = 4,
    #[doc = "5: 256 clock cycles"]
    Per256 = 5,
    #[doc = "6: 512 clock cycles"]
    Per512 = 6,
    #[doc = "7: 1024 clock cycles"]
    _1k = 7,
    #[doc = "8: 2048 clock cycles"]
    _2k = 8,
    #[doc = "9: 4096 clock cycles"]
    _4k = 9,
    #[doc = "10: 8192 clock cycles"]
    _8k = 10,
    #[doc = "11: 16384 clock cycles"]
    _16k = 11,
}
impl From<Perselect> for u8 {
    #[inline(always)]
    fn from(variant: Perselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Perselect {
    type Ux = u8;
}
impl crate::IsEnum for Perselect {}
#[doc = "Field `PER` reader - Time-Out Period"]
pub type PerR = crate::FieldReader<Perselect>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Perselect> {
        match self.bits {
            0 => Some(Perselect::Per8),
            1 => Some(Perselect::Per16),
            2 => Some(Perselect::Per32),
            3 => Some(Perselect::Per64),
            4 => Some(Perselect::Per128),
            5 => Some(Perselect::Per256),
            6 => Some(Perselect::Per512),
            7 => Some(Perselect::_1k),
            8 => Some(Perselect::_2k),
            9 => Some(Perselect::_4k),
            10 => Some(Perselect::_8k),
            11 => Some(Perselect::_16k),
            _ => None,
        }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn is_per_8(&self) -> bool {
        *self == Perselect::Per8
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn is_per_16(&self) -> bool {
        *self == Perselect::Per16
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn is_per_32(&self) -> bool {
        *self == Perselect::Per32
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn is_per_64(&self) -> bool {
        *self == Perselect::Per64
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn is_per_128(&self) -> bool {
        *self == Perselect::Per128
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn is_per_256(&self) -> bool {
        *self == Perselect::Per256
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn is_per_512(&self) -> bool {
        *self == Perselect::Per512
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == Perselect::_1k
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == Perselect::_2k
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == Perselect::_4k
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Perselect::_8k
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Perselect::_16k
    }
}
#[doc = "Field `PER` writer - Time-Out Period"]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 4, Perselect>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn per_8(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Per8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn per_16(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Per16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn per_32(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Per32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn per_64(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Per64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn per_128(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Per128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn per_256(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Per256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn per_512(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Per512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_1k)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn _2k(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_2k)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_4k)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn _8k(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_8k)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_16k)
    }
}
#[doc = "Window Mode Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Windowselect {
    #[doc = "0: 8 clock cycles"]
    Window8 = 0,
    #[doc = "1: 16 clock cycles"]
    Window16 = 1,
    #[doc = "2: 32 clock cycles"]
    Window32 = 2,
    #[doc = "3: 64 clock cycles"]
    Window64 = 3,
    #[doc = "4: 128 clock cycles"]
    Window128 = 4,
    #[doc = "5: 256 clock cycles"]
    Window256 = 5,
    #[doc = "6: 512 clock cycles"]
    Window512 = 6,
    #[doc = "7: 1024 clock cycles"]
    _1k = 7,
    #[doc = "8: 2048 clock cycles"]
    _2k = 8,
    #[doc = "9: 4096 clock cycles"]
    _4k = 9,
    #[doc = "10: 8192 clock cycles"]
    _8k = 10,
    #[doc = "11: 16384 clock cycles"]
    _16k = 11,
}
impl From<Windowselect> for u8 {
    #[inline(always)]
    fn from(variant: Windowselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Windowselect {
    type Ux = u8;
}
impl crate::IsEnum for Windowselect {}
#[doc = "Field `WINDOW` reader - Window Mode Time-Out Period"]
pub type WindowR = crate::FieldReader<Windowselect>;
impl WindowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Windowselect> {
        match self.bits {
            0 => Some(Windowselect::Window8),
            1 => Some(Windowselect::Window16),
            2 => Some(Windowselect::Window32),
            3 => Some(Windowselect::Window64),
            4 => Some(Windowselect::Window128),
            5 => Some(Windowselect::Window256),
            6 => Some(Windowselect::Window512),
            7 => Some(Windowselect::_1k),
            8 => Some(Windowselect::_2k),
            9 => Some(Windowselect::_4k),
            10 => Some(Windowselect::_8k),
            11 => Some(Windowselect::_16k),
            _ => None,
        }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn is_window_8(&self) -> bool {
        *self == Windowselect::Window8
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn is_window_16(&self) -> bool {
        *self == Windowselect::Window16
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn is_window_32(&self) -> bool {
        *self == Windowselect::Window32
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn is_window_64(&self) -> bool {
        *self == Windowselect::Window64
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn is_window_128(&self) -> bool {
        *self == Windowselect::Window128
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn is_window_256(&self) -> bool {
        *self == Windowselect::Window256
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn is_window_512(&self) -> bool {
        *self == Windowselect::Window512
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == Windowselect::_1k
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == Windowselect::_2k
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == Windowselect::_4k
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Windowselect::_8k
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Windowselect::_16k
    }
}
#[doc = "Field `WINDOW` writer - Window Mode Time-Out Period"]
pub type WindowW<'a, REG> = crate::FieldWriter<'a, REG, 4, Windowselect>;
impl<'a, REG> WindowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn window_8(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Window8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn window_16(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Window16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn window_32(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Window32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn window_64(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Window64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn window_128(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Window128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn window_256(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Window256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn window_512(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Window512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_1k)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn _2k(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_2k)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_4k)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn _8k(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_8k)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_16k)
    }
}
impl R {
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline(always)]
    pub fn window(&self) -> WindowR {
        WindowR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, ConfigSpec> {
        PerW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline(always)]
    pub fn window(&mut self) -> WindowW<'_, ConfigSpec> {
        WindowW::new(self, 4)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0xbb"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u8 = 0xbb;
}

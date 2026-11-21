#[doc = "Register `USERCFG` reader"]
pub type R = crate::R<UsercfgSpec>;
#[doc = "Register `USERCFG` writer"]
pub type W = crate::W<UsercfgSpec>;
#[doc = "CRC Polynomial Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcselselect {
    #[doc = "0: CRC-16-CCITT"]
    Crc16 = 0,
    #[doc = "1: CRC-32 (IEEE 802.3)"]
    Crc32 = 1,
}
impl From<Crcselselect> for bool {
    #[inline(always)]
    fn from(variant: Crcselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCSEL` reader - CRC Polynomial Selection"]
pub type CrcselR = crate::BitReader<Crcselselect>;
impl CrcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcselselect {
        match self.bits {
            false => Crcselselect::Crc16,
            true => Crcselselect::Crc32,
        }
    }
    #[doc = "CRC-16-CCITT"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        *self == Crcselselect::Crc16
    }
    #[doc = "CRC-32 (IEEE 802.3)"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == Crcselselect::Crc32
    }
}
#[doc = "Field `CRCSEL` writer - CRC Polynomial Selection"]
pub type CrcselW<'a, REG> = crate::BitWriter<'a, REG, Crcselselect>;
impl<'a, REG> CrcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC-16-CCITT"]
    #[inline(always)]
    pub fn crc16(self) -> &'a mut crate::W<REG> {
        self.variant(Crcselselect::Crc16)
    }
    #[doc = "CRC-32 (IEEE 802.3)"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut crate::W<REG> {
        self.variant(Crcselselect::Crc32)
    }
}
#[doc = "Field `CRCBOOT` reader - CRC Boot"]
pub type CrcbootR = crate::BitReader;
#[doc = "Field `CRCBOOT` writer - CRC Boot"]
pub type CrcbootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Start-up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sutselect {
    #[doc = "0: 0 ms"]
    _0ms = 0,
    #[doc = "1: 1 ms"]
    _1ms = 1,
    #[doc = "2: 2 ms"]
    _2ms = 2,
    #[doc = "3: 4 ms"]
    _4ms = 3,
    #[doc = "4: 8 ms"]
    _8ms = 4,
    #[doc = "5: 16 ms"]
    _16ms = 5,
    #[doc = "6: 32 ms"]
    _32ms = 6,
    #[doc = "7: 64 ms"]
    _64ms = 7,
}
impl From<Sutselect> for u8 {
    #[inline(always)]
    fn from(variant: Sutselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sutselect {
    type Ux = u8;
}
impl crate::IsEnum for Sutselect {}
#[doc = "Field `SUT` reader - Start-up Time"]
pub type SutR = crate::FieldReader<Sutselect>;
impl SutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sutselect {
        match self.bits {
            0 => Sutselect::_0ms,
            1 => Sutselect::_1ms,
            2 => Sutselect::_2ms,
            3 => Sutselect::_4ms,
            4 => Sutselect::_8ms,
            5 => Sutselect::_16ms,
            6 => Sutselect::_32ms,
            7 => Sutselect::_64ms,
            _ => unreachable!(),
        }
    }
    #[doc = "0 ms"]
    #[inline(always)]
    pub fn is_0ms(&self) -> bool {
        *self == Sutselect::_0ms
    }
    #[doc = "1 ms"]
    #[inline(always)]
    pub fn is_1ms(&self) -> bool {
        *self == Sutselect::_1ms
    }
    #[doc = "2 ms"]
    #[inline(always)]
    pub fn is_2ms(&self) -> bool {
        *self == Sutselect::_2ms
    }
    #[doc = "4 ms"]
    #[inline(always)]
    pub fn is_4ms(&self) -> bool {
        *self == Sutselect::_4ms
    }
    #[doc = "8 ms"]
    #[inline(always)]
    pub fn is_8ms(&self) -> bool {
        *self == Sutselect::_8ms
    }
    #[doc = "16 ms"]
    #[inline(always)]
    pub fn is_16ms(&self) -> bool {
        *self == Sutselect::_16ms
    }
    #[doc = "32 ms"]
    #[inline(always)]
    pub fn is_32ms(&self) -> bool {
        *self == Sutselect::_32ms
    }
    #[doc = "64 ms"]
    #[inline(always)]
    pub fn is_64ms(&self) -> bool {
        *self == Sutselect::_64ms
    }
}
#[doc = "Field `SUT` writer - Start-up Time"]
pub type SutW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sutselect, crate::Safe>;
impl<'a, REG> SutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 ms"]
    #[inline(always)]
    pub fn _0ms(self) -> &'a mut crate::W<REG> {
        self.variant(Sutselect::_0ms)
    }
    #[doc = "1 ms"]
    #[inline(always)]
    pub fn _1ms(self) -> &'a mut crate::W<REG> {
        self.variant(Sutselect::_1ms)
    }
    #[doc = "2 ms"]
    #[inline(always)]
    pub fn _2ms(self) -> &'a mut crate::W<REG> {
        self.variant(Sutselect::_2ms)
    }
    #[doc = "4 ms"]
    #[inline(always)]
    pub fn _4ms(self) -> &'a mut crate::W<REG> {
        self.variant(Sutselect::_4ms)
    }
    #[doc = "8 ms"]
    #[inline(always)]
    pub fn _8ms(self) -> &'a mut crate::W<REG> {
        self.variant(Sutselect::_8ms)
    }
    #[doc = "16 ms"]
    #[inline(always)]
    pub fn _16ms(self) -> &'a mut crate::W<REG> {
        self.variant(Sutselect::_16ms)
    }
    #[doc = "32 ms"]
    #[inline(always)]
    pub fn _32ms(self) -> &'a mut crate::W<REG> {
        self.variant(Sutselect::_32ms)
    }
    #[doc = "64 ms"]
    #[inline(always)]
    pub fn _64ms(self) -> &'a mut crate::W<REG> {
        self.variant(Sutselect::_64ms)
    }
}
impl R {
    #[doc = "Bit 6 - CRC Polynomial Selection"]
    #[inline(always)]
    pub fn crcsel(&self) -> CrcselR {
        CrcselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC Boot"]
    #[inline(always)]
    pub fn crcboot(&self) -> CrcbootR {
        CrcbootR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Start-up Time"]
    #[inline(always)]
    pub fn sut(&self) -> SutR {
        SutR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - CRC Polynomial Selection"]
    #[inline(always)]
    pub fn crcsel(&mut self) -> CrcselW<'_, UsercfgSpec> {
        CrcselW::new(self, 6)
    }
    #[doc = "Bit 7 - CRC Boot"]
    #[inline(always)]
    pub fn crcboot(&mut self) -> CrcbootW<'_, UsercfgSpec> {
        CrcbootW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Start-up Time"]
    #[inline(always)]
    pub fn sut(&mut self) -> SutW<'_, UsercfgSpec> {
        SutW::new(self, 8)
    }
}
#[doc = "User Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`usercfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usercfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsercfgSpec;
impl crate::RegisterSpec for UsercfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usercfg::R`](R) reader structure"]
impl crate::Readable for UsercfgSpec {}
#[doc = "`write(|w| ..)` method takes [`usercfg::W`](W) writer structure"]
impl crate::Writable for UsercfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USERCFG to value 0"]
impl crate::Resettable for UsercfgSpec {}

#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsizeselect {
    #[doc = "0: 8-bits character"]
    _8bits = 0,
    #[doc = "1: 9-bits character"]
    _9bits = 1,
    #[doc = "5: 5-bits character"]
    _5bits = 5,
    #[doc = "6: 6-bits character"]
    _6bits = 6,
    #[doc = "7: 7-bits character"]
    _7bits = 7,
}
impl From<Chsizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Chsizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Chsizeselect {}
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type ChsizeR = crate::FieldReader<Chsizeselect>;
impl ChsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsizeselect> {
        match self.bits {
            0 => Some(Chsizeselect::_8bits),
            1 => Some(Chsizeselect::_9bits),
            5 => Some(Chsizeselect::_5bits),
            6 => Some(Chsizeselect::_6bits),
            7 => Some(Chsizeselect::_7bits),
            _ => None,
        }
    }
    #[doc = "8-bits character"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == Chsizeselect::_8bits
    }
    #[doc = "9-bits character"]
    #[inline(always)]
    pub fn is_9bits(&self) -> bool {
        *self == Chsizeselect::_9bits
    }
    #[doc = "5-bits character"]
    #[inline(always)]
    pub fn is_5bits(&self) -> bool {
        *self == Chsizeselect::_5bits
    }
    #[doc = "6-bits character"]
    #[inline(always)]
    pub fn is_6bits(&self) -> bool {
        *self == Chsizeselect::_6bits
    }
    #[doc = "7-bits character"]
    #[inline(always)]
    pub fn is_7bits(&self) -> bool {
        *self == Chsizeselect::_7bits
    }
}
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type ChsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Chsizeselect>;
impl<'a, REG> ChsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bits character"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_8bits)
    }
    #[doc = "9-bits character"]
    #[inline(always)]
    pub fn _9bits(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_9bits)
    }
    #[doc = "5-bits character"]
    #[inline(always)]
    pub fn _5bits(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_5bits)
    }
    #[doc = "6-bits character"]
    #[inline(always)]
    pub fn _6bits(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_6bits)
    }
    #[doc = "7-bits character"]
    #[inline(always)]
    pub fn _7bits(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_7bits)
    }
}
#[doc = "Stop Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbmodeselect {
    #[doc = "0: One stop bit"]
    One = 0,
    #[doc = "1: Two stop bits"]
    Two = 1,
}
impl From<Sbmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Sbmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBMODE` reader - Stop Bit Mode"]
pub type SbmodeR = crate::BitReader<Sbmodeselect>;
impl SbmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbmodeselect {
        match self.bits {
            false => Sbmodeselect::One,
            true => Sbmodeselect::Two,
        }
    }
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Sbmodeselect::One
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Sbmodeselect::Two
    }
}
#[doc = "Field `SBMODE` writer - Stop Bit Mode"]
pub type SbmodeW<'a, REG> = crate::BitWriter<'a, REG, Sbmodeselect>;
impl<'a, REG> SbmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Sbmodeselect::One)
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Sbmodeselect::Two)
    }
}
#[doc = "Field `COLDEN` reader - Collision Detection Enable"]
pub type ColdenR = crate::BitReader;
#[doc = "Field `COLDEN` writer - Collision Detection Enable"]
pub type ColdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFDE` reader - Start of Frame Detection Enable"]
pub type SfdeR = crate::BitReader;
#[doc = "Field `SFDE` writer - Start of Frame Detection Enable"]
pub type SfdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Encoding Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encselect {
    #[doc = "0: Data is not encoded"]
    Disable = 0,
    #[doc = "1: Data is IrDA encoded"]
    Irda = 1,
}
impl From<Encselect> for bool {
    #[inline(always)]
    fn from(variant: Encselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENC` reader - Encoding Format"]
pub type EncR = crate::BitReader<Encselect>;
impl EncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Encselect {
        match self.bits {
            false => Encselect::Disable,
            true => Encselect::Irda,
        }
    }
    #[doc = "Data is not encoded"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Encselect::Disable
    }
    #[doc = "Data is IrDA encoded"]
    #[inline(always)]
    pub fn is_irda(&self) -> bool {
        *self == Encselect::Irda
    }
}
#[doc = "Field `ENC` writer - Encoding Format"]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG, Encselect>;
impl<'a, REG> EncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is not encoded"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Encselect::Disable)
    }
    #[doc = "Data is IrDA encoded"]
    #[inline(always)]
    pub fn irda(self) -> &'a mut crate::W<REG> {
        self.variant(Encselect::Irda)
    }
}
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmodeselect {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
}
impl From<Pmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Pmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMODE` reader - Parity Mode"]
pub type PmodeR = crate::BitReader<Pmodeselect>;
impl PmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmodeselect {
        match self.bits {
            false => Pmodeselect::Even,
            true => Pmodeselect::Odd,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Pmodeselect::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Pmodeselect::Odd
    }
}
#[doc = "Field `PMODE` writer - Parity Mode"]
pub type PmodeW<'a, REG> = crate::BitWriter<'a, REG, Pmodeselect>;
impl<'a, REG> PmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Pmodeselect::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Pmodeselect::Odd)
    }
}
#[doc = "Field `TXEN` reader - Transmitter Enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> ChsizeR {
        ChsizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    pub fn sbmode(&self) -> SbmodeR {
        SbmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    pub fn colden(&self) -> ColdenR {
        ColdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde(&self) -> SfdeR {
        SfdeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    pub fn pmode(&self) -> PmodeR {
        PmodeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&mut self) -> ChsizeW<'_, CtrlbSpec> {
        ChsizeW::new(self, 0)
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    pub fn sbmode(&mut self) -> SbmodeW<'_, CtrlbSpec> {
        SbmodeW::new(self, 6)
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    pub fn colden(&mut self) -> ColdenW<'_, CtrlbSpec> {
        ColdenW::new(self, 8)
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde(&mut self) -> SfdeW<'_, CtrlbSpec> {
        SfdeW::new(self, 9)
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    pub fn enc(&mut self) -> EncW<'_, CtrlbSpec> {
        EncW::new(self, 10)
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    pub fn pmode(&mut self) -> PmodeW<'_, CtrlbSpec> {
        PmodeW::new(self, 13)
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, CtrlbSpec> {
        TxenW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, CtrlbSpec> {
        RxenW::new(self, 17)
    }
}
#[doc = "USART Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}

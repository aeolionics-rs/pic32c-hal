#[doc = "Register `COMPCTRLA` reader"]
pub type R = crate::R<CompctrlaSpec>;
#[doc = "Register `COMPCTRLA` writer"]
pub type W = crate::W<CompctrlaSpec>;
#[doc = "Field `ENABLE` reader - Comparator Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Comparator Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT` reader - Output Enable"]
pub type OutR = crate::BitReader;
#[doc = "Field `OUT` writer - Output Enable"]
pub type OutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTEN` reader - Hysteresis Enable"]
pub type HystenR = crate::BitReader;
#[doc = "Field `HYSTEN` writer - Hysteresis Enable"]
pub type HystenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Intselselect {
    #[doc = "0: Interrupt on comparator n output toggle"]
    Toggle = 0,
    #[doc = "1: Interrupt on comparator n output rising edge"]
    Rising = 1,
    #[doc = "2: Interrupt on comparator n output falling edge"]
    Falling = 2,
}
impl From<Intselselect> for u8 {
    #[inline(always)]
    fn from(variant: Intselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Intselselect {
    type Ux = u8;
}
impl crate::IsEnum for Intselselect {}
#[doc = "Field `INTSEL` reader - Interrupt Selection"]
pub type IntselR = crate::FieldReader<Intselselect>;
impl IntselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Intselselect> {
        match self.bits {
            0 => Some(Intselselect::Toggle),
            1 => Some(Intselselect::Rising),
            2 => Some(Intselselect::Falling),
            _ => None,
        }
    }
    #[doc = "Interrupt on comparator n output toggle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Intselselect::Toggle
    }
    #[doc = "Interrupt on comparator n output rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Intselselect::Rising
    }
    #[doc = "Interrupt on comparator n output falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Intselselect::Falling
    }
}
#[doc = "Field `INTSEL` writer - Interrupt Selection"]
pub type IntselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Intselselect>;
impl<'a, REG> IntselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt on comparator n output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Intselselect::Toggle)
    }
    #[doc = "Interrupt on comparator n output rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Intselselect::Rising)
    }
    #[doc = "Interrupt on comparator n output falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Intselselect::Falling)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Negative Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxnegselect {
    #[doc = "0: Negative pin 0"]
    Ainn0 = 0,
    #[doc = "1: Negative pin 1"]
    Ainn1 = 1,
    #[doc = "2: Negative pin 2"]
    Ainn2 = 2,
    #[doc = "7: Voltage Reference Scaler"]
    Vrefscale = 7,
}
impl From<Muxnegselect> for u8 {
    #[inline(always)]
    fn from(variant: Muxnegselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Muxnegselect {
    type Ux = u8;
}
impl crate::IsEnum for Muxnegselect {}
#[doc = "Field `MUXNEG` reader - Negative Input Selection"]
pub type MuxnegR = crate::FieldReader<Muxnegselect>;
impl MuxnegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Muxnegselect> {
        match self.bits {
            0 => Some(Muxnegselect::Ainn0),
            1 => Some(Muxnegselect::Ainn1),
            2 => Some(Muxnegselect::Ainn2),
            7 => Some(Muxnegselect::Vrefscale),
            _ => None,
        }
    }
    #[doc = "Negative pin 0"]
    #[inline(always)]
    pub fn is_ainn0(&self) -> bool {
        *self == Muxnegselect::Ainn0
    }
    #[doc = "Negative pin 1"]
    #[inline(always)]
    pub fn is_ainn1(&self) -> bool {
        *self == Muxnegselect::Ainn1
    }
    #[doc = "Negative pin 2"]
    #[inline(always)]
    pub fn is_ainn2(&self) -> bool {
        *self == Muxnegselect::Ainn2
    }
    #[doc = "Voltage Reference Scaler"]
    #[inline(always)]
    pub fn is_vrefscale(&self) -> bool {
        *self == Muxnegselect::Vrefscale
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input Selection"]
pub type MuxnegW<'a, REG> = crate::FieldWriter<'a, REG, 3, Muxnegselect>;
impl<'a, REG> MuxnegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Negative pin 0"]
    #[inline(always)]
    pub fn ainn0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ainn0)
    }
    #[doc = "Negative pin 1"]
    #[inline(always)]
    pub fn ainn1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ainn1)
    }
    #[doc = "Negative pin 2"]
    #[inline(always)]
    pub fn ainn2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ainn2)
    }
    #[doc = "Voltage Reference Scaler"]
    #[inline(always)]
    pub fn vrefscale(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Vrefscale)
    }
}
#[doc = "Positive Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxposselect {
    #[doc = "0: Positive pin 0"]
    Ainp0 = 0,
    #[doc = "1: Positive pin 1"]
    Ainp1 = 1,
    #[doc = "2: Positive pin 2"]
    Ainp2 = 2,
    #[doc = "3: Positive pin 3"]
    Ainp3 = 3,
    #[doc = "5: Positive pin 5"]
    Ainp5 = 5,
    #[doc = "6: Positive pin 6"]
    Ainp6 = 6,
    #[doc = "7: VDDIO2 Div10"]
    Vddio2div10 = 7,
}
impl From<Muxposselect> for u8 {
    #[inline(always)]
    fn from(variant: Muxposselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Muxposselect {
    type Ux = u8;
}
impl crate::IsEnum for Muxposselect {}
#[doc = "Field `MUXPOS` reader - Positive Input Selection"]
pub type MuxposR = crate::FieldReader<Muxposselect>;
impl MuxposR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Muxposselect> {
        match self.bits {
            0 => Some(Muxposselect::Ainp0),
            1 => Some(Muxposselect::Ainp1),
            2 => Some(Muxposselect::Ainp2),
            3 => Some(Muxposselect::Ainp3),
            5 => Some(Muxposselect::Ainp5),
            6 => Some(Muxposselect::Ainp6),
            7 => Some(Muxposselect::Vddio2div10),
            _ => None,
        }
    }
    #[doc = "Positive pin 0"]
    #[inline(always)]
    pub fn is_ainp0(&self) -> bool {
        *self == Muxposselect::Ainp0
    }
    #[doc = "Positive pin 1"]
    #[inline(always)]
    pub fn is_ainp1(&self) -> bool {
        *self == Muxposselect::Ainp1
    }
    #[doc = "Positive pin 2"]
    #[inline(always)]
    pub fn is_ainp2(&self) -> bool {
        *self == Muxposselect::Ainp2
    }
    #[doc = "Positive pin 3"]
    #[inline(always)]
    pub fn is_ainp3(&self) -> bool {
        *self == Muxposselect::Ainp3
    }
    #[doc = "Positive pin 5"]
    #[inline(always)]
    pub fn is_ainp5(&self) -> bool {
        *self == Muxposselect::Ainp5
    }
    #[doc = "Positive pin 6"]
    #[inline(always)]
    pub fn is_ainp6(&self) -> bool {
        *self == Muxposselect::Ainp6
    }
    #[doc = "VDDIO2 Div10"]
    #[inline(always)]
    pub fn is_vddio2div10(&self) -> bool {
        *self == Muxposselect::Vddio2div10
    }
}
#[doc = "Field `MUXPOS` writer - Positive Input Selection"]
pub type MuxposW<'a, REG> = crate::FieldWriter<'a, REG, 3, Muxposselect>;
impl<'a, REG> MuxposW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Positive pin 0"]
    #[inline(always)]
    pub fn ainp0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ainp0)
    }
    #[doc = "Positive pin 1"]
    #[inline(always)]
    pub fn ainp1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ainp1)
    }
    #[doc = "Positive pin 2"]
    #[inline(always)]
    pub fn ainp2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ainp2)
    }
    #[doc = "Positive pin 3"]
    #[inline(always)]
    pub fn ainp3(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ainp3)
    }
    #[doc = "Positive pin 5"]
    #[inline(always)]
    pub fn ainp5(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ainp5)
    }
    #[doc = "Positive pin 6"]
    #[inline(always)]
    pub fn ainp6(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ainp6)
    }
    #[doc = "VDDIO2 Div10"]
    #[inline(always)]
    pub fn vddio2div10(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Vddio2div10)
    }
}
impl R {
    #[doc = "Bit 1 - Comparator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Enable"]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&self) -> HystenR {
        HystenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&self) -> IntselR {
        IntselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Negative Input Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MuxnegR {
        MuxnegR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Positive Input Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MuxposR {
        MuxposR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CompctrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Output Enable"]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<'_, CompctrlaSpec> {
        OutW::new(self, 2)
    }
    #[doc = "Bit 3 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&mut self) -> HystenW<'_, CompctrlaSpec> {
        HystenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&mut self) -> IntselW<'_, CompctrlaSpec> {
        IntselW::new(self, 4)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, CompctrlaSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Negative Input Selection"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MuxnegW<'_, CompctrlaSpec> {
        MuxnegW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Positive Input Selection"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MuxposW<'_, CompctrlaSpec> {
        MuxposW::new(self, 12)
    }
}
#[doc = "Comparator # Comparator Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`compctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompctrlaSpec;
impl crate::RegisterSpec for CompctrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compctrla::R`](R) reader structure"]
impl crate::Readable for CompctrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`compctrla::W`](W) writer structure"]
impl crate::Writable for CompctrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPCTRLA to value 0"]
impl crate::Resettable for CompctrlaSpec {}

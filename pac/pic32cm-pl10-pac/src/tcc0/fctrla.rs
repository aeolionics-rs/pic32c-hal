#[doc = "Register `FCTRLA` reader"]
pub type R = crate::R<FctrlaSpec>;
#[doc = "Register `FCTRLA` writer"]
pub type W = crate::W<FctrlaSpec>;
#[doc = "Recoverable Fault Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srcselect {
    #[doc = "0: Fault input disabled"]
    Disable = 0,
    #[doc = "1: MCEn (n = 0,1) event input"]
    Enable = 1,
    #[doc = "2: Inverted MCEn (n = 0,1) event input"]
    Invert = 2,
    #[doc = "3: Alternate fault (A or B) state at the end of the previous period."]
    Altfault = 3,
}
impl From<Srcselect> for u8 {
    #[inline(always)]
    fn from(variant: Srcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srcselect {
    type Ux = u8;
}
impl crate::IsEnum for Srcselect {}
#[doc = "Field `SRC` reader - Recoverable Fault Source"]
pub type SrcR = crate::FieldReader<Srcselect>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcselect {
        match self.bits {
            0 => Srcselect::Disable,
            1 => Srcselect::Enable,
            2 => Srcselect::Invert,
            3 => Srcselect::Altfault,
            _ => unreachable!(),
        }
    }
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Srcselect::Disable
    }
    #[doc = "MCEn (n = 0,1) event input"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Srcselect::Enable
    }
    #[doc = "Inverted MCEn (n = 0,1) event input"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == Srcselect::Invert
    }
    #[doc = "Alternate fault (A or B) state at the end of the previous period."]
    #[inline(always)]
    pub fn is_altfault(&self) -> bool {
        *self == Srcselect::Altfault
    }
}
#[doc = "Field `SRC` writer - Recoverable Fault Source"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Srcselect, crate::Safe>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Disable)
    }
    #[doc = "MCEn (n = 0,1) event input"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Enable)
    }
    #[doc = "Inverted MCEn (n = 0,1) event input"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Invert)
    }
    #[doc = "Alternate fault (A or B) state at the end of the previous period."]
    #[inline(always)]
    pub fn altfault(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Altfault)
    }
}
#[doc = "Field `KEEP` reader - Recoverable Fault Keep"]
pub type KeepR = crate::BitReader;
#[doc = "Field `KEEP` writer - Recoverable Fault Keep"]
pub type KeepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUAL` reader - Recoverable Fault Qualification"]
pub type QualR = crate::BitReader;
#[doc = "Field `QUAL` writer - Recoverable Fault Qualification"]
pub type QualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Recoverable Fault Blanking Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Blankselect {
    #[doc = "0: Blanking is applied from the start of the ramp period"]
    Start = 0,
    #[doc = "1: Blanking is applied from the rising edge of the waveform output"]
    Rise = 1,
    #[doc = "2: Blanking is applied from the falling edge of the waveform output"]
    Fall = 2,
    #[doc = "3: Blanking is applied from each toggle of the waveform output"]
    Both = 3,
}
impl From<Blankselect> for u8 {
    #[inline(always)]
    fn from(variant: Blankselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blankselect {
    type Ux = u8;
}
impl crate::IsEnum for Blankselect {}
#[doc = "Field `BLANK` reader - Recoverable Fault Blanking Operation"]
pub type BlankR = crate::FieldReader<Blankselect>;
impl BlankR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blankselect {
        match self.bits {
            0 => Blankselect::Start,
            1 => Blankselect::Rise,
            2 => Blankselect::Fall,
            3 => Blankselect::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Blanking is applied from the start of the ramp period"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Blankselect::Start
    }
    #[doc = "Blanking is applied from the rising edge of the waveform output"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Blankselect::Rise
    }
    #[doc = "Blanking is applied from the falling edge of the waveform output"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Blankselect::Fall
    }
    #[doc = "Blanking is applied from each toggle of the waveform output"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Blankselect::Both
    }
}
#[doc = "Field `BLANK` writer - Recoverable Fault Blanking Operation"]
pub type BlankW<'a, REG> = crate::FieldWriter<'a, REG, 2, Blankselect, crate::Safe>;
impl<'a, REG> BlankW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Blanking is applied from the start of the ramp period"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Blankselect::Start)
    }
    #[doc = "Blanking is applied from the rising edge of the waveform output"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Blankselect::Rise)
    }
    #[doc = "Blanking is applied from the falling edge of the waveform output"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Blankselect::Fall)
    }
    #[doc = "Blanking is applied from each toggle of the waveform output"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Blankselect::Both)
    }
}
#[doc = "Field `RESTART` reader - Recoverable Fault Restart"]
pub type RestartR = crate::BitReader;
#[doc = "Field `RESTART` writer - Recoverable Fault Restart"]
pub type RestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Recoverable Fault Halt Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Haltselect {
    #[doc = "0: Halt action disabled"]
    Disable = 0,
    #[doc = "1: Hardware halt action"]
    Hw = 1,
    #[doc = "2: Software halt action"]
    Sw = 2,
    #[doc = "3: Non-recoverable fault"]
    Nr = 3,
}
impl From<Haltselect> for u8 {
    #[inline(always)]
    fn from(variant: Haltselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Haltselect {
    type Ux = u8;
}
impl crate::IsEnum for Haltselect {}
#[doc = "Field `HALT` reader - Recoverable Fault Halt Operation"]
pub type HaltR = crate::FieldReader<Haltselect>;
impl HaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Haltselect {
        match self.bits {
            0 => Haltselect::Disable,
            1 => Haltselect::Hw,
            2 => Haltselect::Sw,
            3 => Haltselect::Nr,
            _ => unreachable!(),
        }
    }
    #[doc = "Halt action disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Haltselect::Disable
    }
    #[doc = "Hardware halt action"]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        *self == Haltselect::Hw
    }
    #[doc = "Software halt action"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == Haltselect::Sw
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn is_nr(&self) -> bool {
        *self == Haltselect::Nr
    }
}
#[doc = "Field `HALT` writer - Recoverable Fault Halt Operation"]
pub type HaltW<'a, REG> = crate::FieldWriter<'a, REG, 2, Haltselect, crate::Safe>;
impl<'a, REG> HaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Halt action disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Haltselect::Disable)
    }
    #[doc = "Hardware halt action"]
    #[inline(always)]
    pub fn hw(self) -> &'a mut crate::W<REG> {
        self.variant(Haltselect::Hw)
    }
    #[doc = "Software halt action"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(Haltselect::Sw)
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn nr(self) -> &'a mut crate::W<REG> {
        self.variant(Haltselect::Nr)
    }
}
#[doc = "Recoverable Fault Capture Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chselselect {
    #[doc = "0: The captured value stored in CC0"]
    Cc0 = 0,
    #[doc = "1: The captured value stored in CC1"]
    Cc1 = 1,
    #[doc = "2: The captured value stored in CC2"]
    Cc2 = 2,
    #[doc = "3: The captured value stored in CC3"]
    Cc3 = 3,
}
impl From<Chselselect> for u8 {
    #[inline(always)]
    fn from(variant: Chselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chselselect {
    type Ux = u8;
}
impl crate::IsEnum for Chselselect {}
#[doc = "Field `CHSEL` reader - Recoverable Fault Capture Channel"]
pub type ChselR = crate::FieldReader<Chselselect>;
impl ChselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chselselect {
        match self.bits {
            0 => Chselselect::Cc0,
            1 => Chselselect::Cc1,
            2 => Chselselect::Cc2,
            3 => Chselselect::Cc3,
            _ => unreachable!(),
        }
    }
    #[doc = "The captured value stored in CC0"]
    #[inline(always)]
    pub fn is_cc0(&self) -> bool {
        *self == Chselselect::Cc0
    }
    #[doc = "The captured value stored in CC1"]
    #[inline(always)]
    pub fn is_cc1(&self) -> bool {
        *self == Chselselect::Cc1
    }
    #[doc = "The captured value stored in CC2"]
    #[inline(always)]
    pub fn is_cc2(&self) -> bool {
        *self == Chselselect::Cc2
    }
    #[doc = "The captured value stored in CC3"]
    #[inline(always)]
    pub fn is_cc3(&self) -> bool {
        *self == Chselselect::Cc3
    }
}
#[doc = "Field `CHSEL` writer - Recoverable Fault Capture Channel"]
pub type ChselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chselselect, crate::Safe>;
impl<'a, REG> ChselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The captured value stored in CC0"]
    #[inline(always)]
    pub fn cc0(self) -> &'a mut crate::W<REG> {
        self.variant(Chselselect::Cc0)
    }
    #[doc = "The captured value stored in CC1"]
    #[inline(always)]
    pub fn cc1(self) -> &'a mut crate::W<REG> {
        self.variant(Chselselect::Cc1)
    }
    #[doc = "The captured value stored in CC2"]
    #[inline(always)]
    pub fn cc2(self) -> &'a mut crate::W<REG> {
        self.variant(Chselselect::Cc2)
    }
    #[doc = "The captured value stored in CC3"]
    #[inline(always)]
    pub fn cc3(self) -> &'a mut crate::W<REG> {
        self.variant(Chselselect::Cc3)
    }
}
#[doc = "Recoverable Fault Capture Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Captureselect {
    #[doc = "0: Capture on valid recoverable Fault is disabled"]
    Disable = 0,
    #[doc = "1: On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\]The INTFLAG.FAULTn flag is set on each new captured value."]
    Capt = 1,
    #[doc = "2: On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\] if the COUNT value is lower than the last stored capture value (CC).The INTFLAG.FAULTn flag is set on each local minimum detection."]
    Captmin = 2,
    #[doc = "3: On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\] if the COUNT value is higher than the last stored capture value (CC).The INTFLAG.FAULTn flag is set on each local maximun detection."]
    Captmax = 3,
    #[doc = "4: On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\].The INTFLAG.FAULTn flag is set on each local minimum value detection."]
    Locmin = 4,
    #[doc = "5: On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\].The INTFLAG.FAULTn flag is set on each local maximun detection."]
    Locmax = 5,
    #[doc = "6: On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\].The INTFLAG.FAULTn flag is set on each local maximun or minimum detection."]
    Deriv0 = 6,
    #[doc = "7: Capture with the ramp index as the MSB value"]
    Captmark = 7,
}
impl From<Captureselect> for u8 {
    #[inline(always)]
    fn from(variant: Captureselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Captureselect {
    type Ux = u8;
}
impl crate::IsEnum for Captureselect {}
#[doc = "Field `CAPTURE` reader - Recoverable Fault Capture Action"]
pub type CaptureR = crate::FieldReader<Captureselect>;
impl CaptureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captureselect {
        match self.bits {
            0 => Captureselect::Disable,
            1 => Captureselect::Capt,
            2 => Captureselect::Captmin,
            3 => Captureselect::Captmax,
            4 => Captureselect::Locmin,
            5 => Captureselect::Locmax,
            6 => Captureselect::Deriv0,
            7 => Captureselect::Captmark,
            _ => unreachable!(),
        }
    }
    #[doc = "Capture on valid recoverable Fault is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Captureselect::Disable
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\]The INTFLAG.FAULTn flag is set on each new captured value."]
    #[inline(always)]
    pub fn is_capt(&self) -> bool {
        *self == Captureselect::Capt
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\] if the COUNT value is lower than the last stored capture value (CC).The INTFLAG.FAULTn flag is set on each local minimum detection."]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        *self == Captureselect::Captmin
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\] if the COUNT value is higher than the last stored capture value (CC).The INTFLAG.FAULTn flag is set on each local maximun detection."]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        *self == Captureselect::Captmax
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\].The INTFLAG.FAULTn flag is set on each local minimum value detection."]
    #[inline(always)]
    pub fn is_locmin(&self) -> bool {
        *self == Captureselect::Locmin
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\].The INTFLAG.FAULTn flag is set on each local maximun detection."]
    #[inline(always)]
    pub fn is_locmax(&self) -> bool {
        *self == Captureselect::Locmax
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\].The INTFLAG.FAULTn flag is set on each local maximun or minimum detection."]
    #[inline(always)]
    pub fn is_deriv0(&self) -> bool {
        *self == Captureselect::Deriv0
    }
    #[doc = "Capture with the ramp index as the MSB value"]
    #[inline(always)]
    pub fn is_captmark(&self) -> bool {
        *self == Captureselect::Captmark
    }
}
#[doc = "Field `CAPTURE` writer - Recoverable Fault Capture Action"]
pub type CaptureW<'a, REG> = crate::FieldWriter<'a, REG, 3, Captureselect, crate::Safe>;
impl<'a, REG> CaptureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capture on valid recoverable Fault is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Captureselect::Disable)
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\]The INTFLAG.FAULTn flag is set on each new captured value."]
    #[inline(always)]
    pub fn capt(self) -> &'a mut crate::W<REG> {
        self.variant(Captureselect::Capt)
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\] if the COUNT value is lower than the last stored capture value (CC).The INTFLAG.FAULTn flag is set on each local minimum detection."]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut crate::W<REG> {
        self.variant(Captureselect::Captmin)
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\] if the COUNT value is higher than the last stored capture value (CC).The INTFLAG.FAULTn flag is set on each local maximun detection."]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut crate::W<REG> {
        self.variant(Captureselect::Captmax)
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\].The INTFLAG.FAULTn flag is set on each local minimum value detection."]
    #[inline(always)]
    pub fn locmin(self) -> &'a mut crate::W<REG> {
        self.variant(Captureselect::Locmin)
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\].The INTFLAG.FAULTn flag is set on each local maximun detection."]
    #[inline(always)]
    pub fn locmax(self) -> &'a mut crate::W<REG> {
        self.variant(Captureselect::Locmax)
    }
    #[doc = "On the rising edge of a valid recoverable fault, the counter value is captured on the channel selected by CHSEL\\[1:0\\].The INTFLAG.FAULTn flag is set on each local maximun or minimum detection."]
    #[inline(always)]
    pub fn deriv0(self) -> &'a mut crate::W<REG> {
        self.variant(Captureselect::Deriv0)
    }
    #[doc = "Capture with the ramp index as the MSB value"]
    #[inline(always)]
    pub fn captmark(self) -> &'a mut crate::W<REG> {
        self.variant(Captureselect::Captmark)
    }
}
#[doc = "Field `BLANKPRESC` reader - Recoverable Fault Blanking Value Prescaler"]
pub type BlankprescR = crate::BitReader;
#[doc = "Field `BLANKPRESC` writer - Recoverable Fault Blanking Value Prescaler"]
pub type BlankprescW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKVAL` reader - Recoverable Fault Blanking Value"]
pub type BlankvalR = crate::FieldReader;
#[doc = "Field `BLANKVAL` writer - Recoverable Fault Blanking Value"]
pub type BlankvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FILTERVAL` reader - Recoverable Fault Filter Value"]
pub type FiltervalR = crate::FieldReader;
#[doc = "Field `FILTERVAL` writer - Recoverable Fault Filter Value"]
pub type FiltervalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Recoverable Fault Source"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Recoverable Fault Keep"]
    #[inline(always)]
    pub fn keep(&self) -> KeepR {
        KeepR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Recoverable Fault Qualification"]
    #[inline(always)]
    pub fn qual(&self) -> QualR {
        QualR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Recoverable Fault Blanking Operation"]
    #[inline(always)]
    pub fn blank(&self) -> BlankR {
        BlankR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Recoverable Fault Restart"]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Recoverable Fault Halt Operation"]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Recoverable Fault Capture Channel"]
    #[inline(always)]
    pub fn chsel(&self) -> ChselR {
        ChselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Recoverable Fault Capture Action"]
    #[inline(always)]
    pub fn capture(&self) -> CaptureR {
        CaptureR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Recoverable Fault Blanking Value Prescaler"]
    #[inline(always)]
    pub fn blankpresc(&self) -> BlankprescR {
        BlankprescR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Recoverable Fault Blanking Value"]
    #[inline(always)]
    pub fn blankval(&self) -> BlankvalR {
        BlankvalR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Recoverable Fault Filter Value"]
    #[inline(always)]
    pub fn filterval(&self) -> FiltervalR {
        FiltervalR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Recoverable Fault Source"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, FctrlaSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bit 3 - Recoverable Fault Keep"]
    #[inline(always)]
    pub fn keep(&mut self) -> KeepW<'_, FctrlaSpec> {
        KeepW::new(self, 3)
    }
    #[doc = "Bit 4 - Recoverable Fault Qualification"]
    #[inline(always)]
    pub fn qual(&mut self) -> QualW<'_, FctrlaSpec> {
        QualW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Recoverable Fault Blanking Operation"]
    #[inline(always)]
    pub fn blank(&mut self) -> BlankW<'_, FctrlaSpec> {
        BlankW::new(self, 5)
    }
    #[doc = "Bit 7 - Recoverable Fault Restart"]
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<'_, FctrlaSpec> {
        RestartW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Recoverable Fault Halt Operation"]
    #[inline(always)]
    pub fn halt(&mut self) -> HaltW<'_, FctrlaSpec> {
        HaltW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Recoverable Fault Capture Channel"]
    #[inline(always)]
    pub fn chsel(&mut self) -> ChselW<'_, FctrlaSpec> {
        ChselW::new(self, 10)
    }
    #[doc = "Bits 12:14 - Recoverable Fault Capture Action"]
    #[inline(always)]
    pub fn capture(&mut self) -> CaptureW<'_, FctrlaSpec> {
        CaptureW::new(self, 12)
    }
    #[doc = "Bit 15 - Recoverable Fault Blanking Value Prescaler"]
    #[inline(always)]
    pub fn blankpresc(&mut self) -> BlankprescW<'_, FctrlaSpec> {
        BlankprescW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Recoverable Fault Blanking Value"]
    #[inline(always)]
    pub fn blankval(&mut self) -> BlankvalW<'_, FctrlaSpec> {
        BlankvalW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Recoverable Fault Filter Value"]
    #[inline(always)]
    pub fn filterval(&mut self) -> FiltervalW<'_, FctrlaSpec> {
        FiltervalW::new(self, 24)
    }
}
#[doc = "Recoverable Fault Control #\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FctrlaSpec;
impl crate::RegisterSpec for FctrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrla::R`](R) reader structure"]
impl crate::Readable for FctrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`fctrla::W`](W) writer structure"]
impl crate::Writable for FctrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCTRLA to value 0"]
impl crate::Resettable for FctrlaSpec {}

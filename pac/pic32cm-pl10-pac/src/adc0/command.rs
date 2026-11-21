#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<CommandSpec>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<CommandSpec>;
#[doc = "Start Conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startselect {
    #[doc = "0: Stop an ongoing conversion"]
    Stop = 0,
    #[doc = "1: Start a conversion immediately. This bit will be reset to STOP when the conversion is complete, unless Free-Running mode is enabled."]
    Immediate = 1,
    #[doc = "2: Start a conversion when a write to the INPUTCTRL register is performed"]
    Input = 2,
    #[doc = "4: Start a conversion when an event is received by the ADC. This requires EVCTRL.STARTEI to be set to '1'."]
    Event = 4,
}
impl From<Startselect> for u8 {
    #[inline(always)]
    fn from(variant: Startselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startselect {
    type Ux = u8;
}
impl crate::IsEnum for Startselect {}
#[doc = "Field `START` reader - Start Conversion"]
pub type StartR = crate::FieldReader<Startselect>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Startselect> {
        match self.bits {
            0 => Some(Startselect::Stop),
            1 => Some(Startselect::Immediate),
            2 => Some(Startselect::Input),
            4 => Some(Startselect::Event),
            _ => None,
        }
    }
    #[doc = "Stop an ongoing conversion"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Startselect::Stop
    }
    #[doc = "Start a conversion immediately. This bit will be reset to STOP when the conversion is complete, unless Free-Running mode is enabled."]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == Startselect::Immediate
    }
    #[doc = "Start a conversion when a write to the INPUTCTRL register is performed"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Startselect::Input
    }
    #[doc = "Start a conversion when an event is received by the ADC. This requires EVCTRL.STARTEI to be set to '1'."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Startselect::Event
    }
}
#[doc = "Field `START` writer - Start Conversion"]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 3, Startselect>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop an ongoing conversion"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Stop)
    }
    #[doc = "Start a conversion immediately. This bit will be reset to STOP when the conversion is complete, unless Free-Running mode is enabled."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Immediate)
    }
    #[doc = "Start a conversion when a write to the INPUTCTRL register is performed"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Input)
    }
    #[doc = "Start a conversion when an event is received by the ADC. This requires EVCTRL.STARTEI to be set to '1'."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Event)
    }
}
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Single conversion"]
    Single = 1,
    #[doc = "2: Series mode with accumulation, using a separate trigger for each conversion"]
    Series = 2,
    #[doc = "3: Burst mode with accumulation. A single trigger will initiate the number of conversions configured by CTRLD.SAMPNUM in one sequence."]
    Burst = 3,
    #[doc = "7: Accumulator diagnostics mode"]
    Acctest = 7,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::None),
            1 => Some(Modeselect::Single),
            2 => Some(Modeselect::Series),
            3 => Some(Modeselect::Burst),
            7 => Some(Modeselect::Acctest),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Modeselect::None
    }
    #[doc = "Single conversion"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Modeselect::Single
    }
    #[doc = "Series mode with accumulation, using a separate trigger for each conversion"]
    #[inline(always)]
    pub fn is_series(&self) -> bool {
        *self == Modeselect::Series
    }
    #[doc = "Burst mode with accumulation. A single trigger will initiate the number of conversions configured by CTRLD.SAMPNUM in one sequence."]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == Modeselect::Burst
    }
    #[doc = "Accumulator diagnostics mode"]
    #[inline(always)]
    pub fn is_acctest(&self) -> bool {
        *self == Modeselect::Acctest
    }
}
#[doc = "Field `MODE` writer - Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::None)
    }
    #[doc = "Single conversion"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Single)
    }
    #[doc = "Series mode with accumulation, using a separate trigger for each conversion"]
    #[inline(always)]
    pub fn series(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Series)
    }
    #[doc = "Burst mode with accumulation. A single trigger will initiate the number of conversions configured by CTRLD.SAMPNUM in one sequence."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Burst)
    }
    #[doc = "Accumulator diagnostics mode"]
    #[inline(always)]
    pub fn acctest(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Acctest)
    }
}
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DiffR = crate::BitReader;
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Start Conversion"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Start Conversion"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CommandSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CommandSpec> {
        ModeW::new(self, 4)
    }
    #[doc = "Bit 7 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DiffW<'_, CommandSpec> {
        DiffW::new(self, 7)
    }
}
#[doc = "Command\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandSpec;
impl crate::RegisterSpec for CommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for CommandSpec {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for CommandSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for CommandSpec {}

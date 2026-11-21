#[doc = "Register `CHCTRLB` reader"]
pub type R = crate::R<ChctrlbSpec>;
#[doc = "Register `CHCTRLB` writer"]
pub type W = crate::W<ChctrlbSpec>;
#[doc = "Event Input Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evactselect {
    #[doc = "0: No action"]
    Noact = 0,
    #[doc = "1: Transfer and periodic transfer trigger"]
    Trig = 1,
    #[doc = "2: Conditional transfer trigger"]
    Ctrig = 2,
    #[doc = "3: Conditional block transfer"]
    Cblock = 3,
    #[doc = "4: Channel suspend operation"]
    Suspend = 4,
    #[doc = "5: Channel resume operation"]
    Resume = 5,
    #[doc = "6: Skip next block suspend action"]
    Sskip = 6,
}
impl From<Evactselect> for u8 {
    #[inline(always)]
    fn from(variant: Evactselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evactselect {
    type Ux = u8;
}
impl crate::IsEnum for Evactselect {}
#[doc = "Field `EVACT` reader - Event Input Action"]
pub type EvactR = crate::FieldReader<Evactselect>;
impl EvactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Evactselect> {
        match self.bits {
            0 => Some(Evactselect::Noact),
            1 => Some(Evactselect::Trig),
            2 => Some(Evactselect::Ctrig),
            3 => Some(Evactselect::Cblock),
            4 => Some(Evactselect::Suspend),
            5 => Some(Evactselect::Resume),
            6 => Some(Evactselect::Sskip),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == Evactselect::Noact
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Evactselect::Trig
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn is_ctrig(&self) -> bool {
        *self == Evactselect::Ctrig
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn is_cblock(&self) -> bool {
        *self == Evactselect::Cblock
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Evactselect::Suspend
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Evactselect::Resume
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn is_sskip(&self) -> bool {
        *self == Evactselect::Sskip
    }
}
#[doc = "Field `EVACT` writer - Event Input Action"]
pub type EvactW<'a, REG> = crate::FieldWriter<'a, REG, 3, Evactselect>;
impl<'a, REG> EvactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Noact)
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Trig)
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn ctrig(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Ctrig)
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn cblock(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Cblock)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Suspend)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Resume)
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn sskip(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Sskip)
    }
}
#[doc = "Field `EVIE` reader - Channel Event Input Enable"]
pub type EvieR = crate::BitReader;
#[doc = "Field `EVIE` writer - Channel Event Input Enable"]
pub type EvieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOE` reader - Channel Event Output Enable"]
pub type EvoeR = crate::BitReader;
#[doc = "Field `EVOE` writer - Channel Event Output Enable"]
pub type EvoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVL` reader - Channel Arbitration Level"]
pub type LvlR = crate::FieldReader;
#[doc = "Field `LVL` writer - Channel Arbitration Level"]
pub type LvlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsrcselect {
    #[doc = "0: Only software/event triggers"]
    Disable = 0,
}
impl From<Trigsrcselect> for u8 {
    #[inline(always)]
    fn from(variant: Trigsrcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsrcselect {
    type Ux = u8;
}
impl crate::IsEnum for Trigsrcselect {}
#[doc = "Field `TRIGSRC` reader - Trigger Source"]
pub type TrigsrcR = crate::FieldReader<Trigsrcselect>;
impl TrigsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigsrcselect> {
        match self.bits {
            0 => Some(Trigsrcselect::Disable),
            _ => None,
        }
    }
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Trigsrcselect::Disable
    }
}
#[doc = "Field `TRIGSRC` writer - Trigger Source"]
pub type TrigsrcW<'a, REG> = crate::FieldWriter<'a, REG, 6, Trigsrcselect>;
impl<'a, REG> TrigsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Disable)
    }
}
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigactselect {
    #[doc = "0: One trigger required for each block transfer"]
    Block = 0,
    #[doc = "2: One trigger required for each beat transfer"]
    Beat = 2,
    #[doc = "3: One trigger required for each transaction"]
    Transaction = 3,
}
impl From<Trigactselect> for u8 {
    #[inline(always)]
    fn from(variant: Trigactselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigactselect {
    type Ux = u8;
}
impl crate::IsEnum for Trigactselect {}
#[doc = "Field `TRIGACT` reader - Trigger Action"]
pub type TrigactR = crate::FieldReader<Trigactselect>;
impl TrigactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigactselect> {
        match self.bits {
            0 => Some(Trigactselect::Block),
            2 => Some(Trigactselect::Beat),
            3 => Some(Trigactselect::Transaction),
            _ => None,
        }
    }
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == Trigactselect::Block
    }
    #[doc = "One trigger required for each beat transfer"]
    #[inline(always)]
    pub fn is_beat(&self) -> bool {
        *self == Trigactselect::Beat
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == Trigactselect::Transaction
    }
}
#[doc = "Field `TRIGACT` writer - Trigger Action"]
pub type TrigactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigactselect>;
impl<'a, REG> TrigactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Block)
    }
    #[doc = "One trigger required for each beat transfer"]
    #[inline(always)]
    pub fn beat(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Beat)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Transaction)
    }
}
#[doc = "Software Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdselect {
    #[doc = "0: No action"]
    Noact = 0,
    #[doc = "1: Channel suspend operation"]
    Suspend = 1,
    #[doc = "2: Channel resume operation"]
    Resume = 2,
}
impl From<Cmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmdselect {}
#[doc = "Field `CMD` reader - Software Command"]
pub type CmdR = crate::FieldReader<Cmdselect>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdselect> {
        match self.bits {
            0 => Some(Cmdselect::Noact),
            1 => Some(Cmdselect::Suspend),
            2 => Some(Cmdselect::Resume),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == Cmdselect::Noact
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Cmdselect::Suspend
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Cmdselect::Resume
    }
}
#[doc = "Field `CMD` writer - Software Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmdselect>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Noact)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Suspend)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Resume)
    }
}
impl R {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    pub fn evact(&self) -> EvactR {
        EvactR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&self) -> EvieR {
        EvieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EvoeR {
        EvoeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LvlR {
        LvlR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TrigsrcR {
        TrigsrcR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TrigactR {
        TrigactR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    pub fn evact(&mut self) -> EvactW<'_, ChctrlbSpec> {
        EvactW::new(self, 0)
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&mut self) -> EvieW<'_, ChctrlbSpec> {
        EvieW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&mut self) -> EvoeW<'_, ChctrlbSpec> {
        EvoeW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline(always)]
    pub fn lvl(&mut self) -> LvlW<'_, ChctrlbSpec> {
        LvlW::new(self, 5)
    }
    #[doc = "Bits 8:13 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&mut self) -> TrigsrcW<'_, ChctrlbSpec> {
        TrigsrcW::new(self, 8)
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&mut self) -> TrigactW<'_, ChctrlbSpec> {
        TrigactW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, ChctrlbSpec> {
        CmdW::new(self, 24)
    }
}
#[doc = "Channel Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctrlbSpec;
impl crate::RegisterSpec for ChctrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctrlb::R`](R) reader structure"]
impl crate::Readable for ChctrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`chctrlb::W`](W) writer structure"]
impl crate::Writable for ChctrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTRLB to value 0"]
impl crate::Resettable for ChctrlbSpec {}

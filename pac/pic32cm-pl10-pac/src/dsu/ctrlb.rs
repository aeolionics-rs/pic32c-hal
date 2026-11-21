#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "DMA Trigger 0 Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dccdmalvl0select {
    #[doc = "0: Trigger n rises when DCCn is read and falls when it is written"]
    Empty = 0,
    #[doc = "1: Trigger n rises when DCCn is written and falls when it is read"]
    Full = 1,
}
impl From<Dccdmalvl0select> for bool {
    #[inline(always)]
    fn from(variant: Dccdmalvl0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCCDMALVL0` reader - DMA Trigger 0 Level"]
pub type Dccdmalvl0R = crate::BitReader<Dccdmalvl0select>;
impl Dccdmalvl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dccdmalvl0select {
        match self.bits {
            false => Dccdmalvl0select::Empty,
            true => Dccdmalvl0select::Full,
        }
    }
    #[doc = "Trigger n rises when DCCn is read and falls when it is written"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Dccdmalvl0select::Empty
    }
    #[doc = "Trigger n rises when DCCn is written and falls when it is read"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Dccdmalvl0select::Full
    }
}
#[doc = "Field `DCCDMALVL0` writer - DMA Trigger 0 Level"]
pub type Dccdmalvl0W<'a, REG> = crate::BitWriter<'a, REG, Dccdmalvl0select>;
impl<'a, REG> Dccdmalvl0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger n rises when DCCn is read and falls when it is written"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Dccdmalvl0select::Empty)
    }
    #[doc = "Trigger n rises when DCCn is written and falls when it is read"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Dccdmalvl0select::Full)
    }
}
#[doc = "DMA Trigger 1 Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dccdmalvl1select {
    #[doc = "0: Trigger n rises when DCCn is read and falls when it is written"]
    Empty = 0,
    #[doc = "1: Trigger n rises when DCCn is written and falls when it is read"]
    Full = 1,
}
impl From<Dccdmalvl1select> for bool {
    #[inline(always)]
    fn from(variant: Dccdmalvl1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCCDMALVL1` reader - DMA Trigger 1 Level"]
pub type Dccdmalvl1R = crate::BitReader<Dccdmalvl1select>;
impl Dccdmalvl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dccdmalvl1select {
        match self.bits {
            false => Dccdmalvl1select::Empty,
            true => Dccdmalvl1select::Full,
        }
    }
    #[doc = "Trigger n rises when DCCn is read and falls when it is written"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Dccdmalvl1select::Empty
    }
    #[doc = "Trigger n rises when DCCn is written and falls when it is read"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Dccdmalvl1select::Full
    }
}
#[doc = "Field `DCCDMALVL1` writer - DMA Trigger 1 Level"]
pub type Dccdmalvl1W<'a, REG> = crate::BitWriter<'a, REG, Dccdmalvl1select>;
impl<'a, REG> Dccdmalvl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger n rises when DCCn is read and falls when it is written"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Dccdmalvl1select::Empty)
    }
    #[doc = "Trigger n rises when DCCn is written and falls when it is read"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Dccdmalvl1select::Full)
    }
}
#[doc = "DSU QoS Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qosselect {
    #[doc = "0: Background (no sensitive operation)"]
    Disable = 0,
    #[doc = "1: Sensitive bandwidth"]
    Low = 1,
    #[doc = "2: Sensitive latency"]
    Medium = 2,
    #[doc = "3: Critical latency"]
    High = 3,
}
impl From<Qosselect> for u8 {
    #[inline(always)]
    fn from(variant: Qosselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qosselect {
    type Ux = u8;
}
impl crate::IsEnum for Qosselect {}
#[doc = "Field `QOS` reader - DSU QoS Level"]
pub type QosR = crate::FieldReader<Qosselect>;
impl QosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qosselect {
        match self.bits {
            0 => Qosselect::Disable,
            1 => Qosselect::Low,
            2 => Qosselect::Medium,
            3 => Qosselect::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Qosselect::Disable
    }
    #[doc = "Sensitive bandwidth"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Qosselect::Low
    }
    #[doc = "Sensitive latency"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == Qosselect::Medium
    }
    #[doc = "Critical latency"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Qosselect::High
    }
}
#[doc = "Field `QOS` writer - DSU QoS Level"]
pub type QosW<'a, REG> = crate::FieldWriter<'a, REG, 2, Qosselect, crate::Safe>;
impl<'a, REG> QosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Qosselect::Disable)
    }
    #[doc = "Sensitive bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Qosselect::Low)
    }
    #[doc = "Sensitive latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(Qosselect::Medium)
    }
    #[doc = "Critical latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Qosselect::High)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Trigger 0 Level"]
    #[inline(always)]
    pub fn dccdmalvl0(&self) -> Dccdmalvl0R {
        Dccdmalvl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Trigger 1 Level"]
    #[inline(always)]
    pub fn dccdmalvl1(&self) -> Dccdmalvl1R {
        Dccdmalvl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DSU QoS Level"]
    #[inline(always)]
    pub fn qos(&self) -> QosR {
        QosR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Trigger 0 Level"]
    #[inline(always)]
    pub fn dccdmalvl0(&mut self) -> Dccdmalvl0W<'_, CtrlbSpec> {
        Dccdmalvl0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Trigger 1 Level"]
    #[inline(always)]
    pub fn dccdmalvl1(&mut self) -> Dccdmalvl1W<'_, CtrlbSpec> {
        Dccdmalvl1W::new(self, 1)
    }
    #[doc = "Bits 2:3 - DSU QoS Level"]
    #[inline(always)]
    pub fn qos(&mut self) -> QosW<'_, CtrlbSpec> {
        QosW::new(self, 2)
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRLB to value 0x04"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u32 = 0x04;
}

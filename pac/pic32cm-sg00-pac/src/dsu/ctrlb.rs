#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "DMA Trigger n Level\n\nValue on reset: 0"]
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
#[doc = "Field `DCCDMALVL0` reader - DMA Trigger n Level"]
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
#[doc = "Field `DCCDMALVL0` writer - DMA Trigger n Level"]
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
#[doc = "DMA Trigger n Level\n\nValue on reset: 0"]
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
#[doc = "Field `DCCDMALVL1` reader - DMA Trigger n Level"]
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
#[doc = "Field `DCCDMALVL1` writer - DMA Trigger n Level"]
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
#[doc = "Field `MISC` reader - Device Specific Configuration Register"]
pub type MiscR = crate::FieldReader<u32>;
#[doc = "Field `MISC` writer - Device Specific Configuration Register"]
pub type MiscW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - DMA Trigger n Level"]
    #[inline(always)]
    pub fn dccdmalvl0(&self) -> Dccdmalvl0R {
        Dccdmalvl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Trigger n Level"]
    #[inline(always)]
    pub fn dccdmalvl1(&self) -> Dccdmalvl1R {
        Dccdmalvl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Device Specific Configuration Register"]
    #[inline(always)]
    pub fn misc(&self) -> MiscR {
        MiscR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Trigger n Level"]
    #[inline(always)]
    pub fn dccdmalvl0(&mut self) -> Dccdmalvl0W<'_, CtrlbSpec> {
        Dccdmalvl0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Trigger n Level"]
    #[inline(always)]
    pub fn dccdmalvl1(&mut self) -> Dccdmalvl1W<'_, CtrlbSpec> {
        Dccdmalvl1W::new(self, 1)
    }
    #[doc = "Bits 8:31 - Device Specific Configuration Register"]
    #[inline(always)]
    pub fn misc(&mut self) -> MiscW<'_, CtrlbSpec> {
        MiscW::new(self, 8)
    }
}
#[doc = "Control B REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

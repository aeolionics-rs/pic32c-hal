#[doc = "Register `SEQCTRL` reader"]
pub type R = crate::R<SeqctrlSpec>;
#[doc = "Register `SEQCTRL` writer"]
pub type W = crate::W<SeqctrlSpec>;
#[doc = "Sequential Selection 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Seqsel0select {
    #[doc = "0: Sequential logic is disabled"]
    Disable = 0,
    #[doc = "1: D flip-flop"]
    Dff = 1,
    #[doc = "2: JK flip-flop"]
    Jk = 2,
    #[doc = "3: D latch"]
    Latch = 3,
    #[doc = "4: RS latch"]
    Rs = 4,
}
impl From<Seqsel0select> for u8 {
    #[inline(always)]
    fn from(variant: Seqsel0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Seqsel0select {
    type Ux = u8;
}
impl crate::IsEnum for Seqsel0select {}
#[doc = "Field `SEQSEL0` reader - Sequential Selection 0"]
pub type Seqsel0R = crate::FieldReader<Seqsel0select>;
impl Seqsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Seqsel0select> {
        match self.bits {
            0 => Some(Seqsel0select::Disable),
            1 => Some(Seqsel0select::Dff),
            2 => Some(Seqsel0select::Jk),
            3 => Some(Seqsel0select::Latch),
            4 => Some(Seqsel0select::Rs),
            _ => None,
        }
    }
    #[doc = "Sequential logic is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Seqsel0select::Disable
    }
    #[doc = "D flip-flop"]
    #[inline(always)]
    pub fn is_dff(&self) -> bool {
        *self == Seqsel0select::Dff
    }
    #[doc = "JK flip-flop"]
    #[inline(always)]
    pub fn is_jk(&self) -> bool {
        *self == Seqsel0select::Jk
    }
    #[doc = "D latch"]
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == Seqsel0select::Latch
    }
    #[doc = "RS latch"]
    #[inline(always)]
    pub fn is_rs(&self) -> bool {
        *self == Seqsel0select::Rs
    }
}
#[doc = "Field `SEQSEL0` writer - Sequential Selection 0"]
pub type Seqsel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Seqsel0select>;
impl<'a, REG> Seqsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sequential logic is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Seqsel0select::Disable)
    }
    #[doc = "D flip-flop"]
    #[inline(always)]
    pub fn dff(self) -> &'a mut crate::W<REG> {
        self.variant(Seqsel0select::Dff)
    }
    #[doc = "JK flip-flop"]
    #[inline(always)]
    pub fn jk(self) -> &'a mut crate::W<REG> {
        self.variant(Seqsel0select::Jk)
    }
    #[doc = "D latch"]
    #[inline(always)]
    pub fn latch(self) -> &'a mut crate::W<REG> {
        self.variant(Seqsel0select::Latch)
    }
    #[doc = "RS latch"]
    #[inline(always)]
    pub fn rs(self) -> &'a mut crate::W<REG> {
        self.variant(Seqsel0select::Rs)
    }
}
#[doc = "Sequential Selection 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Seqsel1select {
    #[doc = "0: Sequential logic is disabled"]
    Disable = 0,
    #[doc = "1: D flip-flop"]
    Dff = 1,
    #[doc = "2: JK flip-flop"]
    Jk = 2,
    #[doc = "3: D latch"]
    Latch = 3,
    #[doc = "4: RS latch"]
    Rs = 4,
}
impl From<Seqsel1select> for u8 {
    #[inline(always)]
    fn from(variant: Seqsel1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Seqsel1select {
    type Ux = u8;
}
impl crate::IsEnum for Seqsel1select {}
#[doc = "Field `SEQSEL1` reader - Sequential Selection 1"]
pub type Seqsel1R = crate::FieldReader<Seqsel1select>;
impl Seqsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Seqsel1select> {
        match self.bits {
            0 => Some(Seqsel1select::Disable),
            1 => Some(Seqsel1select::Dff),
            2 => Some(Seqsel1select::Jk),
            3 => Some(Seqsel1select::Latch),
            4 => Some(Seqsel1select::Rs),
            _ => None,
        }
    }
    #[doc = "Sequential logic is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Seqsel1select::Disable
    }
    #[doc = "D flip-flop"]
    #[inline(always)]
    pub fn is_dff(&self) -> bool {
        *self == Seqsel1select::Dff
    }
    #[doc = "JK flip-flop"]
    #[inline(always)]
    pub fn is_jk(&self) -> bool {
        *self == Seqsel1select::Jk
    }
    #[doc = "D latch"]
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == Seqsel1select::Latch
    }
    #[doc = "RS latch"]
    #[inline(always)]
    pub fn is_rs(&self) -> bool {
        *self == Seqsel1select::Rs
    }
}
#[doc = "Field `SEQSEL1` writer - Sequential Selection 1"]
pub type Seqsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Seqsel1select>;
impl<'a, REG> Seqsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sequential logic is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Seqsel1select::Disable)
    }
    #[doc = "D flip-flop"]
    #[inline(always)]
    pub fn dff(self) -> &'a mut crate::W<REG> {
        self.variant(Seqsel1select::Dff)
    }
    #[doc = "JK flip-flop"]
    #[inline(always)]
    pub fn jk(self) -> &'a mut crate::W<REG> {
        self.variant(Seqsel1select::Jk)
    }
    #[doc = "D latch"]
    #[inline(always)]
    pub fn latch(self) -> &'a mut crate::W<REG> {
        self.variant(Seqsel1select::Latch)
    }
    #[doc = "RS latch"]
    #[inline(always)]
    pub fn rs(self) -> &'a mut crate::W<REG> {
        self.variant(Seqsel1select::Rs)
    }
}
impl R {
    #[doc = "Bits 0:3 - Sequential Selection 0"]
    #[inline(always)]
    pub fn seqsel0(&self) -> Seqsel0R {
        Seqsel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Sequential Selection 1"]
    #[inline(always)]
    pub fn seqsel1(&self) -> Seqsel1R {
        Seqsel1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sequential Selection 0"]
    #[inline(always)]
    pub fn seqsel0(&mut self) -> Seqsel0W<'_, SeqctrlSpec> {
        Seqsel0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Sequential Selection 1"]
    #[inline(always)]
    pub fn seqsel1(&mut self) -> Seqsel1W<'_, SeqctrlSpec> {
        Seqsel1W::new(self, 8)
    }
}
#[doc = "Sequential Control\n\nYou can [`read`](crate::Reg::read) this register and get [`seqctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqctrlSpec;
impl crate::RegisterSpec for SeqctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqctrl::R`](R) reader structure"]
impl crate::Readable for SeqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`seqctrl::W`](W) writer structure"]
impl crate::Writable for SeqctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQCTRL to value 0"]
impl crate::Resettable for SeqctrlSpec {}

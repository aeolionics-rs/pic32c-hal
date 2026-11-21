#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "CPUn Debug R/W\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgrw0select {
    #[doc = "0: Allow debugger accesses through CPUn to carry the debug attribute (i.e debugger reads are non-destructive)."]
    Dbgrw0_0 = 0,
    #[doc = "1: Debugger accesses through CPUn do not carry the debug attribute (i.e debugger reads are destructive)."]
    Dbgrw0_1 = 1,
}
impl From<Dbgrw0select> for bool {
    #[inline(always)]
    fn from(variant: Dbgrw0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGRW0` reader - CPUn Debug R/W"]
pub type Dbgrw0R = crate::BitReader<Dbgrw0select>;
impl Dbgrw0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgrw0select {
        match self.bits {
            false => Dbgrw0select::Dbgrw0_0,
            true => Dbgrw0select::Dbgrw0_1,
        }
    }
    #[doc = "Allow debugger accesses through CPUn to carry the debug attribute (i.e debugger reads are non-destructive)."]
    #[inline(always)]
    pub fn is_dbgrw0_0(&self) -> bool {
        *self == Dbgrw0select::Dbgrw0_0
    }
    #[doc = "Debugger accesses through CPUn do not carry the debug attribute (i.e debugger reads are destructive)."]
    #[inline(always)]
    pub fn is_dbgrw0_1(&self) -> bool {
        *self == Dbgrw0select::Dbgrw0_1
    }
}
#[doc = "Field `DBGRW0` writer - CPUn Debug R/W"]
pub type Dbgrw0W<'a, REG> = crate::BitWriter<'a, REG, Dbgrw0select>;
impl<'a, REG> Dbgrw0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow debugger accesses through CPUn to carry the debug attribute (i.e debugger reads are non-destructive)."]
    #[inline(always)]
    pub fn dbgrw0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgrw0select::Dbgrw0_0)
    }
    #[doc = "Debugger accesses through CPUn do not carry the debug attribute (i.e debugger reads are destructive)."]
    #[inline(always)]
    pub fn dbgrw0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgrw0select::Dbgrw0_1)
    }
}
impl R {
    #[doc = "Bit 0 - CPUn Debug R/W"]
    #[inline(always)]
    pub fn dbgrw0(&self) -> Dbgrw0R {
        Dbgrw0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPUn Debug R/W"]
    #[inline(always)]
    pub fn dbgrw0(&mut self) -> Dbgrw0W<'_, CtrlcSpec> {
        Dbgrw0W::new(self, 0)
    }
}
#[doc = "Control C REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {}

#[doc = "Register `SLEEPCFG` reader"]
pub type R = crate::R<SleepcfgSpec>;
#[doc = "Register `SLEEPCFG` writer"]
pub type W = crate::W<SleepcfgSpec>;
#[doc = "Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sleepmodeselect {
    #[doc = "2: Idle sleep mode enabled"]
    Idle = 2,
    #[doc = "4: Standby sleep mode enabled"]
    Standby = 4,
}
impl From<Sleepmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Sleepmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sleepmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Sleepmodeselect {}
#[doc = "Field `SLEEPMODE` reader - Sleep Mode"]
pub type SleepmodeR = crate::FieldReader<Sleepmodeselect>;
impl SleepmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sleepmodeselect> {
        match self.bits {
            2 => Some(Sleepmodeselect::Idle),
            4 => Some(Sleepmodeselect::Standby),
            _ => None,
        }
    }
    #[doc = "Idle sleep mode enabled"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Sleepmodeselect::Idle
    }
    #[doc = "Standby sleep mode enabled"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == Sleepmodeselect::Standby
    }
}
#[doc = "Field `SLEEPMODE` writer - Sleep Mode"]
pub type SleepmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sleepmodeselect>;
impl<'a, REG> SleepmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Idle sleep mode enabled"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepmodeselect::Idle)
    }
    #[doc = "Standby sleep mode enabled"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepmodeselect::Standby)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline(always)]
    pub fn sleepmode(&self) -> SleepmodeR {
        SleepmodeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline(always)]
    pub fn sleepmode(&mut self) -> SleepmodeW<'_, SleepcfgSpec> {
        SleepmodeW::new(self, 0)
    }
}
#[doc = "Sleep Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sleepcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleepcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepcfgSpec;
impl crate::RegisterSpec for SleepcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleepcfg::R`](R) reader structure"]
impl crate::Readable for SleepcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sleepcfg::W`](W) writer structure"]
impl crate::Writable for SleepcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLEEPCFG to value 0"]
impl crate::Resettable for SleepcfgSpec {}

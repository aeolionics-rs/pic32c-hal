#[doc = "Register `CTRL_PWR` reader"]
pub type R = crate::R<CtrlPwrSpec>;
#[doc = "Register `CTRL_PWR` writer"]
pub type W = crate::W<CtrlPwrSpec>;
#[doc = "SRAM Power Switch Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramPwrswselect {
    #[doc = "0: SRAM register power is disabled ( puf_req_power_sram driven to 0)"]
    SramPwrsw0 = 0,
    #[doc = "1: SRAM register power is enabled ( puf_req_power_sram driven to 1)"]
    SramPwrsw1 = 1,
}
impl From<SramPwrswselect> for bool {
    #[inline(always)]
    fn from(variant: SramPwrswselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PWRSW` reader - SRAM Power Switch Enable Bit"]
pub type SramPwrswR = crate::BitReader<SramPwrswselect>;
impl SramPwrswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramPwrswselect {
        match self.bits {
            false => SramPwrswselect::SramPwrsw0,
            true => SramPwrswselect::SramPwrsw1,
        }
    }
    #[doc = "SRAM register power is disabled ( puf_req_power_sram driven to 0)"]
    #[inline(always)]
    pub fn is_sram_pwrsw_0(&self) -> bool {
        *self == SramPwrswselect::SramPwrsw0
    }
    #[doc = "SRAM register power is enabled ( puf_req_power_sram driven to 1)"]
    #[inline(always)]
    pub fn is_sram_pwrsw_1(&self) -> bool {
        *self == SramPwrswselect::SramPwrsw1
    }
}
#[doc = "Field `SRAM_PWRSW` writer - SRAM Power Switch Enable Bit"]
pub type SramPwrswW<'a, REG> = crate::BitWriter<'a, REG, SramPwrswselect>;
impl<'a, REG> SramPwrswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM register power is disabled ( puf_req_power_sram driven to 0)"]
    #[inline(always)]
    pub fn sram_pwrsw_0(self) -> &'a mut crate::W<REG> {
        self.variant(SramPwrswselect::SramPwrsw0)
    }
    #[doc = "SRAM register power is enabled ( puf_req_power_sram driven to 1)"]
    #[inline(always)]
    pub fn sram_pwrsw_1(self) -> &'a mut crate::W<REG> {
        self.variant(SramPwrswselect::SramPwrsw1)
    }
}
impl R {
    #[doc = "Bit 0 - SRAM Power Switch Enable Bit"]
    #[inline(always)]
    pub fn sram_pwrsw(&self) -> SramPwrswR {
        SramPwrswR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM Power Switch Enable Bit"]
    #[inline(always)]
    pub fn sram_pwrsw(&mut self) -> SramPwrswW<'_, CtrlPwrSpec> {
        SramPwrswW::new(self, 0)
    }
}
#[doc = "Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_pwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_pwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlPwrSpec;
impl crate::RegisterSpec for CtrlPwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_pwr::R`](R) reader structure"]
impl crate::Readable for CtrlPwrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_pwr::W`](W) writer structure"]
impl crate::Writable for CtrlPwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL_PWR to value 0"]
impl crate::Resettable for CtrlPwrSpec {}

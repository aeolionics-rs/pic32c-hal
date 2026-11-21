#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `LVDET` reader - Low Voltage Detector Interrupt. Set to one if VDDIO crosses the treshold voltage in the good direction according to LVD.DIR."]
pub type LvdetR = crate::BitReader;
#[doc = "Field `LVDET` writer - Low Voltage Detector Interrupt. Set to one if VDDIO crosses the treshold voltage in the good direction according to LVD.DIR."]
pub type LvdetW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LVDRDY` reader - Low Voltage Detector Ready Interrupt. Set to one if LVD is ready to operate."]
pub type LvdrdyR = crate::BitReader;
#[doc = "Field `LVDRDY` writer - Low Voltage Detector Ready Interrupt. Set to one if LVD is ready to operate."]
pub type LvdrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ULDORDY` reader - User LDO Regulator Ready Interrupt. Set to one if ULDO is ready meaning that output voltage is correct."]
pub type UldordyR = crate::BitReader;
#[doc = "Field `ULDORDY` writer - User LDO Regulator Ready Interrupt. Set to one if ULDO is ready meaning that output voltage is correct."]
pub type UldordyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ULDOOVHEAT` reader - User LDO Regulator OverHeat Interrupt. Is set to one if overheat condition is detected."]
pub type UldoovheatR = crate::BitReader;
#[doc = "Field `ULDOOVHEAT` writer - User LDO Regulator OverHeat Interrupt. Is set to one if overheat condition is detected."]
pub type UldoovheatW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BORVDDUSB` reader - BORVDDUSB Interrupt. Set to one if VDDUSB issue is detected."]
pub type BorvddusbR = crate::BitReader;
#[doc = "Field `BORVDDUSB` writer - BORVDDUSB Interrupt. Set to one if VDDUSB issue is detected."]
pub type BorvddusbW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ADDVREGRDY0` reader - Additional Regulator ready 0 Interrupt. Set to one if additionnal regulator is ready meaning that output voltage is correct."]
pub type Addvregrdy0R = crate::BitReader;
#[doc = "Field `ADDVREGRDY0` writer - Additional Regulator ready 0 Interrupt. Set to one if additionnal regulator is ready meaning that output voltage is correct."]
pub type Addvregrdy0W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Voltage Detector Interrupt. Set to one if VDDIO crosses the treshold voltage in the good direction according to LVD.DIR."]
    #[inline(always)]
    pub fn lvdet(&self) -> LvdetR {
        LvdetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Voltage Detector Ready Interrupt. Set to one if LVD is ready to operate."]
    #[inline(always)]
    pub fn lvdrdy(&self) -> LvdrdyR {
        LvdrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User LDO Regulator Ready Interrupt. Set to one if ULDO is ready meaning that output voltage is correct."]
    #[inline(always)]
    pub fn uldordy(&self) -> UldordyR {
        UldordyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - User LDO Regulator OverHeat Interrupt. Is set to one if overheat condition is detected."]
    #[inline(always)]
    pub fn uldoovheat(&self) -> UldoovheatR {
        UldoovheatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - BORVDDUSB Interrupt. Set to one if VDDUSB issue is detected."]
    #[inline(always)]
    pub fn borvddusb(&self) -> BorvddusbR {
        BorvddusbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Additional Regulator ready 0 Interrupt. Set to one if additionnal regulator is ready meaning that output voltage is correct."]
    #[inline(always)]
    pub fn addvregrdy0(&self) -> Addvregrdy0R {
        Addvregrdy0R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Voltage Detector Interrupt. Set to one if VDDIO crosses the treshold voltage in the good direction according to LVD.DIR."]
    #[inline(always)]
    pub fn lvdet(&mut self) -> LvdetW<'_, IntflagSpec> {
        LvdetW::new(self, 0)
    }
    #[doc = "Bit 1 - Low Voltage Detector Ready Interrupt. Set to one if LVD is ready to operate."]
    #[inline(always)]
    pub fn lvdrdy(&mut self) -> LvdrdyW<'_, IntflagSpec> {
        LvdrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - User LDO Regulator Ready Interrupt. Set to one if ULDO is ready meaning that output voltage is correct."]
    #[inline(always)]
    pub fn uldordy(&mut self) -> UldordyW<'_, IntflagSpec> {
        UldordyW::new(self, 2)
    }
    #[doc = "Bit 3 - User LDO Regulator OverHeat Interrupt. Is set to one if overheat condition is detected."]
    #[inline(always)]
    pub fn uldoovheat(&mut self) -> UldoovheatW<'_, IntflagSpec> {
        UldoovheatW::new(self, 3)
    }
    #[doc = "Bit 5 - BORVDDUSB Interrupt. Set to one if VDDUSB issue is detected."]
    #[inline(always)]
    pub fn borvddusb(&mut self) -> BorvddusbW<'_, IntflagSpec> {
        BorvddusbW::new(self, 5)
    }
    #[doc = "Bit 8 - Additional Regulator ready 0 Interrupt. Set to one if additionnal regulator is ready meaning that output voltage is correct."]
    #[inline(always)]
    pub fn addvregrdy0(&mut self) -> Addvregrdy0W<'_, IntflagSpec> {
        Addvregrdy0W::new(self, 8)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x012f;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}

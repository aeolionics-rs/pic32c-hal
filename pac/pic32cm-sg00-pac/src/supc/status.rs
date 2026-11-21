#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `LVDET` reader - Low Voltage Detector Status."]
pub type LvdetR = crate::BitReader;
#[doc = "Field `LVDRDY` reader - Low Voltage Detector Ready Status"]
pub type LvdrdyR = crate::BitReader;
#[doc = "Field `ULDORDY` reader - User LDO regulator Status. It corresponds to vreg_ready_mv signal from ULDO."]
pub type UldordyR = crate::BitReader;
#[doc = "Field `ULDOOVHEAT` reader - User LDO Regulator OverHeat Status. It corresponds to vreg_overheat_event_mv signal from ULDO."]
pub type UldoovheatR = crate::BitReader;
#[doc = "Field `BORVDDUSB` reader - BORVDDUSB Status. One if VDDUSB is OK. It corresponds to bor_vddusb_n_mv signal of SMOR."]
pub type BorvddusbR = crate::BitReader;
#[doc = "Field `ADDVREGRDY0` reader - Additional Regulator ready 0 Status. One if voltage is OK. It corresponds to vreg_ready_mv signal of additional regulator."]
pub type Addvregrdy0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Low Voltage Detector Status."]
    #[inline(always)]
    pub fn lvdet(&self) -> LvdetR {
        LvdetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Voltage Detector Ready Status"]
    #[inline(always)]
    pub fn lvdrdy(&self) -> LvdrdyR {
        LvdrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User LDO regulator Status. It corresponds to vreg_ready_mv signal from ULDO."]
    #[inline(always)]
    pub fn uldordy(&self) -> UldordyR {
        UldordyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - User LDO Regulator OverHeat Status. It corresponds to vreg_overheat_event_mv signal from ULDO."]
    #[inline(always)]
    pub fn uldoovheat(&self) -> UldoovheatR {
        UldoovheatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - BORVDDUSB Status. One if VDDUSB is OK. It corresponds to bor_vddusb_n_mv signal of SMOR."]
    #[inline(always)]
    pub fn borvddusb(&self) -> BorvddusbR {
        BorvddusbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Additional Regulator ready 0 Status. One if voltage is OK. It corresponds to vreg_ready_mv signal of additional regulator."]
    #[inline(always)]
    pub fn addvregrdy0(&self) -> Addvregrdy0R {
        Addvregrdy0R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Flag status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}

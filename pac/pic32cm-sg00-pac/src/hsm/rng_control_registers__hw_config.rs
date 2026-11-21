#[doc = "Register `RNG_Control_Registers__HwConfig` reader"]
pub type R = crate::R<RngControlRegisters_HwConfigSpec>;
#[doc = "Field `NumbOfRings` reader - Generic g_NumRings value."]
pub type NumbOfRingsR = crate::FieldReader;
#[doc = "Field `AIS31` reader - Generic g_AIS31 value."]
pub type Ais31R = crate::BitReader;
#[doc = "Field `AIS31Full` reader - Generic g_AIS31Full value."]
pub type Ais31fullR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Generic g_NumRings value."]
    #[inline(always)]
    pub fn numb_of_rings(&self) -> NumbOfRingsR {
        NumbOfRingsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Generic g_AIS31 value."]
    #[inline(always)]
    pub fn ais31(&self) -> Ais31R {
        Ais31R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic g_AIS31Full value."]
    #[inline(always)]
    pub fn ais31full(&self) -> Ais31fullR {
        Ais31fullR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Hardware configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__hw_config::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_HwConfigSpec;
impl crate::RegisterSpec for RngControlRegisters_HwConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__hw_config::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_HwConfigSpec {}
#[doc = "`reset()` method sets RNG_Control_Registers__HwConfig to value 0x37"]
impl crate::Resettable for RngControlRegisters_HwConfigSpec {
    const RESET_VALUE: u32 = 0x37;
}

#[doc = "Register `RNG_Control_Registers__Status` reader"]
pub type R = crate::R<RngControlRegisters_StatusSpec>;
#[doc = "Register `RNG_Control_Registers__Status` writer"]
pub type W = crate::W<RngControlRegisters_StatusSpec>;
#[doc = "Field `TestDataBusy` reader - High when data written to TestData register is being processed. (see section 4.7)"]
pub type TestDataBusyR = crate::BitReader;
#[doc = "Field `TestDataBusy` writer - High when data written to TestData register is being processed. (see section 4.7)"]
pub type TestDataBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `State` reader - State of the control FSM: 000: Reset, 001: Startup, 010: Idle (Rings On), 011: Idle (Rings Off), 100: Fill FIFO, 101: Error, 110: Unused, 111: Unused."]
pub type StateR = crate::FieldReader;
#[doc = "Field `State` writer - State of the control FSM: 000: Reset, 001: Startup, 010: Idle (Rings On), 011: Idle (Rings Off), 100: Fill FIFO, 101: Error, 110: Unused, 111: Unused."]
pub type StateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RepFail` reader - NIST-800-90B repetition Count Test interrupt status."]
pub type RepFailR = crate::BitReader;
#[doc = "Field `RepFail` writer - NIST-800-90B repetition Count Test interrupt status."]
pub type RepFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PropFail` reader - NIST-800-90B adaptive Proportion Test (1024-sample window) interrupt status."]
pub type PropFailR = crate::BitReader;
#[doc = "Field `PropFail` writer - NIST-800-90B adaptive Proportion Test (1024-sample window) interrupt status."]
pub type PropFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FullInt` reader - FIFO full status."]
pub type FullIntR = crate::BitReader;
#[doc = "Field `FullInt` writer - FIFO full status."]
pub type FullIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PreInt` reader - AIS31 preliminary noise alarm interrupt status."]
pub type PreIntR = crate::BitReader;
#[doc = "Field `PreInt` writer - AIS31 preliminary noise alarm interrupt status."]
pub type PreIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AlmInt` reader - AIS31 noise alarm interrupt status."]
pub type AlmIntR = crate::BitReader;
#[doc = "Field `AlmInt` writer - AIS31 noise alarm interrupt status."]
pub type AlmIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `StartUpFail` reader - Start-up test failure."]
pub type StartUpFailR = crate::BitReader;
#[doc = "Field `StartUpFail` writer - Start-up test failure."]
pub type StartUpFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FifoAccFail` reader - Set when a FIFO data read is performed while the NDRNG is disabled AND has its FIFO empty (FIFOLevel = 0)."]
pub type FifoAccFailR = crate::BitReader;
#[doc = "Field `FifoAccFail` writer - Set when a FIFO data read is performed while the NDRNG is disabled AND has its FIFO empty (FIFOLevel = 0)."]
pub type FifoAccFailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - High when data written to TestData register is being processed. (see section 4.7)"]
    #[inline(always)]
    pub fn test_data_busy(&self) -> TestDataBusyR {
        TestDataBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - State of the control FSM: 000: Reset, 001: Startup, 010: Idle (Rings On), 011: Idle (Rings Off), 100: Fill FIFO, 101: Error, 110: Unused, 111: Unused."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - NIST-800-90B repetition Count Test interrupt status."]
    #[inline(always)]
    pub fn rep_fail(&self) -> RepFailR {
        RepFailR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NIST-800-90B adaptive Proportion Test (1024-sample window) interrupt status."]
    #[inline(always)]
    pub fn prop_fail(&self) -> PropFailR {
        PropFailR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO full status."]
    #[inline(always)]
    pub fn full_int(&self) -> FullIntR {
        FullIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AIS31 preliminary noise alarm interrupt status."]
    #[inline(always)]
    pub fn pre_int(&self) -> PreIntR {
        PreIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AIS31 noise alarm interrupt status."]
    #[inline(always)]
    pub fn alm_int(&self) -> AlmIntR {
        AlmIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Start-up test failure."]
    #[inline(always)]
    pub fn start_up_fail(&self) -> StartUpFailR {
        StartUpFailR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set when a FIFO data read is performed while the NDRNG is disabled AND has its FIFO empty (FIFOLevel = 0)."]
    #[inline(always)]
    pub fn fifo_acc_fail(&self) -> FifoAccFailR {
        FifoAccFailR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High when data written to TestData register is being processed. (see section 4.7)"]
    #[inline(always)]
    pub fn test_data_busy(&mut self) -> TestDataBusyW<'_, RngControlRegisters_StatusSpec> {
        TestDataBusyW::new(self, 0)
    }
    #[doc = "Bits 1:3 - State of the control FSM: 000: Reset, 001: Startup, 010: Idle (Rings On), 011: Idle (Rings Off), 100: Fill FIFO, 101: Error, 110: Unused, 111: Unused."]
    #[inline(always)]
    pub fn state(&mut self) -> StateW<'_, RngControlRegisters_StatusSpec> {
        StateW::new(self, 1)
    }
    #[doc = "Bit 4 - NIST-800-90B repetition Count Test interrupt status."]
    #[inline(always)]
    pub fn rep_fail(&mut self) -> RepFailW<'_, RngControlRegisters_StatusSpec> {
        RepFailW::new(self, 4)
    }
    #[doc = "Bit 5 - NIST-800-90B adaptive Proportion Test (1024-sample window) interrupt status."]
    #[inline(always)]
    pub fn prop_fail(&mut self) -> PropFailW<'_, RngControlRegisters_StatusSpec> {
        PropFailW::new(self, 5)
    }
    #[doc = "Bit 7 - FIFO full status."]
    #[inline(always)]
    pub fn full_int(&mut self) -> FullIntW<'_, RngControlRegisters_StatusSpec> {
        FullIntW::new(self, 7)
    }
    #[doc = "Bit 8 - AIS31 preliminary noise alarm interrupt status."]
    #[inline(always)]
    pub fn pre_int(&mut self) -> PreIntW<'_, RngControlRegisters_StatusSpec> {
        PreIntW::new(self, 8)
    }
    #[doc = "Bit 9 - AIS31 noise alarm interrupt status."]
    #[inline(always)]
    pub fn alm_int(&mut self) -> AlmIntW<'_, RngControlRegisters_StatusSpec> {
        AlmIntW::new(self, 9)
    }
    #[doc = "Bit 10 - Start-up test failure."]
    #[inline(always)]
    pub fn start_up_fail(&mut self) -> StartUpFailW<'_, RngControlRegisters_StatusSpec> {
        StartUpFailW::new(self, 10)
    }
    #[doc = "Bit 11 - Set when a FIFO data read is performed while the NDRNG is disabled AND has its FIFO empty (FIFOLevel = 0)."]
    #[inline(always)]
    pub fn fifo_acc_fail(&mut self) -> FifoAccFailW<'_, RngControlRegisters_StatusSpec> {
        FifoAccFailW::new(self, 11)
    }
}
#[doc = "Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_StatusSpec;
impl crate::RegisterSpec for RngControlRegisters_StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__status::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__status::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__Status to value 0"]
impl crate::Resettable for RngControlRegisters_StatusSpec {}

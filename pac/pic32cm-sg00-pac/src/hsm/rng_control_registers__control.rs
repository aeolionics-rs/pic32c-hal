#[doc = "Register `RNG_Control_Registers__Control` reader"]
pub type R = crate::R<RngControlRegisters_ControlSpec>;
#[doc = "Register `RNG_Control_Registers__Control` writer"]
pub type W = crate::W<RngControlRegisters_ControlSpec>;
#[doc = "Field `Enable` reader - Enable the NDRNG."]
pub type EnableR = crate::BitReader;
#[doc = "Field `Enable` writer - Enable the NDRNG."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFSREn` reader - Select between the NDRNG with asynchronous free running oscillators (when '0') and the Pseudo-Random generator with synchronous oscillators for simulation purpose (when '1')."]
pub type LfsrenR = crate::BitReader;
#[doc = "Field `LFSREn` writer - Select between the NDRNG with asynchronous free running oscillators (when '0') and the Pseudo-Random generator with synchronous oscillators for simulation purpose (when '1')."]
pub type LfsrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TestEn` reader - Select input for conditioning function and continuous tests: 0: Noise source (normal mode), 1: Test data register (test mode)."]
pub type TestEnR = crate::BitReader;
#[doc = "Field `TestEn` writer - Select input for conditioning function and continuous tests: 0: Noise source (normal mode), 1: Test data register (test mode)."]
pub type TestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CondBypass` reader - 0: the conditioning function is used (normal mode), 1: the conditioning function is bypassed (to observe entropy source directly)."]
pub type CondBypassR = crate::BitReader;
#[doc = "Field `CondBypass` writer - 0: the conditioning function is used (normal mode), 1: the conditioning function is bypassed (to observe entropy source directly)."]
pub type CondBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IntEnRep` reader - Interrupt enable for Repetition Count Test failure."]
pub type IntEnRepR = crate::BitReader;
#[doc = "Field `IntEnRep` writer - Interrupt enable for Repetition Count Test failure."]
pub type IntEnRepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IntEnProp` reader - Interrupt enable for Adaptive Proportion Test failure (1024-sample window)."]
pub type IntEnPropR = crate::BitReader;
#[doc = "Field `IntEnProp` writer - Interrupt enable for Adaptive Proportion Test failure (1024-sample window)."]
pub type IntEnPropW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IntEnFull` reader - Interrupt enable for FIFO full."]
pub type IntEnFullR = crate::BitReader;
#[doc = "Field `IntEnFull` writer - Interrupt enable for FIFO full."]
pub type IntEnFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SoftRst` reader - Software reset: 0: Normal mode, 1: The continuous test, the conditioning function and the FIFO are reset. This bit is not cleared automatically."]
pub type SoftRstR = crate::BitReader;
#[doc = "Field `SoftRst` writer - Software reset: 0: Normal mode, 1: The continuous test, the conditioning function and the FIFO are reset. This bit is not cleared automatically."]
pub type SoftRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IntEnPre` reader - Interrupt enable for AIS31 preliminary noise alarm."]
pub type IntEnPreR = crate::BitReader;
#[doc = "Field `IntEnPre` writer - Interrupt enable for AIS31 preliminary noise alarm."]
pub type IntEnPreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IntEnAlm` reader - Interrupt enable for AIS31 noise alarm."]
pub type IntEnAlmR = crate::BitReader;
#[doc = "Field `IntEnAlm` writer - Interrupt enable for AIS31 noise alarm."]
pub type IntEnAlmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ForceActiveROs` reader - Force oscillators to run when FIFO is full."]
pub type ForceActiveRosR = crate::BitReader;
#[doc = "Field `ForceActiveROs` writer - Force oscillators to run when FIFO is full."]
pub type ForceActiveRosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HealthTestBypass` reader - Bypass NIST tests such that the results of the start-up and online test do not affect the FSM state."]
pub type HealthTestBypassR = crate::BitReader;
#[doc = "Field `HealthTestBypass` writer - Bypass NIST tests such that the results of the start-up and online test do not affect the FSM state."]
pub type HealthTestBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS31Bypass` reader - Bypass AIS31 tests such that the results of the start-up and online tests do not affect the FSM state."]
pub type Ais31bypassR = crate::BitReader;
#[doc = "Field `AIS31Bypass` writer - Bypass AIS31 tests such that the results of the start-up and online tests do not affect the FSM state."]
pub type Ais31bypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HealthTestSel` reader - Select input to health test module: 0: Before conditioning, 1: After conditioning."]
pub type HealthTestSelR = crate::BitReader;
#[doc = "Field `HealthTestSel` writer - Select input to health test module: 0: Before conditioning, 1: After conditioning."]
pub type HealthTestSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS31TestSel` reader - Select input to the AIS31 test module: 0: Before conditioning, 1: After conditioning."]
pub type Ais31testSelR = crate::BitReader;
#[doc = "Field `AIS31TestSel` writer - Select input to the AIS31 test module: 0: Before conditioning, 1: After conditioning."]
pub type Ais31testSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Nb128BitBlocks` reader - Number of 128 bit blocks used in AES-CBCMAC post-processing. This value cannot be zero."]
pub type Nb128bitBlocksR = crate::FieldReader;
#[doc = "Field `Nb128BitBlocks` writer - Number of 128 bit blocks used in AES-CBCMAC post-processing. This value cannot be zero."]
pub type Nb128bitBlocksW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FifoWriteStartUp` reader - Enable write of the samples in the FIFO during start-up."]
pub type FifoWriteStartUpR = crate::BitReader;
#[doc = "Field `FifoWriteStartUp` writer - Enable write of the samples in the FIFO during start-up."]
pub type FifoWriteStartUpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the NDRNG."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select between the NDRNG with asynchronous free running oscillators (when '0') and the Pseudo-Random generator with synchronous oscillators for simulation purpose (when '1')."]
    #[inline(always)]
    pub fn lfsren(&self) -> LfsrenR {
        LfsrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select input for conditioning function and continuous tests: 0: Noise source (normal mode), 1: Test data register (test mode)."]
    #[inline(always)]
    pub fn test_en(&self) -> TestEnR {
        TestEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: the conditioning function is used (normal mode), 1: the conditioning function is bypassed (to observe entropy source directly)."]
    #[inline(always)]
    pub fn cond_bypass(&self) -> CondBypassR {
        CondBypassR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable for Repetition Count Test failure."]
    #[inline(always)]
    pub fn int_en_rep(&self) -> IntEnRepR {
        IntEnRepR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable for Adaptive Proportion Test failure (1024-sample window)."]
    #[inline(always)]
    pub fn int_en_prop(&self) -> IntEnPropR {
        IntEnPropR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for FIFO full."]
    #[inline(always)]
    pub fn int_en_full(&self) -> IntEnFullR {
        IntEnFullR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software reset: 0: Normal mode, 1: The continuous test, the conditioning function and the FIFO are reset. This bit is not cleared automatically."]
    #[inline(always)]
    pub fn soft_rst(&self) -> SoftRstR {
        SoftRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm."]
    #[inline(always)]
    pub fn int_en_pre(&self) -> IntEnPreR {
        IntEnPreR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm."]
    #[inline(always)]
    pub fn int_en_alm(&self) -> IntEnAlmR {
        IntEnAlmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Force oscillators to run when FIFO is full."]
    #[inline(always)]
    pub fn force_active_ros(&self) -> ForceActiveRosR {
        ForceActiveRosR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bypass NIST tests such that the results of the start-up and online test do not affect the FSM state."]
    #[inline(always)]
    pub fn health_test_bypass(&self) -> HealthTestBypassR {
        HealthTestBypassR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bypass AIS31 tests such that the results of the start-up and online tests do not affect the FSM state."]
    #[inline(always)]
    pub fn ais31bypass(&self) -> Ais31bypassR {
        Ais31bypassR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select input to health test module: 0: Before conditioning, 1: After conditioning."]
    #[inline(always)]
    pub fn health_test_sel(&self) -> HealthTestSelR {
        HealthTestSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Select input to the AIS31 test module: 0: Before conditioning, 1: After conditioning."]
    #[inline(always)]
    pub fn ais31test_sel(&self) -> Ais31testSelR {
        Ais31testSelR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Number of 128 bit blocks used in AES-CBCMAC post-processing. This value cannot be zero."]
    #[inline(always)]
    pub fn nb128bit_blocks(&self) -> Nb128bitBlocksR {
        Nb128bitBlocksR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Enable write of the samples in the FIFO during start-up."]
    #[inline(always)]
    pub fn fifo_write_start_up(&self) -> FifoWriteStartUpR {
        FifoWriteStartUpR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the NDRNG."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, RngControlRegisters_ControlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Select between the NDRNG with asynchronous free running oscillators (when '0') and the Pseudo-Random generator with synchronous oscillators for simulation purpose (when '1')."]
    #[inline(always)]
    pub fn lfsren(&mut self) -> LfsrenW<'_, RngControlRegisters_ControlSpec> {
        LfsrenW::new(self, 1)
    }
    #[doc = "Bit 2 - Select input for conditioning function and continuous tests: 0: Noise source (normal mode), 1: Test data register (test mode)."]
    #[inline(always)]
    pub fn test_en(&mut self) -> TestEnW<'_, RngControlRegisters_ControlSpec> {
        TestEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 0: the conditioning function is used (normal mode), 1: the conditioning function is bypassed (to observe entropy source directly)."]
    #[inline(always)]
    pub fn cond_bypass(&mut self) -> CondBypassW<'_, RngControlRegisters_ControlSpec> {
        CondBypassW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable for Repetition Count Test failure."]
    #[inline(always)]
    pub fn int_en_rep(&mut self) -> IntEnRepW<'_, RngControlRegisters_ControlSpec> {
        IntEnRepW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt enable for Adaptive Proportion Test failure (1024-sample window)."]
    #[inline(always)]
    pub fn int_en_prop(&mut self) -> IntEnPropW<'_, RngControlRegisters_ControlSpec> {
        IntEnPropW::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt enable for FIFO full."]
    #[inline(always)]
    pub fn int_en_full(&mut self) -> IntEnFullW<'_, RngControlRegisters_ControlSpec> {
        IntEnFullW::new(self, 7)
    }
    #[doc = "Bit 8 - Software reset: 0: Normal mode, 1: The continuous test, the conditioning function and the FIFO are reset. This bit is not cleared automatically."]
    #[inline(always)]
    pub fn soft_rst(&mut self) -> SoftRstW<'_, RngControlRegisters_ControlSpec> {
        SoftRstW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm."]
    #[inline(always)]
    pub fn int_en_pre(&mut self) -> IntEnPreW<'_, RngControlRegisters_ControlSpec> {
        IntEnPreW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm."]
    #[inline(always)]
    pub fn int_en_alm(&mut self) -> IntEnAlmW<'_, RngControlRegisters_ControlSpec> {
        IntEnAlmW::new(self, 10)
    }
    #[doc = "Bit 11 - Force oscillators to run when FIFO is full."]
    #[inline(always)]
    pub fn force_active_ros(&mut self) -> ForceActiveRosW<'_, RngControlRegisters_ControlSpec> {
        ForceActiveRosW::new(self, 11)
    }
    #[doc = "Bit 12 - Bypass NIST tests such that the results of the start-up and online test do not affect the FSM state."]
    #[inline(always)]
    pub fn health_test_bypass(&mut self) -> HealthTestBypassW<'_, RngControlRegisters_ControlSpec> {
        HealthTestBypassW::new(self, 12)
    }
    #[doc = "Bit 13 - Bypass AIS31 tests such that the results of the start-up and online tests do not affect the FSM state."]
    #[inline(always)]
    pub fn ais31bypass(&mut self) -> Ais31bypassW<'_, RngControlRegisters_ControlSpec> {
        Ais31bypassW::new(self, 13)
    }
    #[doc = "Bit 14 - Select input to health test module: 0: Before conditioning, 1: After conditioning."]
    #[inline(always)]
    pub fn health_test_sel(&mut self) -> HealthTestSelW<'_, RngControlRegisters_ControlSpec> {
        HealthTestSelW::new(self, 14)
    }
    #[doc = "Bit 15 - Select input to the AIS31 test module: 0: Before conditioning, 1: After conditioning."]
    #[inline(always)]
    pub fn ais31test_sel(&mut self) -> Ais31testSelW<'_, RngControlRegisters_ControlSpec> {
        Ais31testSelW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Number of 128 bit blocks used in AES-CBCMAC post-processing. This value cannot be zero."]
    #[inline(always)]
    pub fn nb128bit_blocks(&mut self) -> Nb128bitBlocksW<'_, RngControlRegisters_ControlSpec> {
        Nb128bitBlocksW::new(self, 16)
    }
    #[doc = "Bit 20 - Enable write of the samples in the FIFO during start-up."]
    #[inline(always)]
    pub fn fifo_write_start_up(&mut self) -> FifoWriteStartUpW<'_, RngControlRegisters_ControlSpec> {
        FifoWriteStartUpW::new(self, 20)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngControlRegisters_ControlSpec;
impl crate::RegisterSpec for RngControlRegisters_ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_control_registers__control::R`](R) reader structure"]
impl crate::Readable for RngControlRegisters_ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_control_registers__control::W`](W) writer structure"]
impl crate::Writable for RngControlRegisters_ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_Control_Registers__Control to value 0x0004_0000"]
impl crate::Resettable for RngControlRegisters_ControlSpec {
    const RESET_VALUE: u32 = 0x0004_0000;
}

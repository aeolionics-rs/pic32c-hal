#[doc = "Register `CTRLD` reader"]
pub type R = crate::R<CtrldSpec>;
#[doc = "Register `CTRLD` writer"]
pub type W = crate::W<CtrldSpec>;
#[doc = "Sample Accumulation Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sampnumselect {
    #[doc = "0: No accumulation, single sample per conversion result"]
    None = 0,
    #[doc = "1: 2 samples accumulated"]
    Acc2 = 1,
    #[doc = "2: 4 samples accumulated"]
    Acc4 = 2,
    #[doc = "3: 8 samples accumulated"]
    Acc8 = 3,
    #[doc = "4: 16 samples accumulated"]
    Acc16 = 4,
    #[doc = "5: 32 samples accumulated"]
    Acc32 = 5,
    #[doc = "6: 64 samples accumulated"]
    Acc64 = 6,
    #[doc = "7: 128 samples accumulated"]
    Acc128 = 7,
    #[doc = "8: 256 samples accumulated"]
    Acc256 = 8,
    #[doc = "9: 512 samples accumulated"]
    Acc512 = 9,
    #[doc = "10: 1024 samples accumulated"]
    Acc1024 = 10,
}
impl From<Sampnumselect> for u8 {
    #[inline(always)]
    fn from(variant: Sampnumselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sampnumselect {
    type Ux = u8;
}
impl crate::IsEnum for Sampnumselect {}
#[doc = "Field `SAMPNUM` reader - Sample Accumulation Number Select"]
pub type SampnumR = crate::FieldReader<Sampnumselect>;
impl SampnumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sampnumselect> {
        match self.bits {
            0 => Some(Sampnumselect::None),
            1 => Some(Sampnumselect::Acc2),
            2 => Some(Sampnumselect::Acc4),
            3 => Some(Sampnumselect::Acc8),
            4 => Some(Sampnumselect::Acc16),
            5 => Some(Sampnumselect::Acc32),
            6 => Some(Sampnumselect::Acc64),
            7 => Some(Sampnumselect::Acc128),
            8 => Some(Sampnumselect::Acc256),
            9 => Some(Sampnumselect::Acc512),
            10 => Some(Sampnumselect::Acc1024),
            _ => None,
        }
    }
    #[doc = "No accumulation, single sample per conversion result"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sampnumselect::None
    }
    #[doc = "2 samples accumulated"]
    #[inline(always)]
    pub fn is_acc2(&self) -> bool {
        *self == Sampnumselect::Acc2
    }
    #[doc = "4 samples accumulated"]
    #[inline(always)]
    pub fn is_acc4(&self) -> bool {
        *self == Sampnumselect::Acc4
    }
    #[doc = "8 samples accumulated"]
    #[inline(always)]
    pub fn is_acc8(&self) -> bool {
        *self == Sampnumselect::Acc8
    }
    #[doc = "16 samples accumulated"]
    #[inline(always)]
    pub fn is_acc16(&self) -> bool {
        *self == Sampnumselect::Acc16
    }
    #[doc = "32 samples accumulated"]
    #[inline(always)]
    pub fn is_acc32(&self) -> bool {
        *self == Sampnumselect::Acc32
    }
    #[doc = "64 samples accumulated"]
    #[inline(always)]
    pub fn is_acc64(&self) -> bool {
        *self == Sampnumselect::Acc64
    }
    #[doc = "128 samples accumulated"]
    #[inline(always)]
    pub fn is_acc128(&self) -> bool {
        *self == Sampnumselect::Acc128
    }
    #[doc = "256 samples accumulated"]
    #[inline(always)]
    pub fn is_acc256(&self) -> bool {
        *self == Sampnumselect::Acc256
    }
    #[doc = "512 samples accumulated"]
    #[inline(always)]
    pub fn is_acc512(&self) -> bool {
        *self == Sampnumselect::Acc512
    }
    #[doc = "1024 samples accumulated"]
    #[inline(always)]
    pub fn is_acc1024(&self) -> bool {
        *self == Sampnumselect::Acc1024
    }
}
#[doc = "Field `SAMPNUM` writer - Sample Accumulation Number Select"]
pub type SampnumW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sampnumselect>;
impl<'a, REG> SampnumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No accumulation, single sample per conversion result"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sampnumselect::None)
    }
    #[doc = "2 samples accumulated"]
    #[inline(always)]
    pub fn acc2(self) -> &'a mut crate::W<REG> {
        self.variant(Sampnumselect::Acc2)
    }
    #[doc = "4 samples accumulated"]
    #[inline(always)]
    pub fn acc4(self) -> &'a mut crate::W<REG> {
        self.variant(Sampnumselect::Acc4)
    }
    #[doc = "8 samples accumulated"]
    #[inline(always)]
    pub fn acc8(self) -> &'a mut crate::W<REG> {
        self.variant(Sampnumselect::Acc8)
    }
    #[doc = "16 samples accumulated"]
    #[inline(always)]
    pub fn acc16(self) -> &'a mut crate::W<REG> {
        self.variant(Sampnumselect::Acc16)
    }
    #[doc = "32 samples accumulated"]
    #[inline(always)]
    pub fn acc32(self) -> &'a mut crate::W<REG> {
        self.variant(Sampnumselect::Acc32)
    }
    #[doc = "64 samples accumulated"]
    #[inline(always)]
    pub fn acc64(self) -> &'a mut crate::W<REG> {
        self.variant(Sampnumselect::Acc64)
    }
    #[doc = "128 samples accumulated"]
    #[inline(always)]
    pub fn acc128(self) -> &'a mut crate::W<REG> {
        self.variant(Sampnumselect::Acc128)
    }
    #[doc = "256 samples accumulated"]
    #[inline(always)]
    pub fn acc256(self) -> &'a mut crate::W<REG> {
        self.variant(Sampnumselect::Acc256)
    }
    #[doc = "512 samples accumulated"]
    #[inline(always)]
    pub fn acc512(self) -> &'a mut crate::W<REG> {
        self.variant(Sampnumselect::Acc512)
    }
    #[doc = "1024 samples accumulated"]
    #[inline(always)]
    pub fn acc1024(self) -> &'a mut crate::W<REG> {
        self.variant(Sampnumselect::Acc1024)
    }
}
#[doc = "ADC Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resolutionselect {
    #[doc = "0: 12-bit ADC Result"]
    _12bit = 0,
    #[doc = "1: 13-bit ADC Result"]
    _13bit = 1,
    #[doc = "2: 10-bit ADC Result"]
    _10bit = 2,
    #[doc = "3: 8-bit ADC Result"]
    _8bit = 3,
}
impl From<Resolutionselect> for u8 {
    #[inline(always)]
    fn from(variant: Resolutionselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resolutionselect {
    type Ux = u8;
}
impl crate::IsEnum for Resolutionselect {}
#[doc = "Field `RESOLUTION` reader - ADC Resolution"]
pub type ResolutionR = crate::FieldReader<Resolutionselect>;
impl ResolutionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resolutionselect {
        match self.bits {
            0 => Resolutionselect::_12bit,
            1 => Resolutionselect::_13bit,
            2 => Resolutionselect::_10bit,
            3 => Resolutionselect::_8bit,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit ADC Result"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == Resolutionselect::_12bit
    }
    #[doc = "13-bit ADC Result"]
    #[inline(always)]
    pub fn is_13bit(&self) -> bool {
        *self == Resolutionselect::_13bit
    }
    #[doc = "10-bit ADC Result"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Resolutionselect::_10bit
    }
    #[doc = "8-bit ADC Result"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Resolutionselect::_8bit
    }
}
#[doc = "Field `RESOLUTION` writer - ADC Resolution"]
pub type ResolutionW<'a, REG> = crate::FieldWriter<'a, REG, 2, Resolutionselect, crate::Safe>;
impl<'a, REG> ResolutionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit ADC Result"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resolutionselect::_12bit)
    }
    #[doc = "13-bit ADC Result"]
    #[inline(always)]
    pub fn _13bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resolutionselect::_13bit)
    }
    #[doc = "10-bit ADC Result"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resolutionselect::_10bit)
    }
    #[doc = "8-bit ADC Result"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resolutionselect::_8bit)
    }
}
#[doc = "Result Scaling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scalingselect {
    #[doc = "0: The ADC output of the sample or accumulated result is right adjusted"]
    Normal = 0,
    #[doc = "1: The ADC output is left adjusted"]
    Leftadj = 1,
    #[doc = "2: The accumulated ADC result is averaged and right adjusted"]
    Average = 2,
}
impl From<Scalingselect> for u8 {
    #[inline(always)]
    fn from(variant: Scalingselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scalingselect {
    type Ux = u8;
}
impl crate::IsEnum for Scalingselect {}
#[doc = "Field `SCALING` reader - Result Scaling"]
pub type ScalingR = crate::FieldReader<Scalingselect>;
impl ScalingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Scalingselect> {
        match self.bits {
            0 => Some(Scalingselect::Normal),
            1 => Some(Scalingselect::Leftadj),
            2 => Some(Scalingselect::Average),
            _ => None,
        }
    }
    #[doc = "The ADC output of the sample or accumulated result is right adjusted"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Scalingselect::Normal
    }
    #[doc = "The ADC output is left adjusted"]
    #[inline(always)]
    pub fn is_leftadj(&self) -> bool {
        *self == Scalingselect::Leftadj
    }
    #[doc = "The accumulated ADC result is averaged and right adjusted"]
    #[inline(always)]
    pub fn is_average(&self) -> bool {
        *self == Scalingselect::Average
    }
}
#[doc = "Field `SCALING` writer - Result Scaling"]
pub type ScalingW<'a, REG> = crate::FieldWriter<'a, REG, 2, Scalingselect>;
impl<'a, REG> ScalingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The ADC output of the sample or accumulated result is right adjusted"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Scalingselect::Normal)
    }
    #[doc = "The ADC output is left adjusted"]
    #[inline(always)]
    pub fn leftadj(self) -> &'a mut crate::W<REG> {
        self.variant(Scalingselect::Leftadj)
    }
    #[doc = "The accumulated ADC result is averaged and right adjusted"]
    #[inline(always)]
    pub fn average(self) -> &'a mut crate::W<REG> {
        self.variant(Scalingselect::Average)
    }
}
#[doc = "Field `FREERUN` reader - Free-Running"]
pub type FreerunR = crate::BitReader;
#[doc = "Field `FREERUN` writer - Free-Running"]
pub type FreerunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOPPING` reader - Sign Chopping"]
pub type ChoppingR = crate::BitReader;
#[doc = "Field `CHOPPING` writer - Sign Chopping"]
pub type ChoppingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTER` reader - Low-Pass Filter"]
pub type FilterR = crate::BitReader;
#[doc = "Field `FILTER` writer - Low-Pass Filter"]
pub type FilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPD` reader - Voltage Pump Disable"]
pub type VpdR = crate::BitReader;
#[doc = "Field `VPD` writer - Voltage Pump Disable"]
pub type VpdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Sample Accumulation Number Select"]
    #[inline(always)]
    pub fn sampnum(&self) -> SampnumR {
        SampnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - ADC Resolution"]
    #[inline(always)]
    pub fn resolution(&self) -> ResolutionR {
        ResolutionR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Result Scaling"]
    #[inline(always)]
    pub fn scaling(&self) -> ScalingR {
        ScalingR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Free-Running"]
    #[inline(always)]
    pub fn freerun(&self) -> FreerunR {
        FreerunR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Sign Chopping"]
    #[inline(always)]
    pub fn chopping(&self) -> ChoppingR {
        ChoppingR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Low-Pass Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FilterR {
        FilterR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Voltage Pump Disable"]
    #[inline(always)]
    pub fn vpd(&self) -> VpdR {
        VpdR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample Accumulation Number Select"]
    #[inline(always)]
    pub fn sampnum(&mut self) -> SampnumW<'_, CtrldSpec> {
        SampnumW::new(self, 0)
    }
    #[doc = "Bits 8:9 - ADC Resolution"]
    #[inline(always)]
    pub fn resolution(&mut self) -> ResolutionW<'_, CtrldSpec> {
        ResolutionW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Result Scaling"]
    #[inline(always)]
    pub fn scaling(&mut self) -> ScalingW<'_, CtrldSpec> {
        ScalingW::new(self, 10)
    }
    #[doc = "Bit 12 - Free-Running"]
    #[inline(always)]
    pub fn freerun(&mut self) -> FreerunW<'_, CtrldSpec> {
        FreerunW::new(self, 12)
    }
    #[doc = "Bit 13 - Sign Chopping"]
    #[inline(always)]
    pub fn chopping(&mut self) -> ChoppingW<'_, CtrldSpec> {
        ChoppingW::new(self, 13)
    }
    #[doc = "Bit 14 - Low-Pass Filter"]
    #[inline(always)]
    pub fn filter(&mut self) -> FilterW<'_, CtrldSpec> {
        FilterW::new(self, 14)
    }
    #[doc = "Bit 16 - Voltage Pump Disable"]
    #[inline(always)]
    pub fn vpd(&mut self) -> VpdW<'_, CtrldSpec> {
        VpdW::new(self, 16)
    }
}
#[doc = "Control D\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrldSpec;
impl crate::RegisterSpec for CtrldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrld::R`](R) reader structure"]
impl crate::Readable for CtrldSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrld::W`](W) writer structure"]
impl crate::Writable for CtrldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for CtrldSpec {}

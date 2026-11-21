#[doc = "Register `CRCFXOR` reader"]
pub type R = crate::R<CrcfxorSpec>;
#[doc = "Register `CRCFXOR` writer"]
pub type W = crate::W<CrcfxorSpec>;
#[doc = "CRC Final XOR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Crcfxorselect {
    #[doc = "0: Disable the Final XOR at the associated bit position"]
    Crcfxor0 = 0,
    #[doc = "1: Enable the Final XOR at the associated bit position"]
    Crcfxor1 = 1,
}
impl From<Crcfxorselect> for u32 {
    #[inline(always)]
    fn from(variant: Crcfxorselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crcfxorselect {
    type Ux = u32;
}
impl crate::IsEnum for Crcfxorselect {}
#[doc = "Field `CRCFXOR` reader - CRC Final XOR"]
pub type CrcfxorR = crate::FieldReader<Crcfxorselect>;
impl CrcfxorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Crcfxorselect> {
        match self.bits {
            0 => Some(Crcfxorselect::Crcfxor0),
            1 => Some(Crcfxorselect::Crcfxor1),
            _ => None,
        }
    }
    #[doc = "Disable the Final XOR at the associated bit position"]
    #[inline(always)]
    pub fn is_crcfxor_0(&self) -> bool {
        *self == Crcfxorselect::Crcfxor0
    }
    #[doc = "Enable the Final XOR at the associated bit position"]
    #[inline(always)]
    pub fn is_crcfxor_1(&self) -> bool {
        *self == Crcfxorselect::Crcfxor1
    }
}
#[doc = "Field `CRCFXOR` writer - CRC Final XOR"]
pub type CrcfxorW<'a, REG> = crate::FieldWriter<'a, REG, 32, Crcfxorselect>;
impl<'a, REG> CrcfxorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Disable the Final XOR at the associated bit position"]
    #[inline(always)]
    pub fn crcfxor_0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcfxorselect::Crcfxor0)
    }
    #[doc = "Enable the Final XOR at the associated bit position"]
    #[inline(always)]
    pub fn crcfxor_1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcfxorselect::Crcfxor1)
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Final XOR"]
    #[inline(always)]
    pub fn crcfxor(&self) -> CrcfxorR {
        CrcfxorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Final XOR"]
    #[inline(always)]
    pub fn crcfxor(&mut self) -> CrcfxorW<'_, CrcfxorSpec> {
        CrcfxorW::new(self, 0)
    }
}
#[doc = "CRC Final XOR REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcfxor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcfxor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcfxorSpec;
impl crate::RegisterSpec for CrcfxorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcfxor::R`](R) reader structure"]
impl crate::Readable for CrcfxorSpec {}
#[doc = "`write(|w| ..)` method takes [`crcfxor::W`](W) writer structure"]
impl crate::Writable for CrcfxorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCFXOR to value 0"]
impl crate::Resettable for CrcfxorSpec {}

#[doc = "Register `CRCPOLY` reader"]
pub type R = crate::R<CrcpolySpec>;
#[doc = "Register `CRCPOLY` writer"]
pub type W = crate::W<CrcpolySpec>;
#[doc = "CRC Polynomial Coefficients for LFSR\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Crcpolyselect {
    #[doc = "0: Disable the XOR input to the shift register at the associated bit position"]
    Crcpoly0 = 0,
    #[doc = "1: Enable the XOR input to the shift register at the associated bit position"]
    Crcpoly1 = 1,
}
impl From<Crcpolyselect> for u32 {
    #[inline(always)]
    fn from(variant: Crcpolyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crcpolyselect {
    type Ux = u32;
}
impl crate::IsEnum for Crcpolyselect {}
#[doc = "Field `CRCPOLY` reader - CRC Polynomial Coefficients for LFSR"]
pub type CrcpolyR = crate::FieldReader<Crcpolyselect>;
impl CrcpolyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Crcpolyselect> {
        match self.bits {
            0 => Some(Crcpolyselect::Crcpoly0),
            1 => Some(Crcpolyselect::Crcpoly1),
            _ => None,
        }
    }
    #[doc = "Disable the XOR input to the shift register at the associated bit position"]
    #[inline(always)]
    pub fn is_crcpoly_0(&self) -> bool {
        *self == Crcpolyselect::Crcpoly0
    }
    #[doc = "Enable the XOR input to the shift register at the associated bit position"]
    #[inline(always)]
    pub fn is_crcpoly_1(&self) -> bool {
        *self == Crcpolyselect::Crcpoly1
    }
}
#[doc = "Field `CRCPOLY` writer - CRC Polynomial Coefficients for LFSR"]
pub type CrcpolyW<'a, REG> = crate::FieldWriter<'a, REG, 32, Crcpolyselect>;
impl<'a, REG> CrcpolyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Disable the XOR input to the shift register at the associated bit position"]
    #[inline(always)]
    pub fn crcpoly_0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcpolyselect::Crcpoly0)
    }
    #[doc = "Enable the XOR input to the shift register at the associated bit position"]
    #[inline(always)]
    pub fn crcpoly_1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcpolyselect::Crcpoly1)
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Polynomial Coefficients for LFSR"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CrcpolyR {
        CrcpolyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Polynomial Coefficients for LFSR"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CrcpolyW<'_, CrcpolySpec> {
        CrcpolyW::new(self, 0)
    }
}
#[doc = "CRC Polynomial REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcpolySpec;
impl crate::RegisterSpec for CrcpolySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcpoly::R`](R) reader structure"]
impl crate::Readable for CrcpolySpec {}
#[doc = "`write(|w| ..)` method takes [`crcpoly::W`](W) writer structure"]
impl crate::Writable for CrcpolySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCPOLY to value 0x01"]
impl crate::Resettable for CrcpolySpec {
    const RESET_VALUE: u32 = 0x01;
}

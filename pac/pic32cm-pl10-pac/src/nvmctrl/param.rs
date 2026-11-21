#[doc = "Register `PARAM` reader"]
pub type R = crate::R<ParamSpec>;
#[doc = "Field `NVMP` reader - NVM Parameters"]
pub type NvmpR = crate::FieldReader<u16>;
#[doc = "Page Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pszselect {
    #[doc = "0: 8 bytes"]
    Psz8 = 0,
    #[doc = "1: 16 bytes"]
    Psz16 = 1,
    #[doc = "2: 32 bytes"]
    Psz32 = 2,
    #[doc = "3: 64 bytes"]
    Psz64 = 3,
    #[doc = "4: 128 bytes"]
    Psz128 = 4,
    #[doc = "5: 256 bytes"]
    Psz256 = 5,
    #[doc = "6: 512 bytes"]
    Psz512 = 6,
    #[doc = "7: 1024 bytes"]
    Psz1024 = 7,
}
impl From<Pszselect> for u8 {
    #[inline(always)]
    fn from(variant: Pszselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pszselect {
    type Ux = u8;
}
impl crate::IsEnum for Pszselect {}
#[doc = "Field `PSZ` reader - Page Size"]
pub type PszR = crate::FieldReader<Pszselect>;
impl PszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pszselect {
        match self.bits {
            0 => Pszselect::Psz8,
            1 => Pszselect::Psz16,
            2 => Pszselect::Psz32,
            3 => Pszselect::Psz64,
            4 => Pszselect::Psz128,
            5 => Pszselect::Psz256,
            6 => Pszselect::Psz512,
            7 => Pszselect::Psz1024,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_psz_8(&self) -> bool {
        *self == Pszselect::Psz8
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_psz_16(&self) -> bool {
        *self == Pszselect::Psz16
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_psz_32(&self) -> bool {
        *self == Pszselect::Psz32
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_psz_64(&self) -> bool {
        *self == Pszselect::Psz64
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_psz_128(&self) -> bool {
        *self == Pszselect::Psz128
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_psz_256(&self) -> bool {
        *self == Pszselect::Psz256
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_psz_512(&self) -> bool {
        *self == Pszselect::Psz512
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_psz_1024(&self) -> bool {
        *self == Pszselect::Psz1024
    }
}
#[doc = "Boot Section Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bootprotselect {
    #[doc = "0: 32768 bytes are protected."]
    Size32768bytes = 0,
    #[doc = "1: 16384 bytes are protected."]
    Size16384bytes = 1,
    #[doc = "2: 8192 bytes are protected."]
    Size8192bytes = 2,
    #[doc = "3: 4096 bytes are protected."]
    Size4096bytes = 3,
    #[doc = "4: 2048 bytes are protected."]
    Size2048bytes = 4,
    #[doc = "5: 1024 bytes are protected."]
    Size1024bytes = 5,
    #[doc = "6: 512 bytes are protected."]
    Size512bytes = 6,
    #[doc = "7: 0 bytes are protected."]
    Size0bytes = 7,
}
impl From<Bootprotselect> for u8 {
    #[inline(always)]
    fn from(variant: Bootprotselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bootprotselect {
    type Ux = u8;
}
impl crate::IsEnum for Bootprotselect {}
#[doc = "Field `BOOTPROT` reader - Boot Section Size"]
pub type BootprotR = crate::FieldReader<Bootprotselect>;
impl BootprotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootprotselect {
        match self.bits {
            0 => Bootprotselect::Size32768bytes,
            1 => Bootprotselect::Size16384bytes,
            2 => Bootprotselect::Size8192bytes,
            3 => Bootprotselect::Size4096bytes,
            4 => Bootprotselect::Size2048bytes,
            5 => Bootprotselect::Size1024bytes,
            6 => Bootprotselect::Size512bytes,
            7 => Bootprotselect::Size0bytes,
            _ => unreachable!(),
        }
    }
    #[doc = "32768 bytes are protected."]
    #[inline(always)]
    pub fn is_size_32768bytes(&self) -> bool {
        *self == Bootprotselect::Size32768bytes
    }
    #[doc = "16384 bytes are protected."]
    #[inline(always)]
    pub fn is_size_16384bytes(&self) -> bool {
        *self == Bootprotselect::Size16384bytes
    }
    #[doc = "8192 bytes are protected."]
    #[inline(always)]
    pub fn is_size_8192bytes(&self) -> bool {
        *self == Bootprotselect::Size8192bytes
    }
    #[doc = "4096 bytes are protected."]
    #[inline(always)]
    pub fn is_size_4096bytes(&self) -> bool {
        *self == Bootprotselect::Size4096bytes
    }
    #[doc = "2048 bytes are protected."]
    #[inline(always)]
    pub fn is_size_2048bytes(&self) -> bool {
        *self == Bootprotselect::Size2048bytes
    }
    #[doc = "1024 bytes are protected."]
    #[inline(always)]
    pub fn is_size_1024bytes(&self) -> bool {
        *self == Bootprotselect::Size1024bytes
    }
    #[doc = "512 bytes are protected."]
    #[inline(always)]
    pub fn is_size_512bytes(&self) -> bool {
        *self == Bootprotselect::Size512bytes
    }
    #[doc = "0 bytes are protected."]
    #[inline(always)]
    pub fn is_size_0bytes(&self) -> bool {
        *self == Bootprotselect::Size0bytes
    }
}
impl R {
    #[doc = "Bits 0:15 - NVM Parameters"]
    #[inline(always)]
    pub fn nvmp(&self) -> NvmpR {
        NvmpR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PszR {
        PszR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Boot Section Size"]
    #[inline(always)]
    pub fn bootprot(&self) -> BootprotR {
        BootprotR::new(((self.bits >> 24) & 7) as u8)
    }
}
#[doc = "NVM Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamSpec;
impl crate::RegisterSpec for ParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for ParamSpec {}
#[doc = "`reset()` method sets PARAM to value 0"]
impl crate::Resettable for ParamSpec {}

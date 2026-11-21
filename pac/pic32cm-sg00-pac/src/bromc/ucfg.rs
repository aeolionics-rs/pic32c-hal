#[doc = "Register `UCFG` reader"]
pub type R = crate::R<UcfgSpec>;
#[doc = "Boot CRC Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcrcdisselect {
    #[doc = "0: Boot ROM content is CRC verified by the ROM controller, status is reported to the DSU. On CRC failure, the controller blocks access to the ROM controller."]
    Bcrcdis0 = 0,
    #[doc = "1: No CRC verification at Boot."]
    Bcrcdis1 = 1,
}
impl From<Bcrcdisselect> for bool {
    #[inline(always)]
    fn from(variant: Bcrcdisselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCRCDIS` reader - Boot CRC Disable"]
pub type BcrcdisR = crate::BitReader<Bcrcdisselect>;
impl BcrcdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcrcdisselect {
        match self.bits {
            false => Bcrcdisselect::Bcrcdis0,
            true => Bcrcdisselect::Bcrcdis1,
        }
    }
    #[doc = "Boot ROM content is CRC verified by the ROM controller, status is reported to the DSU. On CRC failure, the controller blocks access to the ROM controller."]
    #[inline(always)]
    pub fn is_bcrcdis_0(&self) -> bool {
        *self == Bcrcdisselect::Bcrcdis0
    }
    #[doc = "No CRC verification at Boot."]
    #[inline(always)]
    pub fn is_bcrcdis_1(&self) -> bool {
        *self == Bcrcdisselect::Bcrcdis1
    }
}
impl R {
    #[doc = "Bit 0 - Boot CRC Disable"]
    #[inline(always)]
    pub fn bcrcdis(&self) -> BcrcdisR {
        BcrcdisR::new((self.bits & 1) != 0)
    }
}
#[doc = "User Configuration REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ucfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcfgSpec;
impl crate::RegisterSpec for UcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ucfg::R`](R) reader structure"]
impl crate::Readable for UcfgSpec {}
#[doc = "`reset()` method sets UCFG to value 0x01"]
impl crate::Resettable for UcfgSpec {
    const RESET_VALUE: u32 = 0x01;
}

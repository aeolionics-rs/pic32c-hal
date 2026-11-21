#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Flash SEC Interrupt Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Serrselect {
    #[doc = "0: Interrupt Disabled"]
    Serr0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Serr1 = 1,
}
impl From<Serrselect> for bool {
    #[inline(always)]
    fn from(variant: Serrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SERR` reader - Flash SEC Interrupt Clear Enable"]
pub type SerrR = crate::BitReader<Serrselect>;
impl SerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Serrselect {
        match self.bits {
            false => Serrselect::Serr0,
            true => Serrselect::Serr1,
        }
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_serr_0(&self) -> bool {
        *self == Serrselect::Serr0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_serr_1(&self) -> bool {
        *self == Serrselect::Serr1
    }
}
#[doc = "Field `SERR` writer - Flash SEC Interrupt Clear Enable"]
pub type SerrW<'a, REG> = crate::BitWriter1C<'a, REG, Serrselect>;
impl<'a, REG> SerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn serr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Serrselect::Serr0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn serr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Serrselect::Serr1)
    }
}
#[doc = "CRC Calculation Done Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcdoneselect {
    #[doc = "0: Interrupt Disabled"]
    Crcdone0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Crcdone1 = 1,
}
impl From<Crcdoneselect> for bool {
    #[inline(always)]
    fn from(variant: Crcdoneselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCDONE` reader - CRC Calculation Done Clear Enable"]
pub type CrcdoneR = crate::BitReader<Crcdoneselect>;
impl CrcdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcdoneselect {
        match self.bits {
            false => Crcdoneselect::Crcdone0,
            true => Crcdoneselect::Crcdone1,
        }
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_crcdone_0(&self) -> bool {
        *self == Crcdoneselect::Crcdone0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_crcdone_1(&self) -> bool {
        *self == Crcdoneselect::Crcdone1
    }
}
#[doc = "Field `CRCDONE` writer - CRC Calculation Done Clear Enable"]
pub type CrcdoneW<'a, REG> = crate::BitWriter1C<'a, REG, Crcdoneselect>;
impl<'a, REG> CrcdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn crcdone_0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcdoneselect::Crcdone0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn crcdone_1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcdoneselect::Crcdone1)
    }
}
#[doc = "CRC Error Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcerrselect {
    #[doc = "0: Interrupt Disabled"]
    Crcerr0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Crcerr1 = 1,
}
impl From<Crcerrselect> for bool {
    #[inline(always)]
    fn from(variant: Crcerrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR` reader - CRC Error Clear Enable"]
pub type CrcerrR = crate::BitReader<Crcerrselect>;
impl CrcerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcerrselect {
        match self.bits {
            false => Crcerrselect::Crcerr0,
            true => Crcerrselect::Crcerr1,
        }
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_crcerr_0(&self) -> bool {
        *self == Crcerrselect::Crcerr0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_crcerr_1(&self) -> bool {
        *self == Crcerrselect::Crcerr1
    }
}
#[doc = "Field `CRCERR` writer - CRC Error Clear Enable"]
pub type CrcerrW<'a, REG> = crate::BitWriter1C<'a, REG, Crcerrselect>;
impl<'a, REG> CrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn crcerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcerrselect::Crcerr0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn crcerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcerrselect::Crcerr1)
    }
}
#[doc = "ECC Fault Capture Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fltcapselect {
    #[doc = "0: Interrupt Disabled"]
    Fltcap0 = 0,
    #[doc = "1: Interrupt Enabled"]
    Fltcap1 = 1,
}
impl From<Fltcapselect> for bool {
    #[inline(always)]
    fn from(variant: Fltcapselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLTCAP` reader - ECC Fault Capture Clear Enable"]
pub type FltcapR = crate::BitReader<Fltcapselect>;
impl FltcapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fltcapselect {
        match self.bits {
            false => Fltcapselect::Fltcap0,
            true => Fltcapselect::Fltcap1,
        }
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_fltcap_0(&self) -> bool {
        *self == Fltcapselect::Fltcap0
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_fltcap_1(&self) -> bool {
        *self == Fltcapselect::Fltcap1
    }
}
#[doc = "Field `FLTCAP` writer - ECC Fault Capture Clear Enable"]
pub type FltcapW<'a, REG> = crate::BitWriter1C<'a, REG, Fltcapselect>;
impl<'a, REG> FltcapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn fltcap_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fltcapselect::Fltcap0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn fltcap_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fltcapselect::Fltcap1)
    }
}
impl R {
    #[doc = "Bit 0 - Flash SEC Interrupt Clear Enable"]
    #[inline(always)]
    pub fn serr(&self) -> SerrR {
        SerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - CRC Calculation Done Clear Enable"]
    #[inline(always)]
    pub fn crcdone(&self) -> CrcdoneR {
        CrcdoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CRC Error Clear Enable"]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - ECC Fault Capture Clear Enable"]
    #[inline(always)]
    pub fn fltcap(&self) -> FltcapR {
        FltcapR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash SEC Interrupt Clear Enable"]
    #[inline(always)]
    pub fn serr(&mut self) -> SerrW<'_, IntenclrSpec> {
        SerrW::new(self, 0)
    }
    #[doc = "Bit 8 - CRC Calculation Done Clear Enable"]
    #[inline(always)]
    pub fn crcdone(&mut self) -> CrcdoneW<'_, IntenclrSpec> {
        CrcdoneW::new(self, 8)
    }
    #[doc = "Bit 9 - CRC Error Clear Enable"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CrcerrW<'_, IntenclrSpec> {
        CrcerrW::new(self, 9)
    }
    #[doc = "Bit 16 - ECC Fault Capture Clear Enable"]
    #[inline(always)]
    pub fn fltcap(&mut self) -> FltcapW<'_, IntenclrSpec> {
        FltcapW::new(self, 16)
    }
}
#[doc = "Interrupt Enable Clear REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0301;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}

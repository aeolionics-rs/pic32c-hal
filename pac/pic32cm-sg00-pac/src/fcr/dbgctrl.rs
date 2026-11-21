#[doc = "Register `DBGCTRL` reader"]
pub type R = crate::R<DbgctrlSpec>;
#[doc = "Register `DBGCTRL` writer"]
pub type W = crate::W<DbgctrlSpec>;
#[doc = "CRC Debug Run Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcrunselect {
    #[doc = "0: CRC Logic Halts in Debug Mode"]
    Halt = 0,
    #[doc = "1: CRC Logic Runs in Debug Mode."]
    Run = 1,
}
impl From<Crcrunselect> for bool {
    #[inline(always)]
    fn from(variant: Crcrunselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCRUN` reader - CRC Debug Run Enable"]
pub type CrcrunR = crate::BitReader<Crcrunselect>;
impl CrcrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcrunselect {
        match self.bits {
            false => Crcrunselect::Halt,
            true => Crcrunselect::Run,
        }
    }
    #[doc = "CRC Logic Halts in Debug Mode"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == Crcrunselect::Halt
    }
    #[doc = "CRC Logic Runs in Debug Mode."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Crcrunselect::Run
    }
}
#[doc = "Field `CRCRUN` writer - CRC Debug Run Enable"]
pub type CrcrunW<'a, REG> = crate::BitWriter<'a, REG, Crcrunselect>;
impl<'a, REG> CrcrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC Logic Halts in Debug Mode"]
    #[inline(always)]
    pub fn halt(self) -> &'a mut crate::W<REG> {
        self.variant(Crcrunselect::Halt)
    }
    #[doc = "CRC Logic Runs in Debug Mode."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Crcrunselect::Run)
    }
}
#[doc = "Debug ECC ModeECC errors from debugger reads are:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dbgeccselect {
    #[doc = "0: Corrected per ECCCTRL; No Bus ERR; INTFLAG, SEC counter, and FLT logic are not updated."]
    Noerr = 0,
    #[doc = "1: Not corrected; No Bus Error; INTFLAG, SEC counter, and FLT logic are not updated."]
    Disable = 1,
    #[doc = "2: Corrected per ECCCTRL; Bus Error, INTFLAG, SEC counter, and FLT logic operates as setup."]
    Enable = 2,
    #[doc = "3: Don't use but same as 01 (DISABLE)"]
    Rsvd11 = 3,
}
impl From<Dbgeccselect> for u8 {
    #[inline(always)]
    fn from(variant: Dbgeccselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dbgeccselect {
    type Ux = u8;
}
impl crate::IsEnum for Dbgeccselect {}
#[doc = "Field `DBGECC` reader - Debug ECC ModeECC errors from debugger reads are:"]
pub type DbgeccR = crate::FieldReader<Dbgeccselect>;
impl DbgeccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgeccselect {
        match self.bits {
            0 => Dbgeccselect::Noerr,
            1 => Dbgeccselect::Disable,
            2 => Dbgeccselect::Enable,
            3 => Dbgeccselect::Rsvd11,
            _ => unreachable!(),
        }
    }
    #[doc = "Corrected per ECCCTRL; No Bus ERR; INTFLAG, SEC counter, and FLT logic are not updated."]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == Dbgeccselect::Noerr
    }
    #[doc = "Not corrected; No Bus Error; INTFLAG, SEC counter, and FLT logic are not updated."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dbgeccselect::Disable
    }
    #[doc = "Corrected per ECCCTRL; Bus Error, INTFLAG, SEC counter, and FLT logic operates as setup."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dbgeccselect::Enable
    }
    #[doc = "Don't use but same as 01 (DISABLE)"]
    #[inline(always)]
    pub fn is_rsvd11(&self) -> bool {
        *self == Dbgeccselect::Rsvd11
    }
}
#[doc = "Field `DBGECC` writer - Debug ECC ModeECC errors from debugger reads are:"]
pub type DbgeccW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dbgeccselect, crate::Safe>;
impl<'a, REG> DbgeccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Corrected per ECCCTRL; No Bus ERR; INTFLAG, SEC counter, and FLT logic are not updated."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgeccselect::Noerr)
    }
    #[doc = "Not corrected; No Bus Error; INTFLAG, SEC counter, and FLT logic are not updated."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgeccselect::Disable)
    }
    #[doc = "Corrected per ECCCTRL; Bus Error, INTFLAG, SEC counter, and FLT logic operates as setup."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgeccselect::Enable)
    }
    #[doc = "Don't use but same as 01 (DISABLE)"]
    #[inline(always)]
    pub fn rsvd11(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgeccselect::Rsvd11)
    }
}
impl R {
    #[doc = "Bit 0 - CRC Debug Run Enable"]
    #[inline(always)]
    pub fn crcrun(&self) -> CrcrunR {
        CrcrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Debug ECC ModeECC errors from debugger reads are:"]
    #[inline(always)]
    pub fn dbgecc(&self) -> DbgeccR {
        DbgeccR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Debug Run Enable"]
    #[inline(always)]
    pub fn crcrun(&mut self) -> CrcrunW<'_, DbgctrlSpec> {
        CrcrunW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Debug ECC ModeECC errors from debugger reads are:"]
    #[inline(always)]
    pub fn dbgecc(&mut self) -> DbgeccW<'_, DbgctrlSpec> {
        DbgeccW::new(self, 1)
    }
}
#[doc = "Debug Control CTRL REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgctrlSpec;
impl crate::RegisterSpec for DbgctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgctrl::R`](R) reader structure"]
impl crate::Readable for DbgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgctrl::W`](W) writer structure"]
impl crate::Writable for DbgctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGCTRL to value 0"]
impl crate::Resettable for DbgctrlSpec {}

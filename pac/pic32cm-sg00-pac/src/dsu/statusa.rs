#[doc = "Register `STATUSA` reader"]
pub type R = crate::R<StatusaSpec>;
#[doc = "Register `STATUSA` writer"]
pub type W = crate::W<StatusaSpec>;
#[doc = "Done Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doneselect {
    #[doc = "0: DSU is executing a command or idle."]
    Done0 = 0,
    #[doc = "1: TCTRL.CMD command completed"]
    Done1 = 1,
}
impl From<Doneselect> for bool {
    #[inline(always)]
    fn from(variant: Doneselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Done Status"]
pub type DoneR = crate::BitReader<Doneselect>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doneselect {
        match self.bits {
            false => Doneselect::Done0,
            true => Doneselect::Done1,
        }
    }
    #[doc = "DSU is executing a command or idle."]
    #[inline(always)]
    pub fn is_done_0(&self) -> bool {
        *self == Doneselect::Done0
    }
    #[doc = "TCTRL.CMD command completed"]
    #[inline(always)]
    pub fn is_done_1(&self) -> bool {
        *self == Doneselect::Done1
    }
}
#[doc = "Field `DONE` writer - Done Status"]
pub type DoneW<'a, REG> = crate::BitWriter1C<'a, REG, Doneselect>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DSU is executing a command or idle."]
    #[inline(always)]
    pub fn done_0(self) -> &'a mut crate::W<REG> {
        self.variant(Doneselect::Done0)
    }
    #[doc = "TCTRL.CMD command completed"]
    #[inline(always)]
    pub fn done_1(self) -> &'a mut crate::W<REG> {
        self.variant(Doneselect::Done1)
    }
}
#[doc = "Bus Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Berrselect {
    #[doc = "0: No bus error detected"]
    Berr0 = 0,
    #[doc = "1: Bus error incurred while performing the DSU command."]
    Berr1 = 1,
}
impl From<Berrselect> for bool {
    #[inline(always)]
    fn from(variant: Berrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BERR` reader - Bus Error Status"]
pub type BerrR = crate::BitReader<Berrselect>;
impl BerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Berrselect {
        match self.bits {
            false => Berrselect::Berr0,
            true => Berrselect::Berr1,
        }
    }
    #[doc = "No bus error detected"]
    #[inline(always)]
    pub fn is_berr_0(&self) -> bool {
        *self == Berrselect::Berr0
    }
    #[doc = "Bus error incurred while performing the DSU command."]
    #[inline(always)]
    pub fn is_berr_1(&self) -> bool {
        *self == Berrselect::Berr1
    }
}
#[doc = "Field `BERR` writer - Bus Error Status"]
pub type BerrW<'a, REG> = crate::BitWriter1C<'a, REG, Berrselect>;
impl<'a, REG> BerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bus error detected"]
    #[inline(always)]
    pub fn berr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Berrselect::Berr0)
    }
    #[doc = "Bus error incurred while performing the DSU command."]
    #[inline(always)]
    pub fn berr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Berrselect::Berr1)
    }
}
#[doc = "Protection Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perrselect {
    #[doc = "0: No protection error detected"]
    Perr0 = 0,
    #[doc = "1: Invalid command written to TCTRL.CMD, partial write to the upper or lower bytes of TCTRL.CMD, or invalid key written to SAFEDOOR."]
    Perr1 = 1,
}
impl From<Perrselect> for bool {
    #[inline(always)]
    fn from(variant: Perrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERR` reader - Protection Error Status"]
pub type PerrR = crate::BitReader<Perrselect>;
impl PerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perrselect {
        match self.bits {
            false => Perrselect::Perr0,
            true => Perrselect::Perr1,
        }
    }
    #[doc = "No protection error detected"]
    #[inline(always)]
    pub fn is_perr_0(&self) -> bool {
        *self == Perrselect::Perr0
    }
    #[doc = "Invalid command written to TCTRL.CMD, partial write to the upper or lower bytes of TCTRL.CMD, or invalid key written to SAFEDOOR."]
    #[inline(always)]
    pub fn is_perr_1(&self) -> bool {
        *self == Perrselect::Perr1
    }
}
#[doc = "Field `PERR` writer - Protection Error Status"]
pub type PerrW<'a, REG> = crate::BitWriter1C<'a, REG, Perrselect>;
impl<'a, REG> PerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No protection error detected"]
    #[inline(always)]
    pub fn perr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Perrselect::Perr0)
    }
    #[doc = "Invalid command written to TCTRL.CMD, partial write to the upper or lower bytes of TCTRL.CMD, or invalid key written to SAFEDOOR."]
    #[inline(always)]
    pub fn perr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Perrselect::Perr1)
    }
}
#[doc = "Boot ROM CRC Self-Check Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Romcrcselect {
    #[doc = "0: CRC is in progress"]
    Romcrc0 = 0,
    #[doc = "1: CRC completed but failed indicating possible issue with the ROM content. System held in lockup state."]
    Romcrc1 = 1,
    #[doc = "2: CRC completed and passed"]
    Romcrc2 = 2,
    #[doc = "3: CRC never executed because user disabled CRC check through fuse settings."]
    Romcrc3 = 3,
}
impl From<Romcrcselect> for u8 {
    #[inline(always)]
    fn from(variant: Romcrcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Romcrcselect {
    type Ux = u8;
}
impl crate::IsEnum for Romcrcselect {}
#[doc = "Field `ROMCRC` reader - Boot ROM CRC Self-Check Status"]
pub type RomcrcR = crate::FieldReader<Romcrcselect>;
impl RomcrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Romcrcselect {
        match self.bits {
            0 => Romcrcselect::Romcrc0,
            1 => Romcrcselect::Romcrc1,
            2 => Romcrcselect::Romcrc2,
            3 => Romcrcselect::Romcrc3,
            _ => unreachable!(),
        }
    }
    #[doc = "CRC is in progress"]
    #[inline(always)]
    pub fn is_romcrc_0(&self) -> bool {
        *self == Romcrcselect::Romcrc0
    }
    #[doc = "CRC completed but failed indicating possible issue with the ROM content. System held in lockup state."]
    #[inline(always)]
    pub fn is_romcrc_1(&self) -> bool {
        *self == Romcrcselect::Romcrc1
    }
    #[doc = "CRC completed and passed"]
    #[inline(always)]
    pub fn is_romcrc_2(&self) -> bool {
        *self == Romcrcselect::Romcrc2
    }
    #[doc = "CRC never executed because user disabled CRC check through fuse settings."]
    #[inline(always)]
    pub fn is_romcrc_3(&self) -> bool {
        *self == Romcrcselect::Romcrc3
    }
}
#[doc = "Field `ROMCRC` writer - Boot ROM CRC Self-Check Status"]
pub type RomcrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Romcrcselect, crate::Safe>;
impl<'a, REG> RomcrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CRC is in progress"]
    #[inline(always)]
    pub fn romcrc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Romcrcselect::Romcrc0)
    }
    #[doc = "CRC completed but failed indicating possible issue with the ROM content. System held in lockup state."]
    #[inline(always)]
    pub fn romcrc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Romcrcselect::Romcrc1)
    }
    #[doc = "CRC completed and passed"]
    #[inline(always)]
    pub fn romcrc_2(self) -> &'a mut crate::W<REG> {
        self.variant(Romcrcselect::Romcrc2)
    }
    #[doc = "CRC never executed because user disabled CRC check through fuse settings."]
    #[inline(always)]
    pub fn romcrc_3(self) -> &'a mut crate::W<REG> {
        self.variant(Romcrcselect::Romcrc3)
    }
}
#[doc = "CPUx Reset Extension Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crstext0select {
    #[doc = "0: CPUx released from reset extension."]
    Crstext0_0 = 0,
    #[doc = "1: CPUx held in reset extension. Set on Cold-Plugging detection."]
    Crstext0_1 = 1,
}
impl From<Crstext0select> for bool {
    #[inline(always)]
    fn from(variant: Crstext0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSTEXT0` reader - CPUx Reset Extension Status"]
pub type Crstext0R = crate::BitReader<Crstext0select>;
impl Crstext0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crstext0select {
        match self.bits {
            false => Crstext0select::Crstext0_0,
            true => Crstext0select::Crstext0_1,
        }
    }
    #[doc = "CPUx released from reset extension."]
    #[inline(always)]
    pub fn is_crstext0_0(&self) -> bool {
        *self == Crstext0select::Crstext0_0
    }
    #[doc = "CPUx held in reset extension. Set on Cold-Plugging detection."]
    #[inline(always)]
    pub fn is_crstext0_1(&self) -> bool {
        *self == Crstext0select::Crstext0_1
    }
}
#[doc = "Field `CRSTEXT0` writer - CPUx Reset Extension Status"]
pub type Crstext0W<'a, REG> = crate::BitWriter1C<'a, REG, Crstext0select>;
impl<'a, REG> Crstext0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPUx released from reset extension."]
    #[inline(always)]
    pub fn crstext0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Crstext0select::Crstext0_0)
    }
    #[doc = "CPUx held in reset extension. Set on Cold-Plugging detection."]
    #[inline(always)]
    pub fn crstext0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Crstext0select::Crstext0_1)
    }
}
#[doc = "BootRom 0 Phase Extension Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brext0select {
    #[doc = "0: Boot ROM Phase Extension released for CPUx."]
    Brext0_0 = 0,
    #[doc = "1: Boot ROM Phase Extension for CPUx. Set on Cold-Plugging detection."]
    Brext0_1 = 1,
}
impl From<Brext0select> for bool {
    #[inline(always)]
    fn from(variant: Brext0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREXT0` reader - BootRom 0 Phase Extension Status"]
pub type Brext0R = crate::BitReader<Brext0select>;
impl Brext0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brext0select {
        match self.bits {
            false => Brext0select::Brext0_0,
            true => Brext0select::Brext0_1,
        }
    }
    #[doc = "Boot ROM Phase Extension released for CPUx."]
    #[inline(always)]
    pub fn is_brext0_0(&self) -> bool {
        *self == Brext0select::Brext0_0
    }
    #[doc = "Boot ROM Phase Extension for CPUx. Set on Cold-Plugging detection."]
    #[inline(always)]
    pub fn is_brext0_1(&self) -> bool {
        *self == Brext0select::Brext0_1
    }
}
#[doc = "Field `BREXT0` writer - BootRom 0 Phase Extension Status"]
pub type Brext0W<'a, REG> = crate::BitWriter1C<'a, REG, Brext0select>;
impl<'a, REG> Brext0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Boot ROM Phase Extension released for CPUx."]
    #[inline(always)]
    pub fn brext0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Brext0select::Brext0_0)
    }
    #[doc = "Boot ROM Phase Extension for CPUx. Set on Cold-Plugging detection."]
    #[inline(always)]
    pub fn brext0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Brext0select::Brext0_1)
    }
}
impl R {
    #[doc = "Bit 0 - Done Status"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Bus Error Status"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protection Error Status"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Boot ROM CRC Self-Check Status"]
    #[inline(always)]
    pub fn romcrc(&self) -> RomcrcR {
        RomcrcR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - CPUx Reset Extension Status"]
    #[inline(always)]
    pub fn crstext0(&self) -> Crstext0R {
        Crstext0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - BootRom 0 Phase Extension Status"]
    #[inline(always)]
    pub fn brext0(&self) -> Brext0R {
        Brext0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Done Status"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, StatusaSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 2 - Bus Error Status"]
    #[inline(always)]
    pub fn berr(&mut self) -> BerrW<'_, StatusaSpec> {
        BerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Protection Error Status"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, StatusaSpec> {
        PerrW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Boot ROM CRC Self-Check Status"]
    #[inline(always)]
    pub fn romcrc(&mut self) -> RomcrcW<'_, StatusaSpec> {
        RomcrcW::new(self, 6)
    }
    #[doc = "Bit 8 - CPUx Reset Extension Status"]
    #[inline(always)]
    pub fn crstext0(&mut self) -> Crstext0W<'_, StatusaSpec> {
        Crstext0W::new(self, 8)
    }
    #[doc = "Bit 16 - BootRom 0 Phase Extension Status"]
    #[inline(always)]
    pub fn brext0(&mut self) -> Brext0W<'_, StatusaSpec> {
        Brext0W::new(self, 16)
    }
}
#[doc = "Status A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusaSpec;
impl crate::RegisterSpec for StatusaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusa::R`](R) reader structure"]
impl crate::Readable for StatusaSpec {}
#[doc = "`write(|w| ..)` method takes [`statusa::W`](W) writer structure"]
impl crate::Writable for StatusaSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_01cd;
}
#[doc = "`reset()` method sets STATUSA to value 0"]
impl crate::Resettable for StatusaSpec {}

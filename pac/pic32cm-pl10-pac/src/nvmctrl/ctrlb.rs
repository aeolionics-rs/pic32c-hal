#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdselect {
    #[doc = "0: No command"]
    Nocmd = 0,
    #[doc = "1: No operation"]
    Noop = 1,
    #[doc = "2: Flash Write Enable"]
    Flwr = 2,
    #[doc = "8: Flash Page Erase Enable"]
    Flper = 8,
    #[doc = "9: Flash 2-page Erase Enable"]
    Flmper2 = 9,
    #[doc = "10: Flash 4-page Erase Enable"]
    Flmper4 = 10,
    #[doc = "11: Flash 8-page Erase Enable"]
    Flmper8 = 11,
    #[doc = "12: Flash 16-page Erase Enable"]
    Flmper16 = 12,
    #[doc = "13: Flash 32-page Erase Enable"]
    Flmper32 = 13,
    #[doc = "14: Lock Region. Sets the bit in the Region n Lock Bits bit field in the Lock Section register (NVMCTRL.LOCK) corresponding to the address location in the ADDR register."]
    Lr = 14,
    #[doc = "15: Unlock Region. Clears the bit in NVMCTRL.LOCK corresponding to the address location in the ADDR register."]
    Ur = 15,
    #[doc = "16: Erase BOOTCFG Page Enable"]
    Ebootcfg = 16,
    #[doc = "17: Write BOOTCFG Page Enable"]
    Wbootcfg = 17,
    #[doc = "18: Write Enable to ROMCFG.NVMLOCKREGION. Writes to other addresses will cause STATUS.PROGE to be set."]
    Wlockregion = 18,
    #[doc = "32: Write ROMCFG Page Enable. The ROMCFG Page is used by the boot ROM to store Debug Access Level (DAL) bits and other security or protection bits for such as CEHL and Immutable boot."]
    Wromcfg = 32,
    #[doc = "33: Erase FlashRequires DAL == 2 or SYSINT privileges"]
    Cher = 33,
}
impl From<Cmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmdselect {}
#[doc = "Field `CMD` reader - Command"]
pub type CmdR = crate::FieldReader<Cmdselect>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdselect> {
        match self.bits {
            0 => Some(Cmdselect::Nocmd),
            1 => Some(Cmdselect::Noop),
            2 => Some(Cmdselect::Flwr),
            8 => Some(Cmdselect::Flper),
            9 => Some(Cmdselect::Flmper2),
            10 => Some(Cmdselect::Flmper4),
            11 => Some(Cmdselect::Flmper8),
            12 => Some(Cmdselect::Flmper16),
            13 => Some(Cmdselect::Flmper32),
            14 => Some(Cmdselect::Lr),
            15 => Some(Cmdselect::Ur),
            16 => Some(Cmdselect::Ebootcfg),
            17 => Some(Cmdselect::Wbootcfg),
            18 => Some(Cmdselect::Wlockregion),
            32 => Some(Cmdselect::Wromcfg),
            33 => Some(Cmdselect::Cher),
            _ => None,
        }
    }
    #[doc = "No command"]
    #[inline(always)]
    pub fn is_nocmd(&self) -> bool {
        *self == Cmdselect::Nocmd
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn is_noop(&self) -> bool {
        *self == Cmdselect::Noop
    }
    #[doc = "Flash Write Enable"]
    #[inline(always)]
    pub fn is_flwr(&self) -> bool {
        *self == Cmdselect::Flwr
    }
    #[doc = "Flash Page Erase Enable"]
    #[inline(always)]
    pub fn is_flper(&self) -> bool {
        *self == Cmdselect::Flper
    }
    #[doc = "Flash 2-page Erase Enable"]
    #[inline(always)]
    pub fn is_flmper2(&self) -> bool {
        *self == Cmdselect::Flmper2
    }
    #[doc = "Flash 4-page Erase Enable"]
    #[inline(always)]
    pub fn is_flmper4(&self) -> bool {
        *self == Cmdselect::Flmper4
    }
    #[doc = "Flash 8-page Erase Enable"]
    #[inline(always)]
    pub fn is_flmper8(&self) -> bool {
        *self == Cmdselect::Flmper8
    }
    #[doc = "Flash 16-page Erase Enable"]
    #[inline(always)]
    pub fn is_flmper16(&self) -> bool {
        *self == Cmdselect::Flmper16
    }
    #[doc = "Flash 32-page Erase Enable"]
    #[inline(always)]
    pub fn is_flmper32(&self) -> bool {
        *self == Cmdselect::Flmper32
    }
    #[doc = "Lock Region. Sets the bit in the Region n Lock Bits bit field in the Lock Section register (NVMCTRL.LOCK) corresponding to the address location in the ADDR register."]
    #[inline(always)]
    pub fn is_lr(&self) -> bool {
        *self == Cmdselect::Lr
    }
    #[doc = "Unlock Region. Clears the bit in NVMCTRL.LOCK corresponding to the address location in the ADDR register."]
    #[inline(always)]
    pub fn is_ur(&self) -> bool {
        *self == Cmdselect::Ur
    }
    #[doc = "Erase BOOTCFG Page Enable"]
    #[inline(always)]
    pub fn is_ebootcfg(&self) -> bool {
        *self == Cmdselect::Ebootcfg
    }
    #[doc = "Write BOOTCFG Page Enable"]
    #[inline(always)]
    pub fn is_wbootcfg(&self) -> bool {
        *self == Cmdselect::Wbootcfg
    }
    #[doc = "Write Enable to ROMCFG.NVMLOCKREGION. Writes to other addresses will cause STATUS.PROGE to be set."]
    #[inline(always)]
    pub fn is_wlockregion(&self) -> bool {
        *self == Cmdselect::Wlockregion
    }
    #[doc = "Write ROMCFG Page Enable. The ROMCFG Page is used by the boot ROM to store Debug Access Level (DAL) bits and other security or protection bits for such as CEHL and Immutable boot."]
    #[inline(always)]
    pub fn is_wromcfg(&self) -> bool {
        *self == Cmdselect::Wromcfg
    }
    #[doc = "Erase FlashRequires DAL == 2 or SYSINT privileges"]
    #[inline(always)]
    pub fn is_cher(&self) -> bool {
        *self == Cmdselect::Cher
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 7, Cmdselect>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No command"]
    #[inline(always)]
    pub fn nocmd(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Nocmd)
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn noop(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Noop)
    }
    #[doc = "Flash Write Enable"]
    #[inline(always)]
    pub fn flwr(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Flwr)
    }
    #[doc = "Flash Page Erase Enable"]
    #[inline(always)]
    pub fn flper(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Flper)
    }
    #[doc = "Flash 2-page Erase Enable"]
    #[inline(always)]
    pub fn flmper2(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Flmper2)
    }
    #[doc = "Flash 4-page Erase Enable"]
    #[inline(always)]
    pub fn flmper4(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Flmper4)
    }
    #[doc = "Flash 8-page Erase Enable"]
    #[inline(always)]
    pub fn flmper8(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Flmper8)
    }
    #[doc = "Flash 16-page Erase Enable"]
    #[inline(always)]
    pub fn flmper16(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Flmper16)
    }
    #[doc = "Flash 32-page Erase Enable"]
    #[inline(always)]
    pub fn flmper32(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Flmper32)
    }
    #[doc = "Lock Region. Sets the bit in the Region n Lock Bits bit field in the Lock Section register (NVMCTRL.LOCK) corresponding to the address location in the ADDR register."]
    #[inline(always)]
    pub fn lr(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Lr)
    }
    #[doc = "Unlock Region. Clears the bit in NVMCTRL.LOCK corresponding to the address location in the ADDR register."]
    #[inline(always)]
    pub fn ur(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Ur)
    }
    #[doc = "Erase BOOTCFG Page Enable"]
    #[inline(always)]
    pub fn ebootcfg(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Ebootcfg)
    }
    #[doc = "Write BOOTCFG Page Enable"]
    #[inline(always)]
    pub fn wbootcfg(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Wbootcfg)
    }
    #[doc = "Write Enable to ROMCFG.NVMLOCKREGION. Writes to other addresses will cause STATUS.PROGE to be set."]
    #[inline(always)]
    pub fn wlockregion(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Wlockregion)
    }
    #[doc = "Write ROMCFG Page Enable. The ROMCFG Page is used by the boot ROM to store Debug Access Level (DAL) bits and other security or protection bits for such as CEHL and Immutable boot."]
    #[inline(always)]
    pub fn wromcfg(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Wromcfg)
    }
    #[doc = "Erase FlashRequires DAL == 2 or SYSINT privileges"]
    #[inline(always)]
    pub fn cher(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Cher)
    }
}
#[doc = "Command Execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdexselect {
    #[doc = "165: Execution key"]
    Key = 165,
}
impl From<Cmdexselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmdexselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdexselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmdexselect {}
#[doc = "Field `CMDEX` reader - Command Execution"]
pub type CmdexR = crate::FieldReader<Cmdexselect>;
impl CmdexR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdexselect> {
        match self.bits {
            165 => Some(Cmdexselect::Key),
            _ => None,
        }
    }
    #[doc = "Execution key"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == Cmdexselect::Key
    }
}
#[doc = "Field `CMDEX` writer - Command Execution"]
pub type CmdexW<'a, REG> = crate::FieldWriter<'a, REG, 8, Cmdexselect>;
impl<'a, REG> CmdexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Execution key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdexselect::Key)
    }
}
impl R {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    pub fn cmdex(&self) -> CmdexR {
        CmdexR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, CtrlbSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    pub fn cmdex(&mut self) -> CmdexW<'_, CtrlbSpec> {
        CmdexW::new(self, 8)
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}

#[doc = "Register `ECCCTRL` reader"]
pub type R = crate::R<EccctrlSpec>;
#[doc = "Register `ECCCTRL` writer"]
pub type W = crate::W<EccctrlSpec>;
#[doc = "NVM ECC Mode Control - restricts one or more NVMOPs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eccctlselect {
    #[doc = "0: ECC Writes with ECC Reads (NVMOP = Single Program Operation disabled)"]
    Strict = 0,
    #[doc = "1: Dynamic Writes with Dynamic Reads"]
    Dynamic = 1,
    #[doc = "2: ECC Writes with Dynamic Reads (NVMOP = Single Program Operation disabled)"]
    Ecc = 2,
    #[doc = "3: Dynamic Writes with No Error Check Reads"]
    Disable = 3,
}
impl From<Eccctlselect> for u8 {
    #[inline(always)]
    fn from(variant: Eccctlselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eccctlselect {
    type Ux = u8;
}
impl crate::IsEnum for Eccctlselect {}
#[doc = "Field `ECCCTL` reader - NVM ECC Mode Control - restricts one or more NVMOPs"]
pub type EccctlR = crate::FieldReader<Eccctlselect>;
impl EccctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eccctlselect {
        match self.bits {
            0 => Eccctlselect::Strict,
            1 => Eccctlselect::Dynamic,
            2 => Eccctlselect::Ecc,
            3 => Eccctlselect::Disable,
            _ => unreachable!(),
        }
    }
    #[doc = "ECC Writes with ECC Reads (NVMOP = Single Program Operation disabled)"]
    #[inline(always)]
    pub fn is_strict(&self) -> bool {
        *self == Eccctlselect::Strict
    }
    #[doc = "Dynamic Writes with Dynamic Reads"]
    #[inline(always)]
    pub fn is_dynamic(&self) -> bool {
        *self == Eccctlselect::Dynamic
    }
    #[doc = "ECC Writes with Dynamic Reads (NVMOP = Single Program Operation disabled)"]
    #[inline(always)]
    pub fn is_ecc(&self) -> bool {
        *self == Eccctlselect::Ecc
    }
    #[doc = "Dynamic Writes with No Error Check Reads"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Eccctlselect::Disable
    }
}
#[doc = "Field `ECCCTL` writer - NVM ECC Mode Control - restricts one or more NVMOPs"]
pub type EccctlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eccctlselect, crate::Safe>;
impl<'a, REG> EccctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ECC Writes with ECC Reads (NVMOP = Single Program Operation disabled)"]
    #[inline(always)]
    pub fn strict(self) -> &'a mut crate::W<REG> {
        self.variant(Eccctlselect::Strict)
    }
    #[doc = "Dynamic Writes with Dynamic Reads"]
    #[inline(always)]
    pub fn dynamic(self) -> &'a mut crate::W<REG> {
        self.variant(Eccctlselect::Dynamic)
    }
    #[doc = "ECC Writes with Dynamic Reads (NVMOP = Single Program Operation disabled)"]
    #[inline(always)]
    pub fn ecc(self) -> &'a mut crate::W<REG> {
        self.variant(Eccctlselect::Ecc)
    }
    #[doc = "Dynamic Writes with No Error Check Reads"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Eccctlselect::Disable)
    }
}
#[doc = "NVM ECC Mode Control Unlock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eccunlckselect {
    #[doc = "0: ECCUNLCK and ECCCTL\\[1:0\\] cannot be written"]
    Locked = 0,
    #[doc = "1: ECCUNLCK and ECCCTL\\[1:0\\] can be written"]
    Unlocked = 1,
}
impl From<Eccunlckselect> for bool {
    #[inline(always)]
    fn from(variant: Eccunlckselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCUNLCK` reader - NVM ECC Mode Control Unlock"]
pub type EccunlckR = crate::BitReader<Eccunlckselect>;
impl EccunlckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eccunlckselect {
        match self.bits {
            false => Eccunlckselect::Locked,
            true => Eccunlckselect::Unlocked,
        }
    }
    #[doc = "ECCUNLCK and ECCCTL\\[1:0\\] cannot be written"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Eccunlckselect::Locked
    }
    #[doc = "ECCUNLCK and ECCCTL\\[1:0\\] can be written"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Eccunlckselect::Unlocked
    }
}
#[doc = "Field `ECCUNLCK` writer - NVM ECC Mode Control Unlock"]
pub type EccunlckW<'a, REG> = crate::BitWriter<'a, REG, Eccunlckselect>;
impl<'a, REG> EccunlckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECCUNLCK and ECCCTL\\[1:0\\] cannot be written"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Eccunlckselect::Locked)
    }
    #[doc = "ECCUNLCK and ECCCTL\\[1:0\\] can be written"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Eccunlckselect::Unlocked)
    }
}
#[doc = "Field `SECCNT` reader - Flash SEC Count"]
pub type SeccntR = crate::FieldReader;
#[doc = "Field `SECCNT` writer - Flash SEC Count"]
pub type SeccntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 4:5 - NVM ECC Mode Control - restricts one or more NVMOPs"]
    #[inline(always)]
    pub fn eccctl(&self) -> EccctlR {
        EccctlR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - NVM ECC Mode Control Unlock"]
    #[inline(always)]
    pub fn eccunlck(&self) -> EccunlckR {
        EccunlckR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Flash SEC Count"]
    #[inline(always)]
    pub fn seccnt(&self) -> SeccntR {
        SeccntR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - NVM ECC Mode Control - restricts one or more NVMOPs"]
    #[inline(always)]
    pub fn eccctl(&mut self) -> EccctlW<'_, EccctrlSpec> {
        EccctlW::new(self, 4)
    }
    #[doc = "Bit 6 - NVM ECC Mode Control Unlock"]
    #[inline(always)]
    pub fn eccunlck(&mut self) -> EccunlckW<'_, EccctrlSpec> {
        EccunlckW::new(self, 6)
    }
    #[doc = "Bits 8:15 - Flash SEC Count"]
    #[inline(always)]
    pub fn seccnt(&mut self) -> SeccntW<'_, EccctrlSpec> {
        SeccntW::new(self, 8)
    }
}
#[doc = "ECC Control REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`eccctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccctrlSpec;
impl crate::RegisterSpec for EccctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccctrl::R`](R) reader structure"]
impl crate::Readable for EccctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`eccctrl::W`](W) writer structure"]
impl crate::Writable for EccctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECCCTRL to value 0x40"]
impl crate::Resettable for EccctrlSpec {
    const RESET_VALUE: u32 = 0x40;
}

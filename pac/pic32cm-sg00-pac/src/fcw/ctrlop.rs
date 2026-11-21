#[doc = "Register `CTRLOP` reader"]
pub type R = crate::R<CtrlopSpec>;
#[doc = "Register `CTRLOP` writer"]
pub type W = crate::W<CtrlopSpec>;
#[doc = "NVM Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nvmopselect {
    #[doc = "0: No Operation"]
    Nop = 0,
    #[doc = "1: Single (Word) Program Operation: Programs word selected by ADDR"]
    Singlewrite = 1,
    #[doc = "2: Quad (Word) Program Operation: Programs flash word selected by ADDR"]
    Quadwrite = 2,
    #[doc = "3: Row Write Operation: Programs row selected by ADDR"]
    Rowwrite = 3,
    #[doc = "4: Page Erase Operation: Erases page selected by ADDR"]
    Pageerase = 4,
    #[doc = "5: Lower PFM Erase Operation: (RSVD NVMOP for Single Panel Devices)"]
    Lpfmerase = 5,
    #[doc = "6: Upper PFM Erase Operation: (RSVD NVMOP for Single Panel Devices)"]
    Upfmerase = 6,
    #[doc = "7: PFM Erase Operation: Upper &amp; Lower PFM Erase"]
    Pfmerase = 7,
    #[doc = "14: Chip Erase Operation: Erases PFM+SRF, BFM, CFM (except for CalOTP, Test, VSS pages if HSM is Present, UserOTP, &amp; ROMCFG)."]
    Ce = 14,
}
impl From<Nvmopselect> for u8 {
    #[inline(always)]
    fn from(variant: Nvmopselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nvmopselect {
    type Ux = u8;
}
impl crate::IsEnum for Nvmopselect {}
#[doc = "Field `NVMOP` reader - NVM Operation"]
pub type NvmopR = crate::FieldReader<Nvmopselect>;
impl NvmopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nvmopselect> {
        match self.bits {
            0 => Some(Nvmopselect::Nop),
            1 => Some(Nvmopselect::Singlewrite),
            2 => Some(Nvmopselect::Quadwrite),
            3 => Some(Nvmopselect::Rowwrite),
            4 => Some(Nvmopselect::Pageerase),
            5 => Some(Nvmopselect::Lpfmerase),
            6 => Some(Nvmopselect::Upfmerase),
            7 => Some(Nvmopselect::Pfmerase),
            14 => Some(Nvmopselect::Ce),
            _ => None,
        }
    }
    #[doc = "No Operation"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == Nvmopselect::Nop
    }
    #[doc = "Single (Word) Program Operation: Programs word selected by ADDR"]
    #[inline(always)]
    pub fn is_singlewrite(&self) -> bool {
        *self == Nvmopselect::Singlewrite
    }
    #[doc = "Quad (Word) Program Operation: Programs flash word selected by ADDR"]
    #[inline(always)]
    pub fn is_quadwrite(&self) -> bool {
        *self == Nvmopselect::Quadwrite
    }
    #[doc = "Row Write Operation: Programs row selected by ADDR"]
    #[inline(always)]
    pub fn is_rowwrite(&self) -> bool {
        *self == Nvmopselect::Rowwrite
    }
    #[doc = "Page Erase Operation: Erases page selected by ADDR"]
    #[inline(always)]
    pub fn is_pageerase(&self) -> bool {
        *self == Nvmopselect::Pageerase
    }
    #[doc = "Lower PFM Erase Operation: (RSVD NVMOP for Single Panel Devices)"]
    #[inline(always)]
    pub fn is_lpfmerase(&self) -> bool {
        *self == Nvmopselect::Lpfmerase
    }
    #[doc = "Upper PFM Erase Operation: (RSVD NVMOP for Single Panel Devices)"]
    #[inline(always)]
    pub fn is_upfmerase(&self) -> bool {
        *self == Nvmopselect::Upfmerase
    }
    #[doc = "PFM Erase Operation: Upper &amp; Lower PFM Erase"]
    #[inline(always)]
    pub fn is_pfmerase(&self) -> bool {
        *self == Nvmopselect::Pfmerase
    }
    #[doc = "Chip Erase Operation: Erases PFM+SRF, BFM, CFM (except for CalOTP, Test, VSS pages if HSM is Present, UserOTP, &amp; ROMCFG)."]
    #[inline(always)]
    pub fn is_ce(&self) -> bool {
        *self == Nvmopselect::Ce
    }
}
#[doc = "Field `NVMOP` writer - NVM Operation"]
pub type NvmopW<'a, REG> = crate::FieldWriter<'a, REG, 4, Nvmopselect>;
impl<'a, REG> NvmopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Operation"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmopselect::Nop)
    }
    #[doc = "Single (Word) Program Operation: Programs word selected by ADDR"]
    #[inline(always)]
    pub fn singlewrite(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmopselect::Singlewrite)
    }
    #[doc = "Quad (Word) Program Operation: Programs flash word selected by ADDR"]
    #[inline(always)]
    pub fn quadwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmopselect::Quadwrite)
    }
    #[doc = "Row Write Operation: Programs row selected by ADDR"]
    #[inline(always)]
    pub fn rowwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmopselect::Rowwrite)
    }
    #[doc = "Page Erase Operation: Erases page selected by ADDR"]
    #[inline(always)]
    pub fn pageerase(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmopselect::Pageerase)
    }
    #[doc = "Lower PFM Erase Operation: (RSVD NVMOP for Single Panel Devices)"]
    #[inline(always)]
    pub fn lpfmerase(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmopselect::Lpfmerase)
    }
    #[doc = "Upper PFM Erase Operation: (RSVD NVMOP for Single Panel Devices)"]
    #[inline(always)]
    pub fn upfmerase(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmopselect::Upfmerase)
    }
    #[doc = "PFM Erase Operation: Upper &amp; Lower PFM Erase"]
    #[inline(always)]
    pub fn pfmerase(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmopselect::Pfmerase)
    }
    #[doc = "Chip Erase Operation: Erases PFM+SRF, BFM, CFM (except for CalOTP, Test, VSS pages if HSM is Present, UserOTP, &amp; ROMCFG)."]
    #[inline(always)]
    pub fn ce(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmopselect::Ce)
    }
}
#[doc = "NVM Pre-Program Configuration Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prepgselect {
    #[doc = "0: Program Operations exclude Pre-Program step"]
    Disable = 0,
    #[doc = "1: Program Operations include Pre-Program step"]
    Enable = 1,
}
impl From<Prepgselect> for bool {
    #[inline(always)]
    fn from(variant: Prepgselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREPG` reader - NVM Pre-Program Configuration Bit"]
pub type PrepgR = crate::BitReader<Prepgselect>;
impl PrepgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prepgselect {
        match self.bits {
            false => Prepgselect::Disable,
            true => Prepgselect::Enable,
        }
    }
    #[doc = "Program Operations exclude Pre-Program step"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Prepgselect::Disable
    }
    #[doc = "Program Operations include Pre-Program step"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Prepgselect::Enable
    }
}
#[doc = "Field `PREPG` writer - NVM Pre-Program Configuration Bit"]
pub type PrepgW<'a, REG> = crate::BitWriter<'a, REG, Prepgselect>;
impl<'a, REG> PrepgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Program Operations exclude Pre-Program step"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Prepgselect::Disable)
    }
    #[doc = "Program Operations include Pre-Program step"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Prepgselect::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - NVM Operation"]
    #[inline(always)]
    pub fn nvmop(&self) -> NvmopR {
        NvmopR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - NVM Pre-Program Configuration Bit"]
    #[inline(always)]
    pub fn prepg(&self) -> PrepgR {
        PrepgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - NVM Operation"]
    #[inline(always)]
    pub fn nvmop(&mut self) -> NvmopW<'_, CtrlopSpec> {
        NvmopW::new(self, 0)
    }
    #[doc = "Bit 7 - NVM Pre-Program Configuration Bit"]
    #[inline(always)]
    pub fn prepg(&mut self) -> PrepgW<'_, CtrlopSpec> {
        PrepgW::new(self, 7)
    }
}
#[doc = "Control Operation REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlopSpec;
impl crate::RegisterSpec for CtrlopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlop::R`](R) reader structure"]
impl crate::Readable for CtrlopSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlop::W`](W) writer structure"]
impl crate::Writable for CtrlopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLOP to value 0"]
impl crate::Resettable for CtrlopSpec {}

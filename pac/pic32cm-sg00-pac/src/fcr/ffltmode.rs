#[doc = "Register `FFLTMODE` reader"]
pub type R = crate::R<FfltmodeSpec>;
#[doc = "Register `FFLTMODE` writer"]
pub type W = crate::W<FfltmodeSpec>;
#[doc = "ECC/Parity Control Fault bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctlfltselect {
    #[doc = "0: No Fault Injected"]
    Ctlflt0 = 0,
    #[doc = "1: Inject a Fault on to the associated ECC/Parity Control bits (CTL\\[n\\])"]
    Ctlflt1 = 1,
}
impl From<Ctlfltselect> for u8 {
    #[inline(always)]
    fn from(variant: Ctlfltselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctlfltselect {
    type Ux = u8;
}
impl crate::IsEnum for Ctlfltselect {}
#[doc = "Field `CTLFLT` reader - ECC/Parity Control Fault bits"]
pub type CtlfltR = crate::FieldReader<Ctlfltselect>;
impl CtlfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctlfltselect> {
        match self.bits {
            0 => Some(Ctlfltselect::Ctlflt0),
            1 => Some(Ctlfltselect::Ctlflt1),
            _ => None,
        }
    }
    #[doc = "No Fault Injected"]
    #[inline(always)]
    pub fn is_ctlflt_0(&self) -> bool {
        *self == Ctlfltselect::Ctlflt0
    }
    #[doc = "Inject a Fault on to the associated ECC/Parity Control bits (CTL\\[n\\])"]
    #[inline(always)]
    pub fn is_ctlflt_1(&self) -> bool {
        *self == Ctlfltselect::Ctlflt1
    }
}
#[doc = "Field `CTLFLT` writer - ECC/Parity Control Fault bits"]
pub type CtlfltW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ctlfltselect>;
impl<'a, REG> CtlfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Fault Injected"]
    #[inline(always)]
    pub fn ctlflt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctlfltselect::Ctlflt0)
    }
    #[doc = "Inject a Fault on to the associated ECC/Parity Control bits (CTL\\[n\\])"]
    #[inline(always)]
    pub fn ctlflt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctlfltselect::Ctlflt1)
    }
}
#[doc = "Fault Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fltmdselect {
    #[doc = "0: Fault Injection Disabled"]
    Disable = 0,
    #[doc = "2: Fault Capture Mode Enabled - Captures the address (in FLTADR) and Syndrome in (FLTSYN)"]
    Capt = 2,
    #[doc = "4: Single Fault Injection (at bit selected by FLT1PTR) for Reads"]
    Singleread = 4,
    #[doc = "5: Double Fault Injection for Reads"]
    Doubleread = 5,
    #[doc = "6: Single Fault Injection (at bit selected by FLT1PTR) for Writes"]
    Singlewrite = 6,
    #[doc = "7: Double Fault Injection for Writes"]
    Doublewrite = 7,
}
impl From<Fltmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Fltmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fltmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Fltmdselect {}
#[doc = "Field `FLTMD` reader - Fault Mode Control"]
pub type FltmdR = crate::FieldReader<Fltmdselect>;
impl FltmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fltmdselect> {
        match self.bits {
            0 => Some(Fltmdselect::Disable),
            2 => Some(Fltmdselect::Capt),
            4 => Some(Fltmdselect::Singleread),
            5 => Some(Fltmdselect::Doubleread),
            6 => Some(Fltmdselect::Singlewrite),
            7 => Some(Fltmdselect::Doublewrite),
            _ => None,
        }
    }
    #[doc = "Fault Injection Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fltmdselect::Disable
    }
    #[doc = "Fault Capture Mode Enabled - Captures the address (in FLTADR) and Syndrome in (FLTSYN)"]
    #[inline(always)]
    pub fn is_capt(&self) -> bool {
        *self == Fltmdselect::Capt
    }
    #[doc = "Single Fault Injection (at bit selected by FLT1PTR) for Reads"]
    #[inline(always)]
    pub fn is_singleread(&self) -> bool {
        *self == Fltmdselect::Singleread
    }
    #[doc = "Double Fault Injection for Reads"]
    #[inline(always)]
    pub fn is_doubleread(&self) -> bool {
        *self == Fltmdselect::Doubleread
    }
    #[doc = "Single Fault Injection (at bit selected by FLT1PTR) for Writes"]
    #[inline(always)]
    pub fn is_singlewrite(&self) -> bool {
        *self == Fltmdselect::Singlewrite
    }
    #[doc = "Double Fault Injection for Writes"]
    #[inline(always)]
    pub fn is_doublewrite(&self) -> bool {
        *self == Fltmdselect::Doublewrite
    }
}
#[doc = "Field `FLTMD` writer - Fault Mode Control"]
pub type FltmdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fltmdselect>;
impl<'a, REG> FltmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fault Injection Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fltmdselect::Disable)
    }
    #[doc = "Fault Capture Mode Enabled - Captures the address (in FLTADR) and Syndrome in (FLTSYN)"]
    #[inline(always)]
    pub fn capt(self) -> &'a mut crate::W<REG> {
        self.variant(Fltmdselect::Capt)
    }
    #[doc = "Single Fault Injection (at bit selected by FLT1PTR) for Reads"]
    #[inline(always)]
    pub fn singleread(self) -> &'a mut crate::W<REG> {
        self.variant(Fltmdselect::Singleread)
    }
    #[doc = "Double Fault Injection for Reads"]
    #[inline(always)]
    pub fn doubleread(self) -> &'a mut crate::W<REG> {
        self.variant(Fltmdselect::Doubleread)
    }
    #[doc = "Single Fault Injection (at bit selected by FLT1PTR) for Writes"]
    #[inline(always)]
    pub fn singlewrite(self) -> &'a mut crate::W<REG> {
        self.variant(Fltmdselect::Singlewrite)
    }
    #[doc = "Double Fault Injection for Writes"]
    #[inline(always)]
    pub fn doublewrite(self) -> &'a mut crate::W<REG> {
        self.variant(Fltmdselect::Doublewrite)
    }
}
impl R {
    #[doc = "Bits 8:10 - ECC/Parity Control Fault bits"]
    #[inline(always)]
    pub fn ctlflt(&self) -> CtlfltR {
        CtlfltR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Fault Mode Control"]
    #[inline(always)]
    pub fn fltmd(&self) -> FltmdR {
        FltmdR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - ECC/Parity Control Fault bits"]
    #[inline(always)]
    pub fn ctlflt(&mut self) -> CtlfltW<'_, FfltmodeSpec> {
        CtlfltW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Fault Mode Control"]
    #[inline(always)]
    pub fn fltmd(&mut self) -> FltmdW<'_, FfltmodeSpec> {
        FltmdW::new(self, 12)
    }
}
#[doc = "Flash ECC Fault Mode REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfltmodeSpec;
impl crate::RegisterSpec for FfltmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffltmode::R`](R) reader structure"]
impl crate::Readable for FfltmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`ffltmode::W`](W) writer structure"]
impl crate::Writable for FfltmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FFLTMODE to value 0"]
impl crate::Resettable for FfltmodeSpec {}

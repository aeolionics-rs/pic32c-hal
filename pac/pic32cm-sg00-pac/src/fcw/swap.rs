#[doc = "Register `SWAP` reader"]
pub type R = crate::R<SwapSpec>;
#[doc = "Register `SWAP` writer"]
pub type W = crate::W<SwapSpec>;
#[doc = "BFM Swap Status/Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfswapselect {
    #[doc = "0: Upper and Lower BFM Regions are NOT Swapped"]
    Unswap = 0,
    #[doc = "1: Upper and Lower BFM Regions are Swapped"]
    Swap = 1,
}
impl From<Bfswapselect> for bool {
    #[inline(always)]
    fn from(variant: Bfswapselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFSWAP` reader - BFM Swap Status/Control Bit"]
pub type BfswapR = crate::BitReader<Bfswapselect>;
impl BfswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfswapselect {
        match self.bits {
            false => Bfswapselect::Unswap,
            true => Bfswapselect::Swap,
        }
    }
    #[doc = "Upper and Lower BFM Regions are NOT Swapped"]
    #[inline(always)]
    pub fn is_unswap(&self) -> bool {
        *self == Bfswapselect::Unswap
    }
    #[doc = "Upper and Lower BFM Regions are Swapped"]
    #[inline(always)]
    pub fn is_swap(&self) -> bool {
        *self == Bfswapselect::Swap
    }
}
#[doc = "Field `BFSWAP` writer - BFM Swap Status/Control Bit"]
pub type BfswapW<'a, REG> = crate::BitWriter<'a, REG, Bfswapselect>;
impl<'a, REG> BfswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Upper and Lower BFM Regions are NOT Swapped"]
    #[inline(always)]
    pub fn unswap(self) -> &'a mut crate::W<REG> {
        self.variant(Bfswapselect::Unswap)
    }
    #[doc = "Upper and Lower BFM Regions are Swapped"]
    #[inline(always)]
    pub fn swap(self) -> &'a mut crate::W<REG> {
        self.variant(Bfswapselect::Swap)
    }
}
#[doc = "BFM Swap Lock Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfslockselect {
    #[doc = "0: BFSLOCK and BFSWAP can be written"]
    Unlocked = 0,
    #[doc = "1: BFSLOCK and BFSWAP cannot be written"]
    Locked = 1,
}
impl From<Bfslockselect> for bool {
    #[inline(always)]
    fn from(variant: Bfslockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFSLOCK` reader - BFM Swap Lock Bit"]
pub type BfslockR = crate::BitReader<Bfslockselect>;
impl BfslockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfslockselect {
        match self.bits {
            false => Bfslockselect::Unlocked,
            true => Bfslockselect::Locked,
        }
    }
    #[doc = "BFSLOCK and BFSWAP can be written"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Bfslockselect::Unlocked
    }
    #[doc = "BFSLOCK and BFSWAP cannot be written"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Bfslockselect::Locked
    }
}
#[doc = "Field `BFSLOCK` writer - BFM Swap Lock Bit"]
pub type BfslockW<'a, REG> = crate::BitWriter<'a, REG, Bfslockselect>;
impl<'a, REG> BfslockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BFSLOCK and BFSWAP can be written"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Bfslockselect::Unlocked)
    }
    #[doc = "BFSLOCK and BFSWAP cannot be written"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Bfslockselect::Locked)
    }
}
#[doc = "PFM Swap Status/Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfswapselect {
    #[doc = "0: Upper and Lower PFM Regions are NOT Swapped"]
    Unswap = 0,
    #[doc = "1: Upper and Lower PFM Regions are Swapped"]
    Swap = 1,
}
impl From<Pfswapselect> for bool {
    #[inline(always)]
    fn from(variant: Pfswapselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFSWAP` reader - PFM Swap Status/Control Bit"]
pub type PfswapR = crate::BitReader<Pfswapselect>;
impl PfswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfswapselect {
        match self.bits {
            false => Pfswapselect::Unswap,
            true => Pfswapselect::Swap,
        }
    }
    #[doc = "Upper and Lower PFM Regions are NOT Swapped"]
    #[inline(always)]
    pub fn is_unswap(&self) -> bool {
        *self == Pfswapselect::Unswap
    }
    #[doc = "Upper and Lower PFM Regions are Swapped"]
    #[inline(always)]
    pub fn is_swap(&self) -> bool {
        *self == Pfswapselect::Swap
    }
}
#[doc = "Field `PFSWAP` writer - PFM Swap Status/Control Bit"]
pub type PfswapW<'a, REG> = crate::BitWriter<'a, REG, Pfswapselect>;
impl<'a, REG> PfswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Upper and Lower PFM Regions are NOT Swapped"]
    #[inline(always)]
    pub fn unswap(self) -> &'a mut crate::W<REG> {
        self.variant(Pfswapselect::Unswap)
    }
    #[doc = "Upper and Lower PFM Regions are Swapped"]
    #[inline(always)]
    pub fn swap(self) -> &'a mut crate::W<REG> {
        self.variant(Pfswapselect::Swap)
    }
}
#[doc = "PFM Swap Lock Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfslockselect {
    #[doc = "0: PFSLOCK and PFSWAP can be written"]
    Unlocked = 0,
    #[doc = "1: PFSLOCK and PFSWAP cannot be written"]
    Locked = 1,
}
impl From<Pfslockselect> for bool {
    #[inline(always)]
    fn from(variant: Pfslockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFSLOCK` reader - PFM Swap Lock Bit"]
pub type PfslockR = crate::BitReader<Pfslockselect>;
impl PfslockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfslockselect {
        match self.bits {
            false => Pfslockselect::Unlocked,
            true => Pfslockselect::Locked,
        }
    }
    #[doc = "PFSLOCK and PFSWAP can be written"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Pfslockselect::Unlocked
    }
    #[doc = "PFSLOCK and PFSWAP cannot be written"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Pfslockselect::Locked
    }
}
#[doc = "Field `PFSLOCK` writer - PFM Swap Lock Bit"]
pub type PfslockW<'a, REG> = crate::BitWriter<'a, REG, Pfslockselect>;
impl<'a, REG> PfslockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PFSLOCK and PFSWAP can be written"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pfslockselect::Unlocked)
    }
    #[doc = "PFSLOCK and PFSWAP cannot be written"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Pfslockselect::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - BFM Swap Status/Control Bit"]
    #[inline(always)]
    pub fn bfswap(&self) -> BfswapR {
        BfswapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BFM Swap Lock Bit"]
    #[inline(always)]
    pub fn bfslock(&self) -> BfslockR {
        BfslockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - PFM Swap Status/Control Bit"]
    #[inline(always)]
    pub fn pfswap(&self) -> PfswapR {
        PfswapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PFM Swap Lock Bit"]
    #[inline(always)]
    pub fn pfslock(&self) -> PfslockR {
        PfslockR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BFM Swap Status/Control Bit"]
    #[inline(always)]
    pub fn bfswap(&mut self) -> BfswapW<'_, SwapSpec> {
        BfswapW::new(self, 0)
    }
    #[doc = "Bit 1 - BFM Swap Lock Bit"]
    #[inline(always)]
    pub fn bfslock(&mut self) -> BfslockW<'_, SwapSpec> {
        BfslockW::new(self, 1)
    }
    #[doc = "Bit 8 - PFM Swap Status/Control Bit"]
    #[inline(always)]
    pub fn pfswap(&mut self) -> PfswapW<'_, SwapSpec> {
        PfswapW::new(self, 8)
    }
    #[doc = "Bit 9 - PFM Swap Lock Bit"]
    #[inline(always)]
    pub fn pfslock(&mut self) -> PfslockW<'_, SwapSpec> {
        PfslockW::new(self, 9)
    }
}
#[doc = "NVM Panel Swap REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`swap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwapSpec;
impl crate::RegisterSpec for SwapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swap::R`](R) reader structure"]
impl crate::Readable for SwapSpec {}
#[doc = "`write(|w| ..)` method takes [`swap::W`](W) writer structure"]
impl crate::Writable for SwapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWAP to value 0"]
impl crate::Resettable for SwapSpec {}

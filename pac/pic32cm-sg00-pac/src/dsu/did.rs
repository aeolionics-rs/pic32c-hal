#[doc = "Register `DID` reader"]
pub type R = crate::R<DidSpec>;
#[doc = "Register `DID` writer"]
pub type W = crate::W<DidSpec>;
#[doc = "Field `FV` reader - Fixed Value of 1"]
pub type FvR = crate::BitReader;
#[doc = "Field `FV` writer - Fixed Value of 1"]
pub type FvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MANID` reader - JEDEC Manufacture ID"]
pub type ManidR = crate::FieldReader<u16>;
#[doc = "Field `MANID` writer - JEDEC Manufacture ID"]
pub type ManidW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PNDID` reader - Part Number Device ID"]
pub type PndidR = crate::FieldReader;
#[doc = "Field `PNDID` writer - Part Number Device ID"]
pub type PndidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PNMID` reader - Part Number Mask ID"]
pub type PnmidR = crate::FieldReader;
#[doc = "Field `PNMID` writer - Part Number Mask ID"]
pub type PnmidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VER` reader - Version Code"]
pub type VerR = crate::FieldReader;
#[doc = "Field `VER` writer - Version Code"]
pub type VerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Fixed Value of 1"]
    #[inline(always)]
    pub fn fv(&self) -> FvR {
        FvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - JEDEC Manufacture ID"]
    #[inline(always)]
    pub fn manid(&self) -> ManidR {
        ManidR::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:19 - Part Number Device ID"]
    #[inline(always)]
    pub fn pndid(&self) -> PndidR {
        PndidR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:27 - Part Number Mask ID"]
    #[inline(always)]
    pub fn pnmid(&self) -> PnmidR {
        PnmidR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - Version Code"]
    #[inline(always)]
    pub fn ver(&self) -> VerR {
        VerR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Fixed Value of 1"]
    #[inline(always)]
    pub fn fv(&mut self) -> FvW<'_, DidSpec> {
        FvW::new(self, 0)
    }
    #[doc = "Bits 1:11 - JEDEC Manufacture ID"]
    #[inline(always)]
    pub fn manid(&mut self) -> ManidW<'_, DidSpec> {
        ManidW::new(self, 1)
    }
    #[doc = "Bits 12:19 - Part Number Device ID"]
    #[inline(always)]
    pub fn pndid(&mut self) -> PndidW<'_, DidSpec> {
        PndidW::new(self, 12)
    }
    #[doc = "Bits 20:27 - Part Number Mask ID"]
    #[inline(always)]
    pub fn pnmid(&mut self) -> PnmidW<'_, DidSpec> {
        PnmidW::new(self, 20)
    }
    #[doc = "Bits 28:31 - Version Code"]
    #[inline(always)]
    pub fn ver(&mut self) -> VerW<'_, DidSpec> {
        VerW::new(self, 28)
    }
}
#[doc = "Device Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`did::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`did::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DidSpec;
impl crate::RegisterSpec for DidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`did::R`](R) reader structure"]
impl crate::Readable for DidSpec {}
#[doc = "`write(|w| ..)` method takes [`did::W`](W) writer structure"]
impl crate::Writable for DidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DID to value 0x0ac0_8053"]
impl crate::Resettable for DidSpec {
    const RESET_VALUE: u32 = 0x0ac0_8053;
}

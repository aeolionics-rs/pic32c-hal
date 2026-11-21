#[doc = "Register `CHINTENCLR` reader"]
pub type R = crate::R<ChintenclrSpec>;
#[doc = "Register `CHINTENCLR` writer"]
pub type W = crate::W<ChintenclrSpec>;
#[doc = "Field `TERR` reader - Transfer Error Interrupt Enable"]
pub type TerrR = crate::BitReader;
#[doc = "Field `TERR` writer - Transfer Error Interrupt Enable"]
pub type TerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TCMPL` reader - Transfer Complete Interrupt Enable"]
pub type TcmplR = crate::BitReader;
#[doc = "Field `TCMPL` writer - Transfer Complete Interrupt Enable"]
pub type TcmplW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SUSP` reader - Suspend Interrupt Enable"]
pub type SuspR = crate::BitReader;
#[doc = "Field `SUSP` writer - Suspend Interrupt Enable"]
pub type SuspW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terr(&self) -> TerrR {
        TerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcmpl(&self) -> TcmplR {
        TcmplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terr(&mut self) -> TerrW<'_, ChintenclrSpec> {
        TerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcmpl(&mut self) -> TcmplW<'_, ChintenclrSpec> {
        TcmplW::new(self, 1)
    }
    #[doc = "Bit 2 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<'_, ChintenclrSpec> {
        SuspW::new(self, 2)
    }
}
#[doc = "Channel Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChintenclrSpec;
impl crate::RegisterSpec for ChintenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chintenclr::R`](R) reader structure"]
impl crate::Readable for ChintenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`chintenclr::W`](W) writer structure"]
impl crate::Writable for ChintenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x07;
}
#[doc = "`reset()` method sets CHINTENCLR to value 0"]
impl crate::Resettable for ChintenclrSpec {}

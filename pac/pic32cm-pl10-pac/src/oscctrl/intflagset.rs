#[doc = "Register `INTFLAGSET` reader"]
pub type R = crate::R<IntflagsetSpec>;
#[doc = "Register `INTFLAGSET` writer"]
pub type W = crate::W<IntflagsetSpec>;
#[doc = "Field `OSCHFRDY` reader - OSCHF is Ready"]
pub type OschfrdyR = crate::BitReader;
#[doc = "Field `OSCHFRDY` writer - OSCHF is Ready"]
pub type OschfrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 4 - OSCHF is Ready"]
    #[inline(always)]
    pub fn oschfrdy(&self) -> OschfrdyR {
        OschfrdyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - OSCHF is Ready"]
    #[inline(always)]
    pub fn oschfrdy(&mut self) -> OschfrdyW<'_, IntflagsetSpec> {
        OschfrdyW::new(self, 4)
    }
}
#[doc = "Interrupt Flag Software Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagsetSpec;
impl crate::RegisterSpec for IntflagsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagset::R`](R) reader structure"]
impl crate::Readable for IntflagsetSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagset::W`](W) writer structure"]
impl crate::Writable for IntflagsetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x10;
}
#[doc = "`reset()` method sets INTFLAGSET to value 0"]
impl crate::Resettable for IntflagsetSpec {}

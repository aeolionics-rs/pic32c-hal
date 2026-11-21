#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `OSCHFRDY` reader - OSCHF Ready Interrupt Enable"]
pub type OschfrdyR = crate::BitReader;
#[doc = "Field `OSCHFRDY` writer - OSCHF Ready Interrupt Enable"]
pub type OschfrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 4 - OSCHF Ready Interrupt Enable"]
    #[inline(always)]
    pub fn oschfrdy(&self) -> OschfrdyR {
        OschfrdyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - OSCHF Ready Interrupt Enable"]
    #[inline(always)]
    pub fn oschfrdy(&mut self) -> OschfrdyW<'_, IntenclrSpec> {
        OschfrdyW::new(self, 4)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x10;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}

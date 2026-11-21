#[doc = "Register `RESTRICT1` reader"]
pub type R = crate::R<Restrict1Spec>;
#[doc = "Register `RESTRICT1` writer"]
pub type W = crate::W<Restrict1Spec>;
#[doc = "Field `RESTRICT_CONTEXT_1` reader - Restrict User Context Writing a 1 in a bit position prevents other PUF keys to be generated with this context bit set. NOTE: Refer to the EIP datasheet for the function of the restrict_user_context signals"]
pub type RestrictContext1R = crate::FieldReader<u32>;
#[doc = "Field `RESTRICT_CONTEXT_1` writer - Restrict User Context Writing a 1 in a bit position prevents other PUF keys to be generated with this context bit set. NOTE: Refer to the EIP datasheet for the function of the restrict_user_context signals"]
pub type RestrictContext1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Restrict User Context Writing a 1 in a bit position prevents other PUF keys to be generated with this context bit set. NOTE: Refer to the EIP datasheet for the function of the restrict_user_context signals"]
    #[inline(always)]
    pub fn restrict_context_1(&self) -> RestrictContext1R {
        RestrictContext1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Restrict User Context Writing a 1 in a bit position prevents other PUF keys to be generated with this context bit set. NOTE: Refer to the EIP datasheet for the function of the restrict_user_context signals"]
    #[inline(always)]
    pub fn restrict_context_1(&mut self) -> RestrictContext1W<'_, Restrict1Spec> {
        RestrictContext1W::new(self, 0)
    }
}
#[doc = "Restrict User Context 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`restrict1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`restrict1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Restrict1Spec;
impl crate::RegisterSpec for Restrict1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`restrict1::R`](R) reader structure"]
impl crate::Readable for Restrict1Spec {}
#[doc = "`write(|w| ..)` method takes [`restrict1::W`](W) writer structure"]
impl crate::Writable for Restrict1Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets RESTRICT1 to value 0"]
impl crate::Resettable for Restrict1Spec {}

#[doc = "Register `RESTRICT0` reader"]
pub type R = crate::R<Restrict0Spec>;
#[doc = "Register `RESTRICT0` writer"]
pub type W = crate::W<Restrict0Spec>;
#[doc = "Field `RESTRICT_CONTEXT_0` reader - Restrict User Context Writing a 1 in a bit position prevents other PUF keys to be generated with this context bit set. NOTE: Refer to the EIP datasheet for the function of the restrict_user_context signals"]
pub type RestrictContext0R = crate::FieldReader<u32>;
#[doc = "Field `RESTRICT_CONTEXT_0` writer - Restrict User Context Writing a 1 in a bit position prevents other PUF keys to be generated with this context bit set. NOTE: Refer to the EIP datasheet for the function of the restrict_user_context signals"]
pub type RestrictContext0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Restrict User Context Writing a 1 in a bit position prevents other PUF keys to be generated with this context bit set. NOTE: Refer to the EIP datasheet for the function of the restrict_user_context signals"]
    #[inline(always)]
    pub fn restrict_context_0(&self) -> RestrictContext0R {
        RestrictContext0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Restrict User Context Writing a 1 in a bit position prevents other PUF keys to be generated with this context bit set. NOTE: Refer to the EIP datasheet for the function of the restrict_user_context signals"]
    #[inline(always)]
    pub fn restrict_context_0(&mut self) -> RestrictContext0W<'_, Restrict0Spec> {
        RestrictContext0W::new(self, 0)
    }
}
#[doc = "Restrict User Context 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`restrict0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`restrict0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Restrict0Spec;
impl crate::RegisterSpec for Restrict0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`restrict0::R`](R) reader structure"]
impl crate::Readable for Restrict0Spec {}
#[doc = "`write(|w| ..)` method takes [`restrict0::W`](W) writer structure"]
impl crate::Writable for Restrict0Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets RESTRICT0 to value 0"]
impl crate::Resettable for Restrict0Spec {}

#[doc = "Register `anti_tamper__SOFTINFO` reader"]
pub type R = crate::R<AntiTamper_SoftinfoSpec>;
#[doc = "Register `anti_tamper__SOFTINFO` writer"]
pub type W = crate::W<AntiTamper_SoftinfoSpec>;
#[doc = "Field `SOFTINFO` reader - Register for software purpose only."]
pub type SoftinfoR = crate::FieldReader<u32>;
#[doc = "Field `SOFTINFO` writer - Register for software purpose only."]
pub type SoftinfoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register for software purpose only."]
    #[inline(always)]
    pub fn softinfo(&self) -> SoftinfoR {
        SoftinfoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register for software purpose only."]
    #[inline(always)]
    pub fn softinfo(&mut self) -> SoftinfoW<'_, AntiTamper_SoftinfoSpec> {
        SoftinfoW::new(self, 0)
    }
}
#[doc = "Register for software purpose only.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__softinfo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__softinfo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AntiTamper_SoftinfoSpec;
impl crate::RegisterSpec for AntiTamper_SoftinfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anti_tamper__softinfo::R`](R) reader structure"]
impl crate::Readable for AntiTamper_SoftinfoSpec {}
#[doc = "`write(|w| ..)` method takes [`anti_tamper__softinfo::W`](W) writer structure"]
impl crate::Writable for AntiTamper_SoftinfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets anti_tamper__SOFTINFO to value 0"]
impl crate::Resettable for AntiTamper_SoftinfoSpec {}

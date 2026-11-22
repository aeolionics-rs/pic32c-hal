#[doc = "Register `BAUD` reader"]
pub type R = crate::R<BaudSpec>;
#[doc = "Register `BAUD` writer"]
pub type W = crate::W<BaudSpec>;
#[doc = "Field `BAUD` reader - Master Baud Rate"]
pub type BaudR = crate::FieldReader;
#[doc = "Field `BAUD` writer - Master Baud Rate"]
pub type BaudW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BAUDLOW` reader - Master Baud Rate Low"]
pub type BaudlowR = crate::FieldReader;
#[doc = "Field `BAUDLOW` writer - Master Baud Rate Low"]
pub type BaudlowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Master Baud Rate"]
    #[inline(always)]
    pub fn baud(&self) -> BaudR {
        BaudR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Master Baud Rate Low"]
    #[inline(always)]
    pub fn baudlow(&self) -> BaudlowR {
        BaudlowR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Master Baud Rate"]
    #[inline(always)]
    pub fn baud(&mut self) -> BaudW<'_, BaudSpec> {
        BaudW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Master Baud Rate Low"]
    #[inline(always)]
    pub fn baudlow(&mut self) -> BaudlowW<'_, BaudSpec> {
        BaudlowW::new(self, 8)
    }
}
#[doc = "I2CM Baud Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudSpec;
impl crate::RegisterSpec for BaudSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`baud::R`](R) reader structure"]
impl crate::Readable for BaudSpec {}
#[doc = "`write(|w| ..)` method takes [`baud::W`](W) writer structure"]
impl crate::Writable for BaudSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BaudSpec {}

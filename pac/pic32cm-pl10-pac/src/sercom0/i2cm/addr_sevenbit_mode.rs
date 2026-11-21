#[doc = "Register `ADDR_SEVENBIT_MODE` reader"]
pub type R = crate::R<AddrSevenbitModeSpec>;
#[doc = "Register `ADDR_SEVENBIT_MODE` writer"]
pub type W = crate::W<AddrSevenbitModeSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, AddrSevenbitModeSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "I2CM Address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_sevenbit_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_sevenbit_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSevenbitModeSpec;
impl crate::RegisterSpec for AddrSevenbitModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_sevenbit_mode::R`](R) reader structure"]
impl crate::Readable for AddrSevenbitModeSpec {}
#[doc = "`write(|w| ..)` method takes [`addr_sevenbit_mode::W`](W) writer structure"]
impl crate::Writable for AddrSevenbitModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR_SEVENBIT_MODE to value 0"]
impl crate::Resettable for AddrSevenbitModeSpec {}

#[doc = "Register `ADDR_SEVENBIT_MODE` reader"]
pub type R = crate::R<AddrSevenbitModeSpec>;
#[doc = "Register `ADDR_SEVENBIT_MODE` writer"]
pub type W = crate::W<AddrSevenbitModeSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADDRMASK` reader - Address Mask"]
pub type AddrmaskR = crate::FieldReader;
#[doc = "Field `ADDRMASK` writer - Address Mask"]
pub type AddrmaskW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 17:23 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> AddrmaskR {
        AddrmaskR::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, AddrSevenbitModeSpec> {
        AddrW::new(self, 1)
    }
    #[doc = "Bits 17:23 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&mut self) -> AddrmaskW<'_, AddrSevenbitModeSpec> {
        AddrmaskW::new(self, 17)
    }
}
#[doc = "I2CS Address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_sevenbit_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_sevenbit_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

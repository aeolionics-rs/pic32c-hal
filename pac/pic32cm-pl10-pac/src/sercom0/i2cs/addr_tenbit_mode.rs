#[doc = "Register `ADDR_TENBIT_MODE` reader"]
pub type R = crate::R<AddrTenbitModeSpec>;
#[doc = "Register `ADDR_TENBIT_MODE` writer"]
pub type W = crate::W<AddrTenbitModeSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ADDRMASK` reader - Address Mask"]
pub type AddrmaskR = crate::FieldReader<u16>;
#[doc = "Field `ADDRMASK` writer - Address Mask"]
pub type AddrmaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 17:26 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> AddrmaskR {
        AddrmaskR::new(((self.bits >> 17) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, AddrTenbitModeSpec> {
        AddrW::new(self, 1)
    }
    #[doc = "Bits 17:26 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&mut self) -> AddrmaskW<'_, AddrTenbitModeSpec> {
        AddrmaskW::new(self, 17)
    }
}
#[doc = "I2CS Address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_tenbit_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_tenbit_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrTenbitModeSpec;
impl crate::RegisterSpec for AddrTenbitModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_tenbit_mode::R`](R) reader structure"]
impl crate::Readable for AddrTenbitModeSpec {}
#[doc = "`write(|w| ..)` method takes [`addr_tenbit_mode::W`](W) writer structure"]
impl crate::Writable for AddrTenbitModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR_TENBIT_MODE to value 0"]
impl crate::Resettable for AddrTenbitModeSpec {}

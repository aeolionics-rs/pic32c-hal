#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `GENCEN` reader - General Call Address Enable"]
pub type GencenR = crate::BitReader;
#[doc = "Field `GENCEN` writer - General Call Address Enable"]
pub type GencenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TENBITEN` reader - Ten Bit Addressing Enable"]
pub type TenbitenR = crate::BitReader;
#[doc = "Field `TENBITEN` writer - Ten Bit Addressing Enable"]
pub type TenbitenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline(always)]
    pub fn gencen(&self) -> GencenR {
        GencenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    pub fn tenbiten(&self) -> TenbitenR {
        TenbitenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline(always)]
    pub fn gencen(&mut self) -> GencenW<'_, AddrSpec> {
        GencenW::new(self, 0)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    pub fn tenbiten(&mut self) -> TenbitenW<'_, AddrSpec> {
        TenbitenW::new(self, 15)
    }
}
#[doc = "I2CS Address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {}

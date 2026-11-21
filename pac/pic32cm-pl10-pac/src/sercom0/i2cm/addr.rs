#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `LENEN` reader - Transfer Length Enable"]
pub type LenenR = crate::BitReader;
#[doc = "Field `LENEN` writer - Transfer Length Enable"]
pub type LenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TENBITEN` reader - Ten Bit Addressing Enable"]
pub type TenbitenR = crate::BitReader;
#[doc = "Field `TENBITEN` writer - Ten Bit Addressing Enable"]
pub type TenbitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN` reader - Transaction Length"]
pub type LenR = crate::FieldReader;
#[doc = "Field `LEN` writer - Transaction Length"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 13 - Transfer Length Enable"]
    #[inline(always)]
    pub fn lenen(&self) -> LenenR {
        LenenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    pub fn tenbiten(&self) -> TenbitenR {
        TenbitenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Transaction Length"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 13 - Transfer Length Enable"]
    #[inline(always)]
    pub fn lenen(&mut self) -> LenenW<'_, AddrSpec> {
        LenenW::new(self, 13)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    pub fn tenbiten(&mut self) -> TenbitenW<'_, AddrSpec> {
        TenbitenW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Transaction Length"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<'_, AddrSpec> {
        LenW::new(self, 16)
    }
}
#[doc = "I2CM Address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

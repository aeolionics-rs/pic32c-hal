#[doc = "Register `QK_DATA_DEST` reader"]
pub type R = crate::R<QkDataDestSpec>;
#[doc = "Register `QK_DATA_DEST` writer"]
pub type W = crate::W<QkDataDestSpec>;
#[doc = "Field `qk_dest_dor` reader - Data will be made available via the QK_DOR register"]
pub type QkDestDorR = crate::BitReader;
#[doc = "Field `qk_dest_dor` writer - Data will be made available via the QK_DOR register"]
pub type QkDestDorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_dest_so` reader - Data will be made available via the SO interface"]
pub type QkDestSoR = crate::BitReader;
#[doc = "Field `qk_dest_so` writer - Data will be made available via the SO interface"]
pub type QkDestSoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data will be made available via the QK_DOR register"]
    #[inline(always)]
    pub fn qk_dest_dor(&self) -> QkDestDorR {
        QkDestDorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data will be made available via the SO interface"]
    #[inline(always)]
    pub fn qk_dest_so(&self) -> QkDestSoR {
        QkDestSoR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data will be made available via the QK_DOR register"]
    #[inline(always)]
    pub fn qk_dest_dor(&mut self) -> QkDestDorW<'_, QkDataDestSpec> {
        QkDestDorW::new(self, 0)
    }
    #[doc = "Bit 1 - Data will be made available via the SO interface"]
    #[inline(always)]
    pub fn qk_dest_so(&mut self) -> QkDestSoW<'_, QkDataDestSpec> {
        QkDestSoW::new(self, 1)
    }
}
#[doc = "Data Destination register; The Data Destination register defines via which interface data is made available by QuiddiKey. Only one bit at a time may be 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_data_dest::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_data_dest::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkDataDestSpec;
impl crate::RegisterSpec for QkDataDestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_data_dest::R`](R) reader structure"]
impl crate::Readable for QkDataDestSpec {}
#[doc = "`write(|w| ..)` method takes [`qk_data_dest::W`](W) writer structure"]
impl crate::Writable for QkDataDestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QK_DATA_DEST to value 0"]
impl crate::Resettable for QkDataDestSpec {}

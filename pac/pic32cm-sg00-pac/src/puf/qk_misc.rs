#[doc = "Register `QK_MISC` reader"]
pub type R = crate::R<QkMiscSpec>;
#[doc = "Register `QK_MISC` writer"]
pub type W = crate::W<QkMiscSpec>;
#[doc = "Field `qk_data_endianness` reader - Defines the endianness of data in QK_DIR and QK_DOR: 0: Little endian 1: Big endian (default)"]
pub type QkDataEndiannessR = crate::BitReader;
#[doc = "Field `qk_data_endianness` writer - Defines the endianness of data in QK_DIR and QK_DOR: 0: Little endian 1: Big endian (default)"]
pub type QkDataEndiannessW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Defines the endianness of data in QK_DIR and QK_DOR: 0: Little endian 1: Big endian (default)"]
    #[inline(always)]
    pub fn qk_data_endianness(&self) -> QkDataEndiannessR {
        QkDataEndiannessR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Defines the endianness of data in QK_DIR and QK_DOR: 0: Little endian 1: Big endian (default)"]
    #[inline(always)]
    pub fn qk_data_endianness(&mut self) -> QkDataEndiannessW<'_, QkMiscSpec> {
        QkDataEndiannessW::new(self, 0)
    }
}
#[doc = "Miscellaneous register; The Miscellaneous register defines miscellaneous settings for QuiddiKey. These usually need to be configured only once after power up orreset.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkMiscSpec;
impl crate::RegisterSpec for QkMiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_misc::R`](R) reader structure"]
impl crate::Readable for QkMiscSpec {}
#[doc = "`write(|w| ..)` method takes [`qk_misc::W`](W) writer structure"]
impl crate::Writable for QkMiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QK_MISC to value 0x01"]
impl crate::Resettable for QkMiscSpec {
    const RESET_VALUE: u32 = 0x01;
}

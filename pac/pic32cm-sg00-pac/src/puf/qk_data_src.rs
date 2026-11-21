#[doc = "Register `QK_DATA_SRC` reader"]
pub type R = crate::R<QkDataSrcSpec>;
#[doc = "Register `QK_DATA_SRC` writer"]
pub type W = crate::W<QkDataSrcSpec>;
#[doc = "Field `qk_src_dir` reader - Data must be provided via the QK_DIR register"]
pub type QkSrcDirR = crate::BitReader;
#[doc = "Field `qk_src_dir` writer - Data must be provided via the QK_DIR register"]
pub type QkSrcDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qk_src_si` reader - Data must be provided via the SI interface"]
pub type QkSrcSiR = crate::BitReader;
#[doc = "Field `qk_src_si` writer - Data must be provided via the SI interface"]
pub type QkSrcSiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data must be provided via the QK_DIR register"]
    #[inline(always)]
    pub fn qk_src_dir(&self) -> QkSrcDirR {
        QkSrcDirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data must be provided via the SI interface"]
    #[inline(always)]
    pub fn qk_src_si(&self) -> QkSrcSiR {
        QkSrcSiR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data must be provided via the QK_DIR register"]
    #[inline(always)]
    pub fn qk_src_dir(&mut self) -> QkSrcDirW<'_, QkDataSrcSpec> {
        QkSrcDirW::new(self, 0)
    }
    #[doc = "Bit 1 - Data must be provided via the SI interface"]
    #[inline(always)]
    pub fn qk_src_si(&mut self) -> QkSrcSiW<'_, QkDataSrcSpec> {
        QkSrcSiW::new(self, 1)
    }
}
#[doc = "Data Source register; The Data Source register defines via which interface data must be provided to QuiddiKey. Combinations of bits may be 1 at a time.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_data_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_data_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QkDataSrcSpec;
impl crate::RegisterSpec for QkDataSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qk_data_src::R`](R) reader structure"]
impl crate::Readable for QkDataSrcSpec {}
#[doc = "`write(|w| ..)` method takes [`qk_data_src::W`](W) writer structure"]
impl crate::Writable for QkDataSrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QK_DATA_SRC to value 0"]
impl crate::Resettable for QkDataSrcSpec {}

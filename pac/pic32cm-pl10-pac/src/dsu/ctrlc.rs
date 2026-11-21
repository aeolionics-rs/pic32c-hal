#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `DBGRW0` reader - CPU0 Debug R/W"]
pub type Dbgrw0R = crate::BitReader;
#[doc = "Field `DBGRW0` writer - CPU0 Debug R/W"]
pub type Dbgrw0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU0 Debug R/W"]
    #[inline(always)]
    pub fn dbgrw0(&self) -> Dbgrw0R {
        Dbgrw0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU0 Debug R/W"]
    #[inline(always)]
    pub fn dbgrw0(&mut self) -> Dbgrw0W<'_, CtrlcSpec> {
        Dbgrw0W::new(self, 0)
    }
}
#[doc = "Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {}

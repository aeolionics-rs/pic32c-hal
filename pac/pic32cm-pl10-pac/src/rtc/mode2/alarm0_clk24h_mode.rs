#[doc = "Register `ALARM0_CLK24H_MODE` reader"]
pub type R = crate::R<Alarm0Clk24hModeSpec>;
#[doc = "Register `ALARM0_CLK24H_MODE` writer"]
pub type W = crate::W<Alarm0Clk24hModeSpec>;
#[doc = "Field `HOUR` reader - Hour"]
pub type HourR = crate::FieldReader;
#[doc = "Field `HOUR` writer - Hour"]
pub type HourW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    pub fn hour(&self) -> HourR {
        HourR::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    pub fn hour(&mut self) -> HourW<'_, Alarm0Clk24hModeSpec> {
        HourW::new(self, 12)
    }
}
#[doc = "MODE2 Alarm 0 Value\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0_clk24h_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0_clk24h_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alarm0Clk24hModeSpec;
impl crate::RegisterSpec for Alarm0Clk24hModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0_clk24h_mode::R`](R) reader structure"]
impl crate::Readable for Alarm0Clk24hModeSpec {}
#[doc = "`write(|w| ..)` method takes [`alarm0_clk24h_mode::W`](W) writer structure"]
impl crate::Writable for Alarm0Clk24hModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALARM0_CLK24H_MODE to value 0"]
impl crate::Resettable for Alarm0Clk24hModeSpec {}

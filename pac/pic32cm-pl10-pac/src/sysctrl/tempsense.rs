#[doc = "Register `TEMPSENSE` reader"]
pub type R = crate::R<TempsenseSpec>;
#[doc = "Field `TEMPSENSE0` reader - Temperature Sensor Calibration Slope"]
pub type Tempsense0R = crate::FieldReader<u16>;
#[doc = "Field `TEMPSENSE1` reader - Temperature Sensor Calibration Offset"]
pub type Tempsense1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature Sensor Calibration Slope"]
    #[inline(always)]
    pub fn tempsense0(&self) -> Tempsense0R {
        Tempsense0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Temperature Sensor Calibration Offset"]
    #[inline(always)]
    pub fn tempsense1(&self) -> Tempsense1R {
        Tempsense1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Temperature Sensor Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`tempsense::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempsenseSpec;
impl crate::RegisterSpec for TempsenseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tempsense::R`](R) reader structure"]
impl crate::Readable for TempsenseSpec {}
#[doc = "`reset()` method sets TEMPSENSE to value 0"]
impl crate::Resettable for TempsenseSpec {}

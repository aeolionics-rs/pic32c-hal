#[doc = "Register `CHID` reader"]
pub type R = crate::R<ChidSpec>;
#[doc = "Register `CHID` writer"]
pub type W = crate::W<ChidSpec>;
#[doc = "Channel ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idselect {
    #[doc = "0: DMA Channel 0"]
    Chn0 = 0,
    #[doc = "1: DMA Channel 1"]
    Chn1 = 1,
}
impl From<Idselect> for u8 {
    #[inline(always)]
    fn from(variant: Idselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idselect {
    type Ux = u8;
}
impl crate::IsEnum for Idselect {}
#[doc = "Field `ID` reader - Channel ID"]
pub type IdR = crate::FieldReader<Idselect>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idselect> {
        match self.bits {
            0 => Some(Idselect::Chn0),
            1 => Some(Idselect::Chn1),
            _ => None,
        }
    }
    #[doc = "DMA Channel 0"]
    #[inline(always)]
    pub fn is_chn0(&self) -> bool {
        *self == Idselect::Chn0
    }
    #[doc = "DMA Channel 1"]
    #[inline(always)]
    pub fn is_chn1(&self) -> bool {
        *self == Idselect::Chn1
    }
}
#[doc = "Field `ID` writer - Channel ID"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 5, Idselect>;
impl<'a, REG> IdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA Channel 0"]
    #[inline(always)]
    pub fn chn0(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Chn0)
    }
    #[doc = "DMA Channel 1"]
    #[inline(always)]
    pub fn chn1(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Chn1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Channel ID"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, ChidSpec> {
        IdW::new(self, 0)
    }
}
#[doc = "Channel ID\n\nYou can [`read`](crate::Reg::read) this register and get [`chid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChidSpec;
impl crate::RegisterSpec for ChidSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chid::R`](R) reader structure"]
impl crate::Readable for ChidSpec {}
#[doc = "`write(|w| ..)` method takes [`chid::W`](W) writer structure"]
impl crate::Writable for ChidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHID to value 0"]
impl crate::Resettable for ChidSpec {}

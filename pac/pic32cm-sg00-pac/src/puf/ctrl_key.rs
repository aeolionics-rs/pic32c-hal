#[doc = "Register `CTRL_KEY` reader"]
pub type R = crate::R<CtrlKeySpec>;
#[doc = "Register `CTRL_KEY` writer"]
pub type W = crate::W<CtrlKeySpec>;
#[doc = "Clear PUF Key Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrPufKeyselect {
    #[doc = "0: NO Affect"]
    ClrPufKey0 = 0,
    #[doc = "1: Clears the following outputs from this macro - puf_key, puf_valid, puf_index"]
    ClrPufKey1 = 1,
}
impl From<ClrPufKeyselect> for bool {
    #[inline(always)]
    fn from(variant: ClrPufKeyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PUF_KEY` reader - Clear PUF Key Control Bit"]
pub type ClrPufKeyR = crate::BitReader<ClrPufKeyselect>;
impl ClrPufKeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrPufKeyselect {
        match self.bits {
            false => ClrPufKeyselect::ClrPufKey0,
            true => ClrPufKeyselect::ClrPufKey1,
        }
    }
    #[doc = "NO Affect"]
    #[inline(always)]
    pub fn is_clr_puf_key_0(&self) -> bool {
        *self == ClrPufKeyselect::ClrPufKey0
    }
    #[doc = "Clears the following outputs from this macro - puf_key, puf_valid, puf_index"]
    #[inline(always)]
    pub fn is_clr_puf_key_1(&self) -> bool {
        *self == ClrPufKeyselect::ClrPufKey1
    }
}
#[doc = "Field `CLR_PUF_KEY` writer - Clear PUF Key Control Bit"]
pub type ClrPufKeyW<'a, REG> = crate::BitWriter<'a, REG, ClrPufKeyselect>;
impl<'a, REG> ClrPufKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO Affect"]
    #[inline(always)]
    pub fn clr_puf_key_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPufKeyselect::ClrPufKey0)
    }
    #[doc = "Clears the following outputs from this macro - puf_key, puf_valid, puf_index"]
    #[inline(always)]
    pub fn clr_puf_key_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPufKeyselect::ClrPufKey1)
    }
}
impl R {
    #[doc = "Bit 0 - Clear PUF Key Control Bit"]
    #[inline(always)]
    pub fn clr_puf_key(&self) -> ClrPufKeyR {
        ClrPufKeyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear PUF Key Control Bit"]
    #[inline(always)]
    pub fn clr_puf_key(&mut self) -> ClrPufKeyW<'_, CtrlKeySpec> {
        ClrPufKeyW::new(self, 0)
    }
}
#[doc = "KEY Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlKeySpec;
impl crate::RegisterSpec for CtrlKeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_key::R`](R) reader structure"]
impl crate::Readable for CtrlKeySpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_key::W`](W) writer structure"]
impl crate::Writable for CtrlKeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL_KEY to value 0"]
impl crate::Resettable for CtrlKeySpec {}

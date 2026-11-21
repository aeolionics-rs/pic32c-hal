#[doc = "Register `MRCR` reader"]
pub type R = crate::R<MrcrSpec>;
#[doc = "Register `MRCR` writer"]
pub type W = crate::W<MrcrSpec>;
#[doc = "Remap Command Bit for Master 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb0select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb0select> for bool {
    #[inline(always)]
    fn from(variant: Rcb0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB0` reader - Remap Command Bit for Master 0"]
pub type Rcb0R = crate::BitReader<Rcb0select>;
impl Rcb0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb0select {
        match self.bits {
            false => Rcb0select::Dis,
            true => Rcb0select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb0select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb0select::Ena
    }
}
#[doc = "Field `RCB0` writer - Remap Command Bit for Master 0"]
pub type Rcb0W<'a, REG> = crate::BitWriter<'a, REG, Rcb0select>;
impl<'a, REG> Rcb0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb0select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb0select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb1select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb1select> for bool {
    #[inline(always)]
    fn from(variant: Rcb1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB1` reader - Remap Command Bit for Master 1"]
pub type Rcb1R = crate::BitReader<Rcb1select>;
impl Rcb1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb1select {
        match self.bits {
            false => Rcb1select::Dis,
            true => Rcb1select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb1select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb1select::Ena
    }
}
#[doc = "Field `RCB1` writer - Remap Command Bit for Master 1"]
pub type Rcb1W<'a, REG> = crate::BitWriter<'a, REG, Rcb1select>;
impl<'a, REG> Rcb1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb1select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb1select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb2select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb2select> for bool {
    #[inline(always)]
    fn from(variant: Rcb2select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB2` reader - Remap Command Bit for Master 2"]
pub type Rcb2R = crate::BitReader<Rcb2select>;
impl Rcb2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb2select {
        match self.bits {
            false => Rcb2select::Dis,
            true => Rcb2select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb2select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb2select::Ena
    }
}
#[doc = "Field `RCB2` writer - Remap Command Bit for Master 2"]
pub type Rcb2W<'a, REG> = crate::BitWriter<'a, REG, Rcb2select>;
impl<'a, REG> Rcb2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb2select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb2select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb3select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb3select> for bool {
    #[inline(always)]
    fn from(variant: Rcb3select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB3` reader - Remap Command Bit for Master 3"]
pub type Rcb3R = crate::BitReader<Rcb3select>;
impl Rcb3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb3select {
        match self.bits {
            false => Rcb3select::Dis,
            true => Rcb3select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb3select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb3select::Ena
    }
}
#[doc = "Field `RCB3` writer - Remap Command Bit for Master 3"]
pub type Rcb3W<'a, REG> = crate::BitWriter<'a, REG, Rcb3select>;
impl<'a, REG> Rcb3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb3select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb3select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb4select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb4select> for bool {
    #[inline(always)]
    fn from(variant: Rcb4select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB4` reader - Remap Command Bit for Master 4"]
pub type Rcb4R = crate::BitReader<Rcb4select>;
impl Rcb4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb4select {
        match self.bits {
            false => Rcb4select::Dis,
            true => Rcb4select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb4select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb4select::Ena
    }
}
#[doc = "Field `RCB4` writer - Remap Command Bit for Master 4"]
pub type Rcb4W<'a, REG> = crate::BitWriter<'a, REG, Rcb4select>;
impl<'a, REG> Rcb4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb4select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb4select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb5select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb5select> for bool {
    #[inline(always)]
    fn from(variant: Rcb5select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB5` reader - Remap Command Bit for Master 5"]
pub type Rcb5R = crate::BitReader<Rcb5select>;
impl Rcb5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb5select {
        match self.bits {
            false => Rcb5select::Dis,
            true => Rcb5select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb5select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb5select::Ena
    }
}
#[doc = "Field `RCB5` writer - Remap Command Bit for Master 5"]
pub type Rcb5W<'a, REG> = crate::BitWriter<'a, REG, Rcb5select>;
impl<'a, REG> Rcb5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb5select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb5select::Ena)
    }
}
impl R {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    pub fn rcb0(&self) -> Rcb0R {
        Rcb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    pub fn rcb1(&self) -> Rcb1R {
        Rcb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    pub fn rcb2(&self) -> Rcb2R {
        Rcb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    pub fn rcb3(&self) -> Rcb3R {
        Rcb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    pub fn rcb4(&self) -> Rcb4R {
        Rcb4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    pub fn rcb5(&self) -> Rcb5R {
        Rcb5R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    pub fn rcb0(&mut self) -> Rcb0W<'_, MrcrSpec> {
        Rcb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    pub fn rcb1(&mut self) -> Rcb1W<'_, MrcrSpec> {
        Rcb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    pub fn rcb2(&mut self) -> Rcb2W<'_, MrcrSpec> {
        Rcb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    pub fn rcb3(&mut self) -> Rcb3W<'_, MrcrSpec> {
        Rcb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    pub fn rcb4(&mut self) -> Rcb4W<'_, MrcrSpec> {
        Rcb4W::new(self, 4)
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    pub fn rcb5(&mut self) -> Rcb5W<'_, MrcrSpec> {
        Rcb5W::new(self, 5)
    }
}
#[doc = "Master Remap Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrcrSpec;
impl crate::RegisterSpec for MrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrcr::R`](R) reader structure"]
impl crate::Readable for MrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mrcr::W`](W) writer structure"]
impl crate::Writable for MrcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MRCR to value 0"]
impl crate::Resettable for MrcrSpec {}

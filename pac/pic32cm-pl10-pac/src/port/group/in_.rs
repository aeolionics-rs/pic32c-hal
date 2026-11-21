#[doc = "Register `IN` reader"]
pub type R = crate::R<InSpec>;
#[doc = "Field `IN0` reader - PORT Data Input Value 0"]
pub type In0R = crate::BitReader;
#[doc = "Field `IN1` reader - PORT Data Input Value 1"]
pub type In1R = crate::BitReader;
#[doc = "Field `IN2` reader - PORT Data Input Value 2"]
pub type In2R = crate::BitReader;
#[doc = "Field `IN3` reader - PORT Data Input Value 3"]
pub type In3R = crate::BitReader;
#[doc = "Field `IN4` reader - PORT Data Input Value 4"]
pub type In4R = crate::BitReader;
#[doc = "Field `IN5` reader - PORT Data Input Value 5"]
pub type In5R = crate::BitReader;
#[doc = "Field `IN6` reader - PORT Data Input Value 6"]
pub type In6R = crate::BitReader;
#[doc = "Field `IN7` reader - PORT Data Input Value 7"]
pub type In7R = crate::BitReader;
#[doc = "Field `IN8` reader - PORT Data Input Value 8"]
pub type In8R = crate::BitReader;
#[doc = "Field `IN9` reader - PORT Data Input Value 9"]
pub type In9R = crate::BitReader;
#[doc = "Field `IN10` reader - PORT Data Input Value 10"]
pub type In10R = crate::BitReader;
#[doc = "Field `IN11` reader - PORT Data Input Value 11"]
pub type In11R = crate::BitReader;
#[doc = "Field `IN12` reader - PORT Data Input Value 12"]
pub type In12R = crate::BitReader;
#[doc = "Field `IN13` reader - PORT Data Input Value 13"]
pub type In13R = crate::BitReader;
#[doc = "Field `IN14` reader - PORT Data Input Value 14"]
pub type In14R = crate::BitReader;
#[doc = "Field `IN15` reader - PORT Data Input Value 15"]
pub type In15R = crate::BitReader;
#[doc = "Field `IN16` reader - PORT Data Input Value 16"]
pub type In16R = crate::BitReader;
#[doc = "Field `IN17` reader - PORT Data Input Value 17"]
pub type In17R = crate::BitReader;
#[doc = "Field `IN18` reader - PORT Data Input Value 18"]
pub type In18R = crate::BitReader;
#[doc = "Field `IN19` reader - PORT Data Input Value 19"]
pub type In19R = crate::BitReader;
#[doc = "Field `IN20` reader - PORT Data Input Value 20"]
pub type In20R = crate::BitReader;
#[doc = "Field `IN21` reader - PORT Data Input Value 21"]
pub type In21R = crate::BitReader;
#[doc = "Field `IN22` reader - PORT Data Input Value 22"]
pub type In22R = crate::BitReader;
#[doc = "Field `IN23` reader - PORT Data Input Value 23"]
pub type In23R = crate::BitReader;
#[doc = "Field `IN24` reader - PORT Data Input Value 24"]
pub type In24R = crate::BitReader;
#[doc = "Field `IN25` reader - PORT Data Input Value 25"]
pub type In25R = crate::BitReader;
#[doc = "Field `IN26` reader - PORT Data Input Value 26"]
pub type In26R = crate::BitReader;
#[doc = "Field `IN27` reader - PORT Data Input Value 27"]
pub type In27R = crate::BitReader;
#[doc = "Field `IN28` reader - PORT Data Input Value 28"]
pub type In28R = crate::BitReader;
#[doc = "Field `IN29` reader - PORT Data Input Value 29"]
pub type In29R = crate::BitReader;
#[doc = "Field `IN30` reader - PORT Data Input Value 30"]
pub type In30R = crate::BitReader;
#[doc = "Field `IN31` reader - PORT Data Input Value 31"]
pub type In31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PORT Data Input Value 0"]
    #[inline(always)]
    pub fn in0(&self) -> In0R {
        In0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PORT Data Input Value 1"]
    #[inline(always)]
    pub fn in1(&self) -> In1R {
        In1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PORT Data Input Value 2"]
    #[inline(always)]
    pub fn in2(&self) -> In2R {
        In2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PORT Data Input Value 3"]
    #[inline(always)]
    pub fn in3(&self) -> In3R {
        In3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PORT Data Input Value 4"]
    #[inline(always)]
    pub fn in4(&self) -> In4R {
        In4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PORT Data Input Value 5"]
    #[inline(always)]
    pub fn in5(&self) -> In5R {
        In5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PORT Data Input Value 6"]
    #[inline(always)]
    pub fn in6(&self) -> In6R {
        In6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PORT Data Input Value 7"]
    #[inline(always)]
    pub fn in7(&self) -> In7R {
        In7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PORT Data Input Value 8"]
    #[inline(always)]
    pub fn in8(&self) -> In8R {
        In8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PORT Data Input Value 9"]
    #[inline(always)]
    pub fn in9(&self) -> In9R {
        In9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PORT Data Input Value 10"]
    #[inline(always)]
    pub fn in10(&self) -> In10R {
        In10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PORT Data Input Value 11"]
    #[inline(always)]
    pub fn in11(&self) -> In11R {
        In11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PORT Data Input Value 12"]
    #[inline(always)]
    pub fn in12(&self) -> In12R {
        In12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PORT Data Input Value 13"]
    #[inline(always)]
    pub fn in13(&self) -> In13R {
        In13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PORT Data Input Value 14"]
    #[inline(always)]
    pub fn in14(&self) -> In14R {
        In14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PORT Data Input Value 15"]
    #[inline(always)]
    pub fn in15(&self) -> In15R {
        In15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PORT Data Input Value 16"]
    #[inline(always)]
    pub fn in16(&self) -> In16R {
        In16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PORT Data Input Value 17"]
    #[inline(always)]
    pub fn in17(&self) -> In17R {
        In17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PORT Data Input Value 18"]
    #[inline(always)]
    pub fn in18(&self) -> In18R {
        In18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PORT Data Input Value 19"]
    #[inline(always)]
    pub fn in19(&self) -> In19R {
        In19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PORT Data Input Value 20"]
    #[inline(always)]
    pub fn in20(&self) -> In20R {
        In20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PORT Data Input Value 21"]
    #[inline(always)]
    pub fn in21(&self) -> In21R {
        In21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PORT Data Input Value 22"]
    #[inline(always)]
    pub fn in22(&self) -> In22R {
        In22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PORT Data Input Value 23"]
    #[inline(always)]
    pub fn in23(&self) -> In23R {
        In23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PORT Data Input Value 24"]
    #[inline(always)]
    pub fn in24(&self) -> In24R {
        In24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PORT Data Input Value 25"]
    #[inline(always)]
    pub fn in25(&self) -> In25R {
        In25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PORT Data Input Value 26"]
    #[inline(always)]
    pub fn in26(&self) -> In26R {
        In26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PORT Data Input Value 27"]
    #[inline(always)]
    pub fn in27(&self) -> In27R {
        In27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PORT Data Input Value 28"]
    #[inline(always)]
    pub fn in28(&self) -> In28R {
        In28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PORT Data Input Value 29"]
    #[inline(always)]
    pub fn in29(&self) -> In29R {
        In29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PORT Data Input Value 30"]
    #[inline(always)]
    pub fn in30(&self) -> In30R {
        In30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PORT Data Input Value 31"]
    #[inline(always)]
    pub fn in31(&self) -> In31R {
        In31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Data Input Value\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InSpec;
impl crate::RegisterSpec for InSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for InSpec {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for InSpec {}

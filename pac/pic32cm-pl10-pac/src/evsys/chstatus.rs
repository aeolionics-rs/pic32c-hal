#[doc = "Register `CHSTATUS` reader"]
pub type R = crate::R<ChstatusSpec>;
#[doc = "Field `USRRDY0` reader - Users Ready on Channel 0"]
pub type Usrrdy0R = crate::BitReader;
#[doc = "Field `USRRDY1` reader - Users Ready on Channel 1"]
pub type Usrrdy1R = crate::BitReader;
#[doc = "Field `USRRDY2` reader - Users Ready on Channel 2"]
pub type Usrrdy2R = crate::BitReader;
#[doc = "Field `USRRDY3` reader - Users Ready on Channel 3"]
pub type Usrrdy3R = crate::BitReader;
#[doc = "Field `CHBUSY0` reader - Channel 0 Busy"]
pub type Chbusy0R = crate::BitReader;
#[doc = "Field `CHBUSY1` reader - Channel 1 Busy"]
pub type Chbusy1R = crate::BitReader;
#[doc = "Field `CHBUSY2` reader - Channel 2 Busy"]
pub type Chbusy2R = crate::BitReader;
#[doc = "Field `CHBUSY3` reader - Channel 3 Busy"]
pub type Chbusy3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Users Ready on Channel 0"]
    #[inline(always)]
    pub fn usrrdy0(&self) -> Usrrdy0R {
        Usrrdy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Users Ready on Channel 1"]
    #[inline(always)]
    pub fn usrrdy1(&self) -> Usrrdy1R {
        Usrrdy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Users Ready on Channel 2"]
    #[inline(always)]
    pub fn usrrdy2(&self) -> Usrrdy2R {
        Usrrdy2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Users Ready on Channel 3"]
    #[inline(always)]
    pub fn usrrdy3(&self) -> Usrrdy3R {
        Usrrdy3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 0 Busy"]
    #[inline(always)]
    pub fn chbusy0(&self) -> Chbusy0R {
        Chbusy0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 1 Busy"]
    #[inline(always)]
    pub fn chbusy1(&self) -> Chbusy1R {
        Chbusy1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 2 Busy"]
    #[inline(always)]
    pub fn chbusy2(&self) -> Chbusy2R {
        Chbusy2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 3 Busy"]
    #[inline(always)]
    pub fn chbusy3(&self) -> Chbusy3R {
        Chbusy3R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Channel Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatusSpec;
impl crate::RegisterSpec for ChstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for ChstatusSpec {}
#[doc = "`reset()` method sets CHSTATUS to value 0"]
impl crate::Resettable for ChstatusSpec {}

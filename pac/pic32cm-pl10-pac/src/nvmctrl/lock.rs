#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Field `LOCK0` reader - Region 0 Lock Bit"]
pub type Lock0R = crate::BitReader;
#[doc = "Field `LOCK1` reader - Region 1 Lock Bit"]
pub type Lock1R = crate::BitReader;
#[doc = "Field `LOCK2` reader - Region 2 Lock Bit"]
pub type Lock2R = crate::BitReader;
#[doc = "Field `LOCK3` reader - Region 3 Lock Bit"]
pub type Lock3R = crate::BitReader;
#[doc = "Field `LOCK4` reader - Region 4 Lock Bit"]
pub type Lock4R = crate::BitReader;
#[doc = "Field `LOCK5` reader - Region 5 Lock Bit"]
pub type Lock5R = crate::BitReader;
#[doc = "Field `LOCK6` reader - Region 6 Lock Bit"]
pub type Lock6R = crate::BitReader;
#[doc = "Field `LOCK7` reader - Region 7 Lock Bit"]
pub type Lock7R = crate::BitReader;
#[doc = "Field `LOCK8` reader - Region 8 Lock Bit"]
pub type Lock8R = crate::BitReader;
#[doc = "Field `LOCK9` reader - Region 9 Lock Bit"]
pub type Lock9R = crate::BitReader;
#[doc = "Field `LOCK10` reader - Region 10 Lock Bit"]
pub type Lock10R = crate::BitReader;
#[doc = "Field `LOCK11` reader - Region 11 Lock Bit"]
pub type Lock11R = crate::BitReader;
#[doc = "Field `LOCK12` reader - Region 12 Lock Bit"]
pub type Lock12R = crate::BitReader;
#[doc = "Field `LOCK13` reader - Region 13 Lock Bit"]
pub type Lock13R = crate::BitReader;
#[doc = "Field `LOCK14` reader - Region 14 Lock Bit"]
pub type Lock14R = crate::BitReader;
#[doc = "Field `LOCK15` reader - Region 15 Lock Bit"]
pub type Lock15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Region 0 Lock Bit"]
    #[inline(always)]
    pub fn lock0(&self) -> Lock0R {
        Lock0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Region 1 Lock Bit"]
    #[inline(always)]
    pub fn lock1(&self) -> Lock1R {
        Lock1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Region 2 Lock Bit"]
    #[inline(always)]
    pub fn lock2(&self) -> Lock2R {
        Lock2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Region 3 Lock Bit"]
    #[inline(always)]
    pub fn lock3(&self) -> Lock3R {
        Lock3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Region 4 Lock Bit"]
    #[inline(always)]
    pub fn lock4(&self) -> Lock4R {
        Lock4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Region 5 Lock Bit"]
    #[inline(always)]
    pub fn lock5(&self) -> Lock5R {
        Lock5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Region 6 Lock Bit"]
    #[inline(always)]
    pub fn lock6(&self) -> Lock6R {
        Lock6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Region 7 Lock Bit"]
    #[inline(always)]
    pub fn lock7(&self) -> Lock7R {
        Lock7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Region 8 Lock Bit"]
    #[inline(always)]
    pub fn lock8(&self) -> Lock8R {
        Lock8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Region 9 Lock Bit"]
    #[inline(always)]
    pub fn lock9(&self) -> Lock9R {
        Lock9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Region 10 Lock Bit"]
    #[inline(always)]
    pub fn lock10(&self) -> Lock10R {
        Lock10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Region 11 Lock Bit"]
    #[inline(always)]
    pub fn lock11(&self) -> Lock11R {
        Lock11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Region 12 Lock Bit"]
    #[inline(always)]
    pub fn lock12(&self) -> Lock12R {
        Lock12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Region 13 Lock Bit"]
    #[inline(always)]
    pub fn lock13(&self) -> Lock13R {
        Lock13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Region 14 Lock Bit"]
    #[inline(always)]
    pub fn lock14(&self) -> Lock14R {
        Lock14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Region 15 Lock Bit"]
    #[inline(always)]
    pub fn lock15(&self) -> Lock15R {
        Lock15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Lock Section\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LockSpec {}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {}

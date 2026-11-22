#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `BUSERR` reader - Bus Error"]
pub type BuserrR = crate::BitReader;
#[doc = "Field `BUSERR` writer - Bus Error"]
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` reader - Arbitration Lost"]
pub type ArblostR = crate::BitReader;
#[doc = "Field `ARBLOST` writer - Arbitration Lost"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNACK` reader - Received Not Acknowledge"]
pub type RxnackR = crate::BitReader;
#[doc = "Field `RXNACK` writer - Received Not Acknowledge"]
pub type RxnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSSTATE` reader - Bus State"]
pub type BusstateR = crate::FieldReader;
#[doc = "Field `BUSSTATE` writer - Bus State"]
pub type BusstateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LOWTOUT` reader - SCL Low Time-out"]
pub type LowtoutR = crate::BitReader;
#[doc = "Field `LOWTOUT` writer - SCL Low Time-out"]
pub type LowtoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKHOLD` reader - Clock Hold"]
pub type ClkholdR = crate::BitReader;
#[doc = "Field `CLKHOLD` writer - Clock Hold"]
pub type ClkholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCBUSY` reader - Synchronization Busy"]
pub type SyncbusyR = crate::BitReader;
#[doc = "Field `SYNCBUSY` writer - Synchronization Busy"]
pub type SyncbusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblost(&self) -> ArblostR {
        ArblostR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received Not Acknowledge"]
    #[inline(always)]
    pub fn rxnack(&self) -> RxnackR {
        RxnackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Bus State"]
    #[inline(always)]
    pub fn busstate(&self) -> BusstateR {
        BusstateR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SCL Low Time-out"]
    #[inline(always)]
    pub fn lowtout(&self) -> LowtoutR {
        LowtoutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Hold"]
    #[inline(always)]
    pub fn clkhold(&self) -> ClkholdR {
        ClkholdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Synchronization Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SyncbusyR {
        SyncbusyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BuserrW<'_, StatusSpec> {
        BuserrW::new(self, 0)
    }
    #[doc = "Bit 1 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<'_, StatusSpec> {
        ArblostW::new(self, 1)
    }
    #[doc = "Bit 2 - Received Not Acknowledge"]
    #[inline(always)]
    pub fn rxnack(&mut self) -> RxnackW<'_, StatusSpec> {
        RxnackW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Bus State"]
    #[inline(always)]
    pub fn busstate(&mut self) -> BusstateW<'_, StatusSpec> {
        BusstateW::new(self, 4)
    }
    #[doc = "Bit 6 - SCL Low Time-out"]
    #[inline(always)]
    pub fn lowtout(&mut self) -> LowtoutW<'_, StatusSpec> {
        LowtoutW::new(self, 6)
    }
    #[doc = "Bit 7 - Clock Hold"]
    #[inline(always)]
    pub fn clkhold(&mut self) -> ClkholdW<'_, StatusSpec> {
        ClkholdW::new(self, 7)
    }
    #[doc = "Bit 15 - Synchronization Busy"]
    #[inline(always)]
    pub fn syncbusy(&mut self) -> SyncbusyW<'_, StatusSpec> {
        SyncbusyW::new(self, 15)
    }
}
#[doc = "I2CM Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}

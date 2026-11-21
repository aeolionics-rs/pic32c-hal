#[doc = "Register `USER[%s]` reader"]
pub type R = crate::R<UserSpec>;
#[doc = "Register `USER[%s]` writer"]
pub type W = crate::W<UserSpec>;
#[doc = "Channel Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Channelselect {
    #[doc = "0: The Event System user is not connected to any channel"]
    Off = 0,
    #[doc = "1: The Event System user is connected to Channel 0"]
    Channel0 = 1,
    #[doc = "2: The Event System user is connected to Channel 1"]
    Channel1 = 2,
    #[doc = "3: The Event System user is connected to Channel 2"]
    Channel2 = 3,
    #[doc = "4: The Event System user is connected to Channel 3"]
    Channel3 = 4,
}
impl From<Channelselect> for u8 {
    #[inline(always)]
    fn from(variant: Channelselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Channelselect {
    type Ux = u8;
}
impl crate::IsEnum for Channelselect {}
#[doc = "Field `CHANNEL` reader - Channel Event Selection"]
pub type ChannelR = crate::FieldReader<Channelselect>;
impl ChannelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Channelselect> {
        match self.bits {
            0 => Some(Channelselect::Off),
            1 => Some(Channelselect::Channel0),
            2 => Some(Channelselect::Channel1),
            3 => Some(Channelselect::Channel2),
            4 => Some(Channelselect::Channel3),
            _ => None,
        }
    }
    #[doc = "The Event System user is not connected to any channel"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Channelselect::Off
    }
    #[doc = "The Event System user is connected to Channel 0"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == Channelselect::Channel0
    }
    #[doc = "The Event System user is connected to Channel 1"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == Channelselect::Channel1
    }
    #[doc = "The Event System user is connected to Channel 2"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == Channelselect::Channel2
    }
    #[doc = "The Event System user is connected to Channel 3"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == Channelselect::Channel3
    }
}
#[doc = "Field `CHANNEL` writer - Channel Event Selection"]
pub type ChannelW<'a, REG> = crate::FieldWriter<'a, REG, 5, Channelselect>;
impl<'a, REG> ChannelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The Event System user is not connected to any channel"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Channelselect::Off)
    }
    #[doc = "The Event System user is connected to Channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(Channelselect::Channel0)
    }
    #[doc = "The Event System user is connected to Channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(Channelselect::Channel1)
    }
    #[doc = "The Event System user is connected to Channel 2"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut crate::W<REG> {
        self.variant(Channelselect::Channel2)
    }
    #[doc = "The Event System user is connected to Channel 3"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut crate::W<REG> {
        self.variant(Channelselect::Channel3)
    }
}
impl R {
    #[doc = "Bits 0:4 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&self) -> ChannelR {
        ChannelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&mut self) -> ChannelW<'_, UserSpec> {
        ChannelW::new(self, 0)
    }
}
#[doc = "User # Channel Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserSpec;
impl crate::RegisterSpec for UserSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user::R`](R) reader structure"]
impl crate::Readable for UserSpec {}
#[doc = "`write(|w| ..)` method takes [`user::W`](W) writer structure"]
impl crate::Writable for UserSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USER[%s] to value 0"]
impl crate::Resettable for UserSpec {}

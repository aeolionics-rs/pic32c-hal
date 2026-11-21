#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_mode0: [u8; 0x25],
}
impl RegisterBlock {
    #[doc = "0x00..0x25 - Clock and Calendar Mode with Alarm"]
    #[inline(always)]
    pub const fn mode2(&self) -> &Mode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00..0x24 - 16-bit Counter Mode"]
    #[inline(always)]
    pub const fn mode1(&self) -> &Mode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00..0x24 - 32-bit Counter Mode"]
    #[inline(always)]
    pub const fn mode0(&self) -> &Mode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
}
#[doc = "32-bit Counter Mode"]
pub use self::mode0::Mode0;
#[doc = r"Cluster"]
#[doc = "32-bit Counter Mode"]
pub mod mode0;
#[doc = "16-bit Counter Mode"]
pub use self::mode1::Mode1;
#[doc = r"Cluster"]
#[doc = "16-bit Counter Mode"]
pub mod mode1;
#[doc = "Clock and Calendar Mode with Alarm"]
pub use self::mode2::Mode2;
#[doc = r"Cluster"]
#[doc = "Clock and Calendar Mode with Alarm"]
pub mod mode2;

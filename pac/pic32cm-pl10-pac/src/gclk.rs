#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    syncbusy: Syncbusy,
    _reserved1: [u8; 0x18],
    genctrl: [Genctrl; 4],
    _reserved2: [u8; 0x50],
    pchctrl: [Pchctrl; 13],
}
impl RegisterBlock {
    #[doc = "0x04 - Synchronization Busy"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x20..0x30 - Generator Control"]
    #[inline(always)]
    pub const fn genctrl(&self, n: usize) -> &Genctrl {
        &self.genctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - Generator Control"]
    #[inline(always)]
    pub fn genctrl_iter(&self) -> impl Iterator<Item = &Genctrl> {
        self.genctrl.iter()
    }
    #[doc = "0x80..0xb4 - Peripheral Channel Control #"]
    #[inline(always)]
    pub const fn pchctrl(&self, n: usize) -> &Pchctrl {
        &self.pchctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xb4 - Peripheral Channel Control #"]
    #[inline(always)]
    pub fn pchctrl_iter(&self) -> impl Iterator<Item = &Pchctrl> {
        self.pchctrl.iter()
    }
}
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "GENCTRL (rw) register accessor: Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`genctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@genctrl`] module"]
#[doc(alias = "GENCTRL")]
pub type Genctrl = crate::Reg<genctrl::GenctrlSpec>;
#[doc = "Generator Control"]
pub mod genctrl;
#[doc = "PCHCTRL (rw) register accessor: Peripheral Channel Control #\n\nYou can [`read`](crate::Reg::read) this register and get [`pchctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pchctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pchctrl`] module"]
#[doc(alias = "PCHCTRL")]
pub type Pchctrl = crate::Reg<pchctrl::PchctrlSpec>;
#[doc = "Peripheral Channel Control #"]
pub mod pchctrl;

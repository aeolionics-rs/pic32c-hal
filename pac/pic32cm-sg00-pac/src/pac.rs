#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    wrctrl: Wrctrl,
    _reserved2: [u8; 0x38],
    status: [Status; 4],
    lock: [Lock; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Control A Register"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Write Control Register"]
    #[inline(always)]
    pub const fn wrctrl(&self) -> &Wrctrl {
        &self.wrctrl
    }
    #[doc = "0x40..0x50 - Peripheral Status n Register"]
    #[inline(always)]
    pub const fn status(&self, n: usize) -> &Status {
        &self.status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Peripheral Status n Register"]
    #[inline(always)]
    pub fn status_iter(&self) -> impl Iterator<Item = &Status> {
        self.status.iter()
    }
    #[doc = "0x50..0x60 - Peripheral Lock n Register"]
    #[inline(always)]
    pub const fn lock(&self, n: usize) -> &Lock {
        &self.lock[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x60 - Peripheral Lock n Register"]
    #[inline(always)]
    pub fn lock_iter(&self) -> impl Iterator<Item = &Lock> {
        self.lock.iter()
    }
}
#[doc = "CTRLA (rw) register accessor: Control A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A Register"]
pub mod ctrla;
#[doc = "WRCTRL (w) register accessor: Write Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrctrl`] module"]
#[doc(alias = "WRCTRL")]
pub type Wrctrl = crate::Reg<wrctrl::WrctrlSpec>;
#[doc = "Write Control Register"]
pub mod wrctrl;
#[doc = "STATUS (r) register accessor: Peripheral Status n Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Peripheral Status n Register"]
pub mod status;
#[doc = "LOCK (r) register accessor: Peripheral Lock n Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Peripheral Lock n Register"]
pub mod lock;

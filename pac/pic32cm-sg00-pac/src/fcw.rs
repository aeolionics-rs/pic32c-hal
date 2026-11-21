#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlop: Ctrlop,
    mutex: Mutex,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    intflagset: Intflagset,
    status: Status,
    key: Key,
    addr: Addr,
    srcaddr: Srcaddr,
    data: [Data; 4],
    _reserved12: [u8; 0x10],
    swap: Swap,
    pwp: [Pwp; 4],
    _reserved14: [u8; 0x10],
    lbwp: Lbwp,
    ubwp: Ubwp,
    uowp: Uowp,
    cwp: Cwp,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A REGISTER"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Control Operation REGISTER"]
    #[inline(always)]
    pub const fn ctrlop(&self) -> &Ctrlop {
        &self.ctrlop
    }
    #[doc = "0x08 - MUTEX REGISTER"]
    #[inline(always)]
    pub const fn mutex(&self) -> &Mutex {
        &self.mutex
    }
    #[doc = "0x0c - Interrupt Enable Clear REGISTER"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x10 - Interrupt Enable Set REGISTER"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x14 - Interrupt Flag REGISTER"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x18 - Interrupt Flag Set REGISTER"]
    #[inline(always)]
    pub const fn intflagset(&self) -> &Intflagset {
        &self.intflagset
    }
    #[doc = "0x1c - Status REGISTER"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x20 - Feature Unlock Key REGISTER"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x24 - Flash Destination Address REGISTER"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x28 - System Source Address REGISTER"]
    #[inline(always)]
    pub const fn srcaddr(&self) -> &Srcaddr {
        &self.srcaddr
    }
    #[doc = "0x2c..0x3c - Flash Write Data n REGISTER"]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &Data {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c..0x3c - Flash Write Data n REGISTER"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &Data> {
        self.data.iter()
    }
    #[doc = "0x4c - NVM Panel Swap REGISTER"]
    #[inline(always)]
    pub const fn swap(&self) -> &Swap {
        &self.swap
    }
    #[doc = "0x50..0x60 - PFM Write Protect Region n REGISTER"]
    #[inline(always)]
    pub const fn pwp(&self, n: usize) -> &Pwp {
        &self.pwp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x60 - PFM Write Protect Region n REGISTER"]
    #[inline(always)]
    pub fn pwp_iter(&self) -> impl Iterator<Item = &Pwp> {
        self.pwp.iter()
    }
    #[doc = "0x70 - Lower BFM Write Protect REGISTER"]
    #[inline(always)]
    pub const fn lbwp(&self) -> &Lbwp {
        &self.lbwp
    }
    #[doc = "0x74 - Upper BFM Write Protect REGISTER"]
    #[inline(always)]
    pub const fn ubwp(&self) -> &Ubwp {
        &self.ubwp
    }
    #[doc = "0x78 - User OTP Write protection REGISTER"]
    #[inline(always)]
    pub const fn uowp(&self) -> &Uowp {
        &self.uowp
    }
    #[doc = "0x7c - CFM Page Write Protect REGISTER"]
    #[inline(always)]
    pub const fn cwp(&self) -> &Cwp {
        &self.cwp
    }
}
#[doc = "CTRLA (rw) register accessor: Control A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A REGISTER"]
pub mod ctrla;
#[doc = "CTRLOP (rw) register accessor: Control Operation REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlop`] module"]
#[doc(alias = "CTRLOP")]
pub type Ctrlop = crate::Reg<ctrlop::CtrlopSpec>;
#[doc = "Control Operation REGISTER"]
pub mod ctrlop;
#[doc = "MUTEX (rw) register accessor: MUTEX REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex`] module"]
#[doc(alias = "MUTEX")]
pub type Mutex = crate::Reg<mutex::MutexSpec>;
#[doc = "MUTEX REGISTER"]
pub mod mutex;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear REGISTER"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set REGISTER"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag REGISTER"]
pub mod intflag;
#[doc = "INTFLAGSET (rw) register accessor: Interrupt Flag Set REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagset`] module"]
#[doc(alias = "INTFLAGSET")]
pub type Intflagset = crate::Reg<intflagset::IntflagsetSpec>;
#[doc = "Interrupt Flag Set REGISTER"]
pub mod intflagset;
#[doc = "STATUS (r) register accessor: Status REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status REGISTER"]
pub mod status;
#[doc = "KEY (rw) register accessor: Feature Unlock Key REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "Feature Unlock Key REGISTER"]
pub mod key;
#[doc = "ADDR (rw) register accessor: Flash Destination Address REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Flash Destination Address REGISTER"]
pub mod addr;
#[doc = "SRCADDR (rw) register accessor: System Source Address REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr`] module"]
#[doc(alias = "SRCADDR")]
pub type Srcaddr = crate::Reg<srcaddr::SrcaddrSpec>;
#[doc = "System Source Address REGISTER"]
pub mod srcaddr;
#[doc = "DATA (rw) register accessor: Flash Write Data n REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Flash Write Data n REGISTER"]
pub mod data;
#[doc = "SWAP (rw) register accessor: NVM Panel Swap REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`swap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swap`] module"]
#[doc(alias = "SWAP")]
pub type Swap = crate::Reg<swap::SwapSpec>;
#[doc = "NVM Panel Swap REGISTER"]
pub mod swap;
#[doc = "PWP (rw) register accessor: PFM Write Protect Region n REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`pwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwp`] module"]
#[doc(alias = "PWP")]
pub type Pwp = crate::Reg<pwp::PwpSpec>;
#[doc = "PFM Write Protect Region n REGISTER"]
pub mod pwp;
#[doc = "LBWP (rw) register accessor: Lower BFM Write Protect REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`lbwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lbwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lbwp`] module"]
#[doc(alias = "LBWP")]
pub type Lbwp = crate::Reg<lbwp::LbwpSpec>;
#[doc = "Lower BFM Write Protect REGISTER"]
pub mod lbwp;
#[doc = "UBWP (rw) register accessor: Upper BFM Write Protect REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ubwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubwp`] module"]
#[doc(alias = "UBWP")]
pub type Ubwp = crate::Reg<ubwp::UbwpSpec>;
#[doc = "Upper BFM Write Protect REGISTER"]
pub mod ubwp;
#[doc = "UOWP (rw) register accessor: User OTP Write protection REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`uowp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uowp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uowp`] module"]
#[doc(alias = "UOWP")]
pub type Uowp = crate::Reg<uowp::UowpSpec>;
#[doc = "User OTP Write protection REGISTER"]
pub mod uowp;
#[doc = "CWP (rw) register accessor: CFM Page Write Protect REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`cwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwp`] module"]
#[doc(alias = "CWP")]
pub type Cwp = crate::Reg<cwp::CwpSpec>;
#[doc = "CFM Page Write Protect REGISTER"]
pub mod cwp;

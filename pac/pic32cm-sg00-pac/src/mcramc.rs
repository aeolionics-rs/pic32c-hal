#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0x04],
    intenclr: Intenclr,
    intenset: Intenset,
    intsta: Intsta,
    fltctrl: Fltctrl,
    fltptr: Fltptr,
    fltadr: Fltadr,
    errcadr: Errcadr,
    errcpar: Errcpar,
    errcsyn: Errcsyn,
    _reserved10: [u8; 0xd0],
    version: Version,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Enable A Register"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x08 - Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x0c - Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x10 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intsta(&self) -> &Intsta {
        &self.intsta
    }
    #[doc = "0x14 - Fault Injection Control Register"]
    #[inline(always)]
    pub const fn fltctrl(&self) -> &Fltctrl {
        &self.fltctrl
    }
    #[doc = "0x18 - Fault Injection Pointer Register"]
    #[inline(always)]
    pub const fn fltptr(&self) -> &Fltptr {
        &self.fltptr
    }
    #[doc = "0x1c - Fault Injection Address Register"]
    #[inline(always)]
    pub const fn fltadr(&self) -> &Fltadr {
        &self.fltadr
    }
    #[doc = "0x20 - Error Capture Address Register"]
    #[inline(always)]
    pub const fn errcadr(&self) -> &Errcadr {
        &self.errcadr
    }
    #[doc = "0x24 - Error Capture Parity Register"]
    #[inline(always)]
    pub const fn errcpar(&self) -> &Errcpar {
        &self.errcpar
    }
    #[doc = "0x28 - Error Capture Syndrome Register"]
    #[inline(always)]
    pub const fn errcsyn(&self) -> &Errcsyn {
        &self.errcsyn
    }
    #[doc = "0xfc - Version Register"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
}
#[doc = "CTRLA (rw) register accessor: Control Enable A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control Enable A Register"]
pub mod ctrla;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set Register"]
pub mod intenset;
#[doc = "INTSTA (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsta`] module"]
#[doc(alias = "INTSTA")]
pub type Intsta = crate::Reg<intsta::IntstaSpec>;
#[doc = "Interrupt Status Register"]
pub mod intsta;
#[doc = "FLTCTRL (rw) register accessor: Fault Injection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fltctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltctrl`] module"]
#[doc(alias = "FLTCTRL")]
pub type Fltctrl = crate::Reg<fltctrl::FltctrlSpec>;
#[doc = "Fault Injection Control Register"]
pub mod fltctrl;
#[doc = "FLTPTR (rw) register accessor: Fault Injection Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fltptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltptr`] module"]
#[doc(alias = "FLTPTR")]
pub type Fltptr = crate::Reg<fltptr::FltptrSpec>;
#[doc = "Fault Injection Pointer Register"]
pub mod fltptr;
#[doc = "FLTADR (rw) register accessor: Fault Injection Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fltadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltadr`] module"]
#[doc(alias = "FLTADR")]
pub type Fltadr = crate::Reg<fltadr::FltadrSpec>;
#[doc = "Fault Injection Address Register"]
pub mod fltadr;
#[doc = "ERRCADR (r) register accessor: Error Capture Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errcadr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errcadr`] module"]
#[doc(alias = "ERRCADR")]
pub type Errcadr = crate::Reg<errcadr::ErrcadrSpec>;
#[doc = "Error Capture Address Register"]
pub mod errcadr;
#[doc = "ERRCPAR (r) register accessor: Error Capture Parity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errcpar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errcpar`] module"]
#[doc(alias = "ERRCPAR")]
pub type Errcpar = crate::Reg<errcpar::ErrcparSpec>;
#[doc = "Error Capture Parity Register"]
pub mod errcpar;
#[doc = "ERRCSYN (r) register accessor: Error Capture Syndrome Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errcsyn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errcsyn`] module"]
#[doc(alias = "ERRCSYN")]
pub type Errcsyn = crate::Reg<errcsyn::ErrcsynSpec>;
#[doc = "Error Capture Syndrome Register"]
pub mod errcsyn;
#[doc = "VERSION (r) register accessor: Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "Version Register"]
pub mod version;

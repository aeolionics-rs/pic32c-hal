#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0x04],
    nonsecclra: Nonsecclra,
    nonsecseta: Nonsecseta,
}
impl RegisterBlock {
    #[doc = "0x00 - H2PB CONTROL Register"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x08 - Non-Security Clear Register A"]
    #[inline(always)]
    pub const fn nonsecclra(&self) -> &Nonsecclra {
        &self.nonsecclra
    }
    #[doc = "0x0c - Non-Security SET Register A"]
    #[inline(always)]
    pub const fn nonsecseta(&self) -> &Nonsecseta {
        &self.nonsecseta
    }
}
#[doc = "CTRLA (rw) register accessor: H2PB CONTROL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "H2PB CONTROL Register"]
pub mod ctrla;
#[doc = "NONSECCLRA (rw) register accessor: Non-Security Clear Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`nonsecclra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonsecclra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonsecclra`] module"]
#[doc(alias = "NONSECCLRA")]
pub type Nonsecclra = crate::Reg<nonsecclra::NonsecclraSpec>;
#[doc = "Non-Security Clear Register A"]
pub mod nonsecclra;
#[doc = "NONSECSETA (rw) register accessor: Non-Security SET Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`nonsecseta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonsecseta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonsecseta`] module"]
#[doc(alias = "NONSECSETA")]
pub type Nonsecseta = crate::Reg<nonsecseta::NonsecsetaSpec>;
#[doc = "Non-Security SET Register A"]
pub mod nonsecseta;

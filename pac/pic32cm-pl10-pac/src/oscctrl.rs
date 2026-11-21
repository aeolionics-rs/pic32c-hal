#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    intflagset: Intflagset,
    status: Status,
    _reserved5: [u8; 0x0c],
    oschfctrl: Oschfctrl,
    oschftune: Oschftune,
    wpctrl: Wpctrl,
}
impl RegisterBlock {
    #[doc = "0x04 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x08 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x0c - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x10 - Interrupt Flag Software Set"]
    #[inline(always)]
    pub const fn intflagset(&self) -> &Intflagset {
        &self.intflagset
    }
    #[doc = "0x14 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x24 - OSCHF Control"]
    #[inline(always)]
    pub const fn oschfctrl(&self) -> &Oschfctrl {
        &self.oschfctrl
    }
    #[doc = "0x28 - OSCHF Oscillator Frequency Tune"]
    #[inline(always)]
    pub const fn oschftune(&self) -> &Oschftune {
        &self.oschftune
    }
    #[doc = "0x2c - Write Protection Control"]
    #[inline(always)]
    pub const fn wpctrl(&self) -> &Wpctrl {
        &self.wpctrl
    }
}
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "INTFLAGSET (rw) register accessor: Interrupt Flag Software Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagset`] module"]
#[doc(alias = "INTFLAGSET")]
pub type Intflagset = crate::Reg<intflagset::IntflagsetSpec>;
#[doc = "Interrupt Flag Software Set"]
pub mod intflagset;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "OSCHFCTRL (rw) register accessor: OSCHF Control\n\nYou can [`read`](crate::Reg::read) this register and get [`oschfctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oschfctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oschfctrl`] module"]
#[doc(alias = "OSCHFCTRL")]
pub type Oschfctrl = crate::Reg<oschfctrl::OschfctrlSpec>;
#[doc = "OSCHF Control"]
pub mod oschfctrl;
#[doc = "OSCHFTUNE (rw) register accessor: OSCHF Oscillator Frequency Tune\n\nYou can [`read`](crate::Reg::read) this register and get [`oschftune::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oschftune::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oschftune`] module"]
#[doc(alias = "OSCHFTUNE")]
pub type Oschftune = crate::Reg<oschftune::OschftuneSpec>;
#[doc = "OSCHF Oscillator Frequency Tune"]
pub mod oschftune;
#[doc = "WPCTRL (rw) register accessor: Write Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpctrl`] module"]
#[doc(alias = "WPCTRL")]
pub type Wpctrl = crate::Reg<wpctrl::WpctrlSpec>;
#[doc = "Write Protection Control"]
pub mod wpctrl;

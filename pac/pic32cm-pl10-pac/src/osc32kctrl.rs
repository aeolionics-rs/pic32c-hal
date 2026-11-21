#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    intflagset: Intflagset,
    status: Status,
    rtcctrl: Rtcctrl,
    xosc32kctrl: Xosc32kctrl,
    cfdctrl: Cfdctrl,
    evctrl: Evctrl,
    osc32kctrl: Osc32kctrl,
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
    #[doc = "0x18 - RTC Control"]
    #[inline(always)]
    pub const fn rtcctrl(&self) -> &Rtcctrl {
        &self.rtcctrl
    }
    #[doc = "0x1c - XOSC32K Control"]
    #[inline(always)]
    pub const fn xosc32kctrl(&self) -> &Xosc32kctrl {
        &self.xosc32kctrl
    }
    #[doc = "0x20 - Clock Failure Detector Control"]
    #[inline(always)]
    pub const fn cfdctrl(&self) -> &Cfdctrl {
        &self.cfdctrl
    }
    #[doc = "0x24 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x28 - 32.768 kHz Internal Oscillator (OSC32K) Control"]
    #[inline(always)]
    pub const fn osc32kctrl(&self) -> &Osc32kctrl {
        &self.osc32kctrl
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
#[doc = "RTCCTRL (rw) register accessor: RTC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcctrl`] module"]
#[doc(alias = "RTCCTRL")]
pub type Rtcctrl = crate::Reg<rtcctrl::RtcctrlSpec>;
#[doc = "RTC Control"]
pub mod rtcctrl;
#[doc = "XOSC32KCTRL (rw) register accessor: XOSC32K Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32kctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc32kctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc32kctrl`] module"]
#[doc(alias = "XOSC32KCTRL")]
pub type Xosc32kctrl = crate::Reg<xosc32kctrl::Xosc32kctrlSpec>;
#[doc = "XOSC32K Control"]
pub mod xosc32kctrl;
#[doc = "CFDCTRL (rw) register accessor: Clock Failure Detector Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cfdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfdctrl`] module"]
#[doc(alias = "CFDCTRL")]
pub type Cfdctrl = crate::Reg<cfdctrl::CfdctrlSpec>;
#[doc = "Clock Failure Detector Control"]
pub mod cfdctrl;
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`] module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "OSC32KCTRL (rw) register accessor: 32.768 kHz Internal Oscillator (OSC32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`osc32kctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc32kctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc32kctrl`] module"]
#[doc(alias = "OSC32KCTRL")]
pub type Osc32kctrl = crate::Reg<osc32kctrl::Osc32kctrlSpec>;
#[doc = "32.768 kHz Internal Oscillator (OSC32K) Control"]
pub mod osc32kctrl;
#[doc = "WPCTRL (rw) register accessor: Write Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpctrl`] module"]
#[doc(alias = "WPCTRL")]
pub type Wpctrl = crate::Reg<wpctrl::WpctrlSpec>;
#[doc = "Write Protection Control"]
pub mod wpctrl;

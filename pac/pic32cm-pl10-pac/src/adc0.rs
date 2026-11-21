#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    ctrlc: Ctrlc,
    ctrld: Ctrld,
    ctrle: Ctrle,
    _reserved5: [u8; 0x08],
    winctrl: Winctrl,
    winlt: Winlt,
    winht: Winht,
    inputctrl: Inputctrl,
    evctrl: Evctrl,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    intflagset: Intflagset,
    status: Status,
    _reserved15: [u8; 0x04],
    command: Command,
    _reserved16: [u8; 0x04],
    result: Result,
    sample: Sample,
    _reserved18: [u8; 0x14],
    dbgctrl: Dbgctrl,
    _reserved19: [u8; 0x0c],
    wpctrl: Wpctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x08 - Control C"]
    #[inline(always)]
    pub const fn ctrlc(&self) -> &Ctrlc {
        &self.ctrlc
    }
    #[doc = "0x0c - Control D"]
    #[inline(always)]
    pub const fn ctrld(&self) -> &Ctrld {
        &self.ctrld
    }
    #[doc = "0x10 - Control E"]
    #[inline(always)]
    pub const fn ctrle(&self) -> &Ctrle {
        &self.ctrle
    }
    #[doc = "0x1c - Window Control"]
    #[inline(always)]
    pub const fn winctrl(&self) -> &Winctrl {
        &self.winctrl
    }
    #[doc = "0x20 - Window Low Threshold"]
    #[inline(always)]
    pub const fn winlt(&self) -> &Winlt {
        &self.winlt
    }
    #[doc = "0x24 - Window High Threshold"]
    #[inline(always)]
    pub const fn winht(&self) -> &Winht {
        &self.winht
    }
    #[doc = "0x28 - Input Multiplexer Control"]
    #[inline(always)]
    pub const fn inputctrl(&self) -> &Inputctrl {
        &self.inputctrl
    }
    #[doc = "0x2c - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x30 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x34 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x38 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x3c - Interrupt Flag Set"]
    #[inline(always)]
    pub const fn intflagset(&self) -> &Intflagset {
        &self.intflagset
    }
    #[doc = "0x40 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x48 - Command"]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    #[doc = "0x50 - Result"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
    #[doc = "0x54 - Sample"]
    #[inline(always)]
    pub const fn sample(&self) -> &Sample {
        &self.sample
    }
    #[doc = "0x6c - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x7c - Write Protection Control"]
    #[inline(always)]
    pub const fn wpctrl(&self) -> &Wpctrl {
        &self.wpctrl
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlc`] module"]
#[doc(alias = "CTRLC")]
pub type Ctrlc = crate::Reg<ctrlc::CtrlcSpec>;
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "CTRLD (rw) register accessor: Control D\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrld`] module"]
#[doc(alias = "CTRLD")]
pub type Ctrld = crate::Reg<ctrld::CtrldSpec>;
#[doc = "Control D"]
pub mod ctrld;
#[doc = "CTRLE (rw) register accessor: Control E\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrle`] module"]
#[doc(alias = "CTRLE")]
pub type Ctrle = crate::Reg<ctrle::CtrleSpec>;
#[doc = "Control E"]
pub mod ctrle;
#[doc = "WINCTRL (rw) register accessor: Window Control\n\nYou can [`read`](crate::Reg::read) this register and get [`winctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winctrl`] module"]
#[doc(alias = "WINCTRL")]
pub type Winctrl = crate::Reg<winctrl::WinctrlSpec>;
#[doc = "Window Control"]
pub mod winctrl;
#[doc = "WINLT (rw) register accessor: Window Low Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`winlt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winlt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winlt`] module"]
#[doc(alias = "WINLT")]
pub type Winlt = crate::Reg<winlt::WinltSpec>;
#[doc = "Window Low Threshold"]
pub mod winlt;
#[doc = "WINHT (rw) register accessor: Window High Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`winht::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winht::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winht`] module"]
#[doc(alias = "WINHT")]
pub type Winht = crate::Reg<winht::WinhtSpec>;
#[doc = "Window High Threshold"]
pub mod winht;
#[doc = "INPUTCTRL (rw) register accessor: Input Multiplexer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`inputctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inputctrl`] module"]
#[doc(alias = "INPUTCTRL")]
pub type Inputctrl = crate::Reg<inputctrl::InputctrlSpec>;
#[doc = "Input Multiplexer Control"]
pub mod inputctrl;
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`] module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "INTFLAGSET (rw) register accessor: Interrupt Flag Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagset`] module"]
#[doc(alias = "INTFLAGSET")]
pub type Intflagset = crate::Reg<intflagset::IntflagsetSpec>;
#[doc = "Interrupt Flag Set"]
pub mod intflagset;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "COMMAND (rw) register accessor: Command\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`] module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "Command"]
pub mod command;
#[doc = "RESULT (rw) register accessor: Result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`] module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "Result"]
pub mod result;
#[doc = "SAMPLE (rw) register accessor: Sample\n\nYou can [`read`](crate::Reg::read) this register and get [`sample::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample`] module"]
#[doc(alias = "SAMPLE")]
pub type Sample = crate::Reg<sample::SampleSpec>;
#[doc = "Sample"]
pub mod sample;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "WPCTRL (rw) register accessor: Write Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpctrl`] module"]
#[doc(alias = "WPCTRL")]
pub type Wpctrl = crate::Reg<wpctrl::WpctrlSpec>;
#[doc = "Write Protection Control"]
pub mod wpctrl;

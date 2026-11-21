#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    evctrl: Evctrl,
    dbgctrl: Dbgctrl,
    ctrlb: Ctrlb,
    ctrlc: Ctrlc,
    ctrld: Ctrld,
    ctrle: Ctrle,
    swtrig: Swtrig,
    winlt: Winlt,
    winht: Winht,
    pinen1: Pinen1,
    pinen2: Pinen2,
    xsel1: Xsel1,
    xsel2: Xsel2,
    ysel1: Ysel1,
    ysel2: Ysel2,
    polctrl1: Polctrl1,
    polctrl2: Polctrl2,
    intenclr: Intenclr,
    intenset: Intenset,
    intflagset: Intflagset,
    intflag: Intflag,
    status: Status,
    syncbusy: Syncbusy,
    dseqctrl: Dseqctrl,
    dseqstat: Dseqstat,
    dseqdata: Dseqdata,
    result: Result,
}
impl RegisterBlock {
    #[doc = "0x00 - CONTROL ENABLE REGISTER"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - EVENT CONTROL REGISTER"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x08 - DEBUG CONTROL Register"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x0c - CONTROL B Register"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x10 - CONTROL C Register"]
    #[inline(always)]
    pub const fn ctrlc(&self) -> &Ctrlc {
        &self.ctrlc
    }
    #[doc = "0x14 - CONTROL D Register"]
    #[inline(always)]
    pub const fn ctrld(&self) -> &Ctrld {
        &self.ctrld
    }
    #[doc = "0x18 - CONTROL E REGISTER"]
    #[inline(always)]
    pub const fn ctrle(&self) -> &Ctrle {
        &self.ctrle
    }
    #[doc = "0x1c - Software Trigger Register"]
    #[inline(always)]
    pub const fn swtrig(&self) -> &Swtrig {
        &self.swtrig
    }
    #[doc = "0x20 - Window Comparator Low Threshold Register"]
    #[inline(always)]
    pub const fn winlt(&self) -> &Winlt {
        &self.winlt
    }
    #[doc = "0x24 - Window Comparator High Threshold Register"]
    #[inline(always)]
    pub const fn winht(&self) -> &Winht {
        &self.winht
    }
    #[doc = "0x28 - PTC Line Enable 1 Register"]
    #[inline(always)]
    pub const fn pinen1(&self) -> &Pinen1 {
        &self.pinen1
    }
    #[doc = "0x2c - PTC Line Enable 2 Register"]
    #[inline(always)]
    pub const fn pinen2(&self) -> &Pinen2 {
        &self.pinen2
    }
    #[doc = "0x30 - X Line Selection 1 Register"]
    #[inline(always)]
    pub const fn xsel1(&self) -> &Xsel1 {
        &self.xsel1
    }
    #[doc = "0x34 - X Line Selection 2 Register"]
    #[inline(always)]
    pub const fn xsel2(&self) -> &Xsel2 {
        &self.xsel2
    }
    #[doc = "0x38 - Y Line Selection 1 Register"]
    #[inline(always)]
    pub const fn ysel1(&self) -> &Ysel1 {
        &self.ysel1
    }
    #[doc = "0x3c - Y Line Selection 2 Register"]
    #[inline(always)]
    pub const fn ysel2(&self) -> &Ysel2 {
        &self.ysel2
    }
    #[doc = "0x40 - Polarity Control 1 Register"]
    #[inline(always)]
    pub const fn polctrl1(&self) -> &Polctrl1 {
        &self.polctrl1
    }
    #[doc = "0x44 - Polarity Control 2 Register"]
    #[inline(always)]
    pub const fn polctrl2(&self) -> &Polctrl2 {
        &self.polctrl2
    }
    #[doc = "0x48 - INTERRUPT ENABLE CLEAR Register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x4c - INTERRUPT ENABLE SET Register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x50 - INTERRUPT FLAG SET Register"]
    #[inline(always)]
    pub const fn intflagset(&self) -> &Intflagset {
        &self.intflagset
    }
    #[doc = "0x54 - INTERRUPT FLAG STATUS and CLEAR Register"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x58 - STATUS Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x5c - Synchronization Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x60 - DMA SEQUENCE CTRL Register"]
    #[inline(always)]
    pub const fn dseqctrl(&self) -> &Dseqctrl {
        &self.dseqctrl
    }
    #[doc = "0x64 - DMA SEQUENCE STATUS - Register"]
    #[inline(always)]
    pub const fn dseqstat(&self) -> &Dseqstat {
        &self.dseqstat
    }
    #[doc = "0x68 - DMA SEQUENCE DATA - Register"]
    #[inline(always)]
    pub const fn dseqdata(&self) -> &Dseqdata {
        &self.dseqdata
    }
    #[doc = "0x6c - Register"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
}
#[doc = "CTRLA (rw) register accessor: CONTROL ENABLE REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "CONTROL ENABLE REGISTER"]
pub mod ctrla;
#[doc = "EVCTRL (rw) register accessor: EVENT CONTROL REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`] module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "EVENT CONTROL REGISTER"]
pub mod evctrl;
#[doc = "DBGCTRL (rw) register accessor: DEBUG CONTROL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "DEBUG CONTROL Register"]
pub mod dbgctrl;
#[doc = "CTRLB (rw) register accessor: CONTROL B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "CONTROL B Register"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: CONTROL C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlc`] module"]
#[doc(alias = "CTRLC")]
pub type Ctrlc = crate::Reg<ctrlc::CtrlcSpec>;
#[doc = "CONTROL C Register"]
pub mod ctrlc;
#[doc = "CTRLD (rw) register accessor: CONTROL D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrld`] module"]
#[doc(alias = "CTRLD")]
pub type Ctrld = crate::Reg<ctrld::CtrldSpec>;
#[doc = "CONTROL D Register"]
pub mod ctrld;
#[doc = "CTRLE (rw) register accessor: CONTROL E REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrle`] module"]
#[doc(alias = "CTRLE")]
pub type Ctrle = crate::Reg<ctrle::CtrleSpec>;
#[doc = "CONTROL E REGISTER"]
pub mod ctrle;
#[doc = "SWTRIG (rw) register accessor: Software Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrig`] module"]
#[doc(alias = "SWTRIG")]
pub type Swtrig = crate::Reg<swtrig::SwtrigSpec>;
#[doc = "Software Trigger Register"]
pub mod swtrig;
#[doc = "WINLT (rw) register accessor: Window Comparator Low Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`winlt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winlt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winlt`] module"]
#[doc(alias = "WINLT")]
pub type Winlt = crate::Reg<winlt::WinltSpec>;
#[doc = "Window Comparator Low Threshold Register"]
pub mod winlt;
#[doc = "WINHT (rw) register accessor: Window Comparator High Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`winht::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winht::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winht`] module"]
#[doc(alias = "WINHT")]
pub type Winht = crate::Reg<winht::WinhtSpec>;
#[doc = "Window Comparator High Threshold Register"]
pub mod winht;
#[doc = "PINEN1 (rw) register accessor: PTC Line Enable 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pinen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinen1`] module"]
#[doc(alias = "PINEN1")]
pub type Pinen1 = crate::Reg<pinen1::Pinen1Spec>;
#[doc = "PTC Line Enable 1 Register"]
pub mod pinen1;
#[doc = "PINEN2 (rw) register accessor: PTC Line Enable 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pinen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinen2`] module"]
#[doc(alias = "PINEN2")]
pub type Pinen2 = crate::Reg<pinen2::Pinen2Spec>;
#[doc = "PTC Line Enable 2 Register"]
pub mod pinen2;
#[doc = "XSEL1 (rw) register accessor: X Line Selection 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xsel1`] module"]
#[doc(alias = "XSEL1")]
pub type Xsel1 = crate::Reg<xsel1::Xsel1Spec>;
#[doc = "X Line Selection 1 Register"]
pub mod xsel1;
#[doc = "XSEL2 (rw) register accessor: X Line Selection 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xsel2`] module"]
#[doc(alias = "XSEL2")]
pub type Xsel2 = crate::Reg<xsel2::Xsel2Spec>;
#[doc = "X Line Selection 2 Register"]
pub mod xsel2;
#[doc = "YSEL1 (rw) register accessor: Y Line Selection 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ysel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ysel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ysel1`] module"]
#[doc(alias = "YSEL1")]
pub type Ysel1 = crate::Reg<ysel1::Ysel1Spec>;
#[doc = "Y Line Selection 1 Register"]
pub mod ysel1;
#[doc = "YSEL2 (rw) register accessor: Y Line Selection 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ysel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ysel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ysel2`] module"]
#[doc(alias = "YSEL2")]
pub type Ysel2 = crate::Reg<ysel2::Ysel2Spec>;
#[doc = "Y Line Selection 2 Register"]
pub mod ysel2;
#[doc = "POLCTRL1 (rw) register accessor: Polarity Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`polctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polctrl1`] module"]
#[doc(alias = "POLCTRL1")]
pub type Polctrl1 = crate::Reg<polctrl1::Polctrl1Spec>;
#[doc = "Polarity Control 1 Register"]
pub mod polctrl1;
#[doc = "POLCTRL2 (rw) register accessor: Polarity Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`polctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polctrl2`] module"]
#[doc(alias = "POLCTRL2")]
pub type Polctrl2 = crate::Reg<polctrl2::Polctrl2Spec>;
#[doc = "Polarity Control 2 Register"]
pub mod polctrl2;
#[doc = "INTENCLR (rw) register accessor: INTERRUPT ENABLE CLEAR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "INTERRUPT ENABLE CLEAR Register"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: INTERRUPT ENABLE SET Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "INTERRUPT ENABLE SET Register"]
pub mod intenset;
#[doc = "INTFLAGSET (rw) register accessor: INTERRUPT FLAG SET Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagset`] module"]
#[doc(alias = "INTFLAGSET")]
pub type Intflagset = crate::Reg<intflagset::IntflagsetSpec>;
#[doc = "INTERRUPT FLAG SET Register"]
pub mod intflagset;
#[doc = "INTFLAG (rw) register accessor: INTERRUPT FLAG STATUS and CLEAR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "INTERRUPT FLAG STATUS and CLEAR Register"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: STATUS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "STATUS Register"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Register"]
pub mod syncbusy;
#[doc = "DSEQCTRL (rw) register accessor: DMA SEQUENCE CTRL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dseqctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dseqctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dseqctrl`] module"]
#[doc(alias = "DSEQCTRL")]
pub type Dseqctrl = crate::Reg<dseqctrl::DseqctrlSpec>;
#[doc = "DMA SEQUENCE CTRL Register"]
pub mod dseqctrl;
#[doc = "DSEQSTAT (r) register accessor: DMA SEQUENCE STATUS - Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dseqstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dseqstat`] module"]
#[doc(alias = "DSEQSTAT")]
pub type Dseqstat = crate::Reg<dseqstat::DseqstatSpec>;
#[doc = "DMA SEQUENCE STATUS - Register"]
pub mod dseqstat;
#[doc = "DSEQDATA (w) register accessor: DMA SEQUENCE DATA - Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dseqdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dseqdata`] module"]
#[doc(alias = "DSEQDATA")]
pub type Dseqdata = crate::Reg<dseqdata::DseqdataSpec>;
#[doc = "DMA SEQUENCE DATA - Register"]
pub mod dseqdata;
#[doc = "RESULT (r) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`] module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "Register"]
pub mod result;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlbclr: Ctrlbclr,
    ctrlbset: Ctrlbset,
    _reserved3: [u8; 0x02],
    syncbusy: Syncbusy,
    fctrla: Fctrla,
    fctrlb: Fctrlb,
    wexctrl: Wexctrl,
    drvctrl: Drvctrl,
    _reserved8: [u8; 0x02],
    dbgctrl: Dbgctrl,
    _reserved9: [u8; 0x01],
    evctrl: Evctrl,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    count: Count,
    patt: Patt,
    _reserved16: [u8; 0x02],
    _reserved_16_wave: [u8; 0x04],
    per: Per,
    cc: [Cc; 4],
    _reserved19: [u8; 0x10],
    pattbuf: Pattbuf,
    _reserved20: [u8; 0x06],
    perbuf: Perbuf,
    ccbuf: [Ccbuf; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Control B Clear"]
    #[inline(always)]
    pub const fn ctrlbclr(&self) -> &Ctrlbclr {
        &self.ctrlbclr
    }
    #[doc = "0x05 - Control B Set"]
    #[inline(always)]
    pub const fn ctrlbset(&self) -> &Ctrlbset {
        &self.ctrlbset
    }
    #[doc = "0x08 - Synchronization Busy"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x0c - Recoverable Fault Control #"]
    #[inline(always)]
    pub const fn fctrla(&self) -> &Fctrla {
        &self.fctrla
    }
    #[doc = "0x10 - Recoverable Fault Control #"]
    #[inline(always)]
    pub const fn fctrlb(&self) -> &Fctrlb {
        &self.fctrlb
    }
    #[doc = "0x14 - Waveform Extension Control"]
    #[inline(always)]
    pub const fn wexctrl(&self) -> &Wexctrl {
        &self.wexctrl
    }
    #[doc = "0x18 - Driver Control"]
    #[inline(always)]
    pub const fn drvctrl(&self) -> &Drvctrl {
        &self.drvctrl
    }
    #[doc = "0x1e - Debug control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x20 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x24 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x28 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x2c - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x30 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x34 - Counter"]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x38 - Pattern"]
    #[inline(always)]
    pub const fn patt(&self) -> &Patt {
        &self.patt
    }
    #[doc = "0x3c - Waveform"]
    #[inline(always)]
    pub const fn wave_singleslope_mode(&self) -> &WaveSingleslopeMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Waveform"]
    #[inline(always)]
    pub const fn wave_dualslope_mode(&self) -> &WaveDualslopeMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Waveform"]
    #[inline(always)]
    pub const fn wave_dualcompare_mode(&self) -> &WaveDualcompareMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Waveform"]
    #[inline(always)]
    pub const fn wave(&self) -> &Wave {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per(&self) -> &Per {
        &self.per
    }
    #[doc = "0x44..0x54 - Compare/Capture Channel #"]
    #[inline(always)]
    pub const fn cc(&self, n: usize) -> &Cc {
        &self.cc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x54 - Compare/Capture Channel #"]
    #[inline(always)]
    pub fn cc_iter(&self) -> impl Iterator<Item = &Cc> {
        self.cc.iter()
    }
    #[doc = "0x64 - Pattern Buffer"]
    #[inline(always)]
    pub const fn pattbuf(&self) -> &Pattbuf {
        &self.pattbuf
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perbuf(&self) -> &Perbuf {
        &self.perbuf
    }
    #[doc = "0x70..0x80 - Compare/Capture Channel # Buffer"]
    #[inline(always)]
    pub const fn ccbuf(&self, n: usize) -> &Ccbuf {
        &self.ccbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - Compare/Capture Channel # Buffer"]
    #[inline(always)]
    pub fn ccbuf_iter(&self) -> impl Iterator<Item = &Ccbuf> {
        self.ccbuf.iter()
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLBCLR (rw) register accessor: Control B Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlbclr`] module"]
#[doc(alias = "CTRLBCLR")]
pub type Ctrlbclr = crate::Reg<ctrlbclr::CtrlbclrSpec>;
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "CTRLBSET (rw) register accessor: Control B Set\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlbset`] module"]
#[doc(alias = "CTRLBSET")]
pub type Ctrlbset = crate::Reg<ctrlbset::CtrlbsetSpec>;
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "FCTRLA (rw) register accessor: Recoverable Fault Control #\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrla`] module"]
#[doc(alias = "FCTRLA")]
pub type Fctrla = crate::Reg<fctrla::FctrlaSpec>;
#[doc = "Recoverable Fault Control #"]
pub mod fctrla;
#[doc = "FCTRLB (rw) register accessor: Recoverable Fault Control #\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrlb`] module"]
#[doc(alias = "FCTRLB")]
pub type Fctrlb = crate::Reg<fctrlb::FctrlbSpec>;
#[doc = "Recoverable Fault Control #"]
pub mod fctrlb;
#[doc = "WEXCTRL (rw) register accessor: Waveform Extension Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wexctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wexctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wexctrl`] module"]
#[doc(alias = "WEXCTRL")]
pub type Wexctrl = crate::Reg<wexctrl::WexctrlSpec>;
#[doc = "Waveform Extension Control"]
pub mod wexctrl;
#[doc = "DRVCTRL (rw) register accessor: Driver Control\n\nYou can [`read`](crate::Reg::read) this register and get [`drvctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drvctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drvctrl`] module"]
#[doc(alias = "DRVCTRL")]
pub type Drvctrl = crate::Reg<drvctrl::DrvctrlSpec>;
#[doc = "Driver Control"]
pub mod drvctrl;
#[doc = "DBGCTRL (rw) register accessor: Debug control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug control"]
pub mod dbgctrl;
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
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "COUNT (rw) register accessor: Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`] module"]
#[doc(alias = "COUNT")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "Counter"]
pub mod count;
#[doc = "PATT (rw) register accessor: Pattern\n\nYou can [`read`](crate::Reg::read) this register and get [`patt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patt`] module"]
#[doc(alias = "PATT")]
pub type Patt = crate::Reg<patt::PattSpec>;
#[doc = "Pattern"]
pub mod patt;
#[doc = "WAVE (rw) register accessor: Waveform\n\nYou can [`read`](crate::Reg::read) this register and get [`wave::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wave::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave`] module"]
#[doc(alias = "WAVE")]
pub type Wave = crate::Reg<wave::WaveSpec>;
#[doc = "Waveform"]
pub mod wave;
#[doc = "WAVE_DUALCOMPARE_MODE (rw) register accessor: Waveform\n\nYou can [`read`](crate::Reg::read) this register and get [`wave_dualcompare_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wave_dualcompare_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave_dualcompare_mode`] module"]
#[doc(alias = "WAVE_DUALCOMPARE_MODE")]
pub type WaveDualcompareMode = crate::Reg<wave_dualcompare_mode::WaveDualcompareModeSpec>;
#[doc = "Waveform"]
pub mod wave_dualcompare_mode;
#[doc = "WAVE_DUALSLOPE_MODE (rw) register accessor: Waveform\n\nYou can [`read`](crate::Reg::read) this register and get [`wave_dualslope_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wave_dualslope_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave_dualslope_mode`] module"]
#[doc(alias = "WAVE_DUALSLOPE_MODE")]
pub type WaveDualslopeMode = crate::Reg<wave_dualslope_mode::WaveDualslopeModeSpec>;
#[doc = "Waveform"]
pub mod wave_dualslope_mode;
#[doc = "WAVE_SINGLESLOPE_MODE (rw) register accessor: Waveform\n\nYou can [`read`](crate::Reg::read) this register and get [`wave_singleslope_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wave_singleslope_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave_singleslope_mode`] module"]
#[doc(alias = "WAVE_SINGLESLOPE_MODE")]
pub type WaveSingleslopeMode = crate::Reg<wave_singleslope_mode::WaveSingleslopeModeSpec>;
#[doc = "Waveform"]
pub mod wave_singleslope_mode;
#[doc = "PER (rw) register accessor: Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per`] module"]
#[doc(alias = "PER")]
pub type Per = crate::Reg<per::PerSpec>;
#[doc = "Period"]
pub mod per;
#[doc = "CC (rw) register accessor: Compare/Capture Channel #\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`] module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "Compare/Capture Channel #"]
pub mod cc;
#[doc = "PATTBUF (rw) register accessor: Pattern Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`pattbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pattbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pattbuf`] module"]
#[doc(alias = "PATTBUF")]
pub type Pattbuf = crate::Reg<pattbuf::PattbufSpec>;
#[doc = "Pattern Buffer"]
pub mod pattbuf;
#[doc = "PERBUF (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbuf`] module"]
#[doc(alias = "PERBUF")]
pub type Perbuf = crate::Reg<perbuf::PerbufSpec>;
#[doc = "Period Buffer"]
pub mod perbuf;
#[doc = "CCBUF (rw) register accessor: Compare/Capture Channel # Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccbuf`] module"]
#[doc(alias = "CCBUF")]
pub type Ccbuf = crate::Reg<ccbuf::CcbufSpec>;
#[doc = "Compare/Capture Channel # Buffer"]
pub mod ccbuf;

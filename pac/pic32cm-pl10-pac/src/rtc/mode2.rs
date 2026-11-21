#[repr(C)]
#[doc = "Clock and Calendar Mode with Alarm"]
#[doc(alias = "MODE2")]
pub struct Mode2 {
    ctrla: Ctrla,
    _reserved1: [u8; 0x02],
    evctrl: Evctrl,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    dbgctrl: Dbgctrl,
    _reserved6: [u8; 0x01],
    syncbusy: Syncbusy,
    freqcorr: Freqcorr,
    _reserved8: [u8; 0x03],
    _reserved_8_clock: [u8; 0x04],
    _reserved9: [u8; 0x04],
    _reserved_9_alarm: [u8; 0x04],
    mask0: Mask0,
}
impl Mode2 {
    #[doc = "0x00 - MODE2 Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - MODE2 Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x08 - MODE2 Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x0a - MODE2 Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x0c - MODE2 Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x0e - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x10 - MODE2 Synchronization Busy"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x14 - Frequency Correction"]
    #[inline(always)]
    pub const fn freqcorr(&self) -> &Freqcorr {
        &self.freqcorr
    }
    #[doc = "0x18 - MODE2 Clock Value"]
    #[inline(always)]
    pub const fn clock_clk24h_mode(&self) -> &ClockClk24hMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - MODE2 Clock Value"]
    #[inline(always)]
    pub const fn clock_clk12h_mode(&self) -> &ClockClk12hMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - MODE2 Clock Value"]
    #[inline(always)]
    pub const fn clock(&self) -> &Clock {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x20 - MODE2 Alarm 0 Value"]
    #[inline(always)]
    pub const fn alarm0_clk24h_mode(&self) -> &Alarm0Clk24hMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - MODE2 Alarm 0 Value"]
    #[inline(always)]
    pub const fn alarm0_clk12h_mode(&self) -> &Alarm0Clk12hMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - MODE2 Alarm 0 Value"]
    #[inline(always)]
    pub const fn alarm0(&self) -> &Alarm0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - MODE2 Alarm 0 Mask"]
    #[inline(always)]
    pub const fn mask0(&self) -> &Mask0 {
        &self.mask0
    }
}
#[doc = "CTRLA (rw) register accessor: MODE2 Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "MODE2 Control A"]
pub mod ctrla;
#[doc = "EVCTRL (rw) register accessor: MODE2 Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`] module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "MODE2 Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: MODE2 Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "MODE2 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: MODE2 Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "MODE2 Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: MODE2 Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "MODE2 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SYNCBUSY (r) register accessor: MODE2 Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "MODE2 Synchronization Busy"]
pub mod syncbusy;
#[doc = "FREQCORR (rw) register accessor: Frequency Correction\n\nYou can [`read`](crate::Reg::read) this register and get [`freqcorr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqcorr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqcorr`] module"]
#[doc(alias = "FREQCORR")]
pub type Freqcorr = crate::Reg<freqcorr::FreqcorrSpec>;
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "CLOCK (rw) register accessor: MODE2 Clock Value\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
#[doc(alias = "CLOCK")]
pub type Clock = crate::Reg<clock::ClockSpec>;
#[doc = "MODE2 Clock Value"]
pub mod clock;
#[doc = "CLOCK_CLK12H_MODE (rw) register accessor: MODE2 Clock Value\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_clk12h_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_clk12h_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_clk12h_mode`] module"]
#[doc(alias = "CLOCK_CLK12H_MODE")]
pub type ClockClk12hMode = crate::Reg<clock_clk12h_mode::ClockClk12hModeSpec>;
#[doc = "MODE2 Clock Value"]
pub mod clock_clk12h_mode;
#[doc = "CLOCK_CLK24H_MODE (rw) register accessor: MODE2 Clock Value\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_clk24h_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_clk24h_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_clk24h_mode`] module"]
#[doc(alias = "CLOCK_CLK24H_MODE")]
pub type ClockClk24hMode = crate::Reg<clock_clk24h_mode::ClockClk24hModeSpec>;
#[doc = "MODE2 Clock Value"]
pub mod clock_clk24h_mode;
#[doc = "ALARM0 (rw) register accessor: MODE2 Alarm 0 Value\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0`] module"]
#[doc(alias = "ALARM0")]
pub type Alarm0 = crate::Reg<alarm0::Alarm0Spec>;
#[doc = "MODE2 Alarm 0 Value"]
pub mod alarm0;
#[doc = "ALARM0_CLK12H_MODE (rw) register accessor: MODE2 Alarm 0 Value\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0_clk12h_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0_clk12h_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0_clk12h_mode`] module"]
#[doc(alias = "ALARM0_CLK12H_MODE")]
pub type Alarm0Clk12hMode = crate::Reg<alarm0_clk12h_mode::Alarm0Clk12hModeSpec>;
#[doc = "MODE2 Alarm 0 Value"]
pub mod alarm0_clk12h_mode;
#[doc = "ALARM0_CLK24H_MODE (rw) register accessor: MODE2 Alarm 0 Value\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0_clk24h_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0_clk24h_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0_clk24h_mode`] module"]
#[doc(alias = "ALARM0_CLK24H_MODE")]
pub type Alarm0Clk24hMode = crate::Reg<alarm0_clk24h_mode::Alarm0Clk24hModeSpec>;
#[doc = "MODE2 Alarm 0 Value"]
pub mod alarm0_clk24h_mode;
#[doc = "MASK0 (rw) register accessor: MODE2 Alarm 0 Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask0`] module"]
#[doc(alias = "MASK0")]
pub type Mask0 = crate::Reg<mask0::Mask0Spec>;
#[doc = "MODE2 Alarm 0 Mask"]
pub mod mask0;

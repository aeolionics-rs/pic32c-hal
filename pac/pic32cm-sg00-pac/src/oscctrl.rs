#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    evctrl: Evctrl,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    xoscctrla: Xoscctrla,
    xoscctrlb: Xoscctrlb,
    _reserved7: [u8; 0x10],
    dfllctrla: Dfllctrla,
    dfllctrlb: Dfllctrlb,
    dflltune: Dflltune,
    dflldiff: Dflldiff,
    dfllmul: Dfllmul,
    pll0ctrl: Pll0ctrl,
    pll0fbdiv: Pll0fbdiv,
    pll0refdiv: Pll0refdiv,
    pll0postdiva: Pll0postdiva,
    pll0postdivb: Pll0postdivb,
    _reserved17: [u8; 0x24],
    syncbusy: Syncbusy,
    _reserved18: [u8; 0x04],
    _reserved_18_xosccal: [u8; 0x04],
    _reserved19: [u8; 0x04],
    _reserved_19_rc48mcal: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
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
    #[doc = "0x10 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - External Multipurpose Crystal Oscillator Control A"]
    #[inline(always)]
    pub const fn xoscctrla(&self) -> &Xoscctrla {
        &self.xoscctrla
    }
    #[doc = "0x18 - External Multipurpose Crystal Oscillator Control B"]
    #[inline(always)]
    pub const fn xoscctrlb(&self) -> &Xoscctrlb {
        &self.xoscctrlb
    }
    #[doc = "0x2c - DFLL48M Control A"]
    #[inline(always)]
    pub const fn dfllctrla(&self) -> &Dfllctrla {
        &self.dfllctrla
    }
    #[doc = "0x30 - DFLL48M Control B"]
    #[inline(always)]
    pub const fn dfllctrlb(&self) -> &Dfllctrlb {
        &self.dfllctrlb
    }
    #[doc = "0x34 - DFLL48M Tune"]
    #[inline(always)]
    pub const fn dflltune(&self) -> &Dflltune {
        &self.dflltune
    }
    #[doc = "0x38 - DFLL48M Diff"]
    #[inline(always)]
    pub const fn dflldiff(&self) -> &Dflldiff {
        &self.dflldiff
    }
    #[doc = "0x3c - DFLL48M Multiplier"]
    #[inline(always)]
    pub const fn dfllmul(&self) -> &Dfllmul {
        &self.dfllmul
    }
    #[doc = "0x40 - PLL Control"]
    #[inline(always)]
    pub const fn pll0ctrl(&self) -> &Pll0ctrl {
        &self.pll0ctrl
    }
    #[doc = "0x44 - PLL Feed-Back Divider"]
    #[inline(always)]
    pub const fn pll0fbdiv(&self) -> &Pll0fbdiv {
        &self.pll0fbdiv
    }
    #[doc = "0x48 - PLL reference divider"]
    #[inline(always)]
    pub const fn pll0refdiv(&self) -> &Pll0refdiv {
        &self.pll0refdiv
    }
    #[doc = "0x4c - PLL output clock divider A"]
    #[inline(always)]
    pub const fn pll0postdiva(&self) -> &Pll0postdiva {
        &self.pll0postdiva
    }
    #[doc = "0x50 - PLL output clock divider B"]
    #[inline(always)]
    pub const fn pll0postdivb(&self) -> &Pll0postdivb {
        &self.pll0postdivb
    }
    #[doc = "0x78 - Synchronization Busy"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x80 - XOSC Calibration Register"]
    #[inline(always)]
    pub const fn xosccal_fuses_osc_xtal_hf_agc_v2_mode(&self) -> &XosccalFusesOscXtalHfAgcV2Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x80 - XOSC Calibration Register"]
    #[inline(always)]
    pub const fn xosccal(&self) -> &Xosccal {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x88 - RC48M Calibration 0"]
    #[inline(always)]
    pub const fn rc48mcal0_fuses_osc_rc48mhz_v1_mode(&self) -> &Rc48mcal0FusesOscRc48mhzV1Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x88 - RC48M Calibration 0"]
    #[inline(always)]
    pub const fn rc48mcal0(&self) -> &Rc48mcal0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
}
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
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "XOSCCTRLA (rw) register accessor: External Multipurpose Crystal Oscillator Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`xoscctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xoscctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xoscctrla`] module"]
#[doc(alias = "XOSCCTRLA")]
pub type Xoscctrla = crate::Reg<xoscctrla::XoscctrlaSpec>;
#[doc = "External Multipurpose Crystal Oscillator Control A"]
pub mod xoscctrla;
#[doc = "XOSCCTRLB (rw) register accessor: External Multipurpose Crystal Oscillator Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`xoscctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xoscctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xoscctrlb`] module"]
#[doc(alias = "XOSCCTRLB")]
pub type Xoscctrlb = crate::Reg<xoscctrlb::XoscctrlbSpec>;
#[doc = "External Multipurpose Crystal Oscillator Control B"]
pub mod xoscctrlb;
#[doc = "DFLLCTRLA (rw) register accessor: DFLL48M Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllctrla`] module"]
#[doc(alias = "DFLLCTRLA")]
pub type Dfllctrla = crate::Reg<dfllctrla::DfllctrlaSpec>;
#[doc = "DFLL48M Control A"]
pub mod dfllctrla;
#[doc = "DFLLCTRLB (rw) register accessor: DFLL48M Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllctrlb`] module"]
#[doc(alias = "DFLLCTRLB")]
pub type Dfllctrlb = crate::Reg<dfllctrlb::DfllctrlbSpec>;
#[doc = "DFLL48M Control B"]
pub mod dfllctrlb;
#[doc = "DFLLTUNE (rw) register accessor: DFLL48M Tune\n\nYou can [`read`](crate::Reg::read) this register and get [`dflltune::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflltune::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dflltune`] module"]
#[doc(alias = "DFLLTUNE")]
pub type Dflltune = crate::Reg<dflltune::DflltuneSpec>;
#[doc = "DFLL48M Tune"]
pub mod dflltune;
#[doc = "DFLLDIFF (r) register accessor: DFLL48M Diff\n\nYou can [`read`](crate::Reg::read) this register and get [`dflldiff::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dflldiff`] module"]
#[doc(alias = "DFLLDIFF")]
pub type Dflldiff = crate::Reg<dflldiff::DflldiffSpec>;
#[doc = "DFLL48M Diff"]
pub mod dflldiff;
#[doc = "DFLLMUL (rw) register accessor: DFLL48M Multiplier\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllmul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllmul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllmul`] module"]
#[doc(alias = "DFLLMUL")]
pub type Dfllmul = crate::Reg<dfllmul::DfllmulSpec>;
#[doc = "DFLL48M Multiplier"]
pub mod dfllmul;
#[doc = "PLL0CTRL (rw) register accessor: PLL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0ctrl`] module"]
#[doc(alias = "PLL0CTRL")]
pub type Pll0ctrl = crate::Reg<pll0ctrl::Pll0ctrlSpec>;
#[doc = "PLL Control"]
pub mod pll0ctrl;
#[doc = "PLL0FBDIV (rw) register accessor: PLL Feed-Back Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0fbdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0fbdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0fbdiv`] module"]
#[doc(alias = "PLL0FBDIV")]
pub type Pll0fbdiv = crate::Reg<pll0fbdiv::Pll0fbdivSpec>;
#[doc = "PLL Feed-Back Divider"]
pub mod pll0fbdiv;
#[doc = "PLL0REFDIV (rw) register accessor: PLL reference divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0refdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0refdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0refdiv`] module"]
#[doc(alias = "PLL0REFDIV")]
pub type Pll0refdiv = crate::Reg<pll0refdiv::Pll0refdivSpec>;
#[doc = "PLL reference divider"]
pub mod pll0refdiv;
#[doc = "PLL0POSTDIVA (rw) register accessor: PLL output clock divider A\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0postdiva::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0postdiva::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0postdiva`] module"]
#[doc(alias = "PLL0POSTDIVA")]
pub type Pll0postdiva = crate::Reg<pll0postdiva::Pll0postdivaSpec>;
#[doc = "PLL output clock divider A"]
pub mod pll0postdiva;
#[doc = "PLL0POSTDIVB (rw) register accessor: PLL output clock divider B\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0postdivb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0postdivb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0postdivb`] module"]
#[doc(alias = "PLL0POSTDIVB")]
pub type Pll0postdivb = crate::Reg<pll0postdivb::Pll0postdivbSpec>;
#[doc = "PLL output clock divider B"]
pub mod pll0postdivb;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "XOSCCAL (rw) register accessor: XOSC Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xosccal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosccal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosccal`] module"]
#[doc(alias = "XOSCCAL")]
pub type Xosccal = crate::Reg<xosccal::XosccalSpec>;
#[doc = "XOSC Calibration Register"]
pub mod xosccal;
#[doc = "XOSCCAL_FUSES_OSC_XTAL_HF_AGC_V2_MODE (rw) register accessor: XOSC Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xosccal_fuses_osc_xtal_hf_agc_v2_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosccal_fuses_osc_xtal_hf_agc_v2_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosccal_fuses_osc_xtal_hf_agc_v2_mode`] module"]
#[doc(alias = "XOSCCAL_FUSES_OSC_XTAL_HF_AGC_V2_MODE")]
pub type XosccalFusesOscXtalHfAgcV2Mode = crate::Reg<xosccal_fuses_osc_xtal_hf_agc_v2_mode::XosccalFusesOscXtalHfAgcV2ModeSpec>;
#[doc = "XOSC Calibration Register"]
pub mod xosccal_fuses_osc_xtal_hf_agc_v2_mode;
#[doc = "RC48MCAL0 (rw) register accessor: RC48M Calibration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rc48mcal0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc48mcal0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc48mcal0`] module"]
#[doc(alias = "RC48MCAL0")]
pub type Rc48mcal0 = crate::Reg<rc48mcal0::Rc48mcal0Spec>;
#[doc = "RC48M Calibration 0"]
pub mod rc48mcal0;
#[doc = "RC48MCAL0_FUSES_OSC_RC48MHZ_V1_MODE (rw) register accessor: RC48M Calibration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rc48mcal0_fuses_osc_rc48mhz_v1_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc48mcal0_fuses_osc_rc48mhz_v1_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc48mcal0_fuses_osc_rc48mhz_v1_mode`] module"]
#[doc(alias = "RC48MCAL0_FUSES_OSC_RC48MHZ_V1_MODE")]
pub type Rc48mcal0FusesOscRc48mhzV1Mode = crate::Reg<rc48mcal0_fuses_osc_rc48mhz_v1_mode::Rc48mcal0FusesOscRc48mhzV1ModeSpec>;
#[doc = "RC48M Calibration 0"]
pub mod rc48mcal0_fuses_osc_rc48mhz_v1_mode;

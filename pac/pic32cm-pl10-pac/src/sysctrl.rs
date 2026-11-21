#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    bootctrl: Bootctrl,
    usercfg: Usercfg,
    tempsense: Tempsense,
}
impl RegisterBlock {
    #[doc = "0x80 - Boot Control"]
    #[inline(always)]
    pub const fn bootctrl(&self) -> &Bootctrl {
        &self.bootctrl
    }
    #[doc = "0x84 - User Configuration"]
    #[inline(always)]
    pub const fn usercfg(&self) -> &Usercfg {
        &self.usercfg
    }
    #[doc = "0x88 - Temperature Sensor Calibration"]
    #[inline(always)]
    pub const fn tempsense(&self) -> &Tempsense {
        &self.tempsense
    }
}
#[doc = "BOOTCTRL (rw) register accessor: Boot Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bootctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootctrl`] module"]
#[doc(alias = "BOOTCTRL")]
pub type Bootctrl = crate::Reg<bootctrl::BootctrlSpec>;
#[doc = "Boot Control"]
pub mod bootctrl;
#[doc = "USERCFG (rw) register accessor: User Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`usercfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usercfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usercfg`] module"]
#[doc(alias = "USERCFG")]
pub type Usercfg = crate::Reg<usercfg::UsercfgSpec>;
#[doc = "User Configuration"]
pub mod usercfg;
#[doc = "TEMPSENSE (r) register accessor: Temperature Sensor Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`tempsense::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempsense`] module"]
#[doc(alias = "TEMPSENSE")]
pub type Tempsense = crate::Reg<tempsense::TempsenseSpec>;
#[doc = "Temperature Sensor Calibration"]
pub mod tempsense;

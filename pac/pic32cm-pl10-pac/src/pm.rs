#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    sleepcfg: Sleepcfg,
    _reserved1: [u8; 0x04],
    wpctrl: Wpctrl,
}
impl RegisterBlock {
    #[doc = "0x04 - Sleep Configuration"]
    #[inline(always)]
    pub const fn sleepcfg(&self) -> &Sleepcfg {
        &self.sleepcfg
    }
    #[doc = "0x0c - Write Protection Control"]
    #[inline(always)]
    pub const fn wpctrl(&self) -> &Wpctrl {
        &self.wpctrl
    }
}
#[doc = "SLEEPCFG (rw) register accessor: Sleep Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sleepcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleepcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepcfg`] module"]
#[doc(alias = "SLEEPCFG")]
pub type Sleepcfg = crate::Reg<sleepcfg::SleepcfgSpec>;
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
#[doc = "WPCTRL (rw) register accessor: Write Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpctrl`] module"]
#[doc(alias = "WPCTRL")]
pub type Wpctrl = crate::Reg<wpctrl::WpctrlSpec>;
#[doc = "Write Protection Control"]
pub mod wpctrl;

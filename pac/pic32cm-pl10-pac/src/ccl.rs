#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    seqctrl: Seqctrl,
    lutctrl: [Lutctrl; 4],
    _reserved3: [u8; 0x10],
    wpctrl: Wpctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Sequential Control"]
    #[inline(always)]
    pub const fn seqctrl(&self) -> &Seqctrl {
        &self.seqctrl
    }
    #[doc = "0x08..0x18 - LUT Control #"]
    #[inline(always)]
    pub const fn lutctrl(&self, n: usize) -> &Lutctrl {
        &self.lutctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x18 - LUT Control #"]
    #[inline(always)]
    pub fn lutctrl_iter(&self) -> impl Iterator<Item = &Lutctrl> {
        self.lutctrl.iter()
    }
    #[doc = "0x28 - Write Protection Control"]
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
#[doc = "SEQCTRL (rw) register accessor: Sequential Control\n\nYou can [`read`](crate::Reg::read) this register and get [`seqctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqctrl`] module"]
#[doc(alias = "SEQCTRL")]
pub type Seqctrl = crate::Reg<seqctrl::SeqctrlSpec>;
#[doc = "Sequential Control"]
pub mod seqctrl;
#[doc = "LUTCTRL (rw) register accessor: LUT Control #\n\nYou can [`read`](crate::Reg::read) this register and get [`lutctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lutctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lutctrl`] module"]
#[doc(alias = "LUTCTRL")]
pub type Lutctrl = crate::Reg<lutctrl::LutctrlSpec>;
#[doc = "LUT Control #"]
pub mod lutctrl;
#[doc = "WPCTRL (rw) register accessor: Write Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpctrl`] module"]
#[doc(alias = "WPCTRL")]
pub type Wpctrl = crate::Reg<wpctrl::WpctrlSpec>;
#[doc = "Write Protection Control"]
pub mod wpctrl;

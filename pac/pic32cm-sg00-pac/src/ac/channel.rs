#[repr(C)]
#[doc = "CHANNEL\\[%s\\]"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    compctrl: [Compctrl; 2],
    dacctrl: Dacctrl,
    winctrl: Winctrl,
}
impl Channel {
    #[doc = "0x00..0x08 - Pair n Comparator Control 0"]
    #[inline(always)]
    pub const fn compctrl(&self, n: usize) -> &Compctrl {
        &self.compctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - Pair n Comparator Control 0"]
    #[inline(always)]
    pub fn compctrl_iter(&self) -> impl Iterator<Item = &Compctrl> {
        self.compctrl.iter()
    }
    #[doc = "0x08 - Dac n Control"]
    #[inline(always)]
    pub const fn dacctrl(&self) -> &Dacctrl {
        &self.dacctrl
    }
    #[doc = "0x0c - Window Monitor n Control"]
    #[inline(always)]
    pub const fn winctrl(&self) -> &Winctrl {
        &self.winctrl
    }
}
#[doc = "COMPCTRL (rw) register accessor: Pair n Comparator Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`compctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compctrl`] module"]
#[doc(alias = "COMPCTRL")]
pub type Compctrl = crate::Reg<compctrl::CompctrlSpec>;
#[doc = "Pair n Comparator Control 0"]
pub mod compctrl;
#[doc = "DACCTRL (rw) register accessor: Dac n Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dacctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacctrl`] module"]
#[doc(alias = "DACCTRL")]
pub type Dacctrl = crate::Reg<dacctrl::DacctrlSpec>;
#[doc = "Dac n Control"]
pub mod dacctrl;
#[doc = "WINCTRL (rw) register accessor: Window Monitor n Control\n\nYou can [`read`](crate::Reg::read) this register and get [`winctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winctrl`] module"]
#[doc(alias = "WINCTRL")]
pub type Winctrl = crate::Reg<winctrl::WinctrlSpec>;
#[doc = "Window Monitor n Control"]
pub mod winctrl;

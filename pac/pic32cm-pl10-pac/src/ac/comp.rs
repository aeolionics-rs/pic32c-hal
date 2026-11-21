#[repr(C)]
#[doc = "COMP\\[%s\\]"]
#[doc(alias = "COMP")]
pub struct Comp {
    compctrla: Compctrla,
    compctrlb: Compctrlb,
    compscaler: Compscaler,
}
impl Comp {
    #[doc = "0x00 - Comparator # Comparator Control A"]
    #[inline(always)]
    pub const fn compctrla(&self) -> &Compctrla {
        &self.compctrla
    }
    #[doc = "0x04 - Comparator # Comparator Control B"]
    #[inline(always)]
    pub const fn compctrlb(&self) -> &Compctrlb {
        &self.compctrlb
    }
    #[doc = "0x08 - Comparator # Comparator Scaler"]
    #[inline(always)]
    pub const fn compscaler(&self) -> &Compscaler {
        &self.compscaler
    }
}
#[doc = "COMPCTRLA (rw) register accessor: Comparator # Comparator Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`compctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compctrla`] module"]
#[doc(alias = "COMPCTRLA")]
pub type Compctrla = crate::Reg<compctrla::CompctrlaSpec>;
#[doc = "Comparator # Comparator Control A"]
pub mod compctrla;
#[doc = "COMPCTRLB (rw) register accessor: Comparator # Comparator Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`compctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compctrlb`] module"]
#[doc(alias = "COMPCTRLB")]
pub type Compctrlb = crate::Reg<compctrlb::CompctrlbSpec>;
#[doc = "Comparator # Comparator Control B"]
pub mod compctrlb;
#[doc = "COMPSCALER (rw) register accessor: Comparator # Comparator Scaler\n\nYou can [`read`](crate::Reg::read) this register and get [`compscaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compscaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compscaler`] module"]
#[doc(alias = "COMPSCALER")]
pub type Compscaler = crate::Reg<compscaler::CompscalerSpec>;
#[doc = "Comparator # Comparator Scaler"]
pub mod compscaler;

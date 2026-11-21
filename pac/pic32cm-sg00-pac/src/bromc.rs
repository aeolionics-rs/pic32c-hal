#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ucfg: Ucfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A REGISTER"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - User Configuration REGISTER"]
    #[inline(always)]
    pub const fn ucfg(&self) -> &Ucfg {
        &self.ucfg
    }
}
#[doc = "CTRLA (rw) register accessor: Control A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A REGISTER"]
pub mod ctrla;
#[doc = "UCFG (r) register accessor: User Configuration REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ucfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucfg`] module"]
#[doc(alias = "UCFG")]
pub type Ucfg = crate::Reg<ucfg::UcfgSpec>;
#[doc = "User Configuration REGISTER"]
pub mod ucfg;

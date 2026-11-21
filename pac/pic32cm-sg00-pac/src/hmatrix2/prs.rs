#[repr(C)]
#[doc = "PRS\\[%s\\]"]
#[doc(alias = "PRS")]
pub struct Prs {
    pras: Pras,
}
impl Prs {
    #[doc = "0x00 - Priority A for Slave"]
    #[inline(always)]
    pub const fn pras(&self) -> &Pras {
        &self.pras
    }
}
#[doc = "PRAS (rw) register accessor: Priority A for Slave\n\nYou can [`read`](crate::Reg::read) this register and get [`pras::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras`] module"]
#[doc(alias = "PRAS")]
pub type Pras = crate::Reg<pras::PrasSpec>;
#[doc = "Priority A for Slave"]
pub mod pras;

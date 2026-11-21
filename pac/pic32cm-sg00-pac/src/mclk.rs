#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    clkdiv: [Clkdiv; 3],
    _reserved4: [u8; 0x24],
    clkmsk: [Clkmsk; 9],
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x04 - Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x08 - Interrupt Flag Status Register"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x0c..0x18 - Clock Divider Control n Register"]
    #[inline(always)]
    pub const fn clkdiv(&self, n: usize) -> &Clkdiv {
        &self.clkdiv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x18 - Clock Divider Control n Register"]
    #[inline(always)]
    pub fn clkdiv_iter(&self) -> impl Iterator<Item = &Clkdiv> {
        self.clkdiv.iter()
    }
    #[doc = "0x3c..0x60 - Peripheral Clock Enable Mask n Register"]
    #[inline(always)]
    pub const fn clkmsk(&self, n: usize) -> &Clkmsk {
        &self.clkmsk[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0x60 - Peripheral Clock Enable Mask n Register"]
    #[inline(always)]
    pub fn clkmsk_iter(&self) -> impl Iterator<Item = &Clkmsk> {
        self.clkmsk.iter()
    }
}
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set Register"]
pub mod intenset;
#[doc = "INTFLAG (w) register accessor: Interrupt Flag Status Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status Register"]
pub mod intflag;
#[doc = "CLKDIV (rw) register accessor: Clock Divider Control n Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock Divider Control n Register"]
pub mod clkdiv;
#[doc = "CLKMSK (rw) register accessor: Peripheral Clock Enable Mask n Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkmsk`] module"]
#[doc(alias = "CLKMSK")]
pub type Clkmsk = crate::Reg<clkmsk::ClkmskSpec>;
#[doc = "Peripheral Clock Enable Mask n Register"]
pub mod clkmsk;

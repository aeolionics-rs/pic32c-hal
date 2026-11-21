#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    intflagset: Intflagset,
    status: Status,
    dbgctrl: Dbgctrl,
    eccctrl: Eccctrl,
    crcctrl: Crcctrl,
    crcmode: Crcmode,
    crcpause: Crcpause,
    crcmadr: Crcmadr,
    crcmlen: Crcmlen,
    crciv: Crciv,
    crcacc: Crcacc,
    crcpoly: Crcpoly,
    crcfxor: Crcfxor,
    crcsum: Crcsum,
    ffltctrl: Ffltctrl,
    ffltmode: Ffltmode,
    ffltptr: Ffltptr,
    ffltadr: Ffltadr,
    ffltcap: Ffltcap,
    ffltpar: Ffltpar,
    ffltsyn: Ffltsyn,
    crp: Crp,
}
impl RegisterBlock {
    #[doc = "0x00 - CTRL A REGISTER"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - CTRL B REGISTER"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x08 - Interrupt Enable Clear REGISTER"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x0c - Interrupt Enable SET REGISTER"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x10 - Interrupt Flag REGISTER"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x14 - Interrupt Flag Set REGISTER"]
    #[inline(always)]
    pub const fn intflagset(&self) -> &Intflagset {
        &self.intflagset
    }
    #[doc = "0x18 - Status REGISTER"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1c - Debug Control CTRL REGISTER"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x20 - ECC Control REGISTER"]
    #[inline(always)]
    pub const fn eccctrl(&self) -> &Eccctrl {
        &self.eccctrl
    }
    #[doc = "0x24 - CRC Control REGISTER"]
    #[inline(always)]
    pub const fn crcctrl(&self) -> &Crcctrl {
        &self.crcctrl
    }
    #[doc = "0x28 - CRC MODE REGISTER"]
    #[inline(always)]
    pub const fn crcmode(&self) -> &Crcmode {
        &self.crcmode
    }
    #[doc = "0x2c - CRC Pause REGISTER"]
    #[inline(always)]
    pub const fn crcpause(&self) -> &Crcpause {
        &self.crcpause
    }
    #[doc = "0x30 - CRC Message Address REGISTER"]
    #[inline(always)]
    pub const fn crcmadr(&self) -> &Crcmadr {
        &self.crcmadr
    }
    #[doc = "0x34 - CRC Message Length REGISTER"]
    #[inline(always)]
    pub const fn crcmlen(&self) -> &Crcmlen {
        &self.crcmlen
    }
    #[doc = "0x38 - CRC Initial Value REGISTER"]
    #[inline(always)]
    pub const fn crciv(&self) -> &Crciv {
        &self.crciv
    }
    #[doc = "0x3c - CRC Accumulator REGISTER"]
    #[inline(always)]
    pub const fn crcacc(&self) -> &Crcacc {
        &self.crcacc
    }
    #[doc = "0x40 - CRC Polynomial REGISTER"]
    #[inline(always)]
    pub const fn crcpoly(&self) -> &Crcpoly {
        &self.crcpoly
    }
    #[doc = "0x44 - CRC Final XOR REGISTER"]
    #[inline(always)]
    pub const fn crcfxor(&self) -> &Crcfxor {
        &self.crcfxor
    }
    #[doc = "0x48 - CRC CheckSUM REGISTER"]
    #[inline(always)]
    pub const fn crcsum(&self) -> &Crcsum {
        &self.crcsum
    }
    #[doc = "0x4c - Flash ECC Fault Control REGISTER"]
    #[inline(always)]
    pub const fn ffltctrl(&self) -> &Ffltctrl {
        &self.ffltctrl
    }
    #[doc = "0x50 - Flash ECC Fault Mode REGISTER"]
    #[inline(always)]
    pub const fn ffltmode(&self) -> &Ffltmode {
        &self.ffltmode
    }
    #[doc = "0x54 - Flash ECC Fault Pointer REGISTER"]
    #[inline(always)]
    pub const fn ffltptr(&self) -> &Ffltptr {
        &self.ffltptr
    }
    #[doc = "0x58 - Flash ECC Fault Address REGISTER"]
    #[inline(always)]
    pub const fn ffltadr(&self) -> &Ffltadr {
        &self.ffltadr
    }
    #[doc = "0x5c - Flash ECC Fault Capture Address REGISTER"]
    #[inline(always)]
    pub const fn ffltcap(&self) -> &Ffltcap {
        &self.ffltcap
    }
    #[doc = "0x60 - Flash ECC Fault Parity REGISTER"]
    #[inline(always)]
    pub const fn ffltpar(&self) -> &Ffltpar {
        &self.ffltpar
    }
    #[doc = "0x64 - Flash ECC Fault Syndrome REGISTER"]
    #[inline(always)]
    pub const fn ffltsyn(&self) -> &Ffltsyn {
        &self.ffltsyn
    }
    #[doc = "0x68 - CFM Page Read Protection REGISTER (CFM8)"]
    #[inline(always)]
    pub const fn crp(&self) -> &Crp {
        &self.crp
    }
}
#[doc = "CTRLA (rw) register accessor: CTRL A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "CTRL A REGISTER"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: CTRL B REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "CTRL B REGISTER"]
pub mod ctrlb;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear REGISTER"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable SET REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable SET REGISTER"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag REGISTER"]
pub mod intflag;
#[doc = "INTFLAGSET (rw) register accessor: Interrupt Flag Set REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagset`] module"]
#[doc(alias = "INTFLAGSET")]
pub type Intflagset = crate::Reg<intflagset::IntflagsetSpec>;
#[doc = "Interrupt Flag Set REGISTER"]
pub mod intflagset;
#[doc = "STATUS (r) register accessor: Status REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status REGISTER"]
pub mod status;
#[doc = "DBGCTRL (rw) register accessor: Debug Control CTRL REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control CTRL REGISTER"]
pub mod dbgctrl;
#[doc = "ECCCTRL (rw) register accessor: ECC Control REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`eccctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccctrl`] module"]
#[doc(alias = "ECCCTRL")]
pub type Eccctrl = crate::Reg<eccctrl::EccctrlSpec>;
#[doc = "ECC Control REGISTER"]
pub mod eccctrl;
#[doc = "CRCCTRL (rw) register accessor: CRC Control REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcctrl`] module"]
#[doc(alias = "CRCCTRL")]
pub type Crcctrl = crate::Reg<crcctrl::CrcctrlSpec>;
#[doc = "CRC Control REGISTER"]
pub mod crcctrl;
#[doc = "CRCMODE (rw) register accessor: CRC MODE REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcmode`] module"]
#[doc(alias = "CRCMODE")]
pub type Crcmode = crate::Reg<crcmode::CrcmodeSpec>;
#[doc = "CRC MODE REGISTER"]
pub mod crcmode;
#[doc = "CRCPAUSE (rw) register accessor: CRC Pause REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpause`] module"]
#[doc(alias = "CRCPAUSE")]
pub type Crcpause = crate::Reg<crcpause::CrcpauseSpec>;
#[doc = "CRC Pause REGISTER"]
pub mod crcpause;
#[doc = "CRCMADR (rw) register accessor: CRC Message Address REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcmadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcmadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcmadr`] module"]
#[doc(alias = "CRCMADR")]
pub type Crcmadr = crate::Reg<crcmadr::CrcmadrSpec>;
#[doc = "CRC Message Address REGISTER"]
pub mod crcmadr;
#[doc = "CRCMLEN (rw) register accessor: CRC Message Length REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcmlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcmlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcmlen`] module"]
#[doc(alias = "CRCMLEN")]
pub type Crcmlen = crate::Reg<crcmlen::CrcmlenSpec>;
#[doc = "CRC Message Length REGISTER"]
pub mod crcmlen;
#[doc = "CRCIV (rw) register accessor: CRC Initial Value REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crciv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crciv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crciv`] module"]
#[doc(alias = "CRCIV")]
pub type Crciv = crate::Reg<crciv::CrcivSpec>;
#[doc = "CRC Initial Value REGISTER"]
pub mod crciv;
#[doc = "CRCACC (r) register accessor: CRC Accumulator REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcacc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcacc`] module"]
#[doc(alias = "CRCACC")]
pub type Crcacc = crate::Reg<crcacc::CrcaccSpec>;
#[doc = "CRC Accumulator REGISTER"]
pub mod crcacc;
#[doc = "CRCPOLY (rw) register accessor: CRC Polynomial REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpoly`] module"]
#[doc(alias = "CRCPOLY")]
pub type Crcpoly = crate::Reg<crcpoly::CrcpolySpec>;
#[doc = "CRC Polynomial REGISTER"]
pub mod crcpoly;
#[doc = "CRCFXOR (rw) register accessor: CRC Final XOR REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcfxor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcfxor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcfxor`] module"]
#[doc(alias = "CRCFXOR")]
pub type Crcfxor = crate::Reg<crcfxor::CrcfxorSpec>;
#[doc = "CRC Final XOR REGISTER"]
pub mod crcfxor;
#[doc = "CRCSUM (rw) register accessor: CRC CheckSUM REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcsum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcsum`] module"]
#[doc(alias = "CRCSUM")]
pub type Crcsum = crate::Reg<crcsum::CrcsumSpec>;
#[doc = "CRC CheckSUM REGISTER"]
pub mod crcsum;
#[doc = "FFLTCTRL (rw) register accessor: Flash ECC Fault Control REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltctrl`] module"]
#[doc(alias = "FFLTCTRL")]
pub type Ffltctrl = crate::Reg<ffltctrl::FfltctrlSpec>;
#[doc = "Flash ECC Fault Control REGISTER"]
pub mod ffltctrl;
#[doc = "FFLTMODE (rw) register accessor: Flash ECC Fault Mode REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltmode`] module"]
#[doc(alias = "FFLTMODE")]
pub type Ffltmode = crate::Reg<ffltmode::FfltmodeSpec>;
#[doc = "Flash ECC Fault Mode REGISTER"]
pub mod ffltmode;
#[doc = "FFLTPTR (rw) register accessor: Flash ECC Fault Pointer REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltptr`] module"]
#[doc(alias = "FFLTPTR")]
pub type Ffltptr = crate::Reg<ffltptr::FfltptrSpec>;
#[doc = "Flash ECC Fault Pointer REGISTER"]
pub mod ffltptr;
#[doc = "FFLTADR (rw) register accessor: Flash ECC Fault Address REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltadr`] module"]
#[doc(alias = "FFLTADR")]
pub type Ffltadr = crate::Reg<ffltadr::FfltadrSpec>;
#[doc = "Flash ECC Fault Address REGISTER"]
pub mod ffltadr;
#[doc = "FFLTCAP (r) register accessor: Flash ECC Fault Capture Address REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltcap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltcap`] module"]
#[doc(alias = "FFLTCAP")]
pub type Ffltcap = crate::Reg<ffltcap::FfltcapSpec>;
#[doc = "Flash ECC Fault Capture Address REGISTER"]
pub mod ffltcap;
#[doc = "FFLTPAR (r) register accessor: Flash ECC Fault Parity REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltpar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltpar`] module"]
#[doc(alias = "FFLTPAR")]
pub type Ffltpar = crate::Reg<ffltpar::FfltparSpec>;
#[doc = "Flash ECC Fault Parity REGISTER"]
pub mod ffltpar;
#[doc = "FFLTSYN (r) register accessor: Flash ECC Fault Syndrome REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltsyn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltsyn`] module"]
#[doc(alias = "FFLTSYN")]
pub type Ffltsyn = crate::Reg<ffltsyn::FfltsynSpec>;
#[doc = "Flash ECC Fault Syndrome REGISTER"]
pub mod ffltsyn;
#[doc = "CRP (rw) register accessor: CFM Page Read Protection REGISTER (CFM8)\n\nYou can [`read`](crate::Reg::read) this register and get [`crp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crp`] module"]
#[doc(alias = "CRP")]
pub type Crp = crate::Reg<crp::CrpSpec>;
#[doc = "CFM Page Read Protection REGISTER (CFM8)"]
pub mod crp;

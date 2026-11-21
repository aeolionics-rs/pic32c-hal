#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    _reserved2: [u8; 0x20],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    intflagset: Intflagset,
    _reserved6: [u8; 0x24],
    syncbusy: Syncbusy,
    _reserved7: [u8; 0xa0],
    crel_mcan: CrelMcan,
    endn: Endn,
    cust: Cust,
    dbtp: Dbtp,
    test: Test,
    rwd: Rwd,
    cccr: Cccr,
    nbtp: Nbtp,
    tscc: Tscc,
    tscv: Tscv,
    tocc: Tocc,
    tocv: Tocv,
    _reserved19: [u8; 0x10],
    ecr: Ecr,
    psr: Psr,
    tdcr: Tdcr,
    _reserved22: [u8; 0x04],
    ir: Ir,
    ie: Ie,
    ils: Ils,
    ile: Ile,
    _reserved26: [u8; 0x20],
    gfc: Gfc,
    sidfc: Sidfc,
    xidfc: Xidfc,
    _reserved29: [u8; 0x04],
    xidam: Xidam,
    hpms: Hpms,
    ndat1: Ndat1,
    ndat2: Ndat2,
    rxf0c: Rxf0c,
    rxf0s: Rxf0s,
    rxf0a: Rxf0a,
    rxbc: Rxbc,
    rxf1c: Rxf1c,
    rxf1s: Rxf1s,
    rxf1a: Rxf1a,
    rxesc: Rxesc,
    txbc: Txbc,
    txfqs: Txfqs,
    txesc: Txesc,
    txbrp: Txbrp,
    txbar: Txbar,
    txbcr: Txbcr,
    txbto: Txbto,
    txbcf: Txbcf,
    txbtie: Txbtie,
    txbcie: Txbcie,
    _reserved51: [u8; 0x08],
    txefc: Txefc,
    txefs: Txefs,
    txefa: Txefa,
    _reserved54: [u8; 0x64],
    crel_tsu: CrelTsu,
    tscfg: Tscfg,
    tss1: Tss1,
    tss2: Tss2,
    ts0: Ts0,
    ts1: Ts1,
    ts2: Ts2,
    ts3: Ts3,
    ts4: Ts4,
    ts5: Ts5,
    ts6: Ts6,
    ts7: Ts7,
    ts8: Ts8,
    ts9: Ts9,
    ts10: Ts10,
    ts11: Ts11,
    ts12: Ts12,
    ts13: Ts13,
    ts14: Ts14,
    ts15: Ts15,
    atb: Atb,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A Register"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Control B Register"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x28 - Interrupt Enable Clear register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x2c - Interrupt Enable Set register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x30 - Interrupt Flag Status and Clear register"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x34 - Interrupt Flag Software Set Register"]
    #[inline(always)]
    pub const fn intflagset(&self) -> &Intflagset {
        &self.intflagset
    }
    #[doc = "0x5c - SYNCBUSY Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x100 - Core Release"]
    #[inline(always)]
    pub const fn crel_mcan(&self) -> &CrelMcan {
        &self.crel_mcan
    }
    #[doc = "0x104 - Endian"]
    #[inline(always)]
    pub const fn endn(&self) -> &Endn {
        &self.endn
    }
    #[doc = "0x108 - Customer Register"]
    #[inline(always)]
    pub const fn cust(&self) -> &Cust {
        &self.cust
    }
    #[doc = "0x10c - Fast Bit Timing and Prescaler"]
    #[inline(always)]
    pub const fn dbtp(&self) -> &Dbtp {
        &self.dbtp
    }
    #[doc = "0x110 - Test"]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
    #[doc = "0x114 - RAM Watchdog"]
    #[inline(always)]
    pub const fn rwd(&self) -> &Rwd {
        &self.rwd
    }
    #[doc = "0x118 - CC Control"]
    #[inline(always)]
    pub const fn cccr(&self) -> &Cccr {
        &self.cccr
    }
    #[doc = "0x11c - Nominal Bit Timing and Prescaler"]
    #[inline(always)]
    pub const fn nbtp(&self) -> &Nbtp {
        &self.nbtp
    }
    #[doc = "0x120 - Timestamp Counter Configuration"]
    #[inline(always)]
    pub const fn tscc(&self) -> &Tscc {
        &self.tscc
    }
    #[doc = "0x124 - Timestamp Counter Value"]
    #[inline(always)]
    pub const fn tscv(&self) -> &Tscv {
        &self.tscv
    }
    #[doc = "0x128 - Timeout Counter Configuration"]
    #[inline(always)]
    pub const fn tocc(&self) -> &Tocc {
        &self.tocc
    }
    #[doc = "0x12c - Timeout Counter Value"]
    #[inline(always)]
    pub const fn tocv(&self) -> &Tocv {
        &self.tocv
    }
    #[doc = "0x140 - Error Counter"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
    #[doc = "0x144 - Protocol Status"]
    #[inline(always)]
    pub const fn psr(&self) -> &Psr {
        &self.psr
    }
    #[doc = "0x148 - Extended ID Filter Configuration"]
    #[inline(always)]
    pub const fn tdcr(&self) -> &Tdcr {
        &self.tdcr
    }
    #[doc = "0x150 - Interrupt"]
    #[inline(always)]
    pub const fn ir(&self) -> &Ir {
        &self.ir
    }
    #[doc = "0x154 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x158 - Interrupt Line Select"]
    #[inline(always)]
    pub const fn ils(&self) -> &Ils {
        &self.ils
    }
    #[doc = "0x15c - Interrupt Line Enable"]
    #[inline(always)]
    pub const fn ile(&self) -> &Ile {
        &self.ile
    }
    #[doc = "0x180 - Global Filter Configuration"]
    #[inline(always)]
    pub const fn gfc(&self) -> &Gfc {
        &self.gfc
    }
    #[doc = "0x184 - Standard ID Filter Configuration"]
    #[inline(always)]
    pub const fn sidfc(&self) -> &Sidfc {
        &self.sidfc
    }
    #[doc = "0x188 - Extended ID Filter Configuration"]
    #[inline(always)]
    pub const fn xidfc(&self) -> &Xidfc {
        &self.xidfc
    }
    #[doc = "0x190 - Extended ID AND Mask"]
    #[inline(always)]
    pub const fn xidam(&self) -> &Xidam {
        &self.xidam
    }
    #[doc = "0x194 - High Priority Message Status"]
    #[inline(always)]
    pub const fn hpms(&self) -> &Hpms {
        &self.hpms
    }
    #[doc = "0x198 - New Data 1"]
    #[inline(always)]
    pub const fn ndat1(&self) -> &Ndat1 {
        &self.ndat1
    }
    #[doc = "0x19c - New Data 2"]
    #[inline(always)]
    pub const fn ndat2(&self) -> &Ndat2 {
        &self.ndat2
    }
    #[doc = "0x1a0 - Rx FIFO 0 Configuration"]
    #[inline(always)]
    pub const fn rxf0c(&self) -> &Rxf0c {
        &self.rxf0c
    }
    #[doc = "0x1a4 - Rx FIFO 0 Status"]
    #[inline(always)]
    pub const fn rxf0s(&self) -> &Rxf0s {
        &self.rxf0s
    }
    #[doc = "0x1a8 - Rx FIFO 0 Acknowledge"]
    #[inline(always)]
    pub const fn rxf0a(&self) -> &Rxf0a {
        &self.rxf0a
    }
    #[doc = "0x1ac - Rx Buffer Configuration"]
    #[inline(always)]
    pub const fn rxbc(&self) -> &Rxbc {
        &self.rxbc
    }
    #[doc = "0x1b0 - Rx FIFO 1 Configuration"]
    #[inline(always)]
    pub const fn rxf1c(&self) -> &Rxf1c {
        &self.rxf1c
    }
    #[doc = "0x1b4 - Rx FIFO 1 Status"]
    #[inline(always)]
    pub const fn rxf1s(&self) -> &Rxf1s {
        &self.rxf1s
    }
    #[doc = "0x1b8 - Rx FIFO 1 Acknowledge"]
    #[inline(always)]
    pub const fn rxf1a(&self) -> &Rxf1a {
        &self.rxf1a
    }
    #[doc = "0x1bc - Rx Buffer / FIFO Element Size Configuration"]
    #[inline(always)]
    pub const fn rxesc(&self) -> &Rxesc {
        &self.rxesc
    }
    #[doc = "0x1c0 - Tx Buffer Configuration"]
    #[inline(always)]
    pub const fn txbc(&self) -> &Txbc {
        &self.txbc
    }
    #[doc = "0x1c4 - Tx FIFO / Queue Status"]
    #[inline(always)]
    pub const fn txfqs(&self) -> &Txfqs {
        &self.txfqs
    }
    #[doc = "0x1c8 - Tx Buffer Element Size Configuration"]
    #[inline(always)]
    pub const fn txesc(&self) -> &Txesc {
        &self.txesc
    }
    #[doc = "0x1cc - Tx Buffer Request Pending"]
    #[inline(always)]
    pub const fn txbrp(&self) -> &Txbrp {
        &self.txbrp
    }
    #[doc = "0x1d0 - Tx Buffer Add Request"]
    #[inline(always)]
    pub const fn txbar(&self) -> &Txbar {
        &self.txbar
    }
    #[doc = "0x1d4 - Tx Buffer Cancellation Request"]
    #[inline(always)]
    pub const fn txbcr(&self) -> &Txbcr {
        &self.txbcr
    }
    #[doc = "0x1d8 - Tx Buffer Transmission Occurred"]
    #[inline(always)]
    pub const fn txbto(&self) -> &Txbto {
        &self.txbto
    }
    #[doc = "0x1dc - Tx Buffer Cancellation Finished"]
    #[inline(always)]
    pub const fn txbcf(&self) -> &Txbcf {
        &self.txbcf
    }
    #[doc = "0x1e0 - Tx Buffer Transmission Interrupt Enable"]
    #[inline(always)]
    pub const fn txbtie(&self) -> &Txbtie {
        &self.txbtie
    }
    #[doc = "0x1e4 - Tx Buffer Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn txbcie(&self) -> &Txbcie {
        &self.txbcie
    }
    #[doc = "0x1f0 - Tx Event FIFO Configuration"]
    #[inline(always)]
    pub const fn txefc(&self) -> &Txefc {
        &self.txefc
    }
    #[doc = "0x1f4 - Tx Event FIFO Status"]
    #[inline(always)]
    pub const fn txefs(&self) -> &Txefs {
        &self.txefs
    }
    #[doc = "0x1f8 - Tx Event FIFO Acknowledge"]
    #[inline(always)]
    pub const fn txefa(&self) -> &Txefa {
        &self.txefa
    }
    #[doc = "0x260 - TSU CREL"]
    #[inline(always)]
    pub const fn crel_tsu(&self) -> &CrelTsu {
        &self.crel_tsu
    }
    #[doc = "0x264 - Timestamp Configuration"]
    #[inline(always)]
    pub const fn tscfg(&self) -> &Tscfg {
        &self.tscfg
    }
    #[doc = "0x268 - Timestamp Status 1"]
    #[inline(always)]
    pub const fn tss1(&self) -> &Tss1 {
        &self.tss1
    }
    #[doc = "0x26c - Timestamp Status 2"]
    #[inline(always)]
    pub const fn tss2(&self) -> &Tss2 {
        &self.tss2
    }
    #[doc = "0x270 - Timestamp 0"]
    #[inline(always)]
    pub const fn ts0(&self) -> &Ts0 {
        &self.ts0
    }
    #[doc = "0x274 - Timestamp 1"]
    #[inline(always)]
    pub const fn ts1(&self) -> &Ts1 {
        &self.ts1
    }
    #[doc = "0x278 - Timestamp 2"]
    #[inline(always)]
    pub const fn ts2(&self) -> &Ts2 {
        &self.ts2
    }
    #[doc = "0x27c - Timestamp 3"]
    #[inline(always)]
    pub const fn ts3(&self) -> &Ts3 {
        &self.ts3
    }
    #[doc = "0x280 - Timestamp 4"]
    #[inline(always)]
    pub const fn ts4(&self) -> &Ts4 {
        &self.ts4
    }
    #[doc = "0x284 - Timestamp 5"]
    #[inline(always)]
    pub const fn ts5(&self) -> &Ts5 {
        &self.ts5
    }
    #[doc = "0x288 - Timestamp 6"]
    #[inline(always)]
    pub const fn ts6(&self) -> &Ts6 {
        &self.ts6
    }
    #[doc = "0x28c - Timestamp 7"]
    #[inline(always)]
    pub const fn ts7(&self) -> &Ts7 {
        &self.ts7
    }
    #[doc = "0x290 - Timestamp 8"]
    #[inline(always)]
    pub const fn ts8(&self) -> &Ts8 {
        &self.ts8
    }
    #[doc = "0x294 - Timestamp 9"]
    #[inline(always)]
    pub const fn ts9(&self) -> &Ts9 {
        &self.ts9
    }
    #[doc = "0x298 - Timestamp 10"]
    #[inline(always)]
    pub const fn ts10(&self) -> &Ts10 {
        &self.ts10
    }
    #[doc = "0x29c - Timestamp 11"]
    #[inline(always)]
    pub const fn ts11(&self) -> &Ts11 {
        &self.ts11
    }
    #[doc = "0x2a0 - Timestamp 12"]
    #[inline(always)]
    pub const fn ts12(&self) -> &Ts12 {
        &self.ts12
    }
    #[doc = "0x2a4 - Timestamp 13"]
    #[inline(always)]
    pub const fn ts13(&self) -> &Ts13 {
        &self.ts13
    }
    #[doc = "0x2a8 - Timestamp 14"]
    #[inline(always)]
    pub const fn ts14(&self) -> &Ts14 {
        &self.ts14
    }
    #[doc = "0x2ac - Timestamp 15"]
    #[inline(always)]
    pub const fn ts15(&self) -> &Ts15 {
        &self.ts15
    }
    #[doc = "0x2b0 - Actual Timebase"]
    #[inline(always)]
    pub const fn atb(&self) -> &Atb {
        &self.atb
    }
}
#[doc = "CTRLA (rw) register accessor: Control A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A Register"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: Control B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "Control B Register"]
pub mod ctrlb;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear register"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set register"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status and Clear register"]
pub mod intflag;
#[doc = "INTFLAGSET (rw) register accessor: Interrupt Flag Software Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagset`] module"]
#[doc(alias = "INTFLAGSET")]
pub type Intflagset = crate::Reg<intflagset::IntflagsetSpec>;
#[doc = "Interrupt Flag Software Set Register"]
pub mod intflagset;
#[doc = "SYNCBUSY (r) register accessor: SYNCBUSY Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "SYNCBUSY Register"]
pub mod syncbusy;
#[doc = "CREL_MCAN (r) register accessor: Core Release\n\nYou can [`read`](crate::Reg::read) this register and get [`crel_mcan::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel_mcan`] module"]
#[doc(alias = "CREL_MCAN")]
pub type CrelMcan = crate::Reg<crel_mcan::CrelMcanSpec>;
#[doc = "Core Release"]
pub mod crel_mcan;
#[doc = "ENDN (r) register accessor: Endian\n\nYou can [`read`](crate::Reg::read) this register and get [`endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endn`] module"]
#[doc(alias = "ENDN")]
pub type Endn = crate::Reg<endn::EndnSpec>;
#[doc = "Endian"]
pub mod endn;
#[doc = "CUST (rw) register accessor: Customer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cust::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cust::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cust`] module"]
#[doc(alias = "CUST")]
pub type Cust = crate::Reg<cust::CustSpec>;
#[doc = "Customer Register"]
pub mod cust;
#[doc = "DBTP (rw) register accessor: Fast Bit Timing and Prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`dbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbtp`] module"]
#[doc(alias = "DBTP")]
pub type Dbtp = crate::Reg<dbtp::DbtpSpec>;
#[doc = "Fast Bit Timing and Prescaler"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: Test\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`] module"]
#[doc(alias = "TEST")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "Test"]
pub mod test;
#[doc = "RWD (rw) register accessor: RAM Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`rwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwd`] module"]
#[doc(alias = "RWD")]
pub type Rwd = crate::Reg<rwd::RwdSpec>;
#[doc = "RAM Watchdog"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: CC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`] module"]
#[doc(alias = "CCCR")]
pub type Cccr = crate::Reg<cccr::CccrSpec>;
#[doc = "CC Control"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: Nominal Bit Timing and Prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtp`] module"]
#[doc(alias = "NBTP")]
pub type Nbtp = crate::Reg<nbtp::NbtpSpec>;
#[doc = "Nominal Bit Timing and Prescaler"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: Timestamp Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscc`] module"]
#[doc(alias = "TSCC")]
pub type Tscc = crate::Reg<tscc::TsccSpec>;
#[doc = "Timestamp Counter Configuration"]
pub mod tscc;
#[doc = "TSCV (r) register accessor: Timestamp Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tscv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscv`] module"]
#[doc(alias = "TSCV")]
pub type Tscv = crate::Reg<tscv::TscvSpec>;
#[doc = "Timestamp Counter Value"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: Timeout Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocc`] module"]
#[doc(alias = "TOCC")]
pub type Tocc = crate::Reg<tocc::ToccSpec>;
#[doc = "Timeout Counter Configuration"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: Timeout Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocv`] module"]
#[doc(alias = "TOCV")]
pub type Tocv = crate::Reg<tocv::TocvSpec>;
#[doc = "Timeout Counter Value"]
pub mod tocv;
#[doc = "ECR (r) register accessor: Error Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`] module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "PSR (r) register accessor: Protocol Status\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`] module"]
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
#[doc = "Protocol Status"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: Extended ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcr`] module"]
#[doc(alias = "TDCR")]
pub type Tdcr = crate::Reg<tdcr::TdcrSpec>;
#[doc = "Extended ID Filter Configuration"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`] module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "Interrupt"]
pub mod ir;
#[doc = "IE (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`] module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "Interrupt Enable"]
pub mod ie;
#[doc = "ILS (rw) register accessor: Interrupt Line Select\n\nYou can [`read`](crate::Reg::read) this register and get [`ils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ils`] module"]
#[doc(alias = "ILS")]
pub type Ils = crate::Reg<ils::IlsSpec>;
#[doc = "Interrupt Line Select"]
pub mod ils;
#[doc = "ILE (rw) register accessor: Interrupt Line Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ile`] module"]
#[doc(alias = "ILE")]
pub type Ile = crate::Reg<ile::IleSpec>;
#[doc = "Interrupt Line Enable"]
pub mod ile;
#[doc = "GFC (rw) register accessor: Global Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfc`] module"]
#[doc(alias = "GFC")]
pub type Gfc = crate::Reg<gfc::GfcSpec>;
#[doc = "Global Filter Configuration"]
pub mod gfc;
#[doc = "SIDFC (rw) register accessor: Standard ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidfc`] module"]
#[doc(alias = "SIDFC")]
pub type Sidfc = crate::Reg<sidfc::SidfcSpec>;
#[doc = "Standard ID Filter Configuration"]
pub mod sidfc;
#[doc = "XIDFC (rw) register accessor: Extended ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`xidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidfc`] module"]
#[doc(alias = "XIDFC")]
pub type Xidfc = crate::Reg<xidfc::XidfcSpec>;
#[doc = "Extended ID Filter Configuration"]
pub mod xidfc;
#[doc = "XIDAM (rw) register accessor: Extended ID AND Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidam`] module"]
#[doc(alias = "XIDAM")]
pub type Xidam = crate::Reg<xidam::XidamSpec>;
#[doc = "Extended ID AND Mask"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: High Priority Message Status\n\nYou can [`read`](crate::Reg::read) this register and get [`hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpms`] module"]
#[doc(alias = "HPMS")]
pub type Hpms = crate::Reg<hpms::HpmsSpec>;
#[doc = "High Priority Message Status"]
pub mod hpms;
#[doc = "NDAT1 (rw) register accessor: New Data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ndat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat1`] module"]
#[doc(alias = "NDAT1")]
pub type Ndat1 = crate::Reg<ndat1::Ndat1Spec>;
#[doc = "New Data 1"]
pub mod ndat1;
#[doc = "NDAT2 (rw) register accessor: New Data 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ndat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat2`] module"]
#[doc(alias = "NDAT2")]
pub type Ndat2 = crate::Reg<ndat2::Ndat2Spec>;
#[doc = "New Data 2"]
pub mod ndat2;
#[doc = "RXF0C (rw) register accessor: Rx FIFO 0 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0c`] module"]
#[doc(alias = "RXF0C")]
pub type Rxf0c = crate::Reg<rxf0c::Rxf0cSpec>;
#[doc = "Rx FIFO 0 Configuration"]
pub mod rxf0c;
#[doc = "RXF0S (r) register accessor: Rx FIFO 0 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0s`] module"]
#[doc(alias = "RXF0S")]
pub type Rxf0s = crate::Reg<rxf0s::Rxf0sSpec>;
#[doc = "Rx FIFO 0 Status"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: Rx FIFO 0 Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0a`] module"]
#[doc(alias = "RXF0A")]
pub type Rxf0a = crate::Reg<rxf0a::Rxf0aSpec>;
#[doc = "Rx FIFO 0 Acknowledge"]
pub mod rxf0a;
#[doc = "RXBC (rw) register accessor: Rx Buffer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbc`] module"]
#[doc(alias = "RXBC")]
pub type Rxbc = crate::Reg<rxbc::RxbcSpec>;
#[doc = "Rx Buffer Configuration"]
pub mod rxbc;
#[doc = "RXF1C (rw) register accessor: Rx FIFO 1 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1c`] module"]
#[doc(alias = "RXF1C")]
pub type Rxf1c = crate::Reg<rxf1c::Rxf1cSpec>;
#[doc = "Rx FIFO 1 Configuration"]
pub mod rxf1c;
#[doc = "RXF1S (r) register accessor: Rx FIFO 1 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1s`] module"]
#[doc(alias = "RXF1S")]
pub type Rxf1s = crate::Reg<rxf1s::Rxf1sSpec>;
#[doc = "Rx FIFO 1 Status"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: Rx FIFO 1 Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1a`] module"]
#[doc(alias = "RXF1A")]
pub type Rxf1a = crate::Reg<rxf1a::Rxf1aSpec>;
#[doc = "Rx FIFO 1 Acknowledge"]
pub mod rxf1a;
#[doc = "RXESC (rw) register accessor: Rx Buffer / FIFO Element Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rxesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxesc`] module"]
#[doc(alias = "RXESC")]
pub type Rxesc = crate::Reg<rxesc::RxescSpec>;
#[doc = "Rx Buffer / FIFO Element Size Configuration"]
pub mod rxesc;
#[doc = "TXBC (rw) register accessor: Tx Buffer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbc`] module"]
#[doc(alias = "TXBC")]
pub type Txbc = crate::Reg<txbc::TxbcSpec>;
#[doc = "Tx Buffer Configuration"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: Tx FIFO / Queue Status\n\nYou can [`read`](crate::Reg::read) this register and get [`txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfqs`] module"]
#[doc(alias = "TXFQS")]
pub type Txfqs = crate::Reg<txfqs::TxfqsSpec>;
#[doc = "Tx FIFO / Queue Status"]
pub mod txfqs;
#[doc = "TXESC (rw) register accessor: Tx Buffer Element Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txesc`] module"]
#[doc(alias = "TXESC")]
pub type Txesc = crate::Reg<txesc::TxescSpec>;
#[doc = "Tx Buffer Element Size Configuration"]
pub mod txesc;
#[doc = "TXBRP (r) register accessor: Tx Buffer Request Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`txbrp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbrp`] module"]
#[doc(alias = "TXBRP")]
pub type Txbrp = crate::Reg<txbrp::TxbrpSpec>;
#[doc = "Tx Buffer Request Pending"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: Tx Buffer Add Request\n\nYou can [`read`](crate::Reg::read) this register and get [`txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbar`] module"]
#[doc(alias = "TXBAR")]
pub type Txbar = crate::Reg<txbar::TxbarSpec>;
#[doc = "Tx Buffer Add Request"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: Tx Buffer Cancellation Request\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcr`] module"]
#[doc(alias = "TXBCR")]
pub type Txbcr = crate::Reg<txbcr::TxbcrSpec>;
#[doc = "Tx Buffer Cancellation Request"]
pub mod txbcr;
#[doc = "TXBTO (r) register accessor: Tx Buffer Transmission Occurred\n\nYou can [`read`](crate::Reg::read) this register and get [`txbto::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbto`] module"]
#[doc(alias = "TXBTO")]
pub type Txbto = crate::Reg<txbto::TxbtoSpec>;
#[doc = "Tx Buffer Transmission Occurred"]
pub mod txbto;
#[doc = "TXBCF (r) register accessor: Tx Buffer Cancellation Finished\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcf`] module"]
#[doc(alias = "TXBCF")]
pub type Txbcf = crate::Reg<txbcf::TxbcfSpec>;
#[doc = "Tx Buffer Cancellation Finished"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: Tx Buffer Transmission Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbtie`] module"]
#[doc(alias = "TXBTIE")]
pub type Txbtie = crate::Reg<txbtie::TxbtieSpec>;
#[doc = "Tx Buffer Transmission Interrupt Enable"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: Tx Buffer Cancellation Finished Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcie`] module"]
#[doc(alias = "TXBCIE")]
pub type Txbcie = crate::Reg<txbcie::TxbcieSpec>;
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
pub mod txbcie;
#[doc = "TXEFC (rw) register accessor: Tx Event FIFO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txefc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefc`] module"]
#[doc(alias = "TXEFC")]
pub type Txefc = crate::Reg<txefc::TxefcSpec>;
#[doc = "Tx Event FIFO Configuration"]
pub mod txefc;
#[doc = "TXEFS (r) register accessor: Tx Event FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`txefs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefs`] module"]
#[doc(alias = "TXEFS")]
pub type Txefs = crate::Reg<txefs::TxefsSpec>;
#[doc = "Tx Event FIFO Status"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: Tx Event FIFO Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefa`] module"]
#[doc(alias = "TXEFA")]
pub type Txefa = crate::Reg<txefa::TxefaSpec>;
#[doc = "Tx Event FIFO Acknowledge"]
pub mod txefa;
#[doc = "CREL_TSU (r) register accessor: TSU CREL\n\nYou can [`read`](crate::Reg::read) this register and get [`crel_tsu::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel_tsu`] module"]
#[doc(alias = "CREL_TSU")]
pub type CrelTsu = crate::Reg<crel_tsu::CrelTsuSpec>;
#[doc = "TSU CREL"]
pub mod crel_tsu;
#[doc = "TSCFG (rw) register accessor: Timestamp Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tscfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscfg`] module"]
#[doc(alias = "TSCFG")]
pub type Tscfg = crate::Reg<tscfg::TscfgSpec>;
#[doc = "Timestamp Configuration"]
pub mod tscfg;
#[doc = "TSS1 (r) register accessor: Timestamp Status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tss1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tss1`] module"]
#[doc(alias = "TSS1")]
pub type Tss1 = crate::Reg<tss1::Tss1Spec>;
#[doc = "Timestamp Status 1"]
pub mod tss1;
#[doc = "TSS2 (r) register accessor: Timestamp Status 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tss2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tss2`] module"]
#[doc(alias = "TSS2")]
pub type Tss2 = crate::Reg<tss2::Tss2Spec>;
#[doc = "Timestamp Status 2"]
pub mod tss2;
#[doc = "TS0 (r) register accessor: Timestamp 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts0`] module"]
#[doc(alias = "TS0")]
pub type Ts0 = crate::Reg<ts0::Ts0Spec>;
#[doc = "Timestamp 0"]
pub mod ts0;
#[doc = "TS1 (r) register accessor: Timestamp 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts1`] module"]
#[doc(alias = "TS1")]
pub type Ts1 = crate::Reg<ts1::Ts1Spec>;
#[doc = "Timestamp 1"]
pub mod ts1;
#[doc = "TS2 (r) register accessor: Timestamp 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts2`] module"]
#[doc(alias = "TS2")]
pub type Ts2 = crate::Reg<ts2::Ts2Spec>;
#[doc = "Timestamp 2"]
pub mod ts2;
#[doc = "TS3 (r) register accessor: Timestamp 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ts3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts3`] module"]
#[doc(alias = "TS3")]
pub type Ts3 = crate::Reg<ts3::Ts3Spec>;
#[doc = "Timestamp 3"]
pub mod ts3;
#[doc = "TS4 (r) register accessor: Timestamp 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ts4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts4`] module"]
#[doc(alias = "TS4")]
pub type Ts4 = crate::Reg<ts4::Ts4Spec>;
#[doc = "Timestamp 4"]
pub mod ts4;
#[doc = "TS5 (r) register accessor: Timestamp 5\n\nYou can [`read`](crate::Reg::read) this register and get [`ts5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts5`] module"]
#[doc(alias = "TS5")]
pub type Ts5 = crate::Reg<ts5::Ts5Spec>;
#[doc = "Timestamp 5"]
pub mod ts5;
#[doc = "TS6 (r) register accessor: Timestamp 6\n\nYou can [`read`](crate::Reg::read) this register and get [`ts6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts6`] module"]
#[doc(alias = "TS6")]
pub type Ts6 = crate::Reg<ts6::Ts6Spec>;
#[doc = "Timestamp 6"]
pub mod ts6;
#[doc = "TS7 (r) register accessor: Timestamp 7\n\nYou can [`read`](crate::Reg::read) this register and get [`ts7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts7`] module"]
#[doc(alias = "TS7")]
pub type Ts7 = crate::Reg<ts7::Ts7Spec>;
#[doc = "Timestamp 7"]
pub mod ts7;
#[doc = "TS8 (r) register accessor: Timestamp 8\n\nYou can [`read`](crate::Reg::read) this register and get [`ts8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts8`] module"]
#[doc(alias = "TS8")]
pub type Ts8 = crate::Reg<ts8::Ts8Spec>;
#[doc = "Timestamp 8"]
pub mod ts8;
#[doc = "TS9 (r) register accessor: Timestamp 9\n\nYou can [`read`](crate::Reg::read) this register and get [`ts9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts9`] module"]
#[doc(alias = "TS9")]
pub type Ts9 = crate::Reg<ts9::Ts9Spec>;
#[doc = "Timestamp 9"]
pub mod ts9;
#[doc = "TS10 (r) register accessor: Timestamp 10\n\nYou can [`read`](crate::Reg::read) this register and get [`ts10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts10`] module"]
#[doc(alias = "TS10")]
pub type Ts10 = crate::Reg<ts10::Ts10Spec>;
#[doc = "Timestamp 10"]
pub mod ts10;
#[doc = "TS11 (r) register accessor: Timestamp 11\n\nYou can [`read`](crate::Reg::read) this register and get [`ts11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts11`] module"]
#[doc(alias = "TS11")]
pub type Ts11 = crate::Reg<ts11::Ts11Spec>;
#[doc = "Timestamp 11"]
pub mod ts11;
#[doc = "TS12 (r) register accessor: Timestamp 12\n\nYou can [`read`](crate::Reg::read) this register and get [`ts12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts12`] module"]
#[doc(alias = "TS12")]
pub type Ts12 = crate::Reg<ts12::Ts12Spec>;
#[doc = "Timestamp 12"]
pub mod ts12;
#[doc = "TS13 (r) register accessor: Timestamp 13\n\nYou can [`read`](crate::Reg::read) this register and get [`ts13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts13`] module"]
#[doc(alias = "TS13")]
pub type Ts13 = crate::Reg<ts13::Ts13Spec>;
#[doc = "Timestamp 13"]
pub mod ts13;
#[doc = "TS14 (r) register accessor: Timestamp 14\n\nYou can [`read`](crate::Reg::read) this register and get [`ts14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts14`] module"]
#[doc(alias = "TS14")]
pub type Ts14 = crate::Reg<ts14::Ts14Spec>;
#[doc = "Timestamp 14"]
pub mod ts14;
#[doc = "TS15 (r) register accessor: Timestamp 15\n\nYou can [`read`](crate::Reg::read) this register and get [`ts15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts15`] module"]
#[doc(alias = "TS15")]
pub type Ts15 = crate::Reg<ts15::Ts15Spec>;
#[doc = "Timestamp 15"]
pub mod ts15;
#[doc = "ATB (rw) register accessor: Actual Timebase\n\nYou can [`read`](crate::Reg::read) this register and get [`atb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atb`] module"]
#[doc(alias = "ATB")]
pub type Atb = crate::Reg<atb::AtbSpec>;
#[doc = "Actual Timebase"]
pub mod atb;

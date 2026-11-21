#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0x34],
    statusa: Statusa,
    _reserved2: [u8; 0xc4],
    disable: Disable,
    restrict0: Restrict0,
    restrict1: Restrict1,
    ctrl_pwr: CtrlPwr,
    ctrl_key: CtrlKey,
    _reserved7: [u8; 0xec],
    qk_cr: QkCr,
    qk_orr: QkOrr,
    qk_sr: QkSr,
    qk_ar: QkAr,
    qk_ier: QkIer,
    qk_imr: QkImr,
    qk_isr: QkIsr,
    _reserved14: [u8; 0x04],
    qk_data_dest: QkDataDest,
    qk_data_src: QkDataSrc,
    qk_key_index: QkKeyIndex,
    _reserved17: [u8; 0x74],
    qk_dir: QkDir,
    _reserved18: [u8; 0x04],
    qk_dor: QkDor,
    _reserved19: [u8; 0x14],
    qk_misc: QkMisc,
    _reserved20: [u8; 0x0c],
    qk_if_sr: QkIfSr,
    _reserved21: [u8; 0x04],
    qk_test: QkTest,
    qk_psr: QkPsr,
    qk_hw_ruc0: QkHwRuc0,
    qk_hw_ruc1: QkHwRuc1,
    _reserved25: [u8; 0x08],
    qk_hw_settings: QkHwSettings,
    qk_hw_info: QkHwInfo,
    qk_hw_id: QkHwId,
    qk_hw_ver: QkHwVer,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x38 - Status Register A"]
    #[inline(always)]
    pub const fn statusa(&self) -> &Statusa {
        &self.statusa
    }
    #[doc = "0x100 - Disable Function Control Register"]
    #[inline(always)]
    pub const fn disable(&self) -> &Disable {
        &self.disable
    }
    #[doc = "0x104 - Restrict User Context 0 Control Register"]
    #[inline(always)]
    pub const fn restrict0(&self) -> &Restrict0 {
        &self.restrict0
    }
    #[doc = "0x108 - Restrict User Context 1 Control Register"]
    #[inline(always)]
    pub const fn restrict1(&self) -> &Restrict1 {
        &self.restrict1
    }
    #[doc = "0x10c - Power Control Register"]
    #[inline(always)]
    pub const fn ctrl_pwr(&self) -> &CtrlPwr {
        &self.ctrl_pwr
    }
    #[doc = "0x110 - KEY Control Register"]
    #[inline(always)]
    pub const fn ctrl_key(&self) -> &CtrlKey {
        &self.ctrl_key
    }
    #[doc = "0x200 - Control register; The Control register defines which command must be executed next. When the command is accepted or rejected, the bits automatically revert to 0. Only one command bit may be written with 1 at a time, with the exception of qk_zeroize. Writing qk_zeroize with 1 takes precedence over all other commands."]
    #[inline(always)]
    pub const fn qk_cr(&self) -> &QkCr {
        &self.qk_cr
    }
    #[doc = "0x204 - Operation Result register; The Operation Result register provides the result code of the last operation (Initialization, Generate Random, etc.). The value in this register is only valid when qk_busy = 0."]
    #[inline(always)]
    pub const fn qk_orr(&self) -> &QkOrr {
        &self.qk_orr
    }
    #[doc = "0x208 - Status register; The Status register shows the current status of QuiddiKey and indicates whether a data transfer is requested."]
    #[inline(always)]
    pub const fn qk_sr(&self) -> &QkSr {
        &self.qk_sr
    }
    #[doc = "0x20c - Allow register; The Allow register indicates which operations are currently allowed."]
    #[inline(always)]
    pub const fn qk_ar(&self) -> &QkAr {
        &self.qk_ar
    }
    #[doc = "0x210 - Interrupt Enable register; With the Interrupt Enable register all QuiddiKey interrupts can be enabled or disabled, without changing the Interrupt Mask register."]
    #[inline(always)]
    pub const fn qk_ier(&self) -> &QkIer {
        &self.qk_ier
    }
    #[doc = "0x214 - Interrupt Mask register; Individual QuiddiKey interrupts can be enabled or disabled with the Interrupt Mask register."]
    #[inline(always)]
    pub const fn qk_imr(&self) -> &QkImr {
        &self.qk_imr
    }
    #[doc = "0x218 - Interrupt Status register; The Interrupt Status register indicates which interrupt has occurred. Multiple interrupts may occur at the same time (e.g. the busy interrupt occurs together with one of the ok and error interrupts). Writing 1 to a bit, clears the corresponding bit."]
    #[inline(always)]
    pub const fn qk_isr(&self) -> &QkIsr {
        &self.qk_isr
    }
    #[doc = "0x220 - Data Destination register; The Data Destination register defines via which interface data is made available by QuiddiKey. Only one bit at a time may be 1."]
    #[inline(always)]
    pub const fn qk_data_dest(&self) -> &QkDataDest {
        &self.qk_data_dest
    }
    #[doc = "0x224 - Data Source register; The Data Source register defines via which interface data must be provided to QuiddiKey. Combinations of bits may be 1 at a time."]
    #[inline(always)]
    pub const fn qk_data_src(&self) -> &QkDataSrc {
        &self.qk_data_src
    }
    #[doc = "0x228 - Key Index register; The Key Index register provides the index of the key that is currently output."]
    #[inline(always)]
    pub const fn qk_key_index(&self) -> &QkKeyIndex {
        &self.qk_key_index
    }
    #[doc = "0x2a0 - Data Input register; The Data Input register is used to transfer data to QuiddiKey. It can only be written when input data is requested (indicated by the qk_di_request signal). When written at any other moment, or when read, an error response is generated. Depending on the way the system is configured and connected, this may generate an exception."]
    #[inline(always)]
    pub const fn qk_dir(&self) -> &QkDir {
        &self.qk_dir
    }
    #[doc = "0x2a8 - Data Output register; The Data Output register is used to transfer data from QuiddiKey. It can only be read when input data is available (indicated by the qk_do_request signal). When read at any other moment, or when written, an error response is generated. Depending on the way the system is configured and connected, this may generate an exception."]
    #[inline(always)]
    pub const fn qk_dor(&self) -> &QkDor {
        &self.qk_dor
    }
    #[doc = "0x2c0 - Miscellaneous register; The Miscellaneous register defines miscellaneous settings for QuiddiKey. These usually need to be configured only once after power up orreset."]
    #[inline(always)]
    pub const fn qk_misc(&self) -> &QkMisc {
        &self.qk_misc
    }
    #[doc = "0x2d0 - Interface Status register; The status of the interface can be monitored with the Interface Status register. Writing 1 to a bit, clears the corresponding bit."]
    #[inline(always)]
    pub const fn qk_if_sr(&self) -> &QkIfSr {
        &self.qk_if_sr
    }
    #[doc = "0x2d8 - Test register; With the Test register a BIST can be started. It also shows the result of the last BIST."]
    #[inline(always)]
    pub const fn qk_test(&self) -> &QkTest {
        &self.qk_test
    }
    #[doc = "0x2dc - PUF Score register; The PUF Score register shows the value of the PUF Score that was obtained during the last Test PUF, Enroll, Start or Reconstruct operation."]
    #[inline(always)]
    pub const fn qk_psr(&self) -> &QkPsr {
        &self.qk_psr
    }
    #[doc = "0x2e0 - Module Restrict User Context 0 register; The Module Restrict User Context 0 register shows the values of the qk_restrict_user_context_0 signal on the signal interface."]
    #[inline(always)]
    pub const fn qk_hw_ruc0(&self) -> &QkHwRuc0 {
        &self.qk_hw_ruc0
    }
    #[doc = "0x2e4 - Module Restrict User Context 1 register; The Module Restrict User Context 1 register shows the values of the qk_restrict_user_context_1 signal on the signal interface."]
    #[inline(always)]
    pub const fn qk_hw_ruc1(&self) -> &QkHwRuc1 {
        &self.qk_hw_ruc1
    }
    #[doc = "0x2f0 - Module Settings register; The Module Settings register shows the values of the module settings on the signal interface."]
    #[inline(always)]
    pub const fn qk_hw_settings(&self) -> &QkHwSettings {
        &self.qk_hw_settings
    }
    #[doc = "0x2f4 - Module Information register; The Module Information register indicates which configuration of QuiddiKey is used for this module and which options it includes."]
    #[inline(always)]
    pub const fn qk_hw_info(&self) -> &QkHwInfo {
        &self.qk_hw_info
    }
    #[doc = "0x2f8 - Module Identifier register; The Module Identifier register provides the identifier of the QuiddiKey Engine netlist."]
    #[inline(always)]
    pub const fn qk_hw_id(&self) -> &QkHwId {
        &self.qk_hw_id
    }
    #[doc = "0x2fc - Module Version register; The Module Version register provides the version of this module: major.minor.patch."]
    #[inline(always)]
    pub const fn qk_hw_ver(&self) -> &QkHwVer {
        &self.qk_hw_ver
    }
}
#[doc = "CTRLA (rw) register accessor: Control Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control Register A"]
pub mod ctrla;
#[doc = "STATUSA (r) register accessor: Status Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusa`] module"]
#[doc(alias = "STATUSA")]
pub type Statusa = crate::Reg<statusa::StatusaSpec>;
#[doc = "Status Register A"]
pub mod statusa;
#[doc = "DISABLE (rw) register accessor: Disable Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable`] module"]
#[doc(alias = "DISABLE")]
pub type Disable = crate::Reg<disable::DisableSpec>;
#[doc = "Disable Function Control Register"]
pub mod disable;
#[doc = "RESTRICT0 (rw) register accessor: Restrict User Context 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`restrict0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`restrict0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@restrict0`] module"]
#[doc(alias = "RESTRICT0")]
pub type Restrict0 = crate::Reg<restrict0::Restrict0Spec>;
#[doc = "Restrict User Context 0 Control Register"]
pub mod restrict0;
#[doc = "RESTRICT1 (rw) register accessor: Restrict User Context 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`restrict1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`restrict1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@restrict1`] module"]
#[doc(alias = "RESTRICT1")]
pub type Restrict1 = crate::Reg<restrict1::Restrict1Spec>;
#[doc = "Restrict User Context 1 Control Register"]
pub mod restrict1;
#[doc = "CTRL_PWR (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_pwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_pwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_pwr`] module"]
#[doc(alias = "CTRL_PWR")]
pub type CtrlPwr = crate::Reg<ctrl_pwr::CtrlPwrSpec>;
#[doc = "Power Control Register"]
pub mod ctrl_pwr;
#[doc = "CTRL_KEY (rw) register accessor: KEY Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_key`] module"]
#[doc(alias = "CTRL_KEY")]
pub type CtrlKey = crate::Reg<ctrl_key::CtrlKeySpec>;
#[doc = "KEY Control Register"]
pub mod ctrl_key;
#[doc = "QK_CR (rw) register accessor: Control register; The Control register defines which command must be executed next. When the command is accepted or rejected, the bits automatically revert to 0. Only one command bit may be written with 1 at a time, with the exception of qk_zeroize. Writing qk_zeroize with 1 takes precedence over all other commands.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_cr`] module"]
#[doc(alias = "QK_CR")]
pub type QkCr = crate::Reg<qk_cr::QkCrSpec>;
#[doc = "Control register; The Control register defines which command must be executed next. When the command is accepted or rejected, the bits automatically revert to 0. Only one command bit may be written with 1 at a time, with the exception of qk_zeroize. Writing qk_zeroize with 1 takes precedence over all other commands."]
pub mod qk_cr;
#[doc = "QK_ORR (r) register accessor: Operation Result register; The Operation Result register provides the result code of the last operation (Initialization, Generate Random, etc.). The value in this register is only valid when qk_busy = 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_orr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_orr`] module"]
#[doc(alias = "QK_ORR")]
pub type QkOrr = crate::Reg<qk_orr::QkOrrSpec>;
#[doc = "Operation Result register; The Operation Result register provides the result code of the last operation (Initialization, Generate Random, etc.). The value in this register is only valid when qk_busy = 0."]
pub mod qk_orr;
#[doc = "QK_SR (rw) register accessor: Status register; The Status register shows the current status of QuiddiKey and indicates whether a data transfer is requested.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_sr`] module"]
#[doc(alias = "QK_SR")]
pub type QkSr = crate::Reg<qk_sr::QkSrSpec>;
#[doc = "Status register; The Status register shows the current status of QuiddiKey and indicates whether a data transfer is requested."]
pub mod qk_sr;
#[doc = "QK_AR (r) register accessor: Allow register; The Allow register indicates which operations are currently allowed.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_ar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_ar`] module"]
#[doc(alias = "QK_AR")]
pub type QkAr = crate::Reg<qk_ar::QkArSpec>;
#[doc = "Allow register; The Allow register indicates which operations are currently allowed."]
pub mod qk_ar;
#[doc = "QK_IER (rw) register accessor: Interrupt Enable register; With the Interrupt Enable register all QuiddiKey interrupts can be enabled or disabled, without changing the Interrupt Mask register.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_ier`] module"]
#[doc(alias = "QK_IER")]
pub type QkIer = crate::Reg<qk_ier::QkIerSpec>;
#[doc = "Interrupt Enable register; With the Interrupt Enable register all QuiddiKey interrupts can be enabled or disabled, without changing the Interrupt Mask register."]
pub mod qk_ier;
#[doc = "QK_IMR (rw) register accessor: Interrupt Mask register; Individual QuiddiKey interrupts can be enabled or disabled with the Interrupt Mask register.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_imr`] module"]
#[doc(alias = "QK_IMR")]
pub type QkImr = crate::Reg<qk_imr::QkImrSpec>;
#[doc = "Interrupt Mask register; Individual QuiddiKey interrupts can be enabled or disabled with the Interrupt Mask register."]
pub mod qk_imr;
#[doc = "QK_ISR (rw) register accessor: Interrupt Status register; The Interrupt Status register indicates which interrupt has occurred. Multiple interrupts may occur at the same time (e.g. the busy interrupt occurs together with one of the ok and error interrupts). Writing 1 to a bit, clears the corresponding bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_isr`] module"]
#[doc(alias = "QK_ISR")]
pub type QkIsr = crate::Reg<qk_isr::QkIsrSpec>;
#[doc = "Interrupt Status register; The Interrupt Status register indicates which interrupt has occurred. Multiple interrupts may occur at the same time (e.g. the busy interrupt occurs together with one of the ok and error interrupts). Writing 1 to a bit, clears the corresponding bit."]
pub mod qk_isr;
#[doc = "QK_DATA_DEST (rw) register accessor: Data Destination register; The Data Destination register defines via which interface data is made available by QuiddiKey. Only one bit at a time may be 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_data_dest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_data_dest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_data_dest`] module"]
#[doc(alias = "QK_DATA_DEST")]
pub type QkDataDest = crate::Reg<qk_data_dest::QkDataDestSpec>;
#[doc = "Data Destination register; The Data Destination register defines via which interface data is made available by QuiddiKey. Only one bit at a time may be 1."]
pub mod qk_data_dest;
#[doc = "QK_DATA_SRC (rw) register accessor: Data Source register; The Data Source register defines via which interface data must be provided to QuiddiKey. Combinations of bits may be 1 at a time.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_data_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_data_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_data_src`] module"]
#[doc(alias = "QK_DATA_SRC")]
pub type QkDataSrc = crate::Reg<qk_data_src::QkDataSrcSpec>;
#[doc = "Data Source register; The Data Source register defines via which interface data must be provided to QuiddiKey. Combinations of bits may be 1 at a time."]
pub mod qk_data_src;
#[doc = "QK_KEY_INDEX (r) register accessor: Key Index register; The Key Index register provides the index of the key that is currently output.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_key_index::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_key_index`] module"]
#[doc(alias = "QK_KEY_INDEX")]
pub type QkKeyIndex = crate::Reg<qk_key_index::QkKeyIndexSpec>;
#[doc = "Key Index register; The Key Index register provides the index of the key that is currently output."]
pub mod qk_key_index;
#[doc = "QK_DIR (w) register accessor: Data Input register; The Data Input register is used to transfer data to QuiddiKey. It can only be written when input data is requested (indicated by the qk_di_request signal). When written at any other moment, or when read, an error response is generated. Depending on the way the system is configured and connected, this may generate an exception.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_dir`] module"]
#[doc(alias = "QK_DIR")]
pub type QkDir = crate::Reg<qk_dir::QkDirSpec>;
#[doc = "Data Input register; The Data Input register is used to transfer data to QuiddiKey. It can only be written when input data is requested (indicated by the qk_di_request signal). When written at any other moment, or when read, an error response is generated. Depending on the way the system is configured and connected, this may generate an exception."]
pub mod qk_dir;
#[doc = "QK_DOR (r) register accessor: Data Output register; The Data Output register is used to transfer data from QuiddiKey. It can only be read when input data is available (indicated by the qk_do_request signal). When read at any other moment, or when written, an error response is generated. Depending on the way the system is configured and connected, this may generate an exception.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_dor`] module"]
#[doc(alias = "QK_DOR")]
pub type QkDor = crate::Reg<qk_dor::QkDorSpec>;
#[doc = "Data Output register; The Data Output register is used to transfer data from QuiddiKey. It can only be read when input data is available (indicated by the qk_do_request signal). When read at any other moment, or when written, an error response is generated. Depending on the way the system is configured and connected, this may generate an exception."]
pub mod qk_dor;
#[doc = "QK_MISC (rw) register accessor: Miscellaneous register; The Miscellaneous register defines miscellaneous settings for QuiddiKey. These usually need to be configured only once after power up orreset.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_misc`] module"]
#[doc(alias = "QK_MISC")]
pub type QkMisc = crate::Reg<qk_misc::QkMiscSpec>;
#[doc = "Miscellaneous register; The Miscellaneous register defines miscellaneous settings for QuiddiKey. These usually need to be configured only once after power up orreset."]
pub mod qk_misc;
#[doc = "QK_IF_SR (rw) register accessor: Interface Status register; The status of the interface can be monitored with the Interface Status register. Writing 1 to a bit, clears the corresponding bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_if_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_if_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_if_sr`] module"]
#[doc(alias = "QK_IF_SR")]
pub type QkIfSr = crate::Reg<qk_if_sr::QkIfSrSpec>;
#[doc = "Interface Status register; The status of the interface can be monitored with the Interface Status register. Writing 1 to a bit, clears the corresponding bit."]
pub mod qk_if_sr;
#[doc = "QK_TEST (rw) register accessor: Test register; With the Test register a BIST can be started. It also shows the result of the last BIST.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qk_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_test`] module"]
#[doc(alias = "QK_TEST")]
pub type QkTest = crate::Reg<qk_test::QkTestSpec>;
#[doc = "Test register; With the Test register a BIST can be started. It also shows the result of the last BIST."]
pub mod qk_test;
#[doc = "QK_PSR (r) register accessor: PUF Score register; The PUF Score register shows the value of the PUF Score that was obtained during the last Test PUF, Enroll, Start or Reconstruct operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_psr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_psr`] module"]
#[doc(alias = "QK_PSR")]
pub type QkPsr = crate::Reg<qk_psr::QkPsrSpec>;
#[doc = "PUF Score register; The PUF Score register shows the value of the PUF Score that was obtained during the last Test PUF, Enroll, Start or Reconstruct operation."]
pub mod qk_psr;
#[doc = "QK_HW_RUC0 (r) register accessor: Module Restrict User Context 0 register; The Module Restrict User Context 0 register shows the values of the qk_restrict_user_context_0 signal on the signal interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_ruc0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_hw_ruc0`] module"]
#[doc(alias = "QK_HW_RUC0")]
pub type QkHwRuc0 = crate::Reg<qk_hw_ruc0::QkHwRuc0Spec>;
#[doc = "Module Restrict User Context 0 register; The Module Restrict User Context 0 register shows the values of the qk_restrict_user_context_0 signal on the signal interface."]
pub mod qk_hw_ruc0;
#[doc = "QK_HW_RUC1 (r) register accessor: Module Restrict User Context 1 register; The Module Restrict User Context 1 register shows the values of the qk_restrict_user_context_1 signal on the signal interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_ruc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_hw_ruc1`] module"]
#[doc(alias = "QK_HW_RUC1")]
pub type QkHwRuc1 = crate::Reg<qk_hw_ruc1::QkHwRuc1Spec>;
#[doc = "Module Restrict User Context 1 register; The Module Restrict User Context 1 register shows the values of the qk_restrict_user_context_1 signal on the signal interface."]
pub mod qk_hw_ruc1;
#[doc = "QK_HW_SETTINGS (r) register accessor: Module Settings register; The Module Settings register shows the values of the module settings on the signal interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_settings::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_hw_settings`] module"]
#[doc(alias = "QK_HW_SETTINGS")]
pub type QkHwSettings = crate::Reg<qk_hw_settings::QkHwSettingsSpec>;
#[doc = "Module Settings register; The Module Settings register shows the values of the module settings on the signal interface."]
pub mod qk_hw_settings;
#[doc = "QK_HW_INFO (r) register accessor: Module Information register; The Module Information register indicates which configuration of QuiddiKey is used for this module and which options it includes.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_hw_info`] module"]
#[doc(alias = "QK_HW_INFO")]
pub type QkHwInfo = crate::Reg<qk_hw_info::QkHwInfoSpec>;
#[doc = "Module Information register; The Module Information register indicates which configuration of QuiddiKey is used for this module and which options it includes."]
pub mod qk_hw_info;
#[doc = "QK_HW_ID (r) register accessor: Module Identifier register; The Module Identifier register provides the identifier of the QuiddiKey Engine netlist.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_hw_id`] module"]
#[doc(alias = "QK_HW_ID")]
pub type QkHwId = crate::Reg<qk_hw_id::QkHwIdSpec>;
#[doc = "Module Identifier register; The Module Identifier register provides the identifier of the QuiddiKey Engine netlist."]
pub mod qk_hw_id;
#[doc = "QK_HW_VER (r) register accessor: Module Version register; The Module Version register provides the version of this module: major.minor.patch.\n\nYou can [`read`](crate::Reg::read) this register and get [`qk_hw_ver::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qk_hw_ver`] module"]
#[doc(alias = "QK_HW_VER")]
pub type QkHwVer = crate::Reg<qk_hw_ver::QkHwVerSpec>;
#[doc = "Module Version register; The Module Version register provides the version of this module: major.minor.patch."]
pub mod qk_hw_ver;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0x34],
    statusa: Statusa,
    _reserved2: [u8; 0x20],
    syncbusy: Syncbusy,
    _reserved3: [u8; 0xffa0],
    crypto_master_dma_registers__fetch_addr_lsb: CryptoMasterDmaRegisters_FetchAddrLsb,
    crypto_master_dma_registers__fetch_addr_msb: CryptoMasterDmaRegisters_FetchAddrMsb,
    crypto_master_dma_registers__fetch_len: CryptoMasterDmaRegisters_FetchLen,
    crypto_master_dma_registers__fetch_tag: CryptoMasterDmaRegisters_FetchTag,
    crypto_master_dma_registers__push_addr_lsb: CryptoMasterDmaRegisters_PushAddrLsb,
    crypto_master_dma_registers__push_addr_msb: CryptoMasterDmaRegisters_PushAddrMsb,
    crypto_master_dma_registers__push_len: CryptoMasterDmaRegisters_PushLen,
    crypto_master_dma_registers__int_en: CryptoMasterDmaRegisters_IntEn,
    crypto_master_dma_registers__int_enset: CryptoMasterDmaRegisters_IntEnset,
    crypto_master_dma_registers__int_enclr: CryptoMasterDmaRegisters_IntEnclr,
    crypto_master_dma_registers__int_statraw: CryptoMasterDmaRegisters_IntStatraw,
    crypto_master_dma_registers__int_stat: CryptoMasterDmaRegisters_IntStat,
    crypto_master_dma_registers__int_statclr: CryptoMasterDmaRegisters_IntStatclr,
    crypto_master_dma_registers__config: CryptoMasterDmaRegisters_Config,
    crypto_master_dma_registers__start: CryptoMasterDmaRegisters_Start,
    crypto_master_dma_registers__status: CryptoMasterDmaRegisters_Status,
    _reserved19: [u8; 0x03c0],
    crypto_master_hwconf_registers__incl_ips_hw_cfg: CryptoMasterHwconfRegisters_InclIpsHwCfg,
    crypto_master_hwconf_registers__ba411e_aes_hw_cfg_1: CryptoMasterHwconfRegisters_Ba411eAesHwCfg1,
    crypto_master_hwconf_registers__ba411e_aes_hw_cfg_2: CryptoMasterHwconfRegisters_Ba411eAesHwCfg2,
    crypto_master_hwconf_registers__ba413_hash_hw_cfg: CryptoMasterHwconfRegisters_Ba413HashHwCfg,
    crypto_master_hwconf_registers__ba418_sha3_hw_cfg: CryptoMasterHwconfRegisters_Ba418Sha3HwCfg,
    crypto_master_hwconf_registers__ba419_sm4_hw_cfg: CryptoMasterHwconfRegisters_Ba419Sm4HwCfg,
    crypto_master_hwconf_registers__ba424_aria_hw_cfg: CryptoMasterHwconfRegisters_Ba424AriaHwCfg,
    _reserved26: [u8; 0x0be4],
    rng_control_registers__control: RngControlRegisters_Control,
    rng_control_registers__fifolevel: RngControlRegisters_Fifolevel,
    rng_control_registers__fifothreshold: RngControlRegisters_Fifothreshold,
    rng_control_registers__fifodepth: RngControlRegisters_Fifodepth,
    rng_control_registers__key0: RngControlRegisters_Key0,
    rng_control_registers__key1: RngControlRegisters_Key1,
    rng_control_registers__key2: RngControlRegisters_Key2,
    rng_control_registers__key3: RngControlRegisters_Key3,
    rng_control_registers__test_data: RngControlRegisters_TestData,
    rng_control_registers__repeat_threshold: RngControlRegisters_RepeatThreshold,
    rng_control_registers__prop_threshold: RngControlRegisters_PropThreshold,
    _reserved37: [u8; 0x04],
    rng_control_registers__status: RngControlRegisters_Status,
    rng_control_registers__init_wait_val: RngControlRegisters_InitWaitVal,
    rng_control_registers__disable_osc0: RngControlRegisters_DisableOsc0,
    rng_control_registers__disable_osc1: RngControlRegisters_DisableOsc1,
    rng_control_registers__sw_off_tmr_val: RngControlRegisters_SwOffTmrVal,
    rng_control_registers__clk_div: RngControlRegisters_ClkDiv,
    rng_control_registers__ais31conf0: RngControlRegisters_Ais31conf0,
    rng_control_registers__ais31conf1: RngControlRegisters_Ais31conf1,
    rng_control_registers__ais31conf2: RngControlRegisters_Ais31conf2,
    rng_control_registers__ais31status: RngControlRegisters_Ais31status,
    rng_control_registers__hw_config: RngControlRegisters_HwConfig,
    _reserved48: [u8; 0x0fa4],
    pk_registers__pointers: PkRegisters_Pointers,
    pk_registers__command: PkRegisters_Command,
    pk_registers__control: PkRegisters_Control,
    pk_registers__status: PkRegisters_Status,
    _reserved52: [u8; 0x04],
    pk_registers__timer: PkRegisters_Timer,
    pk_registers__hw_config: PkRegisters_HwConfig,
    pk_registers__op_size: PkRegisters_OpSize,
    pk_registers__mem_offset: PkRegisters_MemOffset,
    pk_registers__micro_code_offset: PkRegisters_MicroCodeOffset,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x38 - STATUS Register A"]
    #[inline(always)]
    pub const fn statusa(&self) -> &Statusa {
        &self.statusa
    }
    #[doc = "0x5c - SYNCBUSY Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x10000 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__fetch_addr_lsb(&self) -> &CryptoMasterDmaRegisters_FetchAddrLsb {
        &self.crypto_master_dma_registers__fetch_addr_lsb
    }
    #[doc = "0x10004 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__fetch_addr_msb(&self) -> &CryptoMasterDmaRegisters_FetchAddrMsb {
        &self.crypto_master_dma_registers__fetch_addr_msb
    }
    #[doc = "0x10008 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__fetch_len(&self) -> &CryptoMasterDmaRegisters_FetchLen {
        &self.crypto_master_dma_registers__fetch_len
    }
    #[doc = "0x1000c - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__fetch_tag(&self) -> &CryptoMasterDmaRegisters_FetchTag {
        &self.crypto_master_dma_registers__fetch_tag
    }
    #[doc = "0x10010 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__push_addr_lsb(&self) -> &CryptoMasterDmaRegisters_PushAddrLsb {
        &self.crypto_master_dma_registers__push_addr_lsb
    }
    #[doc = "0x10014 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__push_addr_msb(&self) -> &CryptoMasterDmaRegisters_PushAddrMsb {
        &self.crypto_master_dma_registers__push_addr_msb
    }
    #[doc = "0x10018 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__push_len(&self) -> &CryptoMasterDmaRegisters_PushLen {
        &self.crypto_master_dma_registers__push_len
    }
    #[doc = "0x1001c - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__int_en(&self) -> &CryptoMasterDmaRegisters_IntEn {
        &self.crypto_master_dma_registers__int_en
    }
    #[doc = "0x10020 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__int_enset(&self) -> &CryptoMasterDmaRegisters_IntEnset {
        &self.crypto_master_dma_registers__int_enset
    }
    #[doc = "0x10024 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__int_enclr(&self) -> &CryptoMasterDmaRegisters_IntEnclr {
        &self.crypto_master_dma_registers__int_enclr
    }
    #[doc = "0x10028 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__int_statraw(&self) -> &CryptoMasterDmaRegisters_IntStatraw {
        &self.crypto_master_dma_registers__int_statraw
    }
    #[doc = "0x1002c - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__int_stat(&self) -> &CryptoMasterDmaRegisters_IntStat {
        &self.crypto_master_dma_registers__int_stat
    }
    #[doc = "0x10030 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__int_statclr(&self) -> &CryptoMasterDmaRegisters_IntStatclr {
        &self.crypto_master_dma_registers__int_statclr
    }
    #[doc = "0x10034 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__config(&self) -> &CryptoMasterDmaRegisters_Config {
        &self.crypto_master_dma_registers__config
    }
    #[doc = "0x10038 - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__start(&self) -> &CryptoMasterDmaRegisters_Start {
        &self.crypto_master_dma_registers__start
    }
    #[doc = "0x1003c - "]
    #[inline(always)]
    pub const fn crypto_master_dma_registers__status(&self) -> &CryptoMasterDmaRegisters_Status {
        &self.crypto_master_dma_registers__status
    }
    #[doc = "0x10400 - "]
    #[inline(always)]
    pub const fn crypto_master_hwconf_registers__incl_ips_hw_cfg(&self) -> &CryptoMasterHwconfRegisters_InclIpsHwCfg {
        &self.crypto_master_hwconf_registers__incl_ips_hw_cfg
    }
    #[doc = "0x10404 - "]
    #[inline(always)]
    pub const fn crypto_master_hwconf_registers__ba411e_aes_hw_cfg_1(&self) -> &CryptoMasterHwconfRegisters_Ba411eAesHwCfg1 {
        &self.crypto_master_hwconf_registers__ba411e_aes_hw_cfg_1
    }
    #[doc = "0x10408 - Generic g_CtrSize value. BA411E-AES engine configuration."]
    #[inline(always)]
    pub const fn crypto_master_hwconf_registers__ba411e_aes_hw_cfg_2(&self) -> &CryptoMasterHwconfRegisters_Ba411eAesHwCfg2 {
        &self.crypto_master_hwconf_registers__ba411e_aes_hw_cfg_2
    }
    #[doc = "0x1040c - "]
    #[inline(always)]
    pub const fn crypto_master_hwconf_registers__ba413_hash_hw_cfg(&self) -> &CryptoMasterHwconfRegisters_Ba413HashHwCfg {
        &self.crypto_master_hwconf_registers__ba413_hash_hw_cfg
    }
    #[doc = "0x10410 - Generic g_Sha3CtxtEn value. BA418-SHA3 configuration."]
    #[inline(always)]
    pub const fn crypto_master_hwconf_registers__ba418_sha3_hw_cfg(&self) -> &CryptoMasterHwconfRegisters_Ba418Sha3HwCfg {
        &self.crypto_master_hwconf_registers__ba418_sha3_hw_cfg
    }
    #[doc = "0x10414 - Generic g_SM4ModesPoss value. BA419-SM4 engine configuration."]
    #[inline(always)]
    pub const fn crypto_master_hwconf_registers__ba419_sm4_hw_cfg(&self) -> &CryptoMasterHwconfRegisters_Ba419Sm4HwCfg {
        &self.crypto_master_hwconf_registers__ba419_sm4_hw_cfg
    }
    #[doc = "0x10418 - Generic g_aria_modePoss value. BA424-Aria engine configuration."]
    #[inline(always)]
    pub const fn crypto_master_hwconf_registers__ba424_aria_hw_cfg(&self) -> &CryptoMasterHwconfRegisters_Ba424AriaHwCfg {
        &self.crypto_master_hwconf_registers__ba424_aria_hw_cfg
    }
    #[doc = "0x11000 - Control register"]
    #[inline(always)]
    pub const fn rng_control_registers__control(&self) -> &RngControlRegisters_Control {
        &self.rng_control_registers__control
    }
    #[doc = "0x11004 - FIFO level register."]
    #[inline(always)]
    pub const fn rng_control_registers__fifolevel(&self) -> &RngControlRegisters_Fifolevel {
        &self.rng_control_registers__fifolevel
    }
    #[doc = "0x11008 - FIFO threshold register."]
    #[inline(always)]
    pub const fn rng_control_registers__fifothreshold(&self) -> &RngControlRegisters_Fifothreshold {
        &self.rng_control_registers__fifothreshold
    }
    #[doc = "0x1100c - FIFO depth register."]
    #[inline(always)]
    pub const fn rng_control_registers__fifodepth(&self) -> &RngControlRegisters_Fifodepth {
        &self.rng_control_registers__fifodepth
    }
    #[doc = "0x11010 - Key register (MSB)."]
    #[inline(always)]
    pub const fn rng_control_registers__key0(&self) -> &RngControlRegisters_Key0 {
        &self.rng_control_registers__key0
    }
    #[doc = "0x11014 - Key register."]
    #[inline(always)]
    pub const fn rng_control_registers__key1(&self) -> &RngControlRegisters_Key1 {
        &self.rng_control_registers__key1
    }
    #[doc = "0x11018 - Key register."]
    #[inline(always)]
    pub const fn rng_control_registers__key2(&self) -> &RngControlRegisters_Key2 {
        &self.rng_control_registers__key2
    }
    #[doc = "0x1101c - Key register (LSB)."]
    #[inline(always)]
    pub const fn rng_control_registers__key3(&self) -> &RngControlRegisters_Key3 {
        &self.rng_control_registers__key3
    }
    #[doc = "0x11020 - Test data register."]
    #[inline(always)]
    pub const fn rng_control_registers__test_data(&self) -> &RngControlRegisters_TestData {
        &self.rng_control_registers__test_data
    }
    #[doc = "0x11024 - Repetition Test Count Cut-Off value."]
    #[inline(always)]
    pub const fn rng_control_registers__repeat_threshold(&self) -> &RngControlRegisters_RepeatThreshold {
        &self.rng_control_registers__repeat_threshold
    }
    #[doc = "0x11028 - Adaptive Proportion Test (1024-sample window) Cut-Off value."]
    #[inline(always)]
    pub const fn rng_control_registers__prop_threshold(&self) -> &RngControlRegisters_PropThreshold {
        &self.rng_control_registers__prop_threshold
    }
    #[doc = "0x11030 - Status register."]
    #[inline(always)]
    pub const fn rng_control_registers__status(&self) -> &RngControlRegisters_Status {
        &self.rng_control_registers__status
    }
    #[doc = "0x11034 - Initial wait counter value."]
    #[inline(always)]
    pub const fn rng_control_registers__init_wait_val(&self) -> &RngControlRegisters_InitWaitVal {
        &self.rng_control_registers__init_wait_val
    }
    #[doc = "0x11038 - Disable oscillator rings #0 to #31."]
    #[inline(always)]
    pub const fn rng_control_registers__disable_osc0(&self) -> &RngControlRegisters_DisableOsc0 {
        &self.rng_control_registers__disable_osc0
    }
    #[doc = "0x1103c - Disable oscillator rings #32 to #63."]
    #[inline(always)]
    pub const fn rng_control_registers__disable_osc1(&self) -> &RngControlRegisters_DisableOsc1 {
        &self.rng_control_registers__disable_osc1
    }
    #[doc = "0x11040 - Switch off timer value."]
    #[inline(always)]
    pub const fn rng_control_registers__sw_off_tmr_val(&self) -> &RngControlRegisters_SwOffTmrVal {
        &self.rng_control_registers__sw_off_tmr_val
    }
    #[doc = "0x11044 - Sample clock divider."]
    #[inline(always)]
    pub const fn rng_control_registers__clk_div(&self) -> &RngControlRegisters_ClkDiv {
        &self.rng_control_registers__clk_div
    }
    #[doc = "0x11048 - AIS31 configuration register 0."]
    #[inline(always)]
    pub const fn rng_control_registers__ais31conf0(&self) -> &RngControlRegisters_Ais31conf0 {
        &self.rng_control_registers__ais31conf0
    }
    #[doc = "0x1104c - AIS31 configuration register 1."]
    #[inline(always)]
    pub const fn rng_control_registers__ais31conf1(&self) -> &RngControlRegisters_Ais31conf1 {
        &self.rng_control_registers__ais31conf1
    }
    #[doc = "0x11050 - AIS31 configuration register 2."]
    #[inline(always)]
    pub const fn rng_control_registers__ais31conf2(&self) -> &RngControlRegisters_Ais31conf2 {
        &self.rng_control_registers__ais31conf2
    }
    #[doc = "0x11054 - AIS31 status register."]
    #[inline(always)]
    pub const fn rng_control_registers__ais31status(&self) -> &RngControlRegisters_Ais31status {
        &self.rng_control_registers__ais31status
    }
    #[doc = "0x11058 - Hardware configuration register."]
    #[inline(always)]
    pub const fn rng_control_registers__hw_config(&self) -> &RngControlRegisters_HwConfig {
        &self.rng_control_registers__hw_config
    }
    #[doc = "0x12000 - Pointers register."]
    #[inline(always)]
    pub const fn pk_registers__pointers(&self) -> &PkRegisters_Pointers {
        &self.pk_registers__pointers
    }
    #[doc = "0x12004 - Command register."]
    #[inline(always)]
    pub const fn pk_registers__command(&self) -> &PkRegisters_Command {
        &self.pk_registers__command
    }
    #[doc = "0x12008 - Command register."]
    #[inline(always)]
    pub const fn pk_registers__control(&self) -> &PkRegisters_Control {
        &self.pk_registers__control
    }
    #[doc = "0x1200c - Status register."]
    #[inline(always)]
    pub const fn pk_registers__status(&self) -> &PkRegisters_Status {
        &self.pk_registers__status
    }
    #[doc = "0x12014 - Timer register."]
    #[inline(always)]
    pub const fn pk_registers__timer(&self) -> &PkRegisters_Timer {
        &self.pk_registers__timer
    }
    #[doc = "0x12018 - Hardware configuration register."]
    #[inline(always)]
    pub const fn pk_registers__hw_config(&self) -> &PkRegisters_HwConfig {
        &self.pk_registers__hw_config
    }
    #[doc = "0x1201c - Operand size register."]
    #[inline(always)]
    pub const fn pk_registers__op_size(&self) -> &PkRegisters_OpSize {
        &self.pk_registers__op_size
    }
    #[doc = "0x12020 - Memory offset register."]
    #[inline(always)]
    pub const fn pk_registers__mem_offset(&self) -> &PkRegisters_MemOffset {
        &self.pk_registers__mem_offset
    }
    #[doc = "0x12024 - MicroCode offset register."]
    #[inline(always)]
    pub const fn pk_registers__micro_code_offset(&self) -> &PkRegisters_MicroCodeOffset {
        &self.pk_registers__micro_code_offset
    }
}
#[doc = "CTRLA (rw) register accessor: Control Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control Register A"]
pub mod ctrla;
#[doc = "STATUSA (r) register accessor: STATUS Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusa`] module"]
#[doc(alias = "STATUSA")]
pub type Statusa = crate::Reg<statusa::StatusaSpec>;
#[doc = "STATUS Register A"]
pub mod statusa;
#[doc = "SYNCBUSY (r) register accessor: SYNCBUSY Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "SYNCBUSY Register"]
pub mod syncbusy;
#[doc = "CryptoMaster_DMA_Registers__FETCH_ADDR_LSB (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__fetch_addr_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__fetch_addr_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__fetch_addr_lsb`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__FETCH_ADDR_LSB")]
pub type CryptoMasterDmaRegisters_FetchAddrLsb = crate::Reg<crypto_master_dma_registers__fetch_addr_lsb::CryptoMasterDmaRegisters_FetchAddrLsbSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__fetch_addr_lsb;
#[doc = "CryptoMaster_DMA_Registers__FETCH_ADDR_MSB (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__fetch_addr_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__fetch_addr_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__fetch_addr_msb`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__FETCH_ADDR_MSB")]
pub type CryptoMasterDmaRegisters_FetchAddrMsb = crate::Reg<crypto_master_dma_registers__fetch_addr_msb::CryptoMasterDmaRegisters_FetchAddrMsbSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__fetch_addr_msb;
#[doc = "CryptoMaster_DMA_Registers__FETCH_LEN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__fetch_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__fetch_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__fetch_len`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__FETCH_LEN")]
pub type CryptoMasterDmaRegisters_FetchLen = crate::Reg<crypto_master_dma_registers__fetch_len::CryptoMasterDmaRegisters_FetchLenSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__fetch_len;
#[doc = "CryptoMaster_DMA_Registers__FETCH_TAG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__fetch_tag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__fetch_tag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__fetch_tag`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__FETCH_TAG")]
pub type CryptoMasterDmaRegisters_FetchTag = crate::Reg<crypto_master_dma_registers__fetch_tag::CryptoMasterDmaRegisters_FetchTagSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__fetch_tag;
#[doc = "CryptoMaster_DMA_Registers__PUSH_ADDR_LSB (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__push_addr_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__push_addr_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__push_addr_lsb`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__PUSH_ADDR_LSB")]
pub type CryptoMasterDmaRegisters_PushAddrLsb = crate::Reg<crypto_master_dma_registers__push_addr_lsb::CryptoMasterDmaRegisters_PushAddrLsbSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__push_addr_lsb;
#[doc = "CryptoMaster_DMA_Registers__PUSH_ADDR_MSB (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__push_addr_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__push_addr_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__push_addr_msb`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__PUSH_ADDR_MSB")]
pub type CryptoMasterDmaRegisters_PushAddrMsb = crate::Reg<crypto_master_dma_registers__push_addr_msb::CryptoMasterDmaRegisters_PushAddrMsbSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__push_addr_msb;
#[doc = "CryptoMaster_DMA_Registers__PUSH_LEN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__push_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__push_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__push_len`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__PUSH_LEN")]
pub type CryptoMasterDmaRegisters_PushLen = crate::Reg<crypto_master_dma_registers__push_len::CryptoMasterDmaRegisters_PushLenSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__push_len;
#[doc = "CryptoMaster_DMA_Registers__INT_EN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__int_en`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__INT_EN")]
pub type CryptoMasterDmaRegisters_IntEn = crate::Reg<crypto_master_dma_registers__int_en::CryptoMasterDmaRegisters_IntEnSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__int_en;
#[doc = "CryptoMaster_DMA_Registers__INT_ENSET (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__int_enset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__int_enset`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__INT_ENSET")]
pub type CryptoMasterDmaRegisters_IntEnset = crate::Reg<crypto_master_dma_registers__int_enset::CryptoMasterDmaRegisters_IntEnsetSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__int_enset;
#[doc = "CryptoMaster_DMA_Registers__INT_ENCLR (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__int_enclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__int_enclr`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__INT_ENCLR")]
pub type CryptoMasterDmaRegisters_IntEnclr = crate::Reg<crypto_master_dma_registers__int_enclr::CryptoMasterDmaRegisters_IntEnclrSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__int_enclr;
#[doc = "CryptoMaster_DMA_Registers__INT_STATRAW (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__int_statraw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__int_statraw`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__INT_STATRAW")]
pub type CryptoMasterDmaRegisters_IntStatraw = crate::Reg<crypto_master_dma_registers__int_statraw::CryptoMasterDmaRegisters_IntStatrawSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__int_statraw;
#[doc = "CryptoMaster_DMA_Registers__INT_STAT (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__int_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__int_stat`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__INT_STAT")]
pub type CryptoMasterDmaRegisters_IntStat = crate::Reg<crypto_master_dma_registers__int_stat::CryptoMasterDmaRegisters_IntStatSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__int_stat;
#[doc = "CryptoMaster_DMA_Registers__INT_STATCLR (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__int_statclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__int_statclr`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__INT_STATCLR")]
pub type CryptoMasterDmaRegisters_IntStatclr = crate::Reg<crypto_master_dma_registers__int_statclr::CryptoMasterDmaRegisters_IntStatclrSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__int_statclr;
#[doc = "CryptoMaster_DMA_Registers__CONFIG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__config`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__CONFIG")]
pub type CryptoMasterDmaRegisters_Config = crate::Reg<crypto_master_dma_registers__config::CryptoMasterDmaRegisters_ConfigSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__config;
#[doc = "CryptoMaster_DMA_Registers__START (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_master_dma_registers__start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__start`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__START")]
pub type CryptoMasterDmaRegisters_Start = crate::Reg<crypto_master_dma_registers__start::CryptoMasterDmaRegisters_StartSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__start;
#[doc = "CryptoMaster_DMA_Registers__STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_dma_registers__status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_dma_registers__status`] module"]
#[doc(alias = "CryptoMaster_DMA_Registers__STATUS")]
pub type CryptoMasterDmaRegisters_Status = crate::Reg<crypto_master_dma_registers__status::CryptoMasterDmaRegisters_StatusSpec>;
#[doc = ""]
pub mod crypto_master_dma_registers__status;
#[doc = "CryptoMaster_HWConf_Registers__INCL_IPS_HW_CFG (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__incl_ips_hw_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_hwconf_registers__incl_ips_hw_cfg`] module"]
#[doc(alias = "CryptoMaster_HWConf_Registers__INCL_IPS_HW_CFG")]
pub type CryptoMasterHwconfRegisters_InclIpsHwCfg = crate::Reg<crypto_master_hwconf_registers__incl_ips_hw_cfg::CryptoMasterHwconfRegisters_InclIpsHwCfgSpec>;
#[doc = ""]
pub mod crypto_master_hwconf_registers__incl_ips_hw_cfg;
#[doc = "CryptoMaster_HWConf_Registers__BA411E_AES_HW_CFG_1 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba411e_aes_hw_cfg_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_hwconf_registers__ba411e_aes_hw_cfg_1`] module"]
#[doc(alias = "CryptoMaster_HWConf_Registers__BA411E_AES_HW_CFG_1")]
pub type CryptoMasterHwconfRegisters_Ba411eAesHwCfg1 = crate::Reg<crypto_master_hwconf_registers__ba411e_aes_hw_cfg_1::CryptoMasterHwconfRegisters_Ba411eAesHwCfg1Spec>;
#[doc = ""]
pub mod crypto_master_hwconf_registers__ba411e_aes_hw_cfg_1;
#[doc = "CryptoMaster_HWConf_Registers__BA411E_AES_HW_CFG_2 (r) register accessor: Generic g_CtrSize value. BA411E-AES engine configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba411e_aes_hw_cfg_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_hwconf_registers__ba411e_aes_hw_cfg_2`] module"]
#[doc(alias = "CryptoMaster_HWConf_Registers__BA411E_AES_HW_CFG_2")]
pub type CryptoMasterHwconfRegisters_Ba411eAesHwCfg2 = crate::Reg<crypto_master_hwconf_registers__ba411e_aes_hw_cfg_2::CryptoMasterHwconfRegisters_Ba411eAesHwCfg2Spec>;
#[doc = "Generic g_CtrSize value. BA411E-AES engine configuration."]
pub mod crypto_master_hwconf_registers__ba411e_aes_hw_cfg_2;
#[doc = "CryptoMaster_HWConf_Registers__BA413_HASH_HW_CFG (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba413_hash_hw_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_hwconf_registers__ba413_hash_hw_cfg`] module"]
#[doc(alias = "CryptoMaster_HWConf_Registers__BA413_HASH_HW_CFG")]
pub type CryptoMasterHwconfRegisters_Ba413HashHwCfg = crate::Reg<crypto_master_hwconf_registers__ba413_hash_hw_cfg::CryptoMasterHwconfRegisters_Ba413HashHwCfgSpec>;
#[doc = ""]
pub mod crypto_master_hwconf_registers__ba413_hash_hw_cfg;
#[doc = "CryptoMaster_HWConf_Registers__BA418_SHA3_HW_CFG (r) register accessor: Generic g_Sha3CtxtEn value. BA418-SHA3 configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba418_sha3_hw_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_hwconf_registers__ba418_sha3_hw_cfg`] module"]
#[doc(alias = "CryptoMaster_HWConf_Registers__BA418_SHA3_HW_CFG")]
pub type CryptoMasterHwconfRegisters_Ba418Sha3HwCfg = crate::Reg<crypto_master_hwconf_registers__ba418_sha3_hw_cfg::CryptoMasterHwconfRegisters_Ba418Sha3HwCfgSpec>;
#[doc = "Generic g_Sha3CtxtEn value. BA418-SHA3 configuration."]
pub mod crypto_master_hwconf_registers__ba418_sha3_hw_cfg;
#[doc = "CryptoMaster_HWConf_Registers__BA419_SM4_HW_CFG (r) register accessor: Generic g_SM4ModesPoss value. BA419-SM4 engine configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba419_sm4_hw_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_hwconf_registers__ba419_sm4_hw_cfg`] module"]
#[doc(alias = "CryptoMaster_HWConf_Registers__BA419_SM4_HW_CFG")]
pub type CryptoMasterHwconfRegisters_Ba419Sm4HwCfg = crate::Reg<crypto_master_hwconf_registers__ba419_sm4_hw_cfg::CryptoMasterHwconfRegisters_Ba419Sm4HwCfgSpec>;
#[doc = "Generic g_SM4ModesPoss value. BA419-SM4 engine configuration."]
pub mod crypto_master_hwconf_registers__ba419_sm4_hw_cfg;
#[doc = "CryptoMaster_HWConf_Registers__BA424_ARIA_HW_CFG (r) register accessor: Generic g_aria_modePoss value. BA424-Aria engine configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_master_hwconf_registers__ba424_aria_hw_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_master_hwconf_registers__ba424_aria_hw_cfg`] module"]
#[doc(alias = "CryptoMaster_HWConf_Registers__BA424_ARIA_HW_CFG")]
pub type CryptoMasterHwconfRegisters_Ba424AriaHwCfg = crate::Reg<crypto_master_hwconf_registers__ba424_aria_hw_cfg::CryptoMasterHwconfRegisters_Ba424AriaHwCfgSpec>;
#[doc = "Generic g_aria_modePoss value. BA424-Aria engine configuration."]
pub mod crypto_master_hwconf_registers__ba424_aria_hw_cfg;
#[doc = "RNG_Control_Registers__Control (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__control`] module"]
#[doc(alias = "RNG_Control_Registers__Control")]
pub type RngControlRegisters_Control = crate::Reg<rng_control_registers__control::RngControlRegisters_ControlSpec>;
#[doc = "Control register"]
pub mod rng_control_registers__control;
#[doc = "RNG_Control_Registers__FIFOLevel (rw) register accessor: FIFO level register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__fifolevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__fifolevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__fifolevel`] module"]
#[doc(alias = "RNG_Control_Registers__FIFOLevel")]
pub type RngControlRegisters_Fifolevel = crate::Reg<rng_control_registers__fifolevel::RngControlRegisters_FifolevelSpec>;
#[doc = "FIFO level register."]
pub mod rng_control_registers__fifolevel;
#[doc = "RNG_Control_Registers__FIFOThreshold (rw) register accessor: FIFO threshold register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__fifothreshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__fifothreshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__fifothreshold`] module"]
#[doc(alias = "RNG_Control_Registers__FIFOThreshold")]
pub type RngControlRegisters_Fifothreshold = crate::Reg<rng_control_registers__fifothreshold::RngControlRegisters_FifothresholdSpec>;
#[doc = "FIFO threshold register."]
pub mod rng_control_registers__fifothreshold;
#[doc = "RNG_Control_Registers__FIFODepth (r) register accessor: FIFO depth register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__fifodepth::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__fifodepth`] module"]
#[doc(alias = "RNG_Control_Registers__FIFODepth")]
pub type RngControlRegisters_Fifodepth = crate::Reg<rng_control_registers__fifodepth::RngControlRegisters_FifodepthSpec>;
#[doc = "FIFO depth register."]
pub mod rng_control_registers__fifodepth;
#[doc = "RNG_Control_Registers__Key0 (rw) register accessor: Key register (MSB).\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__key0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__key0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__key0`] module"]
#[doc(alias = "RNG_Control_Registers__Key0")]
pub type RngControlRegisters_Key0 = crate::Reg<rng_control_registers__key0::RngControlRegisters_Key0Spec>;
#[doc = "Key register (MSB)."]
pub mod rng_control_registers__key0;
#[doc = "RNG_Control_Registers__Key1 (rw) register accessor: Key register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__key1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__key1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__key1`] module"]
#[doc(alias = "RNG_Control_Registers__Key1")]
pub type RngControlRegisters_Key1 = crate::Reg<rng_control_registers__key1::RngControlRegisters_Key1Spec>;
#[doc = "Key register."]
pub mod rng_control_registers__key1;
#[doc = "RNG_Control_Registers__Key2 (rw) register accessor: Key register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__key2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__key2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__key2`] module"]
#[doc(alias = "RNG_Control_Registers__Key2")]
pub type RngControlRegisters_Key2 = crate::Reg<rng_control_registers__key2::RngControlRegisters_Key2Spec>;
#[doc = "Key register."]
pub mod rng_control_registers__key2;
#[doc = "RNG_Control_Registers__Key3 (rw) register accessor: Key register (LSB).\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__key3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__key3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__key3`] module"]
#[doc(alias = "RNG_Control_Registers__Key3")]
pub type RngControlRegisters_Key3 = crate::Reg<rng_control_registers__key3::RngControlRegisters_Key3Spec>;
#[doc = "Key register (LSB)."]
pub mod rng_control_registers__key3;
#[doc = "RNG_Control_Registers__TestData (w) register accessor: Test data register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__test_data::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__test_data`] module"]
#[doc(alias = "RNG_Control_Registers__TestData")]
pub type RngControlRegisters_TestData = crate::Reg<rng_control_registers__test_data::RngControlRegisters_TestDataSpec>;
#[doc = "Test data register."]
pub mod rng_control_registers__test_data;
#[doc = "RNG_Control_Registers__RepeatThreshold (rw) register accessor: Repetition Test Count Cut-Off value.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__repeat_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__repeat_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__repeat_threshold`] module"]
#[doc(alias = "RNG_Control_Registers__RepeatThreshold")]
pub type RngControlRegisters_RepeatThreshold = crate::Reg<rng_control_registers__repeat_threshold::RngControlRegisters_RepeatThresholdSpec>;
#[doc = "Repetition Test Count Cut-Off value."]
pub mod rng_control_registers__repeat_threshold;
#[doc = "RNG_Control_Registers__PropThreshold (rw) register accessor: Adaptive Proportion Test (1024-sample window) Cut-Off value.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__prop_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__prop_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__prop_threshold`] module"]
#[doc(alias = "RNG_Control_Registers__PropThreshold")]
pub type RngControlRegisters_PropThreshold = crate::Reg<rng_control_registers__prop_threshold::RngControlRegisters_PropThresholdSpec>;
#[doc = "Adaptive Proportion Test (1024-sample window) Cut-Off value."]
pub mod rng_control_registers__prop_threshold;
#[doc = "RNG_Control_Registers__Status (rw) register accessor: Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__status`] module"]
#[doc(alias = "RNG_Control_Registers__Status")]
pub type RngControlRegisters_Status = crate::Reg<rng_control_registers__status::RngControlRegisters_StatusSpec>;
#[doc = "Status register."]
pub mod rng_control_registers__status;
#[doc = "RNG_Control_Registers__InitWaitVal (rw) register accessor: Initial wait counter value.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__init_wait_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__init_wait_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__init_wait_val`] module"]
#[doc(alias = "RNG_Control_Registers__InitWaitVal")]
pub type RngControlRegisters_InitWaitVal = crate::Reg<rng_control_registers__init_wait_val::RngControlRegisters_InitWaitValSpec>;
#[doc = "Initial wait counter value."]
pub mod rng_control_registers__init_wait_val;
#[doc = "RNG_Control_Registers__DisableOsc0 (rw) register accessor: Disable oscillator rings #0 to #31.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__disable_osc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__disable_osc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__disable_osc0`] module"]
#[doc(alias = "RNG_Control_Registers__DisableOsc0")]
pub type RngControlRegisters_DisableOsc0 = crate::Reg<rng_control_registers__disable_osc0::RngControlRegisters_DisableOsc0Spec>;
#[doc = "Disable oscillator rings #0 to #31."]
pub mod rng_control_registers__disable_osc0;
#[doc = "RNG_Control_Registers__DisableOsc1 (rw) register accessor: Disable oscillator rings #32 to #63.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__disable_osc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__disable_osc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__disable_osc1`] module"]
#[doc(alias = "RNG_Control_Registers__DisableOsc1")]
pub type RngControlRegisters_DisableOsc1 = crate::Reg<rng_control_registers__disable_osc1::RngControlRegisters_DisableOsc1Spec>;
#[doc = "Disable oscillator rings #32 to #63."]
pub mod rng_control_registers__disable_osc1;
#[doc = "RNG_Control_Registers__SwOffTmrVal (rw) register accessor: Switch off timer value.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__sw_off_tmr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__sw_off_tmr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__sw_off_tmr_val`] module"]
#[doc(alias = "RNG_Control_Registers__SwOffTmrVal")]
pub type RngControlRegisters_SwOffTmrVal = crate::Reg<rng_control_registers__sw_off_tmr_val::RngControlRegisters_SwOffTmrValSpec>;
#[doc = "Switch off timer value."]
pub mod rng_control_registers__sw_off_tmr_val;
#[doc = "RNG_Control_Registers__ClkDiv (rw) register accessor: Sample clock divider.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__clk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__clk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__clk_div`] module"]
#[doc(alias = "RNG_Control_Registers__ClkDiv")]
pub type RngControlRegisters_ClkDiv = crate::Reg<rng_control_registers__clk_div::RngControlRegisters_ClkDivSpec>;
#[doc = "Sample clock divider."]
pub mod rng_control_registers__clk_div;
#[doc = "RNG_Control_Registers__AIS31Conf0 (rw) register accessor: AIS31 configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__ais31conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__ais31conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__ais31conf0`] module"]
#[doc(alias = "RNG_Control_Registers__AIS31Conf0")]
pub type RngControlRegisters_Ais31conf0 = crate::Reg<rng_control_registers__ais31conf0::RngControlRegisters_Ais31conf0Spec>;
#[doc = "AIS31 configuration register 0."]
pub mod rng_control_registers__ais31conf0;
#[doc = "RNG_Control_Registers__AIS31Conf1 (rw) register accessor: AIS31 configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__ais31conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__ais31conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__ais31conf1`] module"]
#[doc(alias = "RNG_Control_Registers__AIS31Conf1")]
pub type RngControlRegisters_Ais31conf1 = crate::Reg<rng_control_registers__ais31conf1::RngControlRegisters_Ais31conf1Spec>;
#[doc = "AIS31 configuration register 1."]
pub mod rng_control_registers__ais31conf1;
#[doc = "RNG_Control_Registers__AIS31Conf2 (rw) register accessor: AIS31 configuration register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__ais31conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__ais31conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__ais31conf2`] module"]
#[doc(alias = "RNG_Control_Registers__AIS31Conf2")]
pub type RngControlRegisters_Ais31conf2 = crate::Reg<rng_control_registers__ais31conf2::RngControlRegisters_Ais31conf2Spec>;
#[doc = "AIS31 configuration register 2."]
pub mod rng_control_registers__ais31conf2;
#[doc = "RNG_Control_Registers__AIS31Status (rw) register accessor: AIS31 status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__ais31status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_control_registers__ais31status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__ais31status`] module"]
#[doc(alias = "RNG_Control_Registers__AIS31Status")]
pub type RngControlRegisters_Ais31status = crate::Reg<rng_control_registers__ais31status::RngControlRegisters_Ais31statusSpec>;
#[doc = "AIS31 status register."]
pub mod rng_control_registers__ais31status;
#[doc = "RNG_Control_Registers__HwConfig (r) register accessor: Hardware configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_control_registers__hw_config::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_control_registers__hw_config`] module"]
#[doc(alias = "RNG_Control_Registers__HwConfig")]
pub type RngControlRegisters_HwConfig = crate::Reg<rng_control_registers__hw_config::RngControlRegisters_HwConfigSpec>;
#[doc = "Hardware configuration register."]
pub mod rng_control_registers__hw_config;
#[doc = "PK_Registers__Pointers (rw) register accessor: Pointers register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__pointers::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__pointers::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_registers__pointers`] module"]
#[doc(alias = "PK_Registers__Pointers")]
pub type PkRegisters_Pointers = crate::Reg<pk_registers__pointers::PkRegisters_PointersSpec>;
#[doc = "Pointers register."]
pub mod pk_registers__pointers;
#[doc = "PK_Registers__Command (rw) register accessor: Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_registers__command`] module"]
#[doc(alias = "PK_Registers__Command")]
pub type PkRegisters_Command = crate::Reg<pk_registers__command::PkRegisters_CommandSpec>;
#[doc = "Command register."]
pub mod pk_registers__command;
#[doc = "PK_Registers__Control (w) register accessor: Command register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__control::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_registers__control`] module"]
#[doc(alias = "PK_Registers__Control")]
pub type PkRegisters_Control = crate::Reg<pk_registers__control::PkRegisters_ControlSpec>;
#[doc = "Command register."]
pub mod pk_registers__control;
#[doc = "PK_Registers__Status (r) register accessor: Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_registers__status`] module"]
#[doc(alias = "PK_Registers__Status")]
pub type PkRegisters_Status = crate::Reg<pk_registers__status::PkRegisters_StatusSpec>;
#[doc = "Status register."]
pub mod pk_registers__status;
#[doc = "PK_Registers__Timer (rw) register accessor: Timer register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_registers__timer`] module"]
#[doc(alias = "PK_Registers__Timer")]
pub type PkRegisters_Timer = crate::Reg<pk_registers__timer::PkRegisters_TimerSpec>;
#[doc = "Timer register."]
pub mod pk_registers__timer;
#[doc = "PK_Registers__HwConfig (r) register accessor: Hardware configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__hw_config::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_registers__hw_config`] module"]
#[doc(alias = "PK_Registers__HwConfig")]
pub type PkRegisters_HwConfig = crate::Reg<pk_registers__hw_config::PkRegisters_HwConfigSpec>;
#[doc = "Hardware configuration register."]
pub mod pk_registers__hw_config;
#[doc = "PK_Registers__OpSize (rw) register accessor: Operand size register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__op_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__op_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_registers__op_size`] module"]
#[doc(alias = "PK_Registers__OpSize")]
pub type PkRegisters_OpSize = crate::Reg<pk_registers__op_size::PkRegisters_OpSizeSpec>;
#[doc = "Operand size register."]
pub mod pk_registers__op_size;
#[doc = "PK_Registers__MemOffset (rw) register accessor: Memory offset register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__mem_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__mem_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_registers__mem_offset`] module"]
#[doc(alias = "PK_Registers__MemOffset")]
pub type PkRegisters_MemOffset = crate::Reg<pk_registers__mem_offset::PkRegisters_MemOffsetSpec>;
#[doc = "Memory offset register."]
pub mod pk_registers__mem_offset;
#[doc = "PK_Registers__MicroCodeOffset (rw) register accessor: MicroCode offset register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_registers__micro_code_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_registers__micro_code_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_registers__micro_code_offset`] module"]
#[doc(alias = "PK_Registers__MicroCodeOffset")]
pub type PkRegisters_MicroCodeOffset = crate::Reg<pk_registers__micro_code_offset::PkRegisters_MicroCodeOffsetSpec>;
#[doc = "MicroCode offset register."]
pub mod pk_registers__micro_code_offset;

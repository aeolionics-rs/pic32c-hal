#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0xfc],
    tamper_key: [TamperKey; 16],
    _reserved2: [u8; 0xc0],
    anti_tamper__levela: AntiTamper_Levela,
    anti_tamper__levelb: AntiTamper_Levelb,
    anti_tamper__levelc: AntiTamper_Levelc,
    anti_tamper__leveld: AntiTamper_Leveld,
    anti_tamper__setlevel: AntiTamper_Setlevel,
    anti_tamper__filtercfg: AntiTamper_Filtercfg,
    anti_tamper__softtamp: AntiTamper_Softtamp,
    anti_tamper__rawstat: AntiTamper_Rawstat,
    anti_tamper__stat: AntiTamper_Stat,
    anti_tamper__timestamp: AntiTamper_Timestamp,
    anti_tamper__filtercnt: AntiTamper_Filtercnt,
    anti_tamper__softinfo: AntiTamper_Softinfo,
    anti_tamper__tamperflag: AntiTamper_Tamperflag,
    anti_tamper__softreset: AntiTamper_Softreset,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x100..0x140 - Tamper Key n Register"]
    #[inline(always)]
    pub const fn tamper_key(&self, n: usize) -> &TamperKey {
        &self.tamper_key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x140 - Tamper Key n Register"]
    #[inline(always)]
    pub fn tamper_key_iter(&self) -> impl Iterator<Item = &TamperKey> {
        self.tamper_key.iter()
    }
    #[doc = "0x200 - Tamper levels 0 to 7."]
    #[inline(always)]
    pub const fn anti_tamper__levela(&self) -> &AntiTamper_Levela {
        &self.anti_tamper__levela
    }
    #[doc = "0x204 - Tamper levels 8 to 15."]
    #[inline(always)]
    pub const fn anti_tamper__levelb(&self) -> &AntiTamper_Levelb {
        &self.anti_tamper__levelb
    }
    #[doc = "0x208 - Tamper levels 16 to 23."]
    #[inline(always)]
    pub const fn anti_tamper__levelc(&self) -> &AntiTamper_Levelc {
        &self.anti_tamper__levelc
    }
    #[doc = "0x20c - Tamper levels 24 to 31."]
    #[inline(always)]
    pub const fn anti_tamper__leveld(&self) -> &AntiTamper_Leveld {
        &self.anti_tamper__leveld
    }
    #[doc = "0x210 - Set tamper level."]
    #[inline(always)]
    pub const fn anti_tamper__setlevel(&self) -> &AntiTamper_Setlevel {
        &self.anti_tamper__setlevel
    }
    #[doc = "0x214 - Filter configuration (threshold, period)."]
    #[inline(always)]
    pub const fn anti_tamper__filtercfg(&self) -> &AntiTamper_Filtercfg {
        &self.anti_tamper__filtercfg
    }
    #[doc = "0x218 - Software triggered tamper."]
    #[inline(always)]
    pub const fn anti_tamper__softtamp(&self) -> &AntiTamper_Softtamp {
        &self.anti_tamper__softtamp
    }
    #[doc = "0x21c - Raw status."]
    #[inline(always)]
    pub const fn anti_tamper__rawstat(&self) -> &AntiTamper_Rawstat {
        &self.anti_tamper__rawstat
    }
    #[doc = "0x220 - Status (write to clear)."]
    #[inline(always)]
    pub const fn anti_tamper__stat(&self) -> &AntiTamper_Stat {
        &self.anti_tamper__stat
    }
    #[doc = "0x224 - Timestamp for the last tamper."]
    #[inline(always)]
    pub const fn anti_tamper__timestamp(&self) -> &AntiTamper_Timestamp {
        &self.anti_tamper__timestamp
    }
    #[doc = "0x228 - Current value of filter counter."]
    #[inline(always)]
    pub const fn anti_tamper__filtercnt(&self) -> &AntiTamper_Filtercnt {
        &self.anti_tamper__filtercnt
    }
    #[doc = "0x22c - Register for software purpose only."]
    #[inline(always)]
    pub const fn anti_tamper__softinfo(&self) -> &AntiTamper_Softinfo {
        &self.anti_tamper__softinfo
    }
    #[doc = "0x230 - The 1-bit TAMPERFLAG register directly controls the TAMPER_FLAG output."]
    #[inline(always)]
    pub const fn anti_tamper__tamperflag(&self) -> &AntiTamper_Tamperflag {
        &self.anti_tamper__tamperflag
    }
    #[doc = "0x234 - Reading the SOFTLVL4 register returns 1 if any active soft tamper has a level of 4 or above."]
    #[inline(always)]
    pub const fn anti_tamper__softreset(&self) -> &AntiTamper_Softreset {
        &self.anti_tamper__softreset
    }
}
#[doc = "CTRLA (rw) register accessor: Control Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control Register A"]
pub mod ctrla;
#[doc = "TAMPER_KEY (rw) register accessor: Tamper Key n Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tamper_key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamper_key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamper_key`] module"]
#[doc(alias = "TAMPER_KEY")]
pub type TamperKey = crate::Reg<tamper_key::TamperKeySpec>;
#[doc = "Tamper Key n Register"]
pub mod tamper_key;
#[doc = "anti_tamper__LEVELA (r) register accessor: Tamper levels 0 to 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__levela::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__levela`] module"]
#[doc(alias = "anti_tamper__LEVELA")]
pub type AntiTamper_Levela = crate::Reg<anti_tamper__levela::AntiTamper_LevelaSpec>;
#[doc = "Tamper levels 0 to 7."]
pub mod anti_tamper__levela;
#[doc = "anti_tamper__LEVELB (r) register accessor: Tamper levels 8 to 15.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__levelb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__levelb`] module"]
#[doc(alias = "anti_tamper__LEVELB")]
pub type AntiTamper_Levelb = crate::Reg<anti_tamper__levelb::AntiTamper_LevelbSpec>;
#[doc = "Tamper levels 8 to 15."]
pub mod anti_tamper__levelb;
#[doc = "anti_tamper__LEVELC (r) register accessor: Tamper levels 16 to 23.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__levelc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__levelc`] module"]
#[doc(alias = "anti_tamper__LEVELC")]
pub type AntiTamper_Levelc = crate::Reg<anti_tamper__levelc::AntiTamper_LevelcSpec>;
#[doc = "Tamper levels 16 to 23."]
pub mod anti_tamper__levelc;
#[doc = "anti_tamper__LEVELD (r) register accessor: Tamper levels 24 to 31.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__leveld::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__leveld`] module"]
#[doc(alias = "anti_tamper__LEVELD")]
pub type AntiTamper_Leveld = crate::Reg<anti_tamper__leveld::AntiTamper_LeveldSpec>;
#[doc = "Tamper levels 24 to 31."]
pub mod anti_tamper__leveld;
#[doc = "anti_tamper__SETLEVEL (w) register accessor: Set tamper level.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__setlevel::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__setlevel`] module"]
#[doc(alias = "anti_tamper__SETLEVEL")]
pub type AntiTamper_Setlevel = crate::Reg<anti_tamper__setlevel::AntiTamper_SetlevelSpec>;
#[doc = "Set tamper level."]
pub mod anti_tamper__setlevel;
#[doc = "anti_tamper__FILTERCFG (rw) register accessor: Filter configuration (threshold, period).\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__filtercfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__filtercfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__filtercfg`] module"]
#[doc(alias = "anti_tamper__FILTERCFG")]
pub type AntiTamper_Filtercfg = crate::Reg<anti_tamper__filtercfg::AntiTamper_FiltercfgSpec>;
#[doc = "Filter configuration (threshold, period)."]
pub mod anti_tamper__filtercfg;
#[doc = "anti_tamper__SOFTTAMP (rw) register accessor: Software triggered tamper.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__softtamp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__softtamp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__softtamp`] module"]
#[doc(alias = "anti_tamper__SOFTTAMP")]
pub type AntiTamper_Softtamp = crate::Reg<anti_tamper__softtamp::AntiTamper_SofttampSpec>;
#[doc = "Software triggered tamper."]
pub mod anti_tamper__softtamp;
#[doc = "anti_tamper__RAWSTAT (r) register accessor: Raw status.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__rawstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__rawstat`] module"]
#[doc(alias = "anti_tamper__RAWSTAT")]
pub type AntiTamper_Rawstat = crate::Reg<anti_tamper__rawstat::AntiTamper_RawstatSpec>;
#[doc = "Raw status."]
pub mod anti_tamper__rawstat;
#[doc = "anti_tamper__STAT (rw) register accessor: Status (write to clear).\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__stat`] module"]
#[doc(alias = "anti_tamper__STAT")]
pub type AntiTamper_Stat = crate::Reg<anti_tamper__stat::AntiTamper_StatSpec>;
#[doc = "Status (write to clear)."]
pub mod anti_tamper__stat;
#[doc = "anti_tamper__TIMESTAMP (r) register accessor: Timestamp for the last tamper.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__timestamp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__timestamp`] module"]
#[doc(alias = "anti_tamper__TIMESTAMP")]
pub type AntiTamper_Timestamp = crate::Reg<anti_tamper__timestamp::AntiTamper_TimestampSpec>;
#[doc = "Timestamp for the last tamper."]
pub mod anti_tamper__timestamp;
#[doc = "anti_tamper__FILTERCNT (r) register accessor: Current value of filter counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__filtercnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__filtercnt`] module"]
#[doc(alias = "anti_tamper__FILTERCNT")]
pub type AntiTamper_Filtercnt = crate::Reg<anti_tamper__filtercnt::AntiTamper_FiltercntSpec>;
#[doc = "Current value of filter counter."]
pub mod anti_tamper__filtercnt;
#[doc = "anti_tamper__SOFTINFO (rw) register accessor: Register for software purpose only.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__softinfo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__softinfo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__softinfo`] module"]
#[doc(alias = "anti_tamper__SOFTINFO")]
pub type AntiTamper_Softinfo = crate::Reg<anti_tamper__softinfo::AntiTamper_SoftinfoSpec>;
#[doc = "Register for software purpose only."]
pub mod anti_tamper__softinfo;
#[doc = "anti_tamper__TAMPERFLAG (rw) register accessor: The 1-bit TAMPERFLAG register directly controls the TAMPER_FLAG output.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__tamperflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anti_tamper__tamperflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__tamperflag`] module"]
#[doc(alias = "anti_tamper__TAMPERFLAG")]
pub type AntiTamper_Tamperflag = crate::Reg<anti_tamper__tamperflag::AntiTamper_TamperflagSpec>;
#[doc = "The 1-bit TAMPERFLAG register directly controls the TAMPER_FLAG output."]
pub mod anti_tamper__tamperflag;
#[doc = "anti_tamper__SOFTRESET (r) register accessor: Reading the SOFTLVL4 register returns 1 if any active soft tamper has a level of 4 or above.\n\nYou can [`read`](crate::Reg::read) this register and get [`anti_tamper__softreset::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anti_tamper__softreset`] module"]
#[doc(alias = "anti_tamper__SOFTRESET")]
pub type AntiTamper_Softreset = crate::Reg<anti_tamper__softreset::AntiTamper_SoftresetSpec>;
#[doc = "Reading the SOFTLVL4 register returns 1 if any active soft tamper has a level of 4 or above."]
pub mod anti_tamper__softreset;

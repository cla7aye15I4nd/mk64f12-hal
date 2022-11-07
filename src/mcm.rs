#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
    pub plasc: PLASC,
    #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
    pub plamc: PLAMC,
    #[doc = "0x0c - Control Register"]
    pub cr: CR,
    #[doc = "0x10 - Interrupt Status Register"]
    pub iscr: ISCR,
    #[doc = "0x14 - ETB Counter Control register"]
    pub etbcc: ETBCC,
    #[doc = "0x18 - ETB Reload register"]
    pub etbrl: ETBRL,
    #[doc = "0x1c - ETB Counter Value register"]
    pub etbcnt: ETBCNT,
    _reserved7: [u8; 0x10],
    #[doc = "0x30 - Process ID register"]
    pub pid: PID,
}
#[doc = "PLASC (r) register accessor: an alias for `Reg<PLASC_SPEC>`"]
pub type PLASC = crate::Reg<plasc::PLASC_SPEC>;
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "PLAMC (r) register accessor: an alias for `Reg<PLAMC_SPEC>`"]
pub type PLAMC = crate::Reg<plamc::PLAMC_SPEC>;
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "ISCR (rw) register accessor: an alias for `Reg<ISCR_SPEC>`"]
pub type ISCR = crate::Reg<iscr::ISCR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod iscr;
#[doc = "ETBCC (rw) register accessor: an alias for `Reg<ETBCC_SPEC>`"]
pub type ETBCC = crate::Reg<etbcc::ETBCC_SPEC>;
#[doc = "ETB Counter Control register"]
pub mod etbcc;
#[doc = "ETBRL (rw) register accessor: an alias for `Reg<ETBRL_SPEC>`"]
pub type ETBRL = crate::Reg<etbrl::ETBRL_SPEC>;
#[doc = "ETB Reload register"]
pub mod etbrl;
#[doc = "ETBCNT (r) register accessor: an alias for `Reg<ETBCNT_SPEC>`"]
pub type ETBCNT = crate::Reg<etbcnt::ETBCNT_SPEC>;
#[doc = "ETB Counter Value register"]
pub mod etbcnt;
#[doc = "PID (rw) register accessor: an alias for `Reg<PID_SPEC>`"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Process ID register"]
pub mod pid;

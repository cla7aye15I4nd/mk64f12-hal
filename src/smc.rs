#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Mode Protection register"]
    pub pmprot: PMPROT,
    #[doc = "0x01 - Power Mode Control register"]
    pub pmctrl: PMCTRL,
    #[doc = "0x02 - VLLS Control register"]
    pub vllsctrl: VLLSCTRL,
    #[doc = "0x03 - Power Mode Status register"]
    pub pmstat: PMSTAT,
}
#[doc = "PMPROT (rw) register accessor: an alias for `Reg<PMPROT_SPEC>`"]
pub type PMPROT = crate::Reg<pmprot::PMPROT_SPEC>;
#[doc = "Power Mode Protection register"]
pub mod pmprot;
#[doc = "PMCTRL (rw) register accessor: an alias for `Reg<PMCTRL_SPEC>`"]
pub type PMCTRL = crate::Reg<pmctrl::PMCTRL_SPEC>;
#[doc = "Power Mode Control register"]
pub mod pmctrl;
#[doc = "VLLSCTRL (rw) register accessor: an alias for `Reg<VLLSCTRL_SPEC>`"]
pub type VLLSCTRL = crate::Reg<vllsctrl::VLLSCTRL_SPEC>;
#[doc = "VLLS Control register"]
pub mod vllsctrl;
#[doc = "PMSTAT (r) register accessor: an alias for `Reg<PMSTAT_SPEC>`"]
pub type PMSTAT = crate::Reg<pmstat::PMSTAT_SPEC>;
#[doc = "Power Mode Status register"]
pub mod pmstat;

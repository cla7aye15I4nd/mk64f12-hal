#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Service Register"]
    pub serv: SERV,
    #[doc = "0x02 - Compare Low Register"]
    pub cmpl: CMPL,
    #[doc = "0x03 - Compare High Register"]
    pub cmph: CMPH,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "SERV (w) register accessor: an alias for `Reg<SERV_SPEC>`"]
pub type SERV = crate::Reg<serv::SERV_SPEC>;
#[doc = "Service Register"]
pub mod serv;
#[doc = "CMPL (rw) register accessor: an alias for `Reg<CMPL_SPEC>`"]
pub type CMPL = crate::Reg<cmpl::CMPL_SPEC>;
#[doc = "Compare Low Register"]
pub mod cmpl;
#[doc = "CMPH (rw) register accessor: an alias for `Reg<CMPH_SPEC>`"]
pub type CMPH = crate::Reg<cmph::CMPH_SPEC>;
#[doc = "Compare High Register"]
pub mod cmph;

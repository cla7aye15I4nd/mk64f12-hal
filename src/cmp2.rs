#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP Control Register 0"]
    pub cr0: CR0,
    #[doc = "0x01 - CMP Control Register 1"]
    pub cr1: CR1,
    #[doc = "0x02 - CMP Filter Period Register"]
    pub fpr: FPR,
    #[doc = "0x03 - CMP Status and Control Register"]
    pub scr: SCR,
    #[doc = "0x04 - DAC Control Register"]
    pub daccr: DACCR,
    #[doc = "0x05 - MUX Control Register"]
    pub muxcr: MUXCR,
}
#[doc = "CR0 (rw) register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "CMP Control Register 0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "CMP Control Register 1"]
pub mod cr1;
#[doc = "FPR (rw) register accessor: an alias for `Reg<FPR_SPEC>`"]
pub type FPR = crate::Reg<fpr::FPR_SPEC>;
#[doc = "CMP Filter Period Register"]
pub mod fpr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "CMP Status and Control Register"]
pub mod scr;
#[doc = "DACCR (rw) register accessor: an alias for `Reg<DACCR_SPEC>`"]
pub type DACCR = crate::Reg<daccr::DACCR_SPEC>;
#[doc = "DAC Control Register"]
pub mod daccr;
#[doc = "MUXCR (rw) register accessor: an alias for `Reg<MUXCR_SPEC>`"]
pub type MUXCR = crate::Reg<muxcr::MUXCR_SPEC>;
#[doc = "MUX Control Register"]
pub mod muxcr;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Voltage Detect Status And Control 1 register"]
    pub lvdsc1: LVDSC1,
    #[doc = "0x01 - Low Voltage Detect Status And Control 2 register"]
    pub lvdsc2: LVDSC2,
    #[doc = "0x02 - Regulator Status And Control register"]
    pub regsc: REGSC,
}
#[doc = "LVDSC1 (rw) register accessor: an alias for `Reg<LVDSC1_SPEC>`"]
pub type LVDSC1 = crate::Reg<lvdsc1::LVDSC1_SPEC>;
#[doc = "Low Voltage Detect Status And Control 1 register"]
pub mod lvdsc1;
#[doc = "LVDSC2 (rw) register accessor: an alias for `Reg<LVDSC2_SPEC>`"]
pub type LVDSC2 = crate::Reg<lvdsc2::LVDSC2_SPEC>;
#[doc = "Low Voltage Detect Status And Control 2 register"]
pub mod lvdsc2;
#[doc = "REGSC (rw) register accessor: an alias for `Reg<REGSC_SPEC>`"]
pub type REGSC = crate::Reg<regsc::REGSC_SPEC>;
#[doc = "Regulator Status And Control register"]
pub mod regsc;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VREF Trim Register"]
    pub trm: TRM,
    #[doc = "0x01 - VREF Status and Control Register"]
    pub sc: SC,
}
#[doc = "TRM (rw) register accessor: an alias for `Reg<TRM_SPEC>`"]
pub type TRM = crate::Reg<trm::TRM_SPEC>;
#[doc = "VREF Trim Register"]
pub mod trm;
#[doc = "SC (rw) register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "VREF Status and Control Register"]
pub mod sc;

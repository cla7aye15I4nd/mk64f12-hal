#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNGA Control Register"]
    pub cr: CR,
    #[doc = "0x04 - RNGA Status Register"]
    pub sr: SR,
    #[doc = "0x08 - RNGA Entropy Register"]
    pub er: ER,
    #[doc = "0x0c - RNGA Output Register"]
    pub or: OR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "RNGA Control Register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "RNGA Status Register"]
pub mod sr;
#[doc = "ER (w) register accessor: an alias for `Reg<ER_SPEC>`"]
pub type ER = crate::Reg<er::ER_SPEC>;
#[doc = "RNGA Entropy Register"]
pub mod er;
#[doc = "OR (r) register accessor: an alias for `Reg<OR_SPEC>`"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "RNGA Output Register"]
pub mod or;

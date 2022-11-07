#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Data Output Register"]
    pub pdor: PDOR,
    #[doc = "0x04 - Port Set Output Register"]
    pub psor: PSOR,
    #[doc = "0x08 - Port Clear Output Register"]
    pub pcor: PCOR,
    #[doc = "0x0c - Port Toggle Output Register"]
    pub ptor: PTOR,
    #[doc = "0x10 - Port Data Input Register"]
    pub pdir: PDIR,
    #[doc = "0x14 - Port Data Direction Register"]
    pub pddr: PDDR,
}
#[doc = "PDOR (rw) register accessor: an alias for `Reg<PDOR_SPEC>`"]
pub type PDOR = crate::Reg<pdor::PDOR_SPEC>;
#[doc = "Port Data Output Register"]
pub mod pdor;
#[doc = "PSOR (w) register accessor: an alias for `Reg<PSOR_SPEC>`"]
pub type PSOR = crate::Reg<psor::PSOR_SPEC>;
#[doc = "Port Set Output Register"]
pub mod psor;
#[doc = "PCOR (w) register accessor: an alias for `Reg<PCOR_SPEC>`"]
pub type PCOR = crate::Reg<pcor::PCOR_SPEC>;
#[doc = "Port Clear Output Register"]
pub mod pcor;
#[doc = "PTOR (w) register accessor: an alias for `Reg<PTOR_SPEC>`"]
pub type PTOR = crate::Reg<ptor::PTOR_SPEC>;
#[doc = "Port Toggle Output Register"]
pub mod ptor;
#[doc = "PDIR (r) register accessor: an alias for `Reg<PDIR_SPEC>`"]
pub type PDIR = crate::Reg<pdir::PDIR_SPEC>;
#[doc = "Port Data Input Register"]
pub mod pdir;
#[doc = "PDDR (rw) register accessor: an alias for `Reg<PDDR_SPEC>`"]
pub type PDDR = crate::Reg<pddr::PDDR_SPEC>;
#[doc = "Port Data Direction Register"]
pub mod pddr;

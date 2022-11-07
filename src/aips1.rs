#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Privilege Register A"]
    pub mpra: MPRA,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - Peripheral Access Control Register"]
    pub pacra: PACRA,
    #[doc = "0x24 - Peripheral Access Control Register"]
    pub pacrb: PACRB,
    #[doc = "0x28 - Peripheral Access Control Register"]
    pub pacrc: PACRC,
    #[doc = "0x2c - Peripheral Access Control Register"]
    pub pacrd: PACRD,
    _reserved5: [u8; 0x10],
    #[doc = "0x40 - Peripheral Access Control Register"]
    pub pacre: PACRE,
    #[doc = "0x44 - Peripheral Access Control Register"]
    pub pacrf: PACRF,
    #[doc = "0x48 - Peripheral Access Control Register"]
    pub pacrg: PACRG,
    #[doc = "0x4c - Peripheral Access Control Register"]
    pub pacrh: PACRH,
    #[doc = "0x50 - Peripheral Access Control Register"]
    pub pacri: PACRI,
    #[doc = "0x54 - Peripheral Access Control Register"]
    pub pacrj: PACRJ,
    #[doc = "0x58 - Peripheral Access Control Register"]
    pub pacrk: PACRK,
    #[doc = "0x5c - Peripheral Access Control Register"]
    pub pacrl: PACRL,
    #[doc = "0x60 - Peripheral Access Control Register"]
    pub pacrm: PACRM,
    #[doc = "0x64 - Peripheral Access Control Register"]
    pub pacrn: PACRN,
    #[doc = "0x68 - Peripheral Access Control Register"]
    pub pacro: PACRO,
    #[doc = "0x6c - Peripheral Access Control Register"]
    pub pacrp: PACRP,
    _reserved17: [u8; 0x10],
    #[doc = "0x80 - Peripheral Access Control Register"]
    pub pacru: PACRU,
}
#[doc = "MPRA (rw) register accessor: an alias for `Reg<MPRA_SPEC>`"]
pub type MPRA = crate::Reg<mpra::MPRA_SPEC>;
#[doc = "Master Privilege Register A"]
pub mod mpra;
#[doc = "PACRA (rw) register accessor: an alias for `Reg<PACRA_SPEC>`"]
pub type PACRA = crate::Reg<pacra::PACRA_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacra;
#[doc = "PACRB (rw) register accessor: an alias for `Reg<PACRB_SPEC>`"]
pub type PACRB = crate::Reg<pacrb::PACRB_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrb;
#[doc = "PACRC (rw) register accessor: an alias for `Reg<PACRC_SPEC>`"]
pub type PACRC = crate::Reg<pacrc::PACRC_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrc;
#[doc = "PACRD (rw) register accessor: an alias for `Reg<PACRD_SPEC>`"]
pub type PACRD = crate::Reg<pacrd::PACRD_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrd;
#[doc = "PACRE (rw) register accessor: an alias for `Reg<PACRE_SPEC>`"]
pub type PACRE = crate::Reg<pacre::PACRE_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacre;
#[doc = "PACRF (rw) register accessor: an alias for `Reg<PACRF_SPEC>`"]
pub type PACRF = crate::Reg<pacrf::PACRF_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrf;
#[doc = "PACRG (rw) register accessor: an alias for `Reg<PACRG_SPEC>`"]
pub type PACRG = crate::Reg<pacrg::PACRG_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrg;
#[doc = "PACRH (rw) register accessor: an alias for `Reg<PACRH_SPEC>`"]
pub type PACRH = crate::Reg<pacrh::PACRH_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrh;
#[doc = "PACRI (rw) register accessor: an alias for `Reg<PACRI_SPEC>`"]
pub type PACRI = crate::Reg<pacri::PACRI_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacri;
#[doc = "PACRJ (rw) register accessor: an alias for `Reg<PACRJ_SPEC>`"]
pub type PACRJ = crate::Reg<pacrj::PACRJ_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrj;
#[doc = "PACRK (rw) register accessor: an alias for `Reg<PACRK_SPEC>`"]
pub type PACRK = crate::Reg<pacrk::PACRK_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrk;
#[doc = "PACRL (rw) register accessor: an alias for `Reg<PACRL_SPEC>`"]
pub type PACRL = crate::Reg<pacrl::PACRL_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrl;
#[doc = "PACRM (rw) register accessor: an alias for `Reg<PACRM_SPEC>`"]
pub type PACRM = crate::Reg<pacrm::PACRM_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrm;
#[doc = "PACRN (rw) register accessor: an alias for `Reg<PACRN_SPEC>`"]
pub type PACRN = crate::Reg<pacrn::PACRN_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrn;
#[doc = "PACRO (rw) register accessor: an alias for `Reg<PACRO_SPEC>`"]
pub type PACRO = crate::Reg<pacro::PACRO_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacro;
#[doc = "PACRP (rw) register accessor: an alias for `Reg<PACRP_SPEC>`"]
pub type PACRP = crate::Reg<pacrp::PACRP_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrp;
#[doc = "PACRU (rw) register accessor: an alias for `Reg<PACRU_SPEC>`"]
pub type PACRU = crate::Reg<pacru::PACRU_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacru;

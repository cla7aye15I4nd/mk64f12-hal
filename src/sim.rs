#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Options Register 1"]
    pub sopt1: SOPT1,
    #[doc = "0x04 - SOPT1 Configuration Register"]
    pub sopt1cfg: SOPT1CFG,
    _reserved2: [u8; 0x0ffc],
    #[doc = "0x1004 - System Options Register 2"]
    pub sopt2: SOPT2,
    _reserved3: [u8; 0x04],
    #[doc = "0x100c - System Options Register 4"]
    pub sopt4: SOPT4,
    #[doc = "0x1010 - System Options Register 5"]
    pub sopt5: SOPT5,
    _reserved5: [u8; 0x04],
    #[doc = "0x1018 - System Options Register 7"]
    pub sopt7: SOPT7,
    _reserved6: [u8; 0x08],
    #[doc = "0x1024 - System Device Identification Register"]
    pub sdid: SDID,
    #[doc = "0x1028 - System Clock Gating Control Register 1"]
    pub scgc1: SCGC1,
    #[doc = "0x102c - System Clock Gating Control Register 2"]
    pub scgc2: SCGC2,
    #[doc = "0x1030 - System Clock Gating Control Register 3"]
    pub scgc3: SCGC3,
    #[doc = "0x1034 - System Clock Gating Control Register 4"]
    pub scgc4: SCGC4,
    #[doc = "0x1038 - System Clock Gating Control Register 5"]
    pub scgc5: SCGC5,
    #[doc = "0x103c - System Clock Gating Control Register 6"]
    pub scgc6: SCGC6,
    #[doc = "0x1040 - System Clock Gating Control Register 7"]
    pub scgc7: SCGC7,
    #[doc = "0x1044 - System Clock Divider Register 1"]
    pub clkdiv1: CLKDIV1,
    #[doc = "0x1048 - System Clock Divider Register 2"]
    pub clkdiv2: CLKDIV2,
    #[doc = "0x104c - Flash Configuration Register 1"]
    pub fcfg1: FCFG1,
    #[doc = "0x1050 - Flash Configuration Register 2"]
    pub fcfg2: FCFG2,
    #[doc = "0x1054 - Unique Identification Register High"]
    pub uidh: UIDH,
    #[doc = "0x1058 - Unique Identification Register Mid-High"]
    pub uidmh: UIDMH,
    #[doc = "0x105c - Unique Identification Register Mid Low"]
    pub uidml: UIDML,
    #[doc = "0x1060 - Unique Identification Register Low"]
    pub uidl: UIDL,
}
#[doc = "SOPT1 (rw) register accessor: an alias for `Reg<SOPT1_SPEC>`"]
pub type SOPT1 = crate::Reg<sopt1::SOPT1_SPEC>;
#[doc = "System Options Register 1"]
pub mod sopt1;
#[doc = "SOPT1CFG (rw) register accessor: an alias for `Reg<SOPT1CFG_SPEC>`"]
pub type SOPT1CFG = crate::Reg<sopt1cfg::SOPT1CFG_SPEC>;
#[doc = "SOPT1 Configuration Register"]
pub mod sopt1cfg;
#[doc = "SOPT2 (rw) register accessor: an alias for `Reg<SOPT2_SPEC>`"]
pub type SOPT2 = crate::Reg<sopt2::SOPT2_SPEC>;
#[doc = "System Options Register 2"]
pub mod sopt2;
#[doc = "SOPT4 (rw) register accessor: an alias for `Reg<SOPT4_SPEC>`"]
pub type SOPT4 = crate::Reg<sopt4::SOPT4_SPEC>;
#[doc = "System Options Register 4"]
pub mod sopt4;
#[doc = "SOPT5 (rw) register accessor: an alias for `Reg<SOPT5_SPEC>`"]
pub type SOPT5 = crate::Reg<sopt5::SOPT5_SPEC>;
#[doc = "System Options Register 5"]
pub mod sopt5;
#[doc = "SOPT7 (rw) register accessor: an alias for `Reg<SOPT7_SPEC>`"]
pub type SOPT7 = crate::Reg<sopt7::SOPT7_SPEC>;
#[doc = "System Options Register 7"]
pub mod sopt7;
#[doc = "SDID (r) register accessor: an alias for `Reg<SDID_SPEC>`"]
pub type SDID = crate::Reg<sdid::SDID_SPEC>;
#[doc = "System Device Identification Register"]
pub mod sdid;
#[doc = "SCGC1 (rw) register accessor: an alias for `Reg<SCGC1_SPEC>`"]
pub type SCGC1 = crate::Reg<scgc1::SCGC1_SPEC>;
#[doc = "System Clock Gating Control Register 1"]
pub mod scgc1;
#[doc = "SCGC2 (rw) register accessor: an alias for `Reg<SCGC2_SPEC>`"]
pub type SCGC2 = crate::Reg<scgc2::SCGC2_SPEC>;
#[doc = "System Clock Gating Control Register 2"]
pub mod scgc2;
#[doc = "SCGC3 (rw) register accessor: an alias for `Reg<SCGC3_SPEC>`"]
pub type SCGC3 = crate::Reg<scgc3::SCGC3_SPEC>;
#[doc = "System Clock Gating Control Register 3"]
pub mod scgc3;
#[doc = "SCGC4 (rw) register accessor: an alias for `Reg<SCGC4_SPEC>`"]
pub type SCGC4 = crate::Reg<scgc4::SCGC4_SPEC>;
#[doc = "System Clock Gating Control Register 4"]
pub mod scgc4;
#[doc = "SCGC5 (rw) register accessor: an alias for `Reg<SCGC5_SPEC>`"]
pub type SCGC5 = crate::Reg<scgc5::SCGC5_SPEC>;
#[doc = "System Clock Gating Control Register 5"]
pub mod scgc5;
#[doc = "SCGC6 (rw) register accessor: an alias for `Reg<SCGC6_SPEC>`"]
pub type SCGC6 = crate::Reg<scgc6::SCGC6_SPEC>;
#[doc = "System Clock Gating Control Register 6"]
pub mod scgc6;
#[doc = "SCGC7 (rw) register accessor: an alias for `Reg<SCGC7_SPEC>`"]
pub type SCGC7 = crate::Reg<scgc7::SCGC7_SPEC>;
#[doc = "System Clock Gating Control Register 7"]
pub mod scgc7;
#[doc = "CLKDIV1 (rw) register accessor: an alias for `Reg<CLKDIV1_SPEC>`"]
pub type CLKDIV1 = crate::Reg<clkdiv1::CLKDIV1_SPEC>;
#[doc = "System Clock Divider Register 1"]
pub mod clkdiv1;
#[doc = "CLKDIV2 (rw) register accessor: an alias for `Reg<CLKDIV2_SPEC>`"]
pub type CLKDIV2 = crate::Reg<clkdiv2::CLKDIV2_SPEC>;
#[doc = "System Clock Divider Register 2"]
pub mod clkdiv2;
#[doc = "FCFG1 (rw) register accessor: an alias for `Reg<FCFG1_SPEC>`"]
pub type FCFG1 = crate::Reg<fcfg1::FCFG1_SPEC>;
#[doc = "Flash Configuration Register 1"]
pub mod fcfg1;
#[doc = "FCFG2 (r) register accessor: an alias for `Reg<FCFG2_SPEC>`"]
pub type FCFG2 = crate::Reg<fcfg2::FCFG2_SPEC>;
#[doc = "Flash Configuration Register 2"]
pub mod fcfg2;
#[doc = "UIDH (r) register accessor: an alias for `Reg<UIDH_SPEC>`"]
pub type UIDH = crate::Reg<uidh::UIDH_SPEC>;
#[doc = "Unique Identification Register High"]
pub mod uidh;
#[doc = "UIDMH (r) register accessor: an alias for `Reg<UIDMH_SPEC>`"]
pub type UIDMH = crate::Reg<uidmh::UIDMH_SPEC>;
#[doc = "Unique Identification Register Mid-High"]
pub mod uidmh;
#[doc = "UIDML (r) register accessor: an alias for `Reg<UIDML_SPEC>`"]
pub type UIDML = crate::Reg<uidml::UIDML_SPEC>;
#[doc = "Unique Identification Register Mid Low"]
pub mod uidml;
#[doc = "UIDL (r) register accessor: an alias for `Reg<UIDL_SPEC>`"]
pub type UIDL = crate::Reg<uidl::UIDL_SPEC>;
#[doc = "Unique Identification Register Low"]
pub mod uidl;

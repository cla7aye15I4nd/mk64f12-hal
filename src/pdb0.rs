#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control register"]
    pub sc: SC,
    #[doc = "0x04 - Modulus register"]
    pub mod_: MOD,
    #[doc = "0x08 - Counter register"]
    pub cnt: CNT,
    #[doc = "0x0c - Interrupt Delay register"]
    pub idly: IDLY,
    #[doc = "0x10 - Channel n Control register 1"]
    pub ch0c1: CHC1,
    #[doc = "0x14 - Channel n Status register"]
    pub ch0s: CHS,
    #[doc = "0x18 - Channel n Delay 0 register"]
    pub ch0dly0: CHDLY0,
    #[doc = "0x1c - Channel n Delay 1 register"]
    pub ch0dly1: CHDLY1,
    _reserved8: [u8; 0x18],
    #[doc = "0x38 - Channel n Control register 1"]
    pub ch1c1: CHC1,
    #[doc = "0x3c - Channel n Status register"]
    pub ch1s: CHS,
    #[doc = "0x40 - Channel n Delay 0 register"]
    pub ch1dly0: CHDLY0,
    #[doc = "0x44 - Channel n Delay 1 register"]
    pub ch1dly1: CHDLY1,
    _reserved12: [u8; 0x0108],
    #[doc = "0x150 - DAC Interval Trigger n Control register"]
    pub dacintc0: DACINTC,
    #[doc = "0x154 - DAC Interval n register"]
    pub dacint0: DACINT,
    #[doc = "0x158 - DAC Interval Trigger n Control register"]
    pub dacintc1: DACINTC,
    #[doc = "0x15c - DAC Interval n register"]
    pub dacint1: DACINT,
    _reserved16: [u8; 0x30],
    #[doc = "0x190 - Pulse-Out n Enable register"]
    pub poen: POEN,
    #[doc = "0x194..0x1a0 - Pulse-Out n Delay register"]
    pub podly: [PODLY; 3],
}
#[doc = "SC (rw) register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "Status and Control register"]
pub mod sc;
#[doc = "MOD (rw) register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Modulus register"]
pub mod mod_;
#[doc = "CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter register"]
pub mod cnt;
#[doc = "IDLY (rw) register accessor: an alias for `Reg<IDLY_SPEC>`"]
pub type IDLY = crate::Reg<idly::IDLY_SPEC>;
#[doc = "Interrupt Delay register"]
pub mod idly;
#[doc = "CHC1 (rw) register accessor: an alias for `Reg<CHC1_SPEC>`"]
pub type CHC1 = crate::Reg<chc1::CHC1_SPEC>;
#[doc = "Channel n Control register 1"]
pub mod chc1;
#[doc = "CHS (rw) register accessor: an alias for `Reg<CHS_SPEC>`"]
pub type CHS = crate::Reg<chs::CHS_SPEC>;
#[doc = "Channel n Status register"]
pub mod chs;
#[doc = "CHDLY0 (rw) register accessor: an alias for `Reg<CHDLY0_SPEC>`"]
pub type CHDLY0 = crate::Reg<chdly0::CHDLY0_SPEC>;
#[doc = "Channel n Delay 0 register"]
pub mod chdly0;
#[doc = "CHDLY1 (rw) register accessor: an alias for `Reg<CHDLY1_SPEC>`"]
pub type CHDLY1 = crate::Reg<chdly1::CHDLY1_SPEC>;
#[doc = "Channel n Delay 1 register"]
pub mod chdly1;
#[doc = "DACINTC (rw) register accessor: an alias for `Reg<DACINTC_SPEC>`"]
pub type DACINTC = crate::Reg<dacintc::DACINTC_SPEC>;
#[doc = "DAC Interval Trigger n Control register"]
pub mod dacintc;
#[doc = "DACINT (rw) register accessor: an alias for `Reg<DACINT_SPEC>`"]
pub type DACINT = crate::Reg<dacint::DACINT_SPEC>;
#[doc = "DAC Interval n register"]
pub mod dacint;
#[doc = "POEN (rw) register accessor: an alias for `Reg<POEN_SPEC>`"]
pub type POEN = crate::Reg<poen::POEN_SPEC>;
#[doc = "Pulse-Out n Enable register"]
pub mod poen;
#[doc = "PODLY (rw) register accessor: an alias for `Reg<PODLY_SPEC>`"]
pub type PODLY = crate::Reg<podly::PODLY_SPEC>;
#[doc = "Pulse-Out n Delay register"]
pub mod podly;

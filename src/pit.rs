#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIT Module Control Register"]
    pub mcr: MCR,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - Timer Load Value Register"]
    pub ldval0: LDVAL,
    #[doc = "0x104 - Current Timer Value Register"]
    pub cval0: CVAL,
    #[doc = "0x108 - Timer Control Register"]
    pub tctrl0: TCTRL,
    #[doc = "0x10c - Timer Flag Register"]
    pub tflg0: TFLG,
    #[doc = "0x110 - Timer Load Value Register"]
    pub ldval1: LDVAL,
    #[doc = "0x114 - Current Timer Value Register"]
    pub cval1: CVAL,
    #[doc = "0x118 - Timer Control Register"]
    pub tctrl1: TCTRL,
    #[doc = "0x11c - Timer Flag Register"]
    pub tflg1: TFLG,
    #[doc = "0x120 - Timer Load Value Register"]
    pub ldval2: LDVAL,
    #[doc = "0x124 - Current Timer Value Register"]
    pub cval2: CVAL,
    #[doc = "0x128 - Timer Control Register"]
    pub tctrl2: TCTRL,
    #[doc = "0x12c - Timer Flag Register"]
    pub tflg2: TFLG,
    #[doc = "0x130 - Timer Load Value Register"]
    pub ldval3: LDVAL,
    #[doc = "0x134 - Current Timer Value Register"]
    pub cval3: CVAL,
    #[doc = "0x138 - Timer Control Register"]
    pub tctrl3: TCTRL,
    #[doc = "0x13c - Timer Flag Register"]
    pub tflg3: TFLG,
}
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "PIT Module Control Register"]
pub mod mcr;
#[doc = "LDVAL (rw) register accessor: an alias for `Reg<LDVAL_SPEC>`"]
pub type LDVAL = crate::Reg<ldval::LDVAL_SPEC>;
#[doc = "Timer Load Value Register"]
pub mod ldval;
#[doc = "CVAL (r) register accessor: an alias for `Reg<CVAL_SPEC>`"]
pub type CVAL = crate::Reg<cval::CVAL_SPEC>;
#[doc = "Current Timer Value Register"]
pub mod cval;
#[doc = "TCTRL (rw) register accessor: an alias for `Reg<TCTRL_SPEC>`"]
pub type TCTRL = crate::Reg<tctrl::TCTRL_SPEC>;
#[doc = "Timer Control Register"]
pub mod tctrl;
#[doc = "TFLG (rw) register accessor: an alias for `Reg<TFLG_SPEC>`"]
pub type TFLG = crate::Reg<tflg::TFLG_SPEC>;
#[doc = "Timer Flag Register"]
pub mod tflg;

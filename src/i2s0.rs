#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAI Transmit Control Register"]
    pub tcsr: TCSR,
    #[doc = "0x04 - SAI Transmit Configuration 1 Register"]
    pub tcr1: TCR1,
    #[doc = "0x08 - SAI Transmit Configuration 2 Register"]
    pub tcr2: TCR2,
    #[doc = "0x0c - SAI Transmit Configuration 3 Register"]
    pub tcr3: TCR3,
    #[doc = "0x10 - SAI Transmit Configuration 4 Register"]
    pub tcr4: TCR4,
    #[doc = "0x14 - SAI Transmit Configuration 5 Register"]
    pub tcr5: TCR5,
    _reserved6: [u8; 0x08],
    #[doc = "0x20..0x28 - SAI Transmit Data Register"]
    pub tdr: [TDR; 2],
    _reserved7: [u8; 0x18],
    #[doc = "0x40..0x48 - SAI Transmit FIFO Register"]
    pub tfr: [TFR; 2],
    _reserved8: [u8; 0x18],
    #[doc = "0x60 - SAI Transmit Mask Register"]
    pub tmr: TMR,
    _reserved9: [u8; 0x1c],
    #[doc = "0x80 - SAI Receive Control Register"]
    pub rcsr: RCSR,
    #[doc = "0x84 - SAI Receive Configuration 1 Register"]
    pub rcr1: RCR1,
    #[doc = "0x88 - SAI Receive Configuration 2 Register"]
    pub rcr2: RCR2,
    #[doc = "0x8c - SAI Receive Configuration 3 Register"]
    pub rcr3: RCR3,
    #[doc = "0x90 - SAI Receive Configuration 4 Register"]
    pub rcr4: RCR4,
    #[doc = "0x94 - SAI Receive Configuration 5 Register"]
    pub rcr5: RCR5,
    _reserved15: [u8; 0x08],
    #[doc = "0xa0..0xa8 - SAI Receive Data Register"]
    pub rdr: [RDR; 2],
    _reserved16: [u8; 0x18],
    #[doc = "0xc0..0xc8 - SAI Receive FIFO Register"]
    pub rfr: [RFR; 2],
    _reserved17: [u8; 0x18],
    #[doc = "0xe0 - SAI Receive Mask Register"]
    pub rmr: RMR,
    _reserved18: [u8; 0x1c],
    #[doc = "0x100 - SAI MCLK Control Register"]
    pub mcr: MCR,
    #[doc = "0x104 - SAI MCLK Divide Register"]
    pub mdr: MDR,
}
#[doc = "TCSR (rw) register accessor: an alias for `Reg<TCSR_SPEC>`"]
pub type TCSR = crate::Reg<tcsr::TCSR_SPEC>;
#[doc = "SAI Transmit Control Register"]
pub mod tcsr;
#[doc = "TCR1 (rw) register accessor: an alias for `Reg<TCR1_SPEC>`"]
pub type TCR1 = crate::Reg<tcr1::TCR1_SPEC>;
#[doc = "SAI Transmit Configuration 1 Register"]
pub mod tcr1;
#[doc = "TCR2 (rw) register accessor: an alias for `Reg<TCR2_SPEC>`"]
pub type TCR2 = crate::Reg<tcr2::TCR2_SPEC>;
#[doc = "SAI Transmit Configuration 2 Register"]
pub mod tcr2;
#[doc = "TCR3 (rw) register accessor: an alias for `Reg<TCR3_SPEC>`"]
pub type TCR3 = crate::Reg<tcr3::TCR3_SPEC>;
#[doc = "SAI Transmit Configuration 3 Register"]
pub mod tcr3;
#[doc = "TCR4 (rw) register accessor: an alias for `Reg<TCR4_SPEC>`"]
pub type TCR4 = crate::Reg<tcr4::TCR4_SPEC>;
#[doc = "SAI Transmit Configuration 4 Register"]
pub mod tcr4;
#[doc = "TCR5 (rw) register accessor: an alias for `Reg<TCR5_SPEC>`"]
pub type TCR5 = crate::Reg<tcr5::TCR5_SPEC>;
#[doc = "SAI Transmit Configuration 5 Register"]
pub mod tcr5;
#[doc = "TDR (w) register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "SAI Transmit Data Register"]
pub mod tdr;
#[doc = "TFR (r) register accessor: an alias for `Reg<TFR_SPEC>`"]
pub type TFR = crate::Reg<tfr::TFR_SPEC>;
#[doc = "SAI Transmit FIFO Register"]
pub mod tfr;
#[doc = "TMR (rw) register accessor: an alias for `Reg<TMR_SPEC>`"]
pub type TMR = crate::Reg<tmr::TMR_SPEC>;
#[doc = "SAI Transmit Mask Register"]
pub mod tmr;
#[doc = "RCSR (rw) register accessor: an alias for `Reg<RCSR_SPEC>`"]
pub type RCSR = crate::Reg<rcsr::RCSR_SPEC>;
#[doc = "SAI Receive Control Register"]
pub mod rcsr;
#[doc = "RCR1 (rw) register accessor: an alias for `Reg<RCR1_SPEC>`"]
pub type RCR1 = crate::Reg<rcr1::RCR1_SPEC>;
#[doc = "SAI Receive Configuration 1 Register"]
pub mod rcr1;
#[doc = "RCR2 (rw) register accessor: an alias for `Reg<RCR2_SPEC>`"]
pub type RCR2 = crate::Reg<rcr2::RCR2_SPEC>;
#[doc = "SAI Receive Configuration 2 Register"]
pub mod rcr2;
#[doc = "RCR3 (rw) register accessor: an alias for `Reg<RCR3_SPEC>`"]
pub type RCR3 = crate::Reg<rcr3::RCR3_SPEC>;
#[doc = "SAI Receive Configuration 3 Register"]
pub mod rcr3;
#[doc = "RCR4 (rw) register accessor: an alias for `Reg<RCR4_SPEC>`"]
pub type RCR4 = crate::Reg<rcr4::RCR4_SPEC>;
#[doc = "SAI Receive Configuration 4 Register"]
pub mod rcr4;
#[doc = "RCR5 (rw) register accessor: an alias for `Reg<RCR5_SPEC>`"]
pub type RCR5 = crate::Reg<rcr5::RCR5_SPEC>;
#[doc = "SAI Receive Configuration 5 Register"]
pub mod rcr5;
#[doc = "RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "SAI Receive Data Register"]
pub mod rdr;
#[doc = "RFR (r) register accessor: an alias for `Reg<RFR_SPEC>`"]
pub type RFR = crate::Reg<rfr::RFR_SPEC>;
#[doc = "SAI Receive FIFO Register"]
pub mod rfr;
#[doc = "RMR (rw) register accessor: an alias for `Reg<RMR_SPEC>`"]
pub type RMR = crate::Reg<rmr::RMR_SPEC>;
#[doc = "SAI Receive Mask Register"]
pub mod rmr;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "SAI MCLK Control Register"]
pub mod mcr;
#[doc = "MDR (rw) register accessor: an alias for `Reg<MDR_SPEC>`"]
pub type MDR = crate::Reg<mdr::MDR_SPEC>;
#[doc = "SAI MCLK Divide Register"]
pub mod mdr;

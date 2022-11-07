#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LLWU Pin Enable 1 register"]
    pub pe1: PE1,
    #[doc = "0x01 - LLWU Pin Enable 2 register"]
    pub pe2: PE2,
    #[doc = "0x02 - LLWU Pin Enable 3 register"]
    pub pe3: PE3,
    #[doc = "0x03 - LLWU Pin Enable 4 register"]
    pub pe4: PE4,
    #[doc = "0x04 - LLWU Module Enable register"]
    pub me: ME,
    #[doc = "0x05 - LLWU Flag 1 register"]
    pub f1: F1,
    #[doc = "0x06 - LLWU Flag 2 register"]
    pub f2: F2,
    #[doc = "0x07 - LLWU Flag 3 register"]
    pub f3: F3,
    #[doc = "0x08 - LLWU Pin Filter 1 register"]
    pub filt1: FILT1,
    #[doc = "0x09 - LLWU Pin Filter 2 register"]
    pub filt2: FILT2,
    #[doc = "0x0a - LLWU Reset Enable register"]
    pub rst: RST,
}
#[doc = "PE1 (rw) register accessor: an alias for `Reg<PE1_SPEC>`"]
pub type PE1 = crate::Reg<pe1::PE1_SPEC>;
#[doc = "LLWU Pin Enable 1 register"]
pub mod pe1;
#[doc = "PE2 (rw) register accessor: an alias for `Reg<PE2_SPEC>`"]
pub type PE2 = crate::Reg<pe2::PE2_SPEC>;
#[doc = "LLWU Pin Enable 2 register"]
pub mod pe2;
#[doc = "PE3 (rw) register accessor: an alias for `Reg<PE3_SPEC>`"]
pub type PE3 = crate::Reg<pe3::PE3_SPEC>;
#[doc = "LLWU Pin Enable 3 register"]
pub mod pe3;
#[doc = "PE4 (rw) register accessor: an alias for `Reg<PE4_SPEC>`"]
pub type PE4 = crate::Reg<pe4::PE4_SPEC>;
#[doc = "LLWU Pin Enable 4 register"]
pub mod pe4;
#[doc = "ME (rw) register accessor: an alias for `Reg<ME_SPEC>`"]
pub type ME = crate::Reg<me::ME_SPEC>;
#[doc = "LLWU Module Enable register"]
pub mod me;
#[doc = "F1 (rw) register accessor: an alias for `Reg<F1_SPEC>`"]
pub type F1 = crate::Reg<f1::F1_SPEC>;
#[doc = "LLWU Flag 1 register"]
pub mod f1;
#[doc = "F2 (rw) register accessor: an alias for `Reg<F2_SPEC>`"]
pub type F2 = crate::Reg<f2::F2_SPEC>;
#[doc = "LLWU Flag 2 register"]
pub mod f2;
#[doc = "F3 (r) register accessor: an alias for `Reg<F3_SPEC>`"]
pub type F3 = crate::Reg<f3::F3_SPEC>;
#[doc = "LLWU Flag 3 register"]
pub mod f3;
#[doc = "FILT1 (rw) register accessor: an alias for `Reg<FILT1_SPEC>`"]
pub type FILT1 = crate::Reg<filt1::FILT1_SPEC>;
#[doc = "LLWU Pin Filter 1 register"]
pub mod filt1;
#[doc = "FILT2 (rw) register accessor: an alias for `Reg<FILT2_SPEC>`"]
pub type FILT2 = crate::Reg<filt2::FILT2_SPEC>;
#[doc = "LLWU Pin Filter 2 register"]
pub mod filt2;
#[doc = "RST (rw) register accessor: an alias for `Reg<RST_SPEC>`"]
pub type RST = crate::Reg<rst::RST_SPEC>;
#[doc = "LLWU Reset Enable register"]
pub mod rst;

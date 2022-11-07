#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Backdoor Comparison Key 3."]
    pub backkey3: BACKKEY3,
    #[doc = "0x01 - Backdoor Comparison Key 2."]
    pub backkey2: BACKKEY2,
    #[doc = "0x02 - Backdoor Comparison Key 1."]
    pub backkey1: BACKKEY1,
    #[doc = "0x03 - Backdoor Comparison Key 0."]
    pub backkey0: BACKKEY0,
    #[doc = "0x04 - Backdoor Comparison Key 7."]
    pub backkey7: BACKKEY7,
    #[doc = "0x05 - Backdoor Comparison Key 6."]
    pub backkey6: BACKKEY6,
    #[doc = "0x06 - Backdoor Comparison Key 5."]
    pub backkey5: BACKKEY5,
    #[doc = "0x07 - Backdoor Comparison Key 4."]
    pub backkey4: BACKKEY4,
    #[doc = "0x08 - Non-volatile P-Flash Protection 1 - Low Register"]
    pub fprot3: FPROT3,
    #[doc = "0x09 - Non-volatile P-Flash Protection 1 - High Register"]
    pub fprot2: FPROT2,
    #[doc = "0x0a - Non-volatile P-Flash Protection 0 - Low Register"]
    pub fprot1: FPROT1,
    #[doc = "0x0b - Non-volatile P-Flash Protection 0 - High Register"]
    pub fprot0: FPROT0,
    #[doc = "0x0c - Non-volatile Flash Security Register"]
    pub fsec: FSEC,
    #[doc = "0x0d - Non-volatile Flash Option Register"]
    pub fopt: FOPT,
    #[doc = "0x0e - Non-volatile EERAM Protection Register"]
    pub feprot: FEPROT,
    #[doc = "0x0f - Non-volatile D-Flash Protection Register"]
    pub fdprot: FDPROT,
}
#[doc = "BACKKEY3 (r) register accessor: an alias for `Reg<BACKKEY3_SPEC>`"]
pub type BACKKEY3 = crate::Reg<backkey3::BACKKEY3_SPEC>;
#[doc = "Backdoor Comparison Key 3."]
pub mod backkey3;
#[doc = "BACKKEY2 (r) register accessor: an alias for `Reg<BACKKEY2_SPEC>`"]
pub type BACKKEY2 = crate::Reg<backkey2::BACKKEY2_SPEC>;
#[doc = "Backdoor Comparison Key 2."]
pub mod backkey2;
#[doc = "BACKKEY1 (r) register accessor: an alias for `Reg<BACKKEY1_SPEC>`"]
pub type BACKKEY1 = crate::Reg<backkey1::BACKKEY1_SPEC>;
#[doc = "Backdoor Comparison Key 1."]
pub mod backkey1;
#[doc = "BACKKEY0 (r) register accessor: an alias for `Reg<BACKKEY0_SPEC>`"]
pub type BACKKEY0 = crate::Reg<backkey0::BACKKEY0_SPEC>;
#[doc = "Backdoor Comparison Key 0."]
pub mod backkey0;
#[doc = "BACKKEY7 (r) register accessor: an alias for `Reg<BACKKEY7_SPEC>`"]
pub type BACKKEY7 = crate::Reg<backkey7::BACKKEY7_SPEC>;
#[doc = "Backdoor Comparison Key 7."]
pub mod backkey7;
#[doc = "BACKKEY6 (r) register accessor: an alias for `Reg<BACKKEY6_SPEC>`"]
pub type BACKKEY6 = crate::Reg<backkey6::BACKKEY6_SPEC>;
#[doc = "Backdoor Comparison Key 6."]
pub mod backkey6;
#[doc = "BACKKEY5 (r) register accessor: an alias for `Reg<BACKKEY5_SPEC>`"]
pub type BACKKEY5 = crate::Reg<backkey5::BACKKEY5_SPEC>;
#[doc = "Backdoor Comparison Key 5."]
pub mod backkey5;
#[doc = "BACKKEY4 (r) register accessor: an alias for `Reg<BACKKEY4_SPEC>`"]
pub type BACKKEY4 = crate::Reg<backkey4::BACKKEY4_SPEC>;
#[doc = "Backdoor Comparison Key 4."]
pub mod backkey4;
#[doc = "FPROT3 (r) register accessor: an alias for `Reg<FPROT3_SPEC>`"]
pub type FPROT3 = crate::Reg<fprot3::FPROT3_SPEC>;
#[doc = "Non-volatile P-Flash Protection 1 - Low Register"]
pub mod fprot3;
#[doc = "FPROT2 (r) register accessor: an alias for `Reg<FPROT2_SPEC>`"]
pub type FPROT2 = crate::Reg<fprot2::FPROT2_SPEC>;
#[doc = "Non-volatile P-Flash Protection 1 - High Register"]
pub mod fprot2;
#[doc = "FPROT1 (r) register accessor: an alias for `Reg<FPROT1_SPEC>`"]
pub type FPROT1 = crate::Reg<fprot1::FPROT1_SPEC>;
#[doc = "Non-volatile P-Flash Protection 0 - Low Register"]
pub mod fprot1;
#[doc = "FPROT0 (r) register accessor: an alias for `Reg<FPROT0_SPEC>`"]
pub type FPROT0 = crate::Reg<fprot0::FPROT0_SPEC>;
#[doc = "Non-volatile P-Flash Protection 0 - High Register"]
pub mod fprot0;
#[doc = "FSEC (r) register accessor: an alias for `Reg<FSEC_SPEC>`"]
pub type FSEC = crate::Reg<fsec::FSEC_SPEC>;
#[doc = "Non-volatile Flash Security Register"]
pub mod fsec;
#[doc = "FOPT (r) register accessor: an alias for `Reg<FOPT_SPEC>`"]
pub type FOPT = crate::Reg<fopt::FOPT_SPEC>;
#[doc = "Non-volatile Flash Option Register"]
pub mod fopt;
#[doc = "FEPROT (r) register accessor: an alias for `Reg<FEPROT_SPEC>`"]
pub type FEPROT = crate::Reg<feprot::FEPROT_SPEC>;
#[doc = "Non-volatile EERAM Protection Register"]
pub mod feprot;
#[doc = "FDPROT (r) register accessor: an alias for `Reg<FDPROT_SPEC>`"]
pub type FDPROT = crate::Reg<fdprot::FDPROT_SPEC>;
#[doc = "Non-volatile D-Flash Protection Register"]
pub mod fdprot;

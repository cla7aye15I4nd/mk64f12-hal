#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCG Control 1 Register"]
    pub c1: C1,
    #[doc = "0x01 - MCG Control 2 Register"]
    pub c2: C2,
    #[doc = "0x02 - MCG Control 3 Register"]
    pub c3: C3,
    #[doc = "0x03 - MCG Control 4 Register"]
    pub c4: C4,
    #[doc = "0x04 - MCG Control 5 Register"]
    pub c5: C5,
    #[doc = "0x05 - MCG Control 6 Register"]
    pub c6: C6,
    #[doc = "0x06 - MCG Status Register"]
    pub s: S,
    _reserved7: [u8; 0x01],
    #[doc = "0x08 - MCG Status and Control Register"]
    pub sc: SC,
    _reserved8: [u8; 0x01],
    #[doc = "0x0a - MCG Auto Trim Compare Value High Register"]
    pub atcvh: ATCVH,
    #[doc = "0x0b - MCG Auto Trim Compare Value Low Register"]
    pub atcvl: ATCVL,
    #[doc = "0x0c - MCG Control 7 Register"]
    pub c7: C7,
    #[doc = "0x0d - MCG Control 8 Register"]
    pub c8: C8,
}
#[doc = "C1 (rw) register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "MCG Control 1 Register"]
pub mod c1;
#[doc = "C2 (rw) register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "MCG Control 2 Register"]
pub mod c2;
#[doc = "C3 (rw) register accessor: an alias for `Reg<C3_SPEC>`"]
pub type C3 = crate::Reg<c3::C3_SPEC>;
#[doc = "MCG Control 3 Register"]
pub mod c3;
#[doc = "C4 (rw) register accessor: an alias for `Reg<C4_SPEC>`"]
pub type C4 = crate::Reg<c4::C4_SPEC>;
#[doc = "MCG Control 4 Register"]
pub mod c4;
#[doc = "C5 (rw) register accessor: an alias for `Reg<C5_SPEC>`"]
pub type C5 = crate::Reg<c5::C5_SPEC>;
#[doc = "MCG Control 5 Register"]
pub mod c5;
#[doc = "C6 (rw) register accessor: an alias for `Reg<C6_SPEC>`"]
pub type C6 = crate::Reg<c6::C6_SPEC>;
#[doc = "MCG Control 6 Register"]
pub mod c6;
#[doc = "S (rw) register accessor: an alias for `Reg<S_SPEC>`"]
pub type S = crate::Reg<s::S_SPEC>;
#[doc = "MCG Status Register"]
pub mod s;
#[doc = "SC (rw) register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "MCG Status and Control Register"]
pub mod sc;
#[doc = "ATCVH (rw) register accessor: an alias for `Reg<ATCVH_SPEC>`"]
pub type ATCVH = crate::Reg<atcvh::ATCVH_SPEC>;
#[doc = "MCG Auto Trim Compare Value High Register"]
pub mod atcvh;
#[doc = "ATCVL (rw) register accessor: an alias for `Reg<ATCVL_SPEC>`"]
pub type ATCVL = crate::Reg<atcvl::ATCVL_SPEC>;
#[doc = "MCG Auto Trim Compare Value Low Register"]
pub mod atcvl;
#[doc = "C7 (rw) register accessor: an alias for `Reg<C7_SPEC>`"]
pub type C7 = crate::Reg<c7::C7_SPEC>;
#[doc = "MCG Control 7 Register"]
pub mod c7;
#[doc = "C8 (rw) register accessor: an alias for `Reg<C8_SPEC>`"]
pub type C8 = crate::Reg<c8::C8_SPEC>;
#[doc = "MCG Control 8 Register"]
pub mod c8;

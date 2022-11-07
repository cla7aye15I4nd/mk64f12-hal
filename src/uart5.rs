#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Baud Rate Registers: High"]
    pub bdh: BDH,
    #[doc = "0x01 - UART Baud Rate Registers: Low"]
    pub bdl: BDL,
    #[doc = "0x02 - UART Control Register 1"]
    pub c1: C1,
    #[doc = "0x03 - UART Control Register 2"]
    pub c2: C2,
    #[doc = "0x04 - UART Status Register 1"]
    pub s1: S1,
    #[doc = "0x05 - UART Status Register 2"]
    pub s2: S2,
    #[doc = "0x06 - UART Control Register 3"]
    pub c3: C3,
    #[doc = "0x07 - UART Data Register"]
    pub d: D,
    #[doc = "0x08 - UART Match Address Registers 1"]
    pub ma1: MA1,
    #[doc = "0x09 - UART Match Address Registers 2"]
    pub ma2: MA2,
    #[doc = "0x0a - UART Control Register 4"]
    pub c4: C4,
    #[doc = "0x0b - UART Control Register 5"]
    pub c5: C5,
    #[doc = "0x0c - UART Extended Data Register"]
    pub ed: ED,
    #[doc = "0x0d - UART Modem Register"]
    pub modem: MODEM,
    #[doc = "0x0e - UART Infrared Register"]
    pub ir: IR,
    _reserved15: [u8; 0x01],
    #[doc = "0x10 - UART FIFO Parameters"]
    pub pfifo: PFIFO,
    #[doc = "0x11 - UART FIFO Control Register"]
    pub cfifo: CFIFO,
    #[doc = "0x12 - UART FIFO Status Register"]
    pub sfifo: SFIFO,
    #[doc = "0x13 - UART FIFO Transmit Watermark"]
    pub twfifo: TWFIFO,
    #[doc = "0x14 - UART FIFO Transmit Count"]
    pub tcfifo: TCFIFO,
    #[doc = "0x15 - UART FIFO Receive Watermark"]
    pub rwfifo: RWFIFO,
    #[doc = "0x16 - UART FIFO Receive Count"]
    pub rcfifo: RCFIFO,
}
#[doc = "BDH (rw) register accessor: an alias for `Reg<BDH_SPEC>`"]
pub type BDH = crate::Reg<bdh::BDH_SPEC>;
#[doc = "UART Baud Rate Registers: High"]
pub mod bdh;
#[doc = "BDL (rw) register accessor: an alias for `Reg<BDL_SPEC>`"]
pub type BDL = crate::Reg<bdl::BDL_SPEC>;
#[doc = "UART Baud Rate Registers: Low"]
pub mod bdl;
#[doc = "C1 (rw) register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "UART Control Register 1"]
pub mod c1;
#[doc = "C2 (rw) register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "UART Control Register 2"]
pub mod c2;
#[doc = "S1 (r) register accessor: an alias for `Reg<S1_SPEC>`"]
pub type S1 = crate::Reg<s1::S1_SPEC>;
#[doc = "UART Status Register 1"]
pub mod s1;
#[doc = "S2 (rw) register accessor: an alias for `Reg<S2_SPEC>`"]
pub type S2 = crate::Reg<s2::S2_SPEC>;
#[doc = "UART Status Register 2"]
pub mod s2;
#[doc = "C3 (rw) register accessor: an alias for `Reg<C3_SPEC>`"]
pub type C3 = crate::Reg<c3::C3_SPEC>;
#[doc = "UART Control Register 3"]
pub mod c3;
#[doc = "D (rw) register accessor: an alias for `Reg<D_SPEC>`"]
pub type D = crate::Reg<d::D_SPEC>;
#[doc = "UART Data Register"]
pub mod d;
#[doc = "MA1 (rw) register accessor: an alias for `Reg<MA1_SPEC>`"]
pub type MA1 = crate::Reg<ma1::MA1_SPEC>;
#[doc = "UART Match Address Registers 1"]
pub mod ma1;
#[doc = "MA2 (rw) register accessor: an alias for `Reg<MA2_SPEC>`"]
pub type MA2 = crate::Reg<ma2::MA2_SPEC>;
#[doc = "UART Match Address Registers 2"]
pub mod ma2;
#[doc = "C4 (rw) register accessor: an alias for `Reg<C4_SPEC>`"]
pub type C4 = crate::Reg<c4::C4_SPEC>;
#[doc = "UART Control Register 4"]
pub mod c4;
#[doc = "C5 (rw) register accessor: an alias for `Reg<C5_SPEC>`"]
pub type C5 = crate::Reg<c5::C5_SPEC>;
#[doc = "UART Control Register 5"]
pub mod c5;
#[doc = "ED (r) register accessor: an alias for `Reg<ED_SPEC>`"]
pub type ED = crate::Reg<ed::ED_SPEC>;
#[doc = "UART Extended Data Register"]
pub mod ed;
#[doc = "MODEM (rw) register accessor: an alias for `Reg<MODEM_SPEC>`"]
pub type MODEM = crate::Reg<modem::MODEM_SPEC>;
#[doc = "UART Modem Register"]
pub mod modem;
#[doc = "IR (rw) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "UART Infrared Register"]
pub mod ir;
#[doc = "PFIFO (rw) register accessor: an alias for `Reg<PFIFO_SPEC>`"]
pub type PFIFO = crate::Reg<pfifo::PFIFO_SPEC>;
#[doc = "UART FIFO Parameters"]
pub mod pfifo;
#[doc = "CFIFO (rw) register accessor: an alias for `Reg<CFIFO_SPEC>`"]
pub type CFIFO = crate::Reg<cfifo::CFIFO_SPEC>;
#[doc = "UART FIFO Control Register"]
pub mod cfifo;
#[doc = "SFIFO (rw) register accessor: an alias for `Reg<SFIFO_SPEC>`"]
pub type SFIFO = crate::Reg<sfifo::SFIFO_SPEC>;
#[doc = "UART FIFO Status Register"]
pub mod sfifo;
#[doc = "TWFIFO (rw) register accessor: an alias for `Reg<TWFIFO_SPEC>`"]
pub type TWFIFO = crate::Reg<twfifo::TWFIFO_SPEC>;
#[doc = "UART FIFO Transmit Watermark"]
pub mod twfifo;
#[doc = "TCFIFO (r) register accessor: an alias for `Reg<TCFIFO_SPEC>`"]
pub type TCFIFO = crate::Reg<tcfifo::TCFIFO_SPEC>;
#[doc = "UART FIFO Transmit Count"]
pub mod tcfifo;
#[doc = "RWFIFO (rw) register accessor: an alias for `Reg<RWFIFO_SPEC>`"]
pub type RWFIFO = crate::Reg<rwfifo::RWFIFO_SPEC>;
#[doc = "UART FIFO Receive Watermark"]
pub mod rwfifo;
#[doc = "RCFIFO (r) register accessor: an alias for `Reg<RCFIFO_SPEC>`"]
pub type RCFIFO = crate::Reg<rcfifo::RCFIFO_SPEC>;
#[doc = "UART FIFO Receive Count"]
pub mod rcfifo;

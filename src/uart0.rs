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
    _reserved22: [u8; 0x01],
    #[doc = "0x18 - UART 7816 Control Register"]
    pub c7816: C7816,
    #[doc = "0x19 - UART 7816 Interrupt Enable Register"]
    pub ie7816: IE7816,
    #[doc = "0x1a - UART 7816 Interrupt Status Register"]
    pub is7816: IS7816,
    _reserved_25_uart0_wp7816t: [u8; 0x01],
    #[doc = "0x1c - UART 7816 Wait N Register"]
    pub wn7816: WN7816,
    #[doc = "0x1d - UART 7816 Wait FD Register"]
    pub wf7816: WF7816,
    #[doc = "0x1e - UART 7816 Error Threshold Register"]
    pub et7816: ET7816,
    #[doc = "0x1f - UART 7816 Transmit Length Register"]
    pub tl7816: TL7816,
}
impl RegisterBlock {
    #[doc = "0x1b - UART 7816 Wait Parameter Register"]
    #[inline(always)]
    pub const fn uart0_wp7816t1(&self) -> &UART0_WP7816T1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(27usize).cast() }
    }
    #[doc = "0x1b - UART 7816 Wait Parameter Register"]
    #[inline(always)]
    pub const fn uart0_wp7816t0(&self) -> &UART0_WP7816T0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(27usize).cast() }
    }
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
#[doc = "C7816 (rw) register accessor: an alias for `Reg<C7816_SPEC>`"]
pub type C7816 = crate::Reg<c7816::C7816_SPEC>;
#[doc = "UART 7816 Control Register"]
pub mod c7816;
#[doc = "IE7816 (rw) register accessor: an alias for `Reg<IE7816_SPEC>`"]
pub type IE7816 = crate::Reg<ie7816::IE7816_SPEC>;
#[doc = "UART 7816 Interrupt Enable Register"]
pub mod ie7816;
#[doc = "IS7816 (rw) register accessor: an alias for `Reg<IS7816_SPEC>`"]
pub type IS7816 = crate::Reg<is7816::IS7816_SPEC>;
#[doc = "UART 7816 Interrupt Status Register"]
pub mod is7816;
#[doc = "UART0_WP7816T0 (rw) register accessor: an alias for `Reg<UART0_WP7816T0_SPEC>`"]
pub type UART0_WP7816T0 = crate::Reg<uart0_wp7816t0::UART0_WP7816T0_SPEC>;
#[doc = "UART 7816 Wait Parameter Register"]
pub mod uart0_wp7816t0;
#[doc = "UART0_WP7816T1 (rw) register accessor: an alias for `Reg<UART0_WP7816T1_SPEC>`"]
pub type UART0_WP7816T1 = crate::Reg<uart0_wp7816t1::UART0_WP7816T1_SPEC>;
#[doc = "UART 7816 Wait Parameter Register"]
pub mod uart0_wp7816t1;
#[doc = "WN7816 (rw) register accessor: an alias for `Reg<WN7816_SPEC>`"]
pub type WN7816 = crate::Reg<wn7816::WN7816_SPEC>;
#[doc = "UART 7816 Wait N Register"]
pub mod wn7816;
#[doc = "WF7816 (rw) register accessor: an alias for `Reg<WF7816_SPEC>`"]
pub type WF7816 = crate::Reg<wf7816::WF7816_SPEC>;
#[doc = "UART 7816 Wait FD Register"]
pub mod wf7816;
#[doc = "ET7816 (rw) register accessor: an alias for `Reg<ET7816_SPEC>`"]
pub type ET7816 = crate::Reg<et7816::ET7816_SPEC>;
#[doc = "UART 7816 Error Threshold Register"]
pub mod et7816;
#[doc = "TL7816 (rw) register accessor: an alias for `Reg<TL7816_SPEC>`"]
pub type TL7816 = crate::Reg<tl7816::TL7816_SPEC>;
#[doc = "UART 7816 Transmit Length Register"]
pub mod tl7816;

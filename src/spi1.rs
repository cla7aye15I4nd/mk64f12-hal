#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Transfer Count Register"]
    pub tcr: TCR,
    _reserved_2_spi1_: [u8; 0x08],
    _reserved3: [u8; 0x18],
    #[doc = "0x2c - Status Register"]
    pub sr: SR,
    #[doc = "0x30 - DMA/Interrupt Request Select and Enable Register"]
    pub rser: RSER,
    _reserved_5_spi1_: [u8; 0x04],
    #[doc = "0x38 - POP RX FIFO Register"]
    pub popr: POPR,
    #[doc = "0x3c..0x4c - Transmit FIFO Registers"]
    pub txfr: [TXFR; 4],
    _reserved8: [u8; 0x30],
    #[doc = "0x7c..0x8c - Receive FIFO Registers"]
    pub rxfr: [RXFR; 4],
}
impl RegisterBlock {
    #[doc = "0x0c - Clock and Transfer Attributes Register (In Slave Mode)"]
    #[inline(always)]
    pub const fn spi1_ctar_slave(&self) -> &SPI1_CTAR_SLAVE {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c..0x14 - Clock and Transfer Attributes Register (In Master Mode)"]
    #[inline(always)]
    pub const fn spi1_ctar(&self) -> &[SPI1_CTAR; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x34 - PUSH TX FIFO Register In Slave Mode"]
    #[inline(always)]
    pub const fn spi1_pushr_slave(&self) -> &SPI1_PUSHR_SLAVE {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x34 - PUSH TX FIFO Register In Master Mode"]
    #[inline(always)]
    pub const fn spi1_pushr(&self) -> &SPI1_PUSHR {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
}
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transfer Count Register"]
pub mod tcr;
#[doc = "SPI1_CTAR (rw) register accessor: an alias for `Reg<SPI1_CTAR_SPEC>`"]
pub type SPI1_CTAR = crate::Reg<spi1_ctar::SPI1_CTAR_SPEC>;
#[doc = "Clock and Transfer Attributes Register (In Master Mode)"]
pub mod spi1_ctar;
#[doc = "SPI1_CTAR_SLAVE (rw) register accessor: an alias for `Reg<SPI1_CTAR_SLAVE_SPEC>`"]
pub type SPI1_CTAR_SLAVE = crate::Reg<spi1_ctar_slave::SPI1_CTAR_SLAVE_SPEC>;
#[doc = "Clock and Transfer Attributes Register (In Slave Mode)"]
pub mod spi1_ctar_slave;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "RSER (rw) register accessor: an alias for `Reg<RSER_SPEC>`"]
pub type RSER = crate::Reg<rser::RSER_SPEC>;
#[doc = "DMA/Interrupt Request Select and Enable Register"]
pub mod rser;
#[doc = "SPI1_PUSHR (rw) register accessor: an alias for `Reg<SPI1_PUSHR_SPEC>`"]
pub type SPI1_PUSHR = crate::Reg<spi1_pushr::SPI1_PUSHR_SPEC>;
#[doc = "PUSH TX FIFO Register In Master Mode"]
pub mod spi1_pushr;
#[doc = "SPI1_PUSHR_SLAVE (rw) register accessor: an alias for `Reg<SPI1_PUSHR_SLAVE_SPEC>`"]
pub type SPI1_PUSHR_SLAVE = crate::Reg<spi1_pushr_slave::SPI1_PUSHR_SLAVE_SPEC>;
#[doc = "PUSH TX FIFO Register In Slave Mode"]
pub mod spi1_pushr_slave;
#[doc = "POPR (r) register accessor: an alias for `Reg<POPR_SPEC>`"]
pub type POPR = crate::Reg<popr::POPR_SPEC>;
#[doc = "POP RX FIFO Register"]
pub mod popr;
#[doc = "TXFR (r) register accessor: an alias for `Reg<TXFR_SPEC>`"]
pub type TXFR = crate::Reg<txfr::TXFR_SPEC>;
#[doc = "Transmit FIFO Registers"]
pub mod txfr;
#[doc = "RXFR (r) register accessor: an alias for `Reg<RXFR_SPEC>`"]
pub type RXFR = crate::Reg<rxfr::RXFR_SPEC>;
#[doc = "Receive FIFO Registers"]
pub mod rxfr;

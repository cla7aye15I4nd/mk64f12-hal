#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    #[doc = "0x04 - Control 1 register"]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Free Running Timer"]
    pub timer: TIMER,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Rx Mailboxes Global Mask Register"]
    pub rxmgmask: RXMGMASK,
    #[doc = "0x14 - Rx 14 Mask register"]
    pub rx14mask: RX14MASK,
    #[doc = "0x18 - Rx 15 Mask register"]
    pub rx15mask: RX15MASK,
    #[doc = "0x1c - Error Counter"]
    pub ecr: ECR,
    #[doc = "0x20 - Error and Status 1 register"]
    pub esr1: ESR1,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - Interrupt Masks 1 register"]
    pub imask1: IMASK1,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - Interrupt Flags 1 register"]
    pub iflag1: IFLAG1,
    #[doc = "0x34 - Control 2 register"]
    pub ctrl2: CTRL2,
    #[doc = "0x38 - Error and Status 2 register"]
    pub esr2: ESR2,
    _reserved12: [u8; 0x08],
    #[doc = "0x44 - CRC Register"]
    pub crcr: CRCR,
    #[doc = "0x48 - Rx FIFO Global Mask register"]
    pub rxfgmask: RXFGMASK,
    #[doc = "0x4c - Rx FIFO Information Register"]
    pub rxfir: RXFIR,
    _reserved15: [u8; 0x30],
    #[doc = "0x80 - Message Buffer 0 CS Register"]
    pub cs0: CS0,
    #[doc = "0x84 - Message Buffer 0 ID Register"]
    pub id0: ID0,
    #[doc = "0x88 - Message Buffer 0 WORD0 Register"]
    pub word00: WORD00,
    #[doc = "0x8c - Message Buffer 0 WORD1 Register"]
    pub word10: WORD10,
    #[doc = "0x90 - Message Buffer 1 CS Register"]
    pub cs1: CS1,
    #[doc = "0x94 - Message Buffer 1 ID Register"]
    pub id1: ID1,
    #[doc = "0x98 - Message Buffer 1 WORD0 Register"]
    pub word01: WORD01,
    #[doc = "0x9c - Message Buffer 1 WORD1 Register"]
    pub word11: WORD11,
    #[doc = "0xa0 - Message Buffer 2 CS Register"]
    pub cs2: CS2,
    #[doc = "0xa4 - Message Buffer 2 ID Register"]
    pub id2: ID2,
    #[doc = "0xa8 - Message Buffer 2 WORD0 Register"]
    pub word02: WORD02,
    #[doc = "0xac - Message Buffer 2 WORD1 Register"]
    pub word12: WORD12,
    #[doc = "0xb0 - Message Buffer 3 CS Register"]
    pub cs3: CS3,
    #[doc = "0xb4 - Message Buffer 3 ID Register"]
    pub id3: ID3,
    #[doc = "0xb8 - Message Buffer 3 WORD0 Register"]
    pub word03: WORD03,
    #[doc = "0xbc - Message Buffer 3 WORD1 Register"]
    pub word13: WORD13,
    #[doc = "0xc0 - Message Buffer 4 CS Register"]
    pub cs4: CS4,
    #[doc = "0xc4 - Message Buffer 4 ID Register"]
    pub id4: ID4,
    #[doc = "0xc8 - Message Buffer 4 WORD0 Register"]
    pub word04: WORD04,
    #[doc = "0xcc - Message Buffer 4 WORD1 Register"]
    pub word14: WORD14,
    #[doc = "0xd0 - Message Buffer 5 CS Register"]
    pub cs5: CS5,
    #[doc = "0xd4 - Message Buffer 5 ID Register"]
    pub id5: ID5,
    #[doc = "0xd8 - Message Buffer 5 WORD0 Register"]
    pub word05: WORD05,
    #[doc = "0xdc - Message Buffer 5 WORD1 Register"]
    pub word15: WORD15,
    #[doc = "0xe0 - Message Buffer 6 CS Register"]
    pub cs6: CS6,
    #[doc = "0xe4 - Message Buffer 6 ID Register"]
    pub id6: ID6,
    #[doc = "0xe8 - Message Buffer 6 WORD0 Register"]
    pub word06: WORD06,
    #[doc = "0xec - Message Buffer 6 WORD1 Register"]
    pub word16: WORD16,
    #[doc = "0xf0 - Message Buffer 7 CS Register"]
    pub cs7: CS7,
    #[doc = "0xf4 - Message Buffer 7 ID Register"]
    pub id7: ID7,
    #[doc = "0xf8 - Message Buffer 7 WORD0 Register"]
    pub word07: WORD07,
    #[doc = "0xfc - Message Buffer 7 WORD1 Register"]
    pub word17: WORD17,
    #[doc = "0x100 - Message Buffer 8 CS Register"]
    pub cs8: CS8,
    #[doc = "0x104 - Message Buffer 8 ID Register"]
    pub id8: ID8,
    #[doc = "0x108 - Message Buffer 8 WORD0 Register"]
    pub word08: WORD08,
    #[doc = "0x10c - Message Buffer 8 WORD1 Register"]
    pub word18: WORD18,
    #[doc = "0x110 - Message Buffer 9 CS Register"]
    pub cs9: CS9,
    #[doc = "0x114 - Message Buffer 9 ID Register"]
    pub id9: ID9,
    #[doc = "0x118 - Message Buffer 9 WORD0 Register"]
    pub word09: WORD09,
    #[doc = "0x11c - Message Buffer 9 WORD1 Register"]
    pub word19: WORD19,
    #[doc = "0x120 - Message Buffer 10 CS Register"]
    pub cs10: CS10,
    #[doc = "0x124 - Message Buffer 10 ID Register"]
    pub id10: ID10,
    #[doc = "0x128 - Message Buffer 10 WORD0 Register"]
    pub word010: WORD010,
    #[doc = "0x12c - Message Buffer 10 WORD1 Register"]
    pub word110: WORD110,
    #[doc = "0x130 - Message Buffer 11 CS Register"]
    pub cs11: CS11,
    #[doc = "0x134 - Message Buffer 11 ID Register"]
    pub id11: ID11,
    #[doc = "0x138 - Message Buffer 11 WORD0 Register"]
    pub word011: WORD011,
    #[doc = "0x13c - Message Buffer 11 WORD1 Register"]
    pub word111: WORD111,
    #[doc = "0x140 - Message Buffer 12 CS Register"]
    pub cs12: CS12,
    #[doc = "0x144 - Message Buffer 12 ID Register"]
    pub id12: ID12,
    #[doc = "0x148 - Message Buffer 12 WORD0 Register"]
    pub word012: WORD012,
    #[doc = "0x14c - Message Buffer 12 WORD1 Register"]
    pub word112: WORD112,
    #[doc = "0x150 - Message Buffer 13 CS Register"]
    pub cs13: CS13,
    #[doc = "0x154 - Message Buffer 13 ID Register"]
    pub id13: ID13,
    #[doc = "0x158 - Message Buffer 13 WORD0 Register"]
    pub word013: WORD013,
    #[doc = "0x15c - Message Buffer 13 WORD1 Register"]
    pub word113: WORD113,
    #[doc = "0x160 - Message Buffer 14 CS Register"]
    pub cs14: CS14,
    #[doc = "0x164 - Message Buffer 14 ID Register"]
    pub id14: ID14,
    #[doc = "0x168 - Message Buffer 14 WORD0 Register"]
    pub word014: WORD014,
    #[doc = "0x16c - Message Buffer 14 WORD1 Register"]
    pub word114: WORD114,
    #[doc = "0x170 - Message Buffer 15 CS Register"]
    pub cs15: CS15,
    #[doc = "0x174 - Message Buffer 15 ID Register"]
    pub id15: ID15,
    #[doc = "0x178 - Message Buffer 15 WORD0 Register"]
    pub word015: WORD015,
    #[doc = "0x17c - Message Buffer 15 WORD1 Register"]
    pub word115: WORD115,
    _reserved79: [u8; 0x0700],
    #[doc = "0x880..0x8c0 - Rx Individual Mask Registers"]
    pub rximr: [RXIMR; 16],
}
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control 1 register"]
pub mod ctrl1;
#[doc = "TIMER (rw) register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "Free Running Timer"]
pub mod timer;
#[doc = "RXMGMASK (rw) register accessor: an alias for `Reg<RXMGMASK_SPEC>`"]
pub type RXMGMASK = crate::Reg<rxmgmask::RXMGMASK_SPEC>;
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod rxmgmask;
#[doc = "RX14MASK (rw) register accessor: an alias for `Reg<RX14MASK_SPEC>`"]
pub type RX14MASK = crate::Reg<rx14mask::RX14MASK_SPEC>;
#[doc = "Rx 14 Mask register"]
pub mod rx14mask;
#[doc = "RX15MASK (rw) register accessor: an alias for `Reg<RX15MASK_SPEC>`"]
pub type RX15MASK = crate::Reg<rx15mask::RX15MASK_SPEC>;
#[doc = "Rx 15 Mask register"]
pub mod rx15mask;
#[doc = "ECR (rw) register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "ESR1 (rw) register accessor: an alias for `Reg<ESR1_SPEC>`"]
pub type ESR1 = crate::Reg<esr1::ESR1_SPEC>;
#[doc = "Error and Status 1 register"]
pub mod esr1;
#[doc = "IMASK1 (rw) register accessor: an alias for `Reg<IMASK1_SPEC>`"]
pub type IMASK1 = crate::Reg<imask1::IMASK1_SPEC>;
#[doc = "Interrupt Masks 1 register"]
pub mod imask1;
#[doc = "IFLAG1 (rw) register accessor: an alias for `Reg<IFLAG1_SPEC>`"]
pub type IFLAG1 = crate::Reg<iflag1::IFLAG1_SPEC>;
#[doc = "Interrupt Flags 1 register"]
pub mod iflag1;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control 2 register"]
pub mod ctrl2;
#[doc = "ESR2 (r) register accessor: an alias for `Reg<ESR2_SPEC>`"]
pub type ESR2 = crate::Reg<esr2::ESR2_SPEC>;
#[doc = "Error and Status 2 register"]
pub mod esr2;
#[doc = "CRCR (r) register accessor: an alias for `Reg<CRCR_SPEC>`"]
pub type CRCR = crate::Reg<crcr::CRCR_SPEC>;
#[doc = "CRC Register"]
pub mod crcr;
#[doc = "RXFGMASK (rw) register accessor: an alias for `Reg<RXFGMASK_SPEC>`"]
pub type RXFGMASK = crate::Reg<rxfgmask::RXFGMASK_SPEC>;
#[doc = "Rx FIFO Global Mask register"]
pub mod rxfgmask;
#[doc = "RXFIR (r) register accessor: an alias for `Reg<RXFIR_SPEC>`"]
pub type RXFIR = crate::Reg<rxfir::RXFIR_SPEC>;
#[doc = "Rx FIFO Information Register"]
pub mod rxfir;
#[doc = "CS0 (rw) register accessor: an alias for `Reg<CS0_SPEC>`"]
pub type CS0 = crate::Reg<cs0::CS0_SPEC>;
#[doc = "Message Buffer 0 CS Register"]
pub mod cs0;
#[doc = "ID0 (rw) register accessor: an alias for `Reg<ID0_SPEC>`"]
pub type ID0 = crate::Reg<id0::ID0_SPEC>;
#[doc = "Message Buffer 0 ID Register"]
pub mod id0;
#[doc = "WORD00 (rw) register accessor: an alias for `Reg<WORD00_SPEC>`"]
pub type WORD00 = crate::Reg<word00::WORD00_SPEC>;
#[doc = "Message Buffer 0 WORD0 Register"]
pub mod word00;
#[doc = "WORD10 (rw) register accessor: an alias for `Reg<WORD10_SPEC>`"]
pub type WORD10 = crate::Reg<word10::WORD10_SPEC>;
#[doc = "Message Buffer 0 WORD1 Register"]
pub mod word10;
#[doc = "CS1 (rw) register accessor: an alias for `Reg<CS1_SPEC>`"]
pub type CS1 = crate::Reg<cs1::CS1_SPEC>;
#[doc = "Message Buffer 1 CS Register"]
pub mod cs1;
#[doc = "ID1 (rw) register accessor: an alias for `Reg<ID1_SPEC>`"]
pub type ID1 = crate::Reg<id1::ID1_SPEC>;
#[doc = "Message Buffer 1 ID Register"]
pub mod id1;
#[doc = "WORD01 (rw) register accessor: an alias for `Reg<WORD01_SPEC>`"]
pub type WORD01 = crate::Reg<word01::WORD01_SPEC>;
#[doc = "Message Buffer 1 WORD0 Register"]
pub mod word01;
#[doc = "WORD11 (rw) register accessor: an alias for `Reg<WORD11_SPEC>`"]
pub type WORD11 = crate::Reg<word11::WORD11_SPEC>;
#[doc = "Message Buffer 1 WORD1 Register"]
pub mod word11;
#[doc = "CS2 (rw) register accessor: an alias for `Reg<CS2_SPEC>`"]
pub type CS2 = crate::Reg<cs2::CS2_SPEC>;
#[doc = "Message Buffer 2 CS Register"]
pub mod cs2;
#[doc = "ID2 (rw) register accessor: an alias for `Reg<ID2_SPEC>`"]
pub type ID2 = crate::Reg<id2::ID2_SPEC>;
#[doc = "Message Buffer 2 ID Register"]
pub mod id2;
#[doc = "WORD02 (rw) register accessor: an alias for `Reg<WORD02_SPEC>`"]
pub type WORD02 = crate::Reg<word02::WORD02_SPEC>;
#[doc = "Message Buffer 2 WORD0 Register"]
pub mod word02;
#[doc = "WORD12 (rw) register accessor: an alias for `Reg<WORD12_SPEC>`"]
pub type WORD12 = crate::Reg<word12::WORD12_SPEC>;
#[doc = "Message Buffer 2 WORD1 Register"]
pub mod word12;
#[doc = "CS3 (rw) register accessor: an alias for `Reg<CS3_SPEC>`"]
pub type CS3 = crate::Reg<cs3::CS3_SPEC>;
#[doc = "Message Buffer 3 CS Register"]
pub mod cs3;
#[doc = "ID3 (rw) register accessor: an alias for `Reg<ID3_SPEC>`"]
pub type ID3 = crate::Reg<id3::ID3_SPEC>;
#[doc = "Message Buffer 3 ID Register"]
pub mod id3;
#[doc = "WORD03 (rw) register accessor: an alias for `Reg<WORD03_SPEC>`"]
pub type WORD03 = crate::Reg<word03::WORD03_SPEC>;
#[doc = "Message Buffer 3 WORD0 Register"]
pub mod word03;
#[doc = "WORD13 (rw) register accessor: an alias for `Reg<WORD13_SPEC>`"]
pub type WORD13 = crate::Reg<word13::WORD13_SPEC>;
#[doc = "Message Buffer 3 WORD1 Register"]
pub mod word13;
#[doc = "CS4 (rw) register accessor: an alias for `Reg<CS4_SPEC>`"]
pub type CS4 = crate::Reg<cs4::CS4_SPEC>;
#[doc = "Message Buffer 4 CS Register"]
pub mod cs4;
#[doc = "ID4 (rw) register accessor: an alias for `Reg<ID4_SPEC>`"]
pub type ID4 = crate::Reg<id4::ID4_SPEC>;
#[doc = "Message Buffer 4 ID Register"]
pub mod id4;
#[doc = "WORD04 (rw) register accessor: an alias for `Reg<WORD04_SPEC>`"]
pub type WORD04 = crate::Reg<word04::WORD04_SPEC>;
#[doc = "Message Buffer 4 WORD0 Register"]
pub mod word04;
#[doc = "WORD14 (rw) register accessor: an alias for `Reg<WORD14_SPEC>`"]
pub type WORD14 = crate::Reg<word14::WORD14_SPEC>;
#[doc = "Message Buffer 4 WORD1 Register"]
pub mod word14;
#[doc = "CS5 (rw) register accessor: an alias for `Reg<CS5_SPEC>`"]
pub type CS5 = crate::Reg<cs5::CS5_SPEC>;
#[doc = "Message Buffer 5 CS Register"]
pub mod cs5;
#[doc = "ID5 (rw) register accessor: an alias for `Reg<ID5_SPEC>`"]
pub type ID5 = crate::Reg<id5::ID5_SPEC>;
#[doc = "Message Buffer 5 ID Register"]
pub mod id5;
#[doc = "WORD05 (rw) register accessor: an alias for `Reg<WORD05_SPEC>`"]
pub type WORD05 = crate::Reg<word05::WORD05_SPEC>;
#[doc = "Message Buffer 5 WORD0 Register"]
pub mod word05;
#[doc = "WORD15 (rw) register accessor: an alias for `Reg<WORD15_SPEC>`"]
pub type WORD15 = crate::Reg<word15::WORD15_SPEC>;
#[doc = "Message Buffer 5 WORD1 Register"]
pub mod word15;
#[doc = "CS6 (rw) register accessor: an alias for `Reg<CS6_SPEC>`"]
pub type CS6 = crate::Reg<cs6::CS6_SPEC>;
#[doc = "Message Buffer 6 CS Register"]
pub mod cs6;
#[doc = "ID6 (rw) register accessor: an alias for `Reg<ID6_SPEC>`"]
pub type ID6 = crate::Reg<id6::ID6_SPEC>;
#[doc = "Message Buffer 6 ID Register"]
pub mod id6;
#[doc = "WORD06 (rw) register accessor: an alias for `Reg<WORD06_SPEC>`"]
pub type WORD06 = crate::Reg<word06::WORD06_SPEC>;
#[doc = "Message Buffer 6 WORD0 Register"]
pub mod word06;
#[doc = "WORD16 (rw) register accessor: an alias for `Reg<WORD16_SPEC>`"]
pub type WORD16 = crate::Reg<word16::WORD16_SPEC>;
#[doc = "Message Buffer 6 WORD1 Register"]
pub mod word16;
#[doc = "CS7 (rw) register accessor: an alias for `Reg<CS7_SPEC>`"]
pub type CS7 = crate::Reg<cs7::CS7_SPEC>;
#[doc = "Message Buffer 7 CS Register"]
pub mod cs7;
#[doc = "ID7 (rw) register accessor: an alias for `Reg<ID7_SPEC>`"]
pub type ID7 = crate::Reg<id7::ID7_SPEC>;
#[doc = "Message Buffer 7 ID Register"]
pub mod id7;
#[doc = "WORD07 (rw) register accessor: an alias for `Reg<WORD07_SPEC>`"]
pub type WORD07 = crate::Reg<word07::WORD07_SPEC>;
#[doc = "Message Buffer 7 WORD0 Register"]
pub mod word07;
#[doc = "WORD17 (rw) register accessor: an alias for `Reg<WORD17_SPEC>`"]
pub type WORD17 = crate::Reg<word17::WORD17_SPEC>;
#[doc = "Message Buffer 7 WORD1 Register"]
pub mod word17;
#[doc = "CS8 (rw) register accessor: an alias for `Reg<CS8_SPEC>`"]
pub type CS8 = crate::Reg<cs8::CS8_SPEC>;
#[doc = "Message Buffer 8 CS Register"]
pub mod cs8;
#[doc = "ID8 (rw) register accessor: an alias for `Reg<ID8_SPEC>`"]
pub type ID8 = crate::Reg<id8::ID8_SPEC>;
#[doc = "Message Buffer 8 ID Register"]
pub mod id8;
#[doc = "WORD08 (rw) register accessor: an alias for `Reg<WORD08_SPEC>`"]
pub type WORD08 = crate::Reg<word08::WORD08_SPEC>;
#[doc = "Message Buffer 8 WORD0 Register"]
pub mod word08;
#[doc = "WORD18 (rw) register accessor: an alias for `Reg<WORD18_SPEC>`"]
pub type WORD18 = crate::Reg<word18::WORD18_SPEC>;
#[doc = "Message Buffer 8 WORD1 Register"]
pub mod word18;
#[doc = "CS9 (rw) register accessor: an alias for `Reg<CS9_SPEC>`"]
pub type CS9 = crate::Reg<cs9::CS9_SPEC>;
#[doc = "Message Buffer 9 CS Register"]
pub mod cs9;
#[doc = "ID9 (rw) register accessor: an alias for `Reg<ID9_SPEC>`"]
pub type ID9 = crate::Reg<id9::ID9_SPEC>;
#[doc = "Message Buffer 9 ID Register"]
pub mod id9;
#[doc = "WORD09 (rw) register accessor: an alias for `Reg<WORD09_SPEC>`"]
pub type WORD09 = crate::Reg<word09::WORD09_SPEC>;
#[doc = "Message Buffer 9 WORD0 Register"]
pub mod word09;
#[doc = "WORD19 (rw) register accessor: an alias for `Reg<WORD19_SPEC>`"]
pub type WORD19 = crate::Reg<word19::WORD19_SPEC>;
#[doc = "Message Buffer 9 WORD1 Register"]
pub mod word19;
#[doc = "CS10 (rw) register accessor: an alias for `Reg<CS10_SPEC>`"]
pub type CS10 = crate::Reg<cs10::CS10_SPEC>;
#[doc = "Message Buffer 10 CS Register"]
pub mod cs10;
#[doc = "ID10 (rw) register accessor: an alias for `Reg<ID10_SPEC>`"]
pub type ID10 = crate::Reg<id10::ID10_SPEC>;
#[doc = "Message Buffer 10 ID Register"]
pub mod id10;
#[doc = "WORD010 (rw) register accessor: an alias for `Reg<WORD010_SPEC>`"]
pub type WORD010 = crate::Reg<word010::WORD010_SPEC>;
#[doc = "Message Buffer 10 WORD0 Register"]
pub mod word010;
#[doc = "WORD110 (rw) register accessor: an alias for `Reg<WORD110_SPEC>`"]
pub type WORD110 = crate::Reg<word110::WORD110_SPEC>;
#[doc = "Message Buffer 10 WORD1 Register"]
pub mod word110;
#[doc = "CS11 (rw) register accessor: an alias for `Reg<CS11_SPEC>`"]
pub type CS11 = crate::Reg<cs11::CS11_SPEC>;
#[doc = "Message Buffer 11 CS Register"]
pub mod cs11;
#[doc = "ID11 (rw) register accessor: an alias for `Reg<ID11_SPEC>`"]
pub type ID11 = crate::Reg<id11::ID11_SPEC>;
#[doc = "Message Buffer 11 ID Register"]
pub mod id11;
#[doc = "WORD011 (rw) register accessor: an alias for `Reg<WORD011_SPEC>`"]
pub type WORD011 = crate::Reg<word011::WORD011_SPEC>;
#[doc = "Message Buffer 11 WORD0 Register"]
pub mod word011;
#[doc = "WORD111 (rw) register accessor: an alias for `Reg<WORD111_SPEC>`"]
pub type WORD111 = crate::Reg<word111::WORD111_SPEC>;
#[doc = "Message Buffer 11 WORD1 Register"]
pub mod word111;
#[doc = "CS12 (rw) register accessor: an alias for `Reg<CS12_SPEC>`"]
pub type CS12 = crate::Reg<cs12::CS12_SPEC>;
#[doc = "Message Buffer 12 CS Register"]
pub mod cs12;
#[doc = "ID12 (rw) register accessor: an alias for `Reg<ID12_SPEC>`"]
pub type ID12 = crate::Reg<id12::ID12_SPEC>;
#[doc = "Message Buffer 12 ID Register"]
pub mod id12;
#[doc = "WORD012 (rw) register accessor: an alias for `Reg<WORD012_SPEC>`"]
pub type WORD012 = crate::Reg<word012::WORD012_SPEC>;
#[doc = "Message Buffer 12 WORD0 Register"]
pub mod word012;
#[doc = "WORD112 (rw) register accessor: an alias for `Reg<WORD112_SPEC>`"]
pub type WORD112 = crate::Reg<word112::WORD112_SPEC>;
#[doc = "Message Buffer 12 WORD1 Register"]
pub mod word112;
#[doc = "CS13 (rw) register accessor: an alias for `Reg<CS13_SPEC>`"]
pub type CS13 = crate::Reg<cs13::CS13_SPEC>;
#[doc = "Message Buffer 13 CS Register"]
pub mod cs13;
#[doc = "ID13 (rw) register accessor: an alias for `Reg<ID13_SPEC>`"]
pub type ID13 = crate::Reg<id13::ID13_SPEC>;
#[doc = "Message Buffer 13 ID Register"]
pub mod id13;
#[doc = "WORD013 (rw) register accessor: an alias for `Reg<WORD013_SPEC>`"]
pub type WORD013 = crate::Reg<word013::WORD013_SPEC>;
#[doc = "Message Buffer 13 WORD0 Register"]
pub mod word013;
#[doc = "WORD113 (rw) register accessor: an alias for `Reg<WORD113_SPEC>`"]
pub type WORD113 = crate::Reg<word113::WORD113_SPEC>;
#[doc = "Message Buffer 13 WORD1 Register"]
pub mod word113;
#[doc = "CS14 (rw) register accessor: an alias for `Reg<CS14_SPEC>`"]
pub type CS14 = crate::Reg<cs14::CS14_SPEC>;
#[doc = "Message Buffer 14 CS Register"]
pub mod cs14;
#[doc = "ID14 (rw) register accessor: an alias for `Reg<ID14_SPEC>`"]
pub type ID14 = crate::Reg<id14::ID14_SPEC>;
#[doc = "Message Buffer 14 ID Register"]
pub mod id14;
#[doc = "WORD014 (rw) register accessor: an alias for `Reg<WORD014_SPEC>`"]
pub type WORD014 = crate::Reg<word014::WORD014_SPEC>;
#[doc = "Message Buffer 14 WORD0 Register"]
pub mod word014;
#[doc = "WORD114 (rw) register accessor: an alias for `Reg<WORD114_SPEC>`"]
pub type WORD114 = crate::Reg<word114::WORD114_SPEC>;
#[doc = "Message Buffer 14 WORD1 Register"]
pub mod word114;
#[doc = "CS15 (rw) register accessor: an alias for `Reg<CS15_SPEC>`"]
pub type CS15 = crate::Reg<cs15::CS15_SPEC>;
#[doc = "Message Buffer 15 CS Register"]
pub mod cs15;
#[doc = "ID15 (rw) register accessor: an alias for `Reg<ID15_SPEC>`"]
pub type ID15 = crate::Reg<id15::ID15_SPEC>;
#[doc = "Message Buffer 15 ID Register"]
pub mod id15;
#[doc = "WORD015 (rw) register accessor: an alias for `Reg<WORD015_SPEC>`"]
pub type WORD015 = crate::Reg<word015::WORD015_SPEC>;
#[doc = "Message Buffer 15 WORD0 Register"]
pub mod word015;
#[doc = "WORD115 (rw) register accessor: an alias for `Reg<WORD115_SPEC>`"]
pub type WORD115 = crate::Reg<word115::WORD115_SPEC>;
#[doc = "Message Buffer 15 WORD1 Register"]
pub mod word115;
#[doc = "RXIMR (rw) register accessor: an alias for `Reg<RXIMR_SPEC>`"]
pub type RXIMR = crate::Reg<rximr::RXIMR_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr;

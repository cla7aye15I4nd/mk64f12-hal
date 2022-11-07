#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Interrupt Event Register"]
    pub eir: EIR,
    #[doc = "0x08 - Interrupt Mask Register"]
    pub eimr: EIMR,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Receive Descriptor Active Register"]
    pub rdar: RDAR,
    #[doc = "0x14 - Transmit Descriptor Active Register"]
    pub tdar: TDAR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x24 - Ethernet Control Register"]
    pub ecr: ECR,
    _reserved5: [u8; 0x18],
    #[doc = "0x40 - MII Management Frame Register"]
    pub mmfr: MMFR,
    #[doc = "0x44 - MII Speed Control Register"]
    pub mscr: MSCR,
    _reserved7: [u8; 0x1c],
    #[doc = "0x64 - MIB Control Register"]
    pub mibc: MIBC,
    _reserved8: [u8; 0x1c],
    #[doc = "0x84 - Receive Control Register"]
    pub rcr: RCR,
    _reserved9: [u8; 0x3c],
    #[doc = "0xc4 - Transmit Control Register"]
    pub tcr: TCR,
    _reserved10: [u8; 0x1c],
    #[doc = "0xe4 - Physical Address Lower Register"]
    pub palr: PALR,
    #[doc = "0xe8 - Physical Address Upper Register"]
    pub paur: PAUR,
    #[doc = "0xec - Opcode/Pause Duration Register"]
    pub opd: OPD,
    _reserved13: [u8; 0x28],
    #[doc = "0x118 - Descriptor Individual Upper Address Register"]
    pub iaur: IAUR,
    #[doc = "0x11c - Descriptor Individual Lower Address Register"]
    pub ialr: IALR,
    #[doc = "0x120 - Descriptor Group Upper Address Register"]
    pub gaur: GAUR,
    #[doc = "0x124 - Descriptor Group Lower Address Register"]
    pub galr: GALR,
    _reserved17: [u8; 0x1c],
    #[doc = "0x144 - Transmit FIFO Watermark Register"]
    pub tfwr: TFWR,
    _reserved18: [u8; 0x38],
    #[doc = "0x180 - Receive Descriptor Ring Start Register"]
    pub rdsr: RDSR,
    #[doc = "0x184 - Transmit Buffer Descriptor Ring Start Register"]
    pub tdsr: TDSR,
    #[doc = "0x188 - Maximum Receive Buffer Size Register"]
    pub mrbr: MRBR,
    _reserved21: [u8; 0x04],
    #[doc = "0x190 - Receive FIFO Section Full Threshold"]
    pub rsfl: RSFL,
    #[doc = "0x194 - Receive FIFO Section Empty Threshold"]
    pub rsem: RSEM,
    #[doc = "0x198 - Receive FIFO Almost Empty Threshold"]
    pub raem: RAEM,
    #[doc = "0x19c - Receive FIFO Almost Full Threshold"]
    pub rafl: RAFL,
    #[doc = "0x1a0 - Transmit FIFO Section Empty Threshold"]
    pub tsem: TSEM,
    #[doc = "0x1a4 - Transmit FIFO Almost Empty Threshold"]
    pub taem: TAEM,
    #[doc = "0x1a8 - Transmit FIFO Almost Full Threshold"]
    pub tafl: TAFL,
    #[doc = "0x1ac - Transmit Inter-Packet Gap"]
    pub tipg: TIPG,
    #[doc = "0x1b0 - Frame Truncation Length"]
    pub ftrl: FTRL,
    _reserved30: [u8; 0x0c],
    #[doc = "0x1c0 - Transmit Accelerator Function Configuration"]
    pub tacc: TACC,
    #[doc = "0x1c4 - Receive Accelerator Function Configuration"]
    pub racc: RACC,
    _reserved32: [u8; 0x3c],
    #[doc = "0x204 - Tx Packet Count Statistic Register"]
    pub rmon_t_packets: RMON_T_PACKETS,
    #[doc = "0x208 - Tx Broadcast Packets Statistic Register"]
    pub rmon_t_bc_pkt: RMON_T_BC_PKT,
    #[doc = "0x20c - Tx Multicast Packets Statistic Register"]
    pub rmon_t_mc_pkt: RMON_T_MC_PKT,
    #[doc = "0x210 - Tx Packets with CRC/Align Error Statistic Register"]
    pub rmon_t_crc_align: RMON_T_CRC_ALIGN,
    #[doc = "0x214 - Tx Packets Less Than Bytes and Good CRC Statistic Register"]
    pub rmon_t_undersize: RMON_T_UNDERSIZE,
    #[doc = "0x218 - Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
    pub rmon_t_oversize: RMON_T_OVERSIZE,
    #[doc = "0x21c - Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    pub rmon_t_frag: RMON_T_FRAG,
    #[doc = "0x220 - Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
    pub rmon_t_jab: RMON_T_JAB,
    #[doc = "0x224 - Tx Collision Count Statistic Register"]
    pub rmon_t_col: RMON_T_COL,
    #[doc = "0x228 - Tx 64-Byte Packets Statistic Register"]
    pub rmon_t_p64: RMON_T_P64,
    #[doc = "0x22c - Tx 65- to 127-byte Packets Statistic Register"]
    pub rmon_t_p65to127: RMON_T_P65TO127,
    #[doc = "0x230 - Tx 128- to 255-byte Packets Statistic Register"]
    pub rmon_t_p128to255: RMON_T_P128TO255,
    #[doc = "0x234 - Tx 256- to 511-byte Packets Statistic Register"]
    pub rmon_t_p256to511: RMON_T_P256TO511,
    #[doc = "0x238 - Tx 512- to 1023-byte Packets Statistic Register"]
    pub rmon_t_p512to1023: RMON_T_P512TO1023,
    #[doc = "0x23c - Tx 1024- to 2047-byte Packets Statistic Register"]
    pub rmon_t_p1024to2047: RMON_T_P1024TO2047,
    #[doc = "0x240 - Tx Packets Greater Than 2048 Bytes Statistic Register"]
    pub rmon_t_p_gte2048: RMON_T_P_GTE2048,
    #[doc = "0x244 - Tx Octets Statistic Register"]
    pub rmon_t_octets: RMON_T_OCTETS,
    _reserved49: [u8; 0x04],
    #[doc = "0x24c - Frames Transmitted OK Statistic Register"]
    pub ieee_t_frame_ok: IEEE_T_FRAME_OK,
    #[doc = "0x250 - Frames Transmitted with Single Collision Statistic Register"]
    pub ieee_t_1col: IEEE_T_1COL,
    #[doc = "0x254 - Frames Transmitted with Multiple Collisions Statistic Register"]
    pub ieee_t_mcol: IEEE_T_MCOL,
    #[doc = "0x258 - Frames Transmitted after Deferral Delay Statistic Register"]
    pub ieee_t_def: IEEE_T_DEF,
    #[doc = "0x25c - Frames Transmitted with Late Collision Statistic Register"]
    pub ieee_t_lcol: IEEE_T_LCOL,
    #[doc = "0x260 - Frames Transmitted with Excessive Collisions Statistic Register"]
    pub ieee_t_excol: IEEE_T_EXCOL,
    #[doc = "0x264 - Frames Transmitted with Tx FIFO Underrun Statistic Register"]
    pub ieee_t_macerr: IEEE_T_MACERR,
    #[doc = "0x268 - Frames Transmitted with Carrier Sense Error Statistic Register"]
    pub ieee_t_cserr: IEEE_T_CSERR,
    _reserved57: [u8; 0x04],
    #[doc = "0x270 - Flow Control Pause Frames Transmitted Statistic Register"]
    pub ieee_t_fdxfc: IEEE_T_FDXFC,
    #[doc = "0x274 - Octet Count for Frames Transmitted w/o Error Statistic Register"]
    pub ieee_t_octets_ok: IEEE_T_OCTETS_OK,
    _reserved59: [u8; 0x0c],
    #[doc = "0x284 - Rx Packet Count Statistic Register"]
    pub rmon_r_packets: RMON_R_PACKETS,
    #[doc = "0x288 - Rx Broadcast Packets Statistic Register"]
    pub rmon_r_bc_pkt: RMON_R_BC_PKT,
    #[doc = "0x28c - Rx Multicast Packets Statistic Register"]
    pub rmon_r_mc_pkt: RMON_R_MC_PKT,
    #[doc = "0x290 - Rx Packets with CRC/Align Error Statistic Register"]
    pub rmon_r_crc_align: RMON_R_CRC_ALIGN,
    #[doc = "0x294 - Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
    pub rmon_r_undersize: RMON_R_UNDERSIZE,
    #[doc = "0x298 - Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
    pub rmon_r_oversize: RMON_R_OVERSIZE,
    #[doc = "0x29c - Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    pub rmon_r_frag: RMON_R_FRAG,
    #[doc = "0x2a0 - Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
    pub rmon_r_jab: RMON_R_JAB,
    _reserved67: [u8; 0x04],
    #[doc = "0x2a8 - Rx 64-Byte Packets Statistic Register"]
    pub rmon_r_p64: RMON_R_P64,
    #[doc = "0x2ac - Rx 65- to 127-Byte Packets Statistic Register"]
    pub rmon_r_p65to127: RMON_R_P65TO127,
    #[doc = "0x2b0 - Rx 128- to 255-Byte Packets Statistic Register"]
    pub rmon_r_p128to255: RMON_R_P128TO255,
    #[doc = "0x2b4 - Rx 256- to 511-Byte Packets Statistic Register"]
    pub rmon_r_p256to511: RMON_R_P256TO511,
    #[doc = "0x2b8 - Rx 512- to 1023-Byte Packets Statistic Register"]
    pub rmon_r_p512to1023: RMON_R_P512TO1023,
    #[doc = "0x2bc - Rx 1024- to 2047-Byte Packets Statistic Register"]
    pub rmon_r_p1024to2047: RMON_R_P1024TO2047,
    #[doc = "0x2c0 - Rx Packets Greater than 2048 Bytes Statistic Register"]
    pub rmon_r_p_gte2048: RMON_R_P_GTE2048,
    #[doc = "0x2c4 - Rx Octets Statistic Register"]
    pub rmon_r_octets: RMON_R_OCTETS,
    #[doc = "0x2c8 - Frames not Counted Correctly Statistic Register"]
    pub ieee_r_drop: IEEE_R_DROP,
    #[doc = "0x2cc - Frames Received OK Statistic Register"]
    pub ieee_r_frame_ok: IEEE_R_FRAME_OK,
    #[doc = "0x2d0 - Frames Received with CRC Error Statistic Register"]
    pub ieee_r_crc: IEEE_R_CRC,
    #[doc = "0x2d4 - Frames Received with Alignment Error Statistic Register"]
    pub ieee_r_align: IEEE_R_ALIGN,
    #[doc = "0x2d8 - Receive FIFO Overflow Count Statistic Register"]
    pub ieee_r_macerr: IEEE_R_MACERR,
    #[doc = "0x2dc - Flow Control Pause Frames Received Statistic Register"]
    pub ieee_r_fdxfc: IEEE_R_FDXFC,
    #[doc = "0x2e0 - Octet Count for Frames Received without Error Statistic Register"]
    pub ieee_r_octets_ok: IEEE_R_OCTETS_OK,
    _reserved82: [u8; 0x011c],
    #[doc = "0x400 - Adjustable Timer Control Register"]
    pub atcr: ATCR,
    #[doc = "0x404 - Timer Value Register"]
    pub atvr: ATVR,
    #[doc = "0x408 - Timer Offset Register"]
    pub atoff: ATOFF,
    #[doc = "0x40c - Timer Period Register"]
    pub atper: ATPER,
    #[doc = "0x410 - Timer Correction Register"]
    pub atcor: ATCOR,
    #[doc = "0x414 - Time-Stamping Clock Period Register"]
    pub atinc: ATINC,
    #[doc = "0x418 - Timestamp of Last Transmitted Frame"]
    pub atstmp: ATSTMP,
    _reserved89: [u8; 0x01e8],
    #[doc = "0x604 - Timer Global Status Register"]
    pub tgsr: TGSR,
    #[doc = "0x608 - Timer Control Status Register"]
    pub tcsr0: TCSR,
    #[doc = "0x60c - Timer Compare Capture Register"]
    pub tccr0: TCCR,
    #[doc = "0x610 - Timer Control Status Register"]
    pub tcsr1: TCSR,
    #[doc = "0x614 - Timer Compare Capture Register"]
    pub tccr1: TCCR,
    #[doc = "0x618 - Timer Control Status Register"]
    pub tcsr2: TCSR,
    #[doc = "0x61c - Timer Compare Capture Register"]
    pub tccr2: TCCR,
    #[doc = "0x620 - Timer Control Status Register"]
    pub tcsr3: TCSR,
    #[doc = "0x624 - Timer Compare Capture Register"]
    pub tccr3: TCCR,
}
#[doc = "EIR (rw) register accessor: an alias for `Reg<EIR_SPEC>`"]
pub type EIR = crate::Reg<eir::EIR_SPEC>;
#[doc = "Interrupt Event Register"]
pub mod eir;
#[doc = "EIMR (rw) register accessor: an alias for `Reg<EIMR_SPEC>`"]
pub type EIMR = crate::Reg<eimr::EIMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod eimr;
#[doc = "RDAR (rw) register accessor: an alias for `Reg<RDAR_SPEC>`"]
pub type RDAR = crate::Reg<rdar::RDAR_SPEC>;
#[doc = "Receive Descriptor Active Register"]
pub mod rdar;
#[doc = "TDAR (rw) register accessor: an alias for `Reg<TDAR_SPEC>`"]
pub type TDAR = crate::Reg<tdar::TDAR_SPEC>;
#[doc = "Transmit Descriptor Active Register"]
pub mod tdar;
#[doc = "ECR (rw) register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Ethernet Control Register"]
pub mod ecr;
#[doc = "MMFR (rw) register accessor: an alias for `Reg<MMFR_SPEC>`"]
pub type MMFR = crate::Reg<mmfr::MMFR_SPEC>;
#[doc = "MII Management Frame Register"]
pub mod mmfr;
#[doc = "MSCR (rw) register accessor: an alias for `Reg<MSCR_SPEC>`"]
pub type MSCR = crate::Reg<mscr::MSCR_SPEC>;
#[doc = "MII Speed Control Register"]
pub mod mscr;
#[doc = "MIBC (rw) register accessor: an alias for `Reg<MIBC_SPEC>`"]
pub type MIBC = crate::Reg<mibc::MIBC_SPEC>;
#[doc = "MIB Control Register"]
pub mod mibc;
#[doc = "RCR (rw) register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "Receive Control Register"]
pub mod rcr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transmit Control Register"]
pub mod tcr;
#[doc = "PALR (rw) register accessor: an alias for `Reg<PALR_SPEC>`"]
pub type PALR = crate::Reg<palr::PALR_SPEC>;
#[doc = "Physical Address Lower Register"]
pub mod palr;
#[doc = "PAUR (rw) register accessor: an alias for `Reg<PAUR_SPEC>`"]
pub type PAUR = crate::Reg<paur::PAUR_SPEC>;
#[doc = "Physical Address Upper Register"]
pub mod paur;
#[doc = "OPD (rw) register accessor: an alias for `Reg<OPD_SPEC>`"]
pub type OPD = crate::Reg<opd::OPD_SPEC>;
#[doc = "Opcode/Pause Duration Register"]
pub mod opd;
#[doc = "IAUR (rw) register accessor: an alias for `Reg<IAUR_SPEC>`"]
pub type IAUR = crate::Reg<iaur::IAUR_SPEC>;
#[doc = "Descriptor Individual Upper Address Register"]
pub mod iaur;
#[doc = "IALR (rw) register accessor: an alias for `Reg<IALR_SPEC>`"]
pub type IALR = crate::Reg<ialr::IALR_SPEC>;
#[doc = "Descriptor Individual Lower Address Register"]
pub mod ialr;
#[doc = "GAUR (rw) register accessor: an alias for `Reg<GAUR_SPEC>`"]
pub type GAUR = crate::Reg<gaur::GAUR_SPEC>;
#[doc = "Descriptor Group Upper Address Register"]
pub mod gaur;
#[doc = "GALR (rw) register accessor: an alias for `Reg<GALR_SPEC>`"]
pub type GALR = crate::Reg<galr::GALR_SPEC>;
#[doc = "Descriptor Group Lower Address Register"]
pub mod galr;
#[doc = "TFWR (rw) register accessor: an alias for `Reg<TFWR_SPEC>`"]
pub type TFWR = crate::Reg<tfwr::TFWR_SPEC>;
#[doc = "Transmit FIFO Watermark Register"]
pub mod tfwr;
#[doc = "RDSR (rw) register accessor: an alias for `Reg<RDSR_SPEC>`"]
pub type RDSR = crate::Reg<rdsr::RDSR_SPEC>;
#[doc = "Receive Descriptor Ring Start Register"]
pub mod rdsr;
#[doc = "TDSR (rw) register accessor: an alias for `Reg<TDSR_SPEC>`"]
pub type TDSR = crate::Reg<tdsr::TDSR_SPEC>;
#[doc = "Transmit Buffer Descriptor Ring Start Register"]
pub mod tdsr;
#[doc = "MRBR (rw) register accessor: an alias for `Reg<MRBR_SPEC>`"]
pub type MRBR = crate::Reg<mrbr::MRBR_SPEC>;
#[doc = "Maximum Receive Buffer Size Register"]
pub mod mrbr;
#[doc = "RSFL (rw) register accessor: an alias for `Reg<RSFL_SPEC>`"]
pub type RSFL = crate::Reg<rsfl::RSFL_SPEC>;
#[doc = "Receive FIFO Section Full Threshold"]
pub mod rsfl;
#[doc = "RSEM (rw) register accessor: an alias for `Reg<RSEM_SPEC>`"]
pub type RSEM = crate::Reg<rsem::RSEM_SPEC>;
#[doc = "Receive FIFO Section Empty Threshold"]
pub mod rsem;
#[doc = "RAEM (rw) register accessor: an alias for `Reg<RAEM_SPEC>`"]
pub type RAEM = crate::Reg<raem::RAEM_SPEC>;
#[doc = "Receive FIFO Almost Empty Threshold"]
pub mod raem;
#[doc = "RAFL (rw) register accessor: an alias for `Reg<RAFL_SPEC>`"]
pub type RAFL = crate::Reg<rafl::RAFL_SPEC>;
#[doc = "Receive FIFO Almost Full Threshold"]
pub mod rafl;
#[doc = "TSEM (rw) register accessor: an alias for `Reg<TSEM_SPEC>`"]
pub type TSEM = crate::Reg<tsem::TSEM_SPEC>;
#[doc = "Transmit FIFO Section Empty Threshold"]
pub mod tsem;
#[doc = "TAEM (rw) register accessor: an alias for `Reg<TAEM_SPEC>`"]
pub type TAEM = crate::Reg<taem::TAEM_SPEC>;
#[doc = "Transmit FIFO Almost Empty Threshold"]
pub mod taem;
#[doc = "TAFL (rw) register accessor: an alias for `Reg<TAFL_SPEC>`"]
pub type TAFL = crate::Reg<tafl::TAFL_SPEC>;
#[doc = "Transmit FIFO Almost Full Threshold"]
pub mod tafl;
#[doc = "TIPG (rw) register accessor: an alias for `Reg<TIPG_SPEC>`"]
pub type TIPG = crate::Reg<tipg::TIPG_SPEC>;
#[doc = "Transmit Inter-Packet Gap"]
pub mod tipg;
#[doc = "FTRL (rw) register accessor: an alias for `Reg<FTRL_SPEC>`"]
pub type FTRL = crate::Reg<ftrl::FTRL_SPEC>;
#[doc = "Frame Truncation Length"]
pub mod ftrl;
#[doc = "TACC (rw) register accessor: an alias for `Reg<TACC_SPEC>`"]
pub type TACC = crate::Reg<tacc::TACC_SPEC>;
#[doc = "Transmit Accelerator Function Configuration"]
pub mod tacc;
#[doc = "RACC (rw) register accessor: an alias for `Reg<RACC_SPEC>`"]
pub type RACC = crate::Reg<racc::RACC_SPEC>;
#[doc = "Receive Accelerator Function Configuration"]
pub mod racc;
#[doc = "RMON_T_PACKETS (r) register accessor: an alias for `Reg<RMON_T_PACKETS_SPEC>`"]
pub type RMON_T_PACKETS = crate::Reg<rmon_t_packets::RMON_T_PACKETS_SPEC>;
#[doc = "Tx Packet Count Statistic Register"]
pub mod rmon_t_packets;
#[doc = "RMON_T_BC_PKT (r) register accessor: an alias for `Reg<RMON_T_BC_PKT_SPEC>`"]
pub type RMON_T_BC_PKT = crate::Reg<rmon_t_bc_pkt::RMON_T_BC_PKT_SPEC>;
#[doc = "Tx Broadcast Packets Statistic Register"]
pub mod rmon_t_bc_pkt;
#[doc = "RMON_T_MC_PKT (r) register accessor: an alias for `Reg<RMON_T_MC_PKT_SPEC>`"]
pub type RMON_T_MC_PKT = crate::Reg<rmon_t_mc_pkt::RMON_T_MC_PKT_SPEC>;
#[doc = "Tx Multicast Packets Statistic Register"]
pub mod rmon_t_mc_pkt;
#[doc = "RMON_T_CRC_ALIGN (r) register accessor: an alias for `Reg<RMON_T_CRC_ALIGN_SPEC>`"]
pub type RMON_T_CRC_ALIGN = crate::Reg<rmon_t_crc_align::RMON_T_CRC_ALIGN_SPEC>;
#[doc = "Tx Packets with CRC/Align Error Statistic Register"]
pub mod rmon_t_crc_align;
#[doc = "RMON_T_UNDERSIZE (r) register accessor: an alias for `Reg<RMON_T_UNDERSIZE_SPEC>`"]
pub type RMON_T_UNDERSIZE = crate::Reg<rmon_t_undersize::RMON_T_UNDERSIZE_SPEC>;
#[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register"]
pub mod rmon_t_undersize;
#[doc = "RMON_T_OVERSIZE (r) register accessor: an alias for `Reg<RMON_T_OVERSIZE_SPEC>`"]
pub type RMON_T_OVERSIZE = crate::Reg<rmon_t_oversize::RMON_T_OVERSIZE_SPEC>;
#[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
pub mod rmon_t_oversize;
#[doc = "RMON_T_FRAG (r) register accessor: an alias for `Reg<RMON_T_FRAG_SPEC>`"]
pub type RMON_T_FRAG = crate::Reg<rmon_t_frag::RMON_T_FRAG_SPEC>;
#[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub mod rmon_t_frag;
#[doc = "RMON_T_JAB (r) register accessor: an alias for `Reg<RMON_T_JAB_SPEC>`"]
pub type RMON_T_JAB = crate::Reg<rmon_t_jab::RMON_T_JAB_SPEC>;
#[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
pub mod rmon_t_jab;
#[doc = "RMON_T_COL (r) register accessor: an alias for `Reg<RMON_T_COL_SPEC>`"]
pub type RMON_T_COL = crate::Reg<rmon_t_col::RMON_T_COL_SPEC>;
#[doc = "Tx Collision Count Statistic Register"]
pub mod rmon_t_col;
#[doc = "RMON_T_P64 (r) register accessor: an alias for `Reg<RMON_T_P64_SPEC>`"]
pub type RMON_T_P64 = crate::Reg<rmon_t_p64::RMON_T_P64_SPEC>;
#[doc = "Tx 64-Byte Packets Statistic Register"]
pub mod rmon_t_p64;
#[doc = "RMON_T_P65TO127 (r) register accessor: an alias for `Reg<RMON_T_P65TO127_SPEC>`"]
pub type RMON_T_P65TO127 = crate::Reg<rmon_t_p65to127::RMON_T_P65TO127_SPEC>;
#[doc = "Tx 65- to 127-byte Packets Statistic Register"]
pub mod rmon_t_p65to127;
#[doc = "RMON_T_P128TO255 (r) register accessor: an alias for `Reg<RMON_T_P128TO255_SPEC>`"]
pub type RMON_T_P128TO255 = crate::Reg<rmon_t_p128to255::RMON_T_P128TO255_SPEC>;
#[doc = "Tx 128- to 255-byte Packets Statistic Register"]
pub mod rmon_t_p128to255;
#[doc = "RMON_T_P256TO511 (r) register accessor: an alias for `Reg<RMON_T_P256TO511_SPEC>`"]
pub type RMON_T_P256TO511 = crate::Reg<rmon_t_p256to511::RMON_T_P256TO511_SPEC>;
#[doc = "Tx 256- to 511-byte Packets Statistic Register"]
pub mod rmon_t_p256to511;
#[doc = "RMON_T_P512TO1023 (r) register accessor: an alias for `Reg<RMON_T_P512TO1023_SPEC>`"]
pub type RMON_T_P512TO1023 = crate::Reg<rmon_t_p512to1023::RMON_T_P512TO1023_SPEC>;
#[doc = "Tx 512- to 1023-byte Packets Statistic Register"]
pub mod rmon_t_p512to1023;
#[doc = "RMON_T_P1024TO2047 (r) register accessor: an alias for `Reg<RMON_T_P1024TO2047_SPEC>`"]
pub type RMON_T_P1024TO2047 = crate::Reg<rmon_t_p1024to2047::RMON_T_P1024TO2047_SPEC>;
#[doc = "Tx 1024- to 2047-byte Packets Statistic Register"]
pub mod rmon_t_p1024to2047;
#[doc = "RMON_T_P_GTE2048 (r) register accessor: an alias for `Reg<RMON_T_P_GTE2048_SPEC>`"]
pub type RMON_T_P_GTE2048 = crate::Reg<rmon_t_p_gte2048::RMON_T_P_GTE2048_SPEC>;
#[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register"]
pub mod rmon_t_p_gte2048;
#[doc = "RMON_T_OCTETS (r) register accessor: an alias for `Reg<RMON_T_OCTETS_SPEC>`"]
pub type RMON_T_OCTETS = crate::Reg<rmon_t_octets::RMON_T_OCTETS_SPEC>;
#[doc = "Tx Octets Statistic Register"]
pub mod rmon_t_octets;
#[doc = "IEEE_T_FRAME_OK (r) register accessor: an alias for `Reg<IEEE_T_FRAME_OK_SPEC>`"]
pub type IEEE_T_FRAME_OK = crate::Reg<ieee_t_frame_ok::IEEE_T_FRAME_OK_SPEC>;
#[doc = "Frames Transmitted OK Statistic Register"]
pub mod ieee_t_frame_ok;
#[doc = "IEEE_T_1COL (r) register accessor: an alias for `Reg<IEEE_T_1COL_SPEC>`"]
pub type IEEE_T_1COL = crate::Reg<ieee_t_1col::IEEE_T_1COL_SPEC>;
#[doc = "Frames Transmitted with Single Collision Statistic Register"]
pub mod ieee_t_1col;
#[doc = "IEEE_T_MCOL (r) register accessor: an alias for `Reg<IEEE_T_MCOL_SPEC>`"]
pub type IEEE_T_MCOL = crate::Reg<ieee_t_mcol::IEEE_T_MCOL_SPEC>;
#[doc = "Frames Transmitted with Multiple Collisions Statistic Register"]
pub mod ieee_t_mcol;
#[doc = "IEEE_T_DEF (r) register accessor: an alias for `Reg<IEEE_T_DEF_SPEC>`"]
pub type IEEE_T_DEF = crate::Reg<ieee_t_def::IEEE_T_DEF_SPEC>;
#[doc = "Frames Transmitted after Deferral Delay Statistic Register"]
pub mod ieee_t_def;
#[doc = "IEEE_T_LCOL (r) register accessor: an alias for `Reg<IEEE_T_LCOL_SPEC>`"]
pub type IEEE_T_LCOL = crate::Reg<ieee_t_lcol::IEEE_T_LCOL_SPEC>;
#[doc = "Frames Transmitted with Late Collision Statistic Register"]
pub mod ieee_t_lcol;
#[doc = "IEEE_T_EXCOL (r) register accessor: an alias for `Reg<IEEE_T_EXCOL_SPEC>`"]
pub type IEEE_T_EXCOL = crate::Reg<ieee_t_excol::IEEE_T_EXCOL_SPEC>;
#[doc = "Frames Transmitted with Excessive Collisions Statistic Register"]
pub mod ieee_t_excol;
#[doc = "IEEE_T_MACERR (r) register accessor: an alias for `Reg<IEEE_T_MACERR_SPEC>`"]
pub type IEEE_T_MACERR = crate::Reg<ieee_t_macerr::IEEE_T_MACERR_SPEC>;
#[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register"]
pub mod ieee_t_macerr;
#[doc = "IEEE_T_CSERR (r) register accessor: an alias for `Reg<IEEE_T_CSERR_SPEC>`"]
pub type IEEE_T_CSERR = crate::Reg<ieee_t_cserr::IEEE_T_CSERR_SPEC>;
#[doc = "Frames Transmitted with Carrier Sense Error Statistic Register"]
pub mod ieee_t_cserr;
#[doc = "IEEE_T_FDXFC (r) register accessor: an alias for `Reg<IEEE_T_FDXFC_SPEC>`"]
pub type IEEE_T_FDXFC = crate::Reg<ieee_t_fdxfc::IEEE_T_FDXFC_SPEC>;
#[doc = "Flow Control Pause Frames Transmitted Statistic Register"]
pub mod ieee_t_fdxfc;
#[doc = "IEEE_T_OCTETS_OK (r) register accessor: an alias for `Reg<IEEE_T_OCTETS_OK_SPEC>`"]
pub type IEEE_T_OCTETS_OK = crate::Reg<ieee_t_octets_ok::IEEE_T_OCTETS_OK_SPEC>;
#[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register"]
pub mod ieee_t_octets_ok;
#[doc = "RMON_R_PACKETS (r) register accessor: an alias for `Reg<RMON_R_PACKETS_SPEC>`"]
pub type RMON_R_PACKETS = crate::Reg<rmon_r_packets::RMON_R_PACKETS_SPEC>;
#[doc = "Rx Packet Count Statistic Register"]
pub mod rmon_r_packets;
#[doc = "RMON_R_BC_PKT (r) register accessor: an alias for `Reg<RMON_R_BC_PKT_SPEC>`"]
pub type RMON_R_BC_PKT = crate::Reg<rmon_r_bc_pkt::RMON_R_BC_PKT_SPEC>;
#[doc = "Rx Broadcast Packets Statistic Register"]
pub mod rmon_r_bc_pkt;
#[doc = "RMON_R_MC_PKT (r) register accessor: an alias for `Reg<RMON_R_MC_PKT_SPEC>`"]
pub type RMON_R_MC_PKT = crate::Reg<rmon_r_mc_pkt::RMON_R_MC_PKT_SPEC>;
#[doc = "Rx Multicast Packets Statistic Register"]
pub mod rmon_r_mc_pkt;
#[doc = "RMON_R_CRC_ALIGN (r) register accessor: an alias for `Reg<RMON_R_CRC_ALIGN_SPEC>`"]
pub type RMON_R_CRC_ALIGN = crate::Reg<rmon_r_crc_align::RMON_R_CRC_ALIGN_SPEC>;
#[doc = "Rx Packets with CRC/Align Error Statistic Register"]
pub mod rmon_r_crc_align;
#[doc = "RMON_R_UNDERSIZE (r) register accessor: an alias for `Reg<RMON_R_UNDERSIZE_SPEC>`"]
pub type RMON_R_UNDERSIZE = crate::Reg<rmon_r_undersize::RMON_R_UNDERSIZE_SPEC>;
#[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
pub mod rmon_r_undersize;
#[doc = "RMON_R_OVERSIZE (r) register accessor: an alias for `Reg<RMON_R_OVERSIZE_SPEC>`"]
pub type RMON_R_OVERSIZE = crate::Reg<rmon_r_oversize::RMON_R_OVERSIZE_SPEC>;
#[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
pub mod rmon_r_oversize;
#[doc = "RMON_R_FRAG (r) register accessor: an alias for `Reg<RMON_R_FRAG_SPEC>`"]
pub type RMON_R_FRAG = crate::Reg<rmon_r_frag::RMON_R_FRAG_SPEC>;
#[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub mod rmon_r_frag;
#[doc = "RMON_R_JAB (r) register accessor: an alias for `Reg<RMON_R_JAB_SPEC>`"]
pub type RMON_R_JAB = crate::Reg<rmon_r_jab::RMON_R_JAB_SPEC>;
#[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
pub mod rmon_r_jab;
#[doc = "RMON_R_P64 (r) register accessor: an alias for `Reg<RMON_R_P64_SPEC>`"]
pub type RMON_R_P64 = crate::Reg<rmon_r_p64::RMON_R_P64_SPEC>;
#[doc = "Rx 64-Byte Packets Statistic Register"]
pub mod rmon_r_p64;
#[doc = "RMON_R_P65TO127 (r) register accessor: an alias for `Reg<RMON_R_P65TO127_SPEC>`"]
pub type RMON_R_P65TO127 = crate::Reg<rmon_r_p65to127::RMON_R_P65TO127_SPEC>;
#[doc = "Rx 65- to 127-Byte Packets Statistic Register"]
pub mod rmon_r_p65to127;
#[doc = "RMON_R_P128TO255 (r) register accessor: an alias for `Reg<RMON_R_P128TO255_SPEC>`"]
pub type RMON_R_P128TO255 = crate::Reg<rmon_r_p128to255::RMON_R_P128TO255_SPEC>;
#[doc = "Rx 128- to 255-Byte Packets Statistic Register"]
pub mod rmon_r_p128to255;
#[doc = "RMON_R_P256TO511 (r) register accessor: an alias for `Reg<RMON_R_P256TO511_SPEC>`"]
pub type RMON_R_P256TO511 = crate::Reg<rmon_r_p256to511::RMON_R_P256TO511_SPEC>;
#[doc = "Rx 256- to 511-Byte Packets Statistic Register"]
pub mod rmon_r_p256to511;
#[doc = "RMON_R_P512TO1023 (r) register accessor: an alias for `Reg<RMON_R_P512TO1023_SPEC>`"]
pub type RMON_R_P512TO1023 = crate::Reg<rmon_r_p512to1023::RMON_R_P512TO1023_SPEC>;
#[doc = "Rx 512- to 1023-Byte Packets Statistic Register"]
pub mod rmon_r_p512to1023;
#[doc = "RMON_R_P1024TO2047 (r) register accessor: an alias for `Reg<RMON_R_P1024TO2047_SPEC>`"]
pub type RMON_R_P1024TO2047 = crate::Reg<rmon_r_p1024to2047::RMON_R_P1024TO2047_SPEC>;
#[doc = "Rx 1024- to 2047-Byte Packets Statistic Register"]
pub mod rmon_r_p1024to2047;
#[doc = "RMON_R_P_GTE2048 (r) register accessor: an alias for `Reg<RMON_R_P_GTE2048_SPEC>`"]
pub type RMON_R_P_GTE2048 = crate::Reg<rmon_r_p_gte2048::RMON_R_P_GTE2048_SPEC>;
#[doc = "Rx Packets Greater than 2048 Bytes Statistic Register"]
pub mod rmon_r_p_gte2048;
#[doc = "RMON_R_OCTETS (r) register accessor: an alias for `Reg<RMON_R_OCTETS_SPEC>`"]
pub type RMON_R_OCTETS = crate::Reg<rmon_r_octets::RMON_R_OCTETS_SPEC>;
#[doc = "Rx Octets Statistic Register"]
pub mod rmon_r_octets;
#[doc = "IEEE_R_DROP (r) register accessor: an alias for `Reg<IEEE_R_DROP_SPEC>`"]
pub type IEEE_R_DROP = crate::Reg<ieee_r_drop::IEEE_R_DROP_SPEC>;
#[doc = "Frames not Counted Correctly Statistic Register"]
pub mod ieee_r_drop;
#[doc = "IEEE_R_FRAME_OK (r) register accessor: an alias for `Reg<IEEE_R_FRAME_OK_SPEC>`"]
pub type IEEE_R_FRAME_OK = crate::Reg<ieee_r_frame_ok::IEEE_R_FRAME_OK_SPEC>;
#[doc = "Frames Received OK Statistic Register"]
pub mod ieee_r_frame_ok;
#[doc = "IEEE_R_CRC (r) register accessor: an alias for `Reg<IEEE_R_CRC_SPEC>`"]
pub type IEEE_R_CRC = crate::Reg<ieee_r_crc::IEEE_R_CRC_SPEC>;
#[doc = "Frames Received with CRC Error Statistic Register"]
pub mod ieee_r_crc;
#[doc = "IEEE_R_ALIGN (r) register accessor: an alias for `Reg<IEEE_R_ALIGN_SPEC>`"]
pub type IEEE_R_ALIGN = crate::Reg<ieee_r_align::IEEE_R_ALIGN_SPEC>;
#[doc = "Frames Received with Alignment Error Statistic Register"]
pub mod ieee_r_align;
#[doc = "IEEE_R_MACERR (r) register accessor: an alias for `Reg<IEEE_R_MACERR_SPEC>`"]
pub type IEEE_R_MACERR = crate::Reg<ieee_r_macerr::IEEE_R_MACERR_SPEC>;
#[doc = "Receive FIFO Overflow Count Statistic Register"]
pub mod ieee_r_macerr;
#[doc = "IEEE_R_FDXFC (r) register accessor: an alias for `Reg<IEEE_R_FDXFC_SPEC>`"]
pub type IEEE_R_FDXFC = crate::Reg<ieee_r_fdxfc::IEEE_R_FDXFC_SPEC>;
#[doc = "Flow Control Pause Frames Received Statistic Register"]
pub mod ieee_r_fdxfc;
#[doc = "IEEE_R_OCTETS_OK (r) register accessor: an alias for `Reg<IEEE_R_OCTETS_OK_SPEC>`"]
pub type IEEE_R_OCTETS_OK = crate::Reg<ieee_r_octets_ok::IEEE_R_OCTETS_OK_SPEC>;
#[doc = "Octet Count for Frames Received without Error Statistic Register"]
pub mod ieee_r_octets_ok;
#[doc = "ATCR (rw) register accessor: an alias for `Reg<ATCR_SPEC>`"]
pub type ATCR = crate::Reg<atcr::ATCR_SPEC>;
#[doc = "Adjustable Timer Control Register"]
pub mod atcr;
#[doc = "ATVR (rw) register accessor: an alias for `Reg<ATVR_SPEC>`"]
pub type ATVR = crate::Reg<atvr::ATVR_SPEC>;
#[doc = "Timer Value Register"]
pub mod atvr;
#[doc = "ATOFF (rw) register accessor: an alias for `Reg<ATOFF_SPEC>`"]
pub type ATOFF = crate::Reg<atoff::ATOFF_SPEC>;
#[doc = "Timer Offset Register"]
pub mod atoff;
#[doc = "ATPER (rw) register accessor: an alias for `Reg<ATPER_SPEC>`"]
pub type ATPER = crate::Reg<atper::ATPER_SPEC>;
#[doc = "Timer Period Register"]
pub mod atper;
#[doc = "ATCOR (rw) register accessor: an alias for `Reg<ATCOR_SPEC>`"]
pub type ATCOR = crate::Reg<atcor::ATCOR_SPEC>;
#[doc = "Timer Correction Register"]
pub mod atcor;
#[doc = "ATINC (rw) register accessor: an alias for `Reg<ATINC_SPEC>`"]
pub type ATINC = crate::Reg<atinc::ATINC_SPEC>;
#[doc = "Time-Stamping Clock Period Register"]
pub mod atinc;
#[doc = "ATSTMP (r) register accessor: an alias for `Reg<ATSTMP_SPEC>`"]
pub type ATSTMP = crate::Reg<atstmp::ATSTMP_SPEC>;
#[doc = "Timestamp of Last Transmitted Frame"]
pub mod atstmp;
#[doc = "TGSR (rw) register accessor: an alias for `Reg<TGSR_SPEC>`"]
pub type TGSR = crate::Reg<tgsr::TGSR_SPEC>;
#[doc = "Timer Global Status Register"]
pub mod tgsr;
#[doc = "TCSR (rw) register accessor: an alias for `Reg<TCSR_SPEC>`"]
pub type TCSR = crate::Reg<tcsr::TCSR_SPEC>;
#[doc = "Timer Control Status Register"]
pub mod tcsr;
#[doc = "TCCR (rw) register accessor: an alias for `Reg<TCCR_SPEC>`"]
pub type TCCR = crate::Reg<tccr::TCCR_SPEC>;
#[doc = "Timer Compare Capture Register"]
pub mod tccr;

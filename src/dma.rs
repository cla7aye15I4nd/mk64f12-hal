#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Error Status Register"]
    pub es: ES,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: ERQ,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - Enable Error Interrupt Register"]
    pub eei: EEI,
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    pub ceei: CEEI,
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    pub seei: SEEI,
    #[doc = "0x1a - Clear Enable Request Register"]
    pub cerq: CERQ,
    #[doc = "0x1b - Set Enable Request Register"]
    pub serq: SERQ,
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    pub cdne: CDNE,
    #[doc = "0x1d - Set START Bit Register"]
    pub ssrt: SSRT,
    #[doc = "0x1e - Clear Error Register"]
    pub cerr: CERR,
    #[doc = "0x1f - Clear Interrupt Request Register"]
    pub cint: CINT,
    _reserved12: [u8; 0x04],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: INT,
    _reserved13: [u8; 0x04],
    #[doc = "0x2c - Error Register"]
    pub err: ERR,
    _reserved14: [u8; 0x04],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: HRS,
    _reserved15: [u8; 0xc8],
    #[doc = "0x100..0x110 - Channel n Priority Register"]
    pub dchpri: [DCHPRI; 16],
    _reserved16: [u8; 0x0ef0],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: TCD_SADDR,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: TCD_SOFF,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: TCD_ATTR,
    _reserved_19_dma_tcd0_nbytes: [u8; 0x04],
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: TCD_SLAST,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: TCD_DADDR,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: TCD_DOFF,
    _reserved_23_dma_tcd0_citer: [u8; 0x02],
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD_DLASTSGA,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: TCD_CSR,
    _reserved_26_dma_tcd0_biter: [u8; 0x02],
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: TCD_SADDR,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: TCD_SOFF,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: TCD_ATTR,
    _reserved_30_dma_tcd1_nbytes: [u8; 0x04],
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: TCD_SLAST,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: TCD_DADDR,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: TCD_DOFF,
    _reserved_34_dma_tcd1_citer: [u8; 0x02],
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD_DLASTSGA,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: TCD_CSR,
    _reserved_37_dma_tcd1_biter: [u8; 0x02],
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: TCD_SADDR,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: TCD_SOFF,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: TCD_ATTR,
    _reserved_41_dma_tcd2_nbytes: [u8; 0x04],
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: TCD_SLAST,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: TCD_DADDR,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: TCD_DOFF,
    _reserved_45_dma_tcd2_citer: [u8; 0x02],
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD_DLASTSGA,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: TCD_CSR,
    _reserved_48_dma_tcd2_biter: [u8; 0x02],
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: TCD_SADDR,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: TCD_SOFF,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: TCD_ATTR,
    _reserved_52_dma_tcd3_nbytes: [u8; 0x04],
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: TCD_SLAST,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: TCD_DADDR,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: TCD_DOFF,
    _reserved_56_dma_tcd3_citer: [u8; 0x02],
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD_DLASTSGA,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: TCD_CSR,
    _reserved_59_dma_tcd3_biter: [u8; 0x02],
    #[doc = "0x1080 - TCD Source Address"]
    pub tcd4_saddr: TCD_SADDR,
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    pub tcd4_soff: TCD_SOFF,
    #[doc = "0x1086 - TCD Transfer Attributes"]
    pub tcd4_attr: TCD_ATTR,
    _reserved_63_dma_tcd4_nbytes: [u8; 0x04],
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    pub tcd4_slast: TCD_SLAST,
    #[doc = "0x1090 - TCD Destination Address"]
    pub tcd4_daddr: TCD_DADDR,
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    pub tcd4_doff: TCD_DOFF,
    _reserved_67_dma_tcd4_citer: [u8; 0x02],
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: TCD_DLASTSGA,
    #[doc = "0x109c - TCD Control and Status"]
    pub tcd4_csr: TCD_CSR,
    _reserved_70_dma_tcd4_biter: [u8; 0x02],
    #[doc = "0x10a0 - TCD Source Address"]
    pub tcd5_saddr: TCD_SADDR,
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    pub tcd5_soff: TCD_SOFF,
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    pub tcd5_attr: TCD_ATTR,
    _reserved_74_dma_tcd5_nbytes: [u8; 0x04],
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    pub tcd5_slast: TCD_SLAST,
    #[doc = "0x10b0 - TCD Destination Address"]
    pub tcd5_daddr: TCD_DADDR,
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    pub tcd5_doff: TCD_DOFF,
    _reserved_78_dma_tcd5_citer: [u8; 0x02],
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10bc - TCD Control and Status"]
    pub tcd5_csr: TCD_CSR,
    _reserved_81_dma_tcd5_biter: [u8; 0x02],
    #[doc = "0x10c0 - TCD Source Address"]
    pub tcd6_saddr: TCD_SADDR,
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    pub tcd6_soff: TCD_SOFF,
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    pub tcd6_attr: TCD_ATTR,
    _reserved_85_dma_tcd6_nbytes: [u8; 0x04],
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    pub tcd6_slast: TCD_SLAST,
    #[doc = "0x10d0 - TCD Destination Address"]
    pub tcd6_daddr: TCD_DADDR,
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    pub tcd6_doff: TCD_DOFF,
    _reserved_89_dma_tcd6_citer: [u8; 0x02],
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10dc - TCD Control and Status"]
    pub tcd6_csr: TCD_CSR,
    _reserved_92_dma_tcd6_biter: [u8; 0x02],
    #[doc = "0x10e0 - TCD Source Address"]
    pub tcd7_saddr: TCD_SADDR,
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    pub tcd7_soff: TCD_SOFF,
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    pub tcd7_attr: TCD_ATTR,
    _reserved_96_dma_tcd7_nbytes: [u8; 0x04],
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    pub tcd7_slast: TCD_SLAST,
    #[doc = "0x10f0 - TCD Destination Address"]
    pub tcd7_daddr: TCD_DADDR,
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    pub tcd7_doff: TCD_DOFF,
    _reserved_100_dma_tcd7_citer: [u8; 0x02],
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10fc - TCD Control and Status"]
    pub tcd7_csr: TCD_CSR,
    _reserved_103_dma_tcd7_biter: [u8; 0x02],
    #[doc = "0x1100 - TCD Source Address"]
    pub tcd8_saddr: TCD_SADDR,
    #[doc = "0x1104 - TCD Signed Source Address Offset"]
    pub tcd8_soff: TCD_SOFF,
    #[doc = "0x1106 - TCD Transfer Attributes"]
    pub tcd8_attr: TCD_ATTR,
    _reserved_107_dma_tcd8_nbytes: [u8; 0x04],
    #[doc = "0x110c - TCD Last Source Address Adjustment"]
    pub tcd8_slast: TCD_SLAST,
    #[doc = "0x1110 - TCD Destination Address"]
    pub tcd8_daddr: TCD_DADDR,
    #[doc = "0x1114 - TCD Signed Destination Address Offset"]
    pub tcd8_doff: TCD_DOFF,
    _reserved_111_dma_tcd8_citer: [u8; 0x02],
    #[doc = "0x1118 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd8_dlastsga: TCD_DLASTSGA,
    #[doc = "0x111c - TCD Control and Status"]
    pub tcd8_csr: TCD_CSR,
    _reserved_114_dma_tcd8_biter: [u8; 0x02],
    #[doc = "0x1120 - TCD Source Address"]
    pub tcd9_saddr: TCD_SADDR,
    #[doc = "0x1124 - TCD Signed Source Address Offset"]
    pub tcd9_soff: TCD_SOFF,
    #[doc = "0x1126 - TCD Transfer Attributes"]
    pub tcd9_attr: TCD_ATTR,
    _reserved_118_dma_tcd9_nbytes: [u8; 0x04],
    #[doc = "0x112c - TCD Last Source Address Adjustment"]
    pub tcd9_slast: TCD_SLAST,
    #[doc = "0x1130 - TCD Destination Address"]
    pub tcd9_daddr: TCD_DADDR,
    #[doc = "0x1134 - TCD Signed Destination Address Offset"]
    pub tcd9_doff: TCD_DOFF,
    _reserved_122_dma_tcd9_citer: [u8; 0x02],
    #[doc = "0x1138 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd9_dlastsga: TCD_DLASTSGA,
    #[doc = "0x113c - TCD Control and Status"]
    pub tcd9_csr: TCD_CSR,
    _reserved_125_dma_tcd9_biter: [u8; 0x02],
    #[doc = "0x1140 - TCD Source Address"]
    pub tcd10_saddr: TCD_SADDR,
    #[doc = "0x1144 - TCD Signed Source Address Offset"]
    pub tcd10_soff: TCD_SOFF,
    #[doc = "0x1146 - TCD Transfer Attributes"]
    pub tcd10_attr: TCD_ATTR,
    _reserved_129_dma_tcd10_nbytes: [u8; 0x04],
    #[doc = "0x114c - TCD Last Source Address Adjustment"]
    pub tcd10_slast: TCD_SLAST,
    #[doc = "0x1150 - TCD Destination Address"]
    pub tcd10_daddr: TCD_DADDR,
    #[doc = "0x1154 - TCD Signed Destination Address Offset"]
    pub tcd10_doff: TCD_DOFF,
    _reserved_133_dma_tcd10_citer: [u8; 0x02],
    #[doc = "0x1158 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd10_dlastsga: TCD_DLASTSGA,
    #[doc = "0x115c - TCD Control and Status"]
    pub tcd10_csr: TCD_CSR,
    _reserved_136_dma_tcd10_biter: [u8; 0x02],
    #[doc = "0x1160 - TCD Source Address"]
    pub tcd11_saddr: TCD_SADDR,
    #[doc = "0x1164 - TCD Signed Source Address Offset"]
    pub tcd11_soff: TCD_SOFF,
    #[doc = "0x1166 - TCD Transfer Attributes"]
    pub tcd11_attr: TCD_ATTR,
    _reserved_140_dma_tcd11_nbytes: [u8; 0x04],
    #[doc = "0x116c - TCD Last Source Address Adjustment"]
    pub tcd11_slast: TCD_SLAST,
    #[doc = "0x1170 - TCD Destination Address"]
    pub tcd11_daddr: TCD_DADDR,
    #[doc = "0x1174 - TCD Signed Destination Address Offset"]
    pub tcd11_doff: TCD_DOFF,
    _reserved_144_dma_tcd11_citer: [u8; 0x02],
    #[doc = "0x1178 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd11_dlastsga: TCD_DLASTSGA,
    #[doc = "0x117c - TCD Control and Status"]
    pub tcd11_csr: TCD_CSR,
    _reserved_147_dma_tcd11_biter: [u8; 0x02],
    #[doc = "0x1180 - TCD Source Address"]
    pub tcd12_saddr: TCD_SADDR,
    #[doc = "0x1184 - TCD Signed Source Address Offset"]
    pub tcd12_soff: TCD_SOFF,
    #[doc = "0x1186 - TCD Transfer Attributes"]
    pub tcd12_attr: TCD_ATTR,
    _reserved_151_dma_tcd12_nbytes: [u8; 0x04],
    #[doc = "0x118c - TCD Last Source Address Adjustment"]
    pub tcd12_slast: TCD_SLAST,
    #[doc = "0x1190 - TCD Destination Address"]
    pub tcd12_daddr: TCD_DADDR,
    #[doc = "0x1194 - TCD Signed Destination Address Offset"]
    pub tcd12_doff: TCD_DOFF,
    _reserved_155_dma_tcd12_citer: [u8; 0x02],
    #[doc = "0x1198 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd12_dlastsga: TCD_DLASTSGA,
    #[doc = "0x119c - TCD Control and Status"]
    pub tcd12_csr: TCD_CSR,
    _reserved_158_dma_tcd12_biter: [u8; 0x02],
    #[doc = "0x11a0 - TCD Source Address"]
    pub tcd13_saddr: TCD_SADDR,
    #[doc = "0x11a4 - TCD Signed Source Address Offset"]
    pub tcd13_soff: TCD_SOFF,
    #[doc = "0x11a6 - TCD Transfer Attributes"]
    pub tcd13_attr: TCD_ATTR,
    _reserved_162_dma_tcd13_nbytes: [u8; 0x04],
    #[doc = "0x11ac - TCD Last Source Address Adjustment"]
    pub tcd13_slast: TCD_SLAST,
    #[doc = "0x11b0 - TCD Destination Address"]
    pub tcd13_daddr: TCD_DADDR,
    #[doc = "0x11b4 - TCD Signed Destination Address Offset"]
    pub tcd13_doff: TCD_DOFF,
    _reserved_166_dma_tcd13_citer: [u8; 0x02],
    #[doc = "0x11b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd13_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11bc - TCD Control and Status"]
    pub tcd13_csr: TCD_CSR,
    _reserved_169_dma_tcd13_biter: [u8; 0x02],
    #[doc = "0x11c0 - TCD Source Address"]
    pub tcd14_saddr: TCD_SADDR,
    #[doc = "0x11c4 - TCD Signed Source Address Offset"]
    pub tcd14_soff: TCD_SOFF,
    #[doc = "0x11c6 - TCD Transfer Attributes"]
    pub tcd14_attr: TCD_ATTR,
    _reserved_173_dma_tcd14_nbytes: [u8; 0x04],
    #[doc = "0x11cc - TCD Last Source Address Adjustment"]
    pub tcd14_slast: TCD_SLAST,
    #[doc = "0x11d0 - TCD Destination Address"]
    pub tcd14_daddr: TCD_DADDR,
    #[doc = "0x11d4 - TCD Signed Destination Address Offset"]
    pub tcd14_doff: TCD_DOFF,
    _reserved_177_dma_tcd14_citer: [u8; 0x02],
    #[doc = "0x11d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd14_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11dc - TCD Control and Status"]
    pub tcd14_csr: TCD_CSR,
    _reserved_180_dma_tcd14_biter: [u8; 0x02],
    #[doc = "0x11e0 - TCD Source Address"]
    pub tcd15_saddr: TCD_SADDR,
    #[doc = "0x11e4 - TCD Signed Source Address Offset"]
    pub tcd15_soff: TCD_SOFF,
    #[doc = "0x11e6 - TCD Transfer Attributes"]
    pub tcd15_attr: TCD_ATTR,
    _reserved_184_dma_tcd15_nbytes: [u8; 0x04],
    #[doc = "0x11ec - TCD Last Source Address Adjustment"]
    pub tcd15_slast: TCD_SLAST,
    #[doc = "0x11f0 - TCD Destination Address"]
    pub tcd15_daddr: TCD_DADDR,
    #[doc = "0x11f4 - TCD Signed Destination Address Offset"]
    pub tcd15_doff: TCD_DOFF,
    _reserved_188_dma_tcd15_citer: [u8; 0x02],
    #[doc = "0x11f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd15_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11fc - TCD Control and Status"]
    pub tcd15_csr: TCD_CSR,
    _reserved_191_dma_tcd15_biter: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x100 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri3(&self) -> &DCHPRI {
        &self.dchpri[0]
    }
    #[doc = "0x101 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri2(&self) -> &DCHPRI {
        &self.dchpri[1]
    }
    #[doc = "0x102 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri1(&self) -> &DCHPRI {
        &self.dchpri[2]
    }
    #[doc = "0x103 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri0(&self) -> &DCHPRI {
        &self.dchpri[3]
    }
    #[doc = "0x104 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri7(&self) -> &DCHPRI {
        &self.dchpri[4]
    }
    #[doc = "0x105 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri6(&self) -> &DCHPRI {
        &self.dchpri[5]
    }
    #[doc = "0x106 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri5(&self) -> &DCHPRI {
        &self.dchpri[6]
    }
    #[doc = "0x107 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri4(&self) -> &DCHPRI {
        &self.dchpri[7]
    }
    #[doc = "0x108 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri11(&self) -> &DCHPRI {
        &self.dchpri[8]
    }
    #[doc = "0x109 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri10(&self) -> &DCHPRI {
        &self.dchpri[9]
    }
    #[doc = "0x10a - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri9(&self) -> &DCHPRI {
        &self.dchpri[10]
    }
    #[doc = "0x10b - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri8(&self) -> &DCHPRI {
        &self.dchpri[11]
    }
    #[doc = "0x10c - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri15(&self) -> &DCHPRI {
        &self.dchpri[12]
    }
    #[doc = "0x10d - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri14(&self) -> &DCHPRI {
        &self.dchpri[13]
    }
    #[doc = "0x10e - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri13(&self) -> &DCHPRI {
        &self.dchpri[14]
    }
    #[doc = "0x10f - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri12(&self) -> &DCHPRI {
        &self.dchpri[15]
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4104usize).cast() }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4104usize).cast() }
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4104usize).cast() }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4118usize).cast() }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4118usize).cast() }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4126usize).cast() }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4126usize).cast() }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4136usize).cast() }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4136usize).cast() }
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4136usize).cast() }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4150usize).cast() }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4150usize).cast() }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4158usize).cast() }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4158usize).cast() }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4168usize).cast() }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4168usize).cast() }
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4168usize).cast() }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4182usize).cast() }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4182usize).cast() }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4190usize).cast() }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4190usize).cast() }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4200usize).cast() }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4200usize).cast() }
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4200usize).cast() }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4214usize).cast() }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4214usize).cast() }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4222usize).cast() }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4222usize).cast() }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4232usize).cast() }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4232usize).cast() }
    }
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4232usize).cast() }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4246usize).cast() }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4246usize).cast() }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4254usize).cast() }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4254usize).cast() }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4264usize).cast() }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4264usize).cast() }
    }
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4264usize).cast() }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4278usize).cast() }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4278usize).cast() }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4286usize).cast() }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4286usize).cast() }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4296usize).cast() }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4296usize).cast() }
    }
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4296usize).cast() }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4310usize).cast() }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4310usize).cast() }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4318usize).cast() }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4318usize).cast() }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4328usize).cast() }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4328usize).cast() }
    }
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4328usize).cast() }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4342usize).cast() }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4342usize).cast() }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4350usize).cast() }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4350usize).cast() }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4360usize).cast() }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4360usize).cast() }
    }
    #[doc = "0x1108 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4360usize).cast() }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4374usize).cast() }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4374usize).cast() }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4382usize).cast() }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4382usize).cast() }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4392usize).cast() }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4392usize).cast() }
    }
    #[doc = "0x1128 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4392usize).cast() }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4406usize).cast() }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4406usize).cast() }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4414usize).cast() }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4414usize).cast() }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4424usize).cast() }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4424usize).cast() }
    }
    #[doc = "0x1148 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4424usize).cast() }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4438usize).cast() }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4438usize).cast() }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4446usize).cast() }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4446usize).cast() }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4456usize).cast() }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4456usize).cast() }
    }
    #[doc = "0x1168 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4456usize).cast() }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4470usize).cast() }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4470usize).cast() }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4478usize).cast() }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4478usize).cast() }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4488usize).cast() }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4488usize).cast() }
    }
    #[doc = "0x1188 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4488usize).cast() }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4502usize).cast() }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4502usize).cast() }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4510usize).cast() }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4510usize).cast() }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4520usize).cast() }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4520usize).cast() }
    }
    #[doc = "0x11a8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4520usize).cast() }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4534usize).cast() }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4534usize).cast() }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4542usize).cast() }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4542usize).cast() }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4552usize).cast() }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4552usize).cast() }
    }
    #[doc = "0x11c8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4552usize).cast() }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4566usize).cast() }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4566usize).cast() }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4574usize).cast() }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4574usize).cast() }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4584usize).cast() }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4584usize).cast() }
    }
    #[doc = "0x11e8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4584usize).cast() }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4598usize).cast() }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4598usize).cast() }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4606usize).cast() }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4606usize).cast() }
    }
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "ES (r) register accessor: an alias for `Reg<ES_SPEC>`"]
pub type ES = crate::Reg<es::ES_SPEC>;
#[doc = "Error Status Register"]
pub mod es;
#[doc = "ERQ (rw) register accessor: an alias for `Reg<ERQ_SPEC>`"]
pub type ERQ = crate::Reg<erq::ERQ_SPEC>;
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "EEI (rw) register accessor: an alias for `Reg<EEI_SPEC>`"]
pub type EEI = crate::Reg<eei::EEI_SPEC>;
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "CEEI (w) register accessor: an alias for `Reg<CEEI_SPEC>`"]
pub type CEEI = crate::Reg<ceei::CEEI_SPEC>;
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "SEEI (w) register accessor: an alias for `Reg<SEEI_SPEC>`"]
pub type SEEI = crate::Reg<seei::SEEI_SPEC>;
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "CERQ (w) register accessor: an alias for `Reg<CERQ_SPEC>`"]
pub type CERQ = crate::Reg<cerq::CERQ_SPEC>;
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "SERQ (w) register accessor: an alias for `Reg<SERQ_SPEC>`"]
pub type SERQ = crate::Reg<serq::SERQ_SPEC>;
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "CDNE (w) register accessor: an alias for `Reg<CDNE_SPEC>`"]
pub type CDNE = crate::Reg<cdne::CDNE_SPEC>;
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "SSRT (w) register accessor: an alias for `Reg<SSRT_SPEC>`"]
pub type SSRT = crate::Reg<ssrt::SSRT_SPEC>;
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "CERR (w) register accessor: an alias for `Reg<CERR_SPEC>`"]
pub type CERR = crate::Reg<cerr::CERR_SPEC>;
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "CINT (w) register accessor: an alias for `Reg<CINT_SPEC>`"]
pub type CINT = crate::Reg<cint::CINT_SPEC>;
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "INT (rw) register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "ERR (rw) register accessor: an alias for `Reg<ERR_SPEC>`"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Error Register"]
pub mod err;
#[doc = "HRS (r) register accessor: an alias for `Reg<HRS_SPEC>`"]
pub type HRS = crate::Reg<hrs::HRS_SPEC>;
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "DCHPRI (rw) register accessor: an alias for `Reg<DCHPRI_SPEC>`"]
pub type DCHPRI = crate::Reg<dchpri::DCHPRI_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri;
#[doc = "TCD_SADDR (rw) register accessor: an alias for `Reg<TCD_SADDR_SPEC>`"]
pub type TCD_SADDR = crate::Reg<tcd_saddr::TCD_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd_saddr;
#[doc = "TCD_SOFF (rw) register accessor: an alias for `Reg<TCD_SOFF_SPEC>`"]
pub type TCD_SOFF = crate::Reg<tcd_soff::TCD_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd_soff;
#[doc = "TCD_ATTR (rw) register accessor: an alias for `Reg<TCD_ATTR_SPEC>`"]
pub type TCD_ATTR = crate::Reg<tcd_attr::TCD_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd_attr;
#[doc = "DMA_TCD_NBYTES_MLNO (rw) register accessor: an alias for `Reg<DMA_TCD_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD_NBYTES_MLNO = crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Disabled)"]
pub mod dma_tcd_nbytes_mlno;
#[doc = "DMA_TCD_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<DMA_TCD_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD_NBYTES_MLOFFNO = crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
pub mod dma_tcd_nbytes_mloffno;
#[doc = "DMA_TCD_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<DMA_TCD_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
pub mod dma_tcd_nbytes_mloffyes;
#[doc = "TCD_SLAST (rw) register accessor: an alias for `Reg<TCD_SLAST_SPEC>`"]
pub type TCD_SLAST = crate::Reg<tcd_slast::TCD_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd_slast;
#[doc = "TCD_DADDR (rw) register accessor: an alias for `Reg<TCD_DADDR_SPEC>`"]
pub type TCD_DADDR = crate::Reg<tcd_daddr::TCD_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd_daddr;
#[doc = "TCD_DOFF (rw) register accessor: an alias for `Reg<TCD_DOFF_SPEC>`"]
pub type TCD_DOFF = crate::Reg<tcd_doff::TCD_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd_doff;
#[doc = "DMA_TCD_CITER_ELINKNO (rw) register accessor: an alias for `Reg<DMA_TCD_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD_CITER_ELINKNO = crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd_citer_elinkno;
#[doc = "DMA_TCD_CITER_ELINKYES (rw) register accessor: an alias for `Reg<DMA_TCD_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD_CITER_ELINKYES = crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd_citer_elinkyes;
#[doc = "TCD_DLASTSGA (rw) register accessor: an alias for `Reg<TCD_DLASTSGA_SPEC>`"]
pub type TCD_DLASTSGA = crate::Reg<tcd_dlastsga::TCD_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd_dlastsga;
#[doc = "TCD_CSR (rw) register accessor: an alias for `Reg<TCD_CSR_SPEC>`"]
pub type TCD_CSR = crate::Reg<tcd_csr::TCD_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd_csr;
#[doc = "DMA_TCD_BITER_ELINKNO (rw) register accessor: an alias for `Reg<DMA_TCD_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD_BITER_ELINKNO = crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd_biter_elinkno;
#[doc = "DMA_TCD_BITER_ELINKYES (rw) register accessor: an alias for `Reg<DMA_TCD_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD_BITER_ELINKYES = crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd_biter_elinkyes;

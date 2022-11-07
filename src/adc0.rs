#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x08 - ADC Status and Control Registers 1"]
    pub sc1: [SC1; 2],
    #[doc = "0x08 - ADC Configuration Register 1"]
    pub cfg1: CFG1,
    #[doc = "0x0c - ADC Configuration Register 2"]
    pub cfg2: CFG2,
    #[doc = "0x10..0x18 - ADC Data Result Register"]
    pub r: [R; 2],
    #[doc = "0x18..0x20 - Compare Value Registers"]
    pub cv: [CV; 2],
    #[doc = "0x20 - Status and Control Register 2"]
    pub sc2: SC2,
    #[doc = "0x24 - Status and Control Register 3"]
    pub sc3: SC3,
    #[doc = "0x28 - ADC Offset Correction Register"]
    pub ofs: OFS,
    #[doc = "0x2c - ADC Plus-Side Gain Register"]
    pub pg: PG,
    #[doc = "0x30 - ADC Minus-Side Gain Register"]
    pub mg: MG,
    #[doc = "0x34 - ADC Plus-Side General Calibration Value Register"]
    pub clpd: CLPD,
    #[doc = "0x38 - ADC Plus-Side General Calibration Value Register"]
    pub clps: CLPS,
    #[doc = "0x3c - ADC Plus-Side General Calibration Value Register"]
    pub clp4: CLP4,
    #[doc = "0x40 - ADC Plus-Side General Calibration Value Register"]
    pub clp3: CLP3,
    #[doc = "0x44 - ADC Plus-Side General Calibration Value Register"]
    pub clp2: CLP2,
    #[doc = "0x48 - ADC Plus-Side General Calibration Value Register"]
    pub clp1: CLP1,
    #[doc = "0x4c - ADC Plus-Side General Calibration Value Register"]
    pub clp0: CLP0,
    _reserved17: [u8; 0x04],
    #[doc = "0x54 - ADC Minus-Side General Calibration Value Register"]
    pub clmd: CLMD,
    #[doc = "0x58 - ADC Minus-Side General Calibration Value Register"]
    pub clms: CLMS,
    #[doc = "0x5c - ADC Minus-Side General Calibration Value Register"]
    pub clm4: CLM4,
    #[doc = "0x60 - ADC Minus-Side General Calibration Value Register"]
    pub clm3: CLM3,
    #[doc = "0x64 - ADC Minus-Side General Calibration Value Register"]
    pub clm2: CLM2,
    #[doc = "0x68 - ADC Minus-Side General Calibration Value Register"]
    pub clm1: CLM1,
    #[doc = "0x6c - ADC Minus-Side General Calibration Value Register"]
    pub clm0: CLM0,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC Status and Control Registers 1"]
    #[inline(always)]
    pub fn sc1a(&self) -> &SC1 {
        &self.sc1[0]
    }
    #[doc = "0x04 - ADC Status and Control Registers 1"]
    #[inline(always)]
    pub fn sc1b(&self) -> &SC1 {
        &self.sc1[1]
    }
    #[doc = "0x10 - ADC Data Result Register"]
    #[inline(always)]
    pub fn ra(&self) -> &R {
        &self.r[0]
    }
    #[doc = "0x14 - ADC Data Result Register"]
    #[inline(always)]
    pub fn rb(&self) -> &R {
        &self.r[1]
    }
    #[doc = "0x18 - Compare Value Registers"]
    #[inline(always)]
    pub fn cv1(&self) -> &CV {
        &self.cv[0]
    }
    #[doc = "0x1c - Compare Value Registers"]
    #[inline(always)]
    pub fn cv2(&self) -> &CV {
        &self.cv[1]
    }
}
#[doc = "SC1 (rw) register accessor: an alias for `Reg<SC1_SPEC>`"]
pub type SC1 = crate::Reg<sc1::SC1_SPEC>;
#[doc = "ADC Status and Control Registers 1"]
pub mod sc1;
#[doc = "CFG1 (rw) register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "ADC Configuration Register 1"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "ADC Configuration Register 2"]
pub mod cfg2;
#[doc = "R (r) register accessor: an alias for `Reg<R_SPEC>`"]
pub type R = crate::Reg<r::R_SPEC>;
#[doc = "ADC Data Result Register"]
pub mod r;
#[doc = "CV (rw) register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Compare Value Registers"]
pub mod cv;
#[doc = "SC2 (rw) register accessor: an alias for `Reg<SC2_SPEC>`"]
pub type SC2 = crate::Reg<sc2::SC2_SPEC>;
#[doc = "Status and Control Register 2"]
pub mod sc2;
#[doc = "SC3 (rw) register accessor: an alias for `Reg<SC3_SPEC>`"]
pub type SC3 = crate::Reg<sc3::SC3_SPEC>;
#[doc = "Status and Control Register 3"]
pub mod sc3;
#[doc = "OFS (rw) register accessor: an alias for `Reg<OFS_SPEC>`"]
pub type OFS = crate::Reg<ofs::OFS_SPEC>;
#[doc = "ADC Offset Correction Register"]
pub mod ofs;
#[doc = "PG (rw) register accessor: an alias for `Reg<PG_SPEC>`"]
pub type PG = crate::Reg<pg::PG_SPEC>;
#[doc = "ADC Plus-Side Gain Register"]
pub mod pg;
#[doc = "MG (rw) register accessor: an alias for `Reg<MG_SPEC>`"]
pub type MG = crate::Reg<mg::MG_SPEC>;
#[doc = "ADC Minus-Side Gain Register"]
pub mod mg;
#[doc = "CLPD (rw) register accessor: an alias for `Reg<CLPD_SPEC>`"]
pub type CLPD = crate::Reg<clpd::CLPD_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clpd;
#[doc = "CLPS (rw) register accessor: an alias for `Reg<CLPS_SPEC>`"]
pub type CLPS = crate::Reg<clps::CLPS_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clps;
#[doc = "CLP4 (rw) register accessor: an alias for `Reg<CLP4_SPEC>`"]
pub type CLP4 = crate::Reg<clp4::CLP4_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp4;
#[doc = "CLP3 (rw) register accessor: an alias for `Reg<CLP3_SPEC>`"]
pub type CLP3 = crate::Reg<clp3::CLP3_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp3;
#[doc = "CLP2 (rw) register accessor: an alias for `Reg<CLP2_SPEC>`"]
pub type CLP2 = crate::Reg<clp2::CLP2_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp2;
#[doc = "CLP1 (rw) register accessor: an alias for `Reg<CLP1_SPEC>`"]
pub type CLP1 = crate::Reg<clp1::CLP1_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp1;
#[doc = "CLP0 (rw) register accessor: an alias for `Reg<CLP0_SPEC>`"]
pub type CLP0 = crate::Reg<clp0::CLP0_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp0;
#[doc = "CLMD (rw) register accessor: an alias for `Reg<CLMD_SPEC>`"]
pub type CLMD = crate::Reg<clmd::CLMD_SPEC>;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clmd;
#[doc = "CLMS (rw) register accessor: an alias for `Reg<CLMS_SPEC>`"]
pub type CLMS = crate::Reg<clms::CLMS_SPEC>;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clms;
#[doc = "CLM4 (rw) register accessor: an alias for `Reg<CLM4_SPEC>`"]
pub type CLM4 = crate::Reg<clm4::CLM4_SPEC>;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clm4;
#[doc = "CLM3 (rw) register accessor: an alias for `Reg<CLM3_SPEC>`"]
pub type CLM3 = crate::Reg<clm3::CLM3_SPEC>;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clm3;
#[doc = "CLM2 (rw) register accessor: an alias for `Reg<CLM2_SPEC>`"]
pub type CLM2 = crate::Reg<clm2::CLM2_SPEC>;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clm2;
#[doc = "CLM1 (rw) register accessor: an alias for `Reg<CLM1_SPEC>`"]
pub type CLM1 = crate::Reg<clm1::CLM1_SPEC>;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clm1;
#[doc = "CLM0 (rw) register accessor: an alias for `Reg<CLM0_SPEC>`"]
pub type CLM0 = crate::Reg<clm0::CLM0_SPEC>;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clm0;

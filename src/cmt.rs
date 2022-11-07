#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMT Carrier Generator High Data Register 1"]
    pub cgh1: CGH1,
    #[doc = "0x01 - CMT Carrier Generator Low Data Register 1"]
    pub cgl1: CGL1,
    #[doc = "0x02 - CMT Carrier Generator High Data Register 2"]
    pub cgh2: CGH2,
    #[doc = "0x03 - CMT Carrier Generator Low Data Register 2"]
    pub cgl2: CGL2,
    #[doc = "0x04 - CMT Output Control Register"]
    pub oc: OC,
    #[doc = "0x05 - CMT Modulator Status and Control Register"]
    pub msc: MSC,
    #[doc = "0x06 - CMT Modulator Data Register Mark High"]
    pub cmd1: CMD1,
    #[doc = "0x07 - CMT Modulator Data Register Mark Low"]
    pub cmd2: CMD2,
    #[doc = "0x08 - CMT Modulator Data Register Space High"]
    pub cmd3: CMD3,
    #[doc = "0x09 - CMT Modulator Data Register Space Low"]
    pub cmd4: CMD4,
    #[doc = "0x0a - CMT Primary Prescaler Register"]
    pub pps: PPS,
    #[doc = "0x0b - CMT Direct Memory Access Register"]
    pub dma: DMA,
}
#[doc = "CGH1 (rw) register accessor: an alias for `Reg<CGH1_SPEC>`"]
pub type CGH1 = crate::Reg<cgh1::CGH1_SPEC>;
#[doc = "CMT Carrier Generator High Data Register 1"]
pub mod cgh1;
#[doc = "CGL1 (rw) register accessor: an alias for `Reg<CGL1_SPEC>`"]
pub type CGL1 = crate::Reg<cgl1::CGL1_SPEC>;
#[doc = "CMT Carrier Generator Low Data Register 1"]
pub mod cgl1;
#[doc = "CGH2 (rw) register accessor: an alias for `Reg<CGH2_SPEC>`"]
pub type CGH2 = crate::Reg<cgh2::CGH2_SPEC>;
#[doc = "CMT Carrier Generator High Data Register 2"]
pub mod cgh2;
#[doc = "CGL2 (rw) register accessor: an alias for `Reg<CGL2_SPEC>`"]
pub type CGL2 = crate::Reg<cgl2::CGL2_SPEC>;
#[doc = "CMT Carrier Generator Low Data Register 2"]
pub mod cgl2;
#[doc = "OC (rw) register accessor: an alias for `Reg<OC_SPEC>`"]
pub type OC = crate::Reg<oc::OC_SPEC>;
#[doc = "CMT Output Control Register"]
pub mod oc;
#[doc = "MSC (rw) register accessor: an alias for `Reg<MSC_SPEC>`"]
pub type MSC = crate::Reg<msc::MSC_SPEC>;
#[doc = "CMT Modulator Status and Control Register"]
pub mod msc;
#[doc = "CMD1 (rw) register accessor: an alias for `Reg<CMD1_SPEC>`"]
pub type CMD1 = crate::Reg<cmd1::CMD1_SPEC>;
#[doc = "CMT Modulator Data Register Mark High"]
pub mod cmd1;
#[doc = "CMD2 (rw) register accessor: an alias for `Reg<CMD2_SPEC>`"]
pub type CMD2 = crate::Reg<cmd2::CMD2_SPEC>;
#[doc = "CMT Modulator Data Register Mark Low"]
pub mod cmd2;
#[doc = "CMD3 (rw) register accessor: an alias for `Reg<CMD3_SPEC>`"]
pub type CMD3 = crate::Reg<cmd3::CMD3_SPEC>;
#[doc = "CMT Modulator Data Register Space High"]
pub mod cmd3;
#[doc = "CMD4 (rw) register accessor: an alias for `Reg<CMD4_SPEC>`"]
pub type CMD4 = crate::Reg<cmd4::CMD4_SPEC>;
#[doc = "CMT Modulator Data Register Space Low"]
pub mod cmd4;
#[doc = "PPS (rw) register accessor: an alias for `Reg<PPS_SPEC>`"]
pub type PPS = crate::Reg<pps::PPS_SPEC>;
#[doc = "CMT Primary Prescaler Register"]
pub mod pps;
#[doc = "DMA (rw) register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "CMT Direct Memory Access Register"]
pub mod dma;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA System Address register"]
    pub dsaddr: DSADDR,
    #[doc = "0x04 - Block Attributes register"]
    pub blkattr: BLKATTR,
    #[doc = "0x08 - Command Argument register"]
    pub cmdarg: CMDARG,
    #[doc = "0x0c - Transfer Type register"]
    pub xfertyp: XFERTYP,
    #[doc = "0x10 - Command Response 0"]
    pub cmdrsp0: CMDRSP0,
    #[doc = "0x14 - Command Response 1"]
    pub cmdrsp1: CMDRSP1,
    #[doc = "0x18 - Command Response 2"]
    pub cmdrsp2: CMDRSP2,
    #[doc = "0x1c - Command Response 3"]
    pub cmdrsp3: CMDRSP3,
    #[doc = "0x20 - Buffer Data Port register"]
    pub datport: DATPORT,
    #[doc = "0x24 - Present State register"]
    pub prsstat: PRSSTAT,
    #[doc = "0x28 - Protocol Control register"]
    pub proctl: PROCTL,
    #[doc = "0x2c - System Control register"]
    pub sysctl: SYSCTL,
    #[doc = "0x30 - Interrupt Status register"]
    pub irqstat: IRQSTAT,
    #[doc = "0x34 - Interrupt Status Enable register"]
    pub irqstaten: IRQSTATEN,
    #[doc = "0x38 - Interrupt Signal Enable register"]
    pub irqsigen: IRQSIGEN,
    #[doc = "0x3c - Auto CMD12 Error Status Register"]
    pub ac12err: AC12ERR,
    #[doc = "0x40 - Host Controller Capabilities"]
    pub htcapblt: HTCAPBLT,
    #[doc = "0x44 - Watermark Level Register"]
    pub wml: WML,
    _reserved18: [u8; 0x08],
    #[doc = "0x50 - Force Event register"]
    pub fevt: FEVT,
    #[doc = "0x54 - ADMA Error Status register"]
    pub admaes: ADMAES,
    #[doc = "0x58 - ADMA System Addressregister"]
    pub adsaddr: ADSADDR,
    _reserved21: [u8; 0x64],
    #[doc = "0xc0 - Vendor Specific register"]
    pub vendor: VENDOR,
    #[doc = "0xc4 - MMC Boot register"]
    pub mmcboot: MMCBOOT,
    _reserved23: [u8; 0x34],
    #[doc = "0xfc - Host Controller Version"]
    pub hostver: HOSTVER,
}
#[doc = "DSADDR (rw) register accessor: an alias for `Reg<DSADDR_SPEC>`"]
pub type DSADDR = crate::Reg<dsaddr::DSADDR_SPEC>;
#[doc = "DMA System Address register"]
pub mod dsaddr;
#[doc = "BLKATTR (rw) register accessor: an alias for `Reg<BLKATTR_SPEC>`"]
pub type BLKATTR = crate::Reg<blkattr::BLKATTR_SPEC>;
#[doc = "Block Attributes register"]
pub mod blkattr;
#[doc = "CMDARG (rw) register accessor: an alias for `Reg<CMDARG_SPEC>`"]
pub type CMDARG = crate::Reg<cmdarg::CMDARG_SPEC>;
#[doc = "Command Argument register"]
pub mod cmdarg;
#[doc = "XFERTYP (rw) register accessor: an alias for `Reg<XFERTYP_SPEC>`"]
pub type XFERTYP = crate::Reg<xfertyp::XFERTYP_SPEC>;
#[doc = "Transfer Type register"]
pub mod xfertyp;
#[doc = "CMDRSP0 (r) register accessor: an alias for `Reg<CMDRSP0_SPEC>`"]
pub type CMDRSP0 = crate::Reg<cmdrsp0::CMDRSP0_SPEC>;
#[doc = "Command Response 0"]
pub mod cmdrsp0;
#[doc = "CMDRSP1 (r) register accessor: an alias for `Reg<CMDRSP1_SPEC>`"]
pub type CMDRSP1 = crate::Reg<cmdrsp1::CMDRSP1_SPEC>;
#[doc = "Command Response 1"]
pub mod cmdrsp1;
#[doc = "CMDRSP2 (r) register accessor: an alias for `Reg<CMDRSP2_SPEC>`"]
pub type CMDRSP2 = crate::Reg<cmdrsp2::CMDRSP2_SPEC>;
#[doc = "Command Response 2"]
pub mod cmdrsp2;
#[doc = "CMDRSP3 (r) register accessor: an alias for `Reg<CMDRSP3_SPEC>`"]
pub type CMDRSP3 = crate::Reg<cmdrsp3::CMDRSP3_SPEC>;
#[doc = "Command Response 3"]
pub mod cmdrsp3;
#[doc = "DATPORT (rw) register accessor: an alias for `Reg<DATPORT_SPEC>`"]
pub type DATPORT = crate::Reg<datport::DATPORT_SPEC>;
#[doc = "Buffer Data Port register"]
pub mod datport;
#[doc = "PRSSTAT (r) register accessor: an alias for `Reg<PRSSTAT_SPEC>`"]
pub type PRSSTAT = crate::Reg<prsstat::PRSSTAT_SPEC>;
#[doc = "Present State register"]
pub mod prsstat;
#[doc = "PROCTL (rw) register accessor: an alias for `Reg<PROCTL_SPEC>`"]
pub type PROCTL = crate::Reg<proctl::PROCTL_SPEC>;
#[doc = "Protocol Control register"]
pub mod proctl;
#[doc = "SYSCTL (rw) register accessor: an alias for `Reg<SYSCTL_SPEC>`"]
pub type SYSCTL = crate::Reg<sysctl::SYSCTL_SPEC>;
#[doc = "System Control register"]
pub mod sysctl;
#[doc = "IRQSTAT (rw) register accessor: an alias for `Reg<IRQSTAT_SPEC>`"]
pub type IRQSTAT = crate::Reg<irqstat::IRQSTAT_SPEC>;
#[doc = "Interrupt Status register"]
pub mod irqstat;
#[doc = "IRQSTATEN (rw) register accessor: an alias for `Reg<IRQSTATEN_SPEC>`"]
pub type IRQSTATEN = crate::Reg<irqstaten::IRQSTATEN_SPEC>;
#[doc = "Interrupt Status Enable register"]
pub mod irqstaten;
#[doc = "IRQSIGEN (rw) register accessor: an alias for `Reg<IRQSIGEN_SPEC>`"]
pub type IRQSIGEN = crate::Reg<irqsigen::IRQSIGEN_SPEC>;
#[doc = "Interrupt Signal Enable register"]
pub mod irqsigen;
#[doc = "AC12ERR (r) register accessor: an alias for `Reg<AC12ERR_SPEC>`"]
pub type AC12ERR = crate::Reg<ac12err::AC12ERR_SPEC>;
#[doc = "Auto CMD12 Error Status Register"]
pub mod ac12err;
#[doc = "HTCAPBLT (r) register accessor: an alias for `Reg<HTCAPBLT_SPEC>`"]
pub type HTCAPBLT = crate::Reg<htcapblt::HTCAPBLT_SPEC>;
#[doc = "Host Controller Capabilities"]
pub mod htcapblt;
#[doc = "WML (rw) register accessor: an alias for `Reg<WML_SPEC>`"]
pub type WML = crate::Reg<wml::WML_SPEC>;
#[doc = "Watermark Level Register"]
pub mod wml;
#[doc = "FEVT (w) register accessor: an alias for `Reg<FEVT_SPEC>`"]
pub type FEVT = crate::Reg<fevt::FEVT_SPEC>;
#[doc = "Force Event register"]
pub mod fevt;
#[doc = "ADMAES (r) register accessor: an alias for `Reg<ADMAES_SPEC>`"]
pub type ADMAES = crate::Reg<admaes::ADMAES_SPEC>;
#[doc = "ADMA Error Status register"]
pub mod admaes;
#[doc = "ADSADDR (rw) register accessor: an alias for `Reg<ADSADDR_SPEC>`"]
pub type ADSADDR = crate::Reg<adsaddr::ADSADDR_SPEC>;
#[doc = "ADMA System Addressregister"]
pub mod adsaddr;
#[doc = "VENDOR (rw) register accessor: an alias for `Reg<VENDOR_SPEC>`"]
pub type VENDOR = crate::Reg<vendor::VENDOR_SPEC>;
#[doc = "Vendor Specific register"]
pub mod vendor;
#[doc = "MMCBOOT (rw) register accessor: an alias for `Reg<MMCBOOT_SPEC>`"]
pub type MMCBOOT = crate::Reg<mmcboot::MMCBOOT_SPEC>;
#[doc = "MMC Boot register"]
pub mod mmcboot;
#[doc = "HOSTVER (r) register accessor: an alias for `Reg<HOSTVER_SPEC>`"]
pub type HOSTVER = crate::Reg<hostver::HOSTVER_SPEC>;
#[doc = "Host Controller Version"]
pub mod hostver;

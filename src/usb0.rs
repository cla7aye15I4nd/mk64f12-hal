#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral ID register"]
    pub perid: PERID,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Peripheral ID Complement register"]
    pub idcomp: IDCOMP,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - Peripheral Revision register"]
    pub rev: REV,
    _reserved3: [u8; 0x03],
    #[doc = "0x0c - Peripheral Additional Info register"]
    pub addinfo: ADDINFO,
    _reserved4: [u8; 0x03],
    #[doc = "0x10 - OTG Interrupt Status register"]
    pub otgistat: OTGISTAT,
    _reserved5: [u8; 0x03],
    #[doc = "0x14 - OTG Interrupt Control register"]
    pub otgicr: OTGICR,
    _reserved6: [u8; 0x03],
    #[doc = "0x18 - OTG Status register"]
    pub otgstat: OTGSTAT,
    _reserved7: [u8; 0x03],
    #[doc = "0x1c - OTG Control register"]
    pub otgctl: OTGCTL,
    _reserved8: [u8; 0x63],
    #[doc = "0x80 - Interrupt Status register"]
    pub istat: ISTAT,
    _reserved9: [u8; 0x03],
    #[doc = "0x84 - Interrupt Enable register"]
    pub inten: INTEN,
    _reserved10: [u8; 0x03],
    #[doc = "0x88 - Error Interrupt Status register"]
    pub errstat: ERRSTAT,
    _reserved11: [u8; 0x03],
    #[doc = "0x8c - Error Interrupt Enable register"]
    pub erren: ERREN,
    _reserved12: [u8; 0x03],
    #[doc = "0x90 - Status register"]
    pub stat: STAT,
    _reserved13: [u8; 0x03],
    #[doc = "0x94 - Control register"]
    pub ctl: CTL,
    _reserved14: [u8; 0x03],
    #[doc = "0x98 - Address register"]
    pub addr: ADDR,
    _reserved15: [u8; 0x03],
    #[doc = "0x9c - BDT Page register 1"]
    pub bdtpage1: BDTPAGE1,
    _reserved16: [u8; 0x03],
    #[doc = "0xa0 - Frame Number register Low"]
    pub frmnuml: FRMNUML,
    _reserved17: [u8; 0x03],
    #[doc = "0xa4 - Frame Number register High"]
    pub frmnumh: FRMNUMH,
    _reserved18: [u8; 0x03],
    #[doc = "0xa8 - Token register"]
    pub token: TOKEN,
    _reserved19: [u8; 0x03],
    #[doc = "0xac - SOF Threshold register"]
    pub softhld: SOFTHLD,
    _reserved20: [u8; 0x03],
    #[doc = "0xb0 - BDT Page Register 2"]
    pub bdtpage2: BDTPAGE2,
    _reserved21: [u8; 0x03],
    #[doc = "0xb4 - BDT Page Register 3"]
    pub bdtpage3: BDTPAGE3,
    _reserved22: [u8; 0x0b],
    #[doc = "0xc0 - Endpoint Control register"]
    pub endpt0: ENDPT,
    _reserved23: [u8; 0x03],
    #[doc = "0xc4 - Endpoint Control register"]
    pub endpt1: ENDPT,
    _reserved24: [u8; 0x03],
    #[doc = "0xc8 - Endpoint Control register"]
    pub endpt2: ENDPT,
    _reserved25: [u8; 0x03],
    #[doc = "0xcc - Endpoint Control register"]
    pub endpt3: ENDPT,
    _reserved26: [u8; 0x03],
    #[doc = "0xd0 - Endpoint Control register"]
    pub endpt4: ENDPT,
    _reserved27: [u8; 0x03],
    #[doc = "0xd4 - Endpoint Control register"]
    pub endpt5: ENDPT,
    _reserved28: [u8; 0x03],
    #[doc = "0xd8 - Endpoint Control register"]
    pub endpt6: ENDPT,
    _reserved29: [u8; 0x03],
    #[doc = "0xdc - Endpoint Control register"]
    pub endpt7: ENDPT,
    _reserved30: [u8; 0x03],
    #[doc = "0xe0 - Endpoint Control register"]
    pub endpt8: ENDPT,
    _reserved31: [u8; 0x03],
    #[doc = "0xe4 - Endpoint Control register"]
    pub endpt9: ENDPT,
    _reserved32: [u8; 0x03],
    #[doc = "0xe8 - Endpoint Control register"]
    pub endpt10: ENDPT,
    _reserved33: [u8; 0x03],
    #[doc = "0xec - Endpoint Control register"]
    pub endpt11: ENDPT,
    _reserved34: [u8; 0x03],
    #[doc = "0xf0 - Endpoint Control register"]
    pub endpt12: ENDPT,
    _reserved35: [u8; 0x03],
    #[doc = "0xf4 - Endpoint Control register"]
    pub endpt13: ENDPT,
    _reserved36: [u8; 0x03],
    #[doc = "0xf8 - Endpoint Control register"]
    pub endpt14: ENDPT,
    _reserved37: [u8; 0x03],
    #[doc = "0xfc - Endpoint Control register"]
    pub endpt15: ENDPT,
    _reserved38: [u8; 0x03],
    #[doc = "0x100 - USB Control register"]
    pub usbctrl: USBCTRL,
    _reserved39: [u8; 0x03],
    #[doc = "0x104 - USB OTG Observe register"]
    pub observe: OBSERVE,
    _reserved40: [u8; 0x03],
    #[doc = "0x108 - USB OTG Control register"]
    pub control: CONTROL,
    _reserved41: [u8; 0x03],
    #[doc = "0x10c - USB Transceiver Control register 0"]
    pub usbtrc0: USBTRC0,
    _reserved42: [u8; 0x07],
    #[doc = "0x114 - Frame Adjust Register"]
    pub usbfrmadjust: USBFRMADJUST,
    _reserved43: [u8; 0x2b],
    #[doc = "0x140 - USB Clock recovery control"]
    pub clk_recover_ctrl: CLK_RECOVER_CTRL,
    _reserved44: [u8; 0x03],
    #[doc = "0x144 - IRC48M oscillator enable register"]
    pub clk_recover_irc_en: CLK_RECOVER_IRC_EN,
    _reserved45: [u8; 0x17],
    #[doc = "0x15c - Clock recovery separated interrupt status"]
    pub clk_recover_int_status: CLK_RECOVER_INT_STATUS,
}
#[doc = "PERID (r) register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID register"]
pub mod perid;
#[doc = "IDCOMP (r) register accessor: an alias for `Reg<IDCOMP_SPEC>`"]
pub type IDCOMP = crate::Reg<idcomp::IDCOMP_SPEC>;
#[doc = "Peripheral ID Complement register"]
pub mod idcomp;
#[doc = "REV (r) register accessor: an alias for `Reg<REV_SPEC>`"]
pub type REV = crate::Reg<rev::REV_SPEC>;
#[doc = "Peripheral Revision register"]
pub mod rev;
#[doc = "ADDINFO (r) register accessor: an alias for `Reg<ADDINFO_SPEC>`"]
pub type ADDINFO = crate::Reg<addinfo::ADDINFO_SPEC>;
#[doc = "Peripheral Additional Info register"]
pub mod addinfo;
#[doc = "OTGISTAT (rw) register accessor: an alias for `Reg<OTGISTAT_SPEC>`"]
pub type OTGISTAT = crate::Reg<otgistat::OTGISTAT_SPEC>;
#[doc = "OTG Interrupt Status register"]
pub mod otgistat;
#[doc = "OTGICR (rw) register accessor: an alias for `Reg<OTGICR_SPEC>`"]
pub type OTGICR = crate::Reg<otgicr::OTGICR_SPEC>;
#[doc = "OTG Interrupt Control register"]
pub mod otgicr;
#[doc = "OTGSTAT (rw) register accessor: an alias for `Reg<OTGSTAT_SPEC>`"]
pub type OTGSTAT = crate::Reg<otgstat::OTGSTAT_SPEC>;
#[doc = "OTG Status register"]
pub mod otgstat;
#[doc = "OTGCTL (rw) register accessor: an alias for `Reg<OTGCTL_SPEC>`"]
pub type OTGCTL = crate::Reg<otgctl::OTGCTL_SPEC>;
#[doc = "OTG Control register"]
pub mod otgctl;
#[doc = "ISTAT (rw) register accessor: an alias for `Reg<ISTAT_SPEC>`"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "Interrupt Status register"]
pub mod istat;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable register"]
pub mod inten;
#[doc = "ERRSTAT (rw) register accessor: an alias for `Reg<ERRSTAT_SPEC>`"]
pub type ERRSTAT = crate::Reg<errstat::ERRSTAT_SPEC>;
#[doc = "Error Interrupt Status register"]
pub mod errstat;
#[doc = "ERREN (rw) register accessor: an alias for `Reg<ERREN_SPEC>`"]
pub type ERREN = crate::Reg<erren::ERREN_SPEC>;
#[doc = "Error Interrupt Enable register"]
pub mod erren;
#[doc = "STAT (r) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address register"]
pub mod addr;
#[doc = "BDTPAGE1 (rw) register accessor: an alias for `Reg<BDTPAGE1_SPEC>`"]
pub type BDTPAGE1 = crate::Reg<bdtpage1::BDTPAGE1_SPEC>;
#[doc = "BDT Page register 1"]
pub mod bdtpage1;
#[doc = "FRMNUML (rw) register accessor: an alias for `Reg<FRMNUML_SPEC>`"]
pub type FRMNUML = crate::Reg<frmnuml::FRMNUML_SPEC>;
#[doc = "Frame Number register Low"]
pub mod frmnuml;
#[doc = "FRMNUMH (rw) register accessor: an alias for `Reg<FRMNUMH_SPEC>`"]
pub type FRMNUMH = crate::Reg<frmnumh::FRMNUMH_SPEC>;
#[doc = "Frame Number register High"]
pub mod frmnumh;
#[doc = "TOKEN (rw) register accessor: an alias for `Reg<TOKEN_SPEC>`"]
pub type TOKEN = crate::Reg<token::TOKEN_SPEC>;
#[doc = "Token register"]
pub mod token;
#[doc = "SOFTHLD (rw) register accessor: an alias for `Reg<SOFTHLD_SPEC>`"]
pub type SOFTHLD = crate::Reg<softhld::SOFTHLD_SPEC>;
#[doc = "SOF Threshold register"]
pub mod softhld;
#[doc = "BDTPAGE2 (rw) register accessor: an alias for `Reg<BDTPAGE2_SPEC>`"]
pub type BDTPAGE2 = crate::Reg<bdtpage2::BDTPAGE2_SPEC>;
#[doc = "BDT Page Register 2"]
pub mod bdtpage2;
#[doc = "BDTPAGE3 (rw) register accessor: an alias for `Reg<BDTPAGE3_SPEC>`"]
pub type BDTPAGE3 = crate::Reg<bdtpage3::BDTPAGE3_SPEC>;
#[doc = "BDT Page Register 3"]
pub mod bdtpage3;
#[doc = "ENDPT (rw) register accessor: an alias for `Reg<ENDPT_SPEC>`"]
pub type ENDPT = crate::Reg<endpt::ENDPT_SPEC>;
#[doc = "Endpoint Control register"]
pub mod endpt;
#[doc = "USBCTRL (rw) register accessor: an alias for `Reg<USBCTRL_SPEC>`"]
pub type USBCTRL = crate::Reg<usbctrl::USBCTRL_SPEC>;
#[doc = "USB Control register"]
pub mod usbctrl;
#[doc = "OBSERVE (r) register accessor: an alias for `Reg<OBSERVE_SPEC>`"]
pub type OBSERVE = crate::Reg<observe::OBSERVE_SPEC>;
#[doc = "USB OTG Observe register"]
pub mod observe;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "USB OTG Control register"]
pub mod control;
#[doc = "USBTRC0 (rw) register accessor: an alias for `Reg<USBTRC0_SPEC>`"]
pub type USBTRC0 = crate::Reg<usbtrc0::USBTRC0_SPEC>;
#[doc = "USB Transceiver Control register 0"]
pub mod usbtrc0;
#[doc = "USBFRMADJUST (rw) register accessor: an alias for `Reg<USBFRMADJUST_SPEC>`"]
pub type USBFRMADJUST = crate::Reg<usbfrmadjust::USBFRMADJUST_SPEC>;
#[doc = "Frame Adjust Register"]
pub mod usbfrmadjust;
#[doc = "CLK_RECOVER_CTRL (rw) register accessor: an alias for `Reg<CLK_RECOVER_CTRL_SPEC>`"]
pub type CLK_RECOVER_CTRL = crate::Reg<clk_recover_ctrl::CLK_RECOVER_CTRL_SPEC>;
#[doc = "USB Clock recovery control"]
pub mod clk_recover_ctrl;
#[doc = "CLK_RECOVER_IRC_EN (rw) register accessor: an alias for `Reg<CLK_RECOVER_IRC_EN_SPEC>`"]
pub type CLK_RECOVER_IRC_EN = crate::Reg<clk_recover_irc_en::CLK_RECOVER_IRC_EN_SPEC>;
#[doc = "IRC48M oscillator enable register"]
pub mod clk_recover_irc_en;
#[doc = "CLK_RECOVER_INT_STATUS (rw) register accessor: an alias for `Reg<CLK_RECOVER_INT_STATUS_SPEC>`"]
pub type CLK_RECOVER_INT_STATUS = crate::Reg<clk_recover_int_status::CLK_RECOVER_INT_STATUS_SPEC>;
#[doc = "Clock recovery separated interrupt status"]
pub mod clk_recover_int_status;

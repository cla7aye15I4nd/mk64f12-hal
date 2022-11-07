#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status And Control"]
    pub sc: SC,
    #[doc = "0x04 - Counter"]
    pub cnt: CNT,
    #[doc = "0x08 - Modulo"]
    pub mod_: MOD,
    #[doc = "0x0c - Channel (n) Status And Control"]
    pub c0sc: CSC,
    #[doc = "0x10 - Channel (n) Value"]
    pub c0v: CV,
    #[doc = "0x14 - Channel (n) Status And Control"]
    pub c1sc: CSC,
    #[doc = "0x18 - Channel (n) Value"]
    pub c1v: CV,
    #[doc = "0x1c - Channel (n) Status And Control"]
    pub c2sc: CSC,
    #[doc = "0x20 - Channel (n) Value"]
    pub c2v: CV,
    #[doc = "0x24 - Channel (n) Status And Control"]
    pub c3sc: CSC,
    #[doc = "0x28 - Channel (n) Value"]
    pub c3v: CV,
    #[doc = "0x2c - Channel (n) Status And Control"]
    pub c4sc: CSC,
    #[doc = "0x30 - Channel (n) Value"]
    pub c4v: CV,
    #[doc = "0x34 - Channel (n) Status And Control"]
    pub c5sc: CSC,
    #[doc = "0x38 - Channel (n) Value"]
    pub c5v: CV,
    #[doc = "0x3c - Channel (n) Status And Control"]
    pub c6sc: CSC,
    #[doc = "0x40 - Channel (n) Value"]
    pub c6v: CV,
    #[doc = "0x44 - Channel (n) Status And Control"]
    pub c7sc: CSC,
    #[doc = "0x48 - Channel (n) Value"]
    pub c7v: CV,
    #[doc = "0x4c - Counter Initial Value"]
    pub cntin: CNTIN,
    #[doc = "0x50 - Capture And Compare Status"]
    pub status: STATUS,
    #[doc = "0x54 - Features Mode Selection"]
    pub mode: MODE,
    #[doc = "0x58 - Synchronization"]
    pub sync: SYNC,
    #[doc = "0x5c - Initial State For Channels Output"]
    pub outinit: OUTINIT,
    #[doc = "0x60 - Output Mask"]
    pub outmask: OUTMASK,
    #[doc = "0x64 - Function For Linked Channels"]
    pub combine: COMBINE,
    #[doc = "0x68 - Deadtime Insertion Control"]
    pub deadtime: DEADTIME,
    #[doc = "0x6c - FTM External Trigger"]
    pub exttrig: EXTTRIG,
    #[doc = "0x70 - Channels Polarity"]
    pub pol: POL,
    #[doc = "0x74 - Fault Mode Status"]
    pub fms: FMS,
    #[doc = "0x78 - Input Capture Filter Control"]
    pub filter: FILTER,
    #[doc = "0x7c - Fault Control"]
    pub fltctrl: FLTCTRL,
    #[doc = "0x80 - Quadrature Decoder Control And Status"]
    pub qdctrl: QDCTRL,
    #[doc = "0x84 - Configuration"]
    pub conf: CONF,
    #[doc = "0x88 - FTM Fault Input Polarity"]
    pub fltpol: FLTPOL,
    #[doc = "0x8c - Synchronization Configuration"]
    pub synconf: SYNCONF,
    #[doc = "0x90 - FTM Inverting Control"]
    pub invctrl: INVCTRL,
    #[doc = "0x94 - FTM Software Output Control"]
    pub swoctrl: SWOCTRL,
    #[doc = "0x98 - FTM PWM Load"]
    pub pwmload: PWMLOAD,
}
#[doc = "SC (rw) register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "Status And Control"]
pub mod sc;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "MOD (rw) register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Modulo"]
pub mod mod_;
#[doc = "CSC (rw) register accessor: an alias for `Reg<CSC_SPEC>`"]
pub type CSC = crate::Reg<csc::CSC_SPEC>;
#[doc = "Channel (n) Status And Control"]
pub mod csc;
#[doc = "CV (rw) register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Channel (n) Value"]
pub mod cv;
#[doc = "CNTIN (rw) register accessor: an alias for `Reg<CNTIN_SPEC>`"]
pub type CNTIN = crate::Reg<cntin::CNTIN_SPEC>;
#[doc = "Counter Initial Value"]
pub mod cntin;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Capture And Compare Status"]
pub mod status;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Features Mode Selection"]
pub mod mode;
#[doc = "SYNC (rw) register accessor: an alias for `Reg<SYNC_SPEC>`"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "Synchronization"]
pub mod sync;
#[doc = "OUTINIT (rw) register accessor: an alias for `Reg<OUTINIT_SPEC>`"]
pub type OUTINIT = crate::Reg<outinit::OUTINIT_SPEC>;
#[doc = "Initial State For Channels Output"]
pub mod outinit;
#[doc = "OUTMASK (rw) register accessor: an alias for `Reg<OUTMASK_SPEC>`"]
pub type OUTMASK = crate::Reg<outmask::OUTMASK_SPEC>;
#[doc = "Output Mask"]
pub mod outmask;
#[doc = "COMBINE (rw) register accessor: an alias for `Reg<COMBINE_SPEC>`"]
pub type COMBINE = crate::Reg<combine::COMBINE_SPEC>;
#[doc = "Function For Linked Channels"]
pub mod combine;
#[doc = "DEADTIME (rw) register accessor: an alias for `Reg<DEADTIME_SPEC>`"]
pub type DEADTIME = crate::Reg<deadtime::DEADTIME_SPEC>;
#[doc = "Deadtime Insertion Control"]
pub mod deadtime;
#[doc = "EXTTRIG (rw) register accessor: an alias for `Reg<EXTTRIG_SPEC>`"]
pub type EXTTRIG = crate::Reg<exttrig::EXTTRIG_SPEC>;
#[doc = "FTM External Trigger"]
pub mod exttrig;
#[doc = "POL (rw) register accessor: an alias for `Reg<POL_SPEC>`"]
pub type POL = crate::Reg<pol::POL_SPEC>;
#[doc = "Channels Polarity"]
pub mod pol;
#[doc = "FMS (rw) register accessor: an alias for `Reg<FMS_SPEC>`"]
pub type FMS = crate::Reg<fms::FMS_SPEC>;
#[doc = "Fault Mode Status"]
pub mod fms;
#[doc = "FILTER (rw) register accessor: an alias for `Reg<FILTER_SPEC>`"]
pub type FILTER = crate::Reg<filter::FILTER_SPEC>;
#[doc = "Input Capture Filter Control"]
pub mod filter;
#[doc = "FLTCTRL (rw) register accessor: an alias for `Reg<FLTCTRL_SPEC>`"]
pub type FLTCTRL = crate::Reg<fltctrl::FLTCTRL_SPEC>;
#[doc = "Fault Control"]
pub mod fltctrl;
#[doc = "QDCTRL (rw) register accessor: an alias for `Reg<QDCTRL_SPEC>`"]
pub type QDCTRL = crate::Reg<qdctrl::QDCTRL_SPEC>;
#[doc = "Quadrature Decoder Control And Status"]
pub mod qdctrl;
#[doc = "CONF (rw) register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configuration"]
pub mod conf;
#[doc = "FLTPOL (rw) register accessor: an alias for `Reg<FLTPOL_SPEC>`"]
pub type FLTPOL = crate::Reg<fltpol::FLTPOL_SPEC>;
#[doc = "FTM Fault Input Polarity"]
pub mod fltpol;
#[doc = "SYNCONF (rw) register accessor: an alias for `Reg<SYNCONF_SPEC>`"]
pub type SYNCONF = crate::Reg<synconf::SYNCONF_SPEC>;
#[doc = "Synchronization Configuration"]
pub mod synconf;
#[doc = "INVCTRL (rw) register accessor: an alias for `Reg<INVCTRL_SPEC>`"]
pub type INVCTRL = crate::Reg<invctrl::INVCTRL_SPEC>;
#[doc = "FTM Inverting Control"]
pub mod invctrl;
#[doc = "SWOCTRL (rw) register accessor: an alias for `Reg<SWOCTRL_SPEC>`"]
pub type SWOCTRL = crate::Reg<swoctrl::SWOCTRL_SPEC>;
#[doc = "FTM Software Output Control"]
pub mod swoctrl;
#[doc = "PWMLOAD (rw) register accessor: an alias for `Reg<PWMLOAD_SPEC>`"]
pub type PWMLOAD = crate::Reg<pwmload::PWMLOAD_SPEC>;
#[doc = "FTM PWM Load"]
pub mod pwmload;

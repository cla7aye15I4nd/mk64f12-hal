#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Status and Control Register High"]
    pub stctrlh: STCTRLH,
    #[doc = "0x02 - Watchdog Status and Control Register Low"]
    pub stctrll: STCTRLL,
    #[doc = "0x04 - Watchdog Time-out Value Register High"]
    pub tovalh: TOVALH,
    #[doc = "0x06 - Watchdog Time-out Value Register Low"]
    pub tovall: TOVALL,
    #[doc = "0x08 - Watchdog Window Register High"]
    pub winh: WINH,
    #[doc = "0x0a - Watchdog Window Register Low"]
    pub winl: WINL,
    #[doc = "0x0c - Watchdog Refresh register"]
    pub refresh: REFRESH,
    #[doc = "0x0e - Watchdog Unlock register"]
    pub unlock: UNLOCK,
    #[doc = "0x10 - Watchdog Timer Output Register High"]
    pub tmrouth: TMROUTH,
    #[doc = "0x12 - Watchdog Timer Output Register Low"]
    pub tmroutl: TMROUTL,
    #[doc = "0x14 - Watchdog Reset Count register"]
    pub rstcnt: RSTCNT,
    #[doc = "0x16 - Watchdog Prescaler register"]
    pub presc: PRESC,
}
#[doc = "STCTRLH (rw) register accessor: an alias for `Reg<STCTRLH_SPEC>`"]
pub type STCTRLH = crate::Reg<stctrlh::STCTRLH_SPEC>;
#[doc = "Watchdog Status and Control Register High"]
pub mod stctrlh;
#[doc = "STCTRLL (rw) register accessor: an alias for `Reg<STCTRLL_SPEC>`"]
pub type STCTRLL = crate::Reg<stctrll::STCTRLL_SPEC>;
#[doc = "Watchdog Status and Control Register Low"]
pub mod stctrll;
#[doc = "TOVALH (rw) register accessor: an alias for `Reg<TOVALH_SPEC>`"]
pub type TOVALH = crate::Reg<tovalh::TOVALH_SPEC>;
#[doc = "Watchdog Time-out Value Register High"]
pub mod tovalh;
#[doc = "TOVALL (rw) register accessor: an alias for `Reg<TOVALL_SPEC>`"]
pub type TOVALL = crate::Reg<tovall::TOVALL_SPEC>;
#[doc = "Watchdog Time-out Value Register Low"]
pub mod tovall;
#[doc = "WINH (rw) register accessor: an alias for `Reg<WINH_SPEC>`"]
pub type WINH = crate::Reg<winh::WINH_SPEC>;
#[doc = "Watchdog Window Register High"]
pub mod winh;
#[doc = "WINL (rw) register accessor: an alias for `Reg<WINL_SPEC>`"]
pub type WINL = crate::Reg<winl::WINL_SPEC>;
#[doc = "Watchdog Window Register Low"]
pub mod winl;
#[doc = "REFRESH (rw) register accessor: an alias for `Reg<REFRESH_SPEC>`"]
pub type REFRESH = crate::Reg<refresh::REFRESH_SPEC>;
#[doc = "Watchdog Refresh register"]
pub mod refresh;
#[doc = "UNLOCK (rw) register accessor: an alias for `Reg<UNLOCK_SPEC>`"]
pub type UNLOCK = crate::Reg<unlock::UNLOCK_SPEC>;
#[doc = "Watchdog Unlock register"]
pub mod unlock;
#[doc = "TMROUTH (rw) register accessor: an alias for `Reg<TMROUTH_SPEC>`"]
pub type TMROUTH = crate::Reg<tmrouth::TMROUTH_SPEC>;
#[doc = "Watchdog Timer Output Register High"]
pub mod tmrouth;
#[doc = "TMROUTL (rw) register accessor: an alias for `Reg<TMROUTL_SPEC>`"]
pub type TMROUTL = crate::Reg<tmroutl::TMROUTL_SPEC>;
#[doc = "Watchdog Timer Output Register Low"]
pub mod tmroutl;
#[doc = "RSTCNT (rw) register accessor: an alias for `Reg<RSTCNT_SPEC>`"]
pub type RSTCNT = crate::Reg<rstcnt::RSTCNT_SPEC>;
#[doc = "Watchdog Reset Count register"]
pub mod rstcnt;
#[doc = "PRESC (rw) register accessor: an alias for `Reg<PRESC_SPEC>`"]
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
#[doc = "Watchdog Prescaler register"]
pub mod presc;

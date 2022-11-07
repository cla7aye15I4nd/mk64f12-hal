#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub control: CONTROL,
    #[doc = "0x04 - Clock register"]
    pub clock: CLOCK,
    #[doc = "0x08 - Status register"]
    pub status: STATUS,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - TIMER0 register"]
    pub timer0: TIMER0,
    #[doc = "0x14 - TIMER1 register"]
    pub timer1: TIMER1,
    _reserved_5_usbdcd_timer2_bc1: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x18 - TIMER2_BC12 register"]
    #[inline(always)]
    pub const fn usbdcd_timer2_bc12(&self) -> &USBDCD_TIMER2_BC12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - TIMER2_BC11 register"]
    #[inline(always)]
    pub const fn usbdcd_timer2_bc11(&self) -> &USBDCD_TIMER2_BC11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
}
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control register"]
pub mod control;
#[doc = "CLOCK (rw) register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "Clock register"]
pub mod clock;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "TIMER0 (rw) register accessor: an alias for `Reg<TIMER0_SPEC>`"]
pub type TIMER0 = crate::Reg<timer0::TIMER0_SPEC>;
#[doc = "TIMER0 register"]
pub mod timer0;
#[doc = "TIMER1 (rw) register accessor: an alias for `Reg<TIMER1_SPEC>`"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "TIMER1 register"]
pub mod timer1;
#[doc = "USBDCD_TIMER2_BC11 (rw) register accessor: an alias for `Reg<USBDCD_TIMER2_BC11_SPEC>`"]
pub type USBDCD_TIMER2_BC11 = crate::Reg<usbdcd_timer2_bc11::USBDCD_TIMER2_BC11_SPEC>;
#[doc = "TIMER2_BC11 register"]
pub mod usbdcd_timer2_bc11;
#[doc = "USBDCD_TIMER2_BC12 (rw) register accessor: an alias for `Reg<USBDCD_TIMER2_BC12_SPEC>`"]
pub type USBDCD_TIMER2_BC12 = crate::Reg<usbdcd_timer2_bc12::USBDCD_TIMER2_BC12_SPEC>;
#[doc = "TIMER2_BC12 register"]
pub mod usbdcd_timer2_bc12;

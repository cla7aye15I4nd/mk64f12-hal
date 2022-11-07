#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_datahu: [u8; 0x04],
    _reserved_1_gpolyhu: [u8; 0x04],
    _reserved_2_ctrl: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - CRC_DATALL register."]
    #[inline(always)]
    pub const fn crc_datall(&self) -> &CRC_DATALL {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - CRC_DATAL register."]
    #[inline(always)]
    pub const fn crc_datal(&self) -> &CRC_DATAL {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - CRC Data register"]
    #[inline(always)]
    pub const fn crc_data(&self) -> &CRC_DATA {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x01 - CRC_DATALU register."]
    #[inline(always)]
    pub const fn datalu(&self) -> &DATALU {
        unsafe { &*(self as *const Self).cast::<u8>().add(1usize).cast() }
    }
    #[doc = "0x02 - CRC_DATAHL register."]
    #[inline(always)]
    pub const fn crc_datahl(&self) -> &CRC_DATAHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x02 - CRC_DATAH register."]
    #[inline(always)]
    pub const fn crc_datah(&self) -> &CRC_DATAH {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x03 - CRC_DATAHU register."]
    #[inline(always)]
    pub const fn datahu(&self) -> &DATAHU {
        unsafe { &*(self as *const Self).cast::<u8>().add(3usize).cast() }
    }
    #[doc = "0x04 - CRC_GPOLYLL register."]
    #[inline(always)]
    pub const fn crc_gpolyll(&self) -> &CRC_GPOLYLL {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - CRC_GPOLYL register."]
    #[inline(always)]
    pub const fn crc_gpolyl(&self) -> &CRC_GPOLYL {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - CRC Polynomial register"]
    #[inline(always)]
    pub const fn crc_gpoly(&self) -> &CRC_GPOLY {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x05 - CRC_GPOLYLU register."]
    #[inline(always)]
    pub const fn gpolylu(&self) -> &GPOLYLU {
        unsafe { &*(self as *const Self).cast::<u8>().add(5usize).cast() }
    }
    #[doc = "0x06 - CRC_GPOLYHL register."]
    #[inline(always)]
    pub const fn crc_gpolyhl(&self) -> &CRC_GPOLYHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x06 - CRC_GPOLYH register."]
    #[inline(always)]
    pub const fn crc_gpolyh(&self) -> &CRC_GPOLYH {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x07 - CRC_GPOLYHU register."]
    #[inline(always)]
    pub const fn gpolyhu(&self) -> &GPOLYHU {
        unsafe { &*(self as *const Self).cast::<u8>().add(7usize).cast() }
    }
    #[doc = "0x08 - CRC Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x0b - CRC_CTRLHU register."]
    #[inline(always)]
    pub const fn ctrlhu(&self) -> &CTRLHU {
        unsafe { &*(self as *const Self).cast::<u8>().add(11usize).cast() }
    }
}
#[doc = "CRC_DATA (rw) register accessor: an alias for `Reg<CRC_DATA_SPEC>`"]
pub type CRC_DATA = crate::Reg<crc_data::CRC_DATA_SPEC>;
#[doc = "CRC Data register"]
pub mod crc_data;
#[doc = "CRC_DATAL (rw) register accessor: an alias for `Reg<CRC_DATAL_SPEC>`"]
pub type CRC_DATAL = crate::Reg<crc_datal::CRC_DATAL_SPEC>;
#[doc = "CRC_DATAL register."]
pub mod crc_datal;
#[doc = "CRC_DATALL (rw) register accessor: an alias for `Reg<CRC_DATALL_SPEC>`"]
pub type CRC_DATALL = crate::Reg<crc_datall::CRC_DATALL_SPEC>;
#[doc = "CRC_DATALL register."]
pub mod crc_datall;
#[doc = "DATALU (rw) register accessor: an alias for `Reg<DATALU_SPEC>`"]
pub type DATALU = crate::Reg<datalu::DATALU_SPEC>;
#[doc = "CRC_DATALU register."]
pub mod datalu;
#[doc = "CRC_DATAH (rw) register accessor: an alias for `Reg<CRC_DATAH_SPEC>`"]
pub type CRC_DATAH = crate::Reg<crc_datah::CRC_DATAH_SPEC>;
#[doc = "CRC_DATAH register."]
pub mod crc_datah;
#[doc = "CRC_DATAHL (rw) register accessor: an alias for `Reg<CRC_DATAHL_SPEC>`"]
pub type CRC_DATAHL = crate::Reg<crc_datahl::CRC_DATAHL_SPEC>;
#[doc = "CRC_DATAHL register."]
pub mod crc_datahl;
#[doc = "DATAHU (rw) register accessor: an alias for `Reg<DATAHU_SPEC>`"]
pub type DATAHU = crate::Reg<datahu::DATAHU_SPEC>;
#[doc = "CRC_DATAHU register."]
pub mod datahu;
#[doc = "CRC_GPOLY (rw) register accessor: an alias for `Reg<CRC_GPOLY_SPEC>`"]
pub type CRC_GPOLY = crate::Reg<crc_gpoly::CRC_GPOLY_SPEC>;
#[doc = "CRC Polynomial register"]
pub mod crc_gpoly;
#[doc = "CRC_GPOLYL (rw) register accessor: an alias for `Reg<CRC_GPOLYL_SPEC>`"]
pub type CRC_GPOLYL = crate::Reg<crc_gpolyl::CRC_GPOLYL_SPEC>;
#[doc = "CRC_GPOLYL register."]
pub mod crc_gpolyl;
#[doc = "CRC_GPOLYLL (rw) register accessor: an alias for `Reg<CRC_GPOLYLL_SPEC>`"]
pub type CRC_GPOLYLL = crate::Reg<crc_gpolyll::CRC_GPOLYLL_SPEC>;
#[doc = "CRC_GPOLYLL register."]
pub mod crc_gpolyll;
#[doc = "GPOLYLU (rw) register accessor: an alias for `Reg<GPOLYLU_SPEC>`"]
pub type GPOLYLU = crate::Reg<gpolylu::GPOLYLU_SPEC>;
#[doc = "CRC_GPOLYLU register."]
pub mod gpolylu;
#[doc = "CRC_GPOLYH (rw) register accessor: an alias for `Reg<CRC_GPOLYH_SPEC>`"]
pub type CRC_GPOLYH = crate::Reg<crc_gpolyh::CRC_GPOLYH_SPEC>;
#[doc = "CRC_GPOLYH register."]
pub mod crc_gpolyh;
#[doc = "CRC_GPOLYHL (rw) register accessor: an alias for `Reg<CRC_GPOLYHL_SPEC>`"]
pub type CRC_GPOLYHL = crate::Reg<crc_gpolyhl::CRC_GPOLYHL_SPEC>;
#[doc = "CRC_GPOLYHL register."]
pub mod crc_gpolyhl;
#[doc = "GPOLYHU (rw) register accessor: an alias for `Reg<GPOLYHU_SPEC>`"]
pub type GPOLYHU = crate::Reg<gpolyhu::GPOLYHU_SPEC>;
#[doc = "CRC_GPOLYHU register."]
pub mod gpolyhu;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CRC Control register"]
pub mod ctrl;
#[doc = "CTRLHU (rw) register accessor: an alias for `Reg<CTRLHU_SPEC>`"]
pub type CTRLHU = crate::Reg<ctrlhu::CTRLHU_SPEC>;
#[doc = "CRC_CTRLHU register."]
pub mod ctrlhu;

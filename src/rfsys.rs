#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - Register file register"]
    pub reg: [REG; 8],
}
#[doc = "REG (rw) register accessor: an alias for `Reg<REG_SPEC>`"]
pub type REG = crate::Reg<reg::REG_SPEC>;
#[doc = "Register file register"]
pub mod reg;

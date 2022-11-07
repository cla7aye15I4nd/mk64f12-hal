#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Status Register"]
    pub fstat: FSTAT,
    #[doc = "0x01 - Flash Configuration Register"]
    pub fcnfg: FCNFG,
    #[doc = "0x02 - Flash Security Register"]
    pub fsec: FSEC,
    #[doc = "0x03 - Flash Option Register"]
    pub fopt: FOPT,
    #[doc = "0x04..0x10 - Flash Common Command Object Registers"]
    pub fccob: [FCCOB; 12],
    #[doc = "0x10 - Program Flash Protection Registers"]
    pub fprot: [FPROT; 4],
    _reserved6: [u8; 0x02],
    #[doc = "0x16 - EEPROM Protection Register"]
    pub feprot: FEPROT,
    #[doc = "0x17 - Data Flash Protection Register"]
    pub fdprot: FDPROT,
}
impl RegisterBlock {
    #[doc = "0x04 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob3(&self) -> &FCCOB {
        &self.fccob[0]
    }
    #[doc = "0x05 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob2(&self) -> &FCCOB {
        &self.fccob[1]
    }
    #[doc = "0x06 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob1(&self) -> &FCCOB {
        &self.fccob[2]
    }
    #[doc = "0x07 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob0(&self) -> &FCCOB {
        &self.fccob[3]
    }
    #[doc = "0x08 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob7(&self) -> &FCCOB {
        &self.fccob[4]
    }
    #[doc = "0x09 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob6(&self) -> &FCCOB {
        &self.fccob[5]
    }
    #[doc = "0x0a - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob5(&self) -> &FCCOB {
        &self.fccob[6]
    }
    #[doc = "0x0b - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob4(&self) -> &FCCOB {
        &self.fccob[7]
    }
    #[doc = "0x0c - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccobb(&self) -> &FCCOB {
        &self.fccob[8]
    }
    #[doc = "0x0d - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccoba(&self) -> &FCCOB {
        &self.fccob[9]
    }
    #[doc = "0x0e - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob9(&self) -> &FCCOB {
        &self.fccob[10]
    }
    #[doc = "0x0f - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob8(&self) -> &FCCOB {
        &self.fccob[11]
    }
    #[doc = "0x10 - Program Flash Protection Registers"]
    #[inline(always)]
    pub fn fprot3(&self) -> &FPROT {
        &self.fprot[0]
    }
    #[doc = "0x11 - Program Flash Protection Registers"]
    #[inline(always)]
    pub fn fprot2(&self) -> &FPROT {
        &self.fprot[1]
    }
    #[doc = "0x12 - Program Flash Protection Registers"]
    #[inline(always)]
    pub fn fprot1(&self) -> &FPROT {
        &self.fprot[2]
    }
    #[doc = "0x13 - Program Flash Protection Registers"]
    #[inline(always)]
    pub fn fprot0(&self) -> &FPROT {
        &self.fprot[3]
    }
}
#[doc = "FSTAT (rw) register accessor: an alias for `Reg<FSTAT_SPEC>`"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "FCNFG (rw) register accessor: an alias for `Reg<FCNFG_SPEC>`"]
pub type FCNFG = crate::Reg<fcnfg::FCNFG_SPEC>;
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "FSEC (r) register accessor: an alias for `Reg<FSEC_SPEC>`"]
pub type FSEC = crate::Reg<fsec::FSEC_SPEC>;
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "FOPT (r) register accessor: an alias for `Reg<FOPT_SPEC>`"]
pub type FOPT = crate::Reg<fopt::FOPT_SPEC>;
#[doc = "Flash Option Register"]
pub mod fopt;
#[doc = "FCCOB (rw) register accessor: an alias for `Reg<FCCOB_SPEC>`"]
pub type FCCOB = crate::Reg<fccob::FCCOB_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob;
#[doc = "FPROT (rw) register accessor: an alias for `Reg<FPROT_SPEC>`"]
pub type FPROT = crate::Reg<fprot::FPROT_SPEC>;
#[doc = "Program Flash Protection Registers"]
pub mod fprot;
#[doc = "FEPROT (rw) register accessor: an alias for `Reg<FEPROT_SPEC>`"]
pub type FEPROT = crate::Reg<feprot::FEPROT_SPEC>;
#[doc = "EEPROM Protection Register"]
pub mod feprot;
#[doc = "FDPROT (rw) register accessor: an alias for `Reg<FDPROT_SPEC>`"]
pub type FDPROT = crate::Reg<fdprot::FDPROT_SPEC>;
#[doc = "Data Flash Protection Register"]
pub mod fdprot;

#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALT` reader - Halt"]
pub type HALT_R = crate::BitReader<HALT_A>;
#[doc = "Halt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALT_A {
    #[doc = "0: Start transfers."]
    _0 = 0,
    #[doc = "1: Stop transfers."]
    _1 = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
impl HALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::_0,
            true => HALT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HALT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HALT_A::_1
    }
}
#[doc = "Field `HALT` writer - Halt"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, HALT_A, O>;
impl<'a, const O: u8> HALT_W<'a, O> {
    #[doc = "Start transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALT_A::_0)
    }
    #[doc = "Stop transfers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALT_A::_1)
    }
}
#[doc = "Field `SMPL_PT` reader - Sample Point"]
pub type SMPL_PT_R = crate::FieldReader<u8, SMPL_PT_A>;
#[doc = "Sample Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMPL_PT_A {
    #[doc = "0: 0 protocol clock cycles between SCK edge and SIN sample"]
    _00 = 0,
    #[doc = "1: 1 protocol clock cycle between SCK edge and SIN sample"]
    _01 = 1,
    #[doc = "2: 2 protocol clock cycles between SCK edge and SIN sample"]
    _10 = 2,
}
impl From<SMPL_PT_A> for u8 {
    #[inline(always)]
    fn from(variant: SMPL_PT_A) -> Self {
        variant as _
    }
}
impl SMPL_PT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMPL_PT_A> {
        match self.bits {
            0 => Some(SMPL_PT_A::_00),
            1 => Some(SMPL_PT_A::_01),
            2 => Some(SMPL_PT_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SMPL_PT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SMPL_PT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SMPL_PT_A::_10
    }
}
#[doc = "Field `SMPL_PT` writer - Sample Point"]
pub type SMPL_PT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, SMPL_PT_A, 2, O>;
impl<'a, const O: u8> SMPL_PT_W<'a, O> {
    #[doc = "0 protocol clock cycles between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SMPL_PT_A::_00)
    }
    #[doc = "1 protocol clock cycle between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SMPL_PT_A::_01)
    }
    #[doc = "2 protocol clock cycles between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SMPL_PT_A::_10)
    }
}
#[doc = "Flushes the RX FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR_RXF_AW {
    #[doc = "0: Do not clear the RX FIFO counter."]
    _0 = 0,
    #[doc = "1: Clear the RX FIFO counter."]
    _1 = 1,
}
impl From<CLR_RXF_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_RXF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_RXF` writer - Flushes the RX FIFO"]
pub type CLR_RXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, CLR_RXF_AW, O>;
impl<'a, const O: u8> CLR_RXF_W<'a, O> {
    #[doc = "Do not clear the RX FIFO counter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_RXF_AW::_0)
    }
    #[doc = "Clear the RX FIFO counter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_RXF_AW::_1)
    }
}
#[doc = "Clear TX FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR_TXF_AW {
    #[doc = "0: Do not clear the TX FIFO counter."]
    _0 = 0,
    #[doc = "1: Clear the TX FIFO counter."]
    _1 = 1,
}
impl From<CLR_TXF_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_TXF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_TXF` writer - Clear TX FIFO"]
pub type CLR_TXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, CLR_TXF_AW, O>;
impl<'a, const O: u8> CLR_TXF_W<'a, O> {
    #[doc = "Do not clear the TX FIFO counter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_TXF_AW::_0)
    }
    #[doc = "Clear the TX FIFO counter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_TXF_AW::_1)
    }
}
#[doc = "Field `DIS_RXF` reader - Disable Receive FIFO"]
pub type DIS_RXF_R = crate::BitReader<DIS_RXF_A>;
#[doc = "Disable Receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_RXF_A {
    #[doc = "0: RX FIFO is enabled."]
    _0 = 0,
    #[doc = "1: RX FIFO is disabled."]
    _1 = 1,
}
impl From<DIS_RXF_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_RXF_A) -> Self {
        variant as u8 != 0
    }
}
impl DIS_RXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_RXF_A {
        match self.bits {
            false => DIS_RXF_A::_0,
            true => DIS_RXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIS_RXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIS_RXF_A::_1
    }
}
#[doc = "Field `DIS_RXF` writer - Disable Receive FIFO"]
pub type DIS_RXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, DIS_RXF_A, O>;
impl<'a, const O: u8> DIS_RXF_W<'a, O> {
    #[doc = "RX FIFO is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIS_RXF_A::_0)
    }
    #[doc = "RX FIFO is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIS_RXF_A::_1)
    }
}
#[doc = "Field `DIS_TXF` reader - Disable Transmit FIFO"]
pub type DIS_TXF_R = crate::BitReader<DIS_TXF_A>;
#[doc = "Disable Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_TXF_A {
    #[doc = "0: TX FIFO is enabled."]
    _0 = 0,
    #[doc = "1: TX FIFO is disabled."]
    _1 = 1,
}
impl From<DIS_TXF_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_TXF_A) -> Self {
        variant as u8 != 0
    }
}
impl DIS_TXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_TXF_A {
        match self.bits {
            false => DIS_TXF_A::_0,
            true => DIS_TXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIS_TXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIS_TXF_A::_1
    }
}
#[doc = "Field `DIS_TXF` writer - Disable Transmit FIFO"]
pub type DIS_TXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, DIS_TXF_A, O>;
impl<'a, const O: u8> DIS_TXF_W<'a, O> {
    #[doc = "TX FIFO is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIS_TXF_A::_0)
    }
    #[doc = "TX FIFO is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIS_TXF_A::_1)
    }
}
#[doc = "Field `MDIS` reader - Module Disable"]
pub type MDIS_R = crate::BitReader<MDIS_A>;
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDIS_A {
    #[doc = "0: Enables the module clocks."]
    _0 = 0,
    #[doc = "1: Allows external logic to disable the module clocks."]
    _1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl MDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::_0,
            true => MDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MDIS_A::_1
    }
}
#[doc = "Field `MDIS` writer - Module Disable"]
pub type MDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MDIS_A, O>;
impl<'a, const O: u8> MDIS_W<'a, O> {
    #[doc = "Enables the module clocks."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDIS_A::_0)
    }
    #[doc = "Allows external logic to disable the module clocks."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDIS_A::_1)
    }
}
#[doc = "Field `DOZE` reader - Doze Enable"]
pub type DOZE_R = crate::BitReader<DOZE_A>;
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOZE_A {
    #[doc = "0: Doze mode has no effect on the module."]
    _0 = 0,
    #[doc = "1: Doze mode disables the module."]
    _1 = 1,
}
impl From<DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZE_A {
        match self.bits {
            false => DOZE_A::_0,
            true => DOZE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOZE_A::_1
    }
}
#[doc = "Field `DOZE` writer - Doze Enable"]
pub type DOZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, DOZE_A, O>;
impl<'a, const O: u8> DOZE_W<'a, O> {
    #[doc = "Doze mode has no effect on the module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZE_A::_0)
    }
    #[doc = "Doze mode disables the module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZE_A::_1)
    }
}
#[doc = "Field `PCSIS0` reader - Peripheral Chip Select x Inactive State"]
pub type PCSIS0_R = crate::BitReader<PCSIS0_A>;
#[doc = "Peripheral Chip Select x Inactive State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSIS0_A {
    #[doc = "0: The inactive state of PCSx is low."]
    _0 = 0,
    #[doc = "1: The inactive state of PCSx is high."]
    _1 = 1,
}
impl From<PCSIS0_A> for bool {
    #[inline(always)]
    fn from(variant: PCSIS0_A) -> Self {
        variant as u8 != 0
    }
}
impl PCSIS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSIS0_A {
        match self.bits {
            false => PCSIS0_A::_0,
            true => PCSIS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCSIS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCSIS0_A::_1
    }
}
#[doc = "Field `PCSIS0` writer - Peripheral Chip Select x Inactive State"]
pub type PCSIS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, PCSIS0_A, O>;
impl<'a, const O: u8> PCSIS0_W<'a, O> {
    #[doc = "The inactive state of PCSx is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSIS0_A::_0)
    }
    #[doc = "The inactive state of PCSx is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSIS0_A::_1)
    }
}
#[doc = "Field `PCSIS1` reader - Peripheral Chip Select x Inactive State"]
pub type PCSIS1_R = crate::BitReader<PCSIS1_A>;
#[doc = "Peripheral Chip Select x Inactive State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSIS1_A {
    #[doc = "0: The inactive state of PCSx is low."]
    _0 = 0,
    #[doc = "1: The inactive state of PCSx is high."]
    _1 = 1,
}
impl From<PCSIS1_A> for bool {
    #[inline(always)]
    fn from(variant: PCSIS1_A) -> Self {
        variant as u8 != 0
    }
}
impl PCSIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSIS1_A {
        match self.bits {
            false => PCSIS1_A::_0,
            true => PCSIS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCSIS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCSIS1_A::_1
    }
}
#[doc = "Field `PCSIS1` writer - Peripheral Chip Select x Inactive State"]
pub type PCSIS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, PCSIS1_A, O>;
impl<'a, const O: u8> PCSIS1_W<'a, O> {
    #[doc = "The inactive state of PCSx is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSIS1_A::_0)
    }
    #[doc = "The inactive state of PCSx is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSIS1_A::_1)
    }
}
#[doc = "Field `PCSIS2` reader - Peripheral Chip Select x Inactive State"]
pub type PCSIS2_R = crate::BitReader<PCSIS2_A>;
#[doc = "Peripheral Chip Select x Inactive State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSIS2_A {
    #[doc = "0: The inactive state of PCSx is low."]
    _0 = 0,
    #[doc = "1: The inactive state of PCSx is high."]
    _1 = 1,
}
impl From<PCSIS2_A> for bool {
    #[inline(always)]
    fn from(variant: PCSIS2_A) -> Self {
        variant as u8 != 0
    }
}
impl PCSIS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSIS2_A {
        match self.bits {
            false => PCSIS2_A::_0,
            true => PCSIS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCSIS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCSIS2_A::_1
    }
}
#[doc = "Field `PCSIS2` writer - Peripheral Chip Select x Inactive State"]
pub type PCSIS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, PCSIS2_A, O>;
impl<'a, const O: u8> PCSIS2_W<'a, O> {
    #[doc = "The inactive state of PCSx is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSIS2_A::_0)
    }
    #[doc = "The inactive state of PCSx is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSIS2_A::_1)
    }
}
#[doc = "Field `PCSIS3` reader - Peripheral Chip Select x Inactive State"]
pub type PCSIS3_R = crate::BitReader<PCSIS3_A>;
#[doc = "Peripheral Chip Select x Inactive State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSIS3_A {
    #[doc = "0: The inactive state of PCSx is low."]
    _0 = 0,
    #[doc = "1: The inactive state of PCSx is high."]
    _1 = 1,
}
impl From<PCSIS3_A> for bool {
    #[inline(always)]
    fn from(variant: PCSIS3_A) -> Self {
        variant as u8 != 0
    }
}
impl PCSIS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSIS3_A {
        match self.bits {
            false => PCSIS3_A::_0,
            true => PCSIS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCSIS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCSIS3_A::_1
    }
}
#[doc = "Field `PCSIS3` writer - Peripheral Chip Select x Inactive State"]
pub type PCSIS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, PCSIS3_A, O>;
impl<'a, const O: u8> PCSIS3_W<'a, O> {
    #[doc = "The inactive state of PCSx is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSIS3_A::_0)
    }
    #[doc = "The inactive state of PCSx is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSIS3_A::_1)
    }
}
#[doc = "Field `PCSIS4` reader - Peripheral Chip Select x Inactive State"]
pub type PCSIS4_R = crate::BitReader<PCSIS4_A>;
#[doc = "Peripheral Chip Select x Inactive State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSIS4_A {
    #[doc = "0: The inactive state of PCSx is low."]
    _0 = 0,
    #[doc = "1: The inactive state of PCSx is high."]
    _1 = 1,
}
impl From<PCSIS4_A> for bool {
    #[inline(always)]
    fn from(variant: PCSIS4_A) -> Self {
        variant as u8 != 0
    }
}
impl PCSIS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSIS4_A {
        match self.bits {
            false => PCSIS4_A::_0,
            true => PCSIS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCSIS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCSIS4_A::_1
    }
}
#[doc = "Field `PCSIS4` writer - Peripheral Chip Select x Inactive State"]
pub type PCSIS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, PCSIS4_A, O>;
impl<'a, const O: u8> PCSIS4_W<'a, O> {
    #[doc = "The inactive state of PCSx is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSIS4_A::_0)
    }
    #[doc = "The inactive state of PCSx is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSIS4_A::_1)
    }
}
#[doc = "Field `PCSIS5` reader - Peripheral Chip Select x Inactive State"]
pub type PCSIS5_R = crate::BitReader<PCSIS5_A>;
#[doc = "Peripheral Chip Select x Inactive State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSIS5_A {
    #[doc = "0: The inactive state of PCSx is low."]
    _0 = 0,
    #[doc = "1: The inactive state of PCSx is high."]
    _1 = 1,
}
impl From<PCSIS5_A> for bool {
    #[inline(always)]
    fn from(variant: PCSIS5_A) -> Self {
        variant as u8 != 0
    }
}
impl PCSIS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSIS5_A {
        match self.bits {
            false => PCSIS5_A::_0,
            true => PCSIS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCSIS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCSIS5_A::_1
    }
}
#[doc = "Field `PCSIS5` writer - Peripheral Chip Select x Inactive State"]
pub type PCSIS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, PCSIS5_A, O>;
impl<'a, const O: u8> PCSIS5_W<'a, O> {
    #[doc = "The inactive state of PCSx is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSIS5_A::_0)
    }
    #[doc = "The inactive state of PCSx is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSIS5_A::_1)
    }
}
#[doc = "Field `ROOE` reader - Receive FIFO Overflow Overwrite Enable"]
pub type ROOE_R = crate::BitReader<ROOE_A>;
#[doc = "Receive FIFO Overflow Overwrite Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROOE_A {
    #[doc = "0: Incoming data is ignored."]
    _0 = 0,
    #[doc = "1: Incoming data is shifted into the shift register."]
    _1 = 1,
}
impl From<ROOE_A> for bool {
    #[inline(always)]
    fn from(variant: ROOE_A) -> Self {
        variant as u8 != 0
    }
}
impl ROOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROOE_A {
        match self.bits {
            false => ROOE_A::_0,
            true => ROOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ROOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ROOE_A::_1
    }
}
#[doc = "Field `ROOE` writer - Receive FIFO Overflow Overwrite Enable"]
pub type ROOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, ROOE_A, O>;
impl<'a, const O: u8> ROOE_W<'a, O> {
    #[doc = "Incoming data is ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROOE_A::_0)
    }
    #[doc = "Incoming data is shifted into the shift register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROOE_A::_1)
    }
}
#[doc = "Field `PCSSE` reader - Peripheral Chip Select Strobe Enable"]
pub type PCSSE_R = crate::BitReader<PCSSE_A>;
#[doc = "Peripheral Chip Select Strobe Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSSE_A {
    #[doc = "0: PCS5/ PCSS is used as the Peripheral Chip Select\\[5\\]
signal."]
    _0 = 0,
    #[doc = "1: PCS5/ PCSS is used as an active-low PCS Strobe signal."]
    _1 = 1,
}
impl From<PCSSE_A> for bool {
    #[inline(always)]
    fn from(variant: PCSSE_A) -> Self {
        variant as u8 != 0
    }
}
impl PCSSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSSE_A {
        match self.bits {
            false => PCSSE_A::_0,
            true => PCSSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCSSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCSSE_A::_1
    }
}
#[doc = "Field `PCSSE` writer - Peripheral Chip Select Strobe Enable"]
pub type PCSSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, PCSSE_A, O>;
impl<'a, const O: u8> PCSSE_W<'a, O> {
    #[doc = "PCS5/ PCSS is used as the Peripheral Chip Select\\[5\\]
signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSSE_A::_0)
    }
    #[doc = "PCS5/ PCSS is used as an active-low PCS Strobe signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSSE_A::_1)
    }
}
#[doc = "Field `MTFE` reader - Modified Timing Format Enable"]
pub type MTFE_R = crate::BitReader<MTFE_A>;
#[doc = "Modified Timing Format Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTFE_A {
    #[doc = "0: Modified SPI transfer format disabled."]
    _0 = 0,
    #[doc = "1: Modified SPI transfer format enabled."]
    _1 = 1,
}
impl From<MTFE_A> for bool {
    #[inline(always)]
    fn from(variant: MTFE_A) -> Self {
        variant as u8 != 0
    }
}
impl MTFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTFE_A {
        match self.bits {
            false => MTFE_A::_0,
            true => MTFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTFE_A::_1
    }
}
#[doc = "Field `MTFE` writer - Modified Timing Format Enable"]
pub type MTFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MTFE_A, O>;
impl<'a, const O: u8> MTFE_W<'a, O> {
    #[doc = "Modified SPI transfer format disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTFE_A::_0)
    }
    #[doc = "Modified SPI transfer format enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTFE_A::_1)
    }
}
#[doc = "Field `FRZ` reader - Freeze"]
pub type FRZ_R = crate::BitReader<FRZ_A>;
#[doc = "Freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRZ_A {
    #[doc = "0: Do not halt serial transfers in Debug mode."]
    _0 = 0,
    #[doc = "1: Halt serial transfers in Debug mode."]
    _1 = 1,
}
impl From<FRZ_A> for bool {
    #[inline(always)]
    fn from(variant: FRZ_A) -> Self {
        variant as u8 != 0
    }
}
impl FRZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZ_A {
        match self.bits {
            false => FRZ_A::_0,
            true => FRZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRZ_A::_1
    }
}
#[doc = "Field `FRZ` writer - Freeze"]
pub type FRZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, FRZ_A, O>;
impl<'a, const O: u8> FRZ_W<'a, O> {
    #[doc = "Do not halt serial transfers in Debug mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRZ_A::_0)
    }
    #[doc = "Halt serial transfers in Debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRZ_A::_1)
    }
}
#[doc = "Field `DCONF` reader - SPI Configuration."]
pub type DCONF_R = crate::FieldReader<u8, DCONF_A>;
#[doc = "SPI Configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCONF_A {
    #[doc = "0: SPI"]
    _00 = 0,
}
impl From<DCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: DCONF_A) -> Self {
        variant as _
    }
}
impl DCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCONF_A> {
        match self.bits {
            0 => Some(DCONF_A::_00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DCONF_A::_00
    }
}
#[doc = "Field `CONT_SCKE` reader - Continuous SCK Enable"]
pub type CONT_SCKE_R = crate::BitReader<CONT_SCKE_A>;
#[doc = "Continuous SCK Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_SCKE_A {
    #[doc = "0: Continuous SCK disabled."]
    _0 = 0,
    #[doc = "1: Continuous SCK enabled."]
    _1 = 1,
}
impl From<CONT_SCKE_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_SCKE_A) -> Self {
        variant as u8 != 0
    }
}
impl CONT_SCKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_SCKE_A {
        match self.bits {
            false => CONT_SCKE_A::_0,
            true => CONT_SCKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CONT_SCKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CONT_SCKE_A::_1
    }
}
#[doc = "Field `CONT_SCKE` writer - Continuous SCK Enable"]
pub type CONT_SCKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, CONT_SCKE_A, O>;
impl<'a, const O: u8> CONT_SCKE_W<'a, O> {
    #[doc = "Continuous SCK disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONT_SCKE_A::_0)
    }
    #[doc = "Continuous SCK enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONT_SCKE_A::_1)
    }
}
#[doc = "Field `MSTR` reader - Master/Slave Mode Select"]
pub type MSTR_R = crate::BitReader<MSTR_A>;
#[doc = "Master/Slave Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTR_A {
    #[doc = "0: Enables Slave mode"]
    _0 = 0,
    #[doc = "1: Enables Master mode"]
    _1 = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::_0,
            true => MSTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTR_A::_1
    }
}
#[doc = "Field `MSTR` writer - Master/Slave Mode Select"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MSTR_A, O>;
impl<'a, const O: u8> MSTR_W<'a, O> {
    #[doc = "Enables Slave mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTR_A::_0)
    }
    #[doc = "Enables Master mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Halt"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Sample Point"]
    #[inline(always)]
    pub fn smpl_pt(&self) -> SMPL_PT_R {
        SMPL_PT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Disable Receive FIFO"]
    #[inline(always)]
    pub fn dis_rxf(&self) -> DIS_RXF_R {
        DIS_RXF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable Transmit FIFO"]
    #[inline(always)]
    pub fn dis_txf(&self) -> DIS_TXF_R {
        DIS_TXF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline(always)]
    pub fn doze(&self) -> DOZE_R {
        DOZE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis0(&self) -> PCSIS0_R {
        PCSIS0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis1(&self) -> PCSIS1_R {
        PCSIS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis2(&self) -> PCSIS2_R {
        PCSIS2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis3(&self) -> PCSIS3_R {
        PCSIS3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis4(&self) -> PCSIS4_R {
        PCSIS4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis5(&self) -> PCSIS5_R {
        PCSIS5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive FIFO Overflow Overwrite Enable"]
    #[inline(always)]
    pub fn rooe(&self) -> ROOE_R {
        ROOE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral Chip Select Strobe Enable"]
    #[inline(always)]
    pub fn pcsse(&self) -> PCSSE_R {
        PCSSE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Modified Timing Format Enable"]
    #[inline(always)]
    pub fn mtfe(&self) -> MTFE_R {
        MTFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Freeze"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - SPI Configuration."]
    #[inline(always)]
    pub fn dconf(&self) -> DCONF_R {
        DCONF_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Continuous SCK Enable"]
    #[inline(always)]
    pub fn cont_scke(&self) -> CONT_SCKE_R {
        CONT_SCKE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<0> {
        HALT_W::new(self)
    }
    #[doc = "Bits 8:9 - Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn smpl_pt(&mut self) -> SMPL_PT_W<8> {
        SMPL_PT_W::new(self)
    }
    #[doc = "Bit 10 - Flushes the RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rxf(&mut self) -> CLR_RXF_W<10> {
        CLR_RXF_W::new(self)
    }
    #[doc = "Bit 11 - Clear TX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn clr_txf(&mut self) -> CLR_TXF_W<11> {
        CLR_TXF_W::new(self)
    }
    #[doc = "Bit 12 - Disable Receive FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn dis_rxf(&mut self) -> DIS_RXF_W<12> {
        DIS_RXF_W::new(self)
    }
    #[doc = "Bit 13 - Disable Transmit FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn dis_txf(&mut self) -> DIS_TXF_W<13> {
        DIS_TXF_W::new(self)
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mdis(&mut self) -> MDIS_W<14> {
        MDIS_W::new(self)
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn doze(&mut self) -> DOZE_W<15> {
        DOZE_W::new(self)
    }
    #[doc = "Bit 16 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    #[must_use]
    pub fn pcsis0(&mut self) -> PCSIS0_W<16> {
        PCSIS0_W::new(self)
    }
    #[doc = "Bit 17 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    #[must_use]
    pub fn pcsis1(&mut self) -> PCSIS1_W<17> {
        PCSIS1_W::new(self)
    }
    #[doc = "Bit 18 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    #[must_use]
    pub fn pcsis2(&mut self) -> PCSIS2_W<18> {
        PCSIS2_W::new(self)
    }
    #[doc = "Bit 19 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    #[must_use]
    pub fn pcsis3(&mut self) -> PCSIS3_W<19> {
        PCSIS3_W::new(self)
    }
    #[doc = "Bit 20 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    #[must_use]
    pub fn pcsis4(&mut self) -> PCSIS4_W<20> {
        PCSIS4_W::new(self)
    }
    #[doc = "Bit 21 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    #[must_use]
    pub fn pcsis5(&mut self) -> PCSIS5_W<21> {
        PCSIS5_W::new(self)
    }
    #[doc = "Bit 24 - Receive FIFO Overflow Overwrite Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rooe(&mut self) -> ROOE_W<24> {
        ROOE_W::new(self)
    }
    #[doc = "Bit 25 - Peripheral Chip Select Strobe Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcsse(&mut self) -> PCSSE_W<25> {
        PCSSE_W::new(self)
    }
    #[doc = "Bit 26 - Modified Timing Format Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mtfe(&mut self) -> MTFE_W<26> {
        MTFE_W::new(self)
    }
    #[doc = "Bit 27 - Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn frz(&mut self) -> FRZ_W<27> {
        FRZ_W::new(self)
    }
    #[doc = "Bit 30 - Continuous SCK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cont_scke(&mut self) -> CONT_SCKE_W<30> {
        CONT_SCKE_W::new(self)
    }
    #[doc = "Bit 31 - Master/Slave Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<31> {
        MSTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0x4001"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4001;
}

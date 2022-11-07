#[doc = "Register `PROCTL` reader"]
pub struct R(crate::R<PROCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROCTL` writer"]
pub struct W(crate::W<PROCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROCTL_SPEC>;
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
impl From<crate::W<PROCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCTL` reader - LED Control"]
pub type LCTL_R = crate::BitReader<LCTL_A>;
#[doc = "LED Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCTL_A {
    #[doc = "0: LED off."]
    _0 = 0,
    #[doc = "1: LED on."]
    _1 = 1,
}
impl From<LCTL_A> for bool {
    #[inline(always)]
    fn from(variant: LCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl LCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCTL_A {
        match self.bits {
            false => LCTL_A::_0,
            true => LCTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCTL_A::_1
    }
}
#[doc = "Field `LCTL` writer - LED Control"]
pub type LCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROCTL_SPEC, LCTL_A, O>;
impl<'a, const O: u8> LCTL_W<'a, O> {
    #[doc = "LED off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCTL_A::_0)
    }
    #[doc = "LED on."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCTL_A::_1)
    }
}
#[doc = "Field `DTW` reader - Data Transfer Width"]
pub type DTW_R = crate::FieldReader<u8, DTW_A>;
#[doc = "Data Transfer Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTW_A {
    #[doc = "0: 1-bit mode"]
    _00 = 0,
    #[doc = "1: 4-bit mode"]
    _01 = 1,
    #[doc = "2: 8-bit mode"]
    _10 = 2,
}
impl From<DTW_A> for u8 {
    #[inline(always)]
    fn from(variant: DTW_A) -> Self {
        variant as _
    }
}
impl DTW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTW_A> {
        match self.bits {
            0 => Some(DTW_A::_00),
            1 => Some(DTW_A::_01),
            2 => Some(DTW_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DTW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DTW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DTW_A::_10
    }
}
#[doc = "Field `DTW` writer - Data Transfer Width"]
pub type DTW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PROCTL_SPEC, u8, DTW_A, 2, O>;
impl<'a, const O: u8> DTW_W<'a, O> {
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DTW_A::_00)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DTW_A::_01)
    }
    #[doc = "8-bit mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DTW_A::_10)
    }
}
#[doc = "Field `D3CD` reader - DAT3 As Card Detection Pin"]
pub type D3CD_R = crate::BitReader<D3CD_A>;
#[doc = "DAT3 As Card Detection Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3CD_A {
    #[doc = "0: DAT3 does not monitor card Insertion."]
    _0 = 0,
    #[doc = "1: DAT3 as card detection pin."]
    _1 = 1,
}
impl From<D3CD_A> for bool {
    #[inline(always)]
    fn from(variant: D3CD_A) -> Self {
        variant as u8 != 0
    }
}
impl D3CD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D3CD_A {
        match self.bits {
            false => D3CD_A::_0,
            true => D3CD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == D3CD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == D3CD_A::_1
    }
}
#[doc = "Field `D3CD` writer - DAT3 As Card Detection Pin"]
pub type D3CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROCTL_SPEC, D3CD_A, O>;
impl<'a, const O: u8> D3CD_W<'a, O> {
    #[doc = "DAT3 does not monitor card Insertion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(D3CD_A::_0)
    }
    #[doc = "DAT3 as card detection pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(D3CD_A::_1)
    }
}
#[doc = "Field `EMODE` reader - Endian Mode"]
pub type EMODE_R = crate::FieldReader<u8, EMODE_A>;
#[doc = "Endian Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMODE_A {
    #[doc = "0: Big endian mode"]
    _00 = 0,
    #[doc = "1: Half word big endian mode"]
    _01 = 1,
    #[doc = "2: Little endian mode"]
    _10 = 2,
}
impl From<EMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMODE_A) -> Self {
        variant as _
    }
}
impl EMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EMODE_A> {
        match self.bits {
            0 => Some(EMODE_A::_00),
            1 => Some(EMODE_A::_01),
            2 => Some(EMODE_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EMODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EMODE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == EMODE_A::_10
    }
}
#[doc = "Field `EMODE` writer - Endian Mode"]
pub type EMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PROCTL_SPEC, u8, EMODE_A, 2, O>;
impl<'a, const O: u8> EMODE_W<'a, O> {
    #[doc = "Big endian mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(EMODE_A::_00)
    }
    #[doc = "Half word big endian mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(EMODE_A::_01)
    }
    #[doc = "Little endian mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(EMODE_A::_10)
    }
}
#[doc = "Field `CDTL` reader - Card Detect Test Level"]
pub type CDTL_R = crate::BitReader<CDTL_A>;
#[doc = "Card Detect Test Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDTL_A {
    #[doc = "0: Card detect test level is 0, no card inserted."]
    _0 = 0,
    #[doc = "1: Card detect test level is 1, card inserted."]
    _1 = 1,
}
impl From<CDTL_A> for bool {
    #[inline(always)]
    fn from(variant: CDTL_A) -> Self {
        variant as u8 != 0
    }
}
impl CDTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDTL_A {
        match self.bits {
            false => CDTL_A::_0,
            true => CDTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDTL_A::_1
    }
}
#[doc = "Field `CDTL` writer - Card Detect Test Level"]
pub type CDTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROCTL_SPEC, CDTL_A, O>;
impl<'a, const O: u8> CDTL_W<'a, O> {
    #[doc = "Card detect test level is 0, no card inserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDTL_A::_0)
    }
    #[doc = "Card detect test level is 1, card inserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDTL_A::_1)
    }
}
#[doc = "Field `CDSS` reader - Card Detect Signal Selection"]
pub type CDSS_R = crate::BitReader<CDSS_A>;
#[doc = "Card Detect Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDSS_A {
    #[doc = "0: Card detection level is selected for normal purpose."]
    _0 = 0,
    #[doc = "1: Card detection test level is selected for test purpose."]
    _1 = 1,
}
impl From<CDSS_A> for bool {
    #[inline(always)]
    fn from(variant: CDSS_A) -> Self {
        variant as u8 != 0
    }
}
impl CDSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSS_A {
        match self.bits {
            false => CDSS_A::_0,
            true => CDSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDSS_A::_1
    }
}
#[doc = "Field `CDSS` writer - Card Detect Signal Selection"]
pub type CDSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROCTL_SPEC, CDSS_A, O>;
impl<'a, const O: u8> CDSS_W<'a, O> {
    #[doc = "Card detection level is selected for normal purpose."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDSS_A::_0)
    }
    #[doc = "Card detection test level is selected for test purpose."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDSS_A::_1)
    }
}
#[doc = "Field `DMAS` reader - DMA Select"]
pub type DMAS_R = crate::FieldReader<u8, DMAS_A>;
#[doc = "DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAS_A {
    #[doc = "0: No DMA or simple DMA is selected."]
    _00 = 0,
    #[doc = "1: ADMA1 is selected."]
    _01 = 1,
    #[doc = "2: ADMA2 is selected."]
    _10 = 2,
}
impl From<DMAS_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAS_A) -> Self {
        variant as _
    }
}
impl DMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAS_A> {
        match self.bits {
            0 => Some(DMAS_A::_00),
            1 => Some(DMAS_A::_01),
            2 => Some(DMAS_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DMAS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DMAS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DMAS_A::_10
    }
}
#[doc = "Field `DMAS` writer - DMA Select"]
pub type DMAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PROCTL_SPEC, u8, DMAS_A, 2, O>;
impl<'a, const O: u8> DMAS_W<'a, O> {
    #[doc = "No DMA or simple DMA is selected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DMAS_A::_00)
    }
    #[doc = "ADMA1 is selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DMAS_A::_01)
    }
    #[doc = "ADMA2 is selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DMAS_A::_10)
    }
}
#[doc = "Field `SABGREQ` reader - Stop At Block Gap Request"]
pub type SABGREQ_R = crate::BitReader<SABGREQ_A>;
#[doc = "Stop At Block Gap Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SABGREQ_A {
    #[doc = "0: Transfer"]
    _0 = 0,
    #[doc = "1: Stop"]
    _1 = 1,
}
impl From<SABGREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SABGREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SABGREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SABGREQ_A {
        match self.bits {
            false => SABGREQ_A::_0,
            true => SABGREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SABGREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SABGREQ_A::_1
    }
}
#[doc = "Field `SABGREQ` writer - Stop At Block Gap Request"]
pub type SABGREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROCTL_SPEC, SABGREQ_A, O>;
impl<'a, const O: u8> SABGREQ_W<'a, O> {
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SABGREQ_A::_0)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SABGREQ_A::_1)
    }
}
#[doc = "Field `CREQ` reader - Continue Request"]
pub type CREQ_R = crate::BitReader<CREQ_A>;
#[doc = "Continue Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CREQ_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Restart"]
    _1 = 1,
}
impl From<CREQ_A> for bool {
    #[inline(always)]
    fn from(variant: CREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CREQ_A {
        match self.bits {
            false => CREQ_A::_0,
            true => CREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CREQ_A::_1
    }
}
#[doc = "Field `CREQ` writer - Continue Request"]
pub type CREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROCTL_SPEC, CREQ_A, O>;
impl<'a, const O: u8> CREQ_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CREQ_A::_0)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CREQ_A::_1)
    }
}
#[doc = "Field `RWCTL` reader - Read Wait Control"]
pub type RWCTL_R = crate::BitReader<RWCTL_A>;
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWCTL_A {
    #[doc = "0: Disable read wait control, and stop SD clock at block gap when SABGREQ is set."]
    _0 = 0,
    #[doc = "1: Enable read wait control, and assert read wait without stopping SD clock at block gap when SABGREQ bit is set."]
    _1 = 1,
}
impl From<RWCTL_A> for bool {
    #[inline(always)]
    fn from(variant: RWCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl RWCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWCTL_A {
        match self.bits {
            false => RWCTL_A::_0,
            true => RWCTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWCTL_A::_1
    }
}
#[doc = "Field `RWCTL` writer - Read Wait Control"]
pub type RWCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROCTL_SPEC, RWCTL_A, O>;
impl<'a, const O: u8> RWCTL_W<'a, O> {
    #[doc = "Disable read wait control, and stop SD clock at block gap when SABGREQ is set."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWCTL_A::_0)
    }
    #[doc = "Enable read wait control, and assert read wait without stopping SD clock at block gap when SABGREQ bit is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWCTL_A::_1)
    }
}
#[doc = "Field `IABG` reader - Interrupt At Block Gap"]
pub type IABG_R = crate::BitReader<IABG_A>;
#[doc = "Interrupt At Block Gap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IABG_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<IABG_A> for bool {
    #[inline(always)]
    fn from(variant: IABG_A) -> Self {
        variant as u8 != 0
    }
}
impl IABG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IABG_A {
        match self.bits {
            false => IABG_A::_0,
            true => IABG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IABG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IABG_A::_1
    }
}
#[doc = "Field `IABG` writer - Interrupt At Block Gap"]
pub type IABG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROCTL_SPEC, IABG_A, O>;
impl<'a, const O: u8> IABG_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IABG_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IABG_A::_1)
    }
}
#[doc = "Field `WECINT` reader - Wakeup Event Enable On Card Interrupt"]
pub type WECINT_R = crate::BitReader<WECINT_A>;
#[doc = "Wakeup Event Enable On Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WECINT_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<WECINT_A> for bool {
    #[inline(always)]
    fn from(variant: WECINT_A) -> Self {
        variant as u8 != 0
    }
}
impl WECINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECINT_A {
        match self.bits {
            false => WECINT_A::_0,
            true => WECINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WECINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WECINT_A::_1
    }
}
#[doc = "Field `WECINT` writer - Wakeup Event Enable On Card Interrupt"]
pub type WECINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROCTL_SPEC, WECINT_A, O>;
impl<'a, const O: u8> WECINT_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WECINT_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WECINT_A::_1)
    }
}
#[doc = "Field `WECINS` reader - Wakeup Event Enable On SD Card Insertion"]
pub type WECINS_R = crate::BitReader<WECINS_A>;
#[doc = "Wakeup Event Enable On SD Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WECINS_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<WECINS_A> for bool {
    #[inline(always)]
    fn from(variant: WECINS_A) -> Self {
        variant as u8 != 0
    }
}
impl WECINS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECINS_A {
        match self.bits {
            false => WECINS_A::_0,
            true => WECINS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WECINS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WECINS_A::_1
    }
}
#[doc = "Field `WECINS` writer - Wakeup Event Enable On SD Card Insertion"]
pub type WECINS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROCTL_SPEC, WECINS_A, O>;
impl<'a, const O: u8> WECINS_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WECINS_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WECINS_A::_1)
    }
}
#[doc = "Field `WECRM` reader - Wakeup Event Enable On SD Card Removal"]
pub type WECRM_R = crate::BitReader<WECRM_A>;
#[doc = "Wakeup Event Enable On SD Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WECRM_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<WECRM_A> for bool {
    #[inline(always)]
    fn from(variant: WECRM_A) -> Self {
        variant as u8 != 0
    }
}
impl WECRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECRM_A {
        match self.bits {
            false => WECRM_A::_0,
            true => WECRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WECRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WECRM_A::_1
    }
}
#[doc = "Field `WECRM` writer - Wakeup Event Enable On SD Card Removal"]
pub type WECRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROCTL_SPEC, WECRM_A, O>;
impl<'a, const O: u8> WECRM_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WECRM_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WECRM_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn lctl(&self) -> LCTL_R {
        LCTL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline(always)]
    pub fn dtw(&self) -> DTW_R {
        DTW_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - DAT3 As Card Detection Pin"]
    #[inline(always)]
    pub fn d3cd(&self) -> D3CD_R {
        D3CD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline(always)]
    pub fn emode(&self) -> EMODE_R {
        EMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtl(&self) -> CDTL_R {
        CDTL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    pub fn cdss(&self) -> CDSS_R {
        CDSS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn sabgreq(&self) -> SABGREQ_R {
        SABGREQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn creq(&self) -> CREQ_R {
        CREQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctl(&self) -> RWCTL_R {
        RWCTL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn iabg(&self) -> IABG_R {
        IABG_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wecint(&self) -> WECINT_R {
        WECINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wecins(&self) -> WECINS_R {
        WECINS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wecrm(&self) -> WECRM_R {
        WECRM_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    #[must_use]
    pub fn lctl(&mut self) -> LCTL_W<0> {
        LCTL_W::new(self)
    }
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline(always)]
    #[must_use]
    pub fn dtw(&mut self) -> DTW_W<1> {
        DTW_W::new(self)
    }
    #[doc = "Bit 3 - DAT3 As Card Detection Pin"]
    #[inline(always)]
    #[must_use]
    pub fn d3cd(&mut self) -> D3CD_W<3> {
        D3CD_W::new(self)
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline(always)]
    #[must_use]
    pub fn emode(&mut self) -> EMODE_W<4> {
        EMODE_W::new(self)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    #[must_use]
    pub fn cdtl(&mut self) -> CDTL_W<6> {
        CDTL_W::new(self)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cdss(&mut self) -> CDSS_W<7> {
        CDSS_W::new(self)
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn dmas(&mut self) -> DMAS_W<8> {
        DMAS_W::new(self)
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline(always)]
    #[must_use]
    pub fn sabgreq(&mut self) -> SABGREQ_W<16> {
        SABGREQ_W::new(self)
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    #[must_use]
    pub fn creq(&mut self) -> CREQ_W<17> {
        CREQ_W::new(self)
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    #[must_use]
    pub fn rwctl(&mut self) -> RWCTL_W<18> {
        RWCTL_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline(always)]
    #[must_use]
    pub fn iabg(&mut self) -> IABG_W<19> {
        IABG_W::new(self)
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wecint(&mut self) -> WECINT_W<24> {
        WECINT_W::new(self)
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn wecins(&mut self) -> WECINS_W<25> {
        WECINS_W::new(self)
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn wecrm(&mut self) -> WECRM_W<26> {
        WECRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protocol Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proctl](index.html) module"]
pub struct PROCTL_SPEC;
impl crate::RegisterSpec for PROCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proctl::R](R) reader structure"]
impl crate::Readable for PROCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [proctl::W](W) writer structure"]
impl crate::Writable for PROCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROCTL to value 0x20"]
impl crate::Resettable for PROCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}

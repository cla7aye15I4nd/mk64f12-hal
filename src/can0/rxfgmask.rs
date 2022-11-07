#[doc = "Register `RXFGMASK` reader"]
pub struct R(crate::R<RXFGMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFGMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFGMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFGMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFGMASK` writer"]
pub struct W(crate::W<RXFGMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFGMASK_SPEC>;
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
impl From<crate::W<RXFGMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFGMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FGM0` reader - Rx FIFO Global Mask Bits"]
pub type FGM0_R = crate::BitReader<FGM0_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM0_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM0_A> for bool {
    #[inline(always)]
    fn from(variant: FGM0_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM0_A {
        match self.bits {
            false => FGM0_A::_0,
            true => FGM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM0_A::_1
    }
}
#[doc = "Field `FGM0` writer - Rx FIFO Global Mask Bits"]
pub type FGM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM0_A, O>;
impl<'a, const O: u8> FGM0_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM0_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM0_A::_1)
    }
}
#[doc = "Field `FGM1` reader - Rx FIFO Global Mask Bits"]
pub type FGM1_R = crate::BitReader<FGM1_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM1_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM1_A> for bool {
    #[inline(always)]
    fn from(variant: FGM1_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM1_A {
        match self.bits {
            false => FGM1_A::_0,
            true => FGM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM1_A::_1
    }
}
#[doc = "Field `FGM1` writer - Rx FIFO Global Mask Bits"]
pub type FGM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM1_A, O>;
impl<'a, const O: u8> FGM1_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM1_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM1_A::_1)
    }
}
#[doc = "Field `FGM2` reader - Rx FIFO Global Mask Bits"]
pub type FGM2_R = crate::BitReader<FGM2_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM2_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM2_A> for bool {
    #[inline(always)]
    fn from(variant: FGM2_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM2_A {
        match self.bits {
            false => FGM2_A::_0,
            true => FGM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM2_A::_1
    }
}
#[doc = "Field `FGM2` writer - Rx FIFO Global Mask Bits"]
pub type FGM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM2_A, O>;
impl<'a, const O: u8> FGM2_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM2_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM2_A::_1)
    }
}
#[doc = "Field `FGM3` reader - Rx FIFO Global Mask Bits"]
pub type FGM3_R = crate::BitReader<FGM3_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM3_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM3_A> for bool {
    #[inline(always)]
    fn from(variant: FGM3_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM3_A {
        match self.bits {
            false => FGM3_A::_0,
            true => FGM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM3_A::_1
    }
}
#[doc = "Field `FGM3` writer - Rx FIFO Global Mask Bits"]
pub type FGM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM3_A, O>;
impl<'a, const O: u8> FGM3_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM3_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM3_A::_1)
    }
}
#[doc = "Field `FGM4` reader - Rx FIFO Global Mask Bits"]
pub type FGM4_R = crate::BitReader<FGM4_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM4_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM4_A> for bool {
    #[inline(always)]
    fn from(variant: FGM4_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM4_A {
        match self.bits {
            false => FGM4_A::_0,
            true => FGM4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM4_A::_1
    }
}
#[doc = "Field `FGM4` writer - Rx FIFO Global Mask Bits"]
pub type FGM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM4_A, O>;
impl<'a, const O: u8> FGM4_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM4_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM4_A::_1)
    }
}
#[doc = "Field `FGM5` reader - Rx FIFO Global Mask Bits"]
pub type FGM5_R = crate::BitReader<FGM5_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM5_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM5_A> for bool {
    #[inline(always)]
    fn from(variant: FGM5_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM5_A {
        match self.bits {
            false => FGM5_A::_0,
            true => FGM5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM5_A::_1
    }
}
#[doc = "Field `FGM5` writer - Rx FIFO Global Mask Bits"]
pub type FGM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM5_A, O>;
impl<'a, const O: u8> FGM5_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM5_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM5_A::_1)
    }
}
#[doc = "Field `FGM6` reader - Rx FIFO Global Mask Bits"]
pub type FGM6_R = crate::BitReader<FGM6_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM6_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM6_A> for bool {
    #[inline(always)]
    fn from(variant: FGM6_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM6_A {
        match self.bits {
            false => FGM6_A::_0,
            true => FGM6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM6_A::_1
    }
}
#[doc = "Field `FGM6` writer - Rx FIFO Global Mask Bits"]
pub type FGM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM6_A, O>;
impl<'a, const O: u8> FGM6_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM6_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM6_A::_1)
    }
}
#[doc = "Field `FGM7` reader - Rx FIFO Global Mask Bits"]
pub type FGM7_R = crate::BitReader<FGM7_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM7_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM7_A> for bool {
    #[inline(always)]
    fn from(variant: FGM7_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM7_A {
        match self.bits {
            false => FGM7_A::_0,
            true => FGM7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM7_A::_1
    }
}
#[doc = "Field `FGM7` writer - Rx FIFO Global Mask Bits"]
pub type FGM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM7_A, O>;
impl<'a, const O: u8> FGM7_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM7_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM7_A::_1)
    }
}
#[doc = "Field `FGM8` reader - Rx FIFO Global Mask Bits"]
pub type FGM8_R = crate::BitReader<FGM8_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM8_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM8_A> for bool {
    #[inline(always)]
    fn from(variant: FGM8_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM8_A {
        match self.bits {
            false => FGM8_A::_0,
            true => FGM8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM8_A::_1
    }
}
#[doc = "Field `FGM8` writer - Rx FIFO Global Mask Bits"]
pub type FGM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM8_A, O>;
impl<'a, const O: u8> FGM8_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM8_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM8_A::_1)
    }
}
#[doc = "Field `FGM9` reader - Rx FIFO Global Mask Bits"]
pub type FGM9_R = crate::BitReader<FGM9_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM9_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM9_A> for bool {
    #[inline(always)]
    fn from(variant: FGM9_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM9_A {
        match self.bits {
            false => FGM9_A::_0,
            true => FGM9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM9_A::_1
    }
}
#[doc = "Field `FGM9` writer - Rx FIFO Global Mask Bits"]
pub type FGM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM9_A, O>;
impl<'a, const O: u8> FGM9_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM9_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM9_A::_1)
    }
}
#[doc = "Field `FGM10` reader - Rx FIFO Global Mask Bits"]
pub type FGM10_R = crate::BitReader<FGM10_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM10_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM10_A> for bool {
    #[inline(always)]
    fn from(variant: FGM10_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM10_A {
        match self.bits {
            false => FGM10_A::_0,
            true => FGM10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM10_A::_1
    }
}
#[doc = "Field `FGM10` writer - Rx FIFO Global Mask Bits"]
pub type FGM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM10_A, O>;
impl<'a, const O: u8> FGM10_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM10_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM10_A::_1)
    }
}
#[doc = "Field `FGM11` reader - Rx FIFO Global Mask Bits"]
pub type FGM11_R = crate::BitReader<FGM11_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM11_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM11_A> for bool {
    #[inline(always)]
    fn from(variant: FGM11_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM11_A {
        match self.bits {
            false => FGM11_A::_0,
            true => FGM11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM11_A::_1
    }
}
#[doc = "Field `FGM11` writer - Rx FIFO Global Mask Bits"]
pub type FGM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM11_A, O>;
impl<'a, const O: u8> FGM11_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM11_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM11_A::_1)
    }
}
#[doc = "Field `FGM12` reader - Rx FIFO Global Mask Bits"]
pub type FGM12_R = crate::BitReader<FGM12_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM12_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM12_A> for bool {
    #[inline(always)]
    fn from(variant: FGM12_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM12_A {
        match self.bits {
            false => FGM12_A::_0,
            true => FGM12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM12_A::_1
    }
}
#[doc = "Field `FGM12` writer - Rx FIFO Global Mask Bits"]
pub type FGM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM12_A, O>;
impl<'a, const O: u8> FGM12_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM12_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM12_A::_1)
    }
}
#[doc = "Field `FGM13` reader - Rx FIFO Global Mask Bits"]
pub type FGM13_R = crate::BitReader<FGM13_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM13_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM13_A> for bool {
    #[inline(always)]
    fn from(variant: FGM13_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM13_A {
        match self.bits {
            false => FGM13_A::_0,
            true => FGM13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM13_A::_1
    }
}
#[doc = "Field `FGM13` writer - Rx FIFO Global Mask Bits"]
pub type FGM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM13_A, O>;
impl<'a, const O: u8> FGM13_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM13_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM13_A::_1)
    }
}
#[doc = "Field `FGM14` reader - Rx FIFO Global Mask Bits"]
pub type FGM14_R = crate::BitReader<FGM14_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM14_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM14_A> for bool {
    #[inline(always)]
    fn from(variant: FGM14_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM14_A {
        match self.bits {
            false => FGM14_A::_0,
            true => FGM14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM14_A::_1
    }
}
#[doc = "Field `FGM14` writer - Rx FIFO Global Mask Bits"]
pub type FGM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM14_A, O>;
impl<'a, const O: u8> FGM14_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM14_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM14_A::_1)
    }
}
#[doc = "Field `FGM15` reader - Rx FIFO Global Mask Bits"]
pub type FGM15_R = crate::BitReader<FGM15_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM15_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM15_A> for bool {
    #[inline(always)]
    fn from(variant: FGM15_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM15_A {
        match self.bits {
            false => FGM15_A::_0,
            true => FGM15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM15_A::_1
    }
}
#[doc = "Field `FGM15` writer - Rx FIFO Global Mask Bits"]
pub type FGM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM15_A, O>;
impl<'a, const O: u8> FGM15_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM15_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM15_A::_1)
    }
}
#[doc = "Field `FGM16` reader - Rx FIFO Global Mask Bits"]
pub type FGM16_R = crate::BitReader<FGM16_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM16_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM16_A> for bool {
    #[inline(always)]
    fn from(variant: FGM16_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM16_A {
        match self.bits {
            false => FGM16_A::_0,
            true => FGM16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM16_A::_1
    }
}
#[doc = "Field `FGM16` writer - Rx FIFO Global Mask Bits"]
pub type FGM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM16_A, O>;
impl<'a, const O: u8> FGM16_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM16_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM16_A::_1)
    }
}
#[doc = "Field `FGM17` reader - Rx FIFO Global Mask Bits"]
pub type FGM17_R = crate::BitReader<FGM17_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM17_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM17_A> for bool {
    #[inline(always)]
    fn from(variant: FGM17_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM17_A {
        match self.bits {
            false => FGM17_A::_0,
            true => FGM17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM17_A::_1
    }
}
#[doc = "Field `FGM17` writer - Rx FIFO Global Mask Bits"]
pub type FGM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM17_A, O>;
impl<'a, const O: u8> FGM17_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM17_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM17_A::_1)
    }
}
#[doc = "Field `FGM18` reader - Rx FIFO Global Mask Bits"]
pub type FGM18_R = crate::BitReader<FGM18_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM18_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM18_A> for bool {
    #[inline(always)]
    fn from(variant: FGM18_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM18_A {
        match self.bits {
            false => FGM18_A::_0,
            true => FGM18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM18_A::_1
    }
}
#[doc = "Field `FGM18` writer - Rx FIFO Global Mask Bits"]
pub type FGM18_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM18_A, O>;
impl<'a, const O: u8> FGM18_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM18_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM18_A::_1)
    }
}
#[doc = "Field `FGM19` reader - Rx FIFO Global Mask Bits"]
pub type FGM19_R = crate::BitReader<FGM19_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM19_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM19_A> for bool {
    #[inline(always)]
    fn from(variant: FGM19_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM19_A {
        match self.bits {
            false => FGM19_A::_0,
            true => FGM19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM19_A::_1
    }
}
#[doc = "Field `FGM19` writer - Rx FIFO Global Mask Bits"]
pub type FGM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM19_A, O>;
impl<'a, const O: u8> FGM19_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM19_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM19_A::_1)
    }
}
#[doc = "Field `FGM20` reader - Rx FIFO Global Mask Bits"]
pub type FGM20_R = crate::BitReader<FGM20_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM20_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM20_A> for bool {
    #[inline(always)]
    fn from(variant: FGM20_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM20_A {
        match self.bits {
            false => FGM20_A::_0,
            true => FGM20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM20_A::_1
    }
}
#[doc = "Field `FGM20` writer - Rx FIFO Global Mask Bits"]
pub type FGM20_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM20_A, O>;
impl<'a, const O: u8> FGM20_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM20_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM20_A::_1)
    }
}
#[doc = "Field `FGM21` reader - Rx FIFO Global Mask Bits"]
pub type FGM21_R = crate::BitReader<FGM21_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM21_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM21_A> for bool {
    #[inline(always)]
    fn from(variant: FGM21_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM21_A {
        match self.bits {
            false => FGM21_A::_0,
            true => FGM21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM21_A::_1
    }
}
#[doc = "Field `FGM21` writer - Rx FIFO Global Mask Bits"]
pub type FGM21_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM21_A, O>;
impl<'a, const O: u8> FGM21_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM21_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM21_A::_1)
    }
}
#[doc = "Field `FGM22` reader - Rx FIFO Global Mask Bits"]
pub type FGM22_R = crate::BitReader<FGM22_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM22_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM22_A> for bool {
    #[inline(always)]
    fn from(variant: FGM22_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM22_A {
        match self.bits {
            false => FGM22_A::_0,
            true => FGM22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM22_A::_1
    }
}
#[doc = "Field `FGM22` writer - Rx FIFO Global Mask Bits"]
pub type FGM22_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM22_A, O>;
impl<'a, const O: u8> FGM22_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM22_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM22_A::_1)
    }
}
#[doc = "Field `FGM23` reader - Rx FIFO Global Mask Bits"]
pub type FGM23_R = crate::BitReader<FGM23_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM23_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM23_A> for bool {
    #[inline(always)]
    fn from(variant: FGM23_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM23_A {
        match self.bits {
            false => FGM23_A::_0,
            true => FGM23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM23_A::_1
    }
}
#[doc = "Field `FGM23` writer - Rx FIFO Global Mask Bits"]
pub type FGM23_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM23_A, O>;
impl<'a, const O: u8> FGM23_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM23_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM23_A::_1)
    }
}
#[doc = "Field `FGM24` reader - Rx FIFO Global Mask Bits"]
pub type FGM24_R = crate::BitReader<FGM24_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM24_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM24_A> for bool {
    #[inline(always)]
    fn from(variant: FGM24_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM24_A {
        match self.bits {
            false => FGM24_A::_0,
            true => FGM24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM24_A::_1
    }
}
#[doc = "Field `FGM24` writer - Rx FIFO Global Mask Bits"]
pub type FGM24_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM24_A, O>;
impl<'a, const O: u8> FGM24_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM24_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM24_A::_1)
    }
}
#[doc = "Field `FGM25` reader - Rx FIFO Global Mask Bits"]
pub type FGM25_R = crate::BitReader<FGM25_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM25_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM25_A> for bool {
    #[inline(always)]
    fn from(variant: FGM25_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM25_A {
        match self.bits {
            false => FGM25_A::_0,
            true => FGM25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM25_A::_1
    }
}
#[doc = "Field `FGM25` writer - Rx FIFO Global Mask Bits"]
pub type FGM25_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM25_A, O>;
impl<'a, const O: u8> FGM25_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM25_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM25_A::_1)
    }
}
#[doc = "Field `FGM26` reader - Rx FIFO Global Mask Bits"]
pub type FGM26_R = crate::BitReader<FGM26_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM26_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM26_A> for bool {
    #[inline(always)]
    fn from(variant: FGM26_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM26_A {
        match self.bits {
            false => FGM26_A::_0,
            true => FGM26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM26_A::_1
    }
}
#[doc = "Field `FGM26` writer - Rx FIFO Global Mask Bits"]
pub type FGM26_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM26_A, O>;
impl<'a, const O: u8> FGM26_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM26_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM26_A::_1)
    }
}
#[doc = "Field `FGM27` reader - Rx FIFO Global Mask Bits"]
pub type FGM27_R = crate::BitReader<FGM27_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM27_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM27_A> for bool {
    #[inline(always)]
    fn from(variant: FGM27_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM27_A {
        match self.bits {
            false => FGM27_A::_0,
            true => FGM27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM27_A::_1
    }
}
#[doc = "Field `FGM27` writer - Rx FIFO Global Mask Bits"]
pub type FGM27_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM27_A, O>;
impl<'a, const O: u8> FGM27_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM27_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM27_A::_1)
    }
}
#[doc = "Field `FGM28` reader - Rx FIFO Global Mask Bits"]
pub type FGM28_R = crate::BitReader<FGM28_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM28_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM28_A> for bool {
    #[inline(always)]
    fn from(variant: FGM28_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM28_A {
        match self.bits {
            false => FGM28_A::_0,
            true => FGM28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM28_A::_1
    }
}
#[doc = "Field `FGM28` writer - Rx FIFO Global Mask Bits"]
pub type FGM28_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM28_A, O>;
impl<'a, const O: u8> FGM28_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM28_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM28_A::_1)
    }
}
#[doc = "Field `FGM29` reader - Rx FIFO Global Mask Bits"]
pub type FGM29_R = crate::BitReader<FGM29_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM29_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM29_A> for bool {
    #[inline(always)]
    fn from(variant: FGM29_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM29_A {
        match self.bits {
            false => FGM29_A::_0,
            true => FGM29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM29_A::_1
    }
}
#[doc = "Field `FGM29` writer - Rx FIFO Global Mask Bits"]
pub type FGM29_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM29_A, O>;
impl<'a, const O: u8> FGM29_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM29_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM29_A::_1)
    }
}
#[doc = "Field `FGM30` reader - Rx FIFO Global Mask Bits"]
pub type FGM30_R = crate::BitReader<FGM30_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM30_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM30_A> for bool {
    #[inline(always)]
    fn from(variant: FGM30_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM30_A {
        match self.bits {
            false => FGM30_A::_0,
            true => FGM30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM30_A::_1
    }
}
#[doc = "Field `FGM30` writer - Rx FIFO Global Mask Bits"]
pub type FGM30_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM30_A, O>;
impl<'a, const O: u8> FGM30_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM30_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM30_A::_1)
    }
}
#[doc = "Field `FGM31` reader - Rx FIFO Global Mask Bits"]
pub type FGM31_R = crate::BitReader<FGM31_A>;
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FGM31_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<FGM31_A> for bool {
    #[inline(always)]
    fn from(variant: FGM31_A) -> Self {
        variant as u8 != 0
    }
}
impl FGM31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM31_A {
        match self.bits {
            false => FGM31_A::_0,
            true => FGM31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM31_A::_1
    }
}
#[doc = "Field `FGM31` writer - Rx FIFO Global Mask Bits"]
pub type FGM31_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFGMASK_SPEC, FGM31_A, O>;
impl<'a, const O: u8> FGM31_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM31_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm0(&self) -> FGM0_R {
        FGM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm1(&self) -> FGM1_R {
        FGM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm2(&self) -> FGM2_R {
        FGM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm3(&self) -> FGM3_R {
        FGM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm4(&self) -> FGM4_R {
        FGM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm5(&self) -> FGM5_R {
        FGM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm6(&self) -> FGM6_R {
        FGM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm7(&self) -> FGM7_R {
        FGM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm8(&self) -> FGM8_R {
        FGM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm9(&self) -> FGM9_R {
        FGM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm10(&self) -> FGM10_R {
        FGM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm11(&self) -> FGM11_R {
        FGM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm12(&self) -> FGM12_R {
        FGM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm13(&self) -> FGM13_R {
        FGM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm14(&self) -> FGM14_R {
        FGM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm15(&self) -> FGM15_R {
        FGM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm16(&self) -> FGM16_R {
        FGM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm17(&self) -> FGM17_R {
        FGM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm18(&self) -> FGM18_R {
        FGM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm19(&self) -> FGM19_R {
        FGM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm20(&self) -> FGM20_R {
        FGM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm21(&self) -> FGM21_R {
        FGM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm22(&self) -> FGM22_R {
        FGM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm23(&self) -> FGM23_R {
        FGM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm24(&self) -> FGM24_R {
        FGM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm25(&self) -> FGM25_R {
        FGM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm26(&self) -> FGM26_R {
        FGM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm27(&self) -> FGM27_R {
        FGM27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm28(&self) -> FGM28_R {
        FGM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm29(&self) -> FGM29_R {
        FGM29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm30(&self) -> FGM30_R {
        FGM30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm31(&self) -> FGM31_R {
        FGM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm0(&mut self) -> FGM0_W<0> {
        FGM0_W::new(self)
    }
    #[doc = "Bit 1 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm1(&mut self) -> FGM1_W<1> {
        FGM1_W::new(self)
    }
    #[doc = "Bit 2 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm2(&mut self) -> FGM2_W<2> {
        FGM2_W::new(self)
    }
    #[doc = "Bit 3 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm3(&mut self) -> FGM3_W<3> {
        FGM3_W::new(self)
    }
    #[doc = "Bit 4 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm4(&mut self) -> FGM4_W<4> {
        FGM4_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm5(&mut self) -> FGM5_W<5> {
        FGM5_W::new(self)
    }
    #[doc = "Bit 6 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm6(&mut self) -> FGM6_W<6> {
        FGM6_W::new(self)
    }
    #[doc = "Bit 7 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm7(&mut self) -> FGM7_W<7> {
        FGM7_W::new(self)
    }
    #[doc = "Bit 8 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm8(&mut self) -> FGM8_W<8> {
        FGM8_W::new(self)
    }
    #[doc = "Bit 9 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm9(&mut self) -> FGM9_W<9> {
        FGM9_W::new(self)
    }
    #[doc = "Bit 10 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm10(&mut self) -> FGM10_W<10> {
        FGM10_W::new(self)
    }
    #[doc = "Bit 11 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm11(&mut self) -> FGM11_W<11> {
        FGM11_W::new(self)
    }
    #[doc = "Bit 12 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm12(&mut self) -> FGM12_W<12> {
        FGM12_W::new(self)
    }
    #[doc = "Bit 13 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm13(&mut self) -> FGM13_W<13> {
        FGM13_W::new(self)
    }
    #[doc = "Bit 14 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm14(&mut self) -> FGM14_W<14> {
        FGM14_W::new(self)
    }
    #[doc = "Bit 15 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm15(&mut self) -> FGM15_W<15> {
        FGM15_W::new(self)
    }
    #[doc = "Bit 16 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm16(&mut self) -> FGM16_W<16> {
        FGM16_W::new(self)
    }
    #[doc = "Bit 17 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm17(&mut self) -> FGM17_W<17> {
        FGM17_W::new(self)
    }
    #[doc = "Bit 18 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm18(&mut self) -> FGM18_W<18> {
        FGM18_W::new(self)
    }
    #[doc = "Bit 19 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm19(&mut self) -> FGM19_W<19> {
        FGM19_W::new(self)
    }
    #[doc = "Bit 20 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm20(&mut self) -> FGM20_W<20> {
        FGM20_W::new(self)
    }
    #[doc = "Bit 21 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm21(&mut self) -> FGM21_W<21> {
        FGM21_W::new(self)
    }
    #[doc = "Bit 22 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm22(&mut self) -> FGM22_W<22> {
        FGM22_W::new(self)
    }
    #[doc = "Bit 23 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm23(&mut self) -> FGM23_W<23> {
        FGM23_W::new(self)
    }
    #[doc = "Bit 24 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm24(&mut self) -> FGM24_W<24> {
        FGM24_W::new(self)
    }
    #[doc = "Bit 25 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm25(&mut self) -> FGM25_W<25> {
        FGM25_W::new(self)
    }
    #[doc = "Bit 26 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm26(&mut self) -> FGM26_W<26> {
        FGM26_W::new(self)
    }
    #[doc = "Bit 27 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm27(&mut self) -> FGM27_W<27> {
        FGM27_W::new(self)
    }
    #[doc = "Bit 28 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm28(&mut self) -> FGM28_W<28> {
        FGM28_W::new(self)
    }
    #[doc = "Bit 29 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm29(&mut self) -> FGM29_W<29> {
        FGM29_W::new(self)
    }
    #[doc = "Bit 30 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm30(&mut self) -> FGM30_W<30> {
        FGM30_W::new(self)
    }
    #[doc = "Bit 31 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn fgm31(&mut self) -> FGM31_W<31> {
        FGM31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx FIFO Global Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfgmask](index.html) module"]
pub struct RXFGMASK_SPEC;
impl crate::RegisterSpec for RXFGMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfgmask::R](R) reader structure"]
impl crate::Readable for RXFGMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxfgmask::W](W) writer structure"]
impl crate::Writable for RXFGMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFGMASK to value 0xffff_ffff"]
impl crate::Resettable for RXFGMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

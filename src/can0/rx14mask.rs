#[doc = "Register `RX14MASK` reader"]
pub struct R(crate::R<RX14MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX14MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX14MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX14MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX14MASK` writer"]
pub struct W(crate::W<RX14MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX14MASK_SPEC>;
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
impl From<crate::W<RX14MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX14MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX14M0` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M0_R = crate::BitReader<RX14M0_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M0_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M0_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M0_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M0_A {
        match self.bits {
            false => RX14M0_A::_0,
            true => RX14M0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M0_A::_1
    }
}
#[doc = "Field `RX14M0` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M0_A, O>;
impl<'a, const O: u8> RX14M0_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M0_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M0_A::_1)
    }
}
#[doc = "Field `RX14M1` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M1_R = crate::BitReader<RX14M1_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M1_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M1_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M1_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M1_A {
        match self.bits {
            false => RX14M1_A::_0,
            true => RX14M1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M1_A::_1
    }
}
#[doc = "Field `RX14M1` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M1_A, O>;
impl<'a, const O: u8> RX14M1_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M1_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M1_A::_1)
    }
}
#[doc = "Field `RX14M2` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M2_R = crate::BitReader<RX14M2_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M2_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M2_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M2_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M2_A {
        match self.bits {
            false => RX14M2_A::_0,
            true => RX14M2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M2_A::_1
    }
}
#[doc = "Field `RX14M2` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M2_A, O>;
impl<'a, const O: u8> RX14M2_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M2_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M2_A::_1)
    }
}
#[doc = "Field `RX14M3` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M3_R = crate::BitReader<RX14M3_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M3_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M3_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M3_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M3_A {
        match self.bits {
            false => RX14M3_A::_0,
            true => RX14M3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M3_A::_1
    }
}
#[doc = "Field `RX14M3` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M3_A, O>;
impl<'a, const O: u8> RX14M3_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M3_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M3_A::_1)
    }
}
#[doc = "Field `RX14M4` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M4_R = crate::BitReader<RX14M4_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M4_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M4_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M4_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M4_A {
        match self.bits {
            false => RX14M4_A::_0,
            true => RX14M4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M4_A::_1
    }
}
#[doc = "Field `RX14M4` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M4_A, O>;
impl<'a, const O: u8> RX14M4_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M4_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M4_A::_1)
    }
}
#[doc = "Field `RX14M5` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M5_R = crate::BitReader<RX14M5_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M5_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M5_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M5_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M5_A {
        match self.bits {
            false => RX14M5_A::_0,
            true => RX14M5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M5_A::_1
    }
}
#[doc = "Field `RX14M5` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M5_A, O>;
impl<'a, const O: u8> RX14M5_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M5_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M5_A::_1)
    }
}
#[doc = "Field `RX14M6` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M6_R = crate::BitReader<RX14M6_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M6_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M6_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M6_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M6_A {
        match self.bits {
            false => RX14M6_A::_0,
            true => RX14M6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M6_A::_1
    }
}
#[doc = "Field `RX14M6` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M6_A, O>;
impl<'a, const O: u8> RX14M6_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M6_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M6_A::_1)
    }
}
#[doc = "Field `RX14M7` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M7_R = crate::BitReader<RX14M7_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M7_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M7_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M7_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M7_A {
        match self.bits {
            false => RX14M7_A::_0,
            true => RX14M7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M7_A::_1
    }
}
#[doc = "Field `RX14M7` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M7_A, O>;
impl<'a, const O: u8> RX14M7_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M7_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M7_A::_1)
    }
}
#[doc = "Field `RX14M8` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M8_R = crate::BitReader<RX14M8_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M8_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M8_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M8_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M8_A {
        match self.bits {
            false => RX14M8_A::_0,
            true => RX14M8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M8_A::_1
    }
}
#[doc = "Field `RX14M8` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M8_A, O>;
impl<'a, const O: u8> RX14M8_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M8_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M8_A::_1)
    }
}
#[doc = "Field `RX14M9` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M9_R = crate::BitReader<RX14M9_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M9_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M9_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M9_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M9_A {
        match self.bits {
            false => RX14M9_A::_0,
            true => RX14M9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M9_A::_1
    }
}
#[doc = "Field `RX14M9` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M9_A, O>;
impl<'a, const O: u8> RX14M9_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M9_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M9_A::_1)
    }
}
#[doc = "Field `RX14M10` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M10_R = crate::BitReader<RX14M10_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M10_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M10_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M10_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M10_A {
        match self.bits {
            false => RX14M10_A::_0,
            true => RX14M10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M10_A::_1
    }
}
#[doc = "Field `RX14M10` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M10_A, O>;
impl<'a, const O: u8> RX14M10_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M10_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M10_A::_1)
    }
}
#[doc = "Field `RX14M11` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M11_R = crate::BitReader<RX14M11_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M11_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M11_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M11_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M11_A {
        match self.bits {
            false => RX14M11_A::_0,
            true => RX14M11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M11_A::_1
    }
}
#[doc = "Field `RX14M11` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M11_A, O>;
impl<'a, const O: u8> RX14M11_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M11_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M11_A::_1)
    }
}
#[doc = "Field `RX14M12` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M12_R = crate::BitReader<RX14M12_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M12_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M12_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M12_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M12_A {
        match self.bits {
            false => RX14M12_A::_0,
            true => RX14M12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M12_A::_1
    }
}
#[doc = "Field `RX14M12` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M12_A, O>;
impl<'a, const O: u8> RX14M12_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M12_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M12_A::_1)
    }
}
#[doc = "Field `RX14M13` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M13_R = crate::BitReader<RX14M13_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M13_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M13_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M13_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M13_A {
        match self.bits {
            false => RX14M13_A::_0,
            true => RX14M13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M13_A::_1
    }
}
#[doc = "Field `RX14M13` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M13_A, O>;
impl<'a, const O: u8> RX14M13_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M13_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M13_A::_1)
    }
}
#[doc = "Field `RX14M14` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M14_R = crate::BitReader<RX14M14_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M14_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M14_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M14_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M14_A {
        match self.bits {
            false => RX14M14_A::_0,
            true => RX14M14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M14_A::_1
    }
}
#[doc = "Field `RX14M14` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M14_A, O>;
impl<'a, const O: u8> RX14M14_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M14_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M14_A::_1)
    }
}
#[doc = "Field `RX14M15` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M15_R = crate::BitReader<RX14M15_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M15_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M15_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M15_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M15_A {
        match self.bits {
            false => RX14M15_A::_0,
            true => RX14M15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M15_A::_1
    }
}
#[doc = "Field `RX14M15` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M15_A, O>;
impl<'a, const O: u8> RX14M15_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M15_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M15_A::_1)
    }
}
#[doc = "Field `RX14M16` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M16_R = crate::BitReader<RX14M16_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M16_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M16_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M16_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M16_A {
        match self.bits {
            false => RX14M16_A::_0,
            true => RX14M16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M16_A::_1
    }
}
#[doc = "Field `RX14M16` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M16_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M16_A, O>;
impl<'a, const O: u8> RX14M16_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M16_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M16_A::_1)
    }
}
#[doc = "Field `RX14M17` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M17_R = crate::BitReader<RX14M17_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M17_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M17_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M17_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M17_A {
        match self.bits {
            false => RX14M17_A::_0,
            true => RX14M17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M17_A::_1
    }
}
#[doc = "Field `RX14M17` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M17_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M17_A, O>;
impl<'a, const O: u8> RX14M17_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M17_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M17_A::_1)
    }
}
#[doc = "Field `RX14M18` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M18_R = crate::BitReader<RX14M18_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M18_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M18_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M18_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M18_A {
        match self.bits {
            false => RX14M18_A::_0,
            true => RX14M18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M18_A::_1
    }
}
#[doc = "Field `RX14M18` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M18_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M18_A, O>;
impl<'a, const O: u8> RX14M18_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M18_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M18_A::_1)
    }
}
#[doc = "Field `RX14M19` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M19_R = crate::BitReader<RX14M19_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M19_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M19_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M19_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M19_A {
        match self.bits {
            false => RX14M19_A::_0,
            true => RX14M19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M19_A::_1
    }
}
#[doc = "Field `RX14M19` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M19_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M19_A, O>;
impl<'a, const O: u8> RX14M19_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M19_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M19_A::_1)
    }
}
#[doc = "Field `RX14M20` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M20_R = crate::BitReader<RX14M20_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M20_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M20_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M20_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M20_A {
        match self.bits {
            false => RX14M20_A::_0,
            true => RX14M20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M20_A::_1
    }
}
#[doc = "Field `RX14M20` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M20_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M20_A, O>;
impl<'a, const O: u8> RX14M20_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M20_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M20_A::_1)
    }
}
#[doc = "Field `RX14M21` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M21_R = crate::BitReader<RX14M21_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M21_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M21_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M21_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M21_A {
        match self.bits {
            false => RX14M21_A::_0,
            true => RX14M21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M21_A::_1
    }
}
#[doc = "Field `RX14M21` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M21_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M21_A, O>;
impl<'a, const O: u8> RX14M21_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M21_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M21_A::_1)
    }
}
#[doc = "Field `RX14M22` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M22_R = crate::BitReader<RX14M22_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M22_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M22_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M22_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M22_A {
        match self.bits {
            false => RX14M22_A::_0,
            true => RX14M22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M22_A::_1
    }
}
#[doc = "Field `RX14M22` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M22_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M22_A, O>;
impl<'a, const O: u8> RX14M22_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M22_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M22_A::_1)
    }
}
#[doc = "Field `RX14M23` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M23_R = crate::BitReader<RX14M23_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M23_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M23_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M23_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M23_A {
        match self.bits {
            false => RX14M23_A::_0,
            true => RX14M23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M23_A::_1
    }
}
#[doc = "Field `RX14M23` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M23_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M23_A, O>;
impl<'a, const O: u8> RX14M23_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M23_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M23_A::_1)
    }
}
#[doc = "Field `RX14M24` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M24_R = crate::BitReader<RX14M24_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M24_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M24_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M24_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M24_A {
        match self.bits {
            false => RX14M24_A::_0,
            true => RX14M24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M24_A::_1
    }
}
#[doc = "Field `RX14M24` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M24_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M24_A, O>;
impl<'a, const O: u8> RX14M24_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M24_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M24_A::_1)
    }
}
#[doc = "Field `RX14M25` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M25_R = crate::BitReader<RX14M25_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M25_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M25_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M25_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M25_A {
        match self.bits {
            false => RX14M25_A::_0,
            true => RX14M25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M25_A::_1
    }
}
#[doc = "Field `RX14M25` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M25_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M25_A, O>;
impl<'a, const O: u8> RX14M25_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M25_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M25_A::_1)
    }
}
#[doc = "Field `RX14M26` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M26_R = crate::BitReader<RX14M26_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M26_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M26_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M26_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M26_A {
        match self.bits {
            false => RX14M26_A::_0,
            true => RX14M26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M26_A::_1
    }
}
#[doc = "Field `RX14M26` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M26_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M26_A, O>;
impl<'a, const O: u8> RX14M26_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M26_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M26_A::_1)
    }
}
#[doc = "Field `RX14M27` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M27_R = crate::BitReader<RX14M27_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M27_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M27_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M27_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M27_A {
        match self.bits {
            false => RX14M27_A::_0,
            true => RX14M27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M27_A::_1
    }
}
#[doc = "Field `RX14M27` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M27_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M27_A, O>;
impl<'a, const O: u8> RX14M27_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M27_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M27_A::_1)
    }
}
#[doc = "Field `RX14M28` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M28_R = crate::BitReader<RX14M28_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M28_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M28_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M28_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M28_A {
        match self.bits {
            false => RX14M28_A::_0,
            true => RX14M28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M28_A::_1
    }
}
#[doc = "Field `RX14M28` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M28_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M28_A, O>;
impl<'a, const O: u8> RX14M28_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M28_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M28_A::_1)
    }
}
#[doc = "Field `RX14M29` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M29_R = crate::BitReader<RX14M29_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M29_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M29_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M29_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M29_A {
        match self.bits {
            false => RX14M29_A::_0,
            true => RX14M29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M29_A::_1
    }
}
#[doc = "Field `RX14M29` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M29_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M29_A, O>;
impl<'a, const O: u8> RX14M29_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M29_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M29_A::_1)
    }
}
#[doc = "Field `RX14M30` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M30_R = crate::BitReader<RX14M30_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M30_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M30_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M30_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M30_A {
        match self.bits {
            false => RX14M30_A::_0,
            true => RX14M30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M30_A::_1
    }
}
#[doc = "Field `RX14M30` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M30_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M30_A, O>;
impl<'a, const O: u8> RX14M30_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M30_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M30_A::_1)
    }
}
#[doc = "Field `RX14M31` reader - Rx Buffer 14 Mask Bits"]
pub type RX14M31_R = crate::BitReader<RX14M31_A>;
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX14M31_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M31_A> for bool {
    #[inline(always)]
    fn from(variant: RX14M31_A) -> Self {
        variant as u8 != 0
    }
}
impl RX14M31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX14M31_A {
        match self.bits {
            false => RX14M31_A::_0,
            true => RX14M31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M31_A::_1
    }
}
#[doc = "Field `RX14M31` writer - Rx Buffer 14 Mask Bits"]
pub type RX14M31_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX14MASK_SPEC, RX14M31_A, O>;
impl<'a, const O: u8> RX14M31_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M31_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m0(&self) -> RX14M0_R {
        RX14M0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m1(&self) -> RX14M1_R {
        RX14M1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m2(&self) -> RX14M2_R {
        RX14M2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m3(&self) -> RX14M3_R {
        RX14M3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m4(&self) -> RX14M4_R {
        RX14M4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m5(&self) -> RX14M5_R {
        RX14M5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m6(&self) -> RX14M6_R {
        RX14M6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m7(&self) -> RX14M7_R {
        RX14M7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m8(&self) -> RX14M8_R {
        RX14M8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m9(&self) -> RX14M9_R {
        RX14M9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m10(&self) -> RX14M10_R {
        RX14M10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m11(&self) -> RX14M11_R {
        RX14M11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m12(&self) -> RX14M12_R {
        RX14M12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m13(&self) -> RX14M13_R {
        RX14M13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m14(&self) -> RX14M14_R {
        RX14M14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m15(&self) -> RX14M15_R {
        RX14M15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m16(&self) -> RX14M16_R {
        RX14M16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m17(&self) -> RX14M17_R {
        RX14M17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m18(&self) -> RX14M18_R {
        RX14M18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m19(&self) -> RX14M19_R {
        RX14M19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m20(&self) -> RX14M20_R {
        RX14M20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m21(&self) -> RX14M21_R {
        RX14M21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m22(&self) -> RX14M22_R {
        RX14M22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m23(&self) -> RX14M23_R {
        RX14M23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m24(&self) -> RX14M24_R {
        RX14M24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m25(&self) -> RX14M25_R {
        RX14M25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m26(&self) -> RX14M26_R {
        RX14M26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m27(&self) -> RX14M27_R {
        RX14M27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m28(&self) -> RX14M28_R {
        RX14M28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m29(&self) -> RX14M29_R {
        RX14M29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m30(&self) -> RX14M30_R {
        RX14M30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m31(&self) -> RX14M31_R {
        RX14M31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m0(&mut self) -> RX14M0_W<0> {
        RX14M0_W::new(self)
    }
    #[doc = "Bit 1 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m1(&mut self) -> RX14M1_W<1> {
        RX14M1_W::new(self)
    }
    #[doc = "Bit 2 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m2(&mut self) -> RX14M2_W<2> {
        RX14M2_W::new(self)
    }
    #[doc = "Bit 3 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m3(&mut self) -> RX14M3_W<3> {
        RX14M3_W::new(self)
    }
    #[doc = "Bit 4 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m4(&mut self) -> RX14M4_W<4> {
        RX14M4_W::new(self)
    }
    #[doc = "Bit 5 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m5(&mut self) -> RX14M5_W<5> {
        RX14M5_W::new(self)
    }
    #[doc = "Bit 6 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m6(&mut self) -> RX14M6_W<6> {
        RX14M6_W::new(self)
    }
    #[doc = "Bit 7 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m7(&mut self) -> RX14M7_W<7> {
        RX14M7_W::new(self)
    }
    #[doc = "Bit 8 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m8(&mut self) -> RX14M8_W<8> {
        RX14M8_W::new(self)
    }
    #[doc = "Bit 9 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m9(&mut self) -> RX14M9_W<9> {
        RX14M9_W::new(self)
    }
    #[doc = "Bit 10 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m10(&mut self) -> RX14M10_W<10> {
        RX14M10_W::new(self)
    }
    #[doc = "Bit 11 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m11(&mut self) -> RX14M11_W<11> {
        RX14M11_W::new(self)
    }
    #[doc = "Bit 12 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m12(&mut self) -> RX14M12_W<12> {
        RX14M12_W::new(self)
    }
    #[doc = "Bit 13 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m13(&mut self) -> RX14M13_W<13> {
        RX14M13_W::new(self)
    }
    #[doc = "Bit 14 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m14(&mut self) -> RX14M14_W<14> {
        RX14M14_W::new(self)
    }
    #[doc = "Bit 15 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m15(&mut self) -> RX14M15_W<15> {
        RX14M15_W::new(self)
    }
    #[doc = "Bit 16 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m16(&mut self) -> RX14M16_W<16> {
        RX14M16_W::new(self)
    }
    #[doc = "Bit 17 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m17(&mut self) -> RX14M17_W<17> {
        RX14M17_W::new(self)
    }
    #[doc = "Bit 18 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m18(&mut self) -> RX14M18_W<18> {
        RX14M18_W::new(self)
    }
    #[doc = "Bit 19 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m19(&mut self) -> RX14M19_W<19> {
        RX14M19_W::new(self)
    }
    #[doc = "Bit 20 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m20(&mut self) -> RX14M20_W<20> {
        RX14M20_W::new(self)
    }
    #[doc = "Bit 21 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m21(&mut self) -> RX14M21_W<21> {
        RX14M21_W::new(self)
    }
    #[doc = "Bit 22 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m22(&mut self) -> RX14M22_W<22> {
        RX14M22_W::new(self)
    }
    #[doc = "Bit 23 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m23(&mut self) -> RX14M23_W<23> {
        RX14M23_W::new(self)
    }
    #[doc = "Bit 24 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m24(&mut self) -> RX14M24_W<24> {
        RX14M24_W::new(self)
    }
    #[doc = "Bit 25 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m25(&mut self) -> RX14M25_W<25> {
        RX14M25_W::new(self)
    }
    #[doc = "Bit 26 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m26(&mut self) -> RX14M26_W<26> {
        RX14M26_W::new(self)
    }
    #[doc = "Bit 27 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m27(&mut self) -> RX14M27_W<27> {
        RX14M27_W::new(self)
    }
    #[doc = "Bit 28 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m28(&mut self) -> RX14M28_W<28> {
        RX14M28_W::new(self)
    }
    #[doc = "Bit 29 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m29(&mut self) -> RX14M29_W<29> {
        RX14M29_W::new(self)
    }
    #[doc = "Bit 30 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m30(&mut self) -> RX14M30_W<30> {
        RX14M30_W::new(self)
    }
    #[doc = "Bit 31 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx14m31(&mut self) -> RX14M31_W<31> {
        RX14M31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx 14 Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx14mask](index.html) module"]
pub struct RX14MASK_SPEC;
impl crate::RegisterSpec for RX14MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx14mask::R](R) reader structure"]
impl crate::Readable for RX14MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx14mask::W](W) writer structure"]
impl crate::Writable for RX14MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX14MASK to value 0xffff_ffff"]
impl crate::Resettable for RX14MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

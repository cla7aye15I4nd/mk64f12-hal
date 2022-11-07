#[doc = "Register `RX15MASK` reader"]
pub struct R(crate::R<RX15MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX15MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX15MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX15MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX15MASK` writer"]
pub struct W(crate::W<RX15MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX15MASK_SPEC>;
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
impl From<crate::W<RX15MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX15MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX15M0` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M0_R = crate::BitReader<RX15M0_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M0_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M0_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M0_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M0_A {
        match self.bits {
            false => RX15M0_A::_0,
            true => RX15M0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M0_A::_1
    }
}
#[doc = "Field `RX15M0` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M0_A, O>;
impl<'a, const O: u8> RX15M0_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M0_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M0_A::_1)
    }
}
#[doc = "Field `RX15M1` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M1_R = crate::BitReader<RX15M1_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M1_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M1_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M1_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M1_A {
        match self.bits {
            false => RX15M1_A::_0,
            true => RX15M1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M1_A::_1
    }
}
#[doc = "Field `RX15M1` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M1_A, O>;
impl<'a, const O: u8> RX15M1_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M1_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M1_A::_1)
    }
}
#[doc = "Field `RX15M2` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M2_R = crate::BitReader<RX15M2_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M2_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M2_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M2_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M2_A {
        match self.bits {
            false => RX15M2_A::_0,
            true => RX15M2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M2_A::_1
    }
}
#[doc = "Field `RX15M2` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M2_A, O>;
impl<'a, const O: u8> RX15M2_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M2_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M2_A::_1)
    }
}
#[doc = "Field `RX15M3` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M3_R = crate::BitReader<RX15M3_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M3_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M3_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M3_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M3_A {
        match self.bits {
            false => RX15M3_A::_0,
            true => RX15M3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M3_A::_1
    }
}
#[doc = "Field `RX15M3` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M3_A, O>;
impl<'a, const O: u8> RX15M3_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M3_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M3_A::_1)
    }
}
#[doc = "Field `RX15M4` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M4_R = crate::BitReader<RX15M4_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M4_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M4_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M4_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M4_A {
        match self.bits {
            false => RX15M4_A::_0,
            true => RX15M4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M4_A::_1
    }
}
#[doc = "Field `RX15M4` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M4_A, O>;
impl<'a, const O: u8> RX15M4_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M4_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M4_A::_1)
    }
}
#[doc = "Field `RX15M5` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M5_R = crate::BitReader<RX15M5_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M5_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M5_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M5_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M5_A {
        match self.bits {
            false => RX15M5_A::_0,
            true => RX15M5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M5_A::_1
    }
}
#[doc = "Field `RX15M5` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M5_A, O>;
impl<'a, const O: u8> RX15M5_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M5_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M5_A::_1)
    }
}
#[doc = "Field `RX15M6` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M6_R = crate::BitReader<RX15M6_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M6_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M6_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M6_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M6_A {
        match self.bits {
            false => RX15M6_A::_0,
            true => RX15M6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M6_A::_1
    }
}
#[doc = "Field `RX15M6` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M6_A, O>;
impl<'a, const O: u8> RX15M6_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M6_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M6_A::_1)
    }
}
#[doc = "Field `RX15M7` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M7_R = crate::BitReader<RX15M7_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M7_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M7_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M7_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M7_A {
        match self.bits {
            false => RX15M7_A::_0,
            true => RX15M7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M7_A::_1
    }
}
#[doc = "Field `RX15M7` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M7_A, O>;
impl<'a, const O: u8> RX15M7_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M7_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M7_A::_1)
    }
}
#[doc = "Field `RX15M8` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M8_R = crate::BitReader<RX15M8_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M8_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M8_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M8_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M8_A {
        match self.bits {
            false => RX15M8_A::_0,
            true => RX15M8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M8_A::_1
    }
}
#[doc = "Field `RX15M8` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M8_A, O>;
impl<'a, const O: u8> RX15M8_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M8_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M8_A::_1)
    }
}
#[doc = "Field `RX15M9` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M9_R = crate::BitReader<RX15M9_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M9_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M9_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M9_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M9_A {
        match self.bits {
            false => RX15M9_A::_0,
            true => RX15M9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M9_A::_1
    }
}
#[doc = "Field `RX15M9` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M9_A, O>;
impl<'a, const O: u8> RX15M9_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M9_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M9_A::_1)
    }
}
#[doc = "Field `RX15M10` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M10_R = crate::BitReader<RX15M10_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M10_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M10_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M10_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M10_A {
        match self.bits {
            false => RX15M10_A::_0,
            true => RX15M10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M10_A::_1
    }
}
#[doc = "Field `RX15M10` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M10_A, O>;
impl<'a, const O: u8> RX15M10_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M10_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M10_A::_1)
    }
}
#[doc = "Field `RX15M11` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M11_R = crate::BitReader<RX15M11_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M11_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M11_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M11_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M11_A {
        match self.bits {
            false => RX15M11_A::_0,
            true => RX15M11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M11_A::_1
    }
}
#[doc = "Field `RX15M11` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M11_A, O>;
impl<'a, const O: u8> RX15M11_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M11_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M11_A::_1)
    }
}
#[doc = "Field `RX15M12` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M12_R = crate::BitReader<RX15M12_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M12_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M12_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M12_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M12_A {
        match self.bits {
            false => RX15M12_A::_0,
            true => RX15M12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M12_A::_1
    }
}
#[doc = "Field `RX15M12` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M12_A, O>;
impl<'a, const O: u8> RX15M12_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M12_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M12_A::_1)
    }
}
#[doc = "Field `RX15M13` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M13_R = crate::BitReader<RX15M13_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M13_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M13_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M13_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M13_A {
        match self.bits {
            false => RX15M13_A::_0,
            true => RX15M13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M13_A::_1
    }
}
#[doc = "Field `RX15M13` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M13_A, O>;
impl<'a, const O: u8> RX15M13_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M13_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M13_A::_1)
    }
}
#[doc = "Field `RX15M14` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M14_R = crate::BitReader<RX15M14_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M14_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M14_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M14_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M14_A {
        match self.bits {
            false => RX15M14_A::_0,
            true => RX15M14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M14_A::_1
    }
}
#[doc = "Field `RX15M14` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M14_A, O>;
impl<'a, const O: u8> RX15M14_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M14_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M14_A::_1)
    }
}
#[doc = "Field `RX15M15` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M15_R = crate::BitReader<RX15M15_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M15_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M15_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M15_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M15_A {
        match self.bits {
            false => RX15M15_A::_0,
            true => RX15M15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M15_A::_1
    }
}
#[doc = "Field `RX15M15` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M15_A, O>;
impl<'a, const O: u8> RX15M15_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M15_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M15_A::_1)
    }
}
#[doc = "Field `RX15M16` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M16_R = crate::BitReader<RX15M16_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M16_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M16_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M16_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M16_A {
        match self.bits {
            false => RX15M16_A::_0,
            true => RX15M16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M16_A::_1
    }
}
#[doc = "Field `RX15M16` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M16_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M16_A, O>;
impl<'a, const O: u8> RX15M16_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M16_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M16_A::_1)
    }
}
#[doc = "Field `RX15M17` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M17_R = crate::BitReader<RX15M17_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M17_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M17_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M17_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M17_A {
        match self.bits {
            false => RX15M17_A::_0,
            true => RX15M17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M17_A::_1
    }
}
#[doc = "Field `RX15M17` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M17_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M17_A, O>;
impl<'a, const O: u8> RX15M17_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M17_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M17_A::_1)
    }
}
#[doc = "Field `RX15M18` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M18_R = crate::BitReader<RX15M18_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M18_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M18_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M18_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M18_A {
        match self.bits {
            false => RX15M18_A::_0,
            true => RX15M18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M18_A::_1
    }
}
#[doc = "Field `RX15M18` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M18_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M18_A, O>;
impl<'a, const O: u8> RX15M18_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M18_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M18_A::_1)
    }
}
#[doc = "Field `RX15M19` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M19_R = crate::BitReader<RX15M19_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M19_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M19_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M19_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M19_A {
        match self.bits {
            false => RX15M19_A::_0,
            true => RX15M19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M19_A::_1
    }
}
#[doc = "Field `RX15M19` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M19_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M19_A, O>;
impl<'a, const O: u8> RX15M19_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M19_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M19_A::_1)
    }
}
#[doc = "Field `RX15M20` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M20_R = crate::BitReader<RX15M20_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M20_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M20_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M20_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M20_A {
        match self.bits {
            false => RX15M20_A::_0,
            true => RX15M20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M20_A::_1
    }
}
#[doc = "Field `RX15M20` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M20_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M20_A, O>;
impl<'a, const O: u8> RX15M20_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M20_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M20_A::_1)
    }
}
#[doc = "Field `RX15M21` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M21_R = crate::BitReader<RX15M21_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M21_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M21_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M21_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M21_A {
        match self.bits {
            false => RX15M21_A::_0,
            true => RX15M21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M21_A::_1
    }
}
#[doc = "Field `RX15M21` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M21_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M21_A, O>;
impl<'a, const O: u8> RX15M21_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M21_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M21_A::_1)
    }
}
#[doc = "Field `RX15M22` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M22_R = crate::BitReader<RX15M22_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M22_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M22_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M22_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M22_A {
        match self.bits {
            false => RX15M22_A::_0,
            true => RX15M22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M22_A::_1
    }
}
#[doc = "Field `RX15M22` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M22_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M22_A, O>;
impl<'a, const O: u8> RX15M22_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M22_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M22_A::_1)
    }
}
#[doc = "Field `RX15M23` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M23_R = crate::BitReader<RX15M23_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M23_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M23_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M23_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M23_A {
        match self.bits {
            false => RX15M23_A::_0,
            true => RX15M23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M23_A::_1
    }
}
#[doc = "Field `RX15M23` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M23_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M23_A, O>;
impl<'a, const O: u8> RX15M23_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M23_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M23_A::_1)
    }
}
#[doc = "Field `RX15M24` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M24_R = crate::BitReader<RX15M24_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M24_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M24_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M24_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M24_A {
        match self.bits {
            false => RX15M24_A::_0,
            true => RX15M24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M24_A::_1
    }
}
#[doc = "Field `RX15M24` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M24_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M24_A, O>;
impl<'a, const O: u8> RX15M24_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M24_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M24_A::_1)
    }
}
#[doc = "Field `RX15M25` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M25_R = crate::BitReader<RX15M25_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M25_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M25_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M25_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M25_A {
        match self.bits {
            false => RX15M25_A::_0,
            true => RX15M25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M25_A::_1
    }
}
#[doc = "Field `RX15M25` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M25_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M25_A, O>;
impl<'a, const O: u8> RX15M25_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M25_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M25_A::_1)
    }
}
#[doc = "Field `RX15M26` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M26_R = crate::BitReader<RX15M26_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M26_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M26_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M26_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M26_A {
        match self.bits {
            false => RX15M26_A::_0,
            true => RX15M26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M26_A::_1
    }
}
#[doc = "Field `RX15M26` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M26_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M26_A, O>;
impl<'a, const O: u8> RX15M26_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M26_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M26_A::_1)
    }
}
#[doc = "Field `RX15M27` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M27_R = crate::BitReader<RX15M27_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M27_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M27_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M27_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M27_A {
        match self.bits {
            false => RX15M27_A::_0,
            true => RX15M27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M27_A::_1
    }
}
#[doc = "Field `RX15M27` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M27_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M27_A, O>;
impl<'a, const O: u8> RX15M27_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M27_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M27_A::_1)
    }
}
#[doc = "Field `RX15M28` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M28_R = crate::BitReader<RX15M28_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M28_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M28_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M28_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M28_A {
        match self.bits {
            false => RX15M28_A::_0,
            true => RX15M28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M28_A::_1
    }
}
#[doc = "Field `RX15M28` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M28_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M28_A, O>;
impl<'a, const O: u8> RX15M28_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M28_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M28_A::_1)
    }
}
#[doc = "Field `RX15M29` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M29_R = crate::BitReader<RX15M29_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M29_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M29_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M29_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M29_A {
        match self.bits {
            false => RX15M29_A::_0,
            true => RX15M29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M29_A::_1
    }
}
#[doc = "Field `RX15M29` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M29_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M29_A, O>;
impl<'a, const O: u8> RX15M29_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M29_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M29_A::_1)
    }
}
#[doc = "Field `RX15M30` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M30_R = crate::BitReader<RX15M30_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M30_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M30_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M30_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M30_A {
        match self.bits {
            false => RX15M30_A::_0,
            true => RX15M30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M30_A::_1
    }
}
#[doc = "Field `RX15M30` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M30_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M30_A, O>;
impl<'a, const O: u8> RX15M30_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M30_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M30_A::_1)
    }
}
#[doc = "Field `RX15M31` reader - Rx Buffer 15 Mask Bits"]
pub type RX15M31_R = crate::BitReader<RX15M31_A>;
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX15M31_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX15M31_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M31_A) -> Self {
        variant as u8 != 0
    }
}
impl RX15M31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M31_A {
        match self.bits {
            false => RX15M31_A::_0,
            true => RX15M31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M31_A::_1
    }
}
#[doc = "Field `RX15M31` writer - Rx Buffer 15 Mask Bits"]
pub type RX15M31_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX15MASK_SPEC, RX15M31_A, O>;
impl<'a, const O: u8> RX15M31_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M31_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m0(&self) -> RX15M0_R {
        RX15M0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m1(&self) -> RX15M1_R {
        RX15M1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m2(&self) -> RX15M2_R {
        RX15M2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m3(&self) -> RX15M3_R {
        RX15M3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m4(&self) -> RX15M4_R {
        RX15M4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m5(&self) -> RX15M5_R {
        RX15M5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m6(&self) -> RX15M6_R {
        RX15M6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m7(&self) -> RX15M7_R {
        RX15M7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m8(&self) -> RX15M8_R {
        RX15M8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m9(&self) -> RX15M9_R {
        RX15M9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m10(&self) -> RX15M10_R {
        RX15M10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m11(&self) -> RX15M11_R {
        RX15M11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m12(&self) -> RX15M12_R {
        RX15M12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m13(&self) -> RX15M13_R {
        RX15M13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m14(&self) -> RX15M14_R {
        RX15M14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m15(&self) -> RX15M15_R {
        RX15M15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m16(&self) -> RX15M16_R {
        RX15M16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m17(&self) -> RX15M17_R {
        RX15M17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m18(&self) -> RX15M18_R {
        RX15M18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m19(&self) -> RX15M19_R {
        RX15M19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m20(&self) -> RX15M20_R {
        RX15M20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m21(&self) -> RX15M21_R {
        RX15M21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m22(&self) -> RX15M22_R {
        RX15M22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m23(&self) -> RX15M23_R {
        RX15M23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m24(&self) -> RX15M24_R {
        RX15M24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m25(&self) -> RX15M25_R {
        RX15M25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m26(&self) -> RX15M26_R {
        RX15M26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m27(&self) -> RX15M27_R {
        RX15M27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m28(&self) -> RX15M28_R {
        RX15M28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m29(&self) -> RX15M29_R {
        RX15M29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m30(&self) -> RX15M30_R {
        RX15M30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m31(&self) -> RX15M31_R {
        RX15M31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m0(&mut self) -> RX15M0_W<0> {
        RX15M0_W::new(self)
    }
    #[doc = "Bit 1 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m1(&mut self) -> RX15M1_W<1> {
        RX15M1_W::new(self)
    }
    #[doc = "Bit 2 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m2(&mut self) -> RX15M2_W<2> {
        RX15M2_W::new(self)
    }
    #[doc = "Bit 3 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m3(&mut self) -> RX15M3_W<3> {
        RX15M3_W::new(self)
    }
    #[doc = "Bit 4 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m4(&mut self) -> RX15M4_W<4> {
        RX15M4_W::new(self)
    }
    #[doc = "Bit 5 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m5(&mut self) -> RX15M5_W<5> {
        RX15M5_W::new(self)
    }
    #[doc = "Bit 6 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m6(&mut self) -> RX15M6_W<6> {
        RX15M6_W::new(self)
    }
    #[doc = "Bit 7 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m7(&mut self) -> RX15M7_W<7> {
        RX15M7_W::new(self)
    }
    #[doc = "Bit 8 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m8(&mut self) -> RX15M8_W<8> {
        RX15M8_W::new(self)
    }
    #[doc = "Bit 9 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m9(&mut self) -> RX15M9_W<9> {
        RX15M9_W::new(self)
    }
    #[doc = "Bit 10 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m10(&mut self) -> RX15M10_W<10> {
        RX15M10_W::new(self)
    }
    #[doc = "Bit 11 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m11(&mut self) -> RX15M11_W<11> {
        RX15M11_W::new(self)
    }
    #[doc = "Bit 12 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m12(&mut self) -> RX15M12_W<12> {
        RX15M12_W::new(self)
    }
    #[doc = "Bit 13 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m13(&mut self) -> RX15M13_W<13> {
        RX15M13_W::new(self)
    }
    #[doc = "Bit 14 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m14(&mut self) -> RX15M14_W<14> {
        RX15M14_W::new(self)
    }
    #[doc = "Bit 15 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m15(&mut self) -> RX15M15_W<15> {
        RX15M15_W::new(self)
    }
    #[doc = "Bit 16 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m16(&mut self) -> RX15M16_W<16> {
        RX15M16_W::new(self)
    }
    #[doc = "Bit 17 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m17(&mut self) -> RX15M17_W<17> {
        RX15M17_W::new(self)
    }
    #[doc = "Bit 18 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m18(&mut self) -> RX15M18_W<18> {
        RX15M18_W::new(self)
    }
    #[doc = "Bit 19 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m19(&mut self) -> RX15M19_W<19> {
        RX15M19_W::new(self)
    }
    #[doc = "Bit 20 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m20(&mut self) -> RX15M20_W<20> {
        RX15M20_W::new(self)
    }
    #[doc = "Bit 21 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m21(&mut self) -> RX15M21_W<21> {
        RX15M21_W::new(self)
    }
    #[doc = "Bit 22 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m22(&mut self) -> RX15M22_W<22> {
        RX15M22_W::new(self)
    }
    #[doc = "Bit 23 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m23(&mut self) -> RX15M23_W<23> {
        RX15M23_W::new(self)
    }
    #[doc = "Bit 24 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m24(&mut self) -> RX15M24_W<24> {
        RX15M24_W::new(self)
    }
    #[doc = "Bit 25 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m25(&mut self) -> RX15M25_W<25> {
        RX15M25_W::new(self)
    }
    #[doc = "Bit 26 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m26(&mut self) -> RX15M26_W<26> {
        RX15M26_W::new(self)
    }
    #[doc = "Bit 27 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m27(&mut self) -> RX15M27_W<27> {
        RX15M27_W::new(self)
    }
    #[doc = "Bit 28 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m28(&mut self) -> RX15M28_W<28> {
        RX15M28_W::new(self)
    }
    #[doc = "Bit 29 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m29(&mut self) -> RX15M29_W<29> {
        RX15M29_W::new(self)
    }
    #[doc = "Bit 30 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m30(&mut self) -> RX15M30_W<30> {
        RX15M30_W::new(self)
    }
    #[doc = "Bit 31 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m31(&mut self) -> RX15M31_W<31> {
        RX15M31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx 15 Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx15mask](index.html) module"]
pub struct RX15MASK_SPEC;
impl crate::RegisterSpec for RX15MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx15mask::R](R) reader structure"]
impl crate::Readable for RX15MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx15mask::W](W) writer structure"]
impl crate::Writable for RX15MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX15MASK to value 0xffff_ffff"]
impl crate::Resettable for RX15MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

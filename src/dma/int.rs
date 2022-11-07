#[doc = "Register `INT` reader"]
pub struct R(crate::R<INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT` writer"]
pub struct W(crate::W<INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SPEC>;
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
impl From<crate::W<INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT0` reader - Interrupt Request 0"]
pub type INT0_R = crate::BitReader<INT0_A>;
#[doc = "Interrupt Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT0_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT0_A> for bool {
    #[inline(always)]
    fn from(variant: INT0_A) -> Self {
        variant as u8 != 0
    }
}
impl INT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT0_A {
        match self.bits {
            false => INT0_A::_0,
            true => INT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT0_A::_1
    }
}
#[doc = "Field `INT0` writer - Interrupt Request 0"]
pub type INT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT0_A, O>;
impl<'a, const O: u8> INT0_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT0_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT0_A::_1)
    }
}
#[doc = "Field `INT1` reader - Interrupt Request 1"]
pub type INT1_R = crate::BitReader<INT1_A>;
#[doc = "Interrupt Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT1_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT1_A> for bool {
    #[inline(always)]
    fn from(variant: INT1_A) -> Self {
        variant as u8 != 0
    }
}
impl INT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT1_A {
        match self.bits {
            false => INT1_A::_0,
            true => INT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT1_A::_1
    }
}
#[doc = "Field `INT1` writer - Interrupt Request 1"]
pub type INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT1_A, O>;
impl<'a, const O: u8> INT1_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT1_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT1_A::_1)
    }
}
#[doc = "Field `INT2` reader - Interrupt Request 2"]
pub type INT2_R = crate::BitReader<INT2_A>;
#[doc = "Interrupt Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT2_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT2_A> for bool {
    #[inline(always)]
    fn from(variant: INT2_A) -> Self {
        variant as u8 != 0
    }
}
impl INT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT2_A {
        match self.bits {
            false => INT2_A::_0,
            true => INT2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT2_A::_1
    }
}
#[doc = "Field `INT2` writer - Interrupt Request 2"]
pub type INT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT2_A, O>;
impl<'a, const O: u8> INT2_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT2_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT2_A::_1)
    }
}
#[doc = "Field `INT3` reader - Interrupt Request 3"]
pub type INT3_R = crate::BitReader<INT3_A>;
#[doc = "Interrupt Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT3_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT3_A> for bool {
    #[inline(always)]
    fn from(variant: INT3_A) -> Self {
        variant as u8 != 0
    }
}
impl INT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT3_A {
        match self.bits {
            false => INT3_A::_0,
            true => INT3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT3_A::_1
    }
}
#[doc = "Field `INT3` writer - Interrupt Request 3"]
pub type INT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT3_A, O>;
impl<'a, const O: u8> INT3_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT3_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT3_A::_1)
    }
}
#[doc = "Field `INT4` reader - Interrupt Request 4"]
pub type INT4_R = crate::BitReader<INT4_A>;
#[doc = "Interrupt Request 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT4_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT4_A> for bool {
    #[inline(always)]
    fn from(variant: INT4_A) -> Self {
        variant as u8 != 0
    }
}
impl INT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT4_A {
        match self.bits {
            false => INT4_A::_0,
            true => INT4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT4_A::_1
    }
}
#[doc = "Field `INT4` writer - Interrupt Request 4"]
pub type INT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT4_A, O>;
impl<'a, const O: u8> INT4_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT4_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT4_A::_1)
    }
}
#[doc = "Field `INT5` reader - Interrupt Request 5"]
pub type INT5_R = crate::BitReader<INT5_A>;
#[doc = "Interrupt Request 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT5_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT5_A> for bool {
    #[inline(always)]
    fn from(variant: INT5_A) -> Self {
        variant as u8 != 0
    }
}
impl INT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT5_A {
        match self.bits {
            false => INT5_A::_0,
            true => INT5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT5_A::_1
    }
}
#[doc = "Field `INT5` writer - Interrupt Request 5"]
pub type INT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT5_A, O>;
impl<'a, const O: u8> INT5_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT5_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT5_A::_1)
    }
}
#[doc = "Field `INT6` reader - Interrupt Request 6"]
pub type INT6_R = crate::BitReader<INT6_A>;
#[doc = "Interrupt Request 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT6_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT6_A> for bool {
    #[inline(always)]
    fn from(variant: INT6_A) -> Self {
        variant as u8 != 0
    }
}
impl INT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT6_A {
        match self.bits {
            false => INT6_A::_0,
            true => INT6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT6_A::_1
    }
}
#[doc = "Field `INT6` writer - Interrupt Request 6"]
pub type INT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT6_A, O>;
impl<'a, const O: u8> INT6_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT6_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT6_A::_1)
    }
}
#[doc = "Field `INT7` reader - Interrupt Request 7"]
pub type INT7_R = crate::BitReader<INT7_A>;
#[doc = "Interrupt Request 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT7_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT7_A> for bool {
    #[inline(always)]
    fn from(variant: INT7_A) -> Self {
        variant as u8 != 0
    }
}
impl INT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT7_A {
        match self.bits {
            false => INT7_A::_0,
            true => INT7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT7_A::_1
    }
}
#[doc = "Field `INT7` writer - Interrupt Request 7"]
pub type INT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT7_A, O>;
impl<'a, const O: u8> INT7_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT7_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT7_A::_1)
    }
}
#[doc = "Field `INT8` reader - Interrupt Request 8"]
pub type INT8_R = crate::BitReader<INT8_A>;
#[doc = "Interrupt Request 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT8_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT8_A> for bool {
    #[inline(always)]
    fn from(variant: INT8_A) -> Self {
        variant as u8 != 0
    }
}
impl INT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT8_A {
        match self.bits {
            false => INT8_A::_0,
            true => INT8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT8_A::_1
    }
}
#[doc = "Field `INT8` writer - Interrupt Request 8"]
pub type INT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT8_A, O>;
impl<'a, const O: u8> INT8_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT8_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT8_A::_1)
    }
}
#[doc = "Field `INT9` reader - Interrupt Request 9"]
pub type INT9_R = crate::BitReader<INT9_A>;
#[doc = "Interrupt Request 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT9_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT9_A> for bool {
    #[inline(always)]
    fn from(variant: INT9_A) -> Self {
        variant as u8 != 0
    }
}
impl INT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT9_A {
        match self.bits {
            false => INT9_A::_0,
            true => INT9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT9_A::_1
    }
}
#[doc = "Field `INT9` writer - Interrupt Request 9"]
pub type INT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT9_A, O>;
impl<'a, const O: u8> INT9_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT9_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT9_A::_1)
    }
}
#[doc = "Field `INT10` reader - Interrupt Request 10"]
pub type INT10_R = crate::BitReader<INT10_A>;
#[doc = "Interrupt Request 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT10_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT10_A> for bool {
    #[inline(always)]
    fn from(variant: INT10_A) -> Self {
        variant as u8 != 0
    }
}
impl INT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT10_A {
        match self.bits {
            false => INT10_A::_0,
            true => INT10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT10_A::_1
    }
}
#[doc = "Field `INT10` writer - Interrupt Request 10"]
pub type INT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT10_A, O>;
impl<'a, const O: u8> INT10_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT10_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT10_A::_1)
    }
}
#[doc = "Field `INT11` reader - Interrupt Request 11"]
pub type INT11_R = crate::BitReader<INT11_A>;
#[doc = "Interrupt Request 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT11_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT11_A> for bool {
    #[inline(always)]
    fn from(variant: INT11_A) -> Self {
        variant as u8 != 0
    }
}
impl INT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT11_A {
        match self.bits {
            false => INT11_A::_0,
            true => INT11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT11_A::_1
    }
}
#[doc = "Field `INT11` writer - Interrupt Request 11"]
pub type INT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT11_A, O>;
impl<'a, const O: u8> INT11_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT11_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT11_A::_1)
    }
}
#[doc = "Field `INT12` reader - Interrupt Request 12"]
pub type INT12_R = crate::BitReader<INT12_A>;
#[doc = "Interrupt Request 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT12_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT12_A> for bool {
    #[inline(always)]
    fn from(variant: INT12_A) -> Self {
        variant as u8 != 0
    }
}
impl INT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT12_A {
        match self.bits {
            false => INT12_A::_0,
            true => INT12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT12_A::_1
    }
}
#[doc = "Field `INT12` writer - Interrupt Request 12"]
pub type INT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT12_A, O>;
impl<'a, const O: u8> INT12_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT12_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT12_A::_1)
    }
}
#[doc = "Field `INT13` reader - Interrupt Request 13"]
pub type INT13_R = crate::BitReader<INT13_A>;
#[doc = "Interrupt Request 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT13_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT13_A> for bool {
    #[inline(always)]
    fn from(variant: INT13_A) -> Self {
        variant as u8 != 0
    }
}
impl INT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT13_A {
        match self.bits {
            false => INT13_A::_0,
            true => INT13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT13_A::_1
    }
}
#[doc = "Field `INT13` writer - Interrupt Request 13"]
pub type INT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT13_A, O>;
impl<'a, const O: u8> INT13_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT13_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT13_A::_1)
    }
}
#[doc = "Field `INT14` reader - Interrupt Request 14"]
pub type INT14_R = crate::BitReader<INT14_A>;
#[doc = "Interrupt Request 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT14_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT14_A> for bool {
    #[inline(always)]
    fn from(variant: INT14_A) -> Self {
        variant as u8 != 0
    }
}
impl INT14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT14_A {
        match self.bits {
            false => INT14_A::_0,
            true => INT14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT14_A::_1
    }
}
#[doc = "Field `INT14` writer - Interrupt Request 14"]
pub type INT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT14_A, O>;
impl<'a, const O: u8> INT14_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT14_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT14_A::_1)
    }
}
#[doc = "Field `INT15` reader - Interrupt Request 15"]
pub type INT15_R = crate::BitReader<INT15_A>;
#[doc = "Interrupt Request 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT15_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<INT15_A> for bool {
    #[inline(always)]
    fn from(variant: INT15_A) -> Self {
        variant as u8 != 0
    }
}
impl INT15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT15_A {
        match self.bits {
            false => INT15_A::_0,
            true => INT15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT15_A::_1
    }
}
#[doc = "Field `INT15` writer - Interrupt Request 15"]
pub type INT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, INT15_A, O>;
impl<'a, const O: u8> INT15_W<'a, O> {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT15_A::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> INT0_W<0> {
        INT0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> INT1_W<1> {
        INT1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> INT2_W<2> {
        INT2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> INT3_W<3> {
        INT3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> INT4_W<4> {
        INT4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> INT5_W<5> {
        INT5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> INT6_W<6> {
        INT6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> INT7_W<7> {
        INT7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline(always)]
    #[must_use]
    pub fn int8(&mut self) -> INT8_W<8> {
        INT8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> INT9_W<9> {
        INT9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline(always)]
    #[must_use]
    pub fn int10(&mut self) -> INT10_W<10> {
        INT10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> INT11_W<11> {
        INT11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline(always)]
    #[must_use]
    pub fn int12(&mut self) -> INT12_W<12> {
        INT12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> INT13_W<13> {
        INT13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline(always)]
    #[must_use]
    pub fn int14(&mut self) -> INT14_W<14> {
        INT14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline(always)]
    #[must_use]
    pub fn int15(&mut self) -> INT15_W<15> {
        INT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](index.html) module"]
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int::R](R) reader structure"]
impl crate::Readable for INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int::W](W) writer structure"]
impl crate::Writable for INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

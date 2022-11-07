#[doc = "Register `ERR` reader"]
pub struct R(crate::R<ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERR` writer"]
pub struct W(crate::W<ERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERR_SPEC>;
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
impl From<crate::W<ERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERR0` reader - Error In Channel 0"]
pub type ERR0_R = crate::BitReader<ERR0_A>;
#[doc = "Error In Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR0_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR0_A> for bool {
    #[inline(always)]
    fn from(variant: ERR0_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR0_A {
        match self.bits {
            false => ERR0_A::_0,
            true => ERR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR0_A::_1
    }
}
#[doc = "Field `ERR0` writer - Error In Channel 0"]
pub type ERR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR0_A, O>;
impl<'a, const O: u8> ERR0_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR0_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR0_A::_1)
    }
}
#[doc = "Field `ERR1` reader - Error In Channel 1"]
pub type ERR1_R = crate::BitReader<ERR1_A>;
#[doc = "Error In Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR1_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR1_A> for bool {
    #[inline(always)]
    fn from(variant: ERR1_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR1_A {
        match self.bits {
            false => ERR1_A::_0,
            true => ERR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR1_A::_1
    }
}
#[doc = "Field `ERR1` writer - Error In Channel 1"]
pub type ERR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR1_A, O>;
impl<'a, const O: u8> ERR1_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR1_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR1_A::_1)
    }
}
#[doc = "Field `ERR2` reader - Error In Channel 2"]
pub type ERR2_R = crate::BitReader<ERR2_A>;
#[doc = "Error In Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR2_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR2_A> for bool {
    #[inline(always)]
    fn from(variant: ERR2_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR2_A {
        match self.bits {
            false => ERR2_A::_0,
            true => ERR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR2_A::_1
    }
}
#[doc = "Field `ERR2` writer - Error In Channel 2"]
pub type ERR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR2_A, O>;
impl<'a, const O: u8> ERR2_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR2_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR2_A::_1)
    }
}
#[doc = "Field `ERR3` reader - Error In Channel 3"]
pub type ERR3_R = crate::BitReader<ERR3_A>;
#[doc = "Error In Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR3_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR3_A> for bool {
    #[inline(always)]
    fn from(variant: ERR3_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR3_A {
        match self.bits {
            false => ERR3_A::_0,
            true => ERR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR3_A::_1
    }
}
#[doc = "Field `ERR3` writer - Error In Channel 3"]
pub type ERR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR3_A, O>;
impl<'a, const O: u8> ERR3_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR3_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR3_A::_1)
    }
}
#[doc = "Field `ERR4` reader - Error In Channel 4"]
pub type ERR4_R = crate::BitReader<ERR4_A>;
#[doc = "Error In Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR4_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR4_A> for bool {
    #[inline(always)]
    fn from(variant: ERR4_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR4_A {
        match self.bits {
            false => ERR4_A::_0,
            true => ERR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR4_A::_1
    }
}
#[doc = "Field `ERR4` writer - Error In Channel 4"]
pub type ERR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR4_A, O>;
impl<'a, const O: u8> ERR4_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR4_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR4_A::_1)
    }
}
#[doc = "Field `ERR5` reader - Error In Channel 5"]
pub type ERR5_R = crate::BitReader<ERR5_A>;
#[doc = "Error In Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR5_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR5_A> for bool {
    #[inline(always)]
    fn from(variant: ERR5_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR5_A {
        match self.bits {
            false => ERR5_A::_0,
            true => ERR5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR5_A::_1
    }
}
#[doc = "Field `ERR5` writer - Error In Channel 5"]
pub type ERR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR5_A, O>;
impl<'a, const O: u8> ERR5_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR5_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR5_A::_1)
    }
}
#[doc = "Field `ERR6` reader - Error In Channel 6"]
pub type ERR6_R = crate::BitReader<ERR6_A>;
#[doc = "Error In Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR6_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR6_A> for bool {
    #[inline(always)]
    fn from(variant: ERR6_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR6_A {
        match self.bits {
            false => ERR6_A::_0,
            true => ERR6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR6_A::_1
    }
}
#[doc = "Field `ERR6` writer - Error In Channel 6"]
pub type ERR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR6_A, O>;
impl<'a, const O: u8> ERR6_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR6_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR6_A::_1)
    }
}
#[doc = "Field `ERR7` reader - Error In Channel 7"]
pub type ERR7_R = crate::BitReader<ERR7_A>;
#[doc = "Error In Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR7_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR7_A> for bool {
    #[inline(always)]
    fn from(variant: ERR7_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR7_A {
        match self.bits {
            false => ERR7_A::_0,
            true => ERR7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR7_A::_1
    }
}
#[doc = "Field `ERR7` writer - Error In Channel 7"]
pub type ERR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR7_A, O>;
impl<'a, const O: u8> ERR7_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR7_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR7_A::_1)
    }
}
#[doc = "Field `ERR8` reader - Error In Channel 8"]
pub type ERR8_R = crate::BitReader<ERR8_A>;
#[doc = "Error In Channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR8_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR8_A> for bool {
    #[inline(always)]
    fn from(variant: ERR8_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR8_A {
        match self.bits {
            false => ERR8_A::_0,
            true => ERR8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR8_A::_1
    }
}
#[doc = "Field `ERR8` writer - Error In Channel 8"]
pub type ERR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR8_A, O>;
impl<'a, const O: u8> ERR8_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR8_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR8_A::_1)
    }
}
#[doc = "Field `ERR9` reader - Error In Channel 9"]
pub type ERR9_R = crate::BitReader<ERR9_A>;
#[doc = "Error In Channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR9_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR9_A> for bool {
    #[inline(always)]
    fn from(variant: ERR9_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR9_A {
        match self.bits {
            false => ERR9_A::_0,
            true => ERR9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR9_A::_1
    }
}
#[doc = "Field `ERR9` writer - Error In Channel 9"]
pub type ERR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR9_A, O>;
impl<'a, const O: u8> ERR9_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR9_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR9_A::_1)
    }
}
#[doc = "Field `ERR10` reader - Error In Channel 10"]
pub type ERR10_R = crate::BitReader<ERR10_A>;
#[doc = "Error In Channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR10_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR10_A> for bool {
    #[inline(always)]
    fn from(variant: ERR10_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR10_A {
        match self.bits {
            false => ERR10_A::_0,
            true => ERR10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR10_A::_1
    }
}
#[doc = "Field `ERR10` writer - Error In Channel 10"]
pub type ERR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR10_A, O>;
impl<'a, const O: u8> ERR10_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR10_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR10_A::_1)
    }
}
#[doc = "Field `ERR11` reader - Error In Channel 11"]
pub type ERR11_R = crate::BitReader<ERR11_A>;
#[doc = "Error In Channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR11_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR11_A> for bool {
    #[inline(always)]
    fn from(variant: ERR11_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR11_A {
        match self.bits {
            false => ERR11_A::_0,
            true => ERR11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR11_A::_1
    }
}
#[doc = "Field `ERR11` writer - Error In Channel 11"]
pub type ERR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR11_A, O>;
impl<'a, const O: u8> ERR11_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR11_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR11_A::_1)
    }
}
#[doc = "Field `ERR12` reader - Error In Channel 12"]
pub type ERR12_R = crate::BitReader<ERR12_A>;
#[doc = "Error In Channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR12_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR12_A> for bool {
    #[inline(always)]
    fn from(variant: ERR12_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR12_A {
        match self.bits {
            false => ERR12_A::_0,
            true => ERR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR12_A::_1
    }
}
#[doc = "Field `ERR12` writer - Error In Channel 12"]
pub type ERR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR12_A, O>;
impl<'a, const O: u8> ERR12_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR12_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR12_A::_1)
    }
}
#[doc = "Field `ERR13` reader - Error In Channel 13"]
pub type ERR13_R = crate::BitReader<ERR13_A>;
#[doc = "Error In Channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR13_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR13_A> for bool {
    #[inline(always)]
    fn from(variant: ERR13_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR13_A {
        match self.bits {
            false => ERR13_A::_0,
            true => ERR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR13_A::_1
    }
}
#[doc = "Field `ERR13` writer - Error In Channel 13"]
pub type ERR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR13_A, O>;
impl<'a, const O: u8> ERR13_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR13_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR13_A::_1)
    }
}
#[doc = "Field `ERR14` reader - Error In Channel 14"]
pub type ERR14_R = crate::BitReader<ERR14_A>;
#[doc = "Error In Channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR14_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR14_A> for bool {
    #[inline(always)]
    fn from(variant: ERR14_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR14_A {
        match self.bits {
            false => ERR14_A::_0,
            true => ERR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR14_A::_1
    }
}
#[doc = "Field `ERR14` writer - Error In Channel 14"]
pub type ERR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR14_A, O>;
impl<'a, const O: u8> ERR14_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR14_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR14_A::_1)
    }
}
#[doc = "Field `ERR15` reader - Error In Channel 15"]
pub type ERR15_R = crate::BitReader<ERR15_A>;
#[doc = "Error In Channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR15_A {
    #[doc = "0: An error in the corresponding channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in the corresponding channel has occurred"]
    _1 = 1,
}
impl From<ERR15_A> for bool {
    #[inline(always)]
    fn from(variant: ERR15_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR15_A {
        match self.bits {
            false => ERR15_A::_0,
            true => ERR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR15_A::_1
    }
}
#[doc = "Field `ERR15` writer - Error In Channel 15"]
pub type ERR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_SPEC, ERR15_A, O>;
impl<'a, const O: u8> ERR15_W<'a, O> {
    #[doc = "An error in the corresponding channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR15_A::_0)
    }
    #[doc = "An error in the corresponding channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Error In Channel 0"]
    #[inline(always)]
    pub fn err0(&self) -> ERR0_R {
        ERR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error In Channel 1"]
    #[inline(always)]
    pub fn err1(&self) -> ERR1_R {
        ERR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error In Channel 2"]
    #[inline(always)]
    pub fn err2(&self) -> ERR2_R {
        ERR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error In Channel 3"]
    #[inline(always)]
    pub fn err3(&self) -> ERR3_R {
        ERR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Error In Channel 4"]
    #[inline(always)]
    pub fn err4(&self) -> ERR4_R {
        ERR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error In Channel 5"]
    #[inline(always)]
    pub fn err5(&self) -> ERR5_R {
        ERR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error In Channel 6"]
    #[inline(always)]
    pub fn err6(&self) -> ERR6_R {
        ERR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error In Channel 7"]
    #[inline(always)]
    pub fn err7(&self) -> ERR7_R {
        ERR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error In Channel 8"]
    #[inline(always)]
    pub fn err8(&self) -> ERR8_R {
        ERR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error In Channel 9"]
    #[inline(always)]
    pub fn err9(&self) -> ERR9_R {
        ERR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error In Channel 10"]
    #[inline(always)]
    pub fn err10(&self) -> ERR10_R {
        ERR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error In Channel 11"]
    #[inline(always)]
    pub fn err11(&self) -> ERR11_R {
        ERR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Error In Channel 12"]
    #[inline(always)]
    pub fn err12(&self) -> ERR12_R {
        ERR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error In Channel 13"]
    #[inline(always)]
    pub fn err13(&self) -> ERR13_R {
        ERR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error In Channel 14"]
    #[inline(always)]
    pub fn err14(&self) -> ERR14_R {
        ERR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error In Channel 15"]
    #[inline(always)]
    pub fn err15(&self) -> ERR15_R {
        ERR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error In Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn err0(&mut self) -> ERR0_W<0> {
        ERR0_W::new(self)
    }
    #[doc = "Bit 1 - Error In Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn err1(&mut self) -> ERR1_W<1> {
        ERR1_W::new(self)
    }
    #[doc = "Bit 2 - Error In Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn err2(&mut self) -> ERR2_W<2> {
        ERR2_W::new(self)
    }
    #[doc = "Bit 3 - Error In Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn err3(&mut self) -> ERR3_W<3> {
        ERR3_W::new(self)
    }
    #[doc = "Bit 4 - Error In Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn err4(&mut self) -> ERR4_W<4> {
        ERR4_W::new(self)
    }
    #[doc = "Bit 5 - Error In Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn err5(&mut self) -> ERR5_W<5> {
        ERR5_W::new(self)
    }
    #[doc = "Bit 6 - Error In Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn err6(&mut self) -> ERR6_W<6> {
        ERR6_W::new(self)
    }
    #[doc = "Bit 7 - Error In Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn err7(&mut self) -> ERR7_W<7> {
        ERR7_W::new(self)
    }
    #[doc = "Bit 8 - Error In Channel 8"]
    #[inline(always)]
    #[must_use]
    pub fn err8(&mut self) -> ERR8_W<8> {
        ERR8_W::new(self)
    }
    #[doc = "Bit 9 - Error In Channel 9"]
    #[inline(always)]
    #[must_use]
    pub fn err9(&mut self) -> ERR9_W<9> {
        ERR9_W::new(self)
    }
    #[doc = "Bit 10 - Error In Channel 10"]
    #[inline(always)]
    #[must_use]
    pub fn err10(&mut self) -> ERR10_W<10> {
        ERR10_W::new(self)
    }
    #[doc = "Bit 11 - Error In Channel 11"]
    #[inline(always)]
    #[must_use]
    pub fn err11(&mut self) -> ERR11_W<11> {
        ERR11_W::new(self)
    }
    #[doc = "Bit 12 - Error In Channel 12"]
    #[inline(always)]
    #[must_use]
    pub fn err12(&mut self) -> ERR12_W<12> {
        ERR12_W::new(self)
    }
    #[doc = "Bit 13 - Error In Channel 13"]
    #[inline(always)]
    #[must_use]
    pub fn err13(&mut self) -> ERR13_W<13> {
        ERR13_W::new(self)
    }
    #[doc = "Bit 14 - Error In Channel 14"]
    #[inline(always)]
    #[must_use]
    pub fn err14(&mut self) -> ERR14_W<14> {
        ERR14_W::new(self)
    }
    #[doc = "Bit 15 - Error In Channel 15"]
    #[inline(always)]
    #[must_use]
    pub fn err15(&mut self) -> ERR15_W<15> {
        ERR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err](index.html) module"]
pub struct ERR_SPEC;
impl crate::RegisterSpec for ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err::R](R) reader structure"]
impl crate::Readable for ERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [err::W](W) writer structure"]
impl crate::Writable for ERR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

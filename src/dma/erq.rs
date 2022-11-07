#[doc = "Register `ERQ` reader"]
pub struct R(crate::R<ERQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERQ` writer"]
pub struct W(crate::W<ERQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERQ_SPEC>;
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
impl From<crate::W<ERQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERQ0` reader - Enable DMA Request 0"]
pub type ERQ0_R = crate::BitReader<ERQ0_A>;
#[doc = "Enable DMA Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ0_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ0_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ0_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ0_A {
        match self.bits {
            false => ERQ0_A::_0,
            true => ERQ0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ0_A::_1
    }
}
#[doc = "Field `ERQ0` writer - Enable DMA Request 0"]
pub type ERQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ0_A, O>;
impl<'a, const O: u8> ERQ0_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ0_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ0_A::_1)
    }
}
#[doc = "Field `ERQ1` reader - Enable DMA Request 1"]
pub type ERQ1_R = crate::BitReader<ERQ1_A>;
#[doc = "Enable DMA Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ1_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ1_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ1_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ1_A {
        match self.bits {
            false => ERQ1_A::_0,
            true => ERQ1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ1_A::_1
    }
}
#[doc = "Field `ERQ1` writer - Enable DMA Request 1"]
pub type ERQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ1_A, O>;
impl<'a, const O: u8> ERQ1_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ1_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ1_A::_1)
    }
}
#[doc = "Field `ERQ2` reader - Enable DMA Request 2"]
pub type ERQ2_R = crate::BitReader<ERQ2_A>;
#[doc = "Enable DMA Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ2_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ2_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ2_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ2_A {
        match self.bits {
            false => ERQ2_A::_0,
            true => ERQ2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ2_A::_1
    }
}
#[doc = "Field `ERQ2` writer - Enable DMA Request 2"]
pub type ERQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ2_A, O>;
impl<'a, const O: u8> ERQ2_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ2_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ2_A::_1)
    }
}
#[doc = "Field `ERQ3` reader - Enable DMA Request 3"]
pub type ERQ3_R = crate::BitReader<ERQ3_A>;
#[doc = "Enable DMA Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ3_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ3_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ3_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ3_A {
        match self.bits {
            false => ERQ3_A::_0,
            true => ERQ3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ3_A::_1
    }
}
#[doc = "Field `ERQ3` writer - Enable DMA Request 3"]
pub type ERQ3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ3_A, O>;
impl<'a, const O: u8> ERQ3_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ3_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ3_A::_1)
    }
}
#[doc = "Field `ERQ4` reader - Enable DMA Request 4"]
pub type ERQ4_R = crate::BitReader<ERQ4_A>;
#[doc = "Enable DMA Request 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ4_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ4_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ4_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ4_A {
        match self.bits {
            false => ERQ4_A::_0,
            true => ERQ4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ4_A::_1
    }
}
#[doc = "Field `ERQ4` writer - Enable DMA Request 4"]
pub type ERQ4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ4_A, O>;
impl<'a, const O: u8> ERQ4_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ4_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ4_A::_1)
    }
}
#[doc = "Field `ERQ5` reader - Enable DMA Request 5"]
pub type ERQ5_R = crate::BitReader<ERQ5_A>;
#[doc = "Enable DMA Request 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ5_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ5_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ5_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ5_A {
        match self.bits {
            false => ERQ5_A::_0,
            true => ERQ5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ5_A::_1
    }
}
#[doc = "Field `ERQ5` writer - Enable DMA Request 5"]
pub type ERQ5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ5_A, O>;
impl<'a, const O: u8> ERQ5_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ5_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ5_A::_1)
    }
}
#[doc = "Field `ERQ6` reader - Enable DMA Request 6"]
pub type ERQ6_R = crate::BitReader<ERQ6_A>;
#[doc = "Enable DMA Request 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ6_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ6_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ6_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ6_A {
        match self.bits {
            false => ERQ6_A::_0,
            true => ERQ6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ6_A::_1
    }
}
#[doc = "Field `ERQ6` writer - Enable DMA Request 6"]
pub type ERQ6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ6_A, O>;
impl<'a, const O: u8> ERQ6_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ6_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ6_A::_1)
    }
}
#[doc = "Field `ERQ7` reader - Enable DMA Request 7"]
pub type ERQ7_R = crate::BitReader<ERQ7_A>;
#[doc = "Enable DMA Request 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ7_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ7_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ7_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ7_A {
        match self.bits {
            false => ERQ7_A::_0,
            true => ERQ7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ7_A::_1
    }
}
#[doc = "Field `ERQ7` writer - Enable DMA Request 7"]
pub type ERQ7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ7_A, O>;
impl<'a, const O: u8> ERQ7_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ7_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ7_A::_1)
    }
}
#[doc = "Field `ERQ8` reader - Enable DMA Request 8"]
pub type ERQ8_R = crate::BitReader<ERQ8_A>;
#[doc = "Enable DMA Request 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ8_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ8_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ8_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ8_A {
        match self.bits {
            false => ERQ8_A::_0,
            true => ERQ8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ8_A::_1
    }
}
#[doc = "Field `ERQ8` writer - Enable DMA Request 8"]
pub type ERQ8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ8_A, O>;
impl<'a, const O: u8> ERQ8_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ8_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ8_A::_1)
    }
}
#[doc = "Field `ERQ9` reader - Enable DMA Request 9"]
pub type ERQ9_R = crate::BitReader<ERQ9_A>;
#[doc = "Enable DMA Request 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ9_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ9_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ9_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ9_A {
        match self.bits {
            false => ERQ9_A::_0,
            true => ERQ9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ9_A::_1
    }
}
#[doc = "Field `ERQ9` writer - Enable DMA Request 9"]
pub type ERQ9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ9_A, O>;
impl<'a, const O: u8> ERQ9_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ9_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ9_A::_1)
    }
}
#[doc = "Field `ERQ10` reader - Enable DMA Request 10"]
pub type ERQ10_R = crate::BitReader<ERQ10_A>;
#[doc = "Enable DMA Request 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ10_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ10_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ10_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ10_A {
        match self.bits {
            false => ERQ10_A::_0,
            true => ERQ10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ10_A::_1
    }
}
#[doc = "Field `ERQ10` writer - Enable DMA Request 10"]
pub type ERQ10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ10_A, O>;
impl<'a, const O: u8> ERQ10_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ10_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ10_A::_1)
    }
}
#[doc = "Field `ERQ11` reader - Enable DMA Request 11"]
pub type ERQ11_R = crate::BitReader<ERQ11_A>;
#[doc = "Enable DMA Request 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ11_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ11_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ11_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ11_A {
        match self.bits {
            false => ERQ11_A::_0,
            true => ERQ11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ11_A::_1
    }
}
#[doc = "Field `ERQ11` writer - Enable DMA Request 11"]
pub type ERQ11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ11_A, O>;
impl<'a, const O: u8> ERQ11_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ11_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ11_A::_1)
    }
}
#[doc = "Field `ERQ12` reader - Enable DMA Request 12"]
pub type ERQ12_R = crate::BitReader<ERQ12_A>;
#[doc = "Enable DMA Request 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ12_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ12_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ12_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ12_A {
        match self.bits {
            false => ERQ12_A::_0,
            true => ERQ12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ12_A::_1
    }
}
#[doc = "Field `ERQ12` writer - Enable DMA Request 12"]
pub type ERQ12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ12_A, O>;
impl<'a, const O: u8> ERQ12_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ12_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ12_A::_1)
    }
}
#[doc = "Field `ERQ13` reader - Enable DMA Request 13"]
pub type ERQ13_R = crate::BitReader<ERQ13_A>;
#[doc = "Enable DMA Request 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ13_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ13_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ13_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ13_A {
        match self.bits {
            false => ERQ13_A::_0,
            true => ERQ13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ13_A::_1
    }
}
#[doc = "Field `ERQ13` writer - Enable DMA Request 13"]
pub type ERQ13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ13_A, O>;
impl<'a, const O: u8> ERQ13_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ13_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ13_A::_1)
    }
}
#[doc = "Field `ERQ14` reader - Enable DMA Request 14"]
pub type ERQ14_R = crate::BitReader<ERQ14_A>;
#[doc = "Enable DMA Request 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ14_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ14_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ14_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ14_A {
        match self.bits {
            false => ERQ14_A::_0,
            true => ERQ14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ14_A::_1
    }
}
#[doc = "Field `ERQ14` writer - Enable DMA Request 14"]
pub type ERQ14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ14_A, O>;
impl<'a, const O: u8> ERQ14_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ14_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ14_A::_1)
    }
}
#[doc = "Field `ERQ15` reader - Enable DMA Request 15"]
pub type ERQ15_R = crate::BitReader<ERQ15_A>;
#[doc = "Enable DMA Request 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ15_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ15_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ15_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ15_A {
        match self.bits {
            false => ERQ15_A::_0,
            true => ERQ15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ15_A::_1
    }
}
#[doc = "Field `ERQ15` writer - Enable DMA Request 15"]
pub type ERQ15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ15_A, O>;
impl<'a, const O: u8> ERQ15_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ15_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    pub fn erq0(&self) -> ERQ0_R {
        ERQ0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    pub fn erq1(&self) -> ERQ1_R {
        ERQ1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    pub fn erq2(&self) -> ERQ2_R {
        ERQ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    pub fn erq3(&self) -> ERQ3_R {
        ERQ3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline(always)]
    pub fn erq4(&self) -> ERQ4_R {
        ERQ4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline(always)]
    pub fn erq5(&self) -> ERQ5_R {
        ERQ5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline(always)]
    pub fn erq6(&self) -> ERQ6_R {
        ERQ6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline(always)]
    pub fn erq7(&self) -> ERQ7_R {
        ERQ7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline(always)]
    pub fn erq8(&self) -> ERQ8_R {
        ERQ8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline(always)]
    pub fn erq9(&self) -> ERQ9_R {
        ERQ9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline(always)]
    pub fn erq10(&self) -> ERQ10_R {
        ERQ10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline(always)]
    pub fn erq11(&self) -> ERQ11_R {
        ERQ11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline(always)]
    pub fn erq12(&self) -> ERQ12_R {
        ERQ12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline(always)]
    pub fn erq13(&self) -> ERQ13_R {
        ERQ13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline(always)]
    pub fn erq14(&self) -> ERQ14_R {
        ERQ14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline(always)]
    pub fn erq15(&self) -> ERQ15_R {
        ERQ15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    #[must_use]
    pub fn erq0(&mut self) -> ERQ0_W<0> {
        ERQ0_W::new(self)
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    #[must_use]
    pub fn erq1(&mut self) -> ERQ1_W<1> {
        ERQ1_W::new(self)
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    #[must_use]
    pub fn erq2(&mut self) -> ERQ2_W<2> {
        ERQ2_W::new(self)
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    #[must_use]
    pub fn erq3(&mut self) -> ERQ3_W<3> {
        ERQ3_W::new(self)
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline(always)]
    #[must_use]
    pub fn erq4(&mut self) -> ERQ4_W<4> {
        ERQ4_W::new(self)
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline(always)]
    #[must_use]
    pub fn erq5(&mut self) -> ERQ5_W<5> {
        ERQ5_W::new(self)
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline(always)]
    #[must_use]
    pub fn erq6(&mut self) -> ERQ6_W<6> {
        ERQ6_W::new(self)
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline(always)]
    #[must_use]
    pub fn erq7(&mut self) -> ERQ7_W<7> {
        ERQ7_W::new(self)
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline(always)]
    #[must_use]
    pub fn erq8(&mut self) -> ERQ8_W<8> {
        ERQ8_W::new(self)
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline(always)]
    #[must_use]
    pub fn erq9(&mut self) -> ERQ9_W<9> {
        ERQ9_W::new(self)
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline(always)]
    #[must_use]
    pub fn erq10(&mut self) -> ERQ10_W<10> {
        ERQ10_W::new(self)
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline(always)]
    #[must_use]
    pub fn erq11(&mut self) -> ERQ11_W<11> {
        ERQ11_W::new(self)
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline(always)]
    #[must_use]
    pub fn erq12(&mut self) -> ERQ12_W<12> {
        ERQ12_W::new(self)
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline(always)]
    #[must_use]
    pub fn erq13(&mut self) -> ERQ13_W<13> {
        ERQ13_W::new(self)
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline(always)]
    #[must_use]
    pub fn erq14(&mut self) -> ERQ14_W<14> {
        ERQ14_W::new(self)
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline(always)]
    #[must_use]
    pub fn erq15(&mut self) -> ERQ15_W<15> {
        ERQ15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erq](index.html) module"]
pub struct ERQ_SPEC;
impl crate::RegisterSpec for ERQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erq::R](R) reader structure"]
impl crate::Readable for ERQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erq::W](W) writer structure"]
impl crate::Writable for ERQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERQ to value 0"]
impl crate::Resettable for ERQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

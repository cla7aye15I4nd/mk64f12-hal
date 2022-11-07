#[doc = "Register `EEI` reader"]
pub struct R(crate::R<EEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEI` writer"]
pub struct W(crate::W<EEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEI_SPEC>;
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
impl From<crate::W<EEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEI0` reader - Enable Error Interrupt 0"]
pub type EEI0_R = crate::BitReader<EEI0_A>;
#[doc = "Enable Error Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI0_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI0_A> for bool {
    #[inline(always)]
    fn from(variant: EEI0_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI0_A {
        match self.bits {
            false => EEI0_A::_0,
            true => EEI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI0_A::_1
    }
}
#[doc = "Field `EEI0` writer - Enable Error Interrupt 0"]
pub type EEI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI0_A, O>;
impl<'a, const O: u8> EEI0_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI0_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI0_A::_1)
    }
}
#[doc = "Field `EEI1` reader - Enable Error Interrupt 1"]
pub type EEI1_R = crate::BitReader<EEI1_A>;
#[doc = "Enable Error Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI1_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI1_A> for bool {
    #[inline(always)]
    fn from(variant: EEI1_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI1_A {
        match self.bits {
            false => EEI1_A::_0,
            true => EEI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI1_A::_1
    }
}
#[doc = "Field `EEI1` writer - Enable Error Interrupt 1"]
pub type EEI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI1_A, O>;
impl<'a, const O: u8> EEI1_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI1_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI1_A::_1)
    }
}
#[doc = "Field `EEI2` reader - Enable Error Interrupt 2"]
pub type EEI2_R = crate::BitReader<EEI2_A>;
#[doc = "Enable Error Interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI2_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI2_A> for bool {
    #[inline(always)]
    fn from(variant: EEI2_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI2_A {
        match self.bits {
            false => EEI2_A::_0,
            true => EEI2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI2_A::_1
    }
}
#[doc = "Field `EEI2` writer - Enable Error Interrupt 2"]
pub type EEI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI2_A, O>;
impl<'a, const O: u8> EEI2_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI2_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI2_A::_1)
    }
}
#[doc = "Field `EEI3` reader - Enable Error Interrupt 3"]
pub type EEI3_R = crate::BitReader<EEI3_A>;
#[doc = "Enable Error Interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI3_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI3_A> for bool {
    #[inline(always)]
    fn from(variant: EEI3_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI3_A {
        match self.bits {
            false => EEI3_A::_0,
            true => EEI3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI3_A::_1
    }
}
#[doc = "Field `EEI3` writer - Enable Error Interrupt 3"]
pub type EEI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI3_A, O>;
impl<'a, const O: u8> EEI3_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI3_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI3_A::_1)
    }
}
#[doc = "Field `EEI4` reader - Enable Error Interrupt 4"]
pub type EEI4_R = crate::BitReader<EEI4_A>;
#[doc = "Enable Error Interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI4_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI4_A> for bool {
    #[inline(always)]
    fn from(variant: EEI4_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI4_A {
        match self.bits {
            false => EEI4_A::_0,
            true => EEI4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI4_A::_1
    }
}
#[doc = "Field `EEI4` writer - Enable Error Interrupt 4"]
pub type EEI4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI4_A, O>;
impl<'a, const O: u8> EEI4_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI4_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI4_A::_1)
    }
}
#[doc = "Field `EEI5` reader - Enable Error Interrupt 5"]
pub type EEI5_R = crate::BitReader<EEI5_A>;
#[doc = "Enable Error Interrupt 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI5_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI5_A> for bool {
    #[inline(always)]
    fn from(variant: EEI5_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI5_A {
        match self.bits {
            false => EEI5_A::_0,
            true => EEI5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI5_A::_1
    }
}
#[doc = "Field `EEI5` writer - Enable Error Interrupt 5"]
pub type EEI5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI5_A, O>;
impl<'a, const O: u8> EEI5_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI5_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI5_A::_1)
    }
}
#[doc = "Field `EEI6` reader - Enable Error Interrupt 6"]
pub type EEI6_R = crate::BitReader<EEI6_A>;
#[doc = "Enable Error Interrupt 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI6_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI6_A> for bool {
    #[inline(always)]
    fn from(variant: EEI6_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI6_A {
        match self.bits {
            false => EEI6_A::_0,
            true => EEI6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI6_A::_1
    }
}
#[doc = "Field `EEI6` writer - Enable Error Interrupt 6"]
pub type EEI6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI6_A, O>;
impl<'a, const O: u8> EEI6_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI6_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI6_A::_1)
    }
}
#[doc = "Field `EEI7` reader - Enable Error Interrupt 7"]
pub type EEI7_R = crate::BitReader<EEI7_A>;
#[doc = "Enable Error Interrupt 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI7_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI7_A> for bool {
    #[inline(always)]
    fn from(variant: EEI7_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI7_A {
        match self.bits {
            false => EEI7_A::_0,
            true => EEI7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI7_A::_1
    }
}
#[doc = "Field `EEI7` writer - Enable Error Interrupt 7"]
pub type EEI7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI7_A, O>;
impl<'a, const O: u8> EEI7_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI7_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI7_A::_1)
    }
}
#[doc = "Field `EEI8` reader - Enable Error Interrupt 8"]
pub type EEI8_R = crate::BitReader<EEI8_A>;
#[doc = "Enable Error Interrupt 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI8_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI8_A> for bool {
    #[inline(always)]
    fn from(variant: EEI8_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI8_A {
        match self.bits {
            false => EEI8_A::_0,
            true => EEI8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI8_A::_1
    }
}
#[doc = "Field `EEI8` writer - Enable Error Interrupt 8"]
pub type EEI8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI8_A, O>;
impl<'a, const O: u8> EEI8_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI8_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI8_A::_1)
    }
}
#[doc = "Field `EEI9` reader - Enable Error Interrupt 9"]
pub type EEI9_R = crate::BitReader<EEI9_A>;
#[doc = "Enable Error Interrupt 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI9_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI9_A> for bool {
    #[inline(always)]
    fn from(variant: EEI9_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI9_A {
        match self.bits {
            false => EEI9_A::_0,
            true => EEI9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI9_A::_1
    }
}
#[doc = "Field `EEI9` writer - Enable Error Interrupt 9"]
pub type EEI9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI9_A, O>;
impl<'a, const O: u8> EEI9_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI9_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI9_A::_1)
    }
}
#[doc = "Field `EEI10` reader - Enable Error Interrupt 10"]
pub type EEI10_R = crate::BitReader<EEI10_A>;
#[doc = "Enable Error Interrupt 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI10_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI10_A> for bool {
    #[inline(always)]
    fn from(variant: EEI10_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI10_A {
        match self.bits {
            false => EEI10_A::_0,
            true => EEI10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI10_A::_1
    }
}
#[doc = "Field `EEI10` writer - Enable Error Interrupt 10"]
pub type EEI10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI10_A, O>;
impl<'a, const O: u8> EEI10_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI10_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI10_A::_1)
    }
}
#[doc = "Field `EEI11` reader - Enable Error Interrupt 11"]
pub type EEI11_R = crate::BitReader<EEI11_A>;
#[doc = "Enable Error Interrupt 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI11_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI11_A> for bool {
    #[inline(always)]
    fn from(variant: EEI11_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI11_A {
        match self.bits {
            false => EEI11_A::_0,
            true => EEI11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI11_A::_1
    }
}
#[doc = "Field `EEI11` writer - Enable Error Interrupt 11"]
pub type EEI11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI11_A, O>;
impl<'a, const O: u8> EEI11_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI11_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI11_A::_1)
    }
}
#[doc = "Field `EEI12` reader - Enable Error Interrupt 12"]
pub type EEI12_R = crate::BitReader<EEI12_A>;
#[doc = "Enable Error Interrupt 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI12_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI12_A> for bool {
    #[inline(always)]
    fn from(variant: EEI12_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI12_A {
        match self.bits {
            false => EEI12_A::_0,
            true => EEI12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI12_A::_1
    }
}
#[doc = "Field `EEI12` writer - Enable Error Interrupt 12"]
pub type EEI12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI12_A, O>;
impl<'a, const O: u8> EEI12_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI12_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI12_A::_1)
    }
}
#[doc = "Field `EEI13` reader - Enable Error Interrupt 13"]
pub type EEI13_R = crate::BitReader<EEI13_A>;
#[doc = "Enable Error Interrupt 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI13_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI13_A> for bool {
    #[inline(always)]
    fn from(variant: EEI13_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI13_A {
        match self.bits {
            false => EEI13_A::_0,
            true => EEI13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI13_A::_1
    }
}
#[doc = "Field `EEI13` writer - Enable Error Interrupt 13"]
pub type EEI13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI13_A, O>;
impl<'a, const O: u8> EEI13_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI13_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI13_A::_1)
    }
}
#[doc = "Field `EEI14` reader - Enable Error Interrupt 14"]
pub type EEI14_R = crate::BitReader<EEI14_A>;
#[doc = "Enable Error Interrupt 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI14_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI14_A> for bool {
    #[inline(always)]
    fn from(variant: EEI14_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI14_A {
        match self.bits {
            false => EEI14_A::_0,
            true => EEI14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI14_A::_1
    }
}
#[doc = "Field `EEI14` writer - Enable Error Interrupt 14"]
pub type EEI14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI14_A, O>;
impl<'a, const O: u8> EEI14_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI14_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI14_A::_1)
    }
}
#[doc = "Field `EEI15` reader - Enable Error Interrupt 15"]
pub type EEI15_R = crate::BitReader<EEI15_A>;
#[doc = "Enable Error Interrupt 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI15_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI15_A> for bool {
    #[inline(always)]
    fn from(variant: EEI15_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI15_A {
        match self.bits {
            false => EEI15_A::_0,
            true => EEI15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI15_A::_1
    }
}
#[doc = "Field `EEI15` writer - Enable Error Interrupt 15"]
pub type EEI15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI15_A, O>;
impl<'a, const O: u8> EEI15_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI15_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    pub fn eei0(&self) -> EEI0_R {
        EEI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    pub fn eei1(&self) -> EEI1_R {
        EEI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    pub fn eei2(&self) -> EEI2_R {
        EEI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    pub fn eei3(&self) -> EEI3_R {
        EEI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline(always)]
    pub fn eei4(&self) -> EEI4_R {
        EEI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline(always)]
    pub fn eei5(&self) -> EEI5_R {
        EEI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline(always)]
    pub fn eei6(&self) -> EEI6_R {
        EEI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline(always)]
    pub fn eei7(&self) -> EEI7_R {
        EEI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Error Interrupt 8"]
    #[inline(always)]
    pub fn eei8(&self) -> EEI8_R {
        EEI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Error Interrupt 9"]
    #[inline(always)]
    pub fn eei9(&self) -> EEI9_R {
        EEI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Error Interrupt 10"]
    #[inline(always)]
    pub fn eei10(&self) -> EEI10_R {
        EEI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Error Interrupt 11"]
    #[inline(always)]
    pub fn eei11(&self) -> EEI11_R {
        EEI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Error Interrupt 12"]
    #[inline(always)]
    pub fn eei12(&self) -> EEI12_R {
        EEI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Error Interrupt 13"]
    #[inline(always)]
    pub fn eei13(&self) -> EEI13_R {
        EEI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Error Interrupt 14"]
    #[inline(always)]
    pub fn eei14(&self) -> EEI14_R {
        EEI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Error Interrupt 15"]
    #[inline(always)]
    pub fn eei15(&self) -> EEI15_R {
        EEI15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn eei0(&mut self) -> EEI0_W<0> {
        EEI0_W::new(self)
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn eei1(&mut self) -> EEI1_W<1> {
        EEI1_W::new(self)
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn eei2(&mut self) -> EEI2_W<2> {
        EEI2_W::new(self)
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn eei3(&mut self) -> EEI3_W<3> {
        EEI3_W::new(self)
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn eei4(&mut self) -> EEI4_W<4> {
        EEI4_W::new(self)
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn eei5(&mut self) -> EEI5_W<5> {
        EEI5_W::new(self)
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn eei6(&mut self) -> EEI6_W<6> {
        EEI6_W::new(self)
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn eei7(&mut self) -> EEI7_W<7> {
        EEI7_W::new(self)
    }
    #[doc = "Bit 8 - Enable Error Interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn eei8(&mut self) -> EEI8_W<8> {
        EEI8_W::new(self)
    }
    #[doc = "Bit 9 - Enable Error Interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn eei9(&mut self) -> EEI9_W<9> {
        EEI9_W::new(self)
    }
    #[doc = "Bit 10 - Enable Error Interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn eei10(&mut self) -> EEI10_W<10> {
        EEI10_W::new(self)
    }
    #[doc = "Bit 11 - Enable Error Interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn eei11(&mut self) -> EEI11_W<11> {
        EEI11_W::new(self)
    }
    #[doc = "Bit 12 - Enable Error Interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn eei12(&mut self) -> EEI12_W<12> {
        EEI12_W::new(self)
    }
    #[doc = "Bit 13 - Enable Error Interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn eei13(&mut self) -> EEI13_W<13> {
        EEI13_W::new(self)
    }
    #[doc = "Bit 14 - Enable Error Interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn eei14(&mut self) -> EEI14_W<14> {
        EEI14_W::new(self)
    }
    #[doc = "Bit 15 - Enable Error Interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn eei15(&mut self) -> EEI15_W<15> {
        EEI15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eei](index.html) module"]
pub struct EEI_SPEC;
impl crate::RegisterSpec for EEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eei::R](R) reader structure"]
impl crate::Readable for EEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eei::W](W) writer structure"]
impl crate::Writable for EEI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEI to value 0"]
impl crate::Resettable for EEI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

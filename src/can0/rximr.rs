#[doc = "Register `RXIMR%s` reader"]
pub struct R(crate::R<RXIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXIMR%s` writer"]
pub struct W(crate::W<RXIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXIMR_SPEC>;
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
impl From<crate::W<RXIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MI0` reader - Individual Mask Bits"]
pub type MI0_R = crate::BitReader<MI0_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI0_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI0_A> for bool {
    #[inline(always)]
    fn from(variant: MI0_A) -> Self {
        variant as u8 != 0
    }
}
impl MI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI0_A {
        match self.bits {
            false => MI0_A::_0,
            true => MI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI0_A::_1
    }
}
#[doc = "Field `MI0` writer - Individual Mask Bits"]
pub type MI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI0_A, O>;
impl<'a, const O: u8> MI0_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI0_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI0_A::_1)
    }
}
#[doc = "Field `MI1` reader - Individual Mask Bits"]
pub type MI1_R = crate::BitReader<MI1_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI1_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI1_A> for bool {
    #[inline(always)]
    fn from(variant: MI1_A) -> Self {
        variant as u8 != 0
    }
}
impl MI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI1_A {
        match self.bits {
            false => MI1_A::_0,
            true => MI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI1_A::_1
    }
}
#[doc = "Field `MI1` writer - Individual Mask Bits"]
pub type MI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI1_A, O>;
impl<'a, const O: u8> MI1_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI1_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI1_A::_1)
    }
}
#[doc = "Field `MI2` reader - Individual Mask Bits"]
pub type MI2_R = crate::BitReader<MI2_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI2_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI2_A> for bool {
    #[inline(always)]
    fn from(variant: MI2_A) -> Self {
        variant as u8 != 0
    }
}
impl MI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI2_A {
        match self.bits {
            false => MI2_A::_0,
            true => MI2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI2_A::_1
    }
}
#[doc = "Field `MI2` writer - Individual Mask Bits"]
pub type MI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI2_A, O>;
impl<'a, const O: u8> MI2_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI2_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI2_A::_1)
    }
}
#[doc = "Field `MI3` reader - Individual Mask Bits"]
pub type MI3_R = crate::BitReader<MI3_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI3_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI3_A> for bool {
    #[inline(always)]
    fn from(variant: MI3_A) -> Self {
        variant as u8 != 0
    }
}
impl MI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI3_A {
        match self.bits {
            false => MI3_A::_0,
            true => MI3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI3_A::_1
    }
}
#[doc = "Field `MI3` writer - Individual Mask Bits"]
pub type MI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI3_A, O>;
impl<'a, const O: u8> MI3_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI3_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI3_A::_1)
    }
}
#[doc = "Field `MI4` reader - Individual Mask Bits"]
pub type MI4_R = crate::BitReader<MI4_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI4_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI4_A> for bool {
    #[inline(always)]
    fn from(variant: MI4_A) -> Self {
        variant as u8 != 0
    }
}
impl MI4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI4_A {
        match self.bits {
            false => MI4_A::_0,
            true => MI4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI4_A::_1
    }
}
#[doc = "Field `MI4` writer - Individual Mask Bits"]
pub type MI4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI4_A, O>;
impl<'a, const O: u8> MI4_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI4_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI4_A::_1)
    }
}
#[doc = "Field `MI5` reader - Individual Mask Bits"]
pub type MI5_R = crate::BitReader<MI5_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI5_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI5_A> for bool {
    #[inline(always)]
    fn from(variant: MI5_A) -> Self {
        variant as u8 != 0
    }
}
impl MI5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI5_A {
        match self.bits {
            false => MI5_A::_0,
            true => MI5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI5_A::_1
    }
}
#[doc = "Field `MI5` writer - Individual Mask Bits"]
pub type MI5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI5_A, O>;
impl<'a, const O: u8> MI5_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI5_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI5_A::_1)
    }
}
#[doc = "Field `MI6` reader - Individual Mask Bits"]
pub type MI6_R = crate::BitReader<MI6_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI6_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI6_A> for bool {
    #[inline(always)]
    fn from(variant: MI6_A) -> Self {
        variant as u8 != 0
    }
}
impl MI6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI6_A {
        match self.bits {
            false => MI6_A::_0,
            true => MI6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI6_A::_1
    }
}
#[doc = "Field `MI6` writer - Individual Mask Bits"]
pub type MI6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI6_A, O>;
impl<'a, const O: u8> MI6_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI6_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI6_A::_1)
    }
}
#[doc = "Field `MI7` reader - Individual Mask Bits"]
pub type MI7_R = crate::BitReader<MI7_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI7_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI7_A> for bool {
    #[inline(always)]
    fn from(variant: MI7_A) -> Self {
        variant as u8 != 0
    }
}
impl MI7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI7_A {
        match self.bits {
            false => MI7_A::_0,
            true => MI7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI7_A::_1
    }
}
#[doc = "Field `MI7` writer - Individual Mask Bits"]
pub type MI7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI7_A, O>;
impl<'a, const O: u8> MI7_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI7_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI7_A::_1)
    }
}
#[doc = "Field `MI8` reader - Individual Mask Bits"]
pub type MI8_R = crate::BitReader<MI8_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI8_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI8_A> for bool {
    #[inline(always)]
    fn from(variant: MI8_A) -> Self {
        variant as u8 != 0
    }
}
impl MI8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI8_A {
        match self.bits {
            false => MI8_A::_0,
            true => MI8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI8_A::_1
    }
}
#[doc = "Field `MI8` writer - Individual Mask Bits"]
pub type MI8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI8_A, O>;
impl<'a, const O: u8> MI8_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI8_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI8_A::_1)
    }
}
#[doc = "Field `MI9` reader - Individual Mask Bits"]
pub type MI9_R = crate::BitReader<MI9_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI9_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI9_A> for bool {
    #[inline(always)]
    fn from(variant: MI9_A) -> Self {
        variant as u8 != 0
    }
}
impl MI9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI9_A {
        match self.bits {
            false => MI9_A::_0,
            true => MI9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI9_A::_1
    }
}
#[doc = "Field `MI9` writer - Individual Mask Bits"]
pub type MI9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI9_A, O>;
impl<'a, const O: u8> MI9_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI9_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI9_A::_1)
    }
}
#[doc = "Field `MI10` reader - Individual Mask Bits"]
pub type MI10_R = crate::BitReader<MI10_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI10_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI10_A> for bool {
    #[inline(always)]
    fn from(variant: MI10_A) -> Self {
        variant as u8 != 0
    }
}
impl MI10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI10_A {
        match self.bits {
            false => MI10_A::_0,
            true => MI10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI10_A::_1
    }
}
#[doc = "Field `MI10` writer - Individual Mask Bits"]
pub type MI10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI10_A, O>;
impl<'a, const O: u8> MI10_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI10_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI10_A::_1)
    }
}
#[doc = "Field `MI11` reader - Individual Mask Bits"]
pub type MI11_R = crate::BitReader<MI11_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI11_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI11_A> for bool {
    #[inline(always)]
    fn from(variant: MI11_A) -> Self {
        variant as u8 != 0
    }
}
impl MI11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI11_A {
        match self.bits {
            false => MI11_A::_0,
            true => MI11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI11_A::_1
    }
}
#[doc = "Field `MI11` writer - Individual Mask Bits"]
pub type MI11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI11_A, O>;
impl<'a, const O: u8> MI11_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI11_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI11_A::_1)
    }
}
#[doc = "Field `MI12` reader - Individual Mask Bits"]
pub type MI12_R = crate::BitReader<MI12_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI12_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI12_A> for bool {
    #[inline(always)]
    fn from(variant: MI12_A) -> Self {
        variant as u8 != 0
    }
}
impl MI12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI12_A {
        match self.bits {
            false => MI12_A::_0,
            true => MI12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI12_A::_1
    }
}
#[doc = "Field `MI12` writer - Individual Mask Bits"]
pub type MI12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI12_A, O>;
impl<'a, const O: u8> MI12_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI12_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI12_A::_1)
    }
}
#[doc = "Field `MI13` reader - Individual Mask Bits"]
pub type MI13_R = crate::BitReader<MI13_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI13_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI13_A> for bool {
    #[inline(always)]
    fn from(variant: MI13_A) -> Self {
        variant as u8 != 0
    }
}
impl MI13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI13_A {
        match self.bits {
            false => MI13_A::_0,
            true => MI13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI13_A::_1
    }
}
#[doc = "Field `MI13` writer - Individual Mask Bits"]
pub type MI13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI13_A, O>;
impl<'a, const O: u8> MI13_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI13_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI13_A::_1)
    }
}
#[doc = "Field `MI14` reader - Individual Mask Bits"]
pub type MI14_R = crate::BitReader<MI14_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI14_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI14_A> for bool {
    #[inline(always)]
    fn from(variant: MI14_A) -> Self {
        variant as u8 != 0
    }
}
impl MI14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI14_A {
        match self.bits {
            false => MI14_A::_0,
            true => MI14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI14_A::_1
    }
}
#[doc = "Field `MI14` writer - Individual Mask Bits"]
pub type MI14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI14_A, O>;
impl<'a, const O: u8> MI14_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI14_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI14_A::_1)
    }
}
#[doc = "Field `MI15` reader - Individual Mask Bits"]
pub type MI15_R = crate::BitReader<MI15_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI15_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI15_A> for bool {
    #[inline(always)]
    fn from(variant: MI15_A) -> Self {
        variant as u8 != 0
    }
}
impl MI15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI15_A {
        match self.bits {
            false => MI15_A::_0,
            true => MI15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI15_A::_1
    }
}
#[doc = "Field `MI15` writer - Individual Mask Bits"]
pub type MI15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI15_A, O>;
impl<'a, const O: u8> MI15_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI15_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI15_A::_1)
    }
}
#[doc = "Field `MI16` reader - Individual Mask Bits"]
pub type MI16_R = crate::BitReader<MI16_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI16_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI16_A> for bool {
    #[inline(always)]
    fn from(variant: MI16_A) -> Self {
        variant as u8 != 0
    }
}
impl MI16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI16_A {
        match self.bits {
            false => MI16_A::_0,
            true => MI16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI16_A::_1
    }
}
#[doc = "Field `MI16` writer - Individual Mask Bits"]
pub type MI16_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI16_A, O>;
impl<'a, const O: u8> MI16_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI16_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI16_A::_1)
    }
}
#[doc = "Field `MI17` reader - Individual Mask Bits"]
pub type MI17_R = crate::BitReader<MI17_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI17_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI17_A> for bool {
    #[inline(always)]
    fn from(variant: MI17_A) -> Self {
        variant as u8 != 0
    }
}
impl MI17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI17_A {
        match self.bits {
            false => MI17_A::_0,
            true => MI17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI17_A::_1
    }
}
#[doc = "Field `MI17` writer - Individual Mask Bits"]
pub type MI17_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI17_A, O>;
impl<'a, const O: u8> MI17_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI17_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI17_A::_1)
    }
}
#[doc = "Field `MI18` reader - Individual Mask Bits"]
pub type MI18_R = crate::BitReader<MI18_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI18_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI18_A> for bool {
    #[inline(always)]
    fn from(variant: MI18_A) -> Self {
        variant as u8 != 0
    }
}
impl MI18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI18_A {
        match self.bits {
            false => MI18_A::_0,
            true => MI18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI18_A::_1
    }
}
#[doc = "Field `MI18` writer - Individual Mask Bits"]
pub type MI18_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI18_A, O>;
impl<'a, const O: u8> MI18_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI18_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI18_A::_1)
    }
}
#[doc = "Field `MI19` reader - Individual Mask Bits"]
pub type MI19_R = crate::BitReader<MI19_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI19_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI19_A> for bool {
    #[inline(always)]
    fn from(variant: MI19_A) -> Self {
        variant as u8 != 0
    }
}
impl MI19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI19_A {
        match self.bits {
            false => MI19_A::_0,
            true => MI19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI19_A::_1
    }
}
#[doc = "Field `MI19` writer - Individual Mask Bits"]
pub type MI19_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI19_A, O>;
impl<'a, const O: u8> MI19_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI19_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI19_A::_1)
    }
}
#[doc = "Field `MI20` reader - Individual Mask Bits"]
pub type MI20_R = crate::BitReader<MI20_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI20_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI20_A> for bool {
    #[inline(always)]
    fn from(variant: MI20_A) -> Self {
        variant as u8 != 0
    }
}
impl MI20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI20_A {
        match self.bits {
            false => MI20_A::_0,
            true => MI20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI20_A::_1
    }
}
#[doc = "Field `MI20` writer - Individual Mask Bits"]
pub type MI20_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI20_A, O>;
impl<'a, const O: u8> MI20_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI20_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI20_A::_1)
    }
}
#[doc = "Field `MI21` reader - Individual Mask Bits"]
pub type MI21_R = crate::BitReader<MI21_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI21_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI21_A> for bool {
    #[inline(always)]
    fn from(variant: MI21_A) -> Self {
        variant as u8 != 0
    }
}
impl MI21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI21_A {
        match self.bits {
            false => MI21_A::_0,
            true => MI21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI21_A::_1
    }
}
#[doc = "Field `MI21` writer - Individual Mask Bits"]
pub type MI21_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI21_A, O>;
impl<'a, const O: u8> MI21_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI21_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI21_A::_1)
    }
}
#[doc = "Field `MI22` reader - Individual Mask Bits"]
pub type MI22_R = crate::BitReader<MI22_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI22_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI22_A> for bool {
    #[inline(always)]
    fn from(variant: MI22_A) -> Self {
        variant as u8 != 0
    }
}
impl MI22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI22_A {
        match self.bits {
            false => MI22_A::_0,
            true => MI22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI22_A::_1
    }
}
#[doc = "Field `MI22` writer - Individual Mask Bits"]
pub type MI22_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI22_A, O>;
impl<'a, const O: u8> MI22_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI22_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI22_A::_1)
    }
}
#[doc = "Field `MI23` reader - Individual Mask Bits"]
pub type MI23_R = crate::BitReader<MI23_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI23_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI23_A> for bool {
    #[inline(always)]
    fn from(variant: MI23_A) -> Self {
        variant as u8 != 0
    }
}
impl MI23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI23_A {
        match self.bits {
            false => MI23_A::_0,
            true => MI23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI23_A::_1
    }
}
#[doc = "Field `MI23` writer - Individual Mask Bits"]
pub type MI23_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI23_A, O>;
impl<'a, const O: u8> MI23_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI23_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI23_A::_1)
    }
}
#[doc = "Field `MI24` reader - Individual Mask Bits"]
pub type MI24_R = crate::BitReader<MI24_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI24_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI24_A> for bool {
    #[inline(always)]
    fn from(variant: MI24_A) -> Self {
        variant as u8 != 0
    }
}
impl MI24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI24_A {
        match self.bits {
            false => MI24_A::_0,
            true => MI24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI24_A::_1
    }
}
#[doc = "Field `MI24` writer - Individual Mask Bits"]
pub type MI24_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI24_A, O>;
impl<'a, const O: u8> MI24_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI24_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI24_A::_1)
    }
}
#[doc = "Field `MI25` reader - Individual Mask Bits"]
pub type MI25_R = crate::BitReader<MI25_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI25_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI25_A> for bool {
    #[inline(always)]
    fn from(variant: MI25_A) -> Self {
        variant as u8 != 0
    }
}
impl MI25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI25_A {
        match self.bits {
            false => MI25_A::_0,
            true => MI25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI25_A::_1
    }
}
#[doc = "Field `MI25` writer - Individual Mask Bits"]
pub type MI25_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI25_A, O>;
impl<'a, const O: u8> MI25_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI25_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI25_A::_1)
    }
}
#[doc = "Field `MI26` reader - Individual Mask Bits"]
pub type MI26_R = crate::BitReader<MI26_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI26_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI26_A> for bool {
    #[inline(always)]
    fn from(variant: MI26_A) -> Self {
        variant as u8 != 0
    }
}
impl MI26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI26_A {
        match self.bits {
            false => MI26_A::_0,
            true => MI26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI26_A::_1
    }
}
#[doc = "Field `MI26` writer - Individual Mask Bits"]
pub type MI26_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI26_A, O>;
impl<'a, const O: u8> MI26_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI26_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI26_A::_1)
    }
}
#[doc = "Field `MI27` reader - Individual Mask Bits"]
pub type MI27_R = crate::BitReader<MI27_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI27_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI27_A> for bool {
    #[inline(always)]
    fn from(variant: MI27_A) -> Self {
        variant as u8 != 0
    }
}
impl MI27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI27_A {
        match self.bits {
            false => MI27_A::_0,
            true => MI27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI27_A::_1
    }
}
#[doc = "Field `MI27` writer - Individual Mask Bits"]
pub type MI27_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI27_A, O>;
impl<'a, const O: u8> MI27_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI27_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI27_A::_1)
    }
}
#[doc = "Field `MI28` reader - Individual Mask Bits"]
pub type MI28_R = crate::BitReader<MI28_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI28_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI28_A> for bool {
    #[inline(always)]
    fn from(variant: MI28_A) -> Self {
        variant as u8 != 0
    }
}
impl MI28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI28_A {
        match self.bits {
            false => MI28_A::_0,
            true => MI28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI28_A::_1
    }
}
#[doc = "Field `MI28` writer - Individual Mask Bits"]
pub type MI28_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI28_A, O>;
impl<'a, const O: u8> MI28_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI28_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI28_A::_1)
    }
}
#[doc = "Field `MI29` reader - Individual Mask Bits"]
pub type MI29_R = crate::BitReader<MI29_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI29_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI29_A> for bool {
    #[inline(always)]
    fn from(variant: MI29_A) -> Self {
        variant as u8 != 0
    }
}
impl MI29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI29_A {
        match self.bits {
            false => MI29_A::_0,
            true => MI29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI29_A::_1
    }
}
#[doc = "Field `MI29` writer - Individual Mask Bits"]
pub type MI29_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI29_A, O>;
impl<'a, const O: u8> MI29_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI29_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI29_A::_1)
    }
}
#[doc = "Field `MI30` reader - Individual Mask Bits"]
pub type MI30_R = crate::BitReader<MI30_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI30_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI30_A> for bool {
    #[inline(always)]
    fn from(variant: MI30_A) -> Self {
        variant as u8 != 0
    }
}
impl MI30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI30_A {
        match self.bits {
            false => MI30_A::_0,
            true => MI30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI30_A::_1
    }
}
#[doc = "Field `MI30` writer - Individual Mask Bits"]
pub type MI30_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI30_A, O>;
impl<'a, const O: u8> MI30_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI30_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI30_A::_1)
    }
}
#[doc = "Field `MI31` reader - Individual Mask Bits"]
pub type MI31_R = crate::BitReader<MI31_A>;
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI31_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI31_A> for bool {
    #[inline(always)]
    fn from(variant: MI31_A) -> Self {
        variant as u8 != 0
    }
}
impl MI31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI31_A {
        match self.bits {
            false => MI31_A::_0,
            true => MI31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI31_A::_1
    }
}
#[doc = "Field `MI31` writer - Individual Mask Bits"]
pub type MI31_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIMR_SPEC, MI31_A, O>;
impl<'a, const O: u8> MI31_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI31_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi0(&self) -> MI0_R {
        MI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi1(&self) -> MI1_R {
        MI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi2(&self) -> MI2_R {
        MI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi3(&self) -> MI3_R {
        MI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi4(&self) -> MI4_R {
        MI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi5(&self) -> MI5_R {
        MI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi6(&self) -> MI6_R {
        MI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi7(&self) -> MI7_R {
        MI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi8(&self) -> MI8_R {
        MI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi9(&self) -> MI9_R {
        MI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi10(&self) -> MI10_R {
        MI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi11(&self) -> MI11_R {
        MI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi12(&self) -> MI12_R {
        MI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi13(&self) -> MI13_R {
        MI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi14(&self) -> MI14_R {
        MI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi15(&self) -> MI15_R {
        MI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi16(&self) -> MI16_R {
        MI16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi17(&self) -> MI17_R {
        MI17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi18(&self) -> MI18_R {
        MI18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi19(&self) -> MI19_R {
        MI19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi20(&self) -> MI20_R {
        MI20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi21(&self) -> MI21_R {
        MI21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi22(&self) -> MI22_R {
        MI22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi23(&self) -> MI23_R {
        MI23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi24(&self) -> MI24_R {
        MI24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi25(&self) -> MI25_R {
        MI25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi26(&self) -> MI26_R {
        MI26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi27(&self) -> MI27_R {
        MI27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi28(&self) -> MI28_R {
        MI28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi29(&self) -> MI29_R {
        MI29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi30(&self) -> MI30_R {
        MI30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi31(&self) -> MI31_R {
        MI31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi0(&mut self) -> MI0_W<0> {
        MI0_W::new(self)
    }
    #[doc = "Bit 1 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi1(&mut self) -> MI1_W<1> {
        MI1_W::new(self)
    }
    #[doc = "Bit 2 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi2(&mut self) -> MI2_W<2> {
        MI2_W::new(self)
    }
    #[doc = "Bit 3 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi3(&mut self) -> MI3_W<3> {
        MI3_W::new(self)
    }
    #[doc = "Bit 4 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi4(&mut self) -> MI4_W<4> {
        MI4_W::new(self)
    }
    #[doc = "Bit 5 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi5(&mut self) -> MI5_W<5> {
        MI5_W::new(self)
    }
    #[doc = "Bit 6 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi6(&mut self) -> MI6_W<6> {
        MI6_W::new(self)
    }
    #[doc = "Bit 7 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi7(&mut self) -> MI7_W<7> {
        MI7_W::new(self)
    }
    #[doc = "Bit 8 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi8(&mut self) -> MI8_W<8> {
        MI8_W::new(self)
    }
    #[doc = "Bit 9 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi9(&mut self) -> MI9_W<9> {
        MI9_W::new(self)
    }
    #[doc = "Bit 10 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi10(&mut self) -> MI10_W<10> {
        MI10_W::new(self)
    }
    #[doc = "Bit 11 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi11(&mut self) -> MI11_W<11> {
        MI11_W::new(self)
    }
    #[doc = "Bit 12 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi12(&mut self) -> MI12_W<12> {
        MI12_W::new(self)
    }
    #[doc = "Bit 13 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi13(&mut self) -> MI13_W<13> {
        MI13_W::new(self)
    }
    #[doc = "Bit 14 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi14(&mut self) -> MI14_W<14> {
        MI14_W::new(self)
    }
    #[doc = "Bit 15 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi15(&mut self) -> MI15_W<15> {
        MI15_W::new(self)
    }
    #[doc = "Bit 16 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi16(&mut self) -> MI16_W<16> {
        MI16_W::new(self)
    }
    #[doc = "Bit 17 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi17(&mut self) -> MI17_W<17> {
        MI17_W::new(self)
    }
    #[doc = "Bit 18 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi18(&mut self) -> MI18_W<18> {
        MI18_W::new(self)
    }
    #[doc = "Bit 19 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi19(&mut self) -> MI19_W<19> {
        MI19_W::new(self)
    }
    #[doc = "Bit 20 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi20(&mut self) -> MI20_W<20> {
        MI20_W::new(self)
    }
    #[doc = "Bit 21 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi21(&mut self) -> MI21_W<21> {
        MI21_W::new(self)
    }
    #[doc = "Bit 22 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi22(&mut self) -> MI22_W<22> {
        MI22_W::new(self)
    }
    #[doc = "Bit 23 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi23(&mut self) -> MI23_W<23> {
        MI23_W::new(self)
    }
    #[doc = "Bit 24 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi24(&mut self) -> MI24_W<24> {
        MI24_W::new(self)
    }
    #[doc = "Bit 25 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi25(&mut self) -> MI25_W<25> {
        MI25_W::new(self)
    }
    #[doc = "Bit 26 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi26(&mut self) -> MI26_W<26> {
        MI26_W::new(self)
    }
    #[doc = "Bit 27 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi27(&mut self) -> MI27_W<27> {
        MI27_W::new(self)
    }
    #[doc = "Bit 28 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi28(&mut self) -> MI28_W<28> {
        MI28_W::new(self)
    }
    #[doc = "Bit 29 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi29(&mut self) -> MI29_W<29> {
        MI29_W::new(self)
    }
    #[doc = "Bit 30 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi30(&mut self) -> MI30_W<30> {
        MI30_W::new(self)
    }
    #[doc = "Bit 31 - Individual Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mi31(&mut self) -> MI31_W<31> {
        MI31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Individual Mask Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rximr](index.html) module"]
pub struct RXIMR_SPEC;
impl crate::RegisterSpec for RXIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rximr::R](R) reader structure"]
impl crate::Readable for RXIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rximr::W](W) writer structure"]
impl crate::Writable for RXIMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXIMR%s to value 0"]
impl crate::Resettable for RXIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

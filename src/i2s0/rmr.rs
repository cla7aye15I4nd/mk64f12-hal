#[doc = "Register `RMR` reader"]
pub struct R(crate::R<RMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMR` writer"]
pub struct W(crate::W<RMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMR_SPEC>;
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
impl From<crate::W<RMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWM0` reader - Receive Word Mask"]
pub type RWM0_R = crate::BitReader<RWM0_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM0_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM0_A> for bool {
    #[inline(always)]
    fn from(variant: RWM0_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM0_A {
        match self.bits {
            false => RWM0_A::_0,
            true => RWM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM0_A::_1
    }
}
#[doc = "Field `RWM0` writer - Receive Word Mask"]
pub type RWM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM0_A, O>;
impl<'a, const O: u8> RWM0_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM0_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM0_A::_1)
    }
}
#[doc = "Field `RWM1` reader - Receive Word Mask"]
pub type RWM1_R = crate::BitReader<RWM1_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM1_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM1_A> for bool {
    #[inline(always)]
    fn from(variant: RWM1_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM1_A {
        match self.bits {
            false => RWM1_A::_0,
            true => RWM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM1_A::_1
    }
}
#[doc = "Field `RWM1` writer - Receive Word Mask"]
pub type RWM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM1_A, O>;
impl<'a, const O: u8> RWM1_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM1_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM1_A::_1)
    }
}
#[doc = "Field `RWM2` reader - Receive Word Mask"]
pub type RWM2_R = crate::BitReader<RWM2_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM2_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM2_A> for bool {
    #[inline(always)]
    fn from(variant: RWM2_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM2_A {
        match self.bits {
            false => RWM2_A::_0,
            true => RWM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM2_A::_1
    }
}
#[doc = "Field `RWM2` writer - Receive Word Mask"]
pub type RWM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM2_A, O>;
impl<'a, const O: u8> RWM2_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM2_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM2_A::_1)
    }
}
#[doc = "Field `RWM3` reader - Receive Word Mask"]
pub type RWM3_R = crate::BitReader<RWM3_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM3_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM3_A> for bool {
    #[inline(always)]
    fn from(variant: RWM3_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM3_A {
        match self.bits {
            false => RWM3_A::_0,
            true => RWM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM3_A::_1
    }
}
#[doc = "Field `RWM3` writer - Receive Word Mask"]
pub type RWM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM3_A, O>;
impl<'a, const O: u8> RWM3_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM3_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM3_A::_1)
    }
}
#[doc = "Field `RWM4` reader - Receive Word Mask"]
pub type RWM4_R = crate::BitReader<RWM4_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM4_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM4_A> for bool {
    #[inline(always)]
    fn from(variant: RWM4_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM4_A {
        match self.bits {
            false => RWM4_A::_0,
            true => RWM4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM4_A::_1
    }
}
#[doc = "Field `RWM4` writer - Receive Word Mask"]
pub type RWM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM4_A, O>;
impl<'a, const O: u8> RWM4_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM4_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM4_A::_1)
    }
}
#[doc = "Field `RWM5` reader - Receive Word Mask"]
pub type RWM5_R = crate::BitReader<RWM5_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM5_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM5_A> for bool {
    #[inline(always)]
    fn from(variant: RWM5_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM5_A {
        match self.bits {
            false => RWM5_A::_0,
            true => RWM5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM5_A::_1
    }
}
#[doc = "Field `RWM5` writer - Receive Word Mask"]
pub type RWM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM5_A, O>;
impl<'a, const O: u8> RWM5_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM5_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM5_A::_1)
    }
}
#[doc = "Field `RWM6` reader - Receive Word Mask"]
pub type RWM6_R = crate::BitReader<RWM6_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM6_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM6_A> for bool {
    #[inline(always)]
    fn from(variant: RWM6_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM6_A {
        match self.bits {
            false => RWM6_A::_0,
            true => RWM6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM6_A::_1
    }
}
#[doc = "Field `RWM6` writer - Receive Word Mask"]
pub type RWM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM6_A, O>;
impl<'a, const O: u8> RWM6_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM6_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM6_A::_1)
    }
}
#[doc = "Field `RWM7` reader - Receive Word Mask"]
pub type RWM7_R = crate::BitReader<RWM7_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM7_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM7_A> for bool {
    #[inline(always)]
    fn from(variant: RWM7_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM7_A {
        match self.bits {
            false => RWM7_A::_0,
            true => RWM7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM7_A::_1
    }
}
#[doc = "Field `RWM7` writer - Receive Word Mask"]
pub type RWM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM7_A, O>;
impl<'a, const O: u8> RWM7_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM7_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM7_A::_1)
    }
}
#[doc = "Field `RWM8` reader - Receive Word Mask"]
pub type RWM8_R = crate::BitReader<RWM8_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM8_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM8_A> for bool {
    #[inline(always)]
    fn from(variant: RWM8_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM8_A {
        match self.bits {
            false => RWM8_A::_0,
            true => RWM8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM8_A::_1
    }
}
#[doc = "Field `RWM8` writer - Receive Word Mask"]
pub type RWM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM8_A, O>;
impl<'a, const O: u8> RWM8_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM8_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM8_A::_1)
    }
}
#[doc = "Field `RWM9` reader - Receive Word Mask"]
pub type RWM9_R = crate::BitReader<RWM9_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM9_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM9_A> for bool {
    #[inline(always)]
    fn from(variant: RWM9_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM9_A {
        match self.bits {
            false => RWM9_A::_0,
            true => RWM9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM9_A::_1
    }
}
#[doc = "Field `RWM9` writer - Receive Word Mask"]
pub type RWM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM9_A, O>;
impl<'a, const O: u8> RWM9_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM9_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM9_A::_1)
    }
}
#[doc = "Field `RWM10` reader - Receive Word Mask"]
pub type RWM10_R = crate::BitReader<RWM10_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM10_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM10_A> for bool {
    #[inline(always)]
    fn from(variant: RWM10_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM10_A {
        match self.bits {
            false => RWM10_A::_0,
            true => RWM10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM10_A::_1
    }
}
#[doc = "Field `RWM10` writer - Receive Word Mask"]
pub type RWM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM10_A, O>;
impl<'a, const O: u8> RWM10_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM10_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM10_A::_1)
    }
}
#[doc = "Field `RWM11` reader - Receive Word Mask"]
pub type RWM11_R = crate::BitReader<RWM11_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM11_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM11_A> for bool {
    #[inline(always)]
    fn from(variant: RWM11_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM11_A {
        match self.bits {
            false => RWM11_A::_0,
            true => RWM11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM11_A::_1
    }
}
#[doc = "Field `RWM11` writer - Receive Word Mask"]
pub type RWM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM11_A, O>;
impl<'a, const O: u8> RWM11_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM11_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM11_A::_1)
    }
}
#[doc = "Field `RWM12` reader - Receive Word Mask"]
pub type RWM12_R = crate::BitReader<RWM12_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM12_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM12_A> for bool {
    #[inline(always)]
    fn from(variant: RWM12_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM12_A {
        match self.bits {
            false => RWM12_A::_0,
            true => RWM12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM12_A::_1
    }
}
#[doc = "Field `RWM12` writer - Receive Word Mask"]
pub type RWM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM12_A, O>;
impl<'a, const O: u8> RWM12_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM12_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM12_A::_1)
    }
}
#[doc = "Field `RWM13` reader - Receive Word Mask"]
pub type RWM13_R = crate::BitReader<RWM13_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM13_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM13_A> for bool {
    #[inline(always)]
    fn from(variant: RWM13_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM13_A {
        match self.bits {
            false => RWM13_A::_0,
            true => RWM13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM13_A::_1
    }
}
#[doc = "Field `RWM13` writer - Receive Word Mask"]
pub type RWM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM13_A, O>;
impl<'a, const O: u8> RWM13_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM13_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM13_A::_1)
    }
}
#[doc = "Field `RWM14` reader - Receive Word Mask"]
pub type RWM14_R = crate::BitReader<RWM14_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM14_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM14_A> for bool {
    #[inline(always)]
    fn from(variant: RWM14_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM14_A {
        match self.bits {
            false => RWM14_A::_0,
            true => RWM14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM14_A::_1
    }
}
#[doc = "Field `RWM14` writer - Receive Word Mask"]
pub type RWM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM14_A, O>;
impl<'a, const O: u8> RWM14_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM14_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM14_A::_1)
    }
}
#[doc = "Field `RWM15` reader - Receive Word Mask"]
pub type RWM15_R = crate::BitReader<RWM15_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM15_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM15_A> for bool {
    #[inline(always)]
    fn from(variant: RWM15_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM15_A {
        match self.bits {
            false => RWM15_A::_0,
            true => RWM15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM15_A::_1
    }
}
#[doc = "Field `RWM15` writer - Receive Word Mask"]
pub type RWM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM15_A, O>;
impl<'a, const O: u8> RWM15_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM15_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM15_A::_1)
    }
}
#[doc = "Field `RWM16` reader - Receive Word Mask"]
pub type RWM16_R = crate::BitReader<RWM16_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM16_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM16_A> for bool {
    #[inline(always)]
    fn from(variant: RWM16_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM16_A {
        match self.bits {
            false => RWM16_A::_0,
            true => RWM16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM16_A::_1
    }
}
#[doc = "Field `RWM16` writer - Receive Word Mask"]
pub type RWM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM16_A, O>;
impl<'a, const O: u8> RWM16_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM16_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM16_A::_1)
    }
}
#[doc = "Field `RWM17` reader - Receive Word Mask"]
pub type RWM17_R = crate::BitReader<RWM17_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM17_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM17_A> for bool {
    #[inline(always)]
    fn from(variant: RWM17_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM17_A {
        match self.bits {
            false => RWM17_A::_0,
            true => RWM17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM17_A::_1
    }
}
#[doc = "Field `RWM17` writer - Receive Word Mask"]
pub type RWM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM17_A, O>;
impl<'a, const O: u8> RWM17_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM17_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM17_A::_1)
    }
}
#[doc = "Field `RWM18` reader - Receive Word Mask"]
pub type RWM18_R = crate::BitReader<RWM18_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM18_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM18_A> for bool {
    #[inline(always)]
    fn from(variant: RWM18_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM18_A {
        match self.bits {
            false => RWM18_A::_0,
            true => RWM18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM18_A::_1
    }
}
#[doc = "Field `RWM18` writer - Receive Word Mask"]
pub type RWM18_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM18_A, O>;
impl<'a, const O: u8> RWM18_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM18_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM18_A::_1)
    }
}
#[doc = "Field `RWM19` reader - Receive Word Mask"]
pub type RWM19_R = crate::BitReader<RWM19_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM19_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM19_A> for bool {
    #[inline(always)]
    fn from(variant: RWM19_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM19_A {
        match self.bits {
            false => RWM19_A::_0,
            true => RWM19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM19_A::_1
    }
}
#[doc = "Field `RWM19` writer - Receive Word Mask"]
pub type RWM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM19_A, O>;
impl<'a, const O: u8> RWM19_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM19_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM19_A::_1)
    }
}
#[doc = "Field `RWM20` reader - Receive Word Mask"]
pub type RWM20_R = crate::BitReader<RWM20_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM20_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM20_A> for bool {
    #[inline(always)]
    fn from(variant: RWM20_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM20_A {
        match self.bits {
            false => RWM20_A::_0,
            true => RWM20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM20_A::_1
    }
}
#[doc = "Field `RWM20` writer - Receive Word Mask"]
pub type RWM20_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM20_A, O>;
impl<'a, const O: u8> RWM20_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM20_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM20_A::_1)
    }
}
#[doc = "Field `RWM21` reader - Receive Word Mask"]
pub type RWM21_R = crate::BitReader<RWM21_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM21_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM21_A> for bool {
    #[inline(always)]
    fn from(variant: RWM21_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM21_A {
        match self.bits {
            false => RWM21_A::_0,
            true => RWM21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM21_A::_1
    }
}
#[doc = "Field `RWM21` writer - Receive Word Mask"]
pub type RWM21_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM21_A, O>;
impl<'a, const O: u8> RWM21_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM21_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM21_A::_1)
    }
}
#[doc = "Field `RWM22` reader - Receive Word Mask"]
pub type RWM22_R = crate::BitReader<RWM22_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM22_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM22_A> for bool {
    #[inline(always)]
    fn from(variant: RWM22_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM22_A {
        match self.bits {
            false => RWM22_A::_0,
            true => RWM22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM22_A::_1
    }
}
#[doc = "Field `RWM22` writer - Receive Word Mask"]
pub type RWM22_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM22_A, O>;
impl<'a, const O: u8> RWM22_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM22_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM22_A::_1)
    }
}
#[doc = "Field `RWM23` reader - Receive Word Mask"]
pub type RWM23_R = crate::BitReader<RWM23_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM23_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM23_A> for bool {
    #[inline(always)]
    fn from(variant: RWM23_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM23_A {
        match self.bits {
            false => RWM23_A::_0,
            true => RWM23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM23_A::_1
    }
}
#[doc = "Field `RWM23` writer - Receive Word Mask"]
pub type RWM23_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM23_A, O>;
impl<'a, const O: u8> RWM23_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM23_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM23_A::_1)
    }
}
#[doc = "Field `RWM24` reader - Receive Word Mask"]
pub type RWM24_R = crate::BitReader<RWM24_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM24_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM24_A> for bool {
    #[inline(always)]
    fn from(variant: RWM24_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM24_A {
        match self.bits {
            false => RWM24_A::_0,
            true => RWM24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM24_A::_1
    }
}
#[doc = "Field `RWM24` writer - Receive Word Mask"]
pub type RWM24_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM24_A, O>;
impl<'a, const O: u8> RWM24_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM24_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM24_A::_1)
    }
}
#[doc = "Field `RWM25` reader - Receive Word Mask"]
pub type RWM25_R = crate::BitReader<RWM25_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM25_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM25_A> for bool {
    #[inline(always)]
    fn from(variant: RWM25_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM25_A {
        match self.bits {
            false => RWM25_A::_0,
            true => RWM25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM25_A::_1
    }
}
#[doc = "Field `RWM25` writer - Receive Word Mask"]
pub type RWM25_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM25_A, O>;
impl<'a, const O: u8> RWM25_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM25_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM25_A::_1)
    }
}
#[doc = "Field `RWM26` reader - Receive Word Mask"]
pub type RWM26_R = crate::BitReader<RWM26_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM26_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM26_A> for bool {
    #[inline(always)]
    fn from(variant: RWM26_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM26_A {
        match self.bits {
            false => RWM26_A::_0,
            true => RWM26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM26_A::_1
    }
}
#[doc = "Field `RWM26` writer - Receive Word Mask"]
pub type RWM26_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM26_A, O>;
impl<'a, const O: u8> RWM26_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM26_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM26_A::_1)
    }
}
#[doc = "Field `RWM27` reader - Receive Word Mask"]
pub type RWM27_R = crate::BitReader<RWM27_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM27_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM27_A> for bool {
    #[inline(always)]
    fn from(variant: RWM27_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM27_A {
        match self.bits {
            false => RWM27_A::_0,
            true => RWM27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM27_A::_1
    }
}
#[doc = "Field `RWM27` writer - Receive Word Mask"]
pub type RWM27_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM27_A, O>;
impl<'a, const O: u8> RWM27_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM27_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM27_A::_1)
    }
}
#[doc = "Field `RWM28` reader - Receive Word Mask"]
pub type RWM28_R = crate::BitReader<RWM28_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM28_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM28_A> for bool {
    #[inline(always)]
    fn from(variant: RWM28_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM28_A {
        match self.bits {
            false => RWM28_A::_0,
            true => RWM28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM28_A::_1
    }
}
#[doc = "Field `RWM28` writer - Receive Word Mask"]
pub type RWM28_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM28_A, O>;
impl<'a, const O: u8> RWM28_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM28_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM28_A::_1)
    }
}
#[doc = "Field `RWM29` reader - Receive Word Mask"]
pub type RWM29_R = crate::BitReader<RWM29_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM29_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM29_A> for bool {
    #[inline(always)]
    fn from(variant: RWM29_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM29_A {
        match self.bits {
            false => RWM29_A::_0,
            true => RWM29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM29_A::_1
    }
}
#[doc = "Field `RWM29` writer - Receive Word Mask"]
pub type RWM29_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM29_A, O>;
impl<'a, const O: u8> RWM29_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM29_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM29_A::_1)
    }
}
#[doc = "Field `RWM30` reader - Receive Word Mask"]
pub type RWM30_R = crate::BitReader<RWM30_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM30_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM30_A> for bool {
    #[inline(always)]
    fn from(variant: RWM30_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM30_A {
        match self.bits {
            false => RWM30_A::_0,
            true => RWM30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM30_A::_1
    }
}
#[doc = "Field `RWM30` writer - Receive Word Mask"]
pub type RWM30_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM30_A, O>;
impl<'a, const O: u8> RWM30_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM30_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM30_A::_1)
    }
}
#[doc = "Field `RWM31` reader - Receive Word Mask"]
pub type RWM31_R = crate::BitReader<RWM31_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWM31_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM31_A> for bool {
    #[inline(always)]
    fn from(variant: RWM31_A) -> Self {
        variant as u8 != 0
    }
}
impl RWM31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM31_A {
        match self.bits {
            false => RWM31_A::_0,
            true => RWM31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM31_A::_1
    }
}
#[doc = "Field `RWM31` writer - Receive Word Mask"]
pub type RWM31_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMR_SPEC, RWM31_A, O>;
impl<'a, const O: u8> RWM31_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM31_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm0(&self) -> RWM0_R {
        RWM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm1(&self) -> RWM1_R {
        RWM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm2(&self) -> RWM2_R {
        RWM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm3(&self) -> RWM3_R {
        RWM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm4(&self) -> RWM4_R {
        RWM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm5(&self) -> RWM5_R {
        RWM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm6(&self) -> RWM6_R {
        RWM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm7(&self) -> RWM7_R {
        RWM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm8(&self) -> RWM8_R {
        RWM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm9(&self) -> RWM9_R {
        RWM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm10(&self) -> RWM10_R {
        RWM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm11(&self) -> RWM11_R {
        RWM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm12(&self) -> RWM12_R {
        RWM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm13(&self) -> RWM13_R {
        RWM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm14(&self) -> RWM14_R {
        RWM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm15(&self) -> RWM15_R {
        RWM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm16(&self) -> RWM16_R {
        RWM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm17(&self) -> RWM17_R {
        RWM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm18(&self) -> RWM18_R {
        RWM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm19(&self) -> RWM19_R {
        RWM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm20(&self) -> RWM20_R {
        RWM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm21(&self) -> RWM21_R {
        RWM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm22(&self) -> RWM22_R {
        RWM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm23(&self) -> RWM23_R {
        RWM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm24(&self) -> RWM24_R {
        RWM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm25(&self) -> RWM25_R {
        RWM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm26(&self) -> RWM26_R {
        RWM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm27(&self) -> RWM27_R {
        RWM27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm28(&self) -> RWM28_R {
        RWM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm29(&self) -> RWM29_R {
        RWM29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm30(&self) -> RWM30_R {
        RWM30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm31(&self) -> RWM31_R {
        RWM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm0(&mut self) -> RWM0_W<0> {
        RWM0_W::new(self)
    }
    #[doc = "Bit 1 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm1(&mut self) -> RWM1_W<1> {
        RWM1_W::new(self)
    }
    #[doc = "Bit 2 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm2(&mut self) -> RWM2_W<2> {
        RWM2_W::new(self)
    }
    #[doc = "Bit 3 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm3(&mut self) -> RWM3_W<3> {
        RWM3_W::new(self)
    }
    #[doc = "Bit 4 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm4(&mut self) -> RWM4_W<4> {
        RWM4_W::new(self)
    }
    #[doc = "Bit 5 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm5(&mut self) -> RWM5_W<5> {
        RWM5_W::new(self)
    }
    #[doc = "Bit 6 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm6(&mut self) -> RWM6_W<6> {
        RWM6_W::new(self)
    }
    #[doc = "Bit 7 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm7(&mut self) -> RWM7_W<7> {
        RWM7_W::new(self)
    }
    #[doc = "Bit 8 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm8(&mut self) -> RWM8_W<8> {
        RWM8_W::new(self)
    }
    #[doc = "Bit 9 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm9(&mut self) -> RWM9_W<9> {
        RWM9_W::new(self)
    }
    #[doc = "Bit 10 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm10(&mut self) -> RWM10_W<10> {
        RWM10_W::new(self)
    }
    #[doc = "Bit 11 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm11(&mut self) -> RWM11_W<11> {
        RWM11_W::new(self)
    }
    #[doc = "Bit 12 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm12(&mut self) -> RWM12_W<12> {
        RWM12_W::new(self)
    }
    #[doc = "Bit 13 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm13(&mut self) -> RWM13_W<13> {
        RWM13_W::new(self)
    }
    #[doc = "Bit 14 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm14(&mut self) -> RWM14_W<14> {
        RWM14_W::new(self)
    }
    #[doc = "Bit 15 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm15(&mut self) -> RWM15_W<15> {
        RWM15_W::new(self)
    }
    #[doc = "Bit 16 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm16(&mut self) -> RWM16_W<16> {
        RWM16_W::new(self)
    }
    #[doc = "Bit 17 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm17(&mut self) -> RWM17_W<17> {
        RWM17_W::new(self)
    }
    #[doc = "Bit 18 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm18(&mut self) -> RWM18_W<18> {
        RWM18_W::new(self)
    }
    #[doc = "Bit 19 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm19(&mut self) -> RWM19_W<19> {
        RWM19_W::new(self)
    }
    #[doc = "Bit 20 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm20(&mut self) -> RWM20_W<20> {
        RWM20_W::new(self)
    }
    #[doc = "Bit 21 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm21(&mut self) -> RWM21_W<21> {
        RWM21_W::new(self)
    }
    #[doc = "Bit 22 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm22(&mut self) -> RWM22_W<22> {
        RWM22_W::new(self)
    }
    #[doc = "Bit 23 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm23(&mut self) -> RWM23_W<23> {
        RWM23_W::new(self)
    }
    #[doc = "Bit 24 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm24(&mut self) -> RWM24_W<24> {
        RWM24_W::new(self)
    }
    #[doc = "Bit 25 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm25(&mut self) -> RWM25_W<25> {
        RWM25_W::new(self)
    }
    #[doc = "Bit 26 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm26(&mut self) -> RWM26_W<26> {
        RWM26_W::new(self)
    }
    #[doc = "Bit 27 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm27(&mut self) -> RWM27_W<27> {
        RWM27_W::new(self)
    }
    #[doc = "Bit 28 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm28(&mut self) -> RWM28_W<28> {
        RWM28_W::new(self)
    }
    #[doc = "Bit 29 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm29(&mut self) -> RWM29_W<29> {
        RWM29_W::new(self)
    }
    #[doc = "Bit 30 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm30(&mut self) -> RWM30_W<30> {
        RWM30_W::new(self)
    }
    #[doc = "Bit 31 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm31(&mut self) -> RWM31_W<31> {
        RWM31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Receive Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmr](index.html) module"]
pub struct RMR_SPEC;
impl crate::RegisterSpec for RMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmr::R](R) reader structure"]
impl crate::Readable for RMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmr::W](W) writer structure"]
impl crate::Writable for RMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMR to value 0"]
impl crate::Resettable for RMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

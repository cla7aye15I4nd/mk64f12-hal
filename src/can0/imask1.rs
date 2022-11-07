#[doc = "Register `IMASK1` reader"]
pub struct R(crate::R<IMASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMASK1` writer"]
pub struct W(crate::W<IMASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMASK1_SPEC>;
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
impl From<crate::W<IMASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFLM0` reader - Buffer MB i Mask"]
pub type BUFLM0_R = crate::BitReader<BUFLM0_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM0_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM0_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM0_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM0_A {
        match self.bits {
            false => BUFLM0_A::_0,
            true => BUFLM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM0_A::_1
    }
}
#[doc = "Field `BUFLM0` writer - Buffer MB i Mask"]
pub type BUFLM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM0_A, O>;
impl<'a, const O: u8> BUFLM0_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM0_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM0_A::_1)
    }
}
#[doc = "Field `BUFLM1` reader - Buffer MB i Mask"]
pub type BUFLM1_R = crate::BitReader<BUFLM1_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM1_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM1_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM1_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM1_A {
        match self.bits {
            false => BUFLM1_A::_0,
            true => BUFLM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM1_A::_1
    }
}
#[doc = "Field `BUFLM1` writer - Buffer MB i Mask"]
pub type BUFLM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM1_A, O>;
impl<'a, const O: u8> BUFLM1_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM1_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM1_A::_1)
    }
}
#[doc = "Field `BUFLM2` reader - Buffer MB i Mask"]
pub type BUFLM2_R = crate::BitReader<BUFLM2_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM2_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM2_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM2_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM2_A {
        match self.bits {
            false => BUFLM2_A::_0,
            true => BUFLM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM2_A::_1
    }
}
#[doc = "Field `BUFLM2` writer - Buffer MB i Mask"]
pub type BUFLM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM2_A, O>;
impl<'a, const O: u8> BUFLM2_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM2_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM2_A::_1)
    }
}
#[doc = "Field `BUFLM3` reader - Buffer MB i Mask"]
pub type BUFLM3_R = crate::BitReader<BUFLM3_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM3_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM3_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM3_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM3_A {
        match self.bits {
            false => BUFLM3_A::_0,
            true => BUFLM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM3_A::_1
    }
}
#[doc = "Field `BUFLM3` writer - Buffer MB i Mask"]
pub type BUFLM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM3_A, O>;
impl<'a, const O: u8> BUFLM3_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM3_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM3_A::_1)
    }
}
#[doc = "Field `BUFLM4` reader - Buffer MB i Mask"]
pub type BUFLM4_R = crate::BitReader<BUFLM4_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM4_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM4_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM4_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM4_A {
        match self.bits {
            false => BUFLM4_A::_0,
            true => BUFLM4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM4_A::_1
    }
}
#[doc = "Field `BUFLM4` writer - Buffer MB i Mask"]
pub type BUFLM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM4_A, O>;
impl<'a, const O: u8> BUFLM4_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM4_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM4_A::_1)
    }
}
#[doc = "Field `BUFLM5` reader - Buffer MB i Mask"]
pub type BUFLM5_R = crate::BitReader<BUFLM5_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM5_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM5_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM5_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM5_A {
        match self.bits {
            false => BUFLM5_A::_0,
            true => BUFLM5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM5_A::_1
    }
}
#[doc = "Field `BUFLM5` writer - Buffer MB i Mask"]
pub type BUFLM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM5_A, O>;
impl<'a, const O: u8> BUFLM5_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM5_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM5_A::_1)
    }
}
#[doc = "Field `BUFLM6` reader - Buffer MB i Mask"]
pub type BUFLM6_R = crate::BitReader<BUFLM6_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM6_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM6_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM6_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM6_A {
        match self.bits {
            false => BUFLM6_A::_0,
            true => BUFLM6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM6_A::_1
    }
}
#[doc = "Field `BUFLM6` writer - Buffer MB i Mask"]
pub type BUFLM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM6_A, O>;
impl<'a, const O: u8> BUFLM6_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM6_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM6_A::_1)
    }
}
#[doc = "Field `BUFLM7` reader - Buffer MB i Mask"]
pub type BUFLM7_R = crate::BitReader<BUFLM7_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM7_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM7_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM7_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM7_A {
        match self.bits {
            false => BUFLM7_A::_0,
            true => BUFLM7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM7_A::_1
    }
}
#[doc = "Field `BUFLM7` writer - Buffer MB i Mask"]
pub type BUFLM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM7_A, O>;
impl<'a, const O: u8> BUFLM7_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM7_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM7_A::_1)
    }
}
#[doc = "Field `BUFLM8` reader - Buffer MB i Mask"]
pub type BUFLM8_R = crate::BitReader<BUFLM8_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM8_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM8_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM8_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM8_A {
        match self.bits {
            false => BUFLM8_A::_0,
            true => BUFLM8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM8_A::_1
    }
}
#[doc = "Field `BUFLM8` writer - Buffer MB i Mask"]
pub type BUFLM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM8_A, O>;
impl<'a, const O: u8> BUFLM8_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM8_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM8_A::_1)
    }
}
#[doc = "Field `BUFLM9` reader - Buffer MB i Mask"]
pub type BUFLM9_R = crate::BitReader<BUFLM9_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM9_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM9_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM9_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM9_A {
        match self.bits {
            false => BUFLM9_A::_0,
            true => BUFLM9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM9_A::_1
    }
}
#[doc = "Field `BUFLM9` writer - Buffer MB i Mask"]
pub type BUFLM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM9_A, O>;
impl<'a, const O: u8> BUFLM9_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM9_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM9_A::_1)
    }
}
#[doc = "Field `BUFLM10` reader - Buffer MB i Mask"]
pub type BUFLM10_R = crate::BitReader<BUFLM10_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM10_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM10_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM10_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM10_A {
        match self.bits {
            false => BUFLM10_A::_0,
            true => BUFLM10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM10_A::_1
    }
}
#[doc = "Field `BUFLM10` writer - Buffer MB i Mask"]
pub type BUFLM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM10_A, O>;
impl<'a, const O: u8> BUFLM10_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM10_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM10_A::_1)
    }
}
#[doc = "Field `BUFLM11` reader - Buffer MB i Mask"]
pub type BUFLM11_R = crate::BitReader<BUFLM11_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM11_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM11_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM11_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM11_A {
        match self.bits {
            false => BUFLM11_A::_0,
            true => BUFLM11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM11_A::_1
    }
}
#[doc = "Field `BUFLM11` writer - Buffer MB i Mask"]
pub type BUFLM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM11_A, O>;
impl<'a, const O: u8> BUFLM11_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM11_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM11_A::_1)
    }
}
#[doc = "Field `BUFLM12` reader - Buffer MB i Mask"]
pub type BUFLM12_R = crate::BitReader<BUFLM12_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM12_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM12_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM12_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM12_A {
        match self.bits {
            false => BUFLM12_A::_0,
            true => BUFLM12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM12_A::_1
    }
}
#[doc = "Field `BUFLM12` writer - Buffer MB i Mask"]
pub type BUFLM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM12_A, O>;
impl<'a, const O: u8> BUFLM12_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM12_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM12_A::_1)
    }
}
#[doc = "Field `BUFLM13` reader - Buffer MB i Mask"]
pub type BUFLM13_R = crate::BitReader<BUFLM13_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM13_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM13_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM13_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM13_A {
        match self.bits {
            false => BUFLM13_A::_0,
            true => BUFLM13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM13_A::_1
    }
}
#[doc = "Field `BUFLM13` writer - Buffer MB i Mask"]
pub type BUFLM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM13_A, O>;
impl<'a, const O: u8> BUFLM13_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM13_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM13_A::_1)
    }
}
#[doc = "Field `BUFLM14` reader - Buffer MB i Mask"]
pub type BUFLM14_R = crate::BitReader<BUFLM14_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM14_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM14_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM14_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM14_A {
        match self.bits {
            false => BUFLM14_A::_0,
            true => BUFLM14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM14_A::_1
    }
}
#[doc = "Field `BUFLM14` writer - Buffer MB i Mask"]
pub type BUFLM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM14_A, O>;
impl<'a, const O: u8> BUFLM14_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM14_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM14_A::_1)
    }
}
#[doc = "Field `BUFLM15` reader - Buffer MB i Mask"]
pub type BUFLM15_R = crate::BitReader<BUFLM15_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM15_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM15_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM15_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM15_A {
        match self.bits {
            false => BUFLM15_A::_0,
            true => BUFLM15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM15_A::_1
    }
}
#[doc = "Field `BUFLM15` writer - Buffer MB i Mask"]
pub type BUFLM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM15_A, O>;
impl<'a, const O: u8> BUFLM15_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM15_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM15_A::_1)
    }
}
#[doc = "Field `BUFLM16` reader - Buffer MB i Mask"]
pub type BUFLM16_R = crate::BitReader<BUFLM16_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM16_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM16_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM16_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM16_A {
        match self.bits {
            false => BUFLM16_A::_0,
            true => BUFLM16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM16_A::_1
    }
}
#[doc = "Field `BUFLM16` writer - Buffer MB i Mask"]
pub type BUFLM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM16_A, O>;
impl<'a, const O: u8> BUFLM16_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM16_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM16_A::_1)
    }
}
#[doc = "Field `BUFLM17` reader - Buffer MB i Mask"]
pub type BUFLM17_R = crate::BitReader<BUFLM17_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM17_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM17_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM17_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM17_A {
        match self.bits {
            false => BUFLM17_A::_0,
            true => BUFLM17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM17_A::_1
    }
}
#[doc = "Field `BUFLM17` writer - Buffer MB i Mask"]
pub type BUFLM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM17_A, O>;
impl<'a, const O: u8> BUFLM17_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM17_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM17_A::_1)
    }
}
#[doc = "Field `BUFLM18` reader - Buffer MB i Mask"]
pub type BUFLM18_R = crate::BitReader<BUFLM18_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM18_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM18_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM18_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM18_A {
        match self.bits {
            false => BUFLM18_A::_0,
            true => BUFLM18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM18_A::_1
    }
}
#[doc = "Field `BUFLM18` writer - Buffer MB i Mask"]
pub type BUFLM18_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM18_A, O>;
impl<'a, const O: u8> BUFLM18_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM18_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM18_A::_1)
    }
}
#[doc = "Field `BUFLM19` reader - Buffer MB i Mask"]
pub type BUFLM19_R = crate::BitReader<BUFLM19_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM19_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM19_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM19_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM19_A {
        match self.bits {
            false => BUFLM19_A::_0,
            true => BUFLM19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM19_A::_1
    }
}
#[doc = "Field `BUFLM19` writer - Buffer MB i Mask"]
pub type BUFLM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM19_A, O>;
impl<'a, const O: u8> BUFLM19_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM19_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM19_A::_1)
    }
}
#[doc = "Field `BUFLM20` reader - Buffer MB i Mask"]
pub type BUFLM20_R = crate::BitReader<BUFLM20_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM20_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM20_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM20_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM20_A {
        match self.bits {
            false => BUFLM20_A::_0,
            true => BUFLM20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM20_A::_1
    }
}
#[doc = "Field `BUFLM20` writer - Buffer MB i Mask"]
pub type BUFLM20_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM20_A, O>;
impl<'a, const O: u8> BUFLM20_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM20_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM20_A::_1)
    }
}
#[doc = "Field `BUFLM21` reader - Buffer MB i Mask"]
pub type BUFLM21_R = crate::BitReader<BUFLM21_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM21_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM21_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM21_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM21_A {
        match self.bits {
            false => BUFLM21_A::_0,
            true => BUFLM21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM21_A::_1
    }
}
#[doc = "Field `BUFLM21` writer - Buffer MB i Mask"]
pub type BUFLM21_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM21_A, O>;
impl<'a, const O: u8> BUFLM21_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM21_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM21_A::_1)
    }
}
#[doc = "Field `BUFLM22` reader - Buffer MB i Mask"]
pub type BUFLM22_R = crate::BitReader<BUFLM22_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM22_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM22_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM22_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM22_A {
        match self.bits {
            false => BUFLM22_A::_0,
            true => BUFLM22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM22_A::_1
    }
}
#[doc = "Field `BUFLM22` writer - Buffer MB i Mask"]
pub type BUFLM22_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM22_A, O>;
impl<'a, const O: u8> BUFLM22_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM22_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM22_A::_1)
    }
}
#[doc = "Field `BUFLM23` reader - Buffer MB i Mask"]
pub type BUFLM23_R = crate::BitReader<BUFLM23_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM23_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM23_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM23_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM23_A {
        match self.bits {
            false => BUFLM23_A::_0,
            true => BUFLM23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM23_A::_1
    }
}
#[doc = "Field `BUFLM23` writer - Buffer MB i Mask"]
pub type BUFLM23_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM23_A, O>;
impl<'a, const O: u8> BUFLM23_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM23_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM23_A::_1)
    }
}
#[doc = "Field `BUFLM24` reader - Buffer MB i Mask"]
pub type BUFLM24_R = crate::BitReader<BUFLM24_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM24_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM24_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM24_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM24_A {
        match self.bits {
            false => BUFLM24_A::_0,
            true => BUFLM24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM24_A::_1
    }
}
#[doc = "Field `BUFLM24` writer - Buffer MB i Mask"]
pub type BUFLM24_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM24_A, O>;
impl<'a, const O: u8> BUFLM24_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM24_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM24_A::_1)
    }
}
#[doc = "Field `BUFLM25` reader - Buffer MB i Mask"]
pub type BUFLM25_R = crate::BitReader<BUFLM25_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM25_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM25_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM25_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM25_A {
        match self.bits {
            false => BUFLM25_A::_0,
            true => BUFLM25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM25_A::_1
    }
}
#[doc = "Field `BUFLM25` writer - Buffer MB i Mask"]
pub type BUFLM25_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM25_A, O>;
impl<'a, const O: u8> BUFLM25_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM25_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM25_A::_1)
    }
}
#[doc = "Field `BUFLM26` reader - Buffer MB i Mask"]
pub type BUFLM26_R = crate::BitReader<BUFLM26_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM26_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM26_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM26_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM26_A {
        match self.bits {
            false => BUFLM26_A::_0,
            true => BUFLM26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM26_A::_1
    }
}
#[doc = "Field `BUFLM26` writer - Buffer MB i Mask"]
pub type BUFLM26_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM26_A, O>;
impl<'a, const O: u8> BUFLM26_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM26_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM26_A::_1)
    }
}
#[doc = "Field `BUFLM27` reader - Buffer MB i Mask"]
pub type BUFLM27_R = crate::BitReader<BUFLM27_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM27_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM27_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM27_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM27_A {
        match self.bits {
            false => BUFLM27_A::_0,
            true => BUFLM27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM27_A::_1
    }
}
#[doc = "Field `BUFLM27` writer - Buffer MB i Mask"]
pub type BUFLM27_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM27_A, O>;
impl<'a, const O: u8> BUFLM27_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM27_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM27_A::_1)
    }
}
#[doc = "Field `BUFLM28` reader - Buffer MB i Mask"]
pub type BUFLM28_R = crate::BitReader<BUFLM28_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM28_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM28_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM28_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM28_A {
        match self.bits {
            false => BUFLM28_A::_0,
            true => BUFLM28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM28_A::_1
    }
}
#[doc = "Field `BUFLM28` writer - Buffer MB i Mask"]
pub type BUFLM28_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM28_A, O>;
impl<'a, const O: u8> BUFLM28_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM28_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM28_A::_1)
    }
}
#[doc = "Field `BUFLM29` reader - Buffer MB i Mask"]
pub type BUFLM29_R = crate::BitReader<BUFLM29_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM29_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM29_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM29_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM29_A {
        match self.bits {
            false => BUFLM29_A::_0,
            true => BUFLM29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM29_A::_1
    }
}
#[doc = "Field `BUFLM29` writer - Buffer MB i Mask"]
pub type BUFLM29_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM29_A, O>;
impl<'a, const O: u8> BUFLM29_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM29_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM29_A::_1)
    }
}
#[doc = "Field `BUFLM30` reader - Buffer MB i Mask"]
pub type BUFLM30_R = crate::BitReader<BUFLM30_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM30_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM30_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM30_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM30_A {
        match self.bits {
            false => BUFLM30_A::_0,
            true => BUFLM30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM30_A::_1
    }
}
#[doc = "Field `BUFLM30` writer - Buffer MB i Mask"]
pub type BUFLM30_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM30_A, O>;
impl<'a, const O: u8> BUFLM30_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM30_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM30_A::_1)
    }
}
#[doc = "Field `BUFLM31` reader - Buffer MB i Mask"]
pub type BUFLM31_R = crate::BitReader<BUFLM31_A>;
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFLM31_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM31_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM31_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFLM31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM31_A {
        match self.bits {
            false => BUFLM31_A::_0,
            true => BUFLM31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM31_A::_1
    }
}
#[doc = "Field `BUFLM31` writer - Buffer MB i Mask"]
pub type BUFLM31_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK1_SPEC, BUFLM31_A, O>;
impl<'a, const O: u8> BUFLM31_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM31_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm0(&self) -> BUFLM0_R {
        BUFLM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm1(&self) -> BUFLM1_R {
        BUFLM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm2(&self) -> BUFLM2_R {
        BUFLM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm3(&self) -> BUFLM3_R {
        BUFLM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm4(&self) -> BUFLM4_R {
        BUFLM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm5(&self) -> BUFLM5_R {
        BUFLM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm6(&self) -> BUFLM6_R {
        BUFLM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm7(&self) -> BUFLM7_R {
        BUFLM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm8(&self) -> BUFLM8_R {
        BUFLM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm9(&self) -> BUFLM9_R {
        BUFLM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm10(&self) -> BUFLM10_R {
        BUFLM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm11(&self) -> BUFLM11_R {
        BUFLM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm12(&self) -> BUFLM12_R {
        BUFLM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm13(&self) -> BUFLM13_R {
        BUFLM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm14(&self) -> BUFLM14_R {
        BUFLM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm15(&self) -> BUFLM15_R {
        BUFLM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm16(&self) -> BUFLM16_R {
        BUFLM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm17(&self) -> BUFLM17_R {
        BUFLM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm18(&self) -> BUFLM18_R {
        BUFLM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm19(&self) -> BUFLM19_R {
        BUFLM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm20(&self) -> BUFLM20_R {
        BUFLM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm21(&self) -> BUFLM21_R {
        BUFLM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm22(&self) -> BUFLM22_R {
        BUFLM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm23(&self) -> BUFLM23_R {
        BUFLM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm24(&self) -> BUFLM24_R {
        BUFLM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm25(&self) -> BUFLM25_R {
        BUFLM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm26(&self) -> BUFLM26_R {
        BUFLM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm27(&self) -> BUFLM27_R {
        BUFLM27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm28(&self) -> BUFLM28_R {
        BUFLM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm29(&self) -> BUFLM29_R {
        BUFLM29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm30(&self) -> BUFLM30_R {
        BUFLM30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm31(&self) -> BUFLM31_R {
        BUFLM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm0(&mut self) -> BUFLM0_W<0> {
        BUFLM0_W::new(self)
    }
    #[doc = "Bit 1 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm1(&mut self) -> BUFLM1_W<1> {
        BUFLM1_W::new(self)
    }
    #[doc = "Bit 2 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm2(&mut self) -> BUFLM2_W<2> {
        BUFLM2_W::new(self)
    }
    #[doc = "Bit 3 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm3(&mut self) -> BUFLM3_W<3> {
        BUFLM3_W::new(self)
    }
    #[doc = "Bit 4 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm4(&mut self) -> BUFLM4_W<4> {
        BUFLM4_W::new(self)
    }
    #[doc = "Bit 5 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm5(&mut self) -> BUFLM5_W<5> {
        BUFLM5_W::new(self)
    }
    #[doc = "Bit 6 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm6(&mut self) -> BUFLM6_W<6> {
        BUFLM6_W::new(self)
    }
    #[doc = "Bit 7 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm7(&mut self) -> BUFLM7_W<7> {
        BUFLM7_W::new(self)
    }
    #[doc = "Bit 8 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm8(&mut self) -> BUFLM8_W<8> {
        BUFLM8_W::new(self)
    }
    #[doc = "Bit 9 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm9(&mut self) -> BUFLM9_W<9> {
        BUFLM9_W::new(self)
    }
    #[doc = "Bit 10 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm10(&mut self) -> BUFLM10_W<10> {
        BUFLM10_W::new(self)
    }
    #[doc = "Bit 11 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm11(&mut self) -> BUFLM11_W<11> {
        BUFLM11_W::new(self)
    }
    #[doc = "Bit 12 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm12(&mut self) -> BUFLM12_W<12> {
        BUFLM12_W::new(self)
    }
    #[doc = "Bit 13 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm13(&mut self) -> BUFLM13_W<13> {
        BUFLM13_W::new(self)
    }
    #[doc = "Bit 14 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm14(&mut self) -> BUFLM14_W<14> {
        BUFLM14_W::new(self)
    }
    #[doc = "Bit 15 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm15(&mut self) -> BUFLM15_W<15> {
        BUFLM15_W::new(self)
    }
    #[doc = "Bit 16 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm16(&mut self) -> BUFLM16_W<16> {
        BUFLM16_W::new(self)
    }
    #[doc = "Bit 17 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm17(&mut self) -> BUFLM17_W<17> {
        BUFLM17_W::new(self)
    }
    #[doc = "Bit 18 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm18(&mut self) -> BUFLM18_W<18> {
        BUFLM18_W::new(self)
    }
    #[doc = "Bit 19 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm19(&mut self) -> BUFLM19_W<19> {
        BUFLM19_W::new(self)
    }
    #[doc = "Bit 20 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm20(&mut self) -> BUFLM20_W<20> {
        BUFLM20_W::new(self)
    }
    #[doc = "Bit 21 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm21(&mut self) -> BUFLM21_W<21> {
        BUFLM21_W::new(self)
    }
    #[doc = "Bit 22 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm22(&mut self) -> BUFLM22_W<22> {
        BUFLM22_W::new(self)
    }
    #[doc = "Bit 23 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm23(&mut self) -> BUFLM23_W<23> {
        BUFLM23_W::new(self)
    }
    #[doc = "Bit 24 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm24(&mut self) -> BUFLM24_W<24> {
        BUFLM24_W::new(self)
    }
    #[doc = "Bit 25 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm25(&mut self) -> BUFLM25_W<25> {
        BUFLM25_W::new(self)
    }
    #[doc = "Bit 26 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm26(&mut self) -> BUFLM26_W<26> {
        BUFLM26_W::new(self)
    }
    #[doc = "Bit 27 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm27(&mut self) -> BUFLM27_W<27> {
        BUFLM27_W::new(self)
    }
    #[doc = "Bit 28 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm28(&mut self) -> BUFLM28_W<28> {
        BUFLM28_W::new(self)
    }
    #[doc = "Bit 29 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm29(&mut self) -> BUFLM29_W<29> {
        BUFLM29_W::new(self)
    }
    #[doc = "Bit 30 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm30(&mut self) -> BUFLM30_W<30> {
        BUFLM30_W::new(self)
    }
    #[doc = "Bit 31 - Buffer MB i Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm31(&mut self) -> BUFLM31_W<31> {
        BUFLM31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Masks 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imask1](index.html) module"]
pub struct IMASK1_SPEC;
impl crate::RegisterSpec for IMASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imask1::R](R) reader structure"]
impl crate::Readable for IMASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imask1::W](W) writer structure"]
impl crate::Writable for IMASK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMASK1 to value 0"]
impl crate::Resettable for IMASK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

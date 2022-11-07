#[doc = "Register `TMR` reader"]
pub struct R(crate::R<TMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR` writer"]
pub struct W(crate::W<TMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR_SPEC>;
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
impl From<crate::W<TMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWM0` reader - Transmit Word Mask"]
pub type TWM0_R = crate::BitReader<TWM0_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM0_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM0_A> for bool {
    #[inline(always)]
    fn from(variant: TWM0_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM0_A {
        match self.bits {
            false => TWM0_A::_0,
            true => TWM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM0_A::_1
    }
}
#[doc = "Field `TWM0` writer - Transmit Word Mask"]
pub type TWM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM0_A, O>;
impl<'a, const O: u8> TWM0_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM0_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM0_A::_1)
    }
}
#[doc = "Field `TWM1` reader - Transmit Word Mask"]
pub type TWM1_R = crate::BitReader<TWM1_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM1_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM1_A> for bool {
    #[inline(always)]
    fn from(variant: TWM1_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM1_A {
        match self.bits {
            false => TWM1_A::_0,
            true => TWM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM1_A::_1
    }
}
#[doc = "Field `TWM1` writer - Transmit Word Mask"]
pub type TWM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM1_A, O>;
impl<'a, const O: u8> TWM1_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM1_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM1_A::_1)
    }
}
#[doc = "Field `TWM2` reader - Transmit Word Mask"]
pub type TWM2_R = crate::BitReader<TWM2_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM2_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM2_A> for bool {
    #[inline(always)]
    fn from(variant: TWM2_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM2_A {
        match self.bits {
            false => TWM2_A::_0,
            true => TWM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM2_A::_1
    }
}
#[doc = "Field `TWM2` writer - Transmit Word Mask"]
pub type TWM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM2_A, O>;
impl<'a, const O: u8> TWM2_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM2_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM2_A::_1)
    }
}
#[doc = "Field `TWM3` reader - Transmit Word Mask"]
pub type TWM3_R = crate::BitReader<TWM3_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM3_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM3_A> for bool {
    #[inline(always)]
    fn from(variant: TWM3_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM3_A {
        match self.bits {
            false => TWM3_A::_0,
            true => TWM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM3_A::_1
    }
}
#[doc = "Field `TWM3` writer - Transmit Word Mask"]
pub type TWM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM3_A, O>;
impl<'a, const O: u8> TWM3_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM3_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM3_A::_1)
    }
}
#[doc = "Field `TWM4` reader - Transmit Word Mask"]
pub type TWM4_R = crate::BitReader<TWM4_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM4_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM4_A> for bool {
    #[inline(always)]
    fn from(variant: TWM4_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM4_A {
        match self.bits {
            false => TWM4_A::_0,
            true => TWM4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM4_A::_1
    }
}
#[doc = "Field `TWM4` writer - Transmit Word Mask"]
pub type TWM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM4_A, O>;
impl<'a, const O: u8> TWM4_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM4_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM4_A::_1)
    }
}
#[doc = "Field `TWM5` reader - Transmit Word Mask"]
pub type TWM5_R = crate::BitReader<TWM5_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM5_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM5_A> for bool {
    #[inline(always)]
    fn from(variant: TWM5_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM5_A {
        match self.bits {
            false => TWM5_A::_0,
            true => TWM5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM5_A::_1
    }
}
#[doc = "Field `TWM5` writer - Transmit Word Mask"]
pub type TWM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM5_A, O>;
impl<'a, const O: u8> TWM5_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM5_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM5_A::_1)
    }
}
#[doc = "Field `TWM6` reader - Transmit Word Mask"]
pub type TWM6_R = crate::BitReader<TWM6_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM6_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM6_A> for bool {
    #[inline(always)]
    fn from(variant: TWM6_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM6_A {
        match self.bits {
            false => TWM6_A::_0,
            true => TWM6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM6_A::_1
    }
}
#[doc = "Field `TWM6` writer - Transmit Word Mask"]
pub type TWM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM6_A, O>;
impl<'a, const O: u8> TWM6_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM6_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM6_A::_1)
    }
}
#[doc = "Field `TWM7` reader - Transmit Word Mask"]
pub type TWM7_R = crate::BitReader<TWM7_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM7_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM7_A> for bool {
    #[inline(always)]
    fn from(variant: TWM7_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM7_A {
        match self.bits {
            false => TWM7_A::_0,
            true => TWM7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM7_A::_1
    }
}
#[doc = "Field `TWM7` writer - Transmit Word Mask"]
pub type TWM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM7_A, O>;
impl<'a, const O: u8> TWM7_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM7_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM7_A::_1)
    }
}
#[doc = "Field `TWM8` reader - Transmit Word Mask"]
pub type TWM8_R = crate::BitReader<TWM8_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM8_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM8_A> for bool {
    #[inline(always)]
    fn from(variant: TWM8_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM8_A {
        match self.bits {
            false => TWM8_A::_0,
            true => TWM8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM8_A::_1
    }
}
#[doc = "Field `TWM8` writer - Transmit Word Mask"]
pub type TWM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM8_A, O>;
impl<'a, const O: u8> TWM8_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM8_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM8_A::_1)
    }
}
#[doc = "Field `TWM9` reader - Transmit Word Mask"]
pub type TWM9_R = crate::BitReader<TWM9_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM9_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM9_A> for bool {
    #[inline(always)]
    fn from(variant: TWM9_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM9_A {
        match self.bits {
            false => TWM9_A::_0,
            true => TWM9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM9_A::_1
    }
}
#[doc = "Field `TWM9` writer - Transmit Word Mask"]
pub type TWM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM9_A, O>;
impl<'a, const O: u8> TWM9_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM9_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM9_A::_1)
    }
}
#[doc = "Field `TWM10` reader - Transmit Word Mask"]
pub type TWM10_R = crate::BitReader<TWM10_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM10_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM10_A> for bool {
    #[inline(always)]
    fn from(variant: TWM10_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM10_A {
        match self.bits {
            false => TWM10_A::_0,
            true => TWM10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM10_A::_1
    }
}
#[doc = "Field `TWM10` writer - Transmit Word Mask"]
pub type TWM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM10_A, O>;
impl<'a, const O: u8> TWM10_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM10_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM10_A::_1)
    }
}
#[doc = "Field `TWM11` reader - Transmit Word Mask"]
pub type TWM11_R = crate::BitReader<TWM11_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM11_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM11_A> for bool {
    #[inline(always)]
    fn from(variant: TWM11_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM11_A {
        match self.bits {
            false => TWM11_A::_0,
            true => TWM11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM11_A::_1
    }
}
#[doc = "Field `TWM11` writer - Transmit Word Mask"]
pub type TWM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM11_A, O>;
impl<'a, const O: u8> TWM11_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM11_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM11_A::_1)
    }
}
#[doc = "Field `TWM12` reader - Transmit Word Mask"]
pub type TWM12_R = crate::BitReader<TWM12_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM12_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM12_A> for bool {
    #[inline(always)]
    fn from(variant: TWM12_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM12_A {
        match self.bits {
            false => TWM12_A::_0,
            true => TWM12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM12_A::_1
    }
}
#[doc = "Field `TWM12` writer - Transmit Word Mask"]
pub type TWM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM12_A, O>;
impl<'a, const O: u8> TWM12_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM12_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM12_A::_1)
    }
}
#[doc = "Field `TWM13` reader - Transmit Word Mask"]
pub type TWM13_R = crate::BitReader<TWM13_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM13_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM13_A> for bool {
    #[inline(always)]
    fn from(variant: TWM13_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM13_A {
        match self.bits {
            false => TWM13_A::_0,
            true => TWM13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM13_A::_1
    }
}
#[doc = "Field `TWM13` writer - Transmit Word Mask"]
pub type TWM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM13_A, O>;
impl<'a, const O: u8> TWM13_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM13_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM13_A::_1)
    }
}
#[doc = "Field `TWM14` reader - Transmit Word Mask"]
pub type TWM14_R = crate::BitReader<TWM14_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM14_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM14_A> for bool {
    #[inline(always)]
    fn from(variant: TWM14_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM14_A {
        match self.bits {
            false => TWM14_A::_0,
            true => TWM14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM14_A::_1
    }
}
#[doc = "Field `TWM14` writer - Transmit Word Mask"]
pub type TWM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM14_A, O>;
impl<'a, const O: u8> TWM14_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM14_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM14_A::_1)
    }
}
#[doc = "Field `TWM15` reader - Transmit Word Mask"]
pub type TWM15_R = crate::BitReader<TWM15_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM15_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM15_A> for bool {
    #[inline(always)]
    fn from(variant: TWM15_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM15_A {
        match self.bits {
            false => TWM15_A::_0,
            true => TWM15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM15_A::_1
    }
}
#[doc = "Field `TWM15` writer - Transmit Word Mask"]
pub type TWM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM15_A, O>;
impl<'a, const O: u8> TWM15_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM15_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM15_A::_1)
    }
}
#[doc = "Field `TWM16` reader - Transmit Word Mask"]
pub type TWM16_R = crate::BitReader<TWM16_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM16_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM16_A> for bool {
    #[inline(always)]
    fn from(variant: TWM16_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM16_A {
        match self.bits {
            false => TWM16_A::_0,
            true => TWM16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM16_A::_1
    }
}
#[doc = "Field `TWM16` writer - Transmit Word Mask"]
pub type TWM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM16_A, O>;
impl<'a, const O: u8> TWM16_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM16_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM16_A::_1)
    }
}
#[doc = "Field `TWM17` reader - Transmit Word Mask"]
pub type TWM17_R = crate::BitReader<TWM17_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM17_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM17_A> for bool {
    #[inline(always)]
    fn from(variant: TWM17_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM17_A {
        match self.bits {
            false => TWM17_A::_0,
            true => TWM17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM17_A::_1
    }
}
#[doc = "Field `TWM17` writer - Transmit Word Mask"]
pub type TWM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM17_A, O>;
impl<'a, const O: u8> TWM17_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM17_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM17_A::_1)
    }
}
#[doc = "Field `TWM18` reader - Transmit Word Mask"]
pub type TWM18_R = crate::BitReader<TWM18_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM18_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM18_A> for bool {
    #[inline(always)]
    fn from(variant: TWM18_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM18_A {
        match self.bits {
            false => TWM18_A::_0,
            true => TWM18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM18_A::_1
    }
}
#[doc = "Field `TWM18` writer - Transmit Word Mask"]
pub type TWM18_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM18_A, O>;
impl<'a, const O: u8> TWM18_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM18_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM18_A::_1)
    }
}
#[doc = "Field `TWM19` reader - Transmit Word Mask"]
pub type TWM19_R = crate::BitReader<TWM19_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM19_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM19_A> for bool {
    #[inline(always)]
    fn from(variant: TWM19_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM19_A {
        match self.bits {
            false => TWM19_A::_0,
            true => TWM19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM19_A::_1
    }
}
#[doc = "Field `TWM19` writer - Transmit Word Mask"]
pub type TWM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM19_A, O>;
impl<'a, const O: u8> TWM19_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM19_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM19_A::_1)
    }
}
#[doc = "Field `TWM20` reader - Transmit Word Mask"]
pub type TWM20_R = crate::BitReader<TWM20_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM20_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM20_A> for bool {
    #[inline(always)]
    fn from(variant: TWM20_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM20_A {
        match self.bits {
            false => TWM20_A::_0,
            true => TWM20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM20_A::_1
    }
}
#[doc = "Field `TWM20` writer - Transmit Word Mask"]
pub type TWM20_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM20_A, O>;
impl<'a, const O: u8> TWM20_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM20_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM20_A::_1)
    }
}
#[doc = "Field `TWM21` reader - Transmit Word Mask"]
pub type TWM21_R = crate::BitReader<TWM21_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM21_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM21_A> for bool {
    #[inline(always)]
    fn from(variant: TWM21_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM21_A {
        match self.bits {
            false => TWM21_A::_0,
            true => TWM21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM21_A::_1
    }
}
#[doc = "Field `TWM21` writer - Transmit Word Mask"]
pub type TWM21_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM21_A, O>;
impl<'a, const O: u8> TWM21_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM21_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM21_A::_1)
    }
}
#[doc = "Field `TWM22` reader - Transmit Word Mask"]
pub type TWM22_R = crate::BitReader<TWM22_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM22_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM22_A> for bool {
    #[inline(always)]
    fn from(variant: TWM22_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM22_A {
        match self.bits {
            false => TWM22_A::_0,
            true => TWM22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM22_A::_1
    }
}
#[doc = "Field `TWM22` writer - Transmit Word Mask"]
pub type TWM22_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM22_A, O>;
impl<'a, const O: u8> TWM22_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM22_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM22_A::_1)
    }
}
#[doc = "Field `TWM23` reader - Transmit Word Mask"]
pub type TWM23_R = crate::BitReader<TWM23_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM23_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM23_A> for bool {
    #[inline(always)]
    fn from(variant: TWM23_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM23_A {
        match self.bits {
            false => TWM23_A::_0,
            true => TWM23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM23_A::_1
    }
}
#[doc = "Field `TWM23` writer - Transmit Word Mask"]
pub type TWM23_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM23_A, O>;
impl<'a, const O: u8> TWM23_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM23_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM23_A::_1)
    }
}
#[doc = "Field `TWM24` reader - Transmit Word Mask"]
pub type TWM24_R = crate::BitReader<TWM24_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM24_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM24_A> for bool {
    #[inline(always)]
    fn from(variant: TWM24_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM24_A {
        match self.bits {
            false => TWM24_A::_0,
            true => TWM24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM24_A::_1
    }
}
#[doc = "Field `TWM24` writer - Transmit Word Mask"]
pub type TWM24_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM24_A, O>;
impl<'a, const O: u8> TWM24_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM24_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM24_A::_1)
    }
}
#[doc = "Field `TWM25` reader - Transmit Word Mask"]
pub type TWM25_R = crate::BitReader<TWM25_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM25_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM25_A> for bool {
    #[inline(always)]
    fn from(variant: TWM25_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM25_A {
        match self.bits {
            false => TWM25_A::_0,
            true => TWM25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM25_A::_1
    }
}
#[doc = "Field `TWM25` writer - Transmit Word Mask"]
pub type TWM25_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM25_A, O>;
impl<'a, const O: u8> TWM25_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM25_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM25_A::_1)
    }
}
#[doc = "Field `TWM26` reader - Transmit Word Mask"]
pub type TWM26_R = crate::BitReader<TWM26_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM26_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM26_A> for bool {
    #[inline(always)]
    fn from(variant: TWM26_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM26_A {
        match self.bits {
            false => TWM26_A::_0,
            true => TWM26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM26_A::_1
    }
}
#[doc = "Field `TWM26` writer - Transmit Word Mask"]
pub type TWM26_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM26_A, O>;
impl<'a, const O: u8> TWM26_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM26_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM26_A::_1)
    }
}
#[doc = "Field `TWM27` reader - Transmit Word Mask"]
pub type TWM27_R = crate::BitReader<TWM27_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM27_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM27_A> for bool {
    #[inline(always)]
    fn from(variant: TWM27_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM27_A {
        match self.bits {
            false => TWM27_A::_0,
            true => TWM27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM27_A::_1
    }
}
#[doc = "Field `TWM27` writer - Transmit Word Mask"]
pub type TWM27_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM27_A, O>;
impl<'a, const O: u8> TWM27_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM27_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM27_A::_1)
    }
}
#[doc = "Field `TWM28` reader - Transmit Word Mask"]
pub type TWM28_R = crate::BitReader<TWM28_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM28_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM28_A> for bool {
    #[inline(always)]
    fn from(variant: TWM28_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM28_A {
        match self.bits {
            false => TWM28_A::_0,
            true => TWM28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM28_A::_1
    }
}
#[doc = "Field `TWM28` writer - Transmit Word Mask"]
pub type TWM28_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM28_A, O>;
impl<'a, const O: u8> TWM28_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM28_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM28_A::_1)
    }
}
#[doc = "Field `TWM29` reader - Transmit Word Mask"]
pub type TWM29_R = crate::BitReader<TWM29_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM29_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM29_A> for bool {
    #[inline(always)]
    fn from(variant: TWM29_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM29_A {
        match self.bits {
            false => TWM29_A::_0,
            true => TWM29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM29_A::_1
    }
}
#[doc = "Field `TWM29` writer - Transmit Word Mask"]
pub type TWM29_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM29_A, O>;
impl<'a, const O: u8> TWM29_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM29_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM29_A::_1)
    }
}
#[doc = "Field `TWM30` reader - Transmit Word Mask"]
pub type TWM30_R = crate::BitReader<TWM30_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM30_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM30_A> for bool {
    #[inline(always)]
    fn from(variant: TWM30_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM30_A {
        match self.bits {
            false => TWM30_A::_0,
            true => TWM30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM30_A::_1
    }
}
#[doc = "Field `TWM30` writer - Transmit Word Mask"]
pub type TWM30_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM30_A, O>;
impl<'a, const O: u8> TWM30_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM30_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM30_A::_1)
    }
}
#[doc = "Field `TWM31` reader - Transmit Word Mask"]
pub type TWM31_R = crate::BitReader<TWM31_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWM31_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1 = 1,
}
impl From<TWM31_A> for bool {
    #[inline(always)]
    fn from(variant: TWM31_A) -> Self {
        variant as u8 != 0
    }
}
impl TWM31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM31_A {
        match self.bits {
            false => TWM31_A::_0,
            true => TWM31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM31_A::_1
    }
}
#[doc = "Field `TWM31` writer - Transmit Word Mask"]
pub type TWM31_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_SPEC, TWM31_A, O>;
impl<'a, const O: u8> TWM31_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM31_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm0(&self) -> TWM0_R {
        TWM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm1(&self) -> TWM1_R {
        TWM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm2(&self) -> TWM2_R {
        TWM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm3(&self) -> TWM3_R {
        TWM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm4(&self) -> TWM4_R {
        TWM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm5(&self) -> TWM5_R {
        TWM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm6(&self) -> TWM6_R {
        TWM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm7(&self) -> TWM7_R {
        TWM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm8(&self) -> TWM8_R {
        TWM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm9(&self) -> TWM9_R {
        TWM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm10(&self) -> TWM10_R {
        TWM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm11(&self) -> TWM11_R {
        TWM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm12(&self) -> TWM12_R {
        TWM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm13(&self) -> TWM13_R {
        TWM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm14(&self) -> TWM14_R {
        TWM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm15(&self) -> TWM15_R {
        TWM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm16(&self) -> TWM16_R {
        TWM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm17(&self) -> TWM17_R {
        TWM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm18(&self) -> TWM18_R {
        TWM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm19(&self) -> TWM19_R {
        TWM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm20(&self) -> TWM20_R {
        TWM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm21(&self) -> TWM21_R {
        TWM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm22(&self) -> TWM22_R {
        TWM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm23(&self) -> TWM23_R {
        TWM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm24(&self) -> TWM24_R {
        TWM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm25(&self) -> TWM25_R {
        TWM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm26(&self) -> TWM26_R {
        TWM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm27(&self) -> TWM27_R {
        TWM27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm28(&self) -> TWM28_R {
        TWM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm29(&self) -> TWM29_R {
        TWM29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm30(&self) -> TWM30_R {
        TWM30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm31(&self) -> TWM31_R {
        TWM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm0(&mut self) -> TWM0_W<0> {
        TWM0_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm1(&mut self) -> TWM1_W<1> {
        TWM1_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm2(&mut self) -> TWM2_W<2> {
        TWM2_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm3(&mut self) -> TWM3_W<3> {
        TWM3_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm4(&mut self) -> TWM4_W<4> {
        TWM4_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm5(&mut self) -> TWM5_W<5> {
        TWM5_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm6(&mut self) -> TWM6_W<6> {
        TWM6_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm7(&mut self) -> TWM7_W<7> {
        TWM7_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm8(&mut self) -> TWM8_W<8> {
        TWM8_W::new(self)
    }
    #[doc = "Bit 9 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm9(&mut self) -> TWM9_W<9> {
        TWM9_W::new(self)
    }
    #[doc = "Bit 10 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm10(&mut self) -> TWM10_W<10> {
        TWM10_W::new(self)
    }
    #[doc = "Bit 11 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm11(&mut self) -> TWM11_W<11> {
        TWM11_W::new(self)
    }
    #[doc = "Bit 12 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm12(&mut self) -> TWM12_W<12> {
        TWM12_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm13(&mut self) -> TWM13_W<13> {
        TWM13_W::new(self)
    }
    #[doc = "Bit 14 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm14(&mut self) -> TWM14_W<14> {
        TWM14_W::new(self)
    }
    #[doc = "Bit 15 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm15(&mut self) -> TWM15_W<15> {
        TWM15_W::new(self)
    }
    #[doc = "Bit 16 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm16(&mut self) -> TWM16_W<16> {
        TWM16_W::new(self)
    }
    #[doc = "Bit 17 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm17(&mut self) -> TWM17_W<17> {
        TWM17_W::new(self)
    }
    #[doc = "Bit 18 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm18(&mut self) -> TWM18_W<18> {
        TWM18_W::new(self)
    }
    #[doc = "Bit 19 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm19(&mut self) -> TWM19_W<19> {
        TWM19_W::new(self)
    }
    #[doc = "Bit 20 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm20(&mut self) -> TWM20_W<20> {
        TWM20_W::new(self)
    }
    #[doc = "Bit 21 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm21(&mut self) -> TWM21_W<21> {
        TWM21_W::new(self)
    }
    #[doc = "Bit 22 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm22(&mut self) -> TWM22_W<22> {
        TWM22_W::new(self)
    }
    #[doc = "Bit 23 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm23(&mut self) -> TWM23_W<23> {
        TWM23_W::new(self)
    }
    #[doc = "Bit 24 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm24(&mut self) -> TWM24_W<24> {
        TWM24_W::new(self)
    }
    #[doc = "Bit 25 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm25(&mut self) -> TWM25_W<25> {
        TWM25_W::new(self)
    }
    #[doc = "Bit 26 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm26(&mut self) -> TWM26_W<26> {
        TWM26_W::new(self)
    }
    #[doc = "Bit 27 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm27(&mut self) -> TWM27_W<27> {
        TWM27_W::new(self)
    }
    #[doc = "Bit 28 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm28(&mut self) -> TWM28_W<28> {
        TWM28_W::new(self)
    }
    #[doc = "Bit 29 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm29(&mut self) -> TWM29_W<29> {
        TWM29_W::new(self)
    }
    #[doc = "Bit 30 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm30(&mut self) -> TWM30_W<30> {
        TWM30_W::new(self)
    }
    #[doc = "Bit 31 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm31(&mut self) -> TWM31_W<31> {
        TWM31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Transmit Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr](index.html) module"]
pub struct TMR_SPEC;
impl crate::RegisterSpec for TMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr::R](R) reader structure"]
impl crate::Readable for TMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr::W](W) writer structure"]
impl crate::Writable for TMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMR to value 0"]
impl crate::Resettable for TMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

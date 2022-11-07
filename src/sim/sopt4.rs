#[doc = "Register `SOPT4` reader"]
pub struct R(crate::R<SOPT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT4` writer"]
pub struct W(crate::W<SOPT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT4_SPEC>;
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
impl From<crate::W<SOPT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTM0FLT0` reader - FTM0 Fault 0 Select"]
pub type FTM0FLT0_R = crate::BitReader<FTM0FLT0_A>;
#[doc = "FTM0 Fault 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0FLT0_A {
    #[doc = "0: FTM0_FLT0 pin"]
    _0 = 0,
    #[doc = "1: CMP0 out"]
    _1 = 1,
}
impl From<FTM0FLT0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0FLT0_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0FLT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0FLT0_A {
        match self.bits {
            false => FTM0FLT0_A::_0,
            true => FTM0FLT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0FLT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0FLT0_A::_1
    }
}
#[doc = "Field `FTM0FLT0` writer - FTM0 Fault 0 Select"]
pub type FTM0FLT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM0FLT0_A, O>;
impl<'a, const O: u8> FTM0FLT0_W<'a, O> {
    #[doc = "FTM0_FLT0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0FLT0_A::_0)
    }
    #[doc = "CMP0 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0FLT0_A::_1)
    }
}
#[doc = "Field `FTM0FLT1` reader - FTM0 Fault 1 Select"]
pub type FTM0FLT1_R = crate::BitReader<FTM0FLT1_A>;
#[doc = "FTM0 Fault 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0FLT1_A {
    #[doc = "0: FTM0_FLT1 pin"]
    _0 = 0,
    #[doc = "1: CMP1 out"]
    _1 = 1,
}
impl From<FTM0FLT1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0FLT1_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0FLT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0FLT1_A {
        match self.bits {
            false => FTM0FLT1_A::_0,
            true => FTM0FLT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0FLT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0FLT1_A::_1
    }
}
#[doc = "Field `FTM0FLT1` writer - FTM0 Fault 1 Select"]
pub type FTM0FLT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM0FLT1_A, O>;
impl<'a, const O: u8> FTM0FLT1_W<'a, O> {
    #[doc = "FTM0_FLT1 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0FLT1_A::_0)
    }
    #[doc = "CMP1 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0FLT1_A::_1)
    }
}
#[doc = "Field `FTM0FLT2` reader - FTM0 Fault 2 Select"]
pub type FTM0FLT2_R = crate::BitReader<FTM0FLT2_A>;
#[doc = "FTM0 Fault 2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0FLT2_A {
    #[doc = "0: FTM0_FLT2 pin"]
    _0 = 0,
    #[doc = "1: CMP2 out"]
    _1 = 1,
}
impl From<FTM0FLT2_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0FLT2_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0FLT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0FLT2_A {
        match self.bits {
            false => FTM0FLT2_A::_0,
            true => FTM0FLT2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0FLT2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0FLT2_A::_1
    }
}
#[doc = "Field `FTM0FLT2` writer - FTM0 Fault 2 Select"]
pub type FTM0FLT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM0FLT2_A, O>;
impl<'a, const O: u8> FTM0FLT2_W<'a, O> {
    #[doc = "FTM0_FLT2 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0FLT2_A::_0)
    }
    #[doc = "CMP2 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0FLT2_A::_1)
    }
}
#[doc = "Field `FTM1FLT0` reader - FTM1 Fault 0 Select"]
pub type FTM1FLT0_R = crate::BitReader<FTM1FLT0_A>;
#[doc = "FTM1 Fault 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM1FLT0_A {
    #[doc = "0: FTM1_FLT0 pin"]
    _0 = 0,
    #[doc = "1: CMP0 out"]
    _1 = 1,
}
impl From<FTM1FLT0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1FLT0_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM1FLT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1FLT0_A {
        match self.bits {
            false => FTM1FLT0_A::_0,
            true => FTM1FLT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1FLT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1FLT0_A::_1
    }
}
#[doc = "Field `FTM1FLT0` writer - FTM1 Fault 0 Select"]
pub type FTM1FLT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM1FLT0_A, O>;
impl<'a, const O: u8> FTM1FLT0_W<'a, O> {
    #[doc = "FTM1_FLT0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1FLT0_A::_0)
    }
    #[doc = "CMP0 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1FLT0_A::_1)
    }
}
#[doc = "Field `FTM2FLT0` reader - FTM2 Fault 0 Select"]
pub type FTM2FLT0_R = crate::BitReader<FTM2FLT0_A>;
#[doc = "FTM2 Fault 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM2FLT0_A {
    #[doc = "0: FTM2_FLT0 pin"]
    _0 = 0,
    #[doc = "1: CMP0 out"]
    _1 = 1,
}
impl From<FTM2FLT0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2FLT0_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM2FLT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2FLT0_A {
        match self.bits {
            false => FTM2FLT0_A::_0,
            true => FTM2FLT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2FLT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2FLT0_A::_1
    }
}
#[doc = "Field `FTM2FLT0` writer - FTM2 Fault 0 Select"]
pub type FTM2FLT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM2FLT0_A, O>;
impl<'a, const O: u8> FTM2FLT0_W<'a, O> {
    #[doc = "FTM2_FLT0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2FLT0_A::_0)
    }
    #[doc = "CMP0 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2FLT0_A::_1)
    }
}
#[doc = "Field `FTM3FLT0` reader - FTM3 Fault 0 Select"]
pub type FTM3FLT0_R = crate::BitReader<FTM3FLT0_A>;
#[doc = "FTM3 Fault 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM3FLT0_A {
    #[doc = "0: FTM3_FLT0 pin"]
    _0 = 0,
    #[doc = "1: CMP0 out"]
    _1 = 1,
}
impl From<FTM3FLT0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3FLT0_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM3FLT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3FLT0_A {
        match self.bits {
            false => FTM3FLT0_A::_0,
            true => FTM3FLT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3FLT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3FLT0_A::_1
    }
}
#[doc = "Field `FTM3FLT0` writer - FTM3 Fault 0 Select"]
pub type FTM3FLT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM3FLT0_A, O>;
impl<'a, const O: u8> FTM3FLT0_W<'a, O> {
    #[doc = "FTM3_FLT0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3FLT0_A::_0)
    }
    #[doc = "CMP0 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3FLT0_A::_1)
    }
}
#[doc = "Field `FTM1CH0SRC` reader - FTM1 channel 0 input capture source select"]
pub type FTM1CH0SRC_R = crate::FieldReader<u8, FTM1CH0SRC_A>;
#[doc = "FTM1 channel 0 input capture source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTM1CH0SRC_A {
    #[doc = "0: FTM1_CH0 signal"]
    _00 = 0,
    #[doc = "1: CMP0 output"]
    _01 = 1,
    #[doc = "2: CMP1 output"]
    _10 = 2,
    #[doc = "3: USB start of frame pulse"]
    _11 = 3,
}
impl From<FTM1CH0SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM1CH0SRC_A) -> Self {
        variant as _
    }
}
impl FTM1CH0SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1CH0SRC_A {
        match self.bits {
            0 => FTM1CH0SRC_A::_00,
            1 => FTM1CH0SRC_A::_01,
            2 => FTM1CH0SRC_A::_10,
            3 => FTM1CH0SRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM1CH0SRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM1CH0SRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM1CH0SRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FTM1CH0SRC_A::_11
    }
}
#[doc = "Field `FTM1CH0SRC` writer - FTM1 channel 0 input capture source select"]
pub type FTM1CH0SRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SOPT4_SPEC, u8, FTM1CH0SRC_A, 2, O>;
impl<'a, const O: u8> FTM1CH0SRC_W<'a, O> {
    #[doc = "FTM1_CH0 signal"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM1CH0SRC_A::_00)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM1CH0SRC_A::_01)
    }
    #[doc = "CMP1 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM1CH0SRC_A::_10)
    }
    #[doc = "USB start of frame pulse"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM1CH0SRC_A::_11)
    }
}
#[doc = "Field `FTM2CH0SRC` reader - FTM2 channel 0 input capture source select"]
pub type FTM2CH0SRC_R = crate::FieldReader<u8, FTM2CH0SRC_A>;
#[doc = "FTM2 channel 0 input capture source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTM2CH0SRC_A {
    #[doc = "0: FTM2_CH0 signal"]
    _00 = 0,
    #[doc = "1: CMP0 output"]
    _01 = 1,
    #[doc = "2: CMP1 output"]
    _10 = 2,
}
impl From<FTM2CH0SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2CH0SRC_A) -> Self {
        variant as _
    }
}
impl FTM2CH0SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTM2CH0SRC_A> {
        match self.bits {
            0 => Some(FTM2CH0SRC_A::_00),
            1 => Some(FTM2CH0SRC_A::_01),
            2 => Some(FTM2CH0SRC_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM2CH0SRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM2CH0SRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM2CH0SRC_A::_10
    }
}
#[doc = "Field `FTM2CH0SRC` writer - FTM2 channel 0 input capture source select"]
pub type FTM2CH0SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT4_SPEC, u8, FTM2CH0SRC_A, 2, O>;
impl<'a, const O: u8> FTM2CH0SRC_W<'a, O> {
    #[doc = "FTM2_CH0 signal"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2CH0SRC_A::_00)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2CH0SRC_A::_01)
    }
    #[doc = "CMP1 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM2CH0SRC_A::_10)
    }
}
#[doc = "Field `FTM0CLKSEL` reader - FlexTimer 0 External Clock Pin Select"]
pub type FTM0CLKSEL_R = crate::BitReader<FTM0CLKSEL_A>;
#[doc = "FlexTimer 0 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0CLKSEL_A {
    #[doc = "0: FTM_CLK0 pin"]
    _0 = 0,
    #[doc = "1: FTM_CLK1 pin"]
    _1 = 1,
}
impl From<FTM0CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0CLKSEL_A {
        match self.bits {
            false => FTM0CLKSEL_A::_0,
            true => FTM0CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0CLKSEL_A::_1
    }
}
#[doc = "Field `FTM0CLKSEL` writer - FlexTimer 0 External Clock Pin Select"]
pub type FTM0CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM0CLKSEL_A, O>;
impl<'a, const O: u8> FTM0CLKSEL_W<'a, O> {
    #[doc = "FTM_CLK0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_0)
    }
    #[doc = "FTM_CLK1 pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_1)
    }
}
#[doc = "Field `FTM1CLKSEL` reader - FTM1 External Clock Pin Select"]
pub type FTM1CLKSEL_R = crate::BitReader<FTM1CLKSEL_A>;
#[doc = "FTM1 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM1CLKSEL_A {
    #[doc = "0: FTM_CLK0 pin"]
    _0 = 0,
    #[doc = "1: FTM_CLK1 pin"]
    _1 = 1,
}
impl From<FTM1CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM1CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1CLKSEL_A {
        match self.bits {
            false => FTM1CLKSEL_A::_0,
            true => FTM1CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1CLKSEL_A::_1
    }
}
#[doc = "Field `FTM1CLKSEL` writer - FTM1 External Clock Pin Select"]
pub type FTM1CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM1CLKSEL_A, O>;
impl<'a, const O: u8> FTM1CLKSEL_W<'a, O> {
    #[doc = "FTM_CLK0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_0)
    }
    #[doc = "FTM_CLK1 pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_1)
    }
}
#[doc = "Field `FTM2CLKSEL` reader - FlexTimer 2 External Clock Pin Select"]
pub type FTM2CLKSEL_R = crate::BitReader<FTM2CLKSEL_A>;
#[doc = "FlexTimer 2 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM2CLKSEL_A {
    #[doc = "0: FTM2 external clock driven by FTM_CLK0 pin."]
    _0 = 0,
    #[doc = "1: FTM2 external clock driven by FTM_CLK1 pin."]
    _1 = 1,
}
impl From<FTM2CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM2CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2CLKSEL_A {
        match self.bits {
            false => FTM2CLKSEL_A::_0,
            true => FTM2CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2CLKSEL_A::_1
    }
}
#[doc = "Field `FTM2CLKSEL` writer - FlexTimer 2 External Clock Pin Select"]
pub type FTM2CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM2CLKSEL_A, O>;
impl<'a, const O: u8> FTM2CLKSEL_W<'a, O> {
    #[doc = "FTM2 external clock driven by FTM_CLK0 pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_0)
    }
    #[doc = "FTM2 external clock driven by FTM_CLK1 pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_1)
    }
}
#[doc = "Field `FTM3CLKSEL` reader - FlexTimer 3 External Clock Pin Select"]
pub type FTM3CLKSEL_R = crate::BitReader<FTM3CLKSEL_A>;
#[doc = "FlexTimer 3 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM3CLKSEL_A {
    #[doc = "0: FTM3 external clock driven by FTM_CLK0 pin."]
    _0 = 0,
    #[doc = "1: FTM3 external clock driven by FTM_CLK1 pin."]
    _1 = 1,
}
impl From<FTM3CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM3CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3CLKSEL_A {
        match self.bits {
            false => FTM3CLKSEL_A::_0,
            true => FTM3CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3CLKSEL_A::_1
    }
}
#[doc = "Field `FTM3CLKSEL` writer - FlexTimer 3 External Clock Pin Select"]
pub type FTM3CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM3CLKSEL_A, O>;
impl<'a, const O: u8> FTM3CLKSEL_W<'a, O> {
    #[doc = "FTM3 external clock driven by FTM_CLK0 pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_0)
    }
    #[doc = "FTM3 external clock driven by FTM_CLK1 pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_1)
    }
}
#[doc = "Field `FTM0TRG0SRC` reader - FlexTimer 0 Hardware Trigger 0 Source Select"]
pub type FTM0TRG0SRC_R = crate::BitReader<FTM0TRG0SRC_A>;
#[doc = "FlexTimer 0 Hardware Trigger 0 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0TRG0SRC_A {
    #[doc = "0: HSCMP0 output drives FTM0 hardware trigger 0"]
    _0 = 0,
    #[doc = "1: FTM1 channel match drives FTM0 hardware trigger 0"]
    _1 = 1,
}
impl From<FTM0TRG0SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0TRG0SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0TRG0SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0TRG0SRC_A {
        match self.bits {
            false => FTM0TRG0SRC_A::_0,
            true => FTM0TRG0SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0TRG0SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0TRG0SRC_A::_1
    }
}
#[doc = "Field `FTM0TRG0SRC` writer - FlexTimer 0 Hardware Trigger 0 Source Select"]
pub type FTM0TRG0SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM0TRG0SRC_A, O>;
impl<'a, const O: u8> FTM0TRG0SRC_W<'a, O> {
    #[doc = "HSCMP0 output drives FTM0 hardware trigger 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0TRG0SRC_A::_0)
    }
    #[doc = "FTM1 channel match drives FTM0 hardware trigger 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0TRG0SRC_A::_1)
    }
}
#[doc = "Field `FTM0TRG1SRC` reader - FlexTimer 0 Hardware Trigger 1 Source Select"]
pub type FTM0TRG1SRC_R = crate::BitReader<FTM0TRG1SRC_A>;
#[doc = "FlexTimer 0 Hardware Trigger 1 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0TRG1SRC_A {
    #[doc = "0: PDB output trigger 1 drives FTM0 hardware trigger 1"]
    _0 = 0,
    #[doc = "1: FTM2 channel match drives FTM0 hardware trigger 1"]
    _1 = 1,
}
impl From<FTM0TRG1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0TRG1SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0TRG1SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0TRG1SRC_A {
        match self.bits {
            false => FTM0TRG1SRC_A::_0,
            true => FTM0TRG1SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0TRG1SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0TRG1SRC_A::_1
    }
}
#[doc = "Field `FTM0TRG1SRC` writer - FlexTimer 0 Hardware Trigger 1 Source Select"]
pub type FTM0TRG1SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM0TRG1SRC_A, O>;
impl<'a, const O: u8> FTM0TRG1SRC_W<'a, O> {
    #[doc = "PDB output trigger 1 drives FTM0 hardware trigger 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0TRG1SRC_A::_0)
    }
    #[doc = "FTM2 channel match drives FTM0 hardware trigger 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0TRG1SRC_A::_1)
    }
}
#[doc = "Field `FTM3TRG0SRC` reader - FlexTimer 3 Hardware Trigger 0 Source Select"]
pub type FTM3TRG0SRC_R = crate::BitReader<FTM3TRG0SRC_A>;
#[doc = "FlexTimer 3 Hardware Trigger 0 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM3TRG0SRC_A {
    #[doc = "1: FTM1 channel match drives FTM3 hardware trigger 0"]
    _1 = 1,
}
impl From<FTM3TRG0SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3TRG0SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM3TRG0SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTM3TRG0SRC_A> {
        match self.bits {
            true => Some(FTM3TRG0SRC_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3TRG0SRC_A::_1
    }
}
#[doc = "Field `FTM3TRG0SRC` writer - FlexTimer 3 Hardware Trigger 0 Source Select"]
pub type FTM3TRG0SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM3TRG0SRC_A, O>;
impl<'a, const O: u8> FTM3TRG0SRC_W<'a, O> {
    #[doc = "FTM1 channel match drives FTM3 hardware trigger 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3TRG0SRC_A::_1)
    }
}
#[doc = "Field `FTM3TRG1SRC` reader - FlexTimer 3 Hardware Trigger 1 Source Select"]
pub type FTM3TRG1SRC_R = crate::BitReader<FTM3TRG1SRC_A>;
#[doc = "FlexTimer 3 Hardware Trigger 1 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM3TRG1SRC_A {
    #[doc = "1: FTM2 channel match drives FTM3 hardware trigger 1"]
    _1 = 1,
}
impl From<FTM3TRG1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3TRG1SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM3TRG1SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTM3TRG1SRC_A> {
        match self.bits {
            true => Some(FTM3TRG1SRC_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3TRG1SRC_A::_1
    }
}
#[doc = "Field `FTM3TRG1SRC` writer - FlexTimer 3 Hardware Trigger 1 Source Select"]
pub type FTM3TRG1SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT4_SPEC, FTM3TRG1SRC_A, O>;
impl<'a, const O: u8> FTM3TRG1SRC_W<'a, O> {
    #[doc = "FTM2 channel match drives FTM3 hardware trigger 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3TRG1SRC_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - FTM0 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm0flt0(&self) -> FTM0FLT0_R {
        FTM0FLT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FTM0 Fault 1 Select"]
    #[inline(always)]
    pub fn ftm0flt1(&self) -> FTM0FLT1_R {
        FTM0FLT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FTM0 Fault 2 Select"]
    #[inline(always)]
    pub fn ftm0flt2(&self) -> FTM0FLT2_R {
        FTM0FLT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FTM1 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm1flt0(&self) -> FTM1FLT0_R {
        FTM1FLT0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - FTM2 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm2flt0(&self) -> FTM2FLT0_R {
        FTM2FLT0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - FTM3 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm3flt0(&self) -> FTM3FLT0_R {
        FTM3FLT0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 18:19 - FTM1 channel 0 input capture source select"]
    #[inline(always)]
    pub fn ftm1ch0src(&self) -> FTM1CH0SRC_R {
        FTM1CH0SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - FTM2 channel 0 input capture source select"]
    #[inline(always)]
    pub fn ftm2ch0src(&self) -> FTM2CH0SRC_R {
        FTM2CH0SRC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - FlexTimer 0 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm0clksel(&self) -> FTM0CLKSEL_R {
        FTM0CLKSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FTM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm1clksel(&self) -> FTM1CLKSEL_R {
        FTM1CLKSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FlexTimer 2 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm2clksel(&self) -> FTM2CLKSEL_R {
        FTM2CLKSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FlexTimer 3 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm3clksel(&self) -> FTM3CLKSEL_R {
        FTM3CLKSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FlexTimer 0 Hardware Trigger 0 Source Select"]
    #[inline(always)]
    pub fn ftm0trg0src(&self) -> FTM0TRG0SRC_R {
        FTM0TRG0SRC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FlexTimer 0 Hardware Trigger 1 Source Select"]
    #[inline(always)]
    pub fn ftm0trg1src(&self) -> FTM0TRG1SRC_R {
        FTM0TRG1SRC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - FlexTimer 3 Hardware Trigger 0 Source Select"]
    #[inline(always)]
    pub fn ftm3trg0src(&self) -> FTM3TRG0SRC_R {
        FTM3TRG0SRC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FlexTimer 3 Hardware Trigger 1 Source Select"]
    #[inline(always)]
    pub fn ftm3trg1src(&self) -> FTM3TRG1SRC_R {
        FTM3TRG1SRC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FTM0 Fault 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0flt0(&mut self) -> FTM0FLT0_W<0> {
        FTM0FLT0_W::new(self)
    }
    #[doc = "Bit 1 - FTM0 Fault 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0flt1(&mut self) -> FTM0FLT1_W<1> {
        FTM0FLT1_W::new(self)
    }
    #[doc = "Bit 2 - FTM0 Fault 2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0flt2(&mut self) -> FTM0FLT2_W<2> {
        FTM0FLT2_W::new(self)
    }
    #[doc = "Bit 4 - FTM1 Fault 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm1flt0(&mut self) -> FTM1FLT0_W<4> {
        FTM1FLT0_W::new(self)
    }
    #[doc = "Bit 8 - FTM2 Fault 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm2flt0(&mut self) -> FTM2FLT0_W<8> {
        FTM2FLT0_W::new(self)
    }
    #[doc = "Bit 12 - FTM3 Fault 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm3flt0(&mut self) -> FTM3FLT0_W<12> {
        FTM3FLT0_W::new(self)
    }
    #[doc = "Bits 18:19 - FTM1 channel 0 input capture source select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm1ch0src(&mut self) -> FTM1CH0SRC_W<18> {
        FTM1CH0SRC_W::new(self)
    }
    #[doc = "Bits 20:21 - FTM2 channel 0 input capture source select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm2ch0src(&mut self) -> FTM2CH0SRC_W<20> {
        FTM2CH0SRC_W::new(self)
    }
    #[doc = "Bit 24 - FlexTimer 0 External Clock Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0clksel(&mut self) -> FTM0CLKSEL_W<24> {
        FTM0CLKSEL_W::new(self)
    }
    #[doc = "Bit 25 - FTM1 External Clock Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm1clksel(&mut self) -> FTM1CLKSEL_W<25> {
        FTM1CLKSEL_W::new(self)
    }
    #[doc = "Bit 26 - FlexTimer 2 External Clock Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm2clksel(&mut self) -> FTM2CLKSEL_W<26> {
        FTM2CLKSEL_W::new(self)
    }
    #[doc = "Bit 27 - FlexTimer 3 External Clock Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm3clksel(&mut self) -> FTM3CLKSEL_W<27> {
        FTM3CLKSEL_W::new(self)
    }
    #[doc = "Bit 28 - FlexTimer 0 Hardware Trigger 0 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0trg0src(&mut self) -> FTM0TRG0SRC_W<28> {
        FTM0TRG0SRC_W::new(self)
    }
    #[doc = "Bit 29 - FlexTimer 0 Hardware Trigger 1 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0trg1src(&mut self) -> FTM0TRG1SRC_W<29> {
        FTM0TRG1SRC_W::new(self)
    }
    #[doc = "Bit 30 - FlexTimer 3 Hardware Trigger 0 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm3trg0src(&mut self) -> FTM3TRG0SRC_W<30> {
        FTM3TRG0SRC_W::new(self)
    }
    #[doc = "Bit 31 - FlexTimer 3 Hardware Trigger 1 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn ftm3trg1src(&mut self) -> FTM3TRG1SRC_W<31> {
        FTM3TRG1SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt4](index.html) module"]
pub struct SOPT4_SPEC;
impl crate::RegisterSpec for SOPT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt4::R](R) reader structure"]
impl crate::Readable for SOPT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt4::W](W) writer structure"]
impl crate::Writable for SOPT4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOPT4 to value 0"]
impl crate::Resettable for SOPT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

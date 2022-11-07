#[doc = "Register `COMBINE` reader"]
pub struct R(crate::R<COMBINE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMBINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMBINE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMBINE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMBINE` writer"]
pub struct W(crate::W<COMBINE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMBINE_SPEC>;
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
impl From<crate::W<COMBINE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMBINE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMBINE0` reader - Combine Channels For n = 0"]
pub type COMBINE0_R = crate::BitReader<COMBINE0_A>;
#[doc = "Combine Channels For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMBINE0_A {
    #[doc = "0: Channels (n) and (n+1) are independent."]
    _0 = 0,
    #[doc = "1: Channels (n) and (n+1) are combined."]
    _1 = 1,
}
impl From<COMBINE0_A> for bool {
    #[inline(always)]
    fn from(variant: COMBINE0_A) -> Self {
        variant as u8 != 0
    }
}
impl COMBINE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMBINE0_A {
        match self.bits {
            false => COMBINE0_A::_0,
            true => COMBINE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMBINE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMBINE0_A::_1
    }
}
#[doc = "Field `COMBINE0` writer - Combine Channels For n = 0"]
pub type COMBINE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, COMBINE0_A, O>;
impl<'a, const O: u8> COMBINE0_W<'a, O> {
    #[doc = "Channels (n) and (n+1) are independent."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMBINE0_A::_0)
    }
    #[doc = "Channels (n) and (n+1) are combined."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMBINE0_A::_1)
    }
}
#[doc = "Field `COMP0` reader - Complement Of Channel (n) For n = 0"]
pub type COMP0_R = crate::BitReader<COMP0_A>;
#[doc = "Complement Of Channel (n) For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP0_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP0_A> for bool {
    #[inline(always)]
    fn from(variant: COMP0_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP0_A {
        match self.bits {
            false => COMP0_A::_0,
            true => COMP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMP0_A::_1
    }
}
#[doc = "Field `COMP0` writer - Complement Of Channel (n) For n = 0"]
pub type COMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, COMP0_A, O>;
impl<'a, const O: u8> COMP0_W<'a, O> {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP0_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP0_A::_1)
    }
}
#[doc = "Field `DECAPEN0` reader - Dual Edge Capture Mode Enable For n = 0"]
pub type DECAPEN0_R = crate::BitReader<DECAPEN0_A>;
#[doc = "Dual Edge Capture Mode Enable For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DECAPEN0_A {
    #[doc = "0: The Dual Edge Capture mode in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The Dual Edge Capture mode in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DECAPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DECAPEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl DECAPEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAPEN0_A {
        match self.bits {
            false => DECAPEN0_A::_0,
            true => DECAPEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAPEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAPEN0_A::_1
    }
}
#[doc = "Field `DECAPEN0` writer - Dual Edge Capture Mode Enable For n = 0"]
pub type DECAPEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DECAPEN0_A, O>;
impl<'a, const O: u8> DECAPEN0_W<'a, O> {
    #[doc = "The Dual Edge Capture mode in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAPEN0_A::_0)
    }
    #[doc = "The Dual Edge Capture mode in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAPEN0_A::_1)
    }
}
#[doc = "Field `DECAP0` reader - Dual Edge Capture Mode Captures For n = 0"]
pub type DECAP0_R = crate::BitReader<DECAP0_A>;
#[doc = "Dual Edge Capture Mode Captures For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DECAP0_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP0_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP0_A) -> Self {
        variant as u8 != 0
    }
}
impl DECAP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP0_A {
        match self.bits {
            false => DECAP0_A::_0,
            true => DECAP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAP0_A::_1
    }
}
#[doc = "Field `DECAP0` writer - Dual Edge Capture Mode Captures For n = 0"]
pub type DECAP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DECAP0_A, O>;
impl<'a, const O: u8> DECAP0_W<'a, O> {
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP0_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP0_A::_1)
    }
}
#[doc = "Field `DTEN0` reader - Deadtime Enable For n = 0"]
pub type DTEN0_R = crate::BitReader<DTEN0_A>;
#[doc = "Deadtime Enable For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN0_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN0_A {
        match self.bits {
            false => DTEN0_A::_0,
            true => DTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEN0_A::_1
    }
}
#[doc = "Field `DTEN0` writer - Deadtime Enable For n = 0"]
pub type DTEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DTEN0_A, O>;
impl<'a, const O: u8> DTEN0_W<'a, O> {
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN0_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN0_A::_1)
    }
}
#[doc = "Field `SYNCEN0` reader - Synchronization Enable For n = 0"]
pub type SYNCEN0_R = crate::BitReader<SYNCEN0_A>;
#[doc = "Synchronization Enable For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCEN0_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN0_A {
        match self.bits {
            false => SYNCEN0_A::_0,
            true => SYNCEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN0_A::_1
    }
}
#[doc = "Field `SYNCEN0` writer - Synchronization Enable For n = 0"]
pub type SYNCEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, SYNCEN0_A, O>;
impl<'a, const O: u8> SYNCEN0_W<'a, O> {
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN0_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN0_A::_1)
    }
}
#[doc = "Field `FAULTEN0` reader - Fault Control Enable For n = 0"]
pub type FAULTEN0_R = crate::BitReader<FAULTEN0_A>;
#[doc = "Fault Control Enable For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULTEN0_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN0_A {
        match self.bits {
            false => FAULTEN0_A::_0,
            true => FAULTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN0_A::_1
    }
}
#[doc = "Field `FAULTEN0` writer - Fault Control Enable For n = 0"]
pub type FAULTEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, FAULTEN0_A, O>;
impl<'a, const O: u8> FAULTEN0_W<'a, O> {
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN0_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN0_A::_1)
    }
}
#[doc = "Field `COMBINE1` reader - Combine Channels For n = 2"]
pub type COMBINE1_R = crate::BitReader<COMBINE1_A>;
#[doc = "Combine Channels For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMBINE1_A {
    #[doc = "0: Channels (n) and (n+1) are independent."]
    _0 = 0,
    #[doc = "1: Channels (n) and (n+1) are combined."]
    _1 = 1,
}
impl From<COMBINE1_A> for bool {
    #[inline(always)]
    fn from(variant: COMBINE1_A) -> Self {
        variant as u8 != 0
    }
}
impl COMBINE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMBINE1_A {
        match self.bits {
            false => COMBINE1_A::_0,
            true => COMBINE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMBINE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMBINE1_A::_1
    }
}
#[doc = "Field `COMBINE1` writer - Combine Channels For n = 2"]
pub type COMBINE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, COMBINE1_A, O>;
impl<'a, const O: u8> COMBINE1_W<'a, O> {
    #[doc = "Channels (n) and (n+1) are independent."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMBINE1_A::_0)
    }
    #[doc = "Channels (n) and (n+1) are combined."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMBINE1_A::_1)
    }
}
#[doc = "Field `COMP1` reader - Complement Of Channel (n) For n = 2"]
pub type COMP1_R = crate::BitReader<COMP1_A>;
#[doc = "Complement Of Channel (n) For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP1_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1_A {
        match self.bits {
            false => COMP1_A::_0,
            true => COMP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMP1_A::_1
    }
}
#[doc = "Field `COMP1` writer - Complement Of Channel (n) For n = 2"]
pub type COMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, COMP1_A, O>;
impl<'a, const O: u8> COMP1_W<'a, O> {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP1_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP1_A::_1)
    }
}
#[doc = "Field `DECAPEN1` reader - Dual Edge Capture Mode Enable For n = 2"]
pub type DECAPEN1_R = crate::BitReader<DECAPEN1_A>;
#[doc = "Dual Edge Capture Mode Enable For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DECAPEN1_A {
    #[doc = "0: The Dual Edge Capture mode in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The Dual Edge Capture mode in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DECAPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DECAPEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl DECAPEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAPEN1_A {
        match self.bits {
            false => DECAPEN1_A::_0,
            true => DECAPEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAPEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAPEN1_A::_1
    }
}
#[doc = "Field `DECAPEN1` writer - Dual Edge Capture Mode Enable For n = 2"]
pub type DECAPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DECAPEN1_A, O>;
impl<'a, const O: u8> DECAPEN1_W<'a, O> {
    #[doc = "The Dual Edge Capture mode in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAPEN1_A::_0)
    }
    #[doc = "The Dual Edge Capture mode in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAPEN1_A::_1)
    }
}
#[doc = "Field `DECAP1` reader - Dual Edge Capture Mode Captures For n = 2"]
pub type DECAP1_R = crate::BitReader<DECAP1_A>;
#[doc = "Dual Edge Capture Mode Captures For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DECAP1_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP1_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP1_A) -> Self {
        variant as u8 != 0
    }
}
impl DECAP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP1_A {
        match self.bits {
            false => DECAP1_A::_0,
            true => DECAP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAP1_A::_1
    }
}
#[doc = "Field `DECAP1` writer - Dual Edge Capture Mode Captures For n = 2"]
pub type DECAP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DECAP1_A, O>;
impl<'a, const O: u8> DECAP1_W<'a, O> {
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP1_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP1_A::_1)
    }
}
#[doc = "Field `DTEN1` reader - Deadtime Enable For n = 2"]
pub type DTEN1_R = crate::BitReader<DTEN1_A>;
#[doc = "Deadtime Enable For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN1_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN1_A {
        match self.bits {
            false => DTEN1_A::_0,
            true => DTEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEN1_A::_1
    }
}
#[doc = "Field `DTEN1` writer - Deadtime Enable For n = 2"]
pub type DTEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DTEN1_A, O>;
impl<'a, const O: u8> DTEN1_W<'a, O> {
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN1_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN1_A::_1)
    }
}
#[doc = "Field `SYNCEN1` reader - Synchronization Enable For n = 2"]
pub type SYNCEN1_R = crate::BitReader<SYNCEN1_A>;
#[doc = "Synchronization Enable For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCEN1_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN1_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN1_A {
        match self.bits {
            false => SYNCEN1_A::_0,
            true => SYNCEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN1_A::_1
    }
}
#[doc = "Field `SYNCEN1` writer - Synchronization Enable For n = 2"]
pub type SYNCEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, SYNCEN1_A, O>;
impl<'a, const O: u8> SYNCEN1_W<'a, O> {
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN1_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN1_A::_1)
    }
}
#[doc = "Field `FAULTEN1` reader - Fault Control Enable For n = 2"]
pub type FAULTEN1_R = crate::BitReader<FAULTEN1_A>;
#[doc = "Fault Control Enable For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULTEN1_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULTEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN1_A {
        match self.bits {
            false => FAULTEN1_A::_0,
            true => FAULTEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN1_A::_1
    }
}
#[doc = "Field `FAULTEN1` writer - Fault Control Enable For n = 2"]
pub type FAULTEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, FAULTEN1_A, O>;
impl<'a, const O: u8> FAULTEN1_W<'a, O> {
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN1_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN1_A::_1)
    }
}
#[doc = "Field `COMBINE2` reader - Combine Channels For n = 4"]
pub type COMBINE2_R = crate::BitReader<COMBINE2_A>;
#[doc = "Combine Channels For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMBINE2_A {
    #[doc = "0: Channels (n) and (n+1) are independent."]
    _0 = 0,
    #[doc = "1: Channels (n) and (n+1) are combined."]
    _1 = 1,
}
impl From<COMBINE2_A> for bool {
    #[inline(always)]
    fn from(variant: COMBINE2_A) -> Self {
        variant as u8 != 0
    }
}
impl COMBINE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMBINE2_A {
        match self.bits {
            false => COMBINE2_A::_0,
            true => COMBINE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMBINE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMBINE2_A::_1
    }
}
#[doc = "Field `COMBINE2` writer - Combine Channels For n = 4"]
pub type COMBINE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, COMBINE2_A, O>;
impl<'a, const O: u8> COMBINE2_W<'a, O> {
    #[doc = "Channels (n) and (n+1) are independent."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMBINE2_A::_0)
    }
    #[doc = "Channels (n) and (n+1) are combined."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMBINE2_A::_1)
    }
}
#[doc = "Field `COMP2` reader - Complement Of Channel (n) For n = 4"]
pub type COMP2_R = crate::BitReader<COMP2_A>;
#[doc = "Complement Of Channel (n) For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP2_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2_A {
        match self.bits {
            false => COMP2_A::_0,
            true => COMP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMP2_A::_1
    }
}
#[doc = "Field `COMP2` writer - Complement Of Channel (n) For n = 4"]
pub type COMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, COMP2_A, O>;
impl<'a, const O: u8> COMP2_W<'a, O> {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP2_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP2_A::_1)
    }
}
#[doc = "Field `DECAPEN2` reader - Dual Edge Capture Mode Enable For n = 4"]
pub type DECAPEN2_R = crate::BitReader<DECAPEN2_A>;
#[doc = "Dual Edge Capture Mode Enable For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DECAPEN2_A {
    #[doc = "0: The Dual Edge Capture mode in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The Dual Edge Capture mode in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DECAPEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DECAPEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl DECAPEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAPEN2_A {
        match self.bits {
            false => DECAPEN2_A::_0,
            true => DECAPEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAPEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAPEN2_A::_1
    }
}
#[doc = "Field `DECAPEN2` writer - Dual Edge Capture Mode Enable For n = 4"]
pub type DECAPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DECAPEN2_A, O>;
impl<'a, const O: u8> DECAPEN2_W<'a, O> {
    #[doc = "The Dual Edge Capture mode in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAPEN2_A::_0)
    }
    #[doc = "The Dual Edge Capture mode in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAPEN2_A::_1)
    }
}
#[doc = "Field `DECAP2` reader - Dual Edge Capture Mode Captures For n = 4"]
pub type DECAP2_R = crate::BitReader<DECAP2_A>;
#[doc = "Dual Edge Capture Mode Captures For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DECAP2_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP2_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP2_A) -> Self {
        variant as u8 != 0
    }
}
impl DECAP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP2_A {
        match self.bits {
            false => DECAP2_A::_0,
            true => DECAP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAP2_A::_1
    }
}
#[doc = "Field `DECAP2` writer - Dual Edge Capture Mode Captures For n = 4"]
pub type DECAP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DECAP2_A, O>;
impl<'a, const O: u8> DECAP2_W<'a, O> {
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP2_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP2_A::_1)
    }
}
#[doc = "Field `DTEN2` reader - Deadtime Enable For n = 4"]
pub type DTEN2_R = crate::BitReader<DTEN2_A>;
#[doc = "Deadtime Enable For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN2_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN2_A {
        match self.bits {
            false => DTEN2_A::_0,
            true => DTEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEN2_A::_1
    }
}
#[doc = "Field `DTEN2` writer - Deadtime Enable For n = 4"]
pub type DTEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DTEN2_A, O>;
impl<'a, const O: u8> DTEN2_W<'a, O> {
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN2_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN2_A::_1)
    }
}
#[doc = "Field `SYNCEN2` reader - Synchronization Enable For n = 4"]
pub type SYNCEN2_R = crate::BitReader<SYNCEN2_A>;
#[doc = "Synchronization Enable For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCEN2_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN2_A {
        match self.bits {
            false => SYNCEN2_A::_0,
            true => SYNCEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN2_A::_1
    }
}
#[doc = "Field `SYNCEN2` writer - Synchronization Enable For n = 4"]
pub type SYNCEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, SYNCEN2_A, O>;
impl<'a, const O: u8> SYNCEN2_W<'a, O> {
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN2_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN2_A::_1)
    }
}
#[doc = "Field `FAULTEN2` reader - Fault Control Enable For n = 4"]
pub type FAULTEN2_R = crate::BitReader<FAULTEN2_A>;
#[doc = "Fault Control Enable For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULTEN2_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULTEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN2_A {
        match self.bits {
            false => FAULTEN2_A::_0,
            true => FAULTEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN2_A::_1
    }
}
#[doc = "Field `FAULTEN2` writer - Fault Control Enable For n = 4"]
pub type FAULTEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, FAULTEN2_A, O>;
impl<'a, const O: u8> FAULTEN2_W<'a, O> {
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN2_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN2_A::_1)
    }
}
#[doc = "Field `COMBINE3` reader - Combine Channels For n = 6"]
pub type COMBINE3_R = crate::BitReader<COMBINE3_A>;
#[doc = "Combine Channels For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMBINE3_A {
    #[doc = "0: Channels (n) and (n+1) are independent."]
    _0 = 0,
    #[doc = "1: Channels (n) and (n+1) are combined."]
    _1 = 1,
}
impl From<COMBINE3_A> for bool {
    #[inline(always)]
    fn from(variant: COMBINE3_A) -> Self {
        variant as u8 != 0
    }
}
impl COMBINE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMBINE3_A {
        match self.bits {
            false => COMBINE3_A::_0,
            true => COMBINE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMBINE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMBINE3_A::_1
    }
}
#[doc = "Field `COMBINE3` writer - Combine Channels For n = 6"]
pub type COMBINE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, COMBINE3_A, O>;
impl<'a, const O: u8> COMBINE3_W<'a, O> {
    #[doc = "Channels (n) and (n+1) are independent."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMBINE3_A::_0)
    }
    #[doc = "Channels (n) and (n+1) are combined."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMBINE3_A::_1)
    }
}
#[doc = "Field `COMP3` reader - Complement Of Channel (n) for n = 6"]
pub type COMP3_R = crate::BitReader<COMP3_A>;
#[doc = "Complement Of Channel (n) for n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP3_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP3_A> for bool {
    #[inline(always)]
    fn from(variant: COMP3_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP3_A {
        match self.bits {
            false => COMP3_A::_0,
            true => COMP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMP3_A::_1
    }
}
#[doc = "Field `COMP3` writer - Complement Of Channel (n) for n = 6"]
pub type COMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, COMP3_A, O>;
impl<'a, const O: u8> COMP3_W<'a, O> {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP3_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP3_A::_1)
    }
}
#[doc = "Field `DECAPEN3` reader - Dual Edge Capture Mode Enable For n = 6"]
pub type DECAPEN3_R = crate::BitReader<DECAPEN3_A>;
#[doc = "Dual Edge Capture Mode Enable For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DECAPEN3_A {
    #[doc = "0: The Dual Edge Capture mode in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The Dual Edge Capture mode in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DECAPEN3_A> for bool {
    #[inline(always)]
    fn from(variant: DECAPEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl DECAPEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAPEN3_A {
        match self.bits {
            false => DECAPEN3_A::_0,
            true => DECAPEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAPEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAPEN3_A::_1
    }
}
#[doc = "Field `DECAPEN3` writer - Dual Edge Capture Mode Enable For n = 6"]
pub type DECAPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DECAPEN3_A, O>;
impl<'a, const O: u8> DECAPEN3_W<'a, O> {
    #[doc = "The Dual Edge Capture mode in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAPEN3_A::_0)
    }
    #[doc = "The Dual Edge Capture mode in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAPEN3_A::_1)
    }
}
#[doc = "Field `DECAP3` reader - Dual Edge Capture Mode Captures For n = 6"]
pub type DECAP3_R = crate::BitReader<DECAP3_A>;
#[doc = "Dual Edge Capture Mode Captures For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DECAP3_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP3_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP3_A) -> Self {
        variant as u8 != 0
    }
}
impl DECAP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP3_A {
        match self.bits {
            false => DECAP3_A::_0,
            true => DECAP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAP3_A::_1
    }
}
#[doc = "Field `DECAP3` writer - Dual Edge Capture Mode Captures For n = 6"]
pub type DECAP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DECAP3_A, O>;
impl<'a, const O: u8> DECAP3_W<'a, O> {
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP3_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP3_A::_1)
    }
}
#[doc = "Field `DTEN3` reader - Deadtime Enable For n = 6"]
pub type DTEN3_R = crate::BitReader<DTEN3_A>;
#[doc = "Deadtime Enable For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN3_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN3_A {
        match self.bits {
            false => DTEN3_A::_0,
            true => DTEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEN3_A::_1
    }
}
#[doc = "Field `DTEN3` writer - Deadtime Enable For n = 6"]
pub type DTEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, DTEN3_A, O>;
impl<'a, const O: u8> DTEN3_W<'a, O> {
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN3_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN3_A::_1)
    }
}
#[doc = "Field `SYNCEN3` reader - Synchronization Enable For n = 6"]
pub type SYNCEN3_R = crate::BitReader<SYNCEN3_A>;
#[doc = "Synchronization Enable For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCEN3_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN3_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN3_A {
        match self.bits {
            false => SYNCEN3_A::_0,
            true => SYNCEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN3_A::_1
    }
}
#[doc = "Field `SYNCEN3` writer - Synchronization Enable For n = 6"]
pub type SYNCEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, SYNCEN3_A, O>;
impl<'a, const O: u8> SYNCEN3_W<'a, O> {
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN3_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN3_A::_1)
    }
}
#[doc = "Field `FAULTEN3` reader - Fault Control Enable For n = 6"]
pub type FAULTEN3_R = crate::BitReader<FAULTEN3_A>;
#[doc = "Fault Control Enable For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULTEN3_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULTEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN3_A {
        match self.bits {
            false => FAULTEN3_A::_0,
            true => FAULTEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN3_A::_1
    }
}
#[doc = "Field `FAULTEN3` writer - Fault Control Enable For n = 6"]
pub type FAULTEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBINE_SPEC, FAULTEN3_A, O>;
impl<'a, const O: u8> FAULTEN3_W<'a, O> {
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN3_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN3_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Combine Channels For n = 0"]
    #[inline(always)]
    pub fn combine0(&self) -> COMBINE0_R {
        COMBINE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Complement Of Channel (n) For n = 0"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dual Edge Capture Mode Enable For n = 0"]
    #[inline(always)]
    pub fn decapen0(&self) -> DECAPEN0_R {
        DECAPEN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dual Edge Capture Mode Captures For n = 0"]
    #[inline(always)]
    pub fn decap0(&self) -> DECAP0_R {
        DECAP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deadtime Enable For n = 0"]
    #[inline(always)]
    pub fn dten0(&self) -> DTEN0_R {
        DTEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization Enable For n = 0"]
    #[inline(always)]
    pub fn syncen0(&self) -> SYNCEN0_R {
        SYNCEN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault Control Enable For n = 0"]
    #[inline(always)]
    pub fn faulten0(&self) -> FAULTEN0_R {
        FAULTEN0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Combine Channels For n = 2"]
    #[inline(always)]
    pub fn combine1(&self) -> COMBINE1_R {
        COMBINE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Complement Of Channel (n) For n = 2"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Dual Edge Capture Mode Enable For n = 2"]
    #[inline(always)]
    pub fn decapen1(&self) -> DECAPEN1_R {
        DECAPEN1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Dual Edge Capture Mode Captures For n = 2"]
    #[inline(always)]
    pub fn decap1(&self) -> DECAP1_R {
        DECAP1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Deadtime Enable For n = 2"]
    #[inline(always)]
    pub fn dten1(&self) -> DTEN1_R {
        DTEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Synchronization Enable For n = 2"]
    #[inline(always)]
    pub fn syncen1(&self) -> SYNCEN1_R {
        SYNCEN1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fault Control Enable For n = 2"]
    #[inline(always)]
    pub fn faulten1(&self) -> FAULTEN1_R {
        FAULTEN1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Combine Channels For n = 4"]
    #[inline(always)]
    pub fn combine2(&self) -> COMBINE2_R {
        COMBINE2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Complement Of Channel (n) For n = 4"]
    #[inline(always)]
    pub fn comp2(&self) -> COMP2_R {
        COMP2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Dual Edge Capture Mode Enable For n = 4"]
    #[inline(always)]
    pub fn decapen2(&self) -> DECAPEN2_R {
        DECAPEN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Dual Edge Capture Mode Captures For n = 4"]
    #[inline(always)]
    pub fn decap2(&self) -> DECAP2_R {
        DECAP2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Deadtime Enable For n = 4"]
    #[inline(always)]
    pub fn dten2(&self) -> DTEN2_R {
        DTEN2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Synchronization Enable For n = 4"]
    #[inline(always)]
    pub fn syncen2(&self) -> SYNCEN2_R {
        SYNCEN2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Fault Control Enable For n = 4"]
    #[inline(always)]
    pub fn faulten2(&self) -> FAULTEN2_R {
        FAULTEN2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Combine Channels For n = 6"]
    #[inline(always)]
    pub fn combine3(&self) -> COMBINE3_R {
        COMBINE3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Complement Of Channel (n) for n = 6"]
    #[inline(always)]
    pub fn comp3(&self) -> COMP3_R {
        COMP3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Dual Edge Capture Mode Enable For n = 6"]
    #[inline(always)]
    pub fn decapen3(&self) -> DECAPEN3_R {
        DECAPEN3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Dual Edge Capture Mode Captures For n = 6"]
    #[inline(always)]
    pub fn decap3(&self) -> DECAP3_R {
        DECAP3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Deadtime Enable For n = 6"]
    #[inline(always)]
    pub fn dten3(&self) -> DTEN3_R {
        DTEN3_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Synchronization Enable For n = 6"]
    #[inline(always)]
    pub fn syncen3(&self) -> SYNCEN3_R {
        SYNCEN3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Fault Control Enable For n = 6"]
    #[inline(always)]
    pub fn faulten3(&self) -> FAULTEN3_R {
        FAULTEN3_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Combine Channels For n = 0"]
    #[inline(always)]
    #[must_use]
    pub fn combine0(&mut self) -> COMBINE0_W<0> {
        COMBINE0_W::new(self)
    }
    #[doc = "Bit 1 - Complement Of Channel (n) For n = 0"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> COMP0_W<1> {
        COMP0_W::new(self)
    }
    #[doc = "Bit 2 - Dual Edge Capture Mode Enable For n = 0"]
    #[inline(always)]
    #[must_use]
    pub fn decapen0(&mut self) -> DECAPEN0_W<2> {
        DECAPEN0_W::new(self)
    }
    #[doc = "Bit 3 - Dual Edge Capture Mode Captures For n = 0"]
    #[inline(always)]
    #[must_use]
    pub fn decap0(&mut self) -> DECAP0_W<3> {
        DECAP0_W::new(self)
    }
    #[doc = "Bit 4 - Deadtime Enable For n = 0"]
    #[inline(always)]
    #[must_use]
    pub fn dten0(&mut self) -> DTEN0_W<4> {
        DTEN0_W::new(self)
    }
    #[doc = "Bit 5 - Synchronization Enable For n = 0"]
    #[inline(always)]
    #[must_use]
    pub fn syncen0(&mut self) -> SYNCEN0_W<5> {
        SYNCEN0_W::new(self)
    }
    #[doc = "Bit 6 - Fault Control Enable For n = 0"]
    #[inline(always)]
    #[must_use]
    pub fn faulten0(&mut self) -> FAULTEN0_W<6> {
        FAULTEN0_W::new(self)
    }
    #[doc = "Bit 8 - Combine Channels For n = 2"]
    #[inline(always)]
    #[must_use]
    pub fn combine1(&mut self) -> COMBINE1_W<8> {
        COMBINE1_W::new(self)
    }
    #[doc = "Bit 9 - Complement Of Channel (n) For n = 2"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> COMP1_W<9> {
        COMP1_W::new(self)
    }
    #[doc = "Bit 10 - Dual Edge Capture Mode Enable For n = 2"]
    #[inline(always)]
    #[must_use]
    pub fn decapen1(&mut self) -> DECAPEN1_W<10> {
        DECAPEN1_W::new(self)
    }
    #[doc = "Bit 11 - Dual Edge Capture Mode Captures For n = 2"]
    #[inline(always)]
    #[must_use]
    pub fn decap1(&mut self) -> DECAP1_W<11> {
        DECAP1_W::new(self)
    }
    #[doc = "Bit 12 - Deadtime Enable For n = 2"]
    #[inline(always)]
    #[must_use]
    pub fn dten1(&mut self) -> DTEN1_W<12> {
        DTEN1_W::new(self)
    }
    #[doc = "Bit 13 - Synchronization Enable For n = 2"]
    #[inline(always)]
    #[must_use]
    pub fn syncen1(&mut self) -> SYNCEN1_W<13> {
        SYNCEN1_W::new(self)
    }
    #[doc = "Bit 14 - Fault Control Enable For n = 2"]
    #[inline(always)]
    #[must_use]
    pub fn faulten1(&mut self) -> FAULTEN1_W<14> {
        FAULTEN1_W::new(self)
    }
    #[doc = "Bit 16 - Combine Channels For n = 4"]
    #[inline(always)]
    #[must_use]
    pub fn combine2(&mut self) -> COMBINE2_W<16> {
        COMBINE2_W::new(self)
    }
    #[doc = "Bit 17 - Complement Of Channel (n) For n = 4"]
    #[inline(always)]
    #[must_use]
    pub fn comp2(&mut self) -> COMP2_W<17> {
        COMP2_W::new(self)
    }
    #[doc = "Bit 18 - Dual Edge Capture Mode Enable For n = 4"]
    #[inline(always)]
    #[must_use]
    pub fn decapen2(&mut self) -> DECAPEN2_W<18> {
        DECAPEN2_W::new(self)
    }
    #[doc = "Bit 19 - Dual Edge Capture Mode Captures For n = 4"]
    #[inline(always)]
    #[must_use]
    pub fn decap2(&mut self) -> DECAP2_W<19> {
        DECAP2_W::new(self)
    }
    #[doc = "Bit 20 - Deadtime Enable For n = 4"]
    #[inline(always)]
    #[must_use]
    pub fn dten2(&mut self) -> DTEN2_W<20> {
        DTEN2_W::new(self)
    }
    #[doc = "Bit 21 - Synchronization Enable For n = 4"]
    #[inline(always)]
    #[must_use]
    pub fn syncen2(&mut self) -> SYNCEN2_W<21> {
        SYNCEN2_W::new(self)
    }
    #[doc = "Bit 22 - Fault Control Enable For n = 4"]
    #[inline(always)]
    #[must_use]
    pub fn faulten2(&mut self) -> FAULTEN2_W<22> {
        FAULTEN2_W::new(self)
    }
    #[doc = "Bit 24 - Combine Channels For n = 6"]
    #[inline(always)]
    #[must_use]
    pub fn combine3(&mut self) -> COMBINE3_W<24> {
        COMBINE3_W::new(self)
    }
    #[doc = "Bit 25 - Complement Of Channel (n) for n = 6"]
    #[inline(always)]
    #[must_use]
    pub fn comp3(&mut self) -> COMP3_W<25> {
        COMP3_W::new(self)
    }
    #[doc = "Bit 26 - Dual Edge Capture Mode Enable For n = 6"]
    #[inline(always)]
    #[must_use]
    pub fn decapen3(&mut self) -> DECAPEN3_W<26> {
        DECAPEN3_W::new(self)
    }
    #[doc = "Bit 27 - Dual Edge Capture Mode Captures For n = 6"]
    #[inline(always)]
    #[must_use]
    pub fn decap3(&mut self) -> DECAP3_W<27> {
        DECAP3_W::new(self)
    }
    #[doc = "Bit 28 - Deadtime Enable For n = 6"]
    #[inline(always)]
    #[must_use]
    pub fn dten3(&mut self) -> DTEN3_W<28> {
        DTEN3_W::new(self)
    }
    #[doc = "Bit 29 - Synchronization Enable For n = 6"]
    #[inline(always)]
    #[must_use]
    pub fn syncen3(&mut self) -> SYNCEN3_W<29> {
        SYNCEN3_W::new(self)
    }
    #[doc = "Bit 30 - Fault Control Enable For n = 6"]
    #[inline(always)]
    #[must_use]
    pub fn faulten3(&mut self) -> FAULTEN3_W<30> {
        FAULTEN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Function For Linked Channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [combine](index.html) module"]
pub struct COMBINE_SPEC;
impl crate::RegisterSpec for COMBINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [combine::R](R) reader structure"]
impl crate::Readable for COMBINE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [combine::W](W) writer structure"]
impl crate::Writable for COMBINE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMBINE to value 0"]
impl crate::Resettable for COMBINE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

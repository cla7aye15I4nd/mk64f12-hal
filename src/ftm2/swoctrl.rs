#[doc = "Register `SWOCTRL` reader"]
pub struct R(crate::R<SWOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWOCTRL` writer"]
pub struct W(crate::W<SWOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWOCTRL_SPEC>;
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
impl From<crate::W<SWOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0OC` reader - Channel 0 Software Output Control Enable"]
pub type CH0OC_R = crate::BitReader<CH0OC_A>;
#[doc = "Channel 0 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH0OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OC_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0OC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OC_A {
        match self.bits {
            false => CH0OC_A::_0,
            true => CH0OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0OC_A::_1
    }
}
#[doc = "Field `CH0OC` writer - Channel 0 Software Output Control Enable"]
pub type CH0OC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH0OC_A, O>;
impl<'a, const O: u8> CH0OC_W<'a, O> {
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OC_A::_1)
    }
}
#[doc = "Field `CH1OC` reader - Channel 1 Software Output Control Enable"]
pub type CH1OC_R = crate::BitReader<CH1OC_A>;
#[doc = "Channel 1 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH1OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OC_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1OC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OC_A {
        match self.bits {
            false => CH1OC_A::_0,
            true => CH1OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1OC_A::_1
    }
}
#[doc = "Field `CH1OC` writer - Channel 1 Software Output Control Enable"]
pub type CH1OC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH1OC_A, O>;
impl<'a, const O: u8> CH1OC_W<'a, O> {
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OC_A::_1)
    }
}
#[doc = "Field `CH2OC` reader - Channel 2 Software Output Control Enable"]
pub type CH2OC_R = crate::BitReader<CH2OC_A>;
#[doc = "Channel 2 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH2OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OC_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2OC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OC_A {
        match self.bits {
            false => CH2OC_A::_0,
            true => CH2OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2OC_A::_1
    }
}
#[doc = "Field `CH2OC` writer - Channel 2 Software Output Control Enable"]
pub type CH2OC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH2OC_A, O>;
impl<'a, const O: u8> CH2OC_W<'a, O> {
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OC_A::_1)
    }
}
#[doc = "Field `CH3OC` reader - Channel 3 Software Output Control Enable"]
pub type CH3OC_R = crate::BitReader<CH3OC_A>;
#[doc = "Channel 3 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH3OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OC_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3OC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OC_A {
        match self.bits {
            false => CH3OC_A::_0,
            true => CH3OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3OC_A::_1
    }
}
#[doc = "Field `CH3OC` writer - Channel 3 Software Output Control Enable"]
pub type CH3OC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH3OC_A, O>;
impl<'a, const O: u8> CH3OC_W<'a, O> {
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OC_A::_1)
    }
}
#[doc = "Field `CH4OC` reader - Channel 4 Software Output Control Enable"]
pub type CH4OC_R = crate::BitReader<CH4OC_A>;
#[doc = "Channel 4 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH4OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OC_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4OC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OC_A {
        match self.bits {
            false => CH4OC_A::_0,
            true => CH4OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4OC_A::_1
    }
}
#[doc = "Field `CH4OC` writer - Channel 4 Software Output Control Enable"]
pub type CH4OC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH4OC_A, O>;
impl<'a, const O: u8> CH4OC_W<'a, O> {
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OC_A::_1)
    }
}
#[doc = "Field `CH5OC` reader - Channel 5 Software Output Control Enable"]
pub type CH5OC_R = crate::BitReader<CH5OC_A>;
#[doc = "Channel 5 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH5OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OC_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5OC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OC_A {
        match self.bits {
            false => CH5OC_A::_0,
            true => CH5OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5OC_A::_1
    }
}
#[doc = "Field `CH5OC` writer - Channel 5 Software Output Control Enable"]
pub type CH5OC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH5OC_A, O>;
impl<'a, const O: u8> CH5OC_W<'a, O> {
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OC_A::_1)
    }
}
#[doc = "Field `CH6OC` reader - Channel 6 Software Output Control Enable"]
pub type CH6OC_R = crate::BitReader<CH6OC_A>;
#[doc = "Channel 6 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH6OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OC_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6OC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OC_A {
        match self.bits {
            false => CH6OC_A::_0,
            true => CH6OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6OC_A::_1
    }
}
#[doc = "Field `CH6OC` writer - Channel 6 Software Output Control Enable"]
pub type CH6OC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH6OC_A, O>;
impl<'a, const O: u8> CH6OC_W<'a, O> {
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OC_A::_1)
    }
}
#[doc = "Field `CH7OC` reader - Channel 7 Software Output Control Enable"]
pub type CH7OC_R = crate::BitReader<CH7OC_A>;
#[doc = "Channel 7 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH7OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OC_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7OC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OC_A {
        match self.bits {
            false => CH7OC_A::_0,
            true => CH7OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7OC_A::_1
    }
}
#[doc = "Field `CH7OC` writer - Channel 7 Software Output Control Enable"]
pub type CH7OC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH7OC_A, O>;
impl<'a, const O: u8> CH7OC_W<'a, O> {
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OC_A::_1)
    }
}
#[doc = "Field `CH0OCV` reader - Channel 0 Software Output Control Value"]
pub type CH0OCV_R = crate::BitReader<CH0OCV_A>;
#[doc = "Channel 0 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH0OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OCV_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0OCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OCV_A {
        match self.bits {
            false => CH0OCV_A::_0,
            true => CH0OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0OCV_A::_1
    }
}
#[doc = "Field `CH0OCV` writer - Channel 0 Software Output Control Value"]
pub type CH0OCV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH0OCV_A, O>;
impl<'a, const O: u8> CH0OCV_W<'a, O> {
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OCV_A::_1)
    }
}
#[doc = "Field `CH1OCV` reader - Channel 1 Software Output Control Value"]
pub type CH1OCV_R = crate::BitReader<CH1OCV_A>;
#[doc = "Channel 1 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH1OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OCV_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1OCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OCV_A {
        match self.bits {
            false => CH1OCV_A::_0,
            true => CH1OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1OCV_A::_1
    }
}
#[doc = "Field `CH1OCV` writer - Channel 1 Software Output Control Value"]
pub type CH1OCV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH1OCV_A, O>;
impl<'a, const O: u8> CH1OCV_W<'a, O> {
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OCV_A::_1)
    }
}
#[doc = "Field `CH2OCV` reader - Channel 2 Software Output Control Value"]
pub type CH2OCV_R = crate::BitReader<CH2OCV_A>;
#[doc = "Channel 2 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH2OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OCV_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2OCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OCV_A {
        match self.bits {
            false => CH2OCV_A::_0,
            true => CH2OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2OCV_A::_1
    }
}
#[doc = "Field `CH2OCV` writer - Channel 2 Software Output Control Value"]
pub type CH2OCV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH2OCV_A, O>;
impl<'a, const O: u8> CH2OCV_W<'a, O> {
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OCV_A::_1)
    }
}
#[doc = "Field `CH3OCV` reader - Channel 3 Software Output Control Value"]
pub type CH3OCV_R = crate::BitReader<CH3OCV_A>;
#[doc = "Channel 3 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH3OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OCV_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3OCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OCV_A {
        match self.bits {
            false => CH3OCV_A::_0,
            true => CH3OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3OCV_A::_1
    }
}
#[doc = "Field `CH3OCV` writer - Channel 3 Software Output Control Value"]
pub type CH3OCV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH3OCV_A, O>;
impl<'a, const O: u8> CH3OCV_W<'a, O> {
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OCV_A::_1)
    }
}
#[doc = "Field `CH4OCV` reader - Channel 4 Software Output Control Value"]
pub type CH4OCV_R = crate::BitReader<CH4OCV_A>;
#[doc = "Channel 4 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH4OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OCV_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4OCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OCV_A {
        match self.bits {
            false => CH4OCV_A::_0,
            true => CH4OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4OCV_A::_1
    }
}
#[doc = "Field `CH4OCV` writer - Channel 4 Software Output Control Value"]
pub type CH4OCV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH4OCV_A, O>;
impl<'a, const O: u8> CH4OCV_W<'a, O> {
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OCV_A::_1)
    }
}
#[doc = "Field `CH5OCV` reader - Channel 5 Software Output Control Value"]
pub type CH5OCV_R = crate::BitReader<CH5OCV_A>;
#[doc = "Channel 5 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH5OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OCV_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5OCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OCV_A {
        match self.bits {
            false => CH5OCV_A::_0,
            true => CH5OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5OCV_A::_1
    }
}
#[doc = "Field `CH5OCV` writer - Channel 5 Software Output Control Value"]
pub type CH5OCV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH5OCV_A, O>;
impl<'a, const O: u8> CH5OCV_W<'a, O> {
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OCV_A::_1)
    }
}
#[doc = "Field `CH6OCV` reader - Channel 6 Software Output Control Value"]
pub type CH6OCV_R = crate::BitReader<CH6OCV_A>;
#[doc = "Channel 6 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH6OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OCV_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6OCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OCV_A {
        match self.bits {
            false => CH6OCV_A::_0,
            true => CH6OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6OCV_A::_1
    }
}
#[doc = "Field `CH6OCV` writer - Channel 6 Software Output Control Value"]
pub type CH6OCV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH6OCV_A, O>;
impl<'a, const O: u8> CH6OCV_W<'a, O> {
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OCV_A::_1)
    }
}
#[doc = "Field `CH7OCV` reader - Channel 7 Software Output Control Value"]
pub type CH7OCV_R = crate::BitReader<CH7OCV_A>;
#[doc = "Channel 7 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH7OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OCV_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7OCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OCV_A {
        match self.bits {
            false => CH7OCV_A::_0,
            true => CH7OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7OCV_A::_1
    }
}
#[doc = "Field `CH7OCV` writer - Channel 7 Software Output Control Value"]
pub type CH7OCV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWOCTRL_SPEC, CH7OCV_A, O>;
impl<'a, const O: u8> CH7OCV_W<'a, O> {
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OCV_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch0oc(&self) -> CH0OC_R {
        CH0OC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch1oc(&self) -> CH1OC_R {
        CH1OC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch2oc(&self) -> CH2OC_R {
        CH2OC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch3oc(&self) -> CH3OC_R {
        CH3OC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch4oc(&self) -> CH4OC_R {
        CH4OC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch5oc(&self) -> CH5OC_R {
        CH5OC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch6oc(&self) -> CH6OC_R {
        CH6OC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch7oc(&self) -> CH7OC_R {
        CH7OC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Software Output Control Value"]
    #[inline(always)]
    pub fn ch0ocv(&self) -> CH0OCV_R {
        CH0OCV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Software Output Control Value"]
    #[inline(always)]
    pub fn ch1ocv(&self) -> CH1OCV_R {
        CH1OCV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Software Output Control Value"]
    #[inline(always)]
    pub fn ch2ocv(&self) -> CH2OCV_R {
        CH2OCV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Software Output Control Value"]
    #[inline(always)]
    pub fn ch3ocv(&self) -> CH3OCV_R {
        CH3OCV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Software Output Control Value"]
    #[inline(always)]
    pub fn ch4ocv(&self) -> CH4OCV_R {
        CH4OCV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Software Output Control Value"]
    #[inline(always)]
    pub fn ch5ocv(&self) -> CH5OCV_R {
        CH5OCV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Software Output Control Value"]
    #[inline(always)]
    pub fn ch6ocv(&self) -> CH6OCV_R {
        CH6OCV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Software Output Control Value"]
    #[inline(always)]
    pub fn ch7ocv(&self) -> CH7OCV_R {
        CH7OCV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Output Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0oc(&mut self) -> CH0OC_W<0> {
        CH0OC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Software Output Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1oc(&mut self) -> CH1OC_W<1> {
        CH1OC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Software Output Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2oc(&mut self) -> CH2OC_W<2> {
        CH2OC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Software Output Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3oc(&mut self) -> CH3OC_W<3> {
        CH3OC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Software Output Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4oc(&mut self) -> CH4OC_W<4> {
        CH4OC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Software Output Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5oc(&mut self) -> CH5OC_W<5> {
        CH5OC_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Software Output Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6oc(&mut self) -> CH6OC_W<6> {
        CH6OC_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Software Output Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7oc(&mut self) -> CH7OC_W<7> {
        CH7OC_W::new(self)
    }
    #[doc = "Bit 8 - Channel 0 Software Output Control Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ocv(&mut self) -> CH0OCV_W<8> {
        CH0OCV_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 Software Output Control Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ocv(&mut self) -> CH1OCV_W<9> {
        CH1OCV_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 Software Output Control Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ocv(&mut self) -> CH2OCV_W<10> {
        CH2OCV_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 Software Output Control Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ocv(&mut self) -> CH3OCV_W<11> {
        CH3OCV_W::new(self)
    }
    #[doc = "Bit 12 - Channel 4 Software Output Control Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch4ocv(&mut self) -> CH4OCV_W<12> {
        CH4OCV_W::new(self)
    }
    #[doc = "Bit 13 - Channel 5 Software Output Control Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch5ocv(&mut self) -> CH5OCV_W<13> {
        CH5OCV_W::new(self)
    }
    #[doc = "Bit 14 - Channel 6 Software Output Control Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch6ocv(&mut self) -> CH6OCV_W<14> {
        CH6OCV_W::new(self)
    }
    #[doc = "Bit 15 - Channel 7 Software Output Control Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch7ocv(&mut self) -> CH7OCV_W<15> {
        CH7OCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FTM Software Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swoctrl](index.html) module"]
pub struct SWOCTRL_SPEC;
impl crate::RegisterSpec for SWOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swoctrl::R](R) reader structure"]
impl crate::Readable for SWOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swoctrl::W](W) writer structure"]
impl crate::Writable for SWOCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWOCTRL to value 0"]
impl crate::Resettable for SWOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

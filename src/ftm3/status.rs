#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0F` reader - Channel 0 Flag"]
pub type CH0F_R = crate::BitReader<CH0F_A>;
#[doc = "Channel 0 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH0F_A> for bool {
    #[inline(always)]
    fn from(variant: CH0F_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0F_A {
        match self.bits {
            false => CH0F_A::_0,
            true => CH0F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0F_A::_1
    }
}
#[doc = "Field `CH0F` writer - Channel 0 Flag"]
pub type CH0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, CH0F_A, O>;
impl<'a, const O: u8> CH0F_W<'a, O> {
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0F_A::_1)
    }
}
#[doc = "Field `CH1F` reader - Channel 1 Flag"]
pub type CH1F_R = crate::BitReader<CH1F_A>;
#[doc = "Channel 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH1F_A> for bool {
    #[inline(always)]
    fn from(variant: CH1F_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1F_A {
        match self.bits {
            false => CH1F_A::_0,
            true => CH1F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1F_A::_1
    }
}
#[doc = "Field `CH1F` writer - Channel 1 Flag"]
pub type CH1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, CH1F_A, O>;
impl<'a, const O: u8> CH1F_W<'a, O> {
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1F_A::_1)
    }
}
#[doc = "Field `CH2F` reader - Channel 2 Flag"]
pub type CH2F_R = crate::BitReader<CH2F_A>;
#[doc = "Channel 2 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH2F_A> for bool {
    #[inline(always)]
    fn from(variant: CH2F_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2F_A {
        match self.bits {
            false => CH2F_A::_0,
            true => CH2F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2F_A::_1
    }
}
#[doc = "Field `CH2F` writer - Channel 2 Flag"]
pub type CH2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, CH2F_A, O>;
impl<'a, const O: u8> CH2F_W<'a, O> {
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2F_A::_1)
    }
}
#[doc = "Field `CH3F` reader - Channel 3 Flag"]
pub type CH3F_R = crate::BitReader<CH3F_A>;
#[doc = "Channel 3 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH3F_A> for bool {
    #[inline(always)]
    fn from(variant: CH3F_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3F_A {
        match self.bits {
            false => CH3F_A::_0,
            true => CH3F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3F_A::_1
    }
}
#[doc = "Field `CH3F` writer - Channel 3 Flag"]
pub type CH3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, CH3F_A, O>;
impl<'a, const O: u8> CH3F_W<'a, O> {
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3F_A::_1)
    }
}
#[doc = "Field `CH4F` reader - Channel 4 Flag"]
pub type CH4F_R = crate::BitReader<CH4F_A>;
#[doc = "Channel 4 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH4F_A> for bool {
    #[inline(always)]
    fn from(variant: CH4F_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4F_A {
        match self.bits {
            false => CH4F_A::_0,
            true => CH4F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4F_A::_1
    }
}
#[doc = "Field `CH4F` writer - Channel 4 Flag"]
pub type CH4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, CH4F_A, O>;
impl<'a, const O: u8> CH4F_W<'a, O> {
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4F_A::_1)
    }
}
#[doc = "Field `CH5F` reader - Channel 5 Flag"]
pub type CH5F_R = crate::BitReader<CH5F_A>;
#[doc = "Channel 5 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH5F_A> for bool {
    #[inline(always)]
    fn from(variant: CH5F_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5F_A {
        match self.bits {
            false => CH5F_A::_0,
            true => CH5F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5F_A::_1
    }
}
#[doc = "Field `CH5F` writer - Channel 5 Flag"]
pub type CH5F_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, CH5F_A, O>;
impl<'a, const O: u8> CH5F_W<'a, O> {
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5F_A::_1)
    }
}
#[doc = "Field `CH6F` reader - Channel 6 Flag"]
pub type CH6F_R = crate::BitReader<CH6F_A>;
#[doc = "Channel 6 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH6F_A> for bool {
    #[inline(always)]
    fn from(variant: CH6F_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6F_A {
        match self.bits {
            false => CH6F_A::_0,
            true => CH6F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6F_A::_1
    }
}
#[doc = "Field `CH6F` writer - Channel 6 Flag"]
pub type CH6F_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, CH6F_A, O>;
impl<'a, const O: u8> CH6F_W<'a, O> {
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6F_A::_1)
    }
}
#[doc = "Field `CH7F` reader - Channel 7 Flag"]
pub type CH7F_R = crate::BitReader<CH7F_A>;
#[doc = "Channel 7 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH7F_A> for bool {
    #[inline(always)]
    fn from(variant: CH7F_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7F_A {
        match self.bits {
            false => CH7F_A::_0,
            true => CH7F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7F_A::_1
    }
}
#[doc = "Field `CH7F` writer - Channel 7 Flag"]
pub type CH7F_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, CH7F_A, O>;
impl<'a, const O: u8> CH7F_W<'a, O> {
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7F_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Flag"]
    #[inline(always)]
    pub fn ch0f(&self) -> CH0F_R {
        CH0F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Flag"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Flag"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Flag"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Flag"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Flag"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Flag"]
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Flag"]
    #[inline(always)]
    pub fn ch7f(&self) -> CH7F_R {
        CH7F_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0f(&mut self) -> CH0F_W<0> {
        CH0F_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1f(&mut self) -> CH1F_W<1> {
        CH1F_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2f(&mut self) -> CH2F_W<2> {
        CH2F_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3f(&mut self) -> CH3F_W<3> {
        CH3F_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch4f(&mut self) -> CH4F_W<4> {
        CH4F_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch5f(&mut self) -> CH5F_W<5> {
        CH5F_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch6f(&mut self) -> CH6F_W<6> {
        CH6F_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch7f(&mut self) -> CH7F_W<7> {
        CH7F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture And Compare Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

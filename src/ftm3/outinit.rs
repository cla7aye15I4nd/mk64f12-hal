#[doc = "Register `OUTINIT` reader"]
pub struct R(crate::R<OUTINIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTINIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTINIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTINIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTINIT` writer"]
pub struct W(crate::W<OUTINIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTINIT_SPEC>;
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
impl From<crate::W<OUTINIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTINIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0OI` reader - Channel 0 Output Initialization Value"]
pub type CH0OI_R = crate::BitReader<CH0OI_A>;
#[doc = "Channel 0 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH0OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OI_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0OI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OI_A {
        match self.bits {
            false => CH0OI_A::_0,
            true => CH0OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0OI_A::_1
    }
}
#[doc = "Field `CH0OI` writer - Channel 0 Output Initialization Value"]
pub type CH0OI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTINIT_SPEC, CH0OI_A, O>;
impl<'a, const O: u8> CH0OI_W<'a, O> {
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OI_A::_1)
    }
}
#[doc = "Field `CH1OI` reader - Channel 1 Output Initialization Value"]
pub type CH1OI_R = crate::BitReader<CH1OI_A>;
#[doc = "Channel 1 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH1OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OI_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1OI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OI_A {
        match self.bits {
            false => CH1OI_A::_0,
            true => CH1OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1OI_A::_1
    }
}
#[doc = "Field `CH1OI` writer - Channel 1 Output Initialization Value"]
pub type CH1OI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTINIT_SPEC, CH1OI_A, O>;
impl<'a, const O: u8> CH1OI_W<'a, O> {
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OI_A::_1)
    }
}
#[doc = "Field `CH2OI` reader - Channel 2 Output Initialization Value"]
pub type CH2OI_R = crate::BitReader<CH2OI_A>;
#[doc = "Channel 2 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH2OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OI_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2OI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OI_A {
        match self.bits {
            false => CH2OI_A::_0,
            true => CH2OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2OI_A::_1
    }
}
#[doc = "Field `CH2OI` writer - Channel 2 Output Initialization Value"]
pub type CH2OI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTINIT_SPEC, CH2OI_A, O>;
impl<'a, const O: u8> CH2OI_W<'a, O> {
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OI_A::_1)
    }
}
#[doc = "Field `CH3OI` reader - Channel 3 Output Initialization Value"]
pub type CH3OI_R = crate::BitReader<CH3OI_A>;
#[doc = "Channel 3 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH3OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OI_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3OI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OI_A {
        match self.bits {
            false => CH3OI_A::_0,
            true => CH3OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3OI_A::_1
    }
}
#[doc = "Field `CH3OI` writer - Channel 3 Output Initialization Value"]
pub type CH3OI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTINIT_SPEC, CH3OI_A, O>;
impl<'a, const O: u8> CH3OI_W<'a, O> {
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OI_A::_1)
    }
}
#[doc = "Field `CH4OI` reader - Channel 4 Output Initialization Value"]
pub type CH4OI_R = crate::BitReader<CH4OI_A>;
#[doc = "Channel 4 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH4OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OI_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4OI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OI_A {
        match self.bits {
            false => CH4OI_A::_0,
            true => CH4OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4OI_A::_1
    }
}
#[doc = "Field `CH4OI` writer - Channel 4 Output Initialization Value"]
pub type CH4OI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTINIT_SPEC, CH4OI_A, O>;
impl<'a, const O: u8> CH4OI_W<'a, O> {
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OI_A::_1)
    }
}
#[doc = "Field `CH5OI` reader - Channel 5 Output Initialization Value"]
pub type CH5OI_R = crate::BitReader<CH5OI_A>;
#[doc = "Channel 5 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH5OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OI_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5OI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OI_A {
        match self.bits {
            false => CH5OI_A::_0,
            true => CH5OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5OI_A::_1
    }
}
#[doc = "Field `CH5OI` writer - Channel 5 Output Initialization Value"]
pub type CH5OI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTINIT_SPEC, CH5OI_A, O>;
impl<'a, const O: u8> CH5OI_W<'a, O> {
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OI_A::_1)
    }
}
#[doc = "Field `CH6OI` reader - Channel 6 Output Initialization Value"]
pub type CH6OI_R = crate::BitReader<CH6OI_A>;
#[doc = "Channel 6 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH6OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OI_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6OI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OI_A {
        match self.bits {
            false => CH6OI_A::_0,
            true => CH6OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6OI_A::_1
    }
}
#[doc = "Field `CH6OI` writer - Channel 6 Output Initialization Value"]
pub type CH6OI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTINIT_SPEC, CH6OI_A, O>;
impl<'a, const O: u8> CH6OI_W<'a, O> {
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OI_A::_1)
    }
}
#[doc = "Field `CH7OI` reader - Channel 7 Output Initialization Value"]
pub type CH7OI_R = crate::BitReader<CH7OI_A>;
#[doc = "Channel 7 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH7OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OI_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7OI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OI_A {
        match self.bits {
            false => CH7OI_A::_0,
            true => CH7OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7OI_A::_1
    }
}
#[doc = "Field `CH7OI` writer - Channel 7 Output Initialization Value"]
pub type CH7OI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTINIT_SPEC, CH7OI_A, O>;
impl<'a, const O: u8> CH7OI_W<'a, O> {
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OI_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Output Initialization Value"]
    #[inline(always)]
    pub fn ch0oi(&self) -> CH0OI_R {
        CH0OI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Output Initialization Value"]
    #[inline(always)]
    pub fn ch1oi(&self) -> CH1OI_R {
        CH1OI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Output Initialization Value"]
    #[inline(always)]
    pub fn ch2oi(&self) -> CH2OI_R {
        CH2OI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Output Initialization Value"]
    #[inline(always)]
    pub fn ch3oi(&self) -> CH3OI_R {
        CH3OI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Output Initialization Value"]
    #[inline(always)]
    pub fn ch4oi(&self) -> CH4OI_R {
        CH4OI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Output Initialization Value"]
    #[inline(always)]
    pub fn ch5oi(&self) -> CH5OI_R {
        CH5OI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Output Initialization Value"]
    #[inline(always)]
    pub fn ch6oi(&self) -> CH6OI_R {
        CH6OI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Output Initialization Value"]
    #[inline(always)]
    pub fn ch7oi(&self) -> CH7OI_R {
        CH7OI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Output Initialization Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch0oi(&mut self) -> CH0OI_W<0> {
        CH0OI_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Output Initialization Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch1oi(&mut self) -> CH1OI_W<1> {
        CH1OI_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Output Initialization Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch2oi(&mut self) -> CH2OI_W<2> {
        CH2OI_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Output Initialization Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch3oi(&mut self) -> CH3OI_W<3> {
        CH3OI_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Output Initialization Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch4oi(&mut self) -> CH4OI_W<4> {
        CH4OI_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Output Initialization Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch5oi(&mut self) -> CH5OI_W<5> {
        CH5OI_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Output Initialization Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch6oi(&mut self) -> CH6OI_W<6> {
        CH6OI_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Output Initialization Value"]
    #[inline(always)]
    #[must_use]
    pub fn ch7oi(&mut self) -> CH7OI_W<7> {
        CH7OI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initial State For Channels Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outinit](index.html) module"]
pub struct OUTINIT_SPEC;
impl crate::RegisterSpec for OUTINIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outinit::R](R) reader structure"]
impl crate::Readable for OUTINIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outinit::W](W) writer structure"]
impl crate::Writable for OUTINIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTINIT to value 0"]
impl crate::Resettable for OUTINIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

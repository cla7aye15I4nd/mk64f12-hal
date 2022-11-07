#[doc = "Register `OUTMASK` reader"]
pub struct R(crate::R<OUTMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTMASK` writer"]
pub struct W(crate::W<OUTMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTMASK_SPEC>;
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
impl From<crate::W<OUTMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0OM` reader - Channel 0 Output Mask"]
pub type CH0OM_R = crate::BitReader<CH0OM_A>;
#[doc = "Channel 0 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH0OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OM_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0OM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OM_A {
        match self.bits {
            false => CH0OM_A::_0,
            true => CH0OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0OM_A::_1
    }
}
#[doc = "Field `CH0OM` writer - Channel 0 Output Mask"]
pub type CH0OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTMASK_SPEC, CH0OM_A, O>;
impl<'a, const O: u8> CH0OM_W<'a, O> {
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OM_A::_1)
    }
}
#[doc = "Field `CH1OM` reader - Channel 1 Output Mask"]
pub type CH1OM_R = crate::BitReader<CH1OM_A>;
#[doc = "Channel 1 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH1OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OM_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1OM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OM_A {
        match self.bits {
            false => CH1OM_A::_0,
            true => CH1OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1OM_A::_1
    }
}
#[doc = "Field `CH1OM` writer - Channel 1 Output Mask"]
pub type CH1OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTMASK_SPEC, CH1OM_A, O>;
impl<'a, const O: u8> CH1OM_W<'a, O> {
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OM_A::_1)
    }
}
#[doc = "Field `CH2OM` reader - Channel 2 Output Mask"]
pub type CH2OM_R = crate::BitReader<CH2OM_A>;
#[doc = "Channel 2 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH2OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OM_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2OM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OM_A {
        match self.bits {
            false => CH2OM_A::_0,
            true => CH2OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2OM_A::_1
    }
}
#[doc = "Field `CH2OM` writer - Channel 2 Output Mask"]
pub type CH2OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTMASK_SPEC, CH2OM_A, O>;
impl<'a, const O: u8> CH2OM_W<'a, O> {
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OM_A::_1)
    }
}
#[doc = "Field `CH3OM` reader - Channel 3 Output Mask"]
pub type CH3OM_R = crate::BitReader<CH3OM_A>;
#[doc = "Channel 3 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH3OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OM_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3OM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OM_A {
        match self.bits {
            false => CH3OM_A::_0,
            true => CH3OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3OM_A::_1
    }
}
#[doc = "Field `CH3OM` writer - Channel 3 Output Mask"]
pub type CH3OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTMASK_SPEC, CH3OM_A, O>;
impl<'a, const O: u8> CH3OM_W<'a, O> {
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OM_A::_1)
    }
}
#[doc = "Field `CH4OM` reader - Channel 4 Output Mask"]
pub type CH4OM_R = crate::BitReader<CH4OM_A>;
#[doc = "Channel 4 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH4OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OM_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4OM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OM_A {
        match self.bits {
            false => CH4OM_A::_0,
            true => CH4OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4OM_A::_1
    }
}
#[doc = "Field `CH4OM` writer - Channel 4 Output Mask"]
pub type CH4OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTMASK_SPEC, CH4OM_A, O>;
impl<'a, const O: u8> CH4OM_W<'a, O> {
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OM_A::_1)
    }
}
#[doc = "Field `CH5OM` reader - Channel 5 Output Mask"]
pub type CH5OM_R = crate::BitReader<CH5OM_A>;
#[doc = "Channel 5 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH5OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OM_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5OM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OM_A {
        match self.bits {
            false => CH5OM_A::_0,
            true => CH5OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5OM_A::_1
    }
}
#[doc = "Field `CH5OM` writer - Channel 5 Output Mask"]
pub type CH5OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTMASK_SPEC, CH5OM_A, O>;
impl<'a, const O: u8> CH5OM_W<'a, O> {
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OM_A::_1)
    }
}
#[doc = "Field `CH6OM` reader - Channel 6 Output Mask"]
pub type CH6OM_R = crate::BitReader<CH6OM_A>;
#[doc = "Channel 6 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH6OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OM_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6OM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OM_A {
        match self.bits {
            false => CH6OM_A::_0,
            true => CH6OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6OM_A::_1
    }
}
#[doc = "Field `CH6OM` writer - Channel 6 Output Mask"]
pub type CH6OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTMASK_SPEC, CH6OM_A, O>;
impl<'a, const O: u8> CH6OM_W<'a, O> {
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OM_A::_1)
    }
}
#[doc = "Field `CH7OM` reader - Channel 7 Output Mask"]
pub type CH7OM_R = crate::BitReader<CH7OM_A>;
#[doc = "Channel 7 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH7OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OM_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7OM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OM_A {
        match self.bits {
            false => CH7OM_A::_0,
            true => CH7OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7OM_A::_1
    }
}
#[doc = "Field `CH7OM` writer - Channel 7 Output Mask"]
pub type CH7OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTMASK_SPEC, CH7OM_A, O>;
impl<'a, const O: u8> CH7OM_W<'a, O> {
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OM_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Output Mask"]
    #[inline(always)]
    pub fn ch0om(&self) -> CH0OM_R {
        CH0OM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Output Mask"]
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Output Mask"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Output Mask"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Output Mask"]
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Output Mask"]
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Output Mask"]
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Output Mask"]
    #[inline(always)]
    pub fn ch7om(&self) -> CH7OM_R {
        CH7OM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Output Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch0om(&mut self) -> CH0OM_W<0> {
        CH0OM_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Output Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch1om(&mut self) -> CH1OM_W<1> {
        CH1OM_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Output Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch2om(&mut self) -> CH2OM_W<2> {
        CH2OM_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Output Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch3om(&mut self) -> CH3OM_W<3> {
        CH3OM_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Output Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch4om(&mut self) -> CH4OM_W<4> {
        CH4OM_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Output Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch5om(&mut self) -> CH5OM_W<5> {
        CH5OM_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Output Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch6om(&mut self) -> CH6OM_W<6> {
        CH6OM_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Output Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch7om(&mut self) -> CH7OM_W<7> {
        CH7OM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outmask](index.html) module"]
pub struct OUTMASK_SPEC;
impl crate::RegisterSpec for OUTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outmask::R](R) reader structure"]
impl crate::Readable for OUTMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outmask::W](W) writer structure"]
impl crate::Writable for OUTMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTMASK to value 0"]
impl crate::Resettable for OUTMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

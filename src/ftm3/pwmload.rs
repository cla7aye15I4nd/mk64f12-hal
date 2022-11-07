#[doc = "Register `PWMLOAD` reader"]
pub struct R(crate::R<PWMLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMLOAD` writer"]
pub struct W(crate::W<PWMLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMLOAD_SPEC>;
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
impl From<crate::W<PWMLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0SEL` reader - Channel 0 Select"]
pub type CH0SEL_R = crate::BitReader<CH0SEL_A>;
#[doc = "Channel 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0SEL_A {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<CH0SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH0SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0SEL_A {
        match self.bits {
            false => CH0SEL_A::_0,
            true => CH0SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0SEL_A::_1
    }
}
#[doc = "Field `CH0SEL` writer - Channel 0 Select"]
pub type CH0SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMLOAD_SPEC, CH0SEL_A, O>;
impl<'a, const O: u8> CH0SEL_W<'a, O> {
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0SEL_A::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0SEL_A::_1)
    }
}
#[doc = "Field `CH1SEL` reader - Channel 1 Select"]
pub type CH1SEL_R = crate::BitReader<CH1SEL_A>;
#[doc = "Channel 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1SEL_A {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<CH1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH1SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1SEL_A {
        match self.bits {
            false => CH1SEL_A::_0,
            true => CH1SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1SEL_A::_1
    }
}
#[doc = "Field `CH1SEL` writer - Channel 1 Select"]
pub type CH1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMLOAD_SPEC, CH1SEL_A, O>;
impl<'a, const O: u8> CH1SEL_W<'a, O> {
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1SEL_A::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1SEL_A::_1)
    }
}
#[doc = "Field `CH2SEL` reader - Channel 2 Select"]
pub type CH2SEL_R = crate::BitReader<CH2SEL_A>;
#[doc = "Channel 2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2SEL_A {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<CH2SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH2SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2SEL_A {
        match self.bits {
            false => CH2SEL_A::_0,
            true => CH2SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2SEL_A::_1
    }
}
#[doc = "Field `CH2SEL` writer - Channel 2 Select"]
pub type CH2SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMLOAD_SPEC, CH2SEL_A, O>;
impl<'a, const O: u8> CH2SEL_W<'a, O> {
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2SEL_A::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2SEL_A::_1)
    }
}
#[doc = "Field `CH3SEL` reader - Channel 3 Select"]
pub type CH3SEL_R = crate::BitReader<CH3SEL_A>;
#[doc = "Channel 3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3SEL_A {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<CH3SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH3SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3SEL_A {
        match self.bits {
            false => CH3SEL_A::_0,
            true => CH3SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3SEL_A::_1
    }
}
#[doc = "Field `CH3SEL` writer - Channel 3 Select"]
pub type CH3SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMLOAD_SPEC, CH3SEL_A, O>;
impl<'a, const O: u8> CH3SEL_W<'a, O> {
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3SEL_A::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3SEL_A::_1)
    }
}
#[doc = "Field `CH4SEL` reader - Channel 4 Select"]
pub type CH4SEL_R = crate::BitReader<CH4SEL_A>;
#[doc = "Channel 4 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4SEL_A {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<CH4SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH4SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4SEL_A {
        match self.bits {
            false => CH4SEL_A::_0,
            true => CH4SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4SEL_A::_1
    }
}
#[doc = "Field `CH4SEL` writer - Channel 4 Select"]
pub type CH4SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMLOAD_SPEC, CH4SEL_A, O>;
impl<'a, const O: u8> CH4SEL_W<'a, O> {
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4SEL_A::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4SEL_A::_1)
    }
}
#[doc = "Field `CH5SEL` reader - Channel 5 Select"]
pub type CH5SEL_R = crate::BitReader<CH5SEL_A>;
#[doc = "Channel 5 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5SEL_A {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<CH5SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH5SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5SEL_A {
        match self.bits {
            false => CH5SEL_A::_0,
            true => CH5SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5SEL_A::_1
    }
}
#[doc = "Field `CH5SEL` writer - Channel 5 Select"]
pub type CH5SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMLOAD_SPEC, CH5SEL_A, O>;
impl<'a, const O: u8> CH5SEL_W<'a, O> {
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5SEL_A::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5SEL_A::_1)
    }
}
#[doc = "Field `CH6SEL` reader - Channel 6 Select"]
pub type CH6SEL_R = crate::BitReader<CH6SEL_A>;
#[doc = "Channel 6 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6SEL_A {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<CH6SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH6SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6SEL_A {
        match self.bits {
            false => CH6SEL_A::_0,
            true => CH6SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6SEL_A::_1
    }
}
#[doc = "Field `CH6SEL` writer - Channel 6 Select"]
pub type CH6SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMLOAD_SPEC, CH6SEL_A, O>;
impl<'a, const O: u8> CH6SEL_W<'a, O> {
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6SEL_A::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6SEL_A::_1)
    }
}
#[doc = "Field `CH7SEL` reader - Channel 7 Select"]
pub type CH7SEL_R = crate::BitReader<CH7SEL_A>;
#[doc = "Channel 7 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7SEL_A {
    #[doc = "0: Do not include the channel in the matching process."]
    _0 = 0,
    #[doc = "1: Include the channel in the matching process."]
    _1 = 1,
}
impl From<CH7SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH7SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7SEL_A {
        match self.bits {
            false => CH7SEL_A::_0,
            true => CH7SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7SEL_A::_1
    }
}
#[doc = "Field `CH7SEL` writer - Channel 7 Select"]
pub type CH7SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMLOAD_SPEC, CH7SEL_A, O>;
impl<'a, const O: u8> CH7SEL_W<'a, O> {
    #[doc = "Do not include the channel in the matching process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7SEL_A::_0)
    }
    #[doc = "Include the channel in the matching process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7SEL_A::_1)
    }
}
#[doc = "Field `LDOK` reader - Load Enable"]
pub type LDOK_R = crate::BitReader<LDOK_A>;
#[doc = "Load Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDOK_A {
    #[doc = "0: Loading updated values is disabled."]
    _0 = 0,
    #[doc = "1: Loading updated values is enabled."]
    _1 = 1,
}
impl From<LDOK_A> for bool {
    #[inline(always)]
    fn from(variant: LDOK_A) -> Self {
        variant as u8 != 0
    }
}
impl LDOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOK_A {
        match self.bits {
            false => LDOK_A::_0,
            true => LDOK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDOK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDOK_A::_1
    }
}
#[doc = "Field `LDOK` writer - Load Enable"]
pub type LDOK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMLOAD_SPEC, LDOK_A, O>;
impl<'a, const O: u8> LDOK_W<'a, O> {
    #[doc = "Loading updated values is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDOK_A::_0)
    }
    #[doc = "Loading updated values is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDOK_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Select"]
    #[inline(always)]
    pub fn ch0sel(&self) -> CH0SEL_R {
        CH0SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Select"]
    #[inline(always)]
    pub fn ch1sel(&self) -> CH1SEL_R {
        CH1SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Select"]
    #[inline(always)]
    pub fn ch2sel(&self) -> CH2SEL_R {
        CH2SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Select"]
    #[inline(always)]
    pub fn ch3sel(&self) -> CH3SEL_R {
        CH3SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Select"]
    #[inline(always)]
    pub fn ch4sel(&self) -> CH4SEL_R {
        CH4SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Select"]
    #[inline(always)]
    pub fn ch5sel(&self) -> CH5SEL_R {
        CH5SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Select"]
    #[inline(always)]
    pub fn ch6sel(&self) -> CH6SEL_R {
        CH6SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Select"]
    #[inline(always)]
    pub fn ch7sel(&self) -> CH7SEL_R {
        CH7SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Load Enable"]
    #[inline(always)]
    pub fn ldok(&self) -> LDOK_R {
        LDOK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch0sel(&mut self) -> CH0SEL_W<0> {
        CH0SEL_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch1sel(&mut self) -> CH1SEL_W<1> {
        CH1SEL_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch2sel(&mut self) -> CH2SEL_W<2> {
        CH2SEL_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch3sel(&mut self) -> CH3SEL_W<3> {
        CH3SEL_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch4sel(&mut self) -> CH4SEL_W<4> {
        CH4SEL_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch5sel(&mut self) -> CH5SEL_W<5> {
        CH5SEL_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch6sel(&mut self) -> CH6SEL_W<6> {
        CH6SEL_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch7sel(&mut self) -> CH7SEL_W<7> {
        CH7SEL_W::new(self)
    }
    #[doc = "Bit 9 - Load Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ldok(&mut self) -> LDOK_W<9> {
        LDOK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FTM PWM Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmload](index.html) module"]
pub struct PWMLOAD_SPEC;
impl crate::RegisterSpec for PWMLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmload::R](R) reader structure"]
impl crate::Readable for PWMLOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmload::W](W) writer structure"]
impl crate::Writable for PWMLOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWMLOAD to value 0"]
impl crate::Resettable for PWMLOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

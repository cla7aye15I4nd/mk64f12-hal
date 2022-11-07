#[doc = "Register `FLT` reader"]
pub struct R(crate::R<FLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT` writer"]
pub struct W(crate::W<FLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT_SPEC>;
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
impl From<crate::W<FLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT` reader - I2C Programmable Filter Factor"]
pub type FLT_R = crate::FieldReader<u8, FLT_A>;
#[doc = "I2C Programmable Filter Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLT_A {
    #[doc = "0: No filter/bypass"]
    _0 = 0,
}
impl From<FLT_A> for u8 {
    #[inline(always)]
    fn from(variant: FLT_A) -> Self {
        variant as _
    }
}
impl FLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLT_A> {
        match self.bits {
            0 => Some(FLT_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT_A::_0
    }
}
#[doc = "Field `FLT` writer - I2C Programmable Filter Factor"]
pub type FLT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FLT_SPEC, u8, FLT_A, 4, O>;
impl<'a, const O: u8> FLT_W<'a, O> {
    #[doc = "No filter/bypass"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT_A::_0)
    }
}
#[doc = "Field `STARTF` reader - I2C Bus Start Detect Flag"]
pub type STARTF_R = crate::BitReader<STARTF_A>;
#[doc = "I2C Bus Start Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTF_A {
    #[doc = "0: No start happens on I2C bus"]
    _0 = 0,
    #[doc = "1: Start detected on I2C bus"]
    _1 = 1,
}
impl From<STARTF_A> for bool {
    #[inline(always)]
    fn from(variant: STARTF_A) -> Self {
        variant as u8 != 0
    }
}
impl STARTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTF_A {
        match self.bits {
            false => STARTF_A::_0,
            true => STARTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STARTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STARTF_A::_1
    }
}
#[doc = "Field `STARTF` writer - I2C Bus Start Detect Flag"]
pub type STARTF_W<'a, const O: u8> = crate::BitWriter<'a, u8, FLT_SPEC, STARTF_A, O>;
impl<'a, const O: u8> STARTF_W<'a, O> {
    #[doc = "No start happens on I2C bus"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARTF_A::_0)
    }
    #[doc = "Start detected on I2C bus"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STARTF_A::_1)
    }
}
#[doc = "Field `SSIE` reader - I2C Bus Stop or Start Interrupt Enable"]
pub type SSIE_R = crate::BitReader<SSIE_A>;
#[doc = "I2C Bus Stop or Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSIE_A {
    #[doc = "0: Stop or start detection interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Stop or start detection interrupt is enabled"]
    _1 = 1,
}
impl From<SSIE_A> for bool {
    #[inline(always)]
    fn from(variant: SSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIE_A {
        match self.bits {
            false => SSIE_A::_0,
            true => SSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSIE_A::_1
    }
}
#[doc = "Field `SSIE` writer - I2C Bus Stop or Start Interrupt Enable"]
pub type SSIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, FLT_SPEC, SSIE_A, O>;
impl<'a, const O: u8> SSIE_W<'a, O> {
    #[doc = "Stop or start detection interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSIE_A::_0)
    }
    #[doc = "Stop or start detection interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSIE_A::_1)
    }
}
#[doc = "Field `STOPF` reader - I2C Bus Stop Detect Flag"]
pub type STOPF_R = crate::BitReader<STOPF_A>;
#[doc = "I2C Bus Stop Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPF_A {
    #[doc = "0: No stop happens on I2C bus"]
    _0 = 0,
    #[doc = "1: Stop detected on I2C bus"]
    _1 = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::_0,
            true => STOPF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOPF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOPF_A::_1
    }
}
#[doc = "Field `STOPF` writer - I2C Bus Stop Detect Flag"]
pub type STOPF_W<'a, const O: u8> = crate::BitWriter<'a, u8, FLT_SPEC, STOPF_A, O>;
impl<'a, const O: u8> STOPF_W<'a, O> {
    #[doc = "No stop happens on I2C bus"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPF_A::_0)
    }
    #[doc = "Stop detected on I2C bus"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPF_A::_1)
    }
}
#[doc = "Field `SHEN` reader - Stop Hold Enable"]
pub type SHEN_R = crate::BitReader<SHEN_A>;
#[doc = "Stop Hold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHEN_A {
    #[doc = "0: Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    _0 = 0,
    #[doc = "1: Stop holdoff is enabled."]
    _1 = 1,
}
impl From<SHEN_A> for bool {
    #[inline(always)]
    fn from(variant: SHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHEN_A {
        match self.bits {
            false => SHEN_A::_0,
            true => SHEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHEN_A::_1
    }
}
#[doc = "Field `SHEN` writer - Stop Hold Enable"]
pub type SHEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, FLT_SPEC, SHEN_A, O>;
impl<'a, const O: u8> SHEN_W<'a, O> {
    #[doc = "Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHEN_A::_0)
    }
    #[doc = "Stop holdoff is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - I2C Programmable Filter Factor"]
    #[inline(always)]
    pub fn flt(&self) -> FLT_R {
        FLT_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - I2C Bus Start Detect Flag"]
    #[inline(always)]
    pub fn startf(&self) -> STARTF_R {
        STARTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Bus Stop or Start Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&self) -> SSIE_R {
        SSIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Bus Stop Detect Flag"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stop Hold Enable"]
    #[inline(always)]
    pub fn shen(&self) -> SHEN_R {
        SHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - I2C Programmable Filter Factor"]
    #[inline(always)]
    #[must_use]
    pub fn flt(&mut self) -> FLT_W<0> {
        FLT_W::new(self)
    }
    #[doc = "Bit 4 - I2C Bus Start Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn startf(&mut self) -> STARTF_W<4> {
        STARTF_W::new(self)
    }
    #[doc = "Bit 5 - I2C Bus Stop or Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssie(&mut self) -> SSIE_W<5> {
        SSIE_W::new(self)
    }
    #[doc = "Bit 6 - I2C Bus Stop Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stopf(&mut self) -> STOPF_W<6> {
        STOPF_W::new(self)
    }
    #[doc = "Bit 7 - Stop Hold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shen(&mut self) -> SHEN_W<7> {
        SHEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Programmable Input Glitch Filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt](index.html) module"]
pub struct FLT_SPEC;
impl crate::RegisterSpec for FLT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [flt::R](R) reader structure"]
impl crate::Readable for FLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt::W](W) writer structure"]
impl crate::Writable for FLT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLT to value 0"]
impl crate::Resettable for FLT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

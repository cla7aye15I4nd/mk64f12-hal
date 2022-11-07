#[doc = "Register `F` reader"]
pub struct R(crate::R<F_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F` writer"]
pub struct W(crate::W<F_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F_SPEC>;
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
impl From<crate::W<F_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICR` reader - ClockRate"]
pub type ICR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICR` writer - ClockRate"]
pub type ICR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, F_SPEC, u8, u8, 6, O>;
#[doc = "Field `MULT` reader - Multiplier Factor"]
pub type MULT_R = crate::FieldReader<u8, MULT_A>;
#[doc = "Multiplier Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULT_A {
    #[doc = "0: mul = 1"]
    _00 = 0,
    #[doc = "1: mul = 2"]
    _01 = 1,
    #[doc = "2: mul = 4"]
    _10 = 2,
}
impl From<MULT_A> for u8 {
    #[inline(always)]
    fn from(variant: MULT_A) -> Self {
        variant as _
    }
}
impl MULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MULT_A> {
        match self.bits {
            0 => Some(MULT_A::_00),
            1 => Some(MULT_A::_01),
            2 => Some(MULT_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MULT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MULT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MULT_A::_10
    }
}
#[doc = "Field `MULT` writer - Multiplier Factor"]
pub type MULT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, F_SPEC, u8, MULT_A, 2, O>;
impl<'a, const O: u8> MULT_W<'a, O> {
    #[doc = "mul = 1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MULT_A::_00)
    }
    #[doc = "mul = 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MULT_A::_01)
    }
    #[doc = "mul = 4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MULT_A::_10)
    }
}
impl R {
    #[doc = "Bits 0:5 - ClockRate"]
    #[inline(always)]
    pub fn icr(&self) -> ICR_R {
        ICR_R::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - Multiplier Factor"]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - ClockRate"]
    #[inline(always)]
    #[must_use]
    pub fn icr(&mut self) -> ICR_W<0> {
        ICR_W::new(self)
    }
    #[doc = "Bits 6:7 - Multiplier Factor"]
    #[inline(always)]
    #[must_use]
    pub fn mult(&mut self) -> MULT_W<6> {
        MULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Frequency Divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f](index.html) module"]
pub struct F_SPEC;
impl crate::RegisterSpec for F_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [f::R](R) reader structure"]
impl crate::Readable for F_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f::W](W) writer structure"]
impl crate::Writable for F_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets F to value 0"]
impl crate::Resettable for F_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

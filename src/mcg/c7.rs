#[doc = "Register `C7` reader"]
pub struct R(crate::R<C7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C7` writer"]
pub struct W(crate::W<C7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C7_SPEC>;
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
impl From<crate::W<C7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSCSEL` reader - MCG OSC Clock Select"]
pub type OSCSEL_R = crate::FieldReader<u8, OSCSEL_A>;
#[doc = "MCG OSC Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSCSEL_A {
    #[doc = "0: Selects Oscillator (OSCCLK0)."]
    _00 = 0,
    #[doc = "1: Selects 32 kHz RTC Oscillator."]
    _01 = 1,
    #[doc = "2: Selects Oscillator (OSCCLK1)."]
    _10 = 2,
}
impl From<OSCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSCSEL_A) -> Self {
        variant as _
    }
}
impl OSCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSCSEL_A> {
        match self.bits {
            0 => Some(OSCSEL_A::_00),
            1 => Some(OSCSEL_A::_01),
            2 => Some(OSCSEL_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OSCSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OSCSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OSCSEL_A::_10
    }
}
#[doc = "Field `OSCSEL` writer - MCG OSC Clock Select"]
pub type OSCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, C7_SPEC, u8, OSCSEL_A, 2, O>;
impl<'a, const O: u8> OSCSEL_W<'a, O> {
    #[doc = "Selects Oscillator (OSCCLK0)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSCSEL_A::_00)
    }
    #[doc = "Selects 32 kHz RTC Oscillator."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OSCSEL_A::_01)
    }
    #[doc = "Selects Oscillator (OSCCLK1)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSCSEL_A::_10)
    }
}
impl R {
    #[doc = "Bits 0:1 - MCG OSC Clock Select"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - MCG OSC Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn oscsel(&mut self) -> OSCSEL_W<0> {
        OSCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7](index.html) module"]
pub struct C7_SPEC;
impl crate::RegisterSpec for C7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c7::R](R) reader structure"]
impl crate::Readable for C7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c7::W](W) writer structure"]
impl crate::Writable for C7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C7 to value 0"]
impl crate::Resettable for C7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

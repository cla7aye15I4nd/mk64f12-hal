#[doc = "Register `TFLG%s` reader"]
pub struct R(crate::R<TFLG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFLG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFLG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFLG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFLG%s` writer"]
pub struct W(crate::W<TFLG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFLG_SPEC>;
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
impl From<crate::W<TFLG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFLG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIF` reader - Timer Interrupt Flag"]
pub type TIF_R = crate::BitReader<TIF_A>;
#[doc = "Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIF_A {
    #[doc = "0: Timeout has not yet occurred."]
    _0 = 0,
    #[doc = "1: Timeout has occurred."]
    _1 = 1,
}
impl From<TIF_A> for bool {
    #[inline(always)]
    fn from(variant: TIF_A) -> Self {
        variant as u8 != 0
    }
}
impl TIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF_A {
        match self.bits {
            false => TIF_A::_0,
            true => TIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIF_A::_1
    }
}
#[doc = "Field `TIF` writer - Timer Interrupt Flag"]
pub type TIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TFLG_SPEC, TIF_A, O>;
impl<'a, const O: u8> TIF_W<'a, O> {
    #[doc = "Timeout has not yet occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIF_A::_0)
    }
    #[doc = "Timeout has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<0> {
        TIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tflg](index.html) module"]
pub struct TFLG_SPEC;
impl crate::RegisterSpec for TFLG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tflg::R](R) reader structure"]
impl crate::Readable for TFLG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tflg::W](W) writer structure"]
impl crate::Writable for TFLG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFLG%s to value 0"]
impl crate::Resettable for TFLG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

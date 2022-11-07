#[doc = "Register `TRM` reader"]
pub struct R(crate::R<TRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRM` writer"]
pub struct W(crate::W<TRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRM_SPEC>;
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
impl From<crate::W<TRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - Trim bits"]
pub type TRIM_R = crate::FieldReader<u8, TRIM_A>;
#[doc = "Trim bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIM_A {
    #[doc = "0: Min"]
    _000000 = 0,
    #[doc = "63: Max"]
    _111111 = 63,
}
impl From<TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIM_A) -> Self {
        variant as _
    }
}
impl TRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIM_A> {
        match self.bits {
            0 => Some(TRIM_A::_000000),
            63 => Some(TRIM_A::_111111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline(always)]
    pub fn is_000000(&self) -> bool {
        *self == TRIM_A::_000000
    }
    #[doc = "Checks if the value of the field is `_111111`"]
    #[inline(always)]
    pub fn is_111111(&self) -> bool {
        *self == TRIM_A::_111111
    }
}
#[doc = "Field `TRIM` writer - Trim bits"]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TRM_SPEC, u8, TRIM_A, 6, O>;
impl<'a, const O: u8> TRIM_W<'a, O> {
    #[doc = "Min"]
    #[inline(always)]
    pub fn _000000(self) -> &'a mut W {
        self.variant(TRIM_A::_000000)
    }
    #[doc = "Max"]
    #[inline(always)]
    pub fn _111111(self) -> &'a mut W {
        self.variant(TRIM_A::_111111)
    }
}
#[doc = "Field `CHOPEN` reader - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
pub type CHOPEN_R = crate::BitReader<CHOPEN_A>;
#[doc = "Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHOPEN_A {
    #[doc = "0: Chop oscillator is disabled."]
    _0 = 0,
    #[doc = "1: Chop oscillator is enabled."]
    _1 = 1,
}
impl From<CHOPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHOPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHOPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHOPEN_A {
        match self.bits {
            false => CHOPEN_A::_0,
            true => CHOPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHOPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHOPEN_A::_1
    }
}
#[doc = "Field `CHOPEN` writer - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
pub type CHOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, TRM_SPEC, CHOPEN_A, O>;
impl<'a, const O: u8> CHOPEN_W<'a, O> {
    #[doc = "Chop oscillator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHOPEN_A::_0)
    }
    #[doc = "Chop oscillator is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHOPEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - Trim bits"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
    #[inline(always)]
    pub fn chopen(&self) -> CHOPEN_R {
        CHOPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trim bits"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<0> {
        TRIM_W::new(self)
    }
    #[doc = "Bit 6 - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
    #[inline(always)]
    #[must_use]
    pub fn chopen(&mut self) -> CHOPEN_W<6> {
        CHOPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trm](index.html) module"]
pub struct TRM_SPEC;
impl crate::RegisterSpec for TRM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [trm::R](R) reader structure"]
impl crate::Readable for TRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trm::W](W) writer structure"]
impl crate::Writable for TRM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRM to value 0"]
impl crate::Resettable for TRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

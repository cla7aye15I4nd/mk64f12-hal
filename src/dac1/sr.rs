#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACBFRPBF` reader - DAC Buffer Read Pointer Bottom Position Flag"]
pub type DACBFRPBF_R = crate::BitReader<DACBFRPBF_A>;
#[doc = "DAC Buffer Read Pointer Bottom Position Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACBFRPBF_A {
    #[doc = "0: The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    _0 = 0,
    #[doc = "1: The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    _1 = 1,
}
impl From<DACBFRPBF_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFRPBF_A) -> Self {
        variant as u8 != 0
    }
}
impl DACBFRPBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFRPBF_A {
        match self.bits {
            false => DACBFRPBF_A::_0,
            true => DACBFRPBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBFRPBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBFRPBF_A::_1
    }
}
#[doc = "Field `DACBFRPBF` writer - DAC Buffer Read Pointer Bottom Position Flag"]
pub type DACBFRPBF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SR_SPEC, DACBFRPBF_A, O>;
impl<'a, const O: u8> DACBFRPBF_W<'a, O> {
    #[doc = "The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFRPBF_A::_0)
    }
    #[doc = "The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFRPBF_A::_1)
    }
}
#[doc = "Field `DACBFRPTF` reader - DAC Buffer Read Pointer Top Position Flag"]
pub type DACBFRPTF_R = crate::BitReader<DACBFRPTF_A>;
#[doc = "DAC Buffer Read Pointer Top Position Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACBFRPTF_A {
    #[doc = "0: The DAC buffer read pointer is not zero."]
    _0 = 0,
    #[doc = "1: The DAC buffer read pointer is zero."]
    _1 = 1,
}
impl From<DACBFRPTF_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFRPTF_A) -> Self {
        variant as u8 != 0
    }
}
impl DACBFRPTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFRPTF_A {
        match self.bits {
            false => DACBFRPTF_A::_0,
            true => DACBFRPTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBFRPTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBFRPTF_A::_1
    }
}
#[doc = "Field `DACBFRPTF` writer - DAC Buffer Read Pointer Top Position Flag"]
pub type DACBFRPTF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SR_SPEC, DACBFRPTF_A, O>;
impl<'a, const O: u8> DACBFRPTF_W<'a, O> {
    #[doc = "The DAC buffer read pointer is not zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFRPTF_A::_0)
    }
    #[doc = "The DAC buffer read pointer is zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFRPTF_A::_1)
    }
}
#[doc = "Field `DACBFWMF` reader - DAC Buffer Watermark Flag"]
pub type DACBFWMF_R = crate::BitReader<DACBFWMF_A>;
#[doc = "DAC Buffer Watermark Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACBFWMF_A {
    #[doc = "0: The DAC buffer read pointer has not reached the watermark level."]
    _0 = 0,
    #[doc = "1: The DAC buffer read pointer has reached the watermark level."]
    _1 = 1,
}
impl From<DACBFWMF_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFWMF_A) -> Self {
        variant as u8 != 0
    }
}
impl DACBFWMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFWMF_A {
        match self.bits {
            false => DACBFWMF_A::_0,
            true => DACBFWMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBFWMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBFWMF_A::_1
    }
}
#[doc = "Field `DACBFWMF` writer - DAC Buffer Watermark Flag"]
pub type DACBFWMF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SR_SPEC, DACBFWMF_A, O>;
impl<'a, const O: u8> DACBFWMF_W<'a, O> {
    #[doc = "The DAC buffer read pointer has not reached the watermark level."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFWMF_A::_0)
    }
    #[doc = "The DAC buffer read pointer has reached the watermark level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFWMF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Position Flag"]
    #[inline(always)]
    pub fn dacbfrpbf(&self) -> DACBFRPBF_R {
        DACBFRPBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Position Flag"]
    #[inline(always)]
    pub fn dacbfrptf(&self) -> DACBFRPTF_R {
        DACBFRPTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Flag"]
    #[inline(always)]
    pub fn dacbfwmf(&self) -> DACBFWMF_R {
        DACBFWMF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Position Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfrpbf(&mut self) -> DACBFRPBF_W<0> {
        DACBFRPBF_W::new(self)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Position Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfrptf(&mut self) -> DACBFRPTF_W<1> {
        DACBFRPTF_W::new(self)
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfwmf(&mut self) -> DACBFWMF_W<2> {
        DACBFWMF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0x02"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}

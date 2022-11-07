#[doc = "Register `RST` reader"]
pub struct R(crate::R<RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST` writer"]
pub struct W(crate::W<RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_SPEC>;
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
impl From<crate::W<RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTFILT` reader - Digital Filter On RESET Pin"]
pub type RSTFILT_R = crate::BitReader<RSTFILT_A>;
#[doc = "Digital Filter On RESET Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTFILT_A {
    #[doc = "0: Filter not enabled"]
    _0 = 0,
    #[doc = "1: Filter enabled"]
    _1 = 1,
}
impl From<RSTFILT_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFILT_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTFILT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFILT_A {
        match self.bits {
            false => RSTFILT_A::_0,
            true => RSTFILT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSTFILT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSTFILT_A::_1
    }
}
#[doc = "Field `RSTFILT` writer - Digital Filter On RESET Pin"]
pub type RSTFILT_W<'a, const O: u8> = crate::BitWriter<'a, u8, RST_SPEC, RSTFILT_A, O>;
impl<'a, const O: u8> RSTFILT_W<'a, O> {
    #[doc = "Filter not enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTFILT_A::_0)
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTFILT_A::_1)
    }
}
#[doc = "Field `LLRSTE` reader - Low-Leakage Mode RESET Enable"]
pub type LLRSTE_R = crate::BitReader<LLRSTE_A>;
#[doc = "Low-Leakage Mode RESET Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LLRSTE_A {
    #[doc = "0: RESET pin not enabled as a leakage mode exit source"]
    _0 = 0,
    #[doc = "1: RESET pin enabled as a low leakage mode exit source"]
    _1 = 1,
}
impl From<LLRSTE_A> for bool {
    #[inline(always)]
    fn from(variant: LLRSTE_A) -> Self {
        variant as u8 != 0
    }
}
impl LLRSTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLRSTE_A {
        match self.bits {
            false => LLRSTE_A::_0,
            true => LLRSTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LLRSTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LLRSTE_A::_1
    }
}
#[doc = "Field `LLRSTE` writer - Low-Leakage Mode RESET Enable"]
pub type LLRSTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RST_SPEC, LLRSTE_A, O>;
impl<'a, const O: u8> LLRSTE_W<'a, O> {
    #[doc = "RESET pin not enabled as a leakage mode exit source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LLRSTE_A::_0)
    }
    #[doc = "RESET pin enabled as a low leakage mode exit source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LLRSTE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Digital Filter On RESET Pin"]
    #[inline(always)]
    pub fn rstfilt(&self) -> RSTFILT_R {
        RSTFILT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low-Leakage Mode RESET Enable"]
    #[inline(always)]
    pub fn llrste(&self) -> LLRSTE_R {
        LLRSTE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Digital Filter On RESET Pin"]
    #[inline(always)]
    #[must_use]
    pub fn rstfilt(&mut self) -> RSTFILT_W<0> {
        RSTFILT_W::new(self)
    }
    #[doc = "Bit 1 - Low-Leakage Mode RESET Enable"]
    #[inline(always)]
    #[must_use]
    pub fn llrste(&mut self) -> LLRSTE_W<1> {
        LLRSTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Reset Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst](index.html) module"]
pub struct RST_SPEC;
impl crate::RegisterSpec for RST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rst::R](R) reader structure"]
impl crate::Readable for RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst::W](W) writer structure"]
impl crate::Writable for RST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RST to value 0x02"]
impl crate::Resettable for RST_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}

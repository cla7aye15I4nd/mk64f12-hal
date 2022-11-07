#[doc = "Register `TCTRL%s` reader"]
pub struct R(crate::R<TCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCTRL%s` writer"]
pub struct W(crate::W<TCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCTRL_SPEC>;
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
impl From<crate::W<TCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEN` reader - Timer Enable"]
pub type TEN_R = crate::BitReader<TEN_A>;
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN_A {
    #[doc = "0: Timer n is disabled."]
    _0 = 0,
    #[doc = "1: Timer n is enabled."]
    _1 = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::_0,
            true => TEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEN_A::_1
    }
}
#[doc = "Field `TEN` writer - Timer Enable"]
pub type TEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCTRL_SPEC, TEN_A, O>;
impl<'a, const O: u8> TEN_W<'a, O> {
    #[doc = "Timer n is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEN_A::_0)
    }
    #[doc = "Timer n is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEN_A::_1)
    }
}
#[doc = "Field `TIE` reader - Timer Interrupt Enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: Interrupt requests from Timer n are disabled."]
    _0 = 0,
    #[doc = "1: Interrupt will be requested whenever TIF is set."]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Field `TIE` writer - Timer Interrupt Enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCTRL_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "Interrupt requests from Timer n are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Interrupt will be requested whenever TIF is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
    }
}
#[doc = "Field `CHN` reader - Chain Mode"]
pub type CHN_R = crate::BitReader<CHN_A>;
#[doc = "Chain Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHN_A {
    #[doc = "0: Timer is not chained."]
    _0 = 0,
    #[doc = "1: Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    _1 = 1,
}
impl From<CHN_A> for bool {
    #[inline(always)]
    fn from(variant: CHN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHN_A {
        match self.bits {
            false => CHN_A::_0,
            true => CHN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHN_A::_1
    }
}
#[doc = "Field `CHN` writer - Chain Mode"]
pub type CHN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCTRL_SPEC, CHN_A, O>;
impl<'a, const O: u8> CHN_W<'a, O> {
    #[doc = "Timer is not chained."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHN_A::_0)
    }
    #[doc = "Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chain Mode"]
    #[inline(always)]
    pub fn chn(&self) -> CHN_R {
        CHN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<0> {
        TEN_W::new(self)
    }
    #[doc = "Bit 1 - Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<1> {
        TIE_W::new(self)
    }
    #[doc = "Bit 2 - Chain Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chn(&mut self) -> CHN_W<2> {
        CHN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctrl](index.html) module"]
pub struct TCTRL_SPEC;
impl crate::RegisterSpec for TCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tctrl::R](R) reader structure"]
impl crate::Readable for TCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tctrl::W](W) writer structure"]
impl crate::Writable for TCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCTRL%s to value 0"]
impl crate::Resettable for TCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

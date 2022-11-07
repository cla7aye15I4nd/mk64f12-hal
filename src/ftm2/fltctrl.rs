#[doc = "Register `FLTCTRL` reader"]
pub struct R(crate::R<FLTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTCTRL` writer"]
pub struct W(crate::W<FLTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTCTRL_SPEC>;
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
impl From<crate::W<FLTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAULT0EN` reader - Fault Input 0 Enable"]
pub type FAULT0EN_R = crate::BitReader<FAULT0EN_A>;
#[doc = "Fault Input 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULT0EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT0EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT0EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULT0EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT0EN_A {
        match self.bits {
            false => FAULT0EN_A::_0,
            true => FAULT0EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULT0EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULT0EN_A::_1
    }
}
#[doc = "Field `FAULT0EN` writer - Fault Input 0 Enable"]
pub type FAULT0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTCTRL_SPEC, FAULT0EN_A, O>;
impl<'a, const O: u8> FAULT0EN_W<'a, O> {
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT0EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT0EN_A::_1)
    }
}
#[doc = "Field `FAULT1EN` reader - Fault Input 1 Enable"]
pub type FAULT1EN_R = crate::BitReader<FAULT1EN_A>;
#[doc = "Fault Input 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULT1EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT1EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULT1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT1EN_A {
        match self.bits {
            false => FAULT1EN_A::_0,
            true => FAULT1EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULT1EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULT1EN_A::_1
    }
}
#[doc = "Field `FAULT1EN` writer - Fault Input 1 Enable"]
pub type FAULT1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTCTRL_SPEC, FAULT1EN_A, O>;
impl<'a, const O: u8> FAULT1EN_W<'a, O> {
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT1EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT1EN_A::_1)
    }
}
#[doc = "Field `FAULT2EN` reader - Fault Input 2 Enable"]
pub type FAULT2EN_R = crate::BitReader<FAULT2EN_A>;
#[doc = "Fault Input 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULT2EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT2EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULT2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT2EN_A {
        match self.bits {
            false => FAULT2EN_A::_0,
            true => FAULT2EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULT2EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULT2EN_A::_1
    }
}
#[doc = "Field `FAULT2EN` writer - Fault Input 2 Enable"]
pub type FAULT2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTCTRL_SPEC, FAULT2EN_A, O>;
impl<'a, const O: u8> FAULT2EN_W<'a, O> {
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT2EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT2EN_A::_1)
    }
}
#[doc = "Field `FAULT3EN` reader - Fault Input 3 Enable"]
pub type FAULT3EN_R = crate::BitReader<FAULT3EN_A>;
#[doc = "Fault Input 3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULT3EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT3EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT3EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULT3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT3EN_A {
        match self.bits {
            false => FAULT3EN_A::_0,
            true => FAULT3EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULT3EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULT3EN_A::_1
    }
}
#[doc = "Field `FAULT3EN` writer - Fault Input 3 Enable"]
pub type FAULT3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTCTRL_SPEC, FAULT3EN_A, O>;
impl<'a, const O: u8> FAULT3EN_W<'a, O> {
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT3EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT3EN_A::_1)
    }
}
#[doc = "Field `FFLTR0EN` reader - Fault Input 0 Filter Enable"]
pub type FFLTR0EN_R = crate::BitReader<FFLTR0EN_A>;
#[doc = "Fault Input 0 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFLTR0EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR0EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR0EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FFLTR0EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR0EN_A {
        match self.bits {
            false => FFLTR0EN_A::_0,
            true => FFLTR0EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FFLTR0EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FFLTR0EN_A::_1
    }
}
#[doc = "Field `FFLTR0EN` writer - Fault Input 0 Filter Enable"]
pub type FFLTR0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTCTRL_SPEC, FFLTR0EN_A, O>;
impl<'a, const O: u8> FFLTR0EN_W<'a, O> {
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR0EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR0EN_A::_1)
    }
}
#[doc = "Field `FFLTR1EN` reader - Fault Input 1 Filter Enable"]
pub type FFLTR1EN_R = crate::BitReader<FFLTR1EN_A>;
#[doc = "Fault Input 1 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFLTR1EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR1EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FFLTR1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR1EN_A {
        match self.bits {
            false => FFLTR1EN_A::_0,
            true => FFLTR1EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FFLTR1EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FFLTR1EN_A::_1
    }
}
#[doc = "Field `FFLTR1EN` writer - Fault Input 1 Filter Enable"]
pub type FFLTR1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTCTRL_SPEC, FFLTR1EN_A, O>;
impl<'a, const O: u8> FFLTR1EN_W<'a, O> {
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR1EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR1EN_A::_1)
    }
}
#[doc = "Field `FFLTR2EN` reader - Fault Input 2 Filter Enable"]
pub type FFLTR2EN_R = crate::BitReader<FFLTR2EN_A>;
#[doc = "Fault Input 2 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFLTR2EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR2EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FFLTR2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR2EN_A {
        match self.bits {
            false => FFLTR2EN_A::_0,
            true => FFLTR2EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FFLTR2EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FFLTR2EN_A::_1
    }
}
#[doc = "Field `FFLTR2EN` writer - Fault Input 2 Filter Enable"]
pub type FFLTR2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTCTRL_SPEC, FFLTR2EN_A, O>;
impl<'a, const O: u8> FFLTR2EN_W<'a, O> {
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR2EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR2EN_A::_1)
    }
}
#[doc = "Field `FFLTR3EN` reader - Fault Input 3 Filter Enable"]
pub type FFLTR3EN_R = crate::BitReader<FFLTR3EN_A>;
#[doc = "Fault Input 3 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFLTR3EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR3EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR3EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FFLTR3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR3EN_A {
        match self.bits {
            false => FFLTR3EN_A::_0,
            true => FFLTR3EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FFLTR3EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FFLTR3EN_A::_1
    }
}
#[doc = "Field `FFLTR3EN` writer - Fault Input 3 Filter Enable"]
pub type FFLTR3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTCTRL_SPEC, FFLTR3EN_A, O>;
impl<'a, const O: u8> FFLTR3EN_W<'a, O> {
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR3EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR3EN_A::_1)
    }
}
#[doc = "Field `FFVAL` reader - Fault Input Filter"]
pub type FFVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FFVAL` writer - Fault Input Filter"]
pub type FFVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Fault Input 0 Enable"]
    #[inline(always)]
    pub fn fault0en(&self) -> FAULT0EN_R {
        FAULT0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Input 1 Enable"]
    #[inline(always)]
    pub fn fault1en(&self) -> FAULT1EN_R {
        FAULT1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Input 2 Enable"]
    #[inline(always)]
    pub fn fault2en(&self) -> FAULT2EN_R {
        FAULT2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Input 3 Enable"]
    #[inline(always)]
    pub fn fault3en(&self) -> FAULT3EN_R {
        FAULT3EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault Input 0 Filter Enable"]
    #[inline(always)]
    pub fn ffltr0en(&self) -> FFLTR0EN_R {
        FFLTR0EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault Input 1 Filter Enable"]
    #[inline(always)]
    pub fn ffltr1en(&self) -> FFLTR1EN_R {
        FFLTR1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault Input 2 Filter Enable"]
    #[inline(always)]
    pub fn ffltr2en(&self) -> FFLTR2EN_R {
        FFLTR2EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fault Input 3 Filter Enable"]
    #[inline(always)]
    pub fn ffltr3en(&self) -> FFLTR3EN_R {
        FFLTR3EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Fault Input Filter"]
    #[inline(always)]
    pub fn ffval(&self) -> FFVAL_R {
        FFVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Input 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault0en(&mut self) -> FAULT0EN_W<0> {
        FAULT0EN_W::new(self)
    }
    #[doc = "Bit 1 - Fault Input 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault1en(&mut self) -> FAULT1EN_W<1> {
        FAULT1EN_W::new(self)
    }
    #[doc = "Bit 2 - Fault Input 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault2en(&mut self) -> FAULT2EN_W<2> {
        FAULT2EN_W::new(self)
    }
    #[doc = "Bit 3 - Fault Input 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault3en(&mut self) -> FAULT3EN_W<3> {
        FAULT3EN_W::new(self)
    }
    #[doc = "Bit 4 - Fault Input 0 Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffltr0en(&mut self) -> FFLTR0EN_W<4> {
        FFLTR0EN_W::new(self)
    }
    #[doc = "Bit 5 - Fault Input 1 Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffltr1en(&mut self) -> FFLTR1EN_W<5> {
        FFLTR1EN_W::new(self)
    }
    #[doc = "Bit 6 - Fault Input 2 Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffltr2en(&mut self) -> FFLTR2EN_W<6> {
        FFLTR2EN_W::new(self)
    }
    #[doc = "Bit 7 - Fault Input 3 Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffltr3en(&mut self) -> FFLTR3EN_W<7> {
        FFLTR3EN_W::new(self)
    }
    #[doc = "Bits 8:11 - Fault Input Filter"]
    #[inline(always)]
    #[must_use]
    pub fn ffval(&mut self) -> FFVAL_W<8> {
        FFVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltctrl](index.html) module"]
pub struct FLTCTRL_SPEC;
impl crate::RegisterSpec for FLTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltctrl::R](R) reader structure"]
impl crate::Readable for FLTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltctrl::W](W) writer structure"]
impl crate::Writable for FLTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTCTRL to value 0"]
impl crate::Resettable for FLTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

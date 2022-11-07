#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIIE` reader - Time Invalid Interrupt Enable"]
pub type TIIE_R = crate::BitReader<TIIE_A>;
#[doc = "Time Invalid Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIIE_A {
    #[doc = "0: Time invalid flag does not generate an interrupt."]
    _0 = 0,
    #[doc = "1: Time invalid flag does generate an interrupt."]
    _1 = 1,
}
impl From<TIIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIIE_A {
        match self.bits {
            false => TIIE_A::_0,
            true => TIIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIIE_A::_1
    }
}
#[doc = "Field `TIIE` writer - Time Invalid Interrupt Enable"]
pub type TIIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TIIE_A, O>;
impl<'a, const O: u8> TIIE_W<'a, O> {
    #[doc = "Time invalid flag does not generate an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIIE_A::_0)
    }
    #[doc = "Time invalid flag does generate an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIIE_A::_1)
    }
}
#[doc = "Field `TOIE` reader - Time Overflow Interrupt Enable"]
pub type TOIE_R = crate::BitReader<TOIE_A>;
#[doc = "Time Overflow Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOIE_A {
    #[doc = "0: Time overflow flag does not generate an interrupt."]
    _0 = 0,
    #[doc = "1: Time overflow flag does generate an interrupt."]
    _1 = 1,
}
impl From<TOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOIE_A {
        match self.bits {
            false => TOIE_A::_0,
            true => TOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOIE_A::_1
    }
}
#[doc = "Field `TOIE` writer - Time Overflow Interrupt Enable"]
pub type TOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TOIE_A, O>;
impl<'a, const O: u8> TOIE_W<'a, O> {
    #[doc = "Time overflow flag does not generate an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIE_A::_0)
    }
    #[doc = "Time overflow flag does generate an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIE_A::_1)
    }
}
#[doc = "Field `TAIE` reader - Time Alarm Interrupt Enable"]
pub type TAIE_R = crate::BitReader<TAIE_A>;
#[doc = "Time Alarm Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAIE_A {
    #[doc = "0: Time alarm flag does not generate an interrupt."]
    _0 = 0,
    #[doc = "1: Time alarm flag does generate an interrupt."]
    _1 = 1,
}
impl From<TAIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIE_A {
        match self.bits {
            false => TAIE_A::_0,
            true => TAIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TAIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TAIE_A::_1
    }
}
#[doc = "Field `TAIE` writer - Time Alarm Interrupt Enable"]
pub type TAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TAIE_A, O>;
impl<'a, const O: u8> TAIE_W<'a, O> {
    #[doc = "Time alarm flag does not generate an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TAIE_A::_0)
    }
    #[doc = "Time alarm flag does generate an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TAIE_A::_1)
    }
}
#[doc = "Field `TSIE` reader - Time Seconds Interrupt Enable"]
pub type TSIE_R = crate::BitReader<TSIE_A>;
#[doc = "Time Seconds Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIE_A {
    #[doc = "0: Seconds interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Seconds interrupt is enabled."]
    _1 = 1,
}
impl From<TSIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIE_A {
        match self.bits {
            false => TSIE_A::_0,
            true => TSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSIE_A::_1
    }
}
#[doc = "Field `TSIE` writer - Time Seconds Interrupt Enable"]
pub type TSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TSIE_A, O>;
impl<'a, const O: u8> TSIE_W<'a, O> {
    #[doc = "Seconds interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIE_A::_0)
    }
    #[doc = "Seconds interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIE_A::_1)
    }
}
#[doc = "Field `WPON` reader - Wakeup Pin On"]
pub type WPON_R = crate::BitReader<WPON_A>;
#[doc = "Wakeup Pin On\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPON_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: If the wakeup pin is enabled, then the wakeup pin will assert."]
    _1 = 1,
}
impl From<WPON_A> for bool {
    #[inline(always)]
    fn from(variant: WPON_A) -> Self {
        variant as u8 != 0
    }
}
impl WPON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPON_A {
        match self.bits {
            false => WPON_A::_0,
            true => WPON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPON_A::_1
    }
}
#[doc = "Field `WPON` writer - Wakeup Pin On"]
pub type WPON_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, WPON_A, O>;
impl<'a, const O: u8> WPON_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPON_A::_0)
    }
    #[doc = "If the wakeup pin is enabled, then the wakeup pin will assert."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPON_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
    #[inline(always)]
    pub fn tiie(&self) -> TIIE_R {
        TIIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn taie(&self) -> TAIE_R {
        TAIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Pin On"]
    #[inline(always)]
    pub fn wpon(&self) -> WPON_R {
        WPON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tiie(&mut self) -> TIIE_W<0> {
        TIIE_W::new(self)
    }
    #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<1> {
        TOIE_W::new(self)
    }
    #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn taie(&mut self) -> TAIE_W<2> {
        TAIE_W::new(self)
    }
    #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<4> {
        TSIE_W::new(self)
    }
    #[doc = "Bit 7 - Wakeup Pin On"]
    #[inline(always)]
    #[must_use]
    pub fn wpon(&mut self) -> WPON_W<7> {
        WPON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0x07"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}

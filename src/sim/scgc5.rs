#[doc = "Register `SCGC5` reader"]
pub struct R(crate::R<SCGC5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGC5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGC5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGC5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGC5` writer"]
pub struct W(crate::W<SCGC5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGC5_SPEC>;
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
impl From<crate::W<SCGC5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGC5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTMR` reader - Low Power Timer Access Control"]
pub type LPTMR_R = crate::BitReader<LPTMR_A>;
#[doc = "Low Power Timer Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTMR_A {
    #[doc = "0: Access disabled"]
    _0 = 0,
    #[doc = "1: Access enabled"]
    _1 = 1,
}
impl From<LPTMR_A> for bool {
    #[inline(always)]
    fn from(variant: LPTMR_A) -> Self {
        variant as u8 != 0
    }
}
impl LPTMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTMR_A {
        match self.bits {
            false => LPTMR_A::_0,
            true => LPTMR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPTMR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPTMR_A::_1
    }
}
#[doc = "Field `LPTMR` writer - Low Power Timer Access Control"]
pub type LPTMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC5_SPEC, LPTMR_A, O>;
impl<'a, const O: u8> LPTMR_W<'a, O> {
    #[doc = "Access disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPTMR_A::_0)
    }
    #[doc = "Access enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPTMR_A::_1)
    }
}
#[doc = "Field `PORTA` reader - Port A Clock Gate Control"]
pub type PORTA_R = crate::BitReader<PORTA_A>;
#[doc = "Port A Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORTA_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTA_A> for bool {
    #[inline(always)]
    fn from(variant: PORTA_A) -> Self {
        variant as u8 != 0
    }
}
impl PORTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTA_A {
        match self.bits {
            false => PORTA_A::_0,
            true => PORTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTA_A::_1
    }
}
#[doc = "Field `PORTA` writer - Port A Clock Gate Control"]
pub type PORTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC5_SPEC, PORTA_A, O>;
impl<'a, const O: u8> PORTA_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTA_A::_1)
    }
}
#[doc = "Field `PORTB` reader - Port B Clock Gate Control"]
pub type PORTB_R = crate::BitReader<PORTB_A>;
#[doc = "Port B Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORTB_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTB_A> for bool {
    #[inline(always)]
    fn from(variant: PORTB_A) -> Self {
        variant as u8 != 0
    }
}
impl PORTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTB_A {
        match self.bits {
            false => PORTB_A::_0,
            true => PORTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTB_A::_1
    }
}
#[doc = "Field `PORTB` writer - Port B Clock Gate Control"]
pub type PORTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC5_SPEC, PORTB_A, O>;
impl<'a, const O: u8> PORTB_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTB_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTB_A::_1)
    }
}
#[doc = "Field `PORTC` reader - Port C Clock Gate Control"]
pub type PORTC_R = crate::BitReader<PORTC_A>;
#[doc = "Port C Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORTC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTC_A> for bool {
    #[inline(always)]
    fn from(variant: PORTC_A) -> Self {
        variant as u8 != 0
    }
}
impl PORTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTC_A {
        match self.bits {
            false => PORTC_A::_0,
            true => PORTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTC_A::_1
    }
}
#[doc = "Field `PORTC` writer - Port C Clock Gate Control"]
pub type PORTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC5_SPEC, PORTC_A, O>;
impl<'a, const O: u8> PORTC_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTC_A::_1)
    }
}
#[doc = "Field `PORTD` reader - Port D Clock Gate Control"]
pub type PORTD_R = crate::BitReader<PORTD_A>;
#[doc = "Port D Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORTD_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTD_A> for bool {
    #[inline(always)]
    fn from(variant: PORTD_A) -> Self {
        variant as u8 != 0
    }
}
impl PORTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTD_A {
        match self.bits {
            false => PORTD_A::_0,
            true => PORTD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTD_A::_1
    }
}
#[doc = "Field `PORTD` writer - Port D Clock Gate Control"]
pub type PORTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC5_SPEC, PORTD_A, O>;
impl<'a, const O: u8> PORTD_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTD_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTD_A::_1)
    }
}
#[doc = "Field `PORTE` reader - Port E Clock Gate Control"]
pub type PORTE_R = crate::BitReader<PORTE_A>;
#[doc = "Port E Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORTE_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTE_A> for bool {
    #[inline(always)]
    fn from(variant: PORTE_A) -> Self {
        variant as u8 != 0
    }
}
impl PORTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTE_A {
        match self.bits {
            false => PORTE_A::_0,
            true => PORTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTE_A::_1
    }
}
#[doc = "Field `PORTE` writer - Port E Clock Gate Control"]
pub type PORTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC5_SPEC, PORTE_A, O>;
impl<'a, const O: u8> PORTE_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTE_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Low Power Timer Access Control"]
    #[inline(always)]
    pub fn lptmr(&self) -> LPTMR_R {
        LPTMR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - Port A Clock Gate Control"]
    #[inline(always)]
    pub fn porta(&self) -> PORTA_R {
        PORTA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port B Clock Gate Control"]
    #[inline(always)]
    pub fn portb(&self) -> PORTB_R {
        PORTB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port C Clock Gate Control"]
    #[inline(always)]
    pub fn portc(&self) -> PORTC_R {
        PORTC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port D Clock Gate Control"]
    #[inline(always)]
    pub fn portd(&self) -> PORTD_R {
        PORTD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port E Clock Gate Control"]
    #[inline(always)]
    pub fn porte(&self) -> PORTE_R {
        PORTE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Timer Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn lptmr(&mut self) -> LPTMR_W<0> {
        LPTMR_W::new(self)
    }
    #[doc = "Bit 9 - Port A Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn porta(&mut self) -> PORTA_W<9> {
        PORTA_W::new(self)
    }
    #[doc = "Bit 10 - Port B Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn portb(&mut self) -> PORTB_W<10> {
        PORTB_W::new(self)
    }
    #[doc = "Bit 11 - Port C Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn portc(&mut self) -> PORTC_W<11> {
        PORTC_W::new(self)
    }
    #[doc = "Bit 12 - Port D Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn portd(&mut self) -> PORTD_W<12> {
        PORTD_W::new(self)
    }
    #[doc = "Bit 13 - Port E Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn porte(&mut self) -> PORTE_W<13> {
        PORTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Gating Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc5](index.html) module"]
pub struct SCGC5_SPEC;
impl crate::RegisterSpec for SCGC5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgc5::R](R) reader structure"]
impl crate::Readable for SCGC5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgc5::W](W) writer structure"]
impl crate::Writable for SCGC5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGC5 to value 0x0004_0182"]
impl crate::Resettable for SCGC5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0182;
}

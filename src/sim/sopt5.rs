#[doc = "Register `SOPT5` reader"]
pub struct R(crate::R<SOPT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT5` writer"]
pub struct W(crate::W<SOPT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT5_SPEC>;
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
impl From<crate::W<SOPT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART0TXSRC` reader - UART 0 transmit data source select"]
pub type UART0TXSRC_R = crate::FieldReader<u8, UART0TXSRC_A>;
#[doc = "UART 0 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART0TXSRC_A {
    #[doc = "0: UART0_TX pin"]
    _00 = 0,
    #[doc = "1: UART0_TX pin modulated with FTM1 channel 0 output"]
    _01 = 1,
    #[doc = "2: UART0_TX pin modulated with FTM2 channel 0 output"]
    _10 = 2,
}
impl From<UART0TXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART0TXSRC_A) -> Self {
        variant as _
    }
}
impl UART0TXSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART0TXSRC_A> {
        match self.bits {
            0 => Some(UART0TXSRC_A::_00),
            1 => Some(UART0TXSRC_A::_01),
            2 => Some(UART0TXSRC_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UART0TXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UART0TXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UART0TXSRC_A::_10
    }
}
#[doc = "Field `UART0TXSRC` writer - UART 0 transmit data source select"]
pub type UART0TXSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT5_SPEC, u8, UART0TXSRC_A, 2, O>;
impl<'a, const O: u8> UART0TXSRC_W<'a, O> {
    #[doc = "UART0_TX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART0TXSRC_A::_00)
    }
    #[doc = "UART0_TX pin modulated with FTM1 channel 0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART0TXSRC_A::_01)
    }
    #[doc = "UART0_TX pin modulated with FTM2 channel 0 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART0TXSRC_A::_10)
    }
}
#[doc = "Field `UART0RXSRC` reader - UART 0 receive data source select"]
pub type UART0RXSRC_R = crate::FieldReader<u8, UART0RXSRC_A>;
#[doc = "UART 0 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART0RXSRC_A {
    #[doc = "0: UART0_RX pin"]
    _00 = 0,
    #[doc = "1: CMP0"]
    _01 = 1,
    #[doc = "2: CMP1"]
    _10 = 2,
}
impl From<UART0RXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART0RXSRC_A) -> Self {
        variant as _
    }
}
impl UART0RXSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART0RXSRC_A> {
        match self.bits {
            0 => Some(UART0RXSRC_A::_00),
            1 => Some(UART0RXSRC_A::_01),
            2 => Some(UART0RXSRC_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UART0RXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UART0RXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UART0RXSRC_A::_10
    }
}
#[doc = "Field `UART0RXSRC` writer - UART 0 receive data source select"]
pub type UART0RXSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT5_SPEC, u8, UART0RXSRC_A, 2, O>;
impl<'a, const O: u8> UART0RXSRC_W<'a, O> {
    #[doc = "UART0_RX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART0RXSRC_A::_00)
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART0RXSRC_A::_01)
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART0RXSRC_A::_10)
    }
}
#[doc = "Field `UART1TXSRC` reader - UART 1 transmit data source select"]
pub type UART1TXSRC_R = crate::FieldReader<u8, UART1TXSRC_A>;
#[doc = "UART 1 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART1TXSRC_A {
    #[doc = "0: UART1_TX pin"]
    _00 = 0,
    #[doc = "1: UART1_TX pin modulated with FTM1 channel 0 output"]
    _01 = 1,
    #[doc = "2: UART1_TX pin modulated with FTM2 channel 0 output"]
    _10 = 2,
}
impl From<UART1TXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART1TXSRC_A) -> Self {
        variant as _
    }
}
impl UART1TXSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART1TXSRC_A> {
        match self.bits {
            0 => Some(UART1TXSRC_A::_00),
            1 => Some(UART1TXSRC_A::_01),
            2 => Some(UART1TXSRC_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UART1TXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UART1TXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UART1TXSRC_A::_10
    }
}
#[doc = "Field `UART1TXSRC` writer - UART 1 transmit data source select"]
pub type UART1TXSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT5_SPEC, u8, UART1TXSRC_A, 2, O>;
impl<'a, const O: u8> UART1TXSRC_W<'a, O> {
    #[doc = "UART1_TX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART1TXSRC_A::_00)
    }
    #[doc = "UART1_TX pin modulated with FTM1 channel 0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART1TXSRC_A::_01)
    }
    #[doc = "UART1_TX pin modulated with FTM2 channel 0 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART1TXSRC_A::_10)
    }
}
#[doc = "Field `UART1RXSRC` reader - UART 1 receive data source select"]
pub type UART1RXSRC_R = crate::FieldReader<u8, UART1RXSRC_A>;
#[doc = "UART 1 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART1RXSRC_A {
    #[doc = "0: UART1_RX pin"]
    _00 = 0,
    #[doc = "1: CMP0"]
    _01 = 1,
    #[doc = "2: CMP1"]
    _10 = 2,
}
impl From<UART1RXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART1RXSRC_A) -> Self {
        variant as _
    }
}
impl UART1RXSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART1RXSRC_A> {
        match self.bits {
            0 => Some(UART1RXSRC_A::_00),
            1 => Some(UART1RXSRC_A::_01),
            2 => Some(UART1RXSRC_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UART1RXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UART1RXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UART1RXSRC_A::_10
    }
}
#[doc = "Field `UART1RXSRC` writer - UART 1 receive data source select"]
pub type UART1RXSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT5_SPEC, u8, UART1RXSRC_A, 2, O>;
impl<'a, const O: u8> UART1RXSRC_W<'a, O> {
    #[doc = "UART1_RX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART1RXSRC_A::_00)
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART1RXSRC_A::_01)
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART1RXSRC_A::_10)
    }
}
impl R {
    #[doc = "Bits 0:1 - UART 0 transmit data source select"]
    #[inline(always)]
    pub fn uart0txsrc(&self) -> UART0TXSRC_R {
        UART0TXSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - UART 0 receive data source select"]
    #[inline(always)]
    pub fn uart0rxsrc(&self) -> UART0RXSRC_R {
        UART0RXSRC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - UART 1 transmit data source select"]
    #[inline(always)]
    pub fn uart1txsrc(&self) -> UART1TXSRC_R {
        UART1TXSRC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - UART 1 receive data source select"]
    #[inline(always)]
    pub fn uart1rxsrc(&self) -> UART1RXSRC_R {
        UART1RXSRC_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART 0 transmit data source select"]
    #[inline(always)]
    #[must_use]
    pub fn uart0txsrc(&mut self) -> UART0TXSRC_W<0> {
        UART0TXSRC_W::new(self)
    }
    #[doc = "Bits 2:3 - UART 0 receive data source select"]
    #[inline(always)]
    #[must_use]
    pub fn uart0rxsrc(&mut self) -> UART0RXSRC_W<2> {
        UART0RXSRC_W::new(self)
    }
    #[doc = "Bits 4:5 - UART 1 transmit data source select"]
    #[inline(always)]
    #[must_use]
    pub fn uart1txsrc(&mut self) -> UART1TXSRC_W<4> {
        UART1TXSRC_W::new(self)
    }
    #[doc = "Bits 6:7 - UART 1 receive data source select"]
    #[inline(always)]
    #[must_use]
    pub fn uart1rxsrc(&mut self) -> UART1RXSRC_W<6> {
        UART1RXSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt5](index.html) module"]
pub struct SOPT5_SPEC;
impl crate::RegisterSpec for SOPT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt5::R](R) reader structure"]
impl crate::Readable for SOPT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt5::W](W) writer structure"]
impl crate::Writable for SOPT5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOPT5 to value 0"]
impl crate::Resettable for SOPT5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

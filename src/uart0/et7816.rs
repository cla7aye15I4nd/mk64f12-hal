#[doc = "Register `ET7816` reader"]
pub struct R(crate::R<ET7816_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ET7816_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ET7816_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ET7816_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ET7816` writer"]
pub struct W(crate::W<ET7816_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ET7816_SPEC>;
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
impl From<crate::W<ET7816_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ET7816_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXTHRESHOLD` reader - Receive NACK Threshold"]
pub type RXTHRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXTHRESHOLD` writer - Receive NACK Threshold"]
pub type RXTHRESHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ET7816_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXTHRESHOLD` reader - Transmit NACK Threshold"]
pub type TXTHRESHOLD_R = crate::FieldReader<u8, TXTHRESHOLD_A>;
#[doc = "Transmit NACK Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXTHRESHOLD_A {
    #[doc = "0: TXT asserts on the first NACK that is received."]
    _0 = 0,
    #[doc = "1: TXT asserts on the second NACK that is received."]
    _1 = 1,
}
impl From<TXTHRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: TXTHRESHOLD_A) -> Self {
        variant as _
    }
}
impl TXTHRESHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXTHRESHOLD_A> {
        match self.bits {
            0 => Some(TXTHRESHOLD_A::_0),
            1 => Some(TXTHRESHOLD_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXTHRESHOLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXTHRESHOLD_A::_1
    }
}
#[doc = "Field `TXTHRESHOLD` writer - Transmit NACK Threshold"]
pub type TXTHRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ET7816_SPEC, u8, TXTHRESHOLD_A, 4, O>;
impl<'a, const O: u8> TXTHRESHOLD_W<'a, O> {
    #[doc = "TXT asserts on the first NACK that is received."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXTHRESHOLD_A::_0)
    }
    #[doc = "TXT asserts on the second NACK that is received."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXTHRESHOLD_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Receive NACK Threshold"]
    #[inline(always)]
    pub fn rxthreshold(&self) -> RXTHRESHOLD_R {
        RXTHRESHOLD_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Transmit NACK Threshold"]
    #[inline(always)]
    pub fn txthreshold(&self) -> TXTHRESHOLD_R {
        TXTHRESHOLD_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive NACK Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rxthreshold(&mut self) -> RXTHRESHOLD_W<0> {
        RXTHRESHOLD_W::new(self)
    }
    #[doc = "Bits 4:7 - Transmit NACK Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn txthreshold(&mut self) -> TXTHRESHOLD_W<4> {
        TXTHRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 7816 Error Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [et7816](index.html) module"]
pub struct ET7816_SPEC;
impl crate::RegisterSpec for ET7816_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [et7816::R](R) reader structure"]
impl crate::Readable for ET7816_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [et7816::W](W) writer structure"]
impl crate::Writable for ET7816_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ET7816 to value 0"]
impl crate::Resettable for ET7816_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

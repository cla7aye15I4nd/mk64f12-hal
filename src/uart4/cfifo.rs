#[doc = "Register `CFIFO` reader"]
pub struct R(crate::R<CFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFIFO` writer"]
pub struct W(crate::W<CFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFIFO_SPEC>;
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
impl From<crate::W<CFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXUFE` reader - Receive FIFO Underflow Interrupt Enable"]
pub type RXUFE_R = crate::BitReader<RXUFE_A>;
#[doc = "Receive FIFO Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXUFE_A {
    #[doc = "0: RXUF flag does not generate an interrupt to the host."]
    _0 = 0,
    #[doc = "1: RXUF flag generates an interrupt to the host."]
    _1 = 1,
}
impl From<RXUFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXUFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXUFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXUFE_A {
        match self.bits {
            false => RXUFE_A::_0,
            true => RXUFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXUFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXUFE_A::_1
    }
}
#[doc = "Field `RXUFE` writer - Receive FIFO Underflow Interrupt Enable"]
pub type RXUFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFIFO_SPEC, RXUFE_A, O>;
impl<'a, const O: u8> RXUFE_W<'a, O> {
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXUFE_A::_0)
    }
    #[doc = "RXUF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXUFE_A::_1)
    }
}
#[doc = "Field `TXOFE` reader - Transmit FIFO Overflow Interrupt Enable"]
pub type TXOFE_R = crate::BitReader<TXOFE_A>;
#[doc = "Transmit FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXOFE_A {
    #[doc = "0: TXOF flag does not generate an interrupt to the host."]
    _0 = 0,
    #[doc = "1: TXOF flag generates an interrupt to the host."]
    _1 = 1,
}
impl From<TXOFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXOFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXOFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOFE_A {
        match self.bits {
            false => TXOFE_A::_0,
            true => TXOFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXOFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXOFE_A::_1
    }
}
#[doc = "Field `TXOFE` writer - Transmit FIFO Overflow Interrupt Enable"]
pub type TXOFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFIFO_SPEC, TXOFE_A, O>;
impl<'a, const O: u8> TXOFE_W<'a, O> {
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOFE_A::_0)
    }
    #[doc = "TXOF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOFE_A::_1)
    }
}
#[doc = "Field `RXOFE` reader - Receive FIFO Overflow Interrupt Enable"]
pub type RXOFE_R = crate::BitReader<RXOFE_A>;
#[doc = "Receive FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOFE_A {
    #[doc = "0: RXOF flag does not generate an interrupt to the host."]
    _0 = 0,
    #[doc = "1: RXOF flag generates an interrupt to the host."]
    _1 = 1,
}
impl From<RXOFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXOFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOFE_A {
        match self.bits {
            false => RXOFE_A::_0,
            true => RXOFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXOFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXOFE_A::_1
    }
}
#[doc = "Field `RXOFE` writer - Receive FIFO Overflow Interrupt Enable"]
pub type RXOFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFIFO_SPEC, RXOFE_A, O>;
impl<'a, const O: u8> RXOFE_W<'a, O> {
    #[doc = "RXOF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOFE_A::_0)
    }
    #[doc = "RXOF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOFE_A::_1)
    }
}
#[doc = "Receive FIFO/Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFLUSH_AW {
    #[doc = "0: No flush operation occurs."]
    _0 = 0,
    #[doc = "1: All data in the receive FIFO/buffer is cleared out."]
    _1 = 1,
}
impl From<RXFLUSH_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFLUSH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFLUSH` writer - Receive FIFO/Buffer Flush"]
pub type RXFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFIFO_SPEC, RXFLUSH_AW, O>;
impl<'a, const O: u8> RXFLUSH_W<'a, O> {
    #[doc = "No flush operation occurs."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFLUSH_AW::_0)
    }
    #[doc = "All data in the receive FIFO/buffer is cleared out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFLUSH_AW::_1)
    }
}
#[doc = "Transmit FIFO/Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFLUSH_AW {
    #[doc = "0: No flush operation occurs."]
    _0 = 0,
    #[doc = "1: All data in the transmit FIFO/Buffer is cleared out."]
    _1 = 1,
}
impl From<TXFLUSH_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFLUSH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFLUSH` writer - Transmit FIFO/Buffer Flush"]
pub type TXFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFIFO_SPEC, TXFLUSH_AW, O>;
impl<'a, const O: u8> TXFLUSH_W<'a, O> {
    #[doc = "No flush operation occurs."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFLUSH_AW::_0)
    }
    #[doc = "All data in the transmit FIFO/Buffer is cleared out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFLUSH_AW::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxufe(&self) -> RXUFE_R {
        RXUFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txofe(&self) -> TXOFE_R {
        TXOFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxofe(&self) -> RXOFE_R {
        RXOFE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxufe(&mut self) -> RXUFE_W<0> {
        RXUFE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txofe(&mut self) -> TXOFE_W<1> {
        TXOFE_W::new(self)
    }
    #[doc = "Bit 2 - Receive FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxofe(&mut self) -> RXOFE_W<2> {
        RXOFE_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO/Buffer Flush"]
    #[inline(always)]
    #[must_use]
    pub fn rxflush(&mut self) -> RXFLUSH_W<6> {
        RXFLUSH_W::new(self)
    }
    #[doc = "Bit 7 - Transmit FIFO/Buffer Flush"]
    #[inline(always)]
    #[must_use]
    pub fn txflush(&mut self) -> TXFLUSH_W<7> {
        TXFLUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfifo](index.html) module"]
pub struct CFIFO_SPEC;
impl crate::RegisterSpec for CFIFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cfifo::R](R) reader structure"]
impl crate::Readable for CFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfifo::W](W) writer structure"]
impl crate::Writable for CFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFIFO to value 0"]
impl crate::Resettable for CFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

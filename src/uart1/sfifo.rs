#[doc = "Register `SFIFO` reader"]
pub struct R(crate::R<SFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFIFO` writer"]
pub struct W(crate::W<SFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFIFO_SPEC>;
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
impl From<crate::W<SFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXUF` reader - Receiver Buffer Underflow Flag"]
pub type RXUF_R = crate::BitReader<RXUF_A>;
#[doc = "Receiver Buffer Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXUF_A {
    #[doc = "0: No receive buffer underflow has occurred since the last time the flag was cleared."]
    _0 = 0,
    #[doc = "1: At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    _1 = 1,
}
impl From<RXUF_A> for bool {
    #[inline(always)]
    fn from(variant: RXUF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXUF_A {
        match self.bits {
            false => RXUF_A::_0,
            true => RXUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXUF_A::_1
    }
}
#[doc = "Field `RXUF` writer - Receiver Buffer Underflow Flag"]
pub type RXUF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SFIFO_SPEC, RXUF_A, O>;
impl<'a, const O: u8> RXUF_W<'a, O> {
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXUF_A::_0)
    }
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXUF_A::_1)
    }
}
#[doc = "Field `TXOF` reader - Transmitter Buffer Overflow Flag"]
pub type TXOF_R = crate::BitReader<TXOF_A>;
#[doc = "Transmitter Buffer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXOF_A {
    #[doc = "0: No transmit buffer overflow has occurred since the last time the flag was cleared."]
    _0 = 0,
    #[doc = "1: At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    _1 = 1,
}
impl From<TXOF_A> for bool {
    #[inline(always)]
    fn from(variant: TXOF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOF_A {
        match self.bits {
            false => TXOF_A::_0,
            true => TXOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXOF_A::_1
    }
}
#[doc = "Field `TXOF` writer - Transmitter Buffer Overflow Flag"]
pub type TXOF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SFIFO_SPEC, TXOF_A, O>;
impl<'a, const O: u8> TXOF_W<'a, O> {
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOF_A::_0)
    }
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOF_A::_1)
    }
}
#[doc = "Field `RXOF` reader - Receiver Buffer Overflow Flag"]
pub type RXOF_R = crate::BitReader<RXOF_A>;
#[doc = "Receiver Buffer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOF_A {
    #[doc = "0: No receive buffer overflow has occurred since the last time the flag was cleared."]
    _0 = 0,
    #[doc = "1: At least one receive buffer overflow has occurred since the last time the flag was cleared."]
    _1 = 1,
}
impl From<RXOF_A> for bool {
    #[inline(always)]
    fn from(variant: RXOF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOF_A {
        match self.bits {
            false => RXOF_A::_0,
            true => RXOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXOF_A::_1
    }
}
#[doc = "Field `RXOF` writer - Receiver Buffer Overflow Flag"]
pub type RXOF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SFIFO_SPEC, RXOF_A, O>;
impl<'a, const O: u8> RXOF_W<'a, O> {
    #[doc = "No receive buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOF_A::_0)
    }
    #[doc = "At least one receive buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOF_A::_1)
    }
}
#[doc = "Field `RXEMPT` reader - Receive Buffer/FIFO Empty"]
pub type RXEMPT_R = crate::BitReader<RXEMPT_A>;
#[doc = "Receive Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEMPT_A {
    #[doc = "0: Receive buffer is not empty."]
    _0 = 0,
    #[doc = "1: Receive buffer is empty."]
    _1 = 1,
}
impl From<RXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEMPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPT_A {
        match self.bits {
            false => RXEMPT_A::_0,
            true => RXEMPT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXEMPT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXEMPT_A::_1
    }
}
#[doc = "Field `TXEMPT` reader - Transmit Buffer/FIFO Empty"]
pub type TXEMPT_R = crate::BitReader<TXEMPT_A>;
#[doc = "Transmit Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPT_A {
    #[doc = "0: Transmit buffer is not empty."]
    _0 = 0,
    #[doc = "1: Transmit buffer is empty."]
    _1 = 1,
}
impl From<TXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEMPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPT_A {
        match self.bits {
            false => TXEMPT_A::_0,
            true => TXEMPT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXEMPT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXEMPT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver Buffer Overflow Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn rxempt(&self) -> RXEMPT_R {
        RXEMPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn txempt(&self) -> TXEMPT_R {
        TXEMPT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<0> {
        RXUF_W::new(self)
    }
    #[doc = "Bit 1 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<1> {
        TXOF_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Buffer Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RXOF_W<2> {
        RXOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfifo](index.html) module"]
pub struct SFIFO_SPEC;
impl crate::RegisterSpec for SFIFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sfifo::R](R) reader structure"]
impl crate::Readable for SFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfifo::W](W) writer structure"]
impl crate::Writable for SFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFIFO to value 0xc0"]
impl crate::Resettable for SFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}

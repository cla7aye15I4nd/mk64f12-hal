#[doc = "Register `MODEM` reader"]
pub struct R(crate::R<MODEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODEM` writer"]
pub struct W(crate::W<MODEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODEM_SPEC>;
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
impl From<crate::W<MODEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCTSE` reader - Transmitter clear-to-send enable"]
pub type TXCTSE_R = crate::BitReader<TXCTSE_A>;
#[doc = "Transmitter clear-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXCTSE_A {
    #[doc = "0: CTS has no effect on the transmitter."]
    _0 = 0,
    #[doc = "1: Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    _1 = 1,
}
impl From<TXCTSE_A> for bool {
    #[inline(always)]
    fn from(variant: TXCTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXCTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCTSE_A {
        match self.bits {
            false => TXCTSE_A::_0,
            true => TXCTSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXCTSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXCTSE_A::_1
    }
}
#[doc = "Field `TXCTSE` writer - Transmitter clear-to-send enable"]
pub type TXCTSE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MODEM_SPEC, TXCTSE_A, O>;
impl<'a, const O: u8> TXCTSE_W<'a, O> {
    #[doc = "CTS has no effect on the transmitter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXCTSE_A::_0)
    }
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXCTSE_A::_1)
    }
}
#[doc = "Field `TXRTSE` reader - Transmitter request-to-send enable"]
pub type TXRTSE_R = crate::BitReader<TXRTSE_A>;
#[doc = "Transmitter request-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRTSE_A {
    #[doc = "0: The transmitter has no effect on RTS."]
    _0 = 0,
    #[doc = "1: When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit. (FIFO) (FIFO)"]
    _1 = 1,
}
impl From<TXRTSE_A> for bool {
    #[inline(always)]
    fn from(variant: TXRTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXRTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRTSE_A {
        match self.bits {
            false => TXRTSE_A::_0,
            true => TXRTSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXRTSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXRTSE_A::_1
    }
}
#[doc = "Field `TXRTSE` writer - Transmitter request-to-send enable"]
pub type TXRTSE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MODEM_SPEC, TXRTSE_A, O>;
impl<'a, const O: u8> TXRTSE_W<'a, O> {
    #[doc = "The transmitter has no effect on RTS."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRTSE_A::_0)
    }
    #[doc = "When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit. (FIFO) (FIFO)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRTSE_A::_1)
    }
}
#[doc = "Field `TXRTSPOL` reader - Transmitter request-to-send polarity"]
pub type TXRTSPOL_R = crate::BitReader<TXRTSPOL_A>;
#[doc = "Transmitter request-to-send polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRTSPOL_A {
    #[doc = "0: Transmitter RTS is active low."]
    _0 = 0,
    #[doc = "1: Transmitter RTS is active high."]
    _1 = 1,
}
impl From<TXRTSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TXRTSPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl TXRTSPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRTSPOL_A {
        match self.bits {
            false => TXRTSPOL_A::_0,
            true => TXRTSPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXRTSPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXRTSPOL_A::_1
    }
}
#[doc = "Field `TXRTSPOL` writer - Transmitter request-to-send polarity"]
pub type TXRTSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u8, MODEM_SPEC, TXRTSPOL_A, O>;
impl<'a, const O: u8> TXRTSPOL_W<'a, O> {
    #[doc = "Transmitter RTS is active low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRTSPOL_A::_0)
    }
    #[doc = "Transmitter RTS is active high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRTSPOL_A::_1)
    }
}
#[doc = "Field `RXRTSE` reader - Receiver request-to-send enable"]
pub type RXRTSE_R = crate::BitReader<RXRTSE_A>;
#[doc = "Receiver request-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXRTSE_A {
    #[doc = "0: The receiver has no effect on RTS."]
    _0 = 0,
    #[doc = "1: RTS is deasserted if the number of characters in the receiver data register (FIFO) is equal to or greater than RWFIFO\\[RXWATER\\]. RTS is asserted when the number of characters in the receiver data register (FIFO) is less than RWFIFO\\[RXWATER\\]."]
    _1 = 1,
}
impl From<RXRTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RXRTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXRTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRTSE_A {
        match self.bits {
            false => RXRTSE_A::_0,
            true => RXRTSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXRTSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXRTSE_A::_1
    }
}
#[doc = "Field `RXRTSE` writer - Receiver request-to-send enable"]
pub type RXRTSE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MODEM_SPEC, RXRTSE_A, O>;
impl<'a, const O: u8> RXRTSE_W<'a, O> {
    #[doc = "The receiver has no effect on RTS."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRTSE_A::_0)
    }
    #[doc = "RTS is deasserted if the number of characters in the receiver data register (FIFO) is equal to or greater than RWFIFO\\[RXWATER\\]. RTS is asserted when the number of characters in the receiver data register (FIFO) is less than RWFIFO\\[RXWATER\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXRTSE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline(always)]
    pub fn txctse(&self) -> TXCTSE_R {
        TXCTSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline(always)]
    pub fn txrtse(&self) -> TXRTSE_R {
        TXRTSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline(always)]
    pub fn txrtspol(&self) -> TXRTSPOL_R {
        TXRTSPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline(always)]
    pub fn rxrtse(&self) -> RXRTSE_R {
        RXRTSE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline(always)]
    #[must_use]
    pub fn txctse(&mut self) -> TXCTSE_W<0> {
        TXCTSE_W::new(self)
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrtse(&mut self) -> TXRTSE_W<1> {
        TXRTSE_W::new(self)
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline(always)]
    #[must_use]
    pub fn txrtspol(&mut self) -> TXRTSPOL_W<2> {
        TXRTSPOL_W::new(self)
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrtse(&mut self) -> RXRTSE_W<3> {
        RXRTSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Modem Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modem](index.html) module"]
pub struct MODEM_SPEC;
impl crate::RegisterSpec for MODEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [modem::R](R) reader structure"]
impl crate::Readable for MODEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modem::W](W) writer structure"]
impl crate::Writable for MODEM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEM to value 0"]
impl crate::Resettable for MODEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

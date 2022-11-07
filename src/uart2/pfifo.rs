#[doc = "Register `PFIFO` reader"]
pub struct R(crate::R<PFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFIFO` writer"]
pub struct W(crate::W<PFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFIFO_SPEC>;
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
impl From<crate::W<PFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFOSIZE` reader - Receive FIFO. Buffer Depth"]
pub type RXFIFOSIZE_R = crate::FieldReader<u8, RXFIFOSIZE_A>;
#[doc = "Receive FIFO. Buffer Depth\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXFIFOSIZE_A {
    #[doc = "0: Receive FIFO/Buffer depth = 1 dataword."]
    _000 = 0,
    #[doc = "1: Receive FIFO/Buffer depth = 4 datawords."]
    _001 = 1,
    #[doc = "2: Receive FIFO/Buffer depth = 8 datawords."]
    _010 = 2,
    #[doc = "3: Receive FIFO/Buffer depth = 16 datawords."]
    _011 = 3,
    #[doc = "4: Receive FIFO/Buffer depth = 32 datawords."]
    _100 = 4,
    #[doc = "5: Receive FIFO/Buffer depth = 64 datawords."]
    _101 = 5,
    #[doc = "6: Receive FIFO/Buffer depth = 128 datawords."]
    _110 = 6,
}
impl From<RXFIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFIFOSIZE_A) -> Self {
        variant as _
    }
}
impl RXFIFOSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXFIFOSIZE_A> {
        match self.bits {
            0 => Some(RXFIFOSIZE_A::_000),
            1 => Some(RXFIFOSIZE_A::_001),
            2 => Some(RXFIFOSIZE_A::_010),
            3 => Some(RXFIFOSIZE_A::_011),
            4 => Some(RXFIFOSIZE_A::_100),
            5 => Some(RXFIFOSIZE_A::_101),
            6 => Some(RXFIFOSIZE_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RXFIFOSIZE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RXFIFOSIZE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RXFIFOSIZE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RXFIFOSIZE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RXFIFOSIZE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RXFIFOSIZE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RXFIFOSIZE_A::_110
    }
}
#[doc = "Field `RXFE` reader - Receive FIFO Enable"]
pub type RXFE_R = crate::BitReader<RXFE_A>;
#[doc = "Receive FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFE_A {
    #[doc = "0: Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    _0 = 0,
    #[doc = "1: Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    _1 = 1,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFE_A {
        match self.bits {
            false => RXFE_A::_0,
            true => RXFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXFE_A::_1
    }
}
#[doc = "Field `RXFE` writer - Receive FIFO Enable"]
pub type RXFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, PFIFO_SPEC, RXFE_A, O>;
impl<'a, const O: u8> RXFE_W<'a, O> {
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFE_A::_0)
    }
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFE_A::_1)
    }
}
#[doc = "Field `TXFIFOSIZE` reader - Transmit FIFO. Buffer Depth"]
pub type TXFIFOSIZE_R = crate::FieldReader<u8, TXFIFOSIZE_A>;
#[doc = "Transmit FIFO. Buffer Depth\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXFIFOSIZE_A {
    #[doc = "0: Transmit FIFO/Buffer depth = 1 dataword."]
    _000 = 0,
    #[doc = "1: Transmit FIFO/Buffer depth = 4 datawords."]
    _001 = 1,
    #[doc = "2: Transmit FIFO/Buffer depth = 8 datawords."]
    _010 = 2,
    #[doc = "3: Transmit FIFO/Buffer depth = 16 datawords."]
    _011 = 3,
    #[doc = "4: Transmit FIFO/Buffer depth = 32 datawords."]
    _100 = 4,
    #[doc = "5: Transmit FIFO/Buffer depth = 64 datawords."]
    _101 = 5,
    #[doc = "6: Transmit FIFO/Buffer depth = 128 datawords."]
    _110 = 6,
}
impl From<TXFIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFIFOSIZE_A) -> Self {
        variant as _
    }
}
impl TXFIFOSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXFIFOSIZE_A> {
        match self.bits {
            0 => Some(TXFIFOSIZE_A::_000),
            1 => Some(TXFIFOSIZE_A::_001),
            2 => Some(TXFIFOSIZE_A::_010),
            3 => Some(TXFIFOSIZE_A::_011),
            4 => Some(TXFIFOSIZE_A::_100),
            5 => Some(TXFIFOSIZE_A::_101),
            6 => Some(TXFIFOSIZE_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TXFIFOSIZE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TXFIFOSIZE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TXFIFOSIZE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TXFIFOSIZE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TXFIFOSIZE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == TXFIFOSIZE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == TXFIFOSIZE_A::_110
    }
}
#[doc = "Field `TXFE` reader - Transmit FIFO Enable"]
pub type TXFE_R = crate::BitReader<TXFE_A>;
#[doc = "Transmit FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFE_A {
    #[doc = "0: Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    _0 = 0,
    #[doc = "1: Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    _1 = 1,
}
impl From<TXFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFE_A {
        match self.bits {
            false => TXFE_A::_0,
            true => TXFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXFE_A::_1
    }
}
#[doc = "Field `TXFE` writer - Transmit FIFO Enable"]
pub type TXFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, PFIFO_SPEC, TXFE_A, O>;
impl<'a, const O: u8> TXFE_W<'a, O> {
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFE_A::_0)
    }
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO. Buffer Depth"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Transmit FIFO. Buffer Depth"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfe(&mut self) -> RXFE_W<3> {
        RXFE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txfe(&mut self) -> TXFE_W<7> {
        TXFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO Parameters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfifo](index.html) module"]
pub struct PFIFO_SPEC;
impl crate::RegisterSpec for PFIFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pfifo::R](R) reader structure"]
impl crate::Readable for PFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfifo::W](W) writer structure"]
impl crate::Writable for PFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFIFO to value 0"]
impl crate::Resettable for PFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

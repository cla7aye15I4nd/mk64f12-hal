#[doc = "Register `C3` reader"]
pub struct R(crate::R<C3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C3` writer"]
pub struct W(crate::W<C3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C3_SPEC>;
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
impl From<crate::W<C3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEIE` reader - Parity Error Interrupt Enable"]
pub type PEIE_R = crate::BitReader<PEIE_A>;
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEIE_A {
    #[doc = "0: PF interrupt requests are disabled."]
    _0 = 0,
    #[doc = "1: PF interrupt requests are enabled."]
    _1 = 1,
}
impl From<PEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEIE_A {
        match self.bits {
            false => PEIE_A::_0,
            true => PEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEIE_A::_1
    }
}
#[doc = "Field `PEIE` writer - Parity Error Interrupt Enable"]
pub type PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C3_SPEC, PEIE_A, O>;
impl<'a, const O: u8> PEIE_W<'a, O> {
    #[doc = "PF interrupt requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEIE_A::_0)
    }
    #[doc = "PF interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEIE_A::_1)
    }
}
#[doc = "Field `FEIE` reader - Framing Error Interrupt Enable"]
pub type FEIE_R = crate::BitReader<FEIE_A>;
#[doc = "Framing Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIE_A {
    #[doc = "0: FE interrupt requests are disabled."]
    _0 = 0,
    #[doc = "1: FE interrupt requests are enabled."]
    _1 = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::_0,
            true => FEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEIE_A::_1
    }
}
#[doc = "Field `FEIE` writer - Framing Error Interrupt Enable"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C3_SPEC, FEIE_A, O>;
impl<'a, const O: u8> FEIE_W<'a, O> {
    #[doc = "FE interrupt requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEIE_A::_0)
    }
    #[doc = "FE interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEIE_A::_1)
    }
}
#[doc = "Field `NEIE` reader - Noise Error Interrupt Enable"]
pub type NEIE_R = crate::BitReader<NEIE_A>;
#[doc = "Noise Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEIE_A {
    #[doc = "0: NF interrupt requests are disabled."]
    _0 = 0,
    #[doc = "1: NF interrupt requests are enabled."]
    _1 = 1,
}
impl From<NEIE_A> for bool {
    #[inline(always)]
    fn from(variant: NEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl NEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEIE_A {
        match self.bits {
            false => NEIE_A::_0,
            true => NEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NEIE_A::_1
    }
}
#[doc = "Field `NEIE` writer - Noise Error Interrupt Enable"]
pub type NEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C3_SPEC, NEIE_A, O>;
impl<'a, const O: u8> NEIE_W<'a, O> {
    #[doc = "NF interrupt requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NEIE_A::_0)
    }
    #[doc = "NF interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NEIE_A::_1)
    }
}
#[doc = "Field `ORIE` reader - Overrun Error Interrupt Enable"]
pub type ORIE_R = crate::BitReader<ORIE_A>;
#[doc = "Overrun Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORIE_A {
    #[doc = "0: OR interrupts are disabled."]
    _0 = 0,
    #[doc = "1: OR interrupt requests are enabled."]
    _1 = 1,
}
impl From<ORIE_A> for bool {
    #[inline(always)]
    fn from(variant: ORIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ORIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORIE_A {
        match self.bits {
            false => ORIE_A::_0,
            true => ORIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORIE_A::_1
    }
}
#[doc = "Field `ORIE` writer - Overrun Error Interrupt Enable"]
pub type ORIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C3_SPEC, ORIE_A, O>;
impl<'a, const O: u8> ORIE_W<'a, O> {
    #[doc = "OR interrupts are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ORIE_A::_0)
    }
    #[doc = "OR interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ORIE_A::_1)
    }
}
#[doc = "Field `TXINV` reader - Transmit Data Inversion."]
pub type TXINV_R = crate::BitReader<TXINV_A>;
#[doc = "Transmit Data Inversion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXINV_A {
    #[doc = "0: Transmit data is not inverted."]
    _0 = 0,
    #[doc = "1: Transmit data is inverted."]
    _1 = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl TXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::_0,
            true => TXINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXINV_A::_1
    }
}
#[doc = "Field `TXINV` writer - Transmit Data Inversion."]
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u8, C3_SPEC, TXINV_A, O>;
impl<'a, const O: u8> TXINV_W<'a, O> {
    #[doc = "Transmit data is not inverted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXINV_A::_0)
    }
    #[doc = "Transmit data is inverted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXINV_A::_1)
    }
}
#[doc = "Field `TXDIR` reader - Transmitter Pin Data Direction in Single-Wire mode"]
pub type TXDIR_R = crate::BitReader<TXDIR_A>;
#[doc = "Transmitter Pin Data Direction in Single-Wire mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDIR_A {
    #[doc = "0: TXD pin is an input in single wire mode."]
    _0 = 0,
    #[doc = "1: TXD pin is an output in single wire mode."]
    _1 = 1,
}
impl From<TXDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TXDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDIR_A {
        match self.bits {
            false => TXDIR_A::_0,
            true => TXDIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXDIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXDIR_A::_1
    }
}
#[doc = "Field `TXDIR` writer - Transmitter Pin Data Direction in Single-Wire mode"]
pub type TXDIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, C3_SPEC, TXDIR_A, O>;
impl<'a, const O: u8> TXDIR_W<'a, O> {
    #[doc = "TXD pin is an input in single wire mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDIR_A::_0)
    }
    #[doc = "TXD pin is an output in single wire mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDIR_A::_1)
    }
}
#[doc = "Field `T8` reader - Transmit Bit 8"]
pub type T8_R = crate::BitReader<bool>;
#[doc = "Field `T8` writer - Transmit Bit 8"]
pub type T8_W<'a, const O: u8> = crate::BitWriter<'a, u8, C3_SPEC, bool, O>;
#[doc = "Field `R8` reader - Received Bit 8"]
pub type R8_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&self) -> NEIE_R {
        NEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&self) -> ORIE_R {
        ORIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Data Inversion."]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter Pin Data Direction in Single-Wire mode"]
    #[inline(always)]
    pub fn txdir(&self) -> TXDIR_R {
        TXDIR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Bit 8"]
    #[inline(always)]
    pub fn t8(&self) -> T8_R {
        T8_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Received Bit 8"]
    #[inline(always)]
    pub fn r8(&self) -> R8_R {
        R8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<0> {
        PEIE_W::new(self)
    }
    #[doc = "Bit 1 - Framing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<1> {
        FEIE_W::new(self)
    }
    #[doc = "Bit 2 - Noise Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn neie(&mut self) -> NEIE_W<2> {
        NEIE_W::new(self)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn orie(&mut self) -> ORIE_W<3> {
        ORIE_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Data Inversion."]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<4> {
        TXINV_W::new(self)
    }
    #[doc = "Bit 5 - Transmitter Pin Data Direction in Single-Wire mode"]
    #[inline(always)]
    #[must_use]
    pub fn txdir(&mut self) -> TXDIR_W<5> {
        TXDIR_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn t8(&mut self) -> T8_W<6> {
        T8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3](index.html) module"]
pub struct C3_SPEC;
impl crate::RegisterSpec for C3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c3::R](R) reader structure"]
impl crate::Readable for C3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c3::W](W) writer structure"]
impl crate::Writable for C3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C3 to value 0"]
impl crate::Resettable for C3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `C1` reader"]
pub struct R(crate::R<C1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1` writer"]
pub struct W(crate::W<C1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_SPEC>;
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
impl From<crate::W<C1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: All DMA signalling disabled."]
    _0 = 0,
    #[doc = "1: DMA transfer is enabled. While SMB\\[FACK\\]
= 0, the following conditions trigger the DMA request: a data byte is received, and either address or data is transmitted. (ACK/NACK is automatic) the first byte received matches the A1 register or is a general call address. If any address matching occurs, S\\[IAAS\\]
and S\\[TCF\\]
are set. If the direction of transfer is known from master to slave, then it is not required to check S\\[SRW\\]. With this assumption, DMA can also be used in this case. In other cases, if the master reads data from the slave, then it is required to rewrite the C1 register operation. With this assumption, DMA cannot be used. When FACK = 1, an address or a data byte is transmitted."]
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "All DMA signalling disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA transfer is enabled. While SMB\\[FACK\\]
= 0, the following conditions trigger the DMA request: a data byte is received, and either address or data is transmitted. (ACK/NACK is automatic) the first byte received matches the A1 register or is a general call address. If any address matching occurs, S\\[IAAS\\]
and S\\[TCF\\]
are set. If the direction of transfer is known from master to slave, then it is not required to check S\\[SRW\\]. With this assumption, DMA can also be used in this case. In other cases, if the master reads data from the slave, then it is required to rewrite the C1 register operation. With this assumption, DMA cannot be used. When FACK = 1, an address or a data byte is transmitted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
    }
}
#[doc = "Field `WUEN` reader - Wakeup Enable"]
pub type WUEN_R = crate::BitReader<WUEN_A>;
#[doc = "Wakeup Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUEN_A {
    #[doc = "0: Normal operation. No interrupt generated when address matching in low power mode."]
    _0 = 0,
    #[doc = "1: Enables the wakeup function in low power mode."]
    _1 = 1,
}
impl From<WUEN_A> for bool {
    #[inline(always)]
    fn from(variant: WUEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WUEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUEN_A {
        match self.bits {
            false => WUEN_A::_0,
            true => WUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUEN_A::_1
    }
}
#[doc = "Field `WUEN` writer - Wakeup Enable"]
pub type WUEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, WUEN_A, O>;
impl<'a, const O: u8> WUEN_W<'a, O> {
    #[doc = "Normal operation. No interrupt generated when address matching in low power mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUEN_A::_0)
    }
    #[doc = "Enables the wakeup function in low power mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUEN_A::_1)
    }
}
#[doc = "Field `RSTA` writer - Repeat START"]
pub type RSTA_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, bool, O>;
#[doc = "Field `TXAK` reader - Transmit Acknowledge Enable"]
pub type TXAK_R = crate::BitReader<TXAK_A>;
#[doc = "Transmit Acknowledge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXAK_A {
    #[doc = "0: An acknowledge signal is sent to the bus on the following receiving byte (if FACK is cleared) or the current receiving byte (if FACK is set)."]
    _0 = 0,
    #[doc = "1: No acknowledge signal is sent to the bus on the following receiving data byte (if FACK is cleared) or the current receiving data byte (if FACK is set)."]
    _1 = 1,
}
impl From<TXAK_A> for bool {
    #[inline(always)]
    fn from(variant: TXAK_A) -> Self {
        variant as u8 != 0
    }
}
impl TXAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXAK_A {
        match self.bits {
            false => TXAK_A::_0,
            true => TXAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXAK_A::_1
    }
}
#[doc = "Field `TXAK` writer - Transmit Acknowledge Enable"]
pub type TXAK_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, TXAK_A, O>;
impl<'a, const O: u8> TXAK_W<'a, O> {
    #[doc = "An acknowledge signal is sent to the bus on the following receiving byte (if FACK is cleared) or the current receiving byte (if FACK is set)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXAK_A::_0)
    }
    #[doc = "No acknowledge signal is sent to the bus on the following receiving data byte (if FACK is cleared) or the current receiving data byte (if FACK is set)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXAK_A::_1)
    }
}
#[doc = "Field `TX` reader - Transmit Mode Select"]
pub type TX_R = crate::BitReader<TX_A>;
#[doc = "Transmit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_A {
    #[doc = "0: Receive"]
    _0 = 0,
    #[doc = "1: Transmit"]
    _1 = 1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::_0,
            true => TX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TX_A::_1
    }
}
#[doc = "Field `TX` writer - Transmit Mode Select"]
pub type TX_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, TX_A, O>;
impl<'a, const O: u8> TX_W<'a, O> {
    #[doc = "Receive"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_A::_0)
    }
    #[doc = "Transmit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_A::_1)
    }
}
#[doc = "Field `MST` reader - Master Mode Select"]
pub type MST_R = crate::BitReader<MST_A>;
#[doc = "Master Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST_A {
    #[doc = "0: Slave mode"]
    _0 = 0,
    #[doc = "1: Master mode"]
    _1 = 1,
}
impl From<MST_A> for bool {
    #[inline(always)]
    fn from(variant: MST_A) -> Self {
        variant as u8 != 0
    }
}
impl MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST_A {
        match self.bits {
            false => MST_A::_0,
            true => MST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MST_A::_1
    }
}
#[doc = "Field `MST` writer - Master Mode Select"]
pub type MST_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, MST_A, O>;
impl<'a, const O: u8> MST_W<'a, O> {
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MST_A::_0)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MST_A::_1)
    }
}
#[doc = "Field `IICIE` reader - I2C Interrupt Enable"]
pub type IICIE_R = crate::BitReader<IICIE_A>;
#[doc = "I2C Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICIE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<IICIE_A> for bool {
    #[inline(always)]
    fn from(variant: IICIE_A) -> Self {
        variant as u8 != 0
    }
}
impl IICIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICIE_A {
        match self.bits {
            false => IICIE_A::_0,
            true => IICIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICIE_A::_1
    }
}
#[doc = "Field `IICIE` writer - I2C Interrupt Enable"]
pub type IICIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, IICIE_A, O>;
impl<'a, const O: u8> IICIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICIE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICIE_A::_1)
    }
}
#[doc = "Field `IICEN` reader - I2C Enable"]
pub type IICEN_R = crate::BitReader<IICEN_A>;
#[doc = "I2C Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<IICEN_A> for bool {
    #[inline(always)]
    fn from(variant: IICEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICEN_A {
        match self.bits {
            false => IICEN_A::_0,
            true => IICEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICEN_A::_1
    }
}
#[doc = "Field `IICEN` writer - I2C Enable"]
pub type IICEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, IICEN_A, O>;
impl<'a, const O: u8> IICEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Enable"]
    #[inline(always)]
    pub fn wuen(&self) -> WUEN_R {
        WUEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Acknowledge Enable"]
    #[inline(always)]
    pub fn txak(&self) -> TXAK_R {
        TXAK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Mode Select"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master Mode Select"]
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Interrupt Enable"]
    #[inline(always)]
    pub fn iicie(&self) -> IICIE_R {
        IICIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Enable"]
    #[inline(always)]
    pub fn iicen(&self) -> IICEN_R {
        IICEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wuen(&mut self) -> WUEN_W<1> {
        WUEN_W::new(self)
    }
    #[doc = "Bit 2 - Repeat START"]
    #[inline(always)]
    #[must_use]
    pub fn rsta(&mut self) -> RSTA_W<2> {
        RSTA_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Acknowledge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txak(&mut self) -> TXAK_W<3> {
        TXAK_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<4> {
        TX_W::new(self)
    }
    #[doc = "Bit 5 - Master Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mst(&mut self) -> MST_W<5> {
        MST_W::new(self)
    }
    #[doc = "Bit 6 - I2C Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iicie(&mut self) -> IICIE_W<6> {
        IICIE_W::new(self)
    }
    #[doc = "Bit 7 - I2C Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iicen(&mut self) -> IICEN_W<7> {
        IICEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](index.html) module"]
pub struct C1_SPEC;
impl crate::RegisterSpec for C1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c1::R](R) reader structure"]
impl crate::Readable for C1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1::W](W) writer structure"]
impl crate::Writable for C1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

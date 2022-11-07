#[doc = "Register `ESR1` reader"]
pub struct R(crate::R<ESR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESR1` writer"]
pub struct W(crate::W<ESR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESR1_SPEC>;
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
impl From<crate::W<ESR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKINT` reader - Wake-Up Interrupt"]
pub type WAKINT_R = crate::BitReader<WAKINT_A>;
#[doc = "Wake-Up Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKINT_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: Indicates a recessive to dominant transition was received on the CAN bus."]
    _1 = 1,
}
impl From<WAKINT_A> for bool {
    #[inline(always)]
    fn from(variant: WAKINT_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKINT_A {
        match self.bits {
            false => WAKINT_A::_0,
            true => WAKINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKINT_A::_1
    }
}
#[doc = "Field `WAKINT` writer - Wake-Up Interrupt"]
pub type WAKINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESR1_SPEC, WAKINT_A, O>;
impl<'a, const O: u8> WAKINT_W<'a, O> {
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKINT_A::_0)
    }
    #[doc = "Indicates a recessive to dominant transition was received on the CAN bus."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKINT_A::_1)
    }
}
#[doc = "Field `ERRINT` reader - Error Interrupt"]
pub type ERRINT_R = crate::BitReader<ERRINT_A>;
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRINT_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: Indicates setting of any Error Bit in the Error and Status Register."]
    _1 = 1,
}
impl From<ERRINT_A> for bool {
    #[inline(always)]
    fn from(variant: ERRINT_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRINT_A {
        match self.bits {
            false => ERRINT_A::_0,
            true => ERRINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERRINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERRINT_A::_1
    }
}
#[doc = "Field `ERRINT` writer - Error Interrupt"]
pub type ERRINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESR1_SPEC, ERRINT_A, O>;
impl<'a, const O: u8> ERRINT_W<'a, O> {
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERRINT_A::_0)
    }
    #[doc = "Indicates setting of any Error Bit in the Error and Status Register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERRINT_A::_1)
    }
}
#[doc = "Field `BOFFINT` reader - Bus Off Interrupt"]
pub type BOFFINT_R = crate::BitReader<BOFFINT_A>;
#[doc = "Bus Off Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFFINT_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: FlexCAN module entered Bus Off state."]
    _1 = 1,
}
impl From<BOFFINT_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFINT_A) -> Self {
        variant as u8 != 0
    }
}
impl BOFFINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFINT_A {
        match self.bits {
            false => BOFFINT_A::_0,
            true => BOFFINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOFFINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOFFINT_A::_1
    }
}
#[doc = "Field `BOFFINT` writer - Bus Off Interrupt"]
pub type BOFFINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESR1_SPEC, BOFFINT_A, O>;
impl<'a, const O: u8> BOFFINT_W<'a, O> {
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOFFINT_A::_0)
    }
    #[doc = "FlexCAN module entered Bus Off state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOFFINT_A::_1)
    }
}
#[doc = "Field `RX` reader - FlexCAN In Reception"]
pub type RX_R = crate::BitReader<RX_A>;
#[doc = "FlexCAN In Reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_A {
    #[doc = "0: FlexCAN is not receiving a message."]
    _0 = 0,
    #[doc = "1: FlexCAN is receiving a message."]
    _1 = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::_0,
            true => RX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX_A::_1
    }
}
#[doc = "Field `FLTCONF` reader - Fault Confinement State"]
pub type FLTCONF_R = crate::FieldReader<u8, FLTCONF_A>;
#[doc = "Fault Confinement State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLTCONF_A {
    #[doc = "0: Error Active"]
    _00 = 0,
    #[doc = "1: Error Passive"]
    _01 = 1,
}
impl From<FLTCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTCONF_A) -> Self {
        variant as _
    }
}
impl FLTCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLTCONF_A> {
        match self.bits {
            0 => Some(FLTCONF_A::_00),
            1 => Some(FLTCONF_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTCONF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTCONF_A::_01
    }
}
#[doc = "Field `TX` reader - FlexCAN In Transmission"]
pub type TX_R = crate::BitReader<TX_A>;
#[doc = "FlexCAN In Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_A {
    #[doc = "0: FlexCAN is not transmitting a message."]
    _0 = 0,
    #[doc = "1: FlexCAN is transmitting a message."]
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
#[doc = "Field `IDLE` reader - This bit indicates when CAN bus is in IDLE state"]
pub type IDLE_R = crate::BitReader<IDLE_A>;
#[doc = "This bit indicates when CAN bus is in IDLE state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: CAN bus is now IDLE."]
    _1 = 1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::_0,
            true => IDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDLE_A::_1
    }
}
#[doc = "Field `RXWRN` reader - Rx Error Warning"]
pub type RXWRN_R = crate::BitReader<RXWRN_A>;
#[doc = "Rx Error Warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXWRN_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: RXERRCNT is greater than or equal to 96."]
    _1 = 1,
}
impl From<RXWRN_A> for bool {
    #[inline(always)]
    fn from(variant: RXWRN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXWRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXWRN_A {
        match self.bits {
            false => RXWRN_A::_0,
            true => RXWRN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXWRN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXWRN_A::_1
    }
}
#[doc = "Field `TXWRN` reader - TX Error Warning"]
pub type TXWRN_R = crate::BitReader<TXWRN_A>;
#[doc = "TX Error Warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXWRN_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: TXERRCNT is greater than or equal to 96."]
    _1 = 1,
}
impl From<TXWRN_A> for bool {
    #[inline(always)]
    fn from(variant: TXWRN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXWRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXWRN_A {
        match self.bits {
            false => TXWRN_A::_0,
            true => TXWRN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXWRN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXWRN_A::_1
    }
}
#[doc = "Field `STFERR` reader - Stuffing Error"]
pub type STFERR_R = crate::BitReader<STFERR_A>;
#[doc = "Stuffing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STFERR_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: A Stuffing Error occurred since last read of this register."]
    _1 = 1,
}
impl From<STFERR_A> for bool {
    #[inline(always)]
    fn from(variant: STFERR_A) -> Self {
        variant as u8 != 0
    }
}
impl STFERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STFERR_A {
        match self.bits {
            false => STFERR_A::_0,
            true => STFERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STFERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STFERR_A::_1
    }
}
#[doc = "Field `FRMERR` reader - Form Error"]
pub type FRMERR_R = crate::BitReader<FRMERR_A>;
#[doc = "Form Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRMERR_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: A Form Error occurred since last read of this register."]
    _1 = 1,
}
impl From<FRMERR_A> for bool {
    #[inline(always)]
    fn from(variant: FRMERR_A) -> Self {
        variant as u8 != 0
    }
}
impl FRMERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRMERR_A {
        match self.bits {
            false => FRMERR_A::_0,
            true => FRMERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRMERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRMERR_A::_1
    }
}
#[doc = "Field `CRCERR` reader - Cyclic Redundancy Check Error"]
pub type CRCERR_R = crate::BitReader<CRCERR_A>;
#[doc = "Cyclic Redundancy Check Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERR_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: A CRC error occurred since last read of this register."]
    _1 = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::_0,
            true => CRCERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCERR_A::_1
    }
}
#[doc = "Field `ACKERR` reader - Acknowledge Error"]
pub type ACKERR_R = crate::BitReader<ACKERR_A>;
#[doc = "Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKERR_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: An ACK error occurred since last read of this register."]
    _1 = 1,
}
impl From<ACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACKERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKERR_A {
        match self.bits {
            false => ACKERR_A::_0,
            true => ACKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKERR_A::_1
    }
}
#[doc = "Field `BIT0ERR` reader - Bit0 Error"]
pub type BIT0ERR_R = crate::BitReader<BIT0ERR_A>;
#[doc = "Bit0 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT0ERR_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: At least one bit sent as dominant is received as recessive."]
    _1 = 1,
}
impl From<BIT0ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT0ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl BIT0ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT0ERR_A {
        match self.bits {
            false => BIT0ERR_A::_0,
            true => BIT0ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BIT0ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BIT0ERR_A::_1
    }
}
#[doc = "Field `BIT1ERR` reader - Bit1 Error"]
pub type BIT1ERR_R = crate::BitReader<BIT1ERR_A>;
#[doc = "Bit1 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT1ERR_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: At least one bit sent as recessive is received as dominant."]
    _1 = 1,
}
impl From<BIT1ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT1ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl BIT1ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT1ERR_A {
        match self.bits {
            false => BIT1ERR_A::_0,
            true => BIT1ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BIT1ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BIT1ERR_A::_1
    }
}
#[doc = "Field `RWRNINT` reader - Rx Warning Interrupt Flag"]
pub type RWRNINT_R = crate::BitReader<RWRNINT_A>;
#[doc = "Rx Warning Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWRNINT_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    _1 = 1,
}
impl From<RWRNINT_A> for bool {
    #[inline(always)]
    fn from(variant: RWRNINT_A) -> Self {
        variant as u8 != 0
    }
}
impl RWRNINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWRNINT_A {
        match self.bits {
            false => RWRNINT_A::_0,
            true => RWRNINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWRNINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWRNINT_A::_1
    }
}
#[doc = "Field `RWRNINT` writer - Rx Warning Interrupt Flag"]
pub type RWRNINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESR1_SPEC, RWRNINT_A, O>;
impl<'a, const O: u8> RWRNINT_W<'a, O> {
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWRNINT_A::_0)
    }
    #[doc = "The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWRNINT_A::_1)
    }
}
#[doc = "Field `TWRNINT` reader - Tx Warning Interrupt Flag"]
pub type TWRNINT_R = crate::BitReader<TWRNINT_A>;
#[doc = "Tx Warning Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWRNINT_A {
    #[doc = "0: No such occurrence."]
    _0 = 0,
    #[doc = "1: The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    _1 = 1,
}
impl From<TWRNINT_A> for bool {
    #[inline(always)]
    fn from(variant: TWRNINT_A) -> Self {
        variant as u8 != 0
    }
}
impl TWRNINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWRNINT_A {
        match self.bits {
            false => TWRNINT_A::_0,
            true => TWRNINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWRNINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWRNINT_A::_1
    }
}
#[doc = "Field `TWRNINT` writer - Tx Warning Interrupt Flag"]
pub type TWRNINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESR1_SPEC, TWRNINT_A, O>;
impl<'a, const O: u8> TWRNINT_W<'a, O> {
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWRNINT_A::_0)
    }
    #[doc = "The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWRNINT_A::_1)
    }
}
#[doc = "Field `SYNCH` reader - CAN Synchronization Status"]
pub type SYNCH_R = crate::BitReader<SYNCH_A>;
#[doc = "CAN Synchronization Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCH_A {
    #[doc = "0: FlexCAN is not synchronized to the CAN bus."]
    _0 = 0,
    #[doc = "1: FlexCAN is synchronized to the CAN bus."]
    _1 = 1,
}
impl From<SYNCH_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCH_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCH_A {
        match self.bits {
            false => SYNCH_A::_0,
            true => SYNCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCH_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up Interrupt"]
    #[inline(always)]
    pub fn wakint(&self) -> WAKINT_R {
        WAKINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&self) -> ERRINT_R {
        ERRINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline(always)]
    pub fn boffint(&self) -> BOFFINT_R {
        BOFFINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FlexCAN In Reception"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Fault Confinement State"]
    #[inline(always)]
    pub fn fltconf(&self) -> FLTCONF_R {
        FLTCONF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - FlexCAN In Transmission"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit indicates when CAN bus is in IDLE state"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rx Error Warning"]
    #[inline(always)]
    pub fn rxwrn(&self) -> RXWRN_R {
        RXWRN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX Error Warning"]
    #[inline(always)]
    pub fn txwrn(&self) -> TXWRN_R {
        TXWRN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stuffing Error"]
    #[inline(always)]
    pub fn stferr(&self) -> STFERR_R {
        STFERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Form Error"]
    #[inline(always)]
    pub fn frmerr(&self) -> FRMERR_R {
        FRMERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cyclic Redundancy Check Error"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Acknowledge Error"]
    #[inline(always)]
    pub fn ackerr(&self) -> ACKERR_R {
        ACKERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit0 Error"]
    #[inline(always)]
    pub fn bit0err(&self) -> BIT0ERR_R {
        BIT0ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bit1 Error"]
    #[inline(always)]
    pub fn bit1err(&self) -> BIT1ERR_R {
        BIT1ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn rwrnint(&self) -> RWRNINT_R {
        RWRNINT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn twrnint(&self) -> TWRNINT_R {
        TWRNINT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CAN Synchronization Status"]
    #[inline(always)]
    pub fn synch(&self) -> SYNCH_R {
        SYNCH_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wakint(&mut self) -> WAKINT_W<0> {
        WAKINT_W::new(self)
    }
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn errint(&mut self) -> ERRINT_W<1> {
        ERRINT_W::new(self)
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn boffint(&mut self) -> BOFFINT_W<2> {
        BOFFINT_W::new(self)
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rwrnint(&mut self) -> RWRNINT_W<16> {
        RWRNINT_W::new(self)
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn twrnint(&mut self) -> TWRNINT_W<17> {
        TWRNINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error and Status 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr1](index.html) module"]
pub struct ESR1_SPEC;
impl crate::RegisterSpec for ESR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esr1::R](R) reader structure"]
impl crate::Readable for ESR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esr1::W](W) writer structure"]
impl crate::Writable for ESR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESR1 to value 0"]
impl crate::Resettable for ESR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

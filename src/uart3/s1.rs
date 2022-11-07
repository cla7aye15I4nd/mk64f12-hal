#[doc = "Register `S1` reader"]
pub struct R(crate::R<S1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PF` reader - Parity Error Flag"]
pub type PF_R = crate::BitReader<PF_A>;
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PF_A {
    #[doc = "0: No parity error detected since the last time this flag was cleared. If the receive buffer has a depth greater than 1, then there may be data in the receive buffer what was received with a parity error."]
    _0 = 0,
    #[doc = "1: At least one dataword was received with a parity error since the last time this flag was cleared."]
    _1 = 1,
}
impl From<PF_A> for bool {
    #[inline(always)]
    fn from(variant: PF_A) -> Self {
        variant as u8 != 0
    }
}
impl PF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF_A {
        match self.bits {
            false => PF_A::_0,
            true => PF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PF_A::_1
    }
}
#[doc = "Field `FE` reader - Framing Error Flag"]
pub type FE_R = crate::BitReader<FE_A>;
#[doc = "Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_A {
    #[doc = "0: No framing error detected."]
    _0 = 0,
    #[doc = "1: Framing error."]
    _1 = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
impl FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::_0,
            true => FE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FE_A::_1
    }
}
#[doc = "Field `NF` reader - Noise Flag"]
pub type NF_R = crate::BitReader<NF_A>;
#[doc = "Noise Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NF_A {
    #[doc = "0: No noise detected since the last time this flag was cleared. If the receive buffer has a depth greater than 1 then there may be data in the receiver buffer that was received with noise."]
    _0 = 0,
    #[doc = "1: At least one dataword was received with noise detected since the last time the flag was cleared."]
    _1 = 1,
}
impl From<NF_A> for bool {
    #[inline(always)]
    fn from(variant: NF_A) -> Self {
        variant as u8 != 0
    }
}
impl NF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NF_A {
        match self.bits {
            false => NF_A::_0,
            true => NF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NF_A::_1
    }
}
#[doc = "Field `OR` reader - Receiver Overrun Flag"]
pub type OR_R = crate::BitReader<OR_A>;
#[doc = "Receiver Overrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OR_A {
    #[doc = "0: No overrun has occurred since the last time the flag was cleared."]
    _0 = 0,
    #[doc = "1: Overrun has occurred or the overrun flag has not been cleared since the last overrun occurred."]
    _1 = 1,
}
impl From<OR_A> for bool {
    #[inline(always)]
    fn from(variant: OR_A) -> Self {
        variant as u8 != 0
    }
}
impl OR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OR_A {
        match self.bits {
            false => OR_A::_0,
            true => OR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OR_A::_1
    }
}
#[doc = "Field `IDLE` reader - Idle Line Flag"]
pub type IDLE_R = crate::BitReader<IDLE_A>;
#[doc = "Idle Line Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE_A {
    #[doc = "0: Receiver input is either active now or has never become active since the IDLE flag was last cleared."]
    _0 = 0,
    #[doc = "1: Receiver input has become idle or the flag has not been cleared since it last asserted."]
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
#[doc = "Field `RDRF` reader - Receive Data Register Full Flag"]
pub type RDRF_R = crate::BitReader<RDRF_A>;
#[doc = "Receive Data Register Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRF_A {
    #[doc = "0: The number of datawords in the receive buffer is less than the number indicated by RXWATER."]
    _0 = 0,
    #[doc = "1: The number of datawords in the receive buffer is equal to or greater than the number indicated by RXWATER at some point in time since this flag was last cleared."]
    _1 = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
impl RDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::_0,
            true => RDRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRF_A::_1
    }
}
#[doc = "Field `TC` reader - Transmit Complete Flag"]
pub type TC_R = crate::BitReader<TC_A>;
#[doc = "Transmit Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    #[doc = "0: Transmitter active (sending data, a preamble, or a break)."]
    _0 = 0,
    #[doc = "1: Transmitter idle (transmission activity complete)."]
    _1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::_0,
            true => TC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TC_A::_1
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Register Empty Flag"]
pub type TDRE_R = crate::BitReader<TDRE_A>;
#[doc = "Transmit Data Register Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRE_A {
    #[doc = "0: The amount of data in the transmit buffer is greater than the value indicated by TWFIFO\\[TXWATER\\]."]
    _0 = 0,
    #[doc = "1: The amount of data in the transmit buffer is less than or equal to the value indicated by TWFIFO\\[TXWATER\\]
at some point in time since the flag has been cleared."]
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRE_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Flag"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing Error Flag"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise Flag"]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver Overrun Flag"]
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle Line Flag"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Register Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Complete Flag"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Register Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s1](index.html) module"]
pub struct S1_SPEC;
impl crate::RegisterSpec for S1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [s1::R](R) reader structure"]
impl crate::Readable for S1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S1 to value 0xc0"]
impl crate::Resettable for S1_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}

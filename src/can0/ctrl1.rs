#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROPSEG` reader - Propagation Segment"]
pub type PROPSEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROPSEG` writer - Propagation Segment"]
pub type PROPSEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `LOM` reader - Listen-Only Mode"]
pub type LOM_R = crate::BitReader<LOM_A>;
#[doc = "Listen-Only Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOM_A {
    #[doc = "0: Listen-Only mode is deactivated."]
    _0 = 0,
    #[doc = "1: FlexCAN module operates in Listen-Only mode."]
    _1 = 1,
}
impl From<LOM_A> for bool {
    #[inline(always)]
    fn from(variant: LOM_A) -> Self {
        variant as u8 != 0
    }
}
impl LOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOM_A {
        match self.bits {
            false => LOM_A::_0,
            true => LOM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOM_A::_1
    }
}
#[doc = "Field `LOM` writer - Listen-Only Mode"]
pub type LOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, LOM_A, O>;
impl<'a, const O: u8> LOM_W<'a, O> {
    #[doc = "Listen-Only mode is deactivated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOM_A::_0)
    }
    #[doc = "FlexCAN module operates in Listen-Only mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOM_A::_1)
    }
}
#[doc = "Field `LBUF` reader - Lowest Buffer Transmitted First"]
pub type LBUF_R = crate::BitReader<LBUF_A>;
#[doc = "Lowest Buffer Transmitted First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBUF_A {
    #[doc = "0: Buffer with highest priority is transmitted first."]
    _0 = 0,
    #[doc = "1: Lowest number buffer is transmitted first."]
    _1 = 1,
}
impl From<LBUF_A> for bool {
    #[inline(always)]
    fn from(variant: LBUF_A) -> Self {
        variant as u8 != 0
    }
}
impl LBUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBUF_A {
        match self.bits {
            false => LBUF_A::_0,
            true => LBUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBUF_A::_1
    }
}
#[doc = "Field `LBUF` writer - Lowest Buffer Transmitted First"]
pub type LBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, LBUF_A, O>;
impl<'a, const O: u8> LBUF_W<'a, O> {
    #[doc = "Buffer with highest priority is transmitted first."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBUF_A::_0)
    }
    #[doc = "Lowest number buffer is transmitted first."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBUF_A::_1)
    }
}
#[doc = "Field `TSYN` reader - Timer Sync"]
pub type TSYN_R = crate::BitReader<TSYN_A>;
#[doc = "Timer Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSYN_A {
    #[doc = "0: Timer Sync feature disabled"]
    _0 = 0,
    #[doc = "1: Timer Sync feature enabled"]
    _1 = 1,
}
impl From<TSYN_A> for bool {
    #[inline(always)]
    fn from(variant: TSYN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSYN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSYN_A {
        match self.bits {
            false => TSYN_A::_0,
            true => TSYN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSYN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSYN_A::_1
    }
}
#[doc = "Field `TSYN` writer - Timer Sync"]
pub type TSYN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, TSYN_A, O>;
impl<'a, const O: u8> TSYN_W<'a, O> {
    #[doc = "Timer Sync feature disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSYN_A::_0)
    }
    #[doc = "Timer Sync feature enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSYN_A::_1)
    }
}
#[doc = "Field `BOFFREC` reader - Bus Off Recovery"]
pub type BOFFREC_R = crate::BitReader<BOFFREC_A>;
#[doc = "Bus Off Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFFREC_A {
    #[doc = "0: Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B."]
    _0 = 0,
    #[doc = "1: Automatic recovering from Bus Off state disabled."]
    _1 = 1,
}
impl From<BOFFREC_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFREC_A) -> Self {
        variant as u8 != 0
    }
}
impl BOFFREC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFREC_A {
        match self.bits {
            false => BOFFREC_A::_0,
            true => BOFFREC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOFFREC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOFFREC_A::_1
    }
}
#[doc = "Field `BOFFREC` writer - Bus Off Recovery"]
pub type BOFFREC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, BOFFREC_A, O>;
impl<'a, const O: u8> BOFFREC_W<'a, O> {
    #[doc = "Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOFFREC_A::_0)
    }
    #[doc = "Automatic recovering from Bus Off state disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOFFREC_A::_1)
    }
}
#[doc = "Field `SMP` reader - CAN Bit Sampling"]
pub type SMP_R = crate::BitReader<SMP_A>;
#[doc = "CAN Bit Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMP_A {
    #[doc = "0: Just one sample is used to determine the bit value."]
    _0 = 0,
    #[doc = "1: Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples; a majority rule is used."]
    _1 = 1,
}
impl From<SMP_A> for bool {
    #[inline(always)]
    fn from(variant: SMP_A) -> Self {
        variant as u8 != 0
    }
}
impl SMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_A {
        match self.bits {
            false => SMP_A::_0,
            true => SMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMP_A::_1
    }
}
#[doc = "Field `SMP` writer - CAN Bit Sampling"]
pub type SMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, SMP_A, O>;
impl<'a, const O: u8> SMP_W<'a, O> {
    #[doc = "Just one sample is used to determine the bit value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMP_A::_0)
    }
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples; a majority rule is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMP_A::_1)
    }
}
#[doc = "Field `RWRNMSK` reader - Rx Warning Interrupt Mask"]
pub type RWRNMSK_R = crate::BitReader<RWRNMSK_A>;
#[doc = "Rx Warning Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWRNMSK_A {
    #[doc = "0: Rx Warning Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Rx Warning Interrupt enabled."]
    _1 = 1,
}
impl From<RWRNMSK_A> for bool {
    #[inline(always)]
    fn from(variant: RWRNMSK_A) -> Self {
        variant as u8 != 0
    }
}
impl RWRNMSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWRNMSK_A {
        match self.bits {
            false => RWRNMSK_A::_0,
            true => RWRNMSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWRNMSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWRNMSK_A::_1
    }
}
#[doc = "Field `RWRNMSK` writer - Rx Warning Interrupt Mask"]
pub type RWRNMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, RWRNMSK_A, O>;
impl<'a, const O: u8> RWRNMSK_W<'a, O> {
    #[doc = "Rx Warning Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWRNMSK_A::_0)
    }
    #[doc = "Rx Warning Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWRNMSK_A::_1)
    }
}
#[doc = "Field `TWRNMSK` reader - Tx Warning Interrupt Mask"]
pub type TWRNMSK_R = crate::BitReader<TWRNMSK_A>;
#[doc = "Tx Warning Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWRNMSK_A {
    #[doc = "0: Tx Warning Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Tx Warning Interrupt enabled."]
    _1 = 1,
}
impl From<TWRNMSK_A> for bool {
    #[inline(always)]
    fn from(variant: TWRNMSK_A) -> Self {
        variant as u8 != 0
    }
}
impl TWRNMSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWRNMSK_A {
        match self.bits {
            false => TWRNMSK_A::_0,
            true => TWRNMSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWRNMSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWRNMSK_A::_1
    }
}
#[doc = "Field `TWRNMSK` writer - Tx Warning Interrupt Mask"]
pub type TWRNMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, TWRNMSK_A, O>;
impl<'a, const O: u8> TWRNMSK_W<'a, O> {
    #[doc = "Tx Warning Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWRNMSK_A::_0)
    }
    #[doc = "Tx Warning Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWRNMSK_A::_1)
    }
}
#[doc = "Field `LPB` reader - Loop Back Mode"]
pub type LPB_R = crate::BitReader<LPB_A>;
#[doc = "Loop Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPB_A {
    #[doc = "0: Loop Back disabled."]
    _0 = 0,
    #[doc = "1: Loop Back enabled."]
    _1 = 1,
}
impl From<LPB_A> for bool {
    #[inline(always)]
    fn from(variant: LPB_A) -> Self {
        variant as u8 != 0
    }
}
impl LPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPB_A {
        match self.bits {
            false => LPB_A::_0,
            true => LPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPB_A::_1
    }
}
#[doc = "Field `LPB` writer - Loop Back Mode"]
pub type LPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, LPB_A, O>;
impl<'a, const O: u8> LPB_W<'a, O> {
    #[doc = "Loop Back disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPB_A::_0)
    }
    #[doc = "Loop Back enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPB_A::_1)
    }
}
#[doc = "Field `CLKSRC` reader - CAN Engine Clock Source"]
pub type CLKSRC_R = crate::BitReader<CLKSRC_A>;
#[doc = "CAN Engine Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSRC_A {
    #[doc = "0: The CAN engine clock source is the oscillator clock. Under this condition, the oscillator clock frequency must be lower than the bus clock."]
    _0 = 0,
    #[doc = "1: The CAN engine clock source is the peripheral clock."]
    _1 = 1,
}
impl From<CLKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSRC_A {
        match self.bits {
            false => CLKSRC_A::_0,
            true => CLKSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKSRC_A::_1
    }
}
#[doc = "Field `CLKSRC` writer - CAN Engine Clock Source"]
pub type CLKSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, CLKSRC_A, O>;
impl<'a, const O: u8> CLKSRC_W<'a, O> {
    #[doc = "The CAN engine clock source is the oscillator clock. Under this condition, the oscillator clock frequency must be lower than the bus clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKSRC_A::_0)
    }
    #[doc = "The CAN engine clock source is the peripheral clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKSRC_A::_1)
    }
}
#[doc = "Field `ERRMSK` reader - Error Mask"]
pub type ERRMSK_R = crate::BitReader<ERRMSK_A>;
#[doc = "Error Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRMSK_A {
    #[doc = "0: Error interrupt disabled."]
    _0 = 0,
    #[doc = "1: Error interrupt enabled."]
    _1 = 1,
}
impl From<ERRMSK_A> for bool {
    #[inline(always)]
    fn from(variant: ERRMSK_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRMSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRMSK_A {
        match self.bits {
            false => ERRMSK_A::_0,
            true => ERRMSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERRMSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERRMSK_A::_1
    }
}
#[doc = "Field `ERRMSK` writer - Error Mask"]
pub type ERRMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, ERRMSK_A, O>;
impl<'a, const O: u8> ERRMSK_W<'a, O> {
    #[doc = "Error interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERRMSK_A::_0)
    }
    #[doc = "Error interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERRMSK_A::_1)
    }
}
#[doc = "Field `BOFFMSK` reader - Bus Off Mask"]
pub type BOFFMSK_R = crate::BitReader<BOFFMSK_A>;
#[doc = "Bus Off Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFFMSK_A {
    #[doc = "0: Bus Off interrupt disabled."]
    _0 = 0,
    #[doc = "1: Bus Off interrupt enabled."]
    _1 = 1,
}
impl From<BOFFMSK_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFMSK_A) -> Self {
        variant as u8 != 0
    }
}
impl BOFFMSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFMSK_A {
        match self.bits {
            false => BOFFMSK_A::_0,
            true => BOFFMSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOFFMSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOFFMSK_A::_1
    }
}
#[doc = "Field `BOFFMSK` writer - Bus Off Mask"]
pub type BOFFMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, BOFFMSK_A, O>;
impl<'a, const O: u8> BOFFMSK_W<'a, O> {
    #[doc = "Bus Off interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOFFMSK_A::_0)
    }
    #[doc = "Bus Off interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOFFMSK_A::_1)
    }
}
#[doc = "Field `PSEG2` reader - Phase Segment 2"]
pub type PSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSEG2` writer - Phase Segment 2"]
pub type PSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `PSEG1` reader - Phase Segment 1"]
pub type PSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSEG1` writer - Phase Segment 1"]
pub type PSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `RJW` reader - Resync Jump Width"]
pub type RJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RJW` writer - Resync Jump Width"]
pub type RJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRESDIV` reader - Prescaler Division Factor"]
pub type PRESDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESDIV` writer - Prescaler Division Factor"]
pub type PRESDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2 - Propagation Segment"]
    #[inline(always)]
    pub fn propseg(&self) -> PROPSEG_R {
        PROPSEG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Listen-Only Mode"]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lowest Buffer Transmitted First"]
    #[inline(always)]
    pub fn lbuf(&self) -> LBUF_R {
        LBUF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer Sync"]
    #[inline(always)]
    pub fn tsyn(&self) -> TSYN_R {
        TSYN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bus Off Recovery"]
    #[inline(always)]
    pub fn boffrec(&self) -> BOFFREC_R {
        BOFFREC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN Bit Sampling"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Rx Warning Interrupt Mask"]
    #[inline(always)]
    pub fn rwrnmsk(&self) -> RWRNMSK_R {
        RWRNMSK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx Warning Interrupt Mask"]
    #[inline(always)]
    pub fn twrnmsk(&self) -> TWRNMSK_R {
        TWRNMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loop Back Mode"]
    #[inline(always)]
    pub fn lpb(&self) -> LPB_R {
        LPB_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CAN Engine Clock Source"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error Mask"]
    #[inline(always)]
    pub fn errmsk(&self) -> ERRMSK_R {
        ERRMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus Off Mask"]
    #[inline(always)]
    pub fn boffmsk(&self) -> BOFFMSK_R {
        BOFFMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Phase Segment 2"]
    #[inline(always)]
    pub fn pseg2(&self) -> PSEG2_R {
        PSEG2_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Phase Segment 1"]
    #[inline(always)]
    pub fn pseg1(&self) -> PSEG1_R {
        PSEG1_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - Resync Jump Width"]
    #[inline(always)]
    pub fn rjw(&self) -> RJW_R {
        RJW_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Prescaler Division Factor"]
    #[inline(always)]
    pub fn presdiv(&self) -> PRESDIV_R {
        PRESDIV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Propagation Segment"]
    #[inline(always)]
    #[must_use]
    pub fn propseg(&mut self) -> PROPSEG_W<0> {
        PROPSEG_W::new(self)
    }
    #[doc = "Bit 3 - Listen-Only Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lom(&mut self) -> LOM_W<3> {
        LOM_W::new(self)
    }
    #[doc = "Bit 4 - Lowest Buffer Transmitted First"]
    #[inline(always)]
    #[must_use]
    pub fn lbuf(&mut self) -> LBUF_W<4> {
        LBUF_W::new(self)
    }
    #[doc = "Bit 5 - Timer Sync"]
    #[inline(always)]
    #[must_use]
    pub fn tsyn(&mut self) -> TSYN_W<5> {
        TSYN_W::new(self)
    }
    #[doc = "Bit 6 - Bus Off Recovery"]
    #[inline(always)]
    #[must_use]
    pub fn boffrec(&mut self) -> BOFFREC_W<6> {
        BOFFREC_W::new(self)
    }
    #[doc = "Bit 7 - CAN Bit Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn smp(&mut self) -> SMP_W<7> {
        SMP_W::new(self)
    }
    #[doc = "Bit 10 - Rx Warning Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwrnmsk(&mut self) -> RWRNMSK_W<10> {
        RWRNMSK_W::new(self)
    }
    #[doc = "Bit 11 - Tx Warning Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twrnmsk(&mut self) -> TWRNMSK_W<11> {
        TWRNMSK_W::new(self)
    }
    #[doc = "Bit 12 - Loop Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpb(&mut self) -> LPB_W<12> {
        LPB_W::new(self)
    }
    #[doc = "Bit 13 - CAN Engine Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn clksrc(&mut self) -> CLKSRC_W<13> {
        CLKSRC_W::new(self)
    }
    #[doc = "Bit 14 - Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn errmsk(&mut self) -> ERRMSK_W<14> {
        ERRMSK_W::new(self)
    }
    #[doc = "Bit 15 - Bus Off Mask"]
    #[inline(always)]
    #[must_use]
    pub fn boffmsk(&mut self) -> BOFFMSK_W<15> {
        BOFFMSK_W::new(self)
    }
    #[doc = "Bits 16:18 - Phase Segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn pseg2(&mut self) -> PSEG2_W<16> {
        PSEG2_W::new(self)
    }
    #[doc = "Bits 19:21 - Phase Segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn pseg1(&mut self) -> PSEG1_W<19> {
        PSEG1_W::new(self)
    }
    #[doc = "Bits 22:23 - Resync Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn rjw(&mut self) -> RJW_W<22> {
        RJW_W::new(self)
    }
    #[doc = "Bits 24:31 - Prescaler Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn presdiv(&mut self) -> PRESDIV_W<24> {
        PRESDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

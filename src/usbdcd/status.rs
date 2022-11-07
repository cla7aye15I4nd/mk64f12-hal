#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEQ_RES` reader - Charger Detection Sequence Results"]
pub type SEQ_RES_R = crate::FieldReader<u8, SEQ_RES_A>;
#[doc = "Charger Detection Sequence Results\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEQ_RES_A {
    #[doc = "0: No results to report."]
    _00 = 0,
    #[doc = "1: Attached to a standard host. Must comply with USB 2.0 by drawing only 2.5 mA (max) until connected."]
    _01 = 1,
    #[doc = "2: Attached to a charging port. The exact meaning depends on bit 18: 0: Attached to either a charging host or a dedicated charger. The charger type detection has not completed. 1: Attached to a charging host. The charger type detection has completed."]
    _10 = 2,
    #[doc = "3: Attached to a dedicated charger."]
    _11 = 3,
}
impl From<SEQ_RES_A> for u8 {
    #[inline(always)]
    fn from(variant: SEQ_RES_A) -> Self {
        variant as _
    }
}
impl SEQ_RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQ_RES_A {
        match self.bits {
            0 => SEQ_RES_A::_00,
            1 => SEQ_RES_A::_01,
            2 => SEQ_RES_A::_10,
            3 => SEQ_RES_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SEQ_RES_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SEQ_RES_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEQ_RES_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEQ_RES_A::_11
    }
}
#[doc = "Field `SEQ_STAT` reader - Charger Detection Sequence Status"]
pub type SEQ_STAT_R = crate::FieldReader<u8, SEQ_STAT_A>;
#[doc = "Charger Detection Sequence Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEQ_STAT_A {
    #[doc = "0: The module is either not enabled, or the module is enabled but the data pins have not yet been detected."]
    _00 = 0,
    #[doc = "1: Data pin contact detection is complete."]
    _01 = 1,
    #[doc = "2: Charging port detection is complete."]
    _10 = 2,
    #[doc = "3: Charger type detection is complete."]
    _11 = 3,
}
impl From<SEQ_STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: SEQ_STAT_A) -> Self {
        variant as _
    }
}
impl SEQ_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQ_STAT_A {
        match self.bits {
            0 => SEQ_STAT_A::_00,
            1 => SEQ_STAT_A::_01,
            2 => SEQ_STAT_A::_10,
            3 => SEQ_STAT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SEQ_STAT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SEQ_STAT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEQ_STAT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEQ_STAT_A::_11
    }
}
#[doc = "Field `ERR` reader - Error Flag"]
pub type ERR_R = crate::BitReader<ERR_A>;
#[doc = "Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_A {
    #[doc = "0: No sequence errors."]
    _0 = 0,
    #[doc = "1: Error in the detection sequence. See the SEQ_STAT field to determine the phase in which the error occurred."]
    _1 = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_A {
        match self.bits {
            false => ERR_A::_0,
            true => ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR_A::_1
    }
}
#[doc = "Field `TO` reader - Timeout Flag"]
pub type TO_R = crate::BitReader<TO_A>;
#[doc = "Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TO_A {
    #[doc = "0: The detection sequence has not been running for over 1 s."]
    _0 = 0,
    #[doc = "1: It has been over 1 s since the data pin contact was detected and debounced."]
    _1 = 1,
}
impl From<TO_A> for bool {
    #[inline(always)]
    fn from(variant: TO_A) -> Self {
        variant as u8 != 0
    }
}
impl TO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TO_A {
        match self.bits {
            false => TO_A::_0,
            true => TO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TO_A::_1
    }
}
#[doc = "Field `ACTIVE` reader - Active Status Indicator"]
pub type ACTIVE_R = crate::BitReader<ACTIVE_A>;
#[doc = "Active Status Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE_A {
    #[doc = "0: The sequence is not running."]
    _0 = 0,
    #[doc = "1: The sequence is running."]
    _1 = 1,
}
impl From<ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE_A {
        match self.bits {
            false => ACTIVE_A::_0,
            true => ACTIVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTIVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTIVE_A::_1
    }
}
impl R {
    #[doc = "Bits 16:17 - Charger Detection Sequence Results"]
    #[inline(always)]
    pub fn seq_res(&self) -> SEQ_RES_R {
        SEQ_RES_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Charger Detection Sequence Status"]
    #[inline(always)]
    pub fn seq_stat(&self) -> SEQ_STAT_R {
        SEQ_STAT_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Error Flag"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timeout Flag"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Active Status Indicator"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

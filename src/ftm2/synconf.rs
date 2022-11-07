#[doc = "Register `SYNCONF` reader"]
pub struct R(crate::R<SYNCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCONF` writer"]
pub struct W(crate::W<SYNCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCONF_SPEC>;
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
impl From<crate::W<SYNCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWTRIGMODE` reader - Hardware Trigger Mode"]
pub type HWTRIGMODE_R = crate::BitReader<HWTRIGMODE_A>;
#[doc = "Hardware Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWTRIGMODE_A {
    #[doc = "0: FTM clears the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    _0 = 0,
    #[doc = "1: FTM does not clear the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    _1 = 1,
}
impl From<HWTRIGMODE_A> for bool {
    #[inline(always)]
    fn from(variant: HWTRIGMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl HWTRIGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWTRIGMODE_A {
        match self.bits {
            false => HWTRIGMODE_A::_0,
            true => HWTRIGMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWTRIGMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWTRIGMODE_A::_1
    }
}
#[doc = "Field `HWTRIGMODE` writer - Hardware Trigger Mode"]
pub type HWTRIGMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, HWTRIGMODE_A, O>;
impl<'a, const O: u8> HWTRIGMODE_W<'a, O> {
    #[doc = "FTM clears the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWTRIGMODE_A::_0)
    }
    #[doc = "FTM does not clear the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWTRIGMODE_A::_1)
    }
}
#[doc = "Field `CNTINC` reader - CNTIN Register Synchronization"]
pub type CNTINC_R = crate::BitReader<CNTINC_A>;
#[doc = "CNTIN Register Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTINC_A {
    #[doc = "0: CNTIN register is updated with its buffer value at all rising edges of system clock."]
    _0 = 0,
    #[doc = "1: CNTIN register is updated with its buffer value by the PWM synchronization."]
    _1 = 1,
}
impl From<CNTINC_A> for bool {
    #[inline(always)]
    fn from(variant: CNTINC_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTINC_A {
        match self.bits {
            false => CNTINC_A::_0,
            true => CNTINC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTINC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTINC_A::_1
    }
}
#[doc = "Field `CNTINC` writer - CNTIN Register Synchronization"]
pub type CNTINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, CNTINC_A, O>;
impl<'a, const O: u8> CNTINC_W<'a, O> {
    #[doc = "CNTIN register is updated with its buffer value at all rising edges of system clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTINC_A::_0)
    }
    #[doc = "CNTIN register is updated with its buffer value by the PWM synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTINC_A::_1)
    }
}
#[doc = "Field `INVC` reader - INVCTRL Register Synchronization"]
pub type INVC_R = crate::BitReader<INVC_A>;
#[doc = "INVCTRL Register Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVC_A {
    #[doc = "0: INVCTRL register is updated with its buffer value at all rising edges of system clock."]
    _0 = 0,
    #[doc = "1: INVCTRL register is updated with its buffer value by the PWM synchronization."]
    _1 = 1,
}
impl From<INVC_A> for bool {
    #[inline(always)]
    fn from(variant: INVC_A) -> Self {
        variant as u8 != 0
    }
}
impl INVC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVC_A {
        match self.bits {
            false => INVC_A::_0,
            true => INVC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INVC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INVC_A::_1
    }
}
#[doc = "Field `INVC` writer - INVCTRL Register Synchronization"]
pub type INVC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, INVC_A, O>;
impl<'a, const O: u8> INVC_W<'a, O> {
    #[doc = "INVCTRL register is updated with its buffer value at all rising edges of system clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVC_A::_0)
    }
    #[doc = "INVCTRL register is updated with its buffer value by the PWM synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVC_A::_1)
    }
}
#[doc = "Field `SWOC` reader - SWOCTRL Register Synchronization"]
pub type SWOC_R = crate::BitReader<SWOC_A>;
#[doc = "SWOCTRL Register Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWOC_A {
    #[doc = "0: SWOCTRL register is updated with its buffer value at all rising edges of system clock."]
    _0 = 0,
    #[doc = "1: SWOCTRL register is updated with its buffer value by the PWM synchronization."]
    _1 = 1,
}
impl From<SWOC_A> for bool {
    #[inline(always)]
    fn from(variant: SWOC_A) -> Self {
        variant as u8 != 0
    }
}
impl SWOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWOC_A {
        match self.bits {
            false => SWOC_A::_0,
            true => SWOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWOC_A::_1
    }
}
#[doc = "Field `SWOC` writer - SWOCTRL Register Synchronization"]
pub type SWOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, SWOC_A, O>;
impl<'a, const O: u8> SWOC_W<'a, O> {
    #[doc = "SWOCTRL register is updated with its buffer value at all rising edges of system clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWOC_A::_0)
    }
    #[doc = "SWOCTRL register is updated with its buffer value by the PWM synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWOC_A::_1)
    }
}
#[doc = "Field `SYNCMODE` reader - Synchronization Mode"]
pub type SYNCMODE_R = crate::BitReader<SYNCMODE_A>;
#[doc = "Synchronization Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCMODE_A {
    #[doc = "0: Legacy PWM synchronization is selected."]
    _0 = 0,
    #[doc = "1: Enhanced PWM synchronization is selected."]
    _1 = 1,
}
impl From<SYNCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCMODE_A {
        match self.bits {
            false => SYNCMODE_A::_0,
            true => SYNCMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCMODE_A::_1
    }
}
#[doc = "Field `SYNCMODE` writer - Synchronization Mode"]
pub type SYNCMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, SYNCMODE_A, O>;
impl<'a, const O: u8> SYNCMODE_W<'a, O> {
    #[doc = "Legacy PWM synchronization is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCMODE_A::_0)
    }
    #[doc = "Enhanced PWM synchronization is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCMODE_A::_1)
    }
}
#[doc = "Field `SWRSTCNT` reader - FTM counter synchronization is activated by the software trigger."]
pub type SWRSTCNT_R = crate::BitReader<SWRSTCNT_A>;
#[doc = "FTM counter synchronization is activated by the software trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTCNT_A {
    #[doc = "0: The software trigger does not activate the FTM counter synchronization."]
    _0 = 0,
    #[doc = "1: The software trigger activates the FTM counter synchronization."]
    _1 = 1,
}
impl From<SWRSTCNT_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTCNT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRSTCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTCNT_A {
        match self.bits {
            false => SWRSTCNT_A::_0,
            true => SWRSTCNT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRSTCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRSTCNT_A::_1
    }
}
#[doc = "Field `SWRSTCNT` writer - FTM counter synchronization is activated by the software trigger."]
pub type SWRSTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, SWRSTCNT_A, O>;
impl<'a, const O: u8> SWRSTCNT_W<'a, O> {
    #[doc = "The software trigger does not activate the FTM counter synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTCNT_A::_0)
    }
    #[doc = "The software trigger activates the FTM counter synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTCNT_A::_1)
    }
}
#[doc = "Field `SWWRBUF` reader - MOD, CNTIN, and CV registers synchronization is activated by the software trigger."]
pub type SWWRBUF_R = crate::BitReader<SWWRBUF_A>;
#[doc = "MOD, CNTIN, and CV registers synchronization is activated by the software trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWWRBUF_A {
    #[doc = "0: The software trigger does not activate MOD, CNTIN, and CV registers synchronization."]
    _0 = 0,
    #[doc = "1: The software trigger activates MOD, CNTIN, and CV registers synchronization."]
    _1 = 1,
}
impl From<SWWRBUF_A> for bool {
    #[inline(always)]
    fn from(variant: SWWRBUF_A) -> Self {
        variant as u8 != 0
    }
}
impl SWWRBUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWWRBUF_A {
        match self.bits {
            false => SWWRBUF_A::_0,
            true => SWWRBUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWWRBUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWWRBUF_A::_1
    }
}
#[doc = "Field `SWWRBUF` writer - MOD, CNTIN, and CV registers synchronization is activated by the software trigger."]
pub type SWWRBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, SWWRBUF_A, O>;
impl<'a, const O: u8> SWWRBUF_W<'a, O> {
    #[doc = "The software trigger does not activate MOD, CNTIN, and CV registers synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWWRBUF_A::_0)
    }
    #[doc = "The software trigger activates MOD, CNTIN, and CV registers synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWWRBUF_A::_1)
    }
}
#[doc = "Field `SWOM` reader - Output mask synchronization is activated by the software trigger."]
pub type SWOM_R = crate::BitReader<SWOM_A>;
#[doc = "Output mask synchronization is activated by the software trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWOM_A {
    #[doc = "0: The software trigger does not activate the OUTMASK register synchronization."]
    _0 = 0,
    #[doc = "1: The software trigger activates the OUTMASK register synchronization."]
    _1 = 1,
}
impl From<SWOM_A> for bool {
    #[inline(always)]
    fn from(variant: SWOM_A) -> Self {
        variant as u8 != 0
    }
}
impl SWOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWOM_A {
        match self.bits {
            false => SWOM_A::_0,
            true => SWOM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWOM_A::_1
    }
}
#[doc = "Field `SWOM` writer - Output mask synchronization is activated by the software trigger."]
pub type SWOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, SWOM_A, O>;
impl<'a, const O: u8> SWOM_W<'a, O> {
    #[doc = "The software trigger does not activate the OUTMASK register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWOM_A::_0)
    }
    #[doc = "The software trigger activates the OUTMASK register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWOM_A::_1)
    }
}
#[doc = "Field `SWINVC` reader - Inverting control synchronization is activated by the software trigger."]
pub type SWINVC_R = crate::BitReader<SWINVC_A>;
#[doc = "Inverting control synchronization is activated by the software trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWINVC_A {
    #[doc = "0: The software trigger does not activate the INVCTRL register synchronization."]
    _0 = 0,
    #[doc = "1: The software trigger activates the INVCTRL register synchronization."]
    _1 = 1,
}
impl From<SWINVC_A> for bool {
    #[inline(always)]
    fn from(variant: SWINVC_A) -> Self {
        variant as u8 != 0
    }
}
impl SWINVC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWINVC_A {
        match self.bits {
            false => SWINVC_A::_0,
            true => SWINVC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWINVC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWINVC_A::_1
    }
}
#[doc = "Field `SWINVC` writer - Inverting control synchronization is activated by the software trigger."]
pub type SWINVC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, SWINVC_A, O>;
impl<'a, const O: u8> SWINVC_W<'a, O> {
    #[doc = "The software trigger does not activate the INVCTRL register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWINVC_A::_0)
    }
    #[doc = "The software trigger activates the INVCTRL register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWINVC_A::_1)
    }
}
#[doc = "Field `SWSOC` reader - Software output control synchronization is activated by the software trigger."]
pub type SWSOC_R = crate::BitReader<SWSOC_A>;
#[doc = "Software output control synchronization is activated by the software trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSOC_A {
    #[doc = "0: The software trigger does not activate the SWOCTRL register synchronization."]
    _0 = 0,
    #[doc = "1: The software trigger activates the SWOCTRL register synchronization."]
    _1 = 1,
}
impl From<SWSOC_A> for bool {
    #[inline(always)]
    fn from(variant: SWSOC_A) -> Self {
        variant as u8 != 0
    }
}
impl SWSOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSOC_A {
        match self.bits {
            false => SWSOC_A::_0,
            true => SWSOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWSOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWSOC_A::_1
    }
}
#[doc = "Field `SWSOC` writer - Software output control synchronization is activated by the software trigger."]
pub type SWSOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, SWSOC_A, O>;
impl<'a, const O: u8> SWSOC_W<'a, O> {
    #[doc = "The software trigger does not activate the SWOCTRL register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWSOC_A::_0)
    }
    #[doc = "The software trigger activates the SWOCTRL register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWSOC_A::_1)
    }
}
#[doc = "Field `HWRSTCNT` reader - FTM counter synchronization is activated by a hardware trigger."]
pub type HWRSTCNT_R = crate::BitReader<HWRSTCNT_A>;
#[doc = "FTM counter synchronization is activated by a hardware trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWRSTCNT_A {
    #[doc = "0: A hardware trigger does not activate the FTM counter synchronization."]
    _0 = 0,
    #[doc = "1: A hardware trigger activates the FTM counter synchronization."]
    _1 = 1,
}
impl From<HWRSTCNT_A> for bool {
    #[inline(always)]
    fn from(variant: HWRSTCNT_A) -> Self {
        variant as u8 != 0
    }
}
impl HWRSTCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWRSTCNT_A {
        match self.bits {
            false => HWRSTCNT_A::_0,
            true => HWRSTCNT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWRSTCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWRSTCNT_A::_1
    }
}
#[doc = "Field `HWRSTCNT` writer - FTM counter synchronization is activated by a hardware trigger."]
pub type HWRSTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, HWRSTCNT_A, O>;
impl<'a, const O: u8> HWRSTCNT_W<'a, O> {
    #[doc = "A hardware trigger does not activate the FTM counter synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWRSTCNT_A::_0)
    }
    #[doc = "A hardware trigger activates the FTM counter synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWRSTCNT_A::_1)
    }
}
#[doc = "Field `HWWRBUF` reader - MOD, CNTIN, and CV registers synchronization is activated by a hardware trigger."]
pub type HWWRBUF_R = crate::BitReader<HWWRBUF_A>;
#[doc = "MOD, CNTIN, and CV registers synchronization is activated by a hardware trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWWRBUF_A {
    #[doc = "0: A hardware trigger does not activate MOD, CNTIN, and CV registers synchronization."]
    _0 = 0,
    #[doc = "1: A hardware trigger activates MOD, CNTIN, and CV registers synchronization."]
    _1 = 1,
}
impl From<HWWRBUF_A> for bool {
    #[inline(always)]
    fn from(variant: HWWRBUF_A) -> Self {
        variant as u8 != 0
    }
}
impl HWWRBUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWWRBUF_A {
        match self.bits {
            false => HWWRBUF_A::_0,
            true => HWWRBUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWWRBUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWWRBUF_A::_1
    }
}
#[doc = "Field `HWWRBUF` writer - MOD, CNTIN, and CV registers synchronization is activated by a hardware trigger."]
pub type HWWRBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, HWWRBUF_A, O>;
impl<'a, const O: u8> HWWRBUF_W<'a, O> {
    #[doc = "A hardware trigger does not activate MOD, CNTIN, and CV registers synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWWRBUF_A::_0)
    }
    #[doc = "A hardware trigger activates MOD, CNTIN, and CV registers synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWWRBUF_A::_1)
    }
}
#[doc = "Field `HWOM` reader - Output mask synchronization is activated by a hardware trigger."]
pub type HWOM_R = crate::BitReader<HWOM_A>;
#[doc = "Output mask synchronization is activated by a hardware trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWOM_A {
    #[doc = "0: A hardware trigger does not activate the OUTMASK register synchronization."]
    _0 = 0,
    #[doc = "1: A hardware trigger activates the OUTMASK register synchronization."]
    _1 = 1,
}
impl From<HWOM_A> for bool {
    #[inline(always)]
    fn from(variant: HWOM_A) -> Self {
        variant as u8 != 0
    }
}
impl HWOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWOM_A {
        match self.bits {
            false => HWOM_A::_0,
            true => HWOM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWOM_A::_1
    }
}
#[doc = "Field `HWOM` writer - Output mask synchronization is activated by a hardware trigger."]
pub type HWOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, HWOM_A, O>;
impl<'a, const O: u8> HWOM_W<'a, O> {
    #[doc = "A hardware trigger does not activate the OUTMASK register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWOM_A::_0)
    }
    #[doc = "A hardware trigger activates the OUTMASK register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWOM_A::_1)
    }
}
#[doc = "Field `HWINVC` reader - Inverting control synchronization is activated by a hardware trigger."]
pub type HWINVC_R = crate::BitReader<HWINVC_A>;
#[doc = "Inverting control synchronization is activated by a hardware trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWINVC_A {
    #[doc = "0: A hardware trigger does not activate the INVCTRL register synchronization."]
    _0 = 0,
    #[doc = "1: A hardware trigger activates the INVCTRL register synchronization."]
    _1 = 1,
}
impl From<HWINVC_A> for bool {
    #[inline(always)]
    fn from(variant: HWINVC_A) -> Self {
        variant as u8 != 0
    }
}
impl HWINVC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWINVC_A {
        match self.bits {
            false => HWINVC_A::_0,
            true => HWINVC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWINVC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWINVC_A::_1
    }
}
#[doc = "Field `HWINVC` writer - Inverting control synchronization is activated by a hardware trigger."]
pub type HWINVC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, HWINVC_A, O>;
impl<'a, const O: u8> HWINVC_W<'a, O> {
    #[doc = "A hardware trigger does not activate the INVCTRL register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWINVC_A::_0)
    }
    #[doc = "A hardware trigger activates the INVCTRL register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWINVC_A::_1)
    }
}
#[doc = "Field `HWSOC` reader - Software output control synchronization is activated by a hardware trigger."]
pub type HWSOC_R = crate::BitReader<HWSOC_A>;
#[doc = "Software output control synchronization is activated by a hardware trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWSOC_A {
    #[doc = "0: A hardware trigger does not activate the SWOCTRL register synchronization."]
    _0 = 0,
    #[doc = "1: A hardware trigger activates the SWOCTRL register synchronization."]
    _1 = 1,
}
impl From<HWSOC_A> for bool {
    #[inline(always)]
    fn from(variant: HWSOC_A) -> Self {
        variant as u8 != 0
    }
}
impl HWSOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWSOC_A {
        match self.bits {
            false => HWSOC_A::_0,
            true => HWSOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWSOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWSOC_A::_1
    }
}
#[doc = "Field `HWSOC` writer - Software output control synchronization is activated by a hardware trigger."]
pub type HWSOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCONF_SPEC, HWSOC_A, O>;
impl<'a, const O: u8> HWSOC_W<'a, O> {
    #[doc = "A hardware trigger does not activate the SWOCTRL register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWSOC_A::_0)
    }
    #[doc = "A hardware trigger activates the SWOCTRL register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWSOC_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Hardware Trigger Mode"]
    #[inline(always)]
    pub fn hwtrigmode(&self) -> HWTRIGMODE_R {
        HWTRIGMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CNTIN Register Synchronization"]
    #[inline(always)]
    pub fn cntinc(&self) -> CNTINC_R {
        CNTINC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - INVCTRL Register Synchronization"]
    #[inline(always)]
    pub fn invc(&self) -> INVC_R {
        INVC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SWOCTRL Register Synchronization"]
    #[inline(always)]
    pub fn swoc(&self) -> SWOC_R {
        SWOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization Mode"]
    #[inline(always)]
    pub fn syncmode(&self) -> SYNCMODE_R {
        SYNCMODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FTM counter synchronization is activated by the software trigger."]
    #[inline(always)]
    pub fn swrstcnt(&self) -> SWRSTCNT_R {
        SWRSTCNT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MOD, CNTIN, and CV registers synchronization is activated by the software trigger."]
    #[inline(always)]
    pub fn swwrbuf(&self) -> SWWRBUF_R {
        SWWRBUF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output mask synchronization is activated by the software trigger."]
    #[inline(always)]
    pub fn swom(&self) -> SWOM_R {
        SWOM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Inverting control synchronization is activated by the software trigger."]
    #[inline(always)]
    pub fn swinvc(&self) -> SWINVC_R {
        SWINVC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software output control synchronization is activated by the software trigger."]
    #[inline(always)]
    pub fn swsoc(&self) -> SWSOC_R {
        SWSOC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - FTM counter synchronization is activated by a hardware trigger."]
    #[inline(always)]
    pub fn hwrstcnt(&self) -> HWRSTCNT_R {
        HWRSTCNT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MOD, CNTIN, and CV registers synchronization is activated by a hardware trigger."]
    #[inline(always)]
    pub fn hwwrbuf(&self) -> HWWRBUF_R {
        HWWRBUF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output mask synchronization is activated by a hardware trigger."]
    #[inline(always)]
    pub fn hwom(&self) -> HWOM_R {
        HWOM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Inverting control synchronization is activated by a hardware trigger."]
    #[inline(always)]
    pub fn hwinvc(&self) -> HWINVC_R {
        HWINVC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Software output control synchronization is activated by a hardware trigger."]
    #[inline(always)]
    pub fn hwsoc(&self) -> HWSOC_R {
        HWSOC_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hwtrigmode(&mut self) -> HWTRIGMODE_W<0> {
        HWTRIGMODE_W::new(self)
    }
    #[doc = "Bit 2 - CNTIN Register Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn cntinc(&mut self) -> CNTINC_W<2> {
        CNTINC_W::new(self)
    }
    #[doc = "Bit 4 - INVCTRL Register Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn invc(&mut self) -> INVC_W<4> {
        INVC_W::new(self)
    }
    #[doc = "Bit 5 - SWOCTRL Register Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn swoc(&mut self) -> SWOC_W<5> {
        SWOC_W::new(self)
    }
    #[doc = "Bit 7 - Synchronization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn syncmode(&mut self) -> SYNCMODE_W<7> {
        SYNCMODE_W::new(self)
    }
    #[doc = "Bit 8 - FTM counter synchronization is activated by the software trigger."]
    #[inline(always)]
    #[must_use]
    pub fn swrstcnt(&mut self) -> SWRSTCNT_W<8> {
        SWRSTCNT_W::new(self)
    }
    #[doc = "Bit 9 - MOD, CNTIN, and CV registers synchronization is activated by the software trigger."]
    #[inline(always)]
    #[must_use]
    pub fn swwrbuf(&mut self) -> SWWRBUF_W<9> {
        SWWRBUF_W::new(self)
    }
    #[doc = "Bit 10 - Output mask synchronization is activated by the software trigger."]
    #[inline(always)]
    #[must_use]
    pub fn swom(&mut self) -> SWOM_W<10> {
        SWOM_W::new(self)
    }
    #[doc = "Bit 11 - Inverting control synchronization is activated by the software trigger."]
    #[inline(always)]
    #[must_use]
    pub fn swinvc(&mut self) -> SWINVC_W<11> {
        SWINVC_W::new(self)
    }
    #[doc = "Bit 12 - Software output control synchronization is activated by the software trigger."]
    #[inline(always)]
    #[must_use]
    pub fn swsoc(&mut self) -> SWSOC_W<12> {
        SWSOC_W::new(self)
    }
    #[doc = "Bit 16 - FTM counter synchronization is activated by a hardware trigger."]
    #[inline(always)]
    #[must_use]
    pub fn hwrstcnt(&mut self) -> HWRSTCNT_W<16> {
        HWRSTCNT_W::new(self)
    }
    #[doc = "Bit 17 - MOD, CNTIN, and CV registers synchronization is activated by a hardware trigger."]
    #[inline(always)]
    #[must_use]
    pub fn hwwrbuf(&mut self) -> HWWRBUF_W<17> {
        HWWRBUF_W::new(self)
    }
    #[doc = "Bit 18 - Output mask synchronization is activated by a hardware trigger."]
    #[inline(always)]
    #[must_use]
    pub fn hwom(&mut self) -> HWOM_W<18> {
        HWOM_W::new(self)
    }
    #[doc = "Bit 19 - Inverting control synchronization is activated by a hardware trigger."]
    #[inline(always)]
    #[must_use]
    pub fn hwinvc(&mut self) -> HWINVC_W<19> {
        HWINVC_W::new(self)
    }
    #[doc = "Bit 20 - Software output control synchronization is activated by a hardware trigger."]
    #[inline(always)]
    #[must_use]
    pub fn hwsoc(&mut self) -> HWSOC_W<20> {
        HWSOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronization Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synconf](index.html) module"]
pub struct SYNCONF_SPEC;
impl crate::RegisterSpec for SYNCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [synconf::R](R) reader structure"]
impl crate::Readable for SYNCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [synconf::W](W) writer structure"]
impl crate::Writable for SYNCONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCONF to value 0"]
impl crate::Resettable for SYNCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
